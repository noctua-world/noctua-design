// Everything a `Select` does once a script is running.
//
// IT SHIPS, for the reason `menu.js` gives: a component whose consumer cannot
// use it is a component that lies. With no script a `Select` is a real
// `<select>` and works completely; what this adds is the listbox — a filter, a
// second line per option, a swatch, and the type-ahead a native menu on a
// desktop already has and one on a telephone does not.
//
// THE ANCHORING IS NOT WRITTEN HERE, and neither is which panel may be open:
// `anchor.js` places the list and `panels.js` decides that only one floating
// list on the page is open at a time.
//
// A HOST THAT HAS TO DO SOMETHING WHEN THE LIST OPENS HEARS `nd-select-open`.
// The documentation site loads a palette stylesheet per swatch on open, which
// is a thing about that site and not about this component — so the component
// says when, and the host says what.

import { keepAnchored, releaseAnchor } from "./anchor.js";
import { claimOpenPanel, releaseOpenPanel } from "./panels.js";
import { message, saySentence } from "./messages.js";

/** How long a run of typed letters counts as one word, with no filter box. */
const TYPE_AHEAD = 700;


// WHAT THIS ADDS, AND WHAT IT IS FORBIDDEN TO TAKE AWAY.
//
// The control under all of this is a real `<select>` with real `<option>`s in
// it. Everything below reads its state from that select and writes every choice
// back into it — so the value that reaches a server is the browser's own, and a
// page whose script failed to load is a page that still submits. The usual
// alternative, a `role="listbox"` beside a hidden `<input>`, is where this whole
// category of component gets its signature bug: a control that looks right and
// submits nothing.
//
// EVERY WORD comes off a `data-nd-*` attribute the component wrote. §9.9 forbids
// the design system from carrying text in any language and this file is not a
// loophole — it is the same file, one build later.
//
// EVERY DRAWING is cloned from a `<template>` the markup already carries. §9.5
// says icons come from one place; a script that carried an SVG would be a
// second one, and the second one is the one nobody audits.
//
// THE SECURITY QUESTION, ANSWERED IN WRITING (`NOCTUA.md §2.4).
//
// Three places here take a string that did not come from this build, and each
// is closed deliberately rather than by luck:
//
//   * `creatable` puts what a reader typed into the page. It goes in through
//     `value` and `textContent` on an element this file constructs — never
//     `innerHTML`, never a string of markup — so a typed `<script>` is a typed
//     `<script>`: eleven characters in a list, and nothing else.
//   * `data-nd-swatch` becomes a stylesheet request, and a request built from
//     an attribute is a request an attribute can aim. `ensureStylesheet` is
//     called only for a name the generated `paletteValues` already knows, so
//     the set of URLs this can fetch is the set the build shipped.
//   * `data-nd-icon` and `name` are interpolated into selectors, through
//     `CSS.escape`. A selector is not a sink on its own; an unescaped one is a
//     query that silently matches the wrong element, which is the quieter bug.

const SELECT_FIELD = "[data-nd-select]";

/** Everything one enhanced field needs to know about itself. */
const selects = new WeakMap();

/** The one open list. Two would be two things claiming the arrow keys. */
let openSelect = null;


export function enhanceSelects(root = document) {
  for (const field of root.querySelectorAll(SELECT_FIELD)) enhanceSelect(field);
}

/**
 * Takes the behaviour back off every enhanced field under `root`.
 *
 * A BEHAVIOUR THAT CANNOT BE RELEASED IS A DEFECT FOR EVERY CONSUMER WITH A
 * CLIENT-SIDE ROUTER. What a `Select` leaves behind is not a listener on the
 * document, because the outside press is one delegated listener for the whole
 * page. It is three module-level references to a field no longer on it:
 * `openSelect`, the `openPanels` registry, and `anchor.js`'s own map.
 */
export function releaseSelects(root = document) {
  for (const field of root.querySelectorAll(SELECT_FIELD)) {
    const state = selects.get(field);
    if (!state) continue;
    if (openSelect === state) openSelect = null;
    if (state.close) releaseOpenPanel(state.close);
    releaseAnchor(state.panel);
    state.binding.abort();
    selects.delete(field);
  }
}

function enhanceSelect(field) {
  if (selects.has(field)) return;
  const binding = new AbortController();
  const signal = binding.signal;
  const native = field.querySelector(".nd-select-native");
  const box = field.querySelector(".nd-select-field");
  // `Field` draws the question now, so it is a sibling of `.nd-select` rather
  // than a child of it — and it carries `{control}-label`, which is what the
  // enhanced control points `aria-labelledby` at.
  const label = field.closest(".nd-field")?.querySelector(".nd-label");
  if (!native || !box || !label || !native.id) return;

  const id = native.id;
  const searchable = field.dataset.ndSearchable === "true";
  // `creatable` NEEDS A FILTER BOX, AND USED TO ACCEPT ONE WITHOUT. The offer
  // to add is a row inside the filtered list and `create()` reads the box, so
  // without one the attribute reached nothing and would have thrown if it had.
  // Read here rather than at each use, so the dependency is stated once.
  const creatable = searchable && field.dataset.ndCreatable === "true";
  label.id = `${id}-label`;

  // THE NATIVE CONTROL IS HIDDEN, NOT `aria-hidden`.
  //
  // `hidden` takes it out of the accessibility tree AND out of the tab order in
  // one move, with no focusable element left claiming to be invisible — which
  // is what `aria-hidden` on a tabbable control is, and what axe calls
  // `aria-hidden-focus`. A hidden form control still submits: that is the whole
  // arrangement.
  native.hidden = true;

  const control = document.createElement("button");
  control.type = "button";
  control.id = `${id}-control`;
  control.className = "nd-select-control";
  control.disabled = native.disabled;
  control.setAttribute("aria-haspopup", "listbox");
  control.setAttribute("aria-expanded", "false");
  control.setAttribute("aria-controls", `${id}-listbox`);
  // Named by the field's own label and by its own text, which is the answer.
  // That is APG's select-only combobox, and it is why the reference to itself
  // is deliberate rather than a mistake.
  control.setAttribute("aria-labelledby", `${label.id} ${control.id}`);
  const described = native.getAttribute("aria-describedby");
  if (described) control.setAttribute("aria-describedby", described);
  // Only the button IS the combobox when there is no filter box. With one,
  // focus moves into the input and the input carries the role — a `combobox`
  // that does not have focus cannot own `aria-activedescendant`.
  if (!searchable) control.setAttribute("role", "combobox");
  // AN ANSWER IS COMPULSORY, AND THE ENHANCED CONTROL NEVER SAID SO. The
  // native `<select>` carries the real `required` and is `hidden`, so the only
  // thing a reader can reach was a plain button with nothing on it — the
  // string `aria-required` did not exist anywhere in this repository, under a
  // contract promising it follows from the prop. `required` is not an
  // attribute a `<button>` has, so this is the one way to state the fact.
  if (native.required) control.setAttribute("aria-required", "true");
  const value = document.createElement("span");
  value.className = "nd-select-value";
  control.append(value);
  // The label pointed at the control that is now hidden. A `<button>` is a
  // labelable element, so it can take the same label rather than needing a
  // second copy of the words.
  label.htmlFor = control.id;
  box.prepend(control);

  const panel = document.createElement("div");
  panel.className = "nd-panel nd-select-panel";
  panel.hidden = true;

  let search = null;
  if (searchable) {
    search = document.createElement("input");
    search.type = "text";
    search.className = "nd-select-search";
    search.setAttribute("role", "combobox");
    search.setAttribute("aria-expanded", "true");
    search.setAttribute("aria-controls", `${id}-listbox`);
    search.setAttribute("aria-autocomplete", "list");
    search.autocomplete = "off";
    search.spellcheck = false;
    if (field.dataset.ndSearchLabel) search.setAttribute("aria-label", field.dataset.ndSearchLabel);
    // The filter box is the combobox when there is one, so the fact moves with
    // the role rather than being stated on the element beside it.
    if (native.required) search.setAttribute("aria-required", "true");
    panel.append(search);
  }

  const list = document.createElement("ul");
  list.className = "nd-select-list";
  list.id = `${id}-listbox`;
  list.setAttribute("role", "listbox");
  if (native.multiple) list.setAttribute("aria-multiselectable", "true");
  panel.append(list);

  // One line for three opposite facts — nothing matched, nothing has arrived
  // yet, and here is what typing it would add — because they never co-occur and
  // three empty paragraphs would be three things to keep hidden.
  const note = document.createElement("p");
  note.className = "nd-select-note";
  panel.append(note);

  box.append(panel);
  box.dataset.ndEnhanced = "true";

  const state = {
    // CARRIED ON THE STATE, because every helper below already takes it. A
    // parallel argument threaded through a dozen call sites is a dozen places
    // for one of them to be forgotten.
    binding,
    signal,
    field,
    native,
    box,
    label,
    control,
    panel,
    list,
    note,
    search,
    creatable,
    id,
    rows: [],
    active: -1,
    typed: "",
    typedAt: 0,
    // Read once, before anything can change it: whether the sentence in the
    // error slot is the caller's. If it is, the script neither writes over it
    // nor clears it — a server's answer is not the browser's to withdraw.
    saidByCaller: field.dataset.ndInvalid === "true",
  };
  selects.set(field, state);

  control.addEventListener("click", () => (state.panel.hidden ? openList(state) : closeList(state)), { signal });
  control.addEventListener("keydown", (event) => onSelectKey(state, event), { signal });
  if (search) {
    search.addEventListener("keydown", (event) => onSelectKey(state, event), { signal });
    search.addEventListener("input", () => {
      applyFilter(state);
      moveTo(state, firstVisible(state));
    }, { signal });
  }
  list.addEventListener("click", (event) => {
    const item = event.target.closest?.(".nd-select-item");
    if (!item || item.getAttribute("aria-disabled") === "true") return;
    takeOption(state, state.rows.findIndex((row) => row.item === item));
  }, { signal });
  // A pointer moving over the list moves the highlight with it, so the keyboard
  // and the mouse never disagree about which option Enter would take.
  list.addEventListener("pointermove", (event) => {
    const item = event.target.closest?.(".nd-select-item");
    if (item) moveTo(state, state.rows.findIndex((row) => row.item === item), false);
  }, { signal });
  const clear = box.querySelector("[data-nd-select-clear]");
  if (clear) {
    clear.addEventListener("click", () => {
      for (const option of state.native.options) option.selected = false;
      // A single select always has SOMETHING selected as far as the DOM is
      // concerned, so clearing means choosing the empty option if there is one.
      if (!state.native.multiple) state.native.selectedIndex = -1;
      announce(state);
      paint(state);
      state.control.focus();
    }, { signal });
  }
  // Something else changed the answer — a second control, a test.
  native.addEventListener("change", () => paint(state), { signal });
  wireForm(state);

  rebuild(state);
  paint(state);
}

/**
 * The two things a form does to a control that it does not announce.
 *
 * A RESET FIRES NO `change`. `form.reset()` dispatches one `reset` event on the
 * form and nothing at all on the controls inside it, so the listener above
 * never runs and the enhanced control goes on showing an answer the native
 * select no longer holds. Measured: after a reset the control read
 * `ochre-balanced` while the select held nothing.
 *
 * And the event fires BEFORE the reset happens, and is cancelable — so the
 * repaint has to be deferred by a task, not done in the handler.
 *
 * A REQUIRED CONTROL THAT IS `display: none` CANNOT BE REPORTED. The browser
 * has to focus an invalid control to show its bubble; `native.hidden` means it
 * cannot, so Chrome logs *"An invalid form control with name='…' is not
 * focusable"* and the submit silently does nothing, and Firefox blocks it with
 * nothing said at all. `Select { multiple, required }` has shipped in the
 * examples the whole time and was never seen, because no example is inside a
 * `<form>`. The validity stays on the native control — it is the value, and it
 * is what a server-side check would agree with — and the REPORT moves to the
 * button, which is the focusable thing the reader can actually see.
 */
function wireForm(state) {
  const form = state.native.form;
  if (!form) return;

  form.addEventListener("reset", () => {
    setTimeout(() => {
      for (const option of [...state.native.options]) {
        if (option.dataset.ndCreated !== undefined) option.remove();
      }
      rebuild(state);
      paint(state);
    }, 0);
  }, { signal: state.signal });

  // `invalid` does not bubble, so it is caught on the way down.
  form.addEventListener(
    "invalid",
    (event) => {
      if (event.target !== state.native) return;
      // The browser's attempt to show a bubble is what logs the error, and it
      // was never going to succeed. Refuse it, and say the same sentence in the
      // place this component already says sentences — the browser's own
      // `validationMessage`, so it is in the reader's language without this
      // file carrying a word of it.
      event.preventDefault();
      if (state.saidByCaller) return;
      const slot = state.field.closest(".nd-field")?.querySelector("[data-nd-field-error]");
      saySentence(slot, state.native.validationMessage);
      state.field.dataset.ndInvalid = "true";
      state.control.setAttribute("aria-invalid", "true");
      // And focus lands on the control the reader can see, which is the whole
      // thing the browser could not do. Deferred by a task: the browser makes
      // its own attempt to focus the invalid control after this event returns,
      // fails, and leaves focus wherever it was — so a synchronous call here
      // is overwritten by a failure.
      //
      // The control, not `focusHolder`: the filter box is inside a panel that
      // is closed at this moment, and `.focus()` on a `display: none` element
      // is a silent no-op — which is the same class of failure this whole
      // repair is about.
      setTimeout(() => state.control.focus(), 0);
    },
    true, { signal: state.signal },
  );

  // Whatever is said stops being true the moment the answer changes.
  state.native.addEventListener("change", () => {
    if (state.native.checkValidity()) clearComplaint(state);
  }, { signal: state.signal });
  form.addEventListener("reset", () => setTimeout(() => clearComplaint(state), 0), { signal: state.signal });
}

/**
 * Takes back the browser's complaint, and the flags that went with it.
 *
 * Only ever what the script wrote. An `error` prop is the caller's sentence —
 * it is usually the one a server sent back — and clearing it because the
 * browser is now satisfied would delete an answer the browser never had.
 */
function clearComplaint(state) {
  if (state.saidByCaller) return;
  const slot = state.field.closest(".nd-field")?.querySelector("[data-nd-field-error]");
  saySentence(slot, "");
  state.field.dataset.ndInvalid = "false";
  state.control.removeAttribute("aria-invalid");
}

/** The rows, built from the options the native control actually has. */
function rebuild(state) {
  state.list.textContent = "";
  state.rows = [];
  const loading = state.field.dataset.ndLoading === "true";
  if (loading) {
    say(state, state.field.dataset.ndLoadingLabel);
    return;
  }
  let index = 0;
  const row = (option, into, groupDisabled) => {
    // The placeholder is not an answer, so it is not an option to choose. The
    // way back to nothing is `clearable`, which says so with a control.
    if (option.value === "") return;
    const item = document.createElement("li");
    item.className = "nd-select-item";
    item.id = `${state.id}-option-${index}`;
    item.setAttribute("role", "option");
    item.dataset.value = option.value;
    if (option.disabled || groupDisabled) item.setAttribute("aria-disabled", "true");

    if (option.dataset.ndSwatch) {
      // `Swatch`'s own mechanism: a chip that declares a palette and takes its
      // colour from it, so no colour is named here either.
      const swatch = document.createElement("span");
      swatch.className = "nd-swatch";
      swatch.dataset.palette = option.dataset.ndSwatch;
      swatch.dataset.ndSize = "small";
      swatch.dataset.ndShape = "pill";
      swatch.setAttribute("aria-hidden", "true");
      item.append(swatch);
    }
    const glyph = option.dataset.ndIcon && template(state, option.dataset.ndIcon);
    if (glyph) item.append(glyph);

    const text = document.createElement("span");
    text.className = "nd-select-item-text";
    const name = document.createElement("span");
    name.className = "nd-select-item-label";
    name.textContent = option.textContent;
    text.append(name);
    if (option.dataset.ndDetail) {
      // Inside the option and therefore part of its name: a row that shows two
      // lines and announces one has told a screen-reader user less than the
      // page shows, which is the failure a second line usually introduces.
      const detail = document.createElement("span");
      detail.className = "nd-select-item-detail";
      detail.textContent = option.dataset.ndDetail;
      text.append(detail);
    }
    item.append(text);
    const tick = template(state, "check");
    if (tick) {
      const mark = document.createElement("span");
      mark.className = "nd-select-item-mark";
      mark.setAttribute("aria-hidden", "true");
      mark.append(tick);
      item.append(mark);
    }
    into.append(item);
    state.rows.push({ item, option });
    index += 1;
  };

  for (const child of state.native.children) {
    if (child.tagName === "OPTGROUP") {
      // `role="group"` inside the listbox, with the run's own `<ul>` marked
      // `none` so its options are owned by the group rather than by a second
      // list. A `<li>` cannot hold a `<li>`; this is the shape that can.
      const group = document.createElement("li");
      group.className = "nd-select-group-row";
      group.setAttribute("role", "group");
      group.setAttribute("aria-label", child.label);
      const heading = document.createElement("span");
      heading.className = "nd-select-group-label";
      heading.setAttribute("aria-hidden", "true");
      heading.textContent = child.label;
      group.append(heading);
      const inner = document.createElement("ul");
      inner.className = "nd-select-group-list";
      inner.setAttribute("role", "none");
      group.append(inner);
      state.list.append(group);
      for (const option of child.children) row(option, inner, child.disabled);
    } else if (child.tagName === "OPTION") {
      row(child, state.list, false);
    }
  }
}

/** A drawing the markup already carries, cloned. This file ships none. */
function template(state, glyph) {
  const found = state.field.querySelector(`template[data-nd-glyph="${CSS.escape(glyph)}"]`);
  return found ? found.content.cloneNode(true) : null;
}

/** The one line the panel says when it has no list to show. */
function say(state, words) {
  state.note.textContent = words ?? "";
  state.note.hidden = !words;
}

/** What the closed control reads, and which rows are marked chosen. */
function paint(state) {
  const chosen = [...state.native.selectedOptions].filter((option) => option.value !== "");
  const value = state.control.querySelector(".nd-select-value");
  if (chosen.length === 0) {
    // FROM THE ATTRIBUTE, NOT FROM THE EMPTY OPTION.
    //
    // A `multiple` select carries no `<option value="">` — nothing chosen
    // already submits nothing — so reading the empty state off one left the
    // control saying nothing at all, which reads as broken rather than as
    // unanswered. The component writes the word where both kinds can find it.
    value.textContent = state.field.dataset.ndPlaceholder ?? "";
    value.dataset.ndEmpty = "true";
  } else if (chosen.length === 1) {
    value.textContent = chosen[0].textContent;
    delete value.dataset.ndEmpty;
  } else {
    // A list of eleven names does not fit in a control, and a control that
    // shows three and hides eight has lied about the answer.
    value.textContent = message(state.field.dataset.ndSelectedLabel ?? "", {
      count: chosen.length,
    });
    delete value.dataset.ndEmpty;
  }
  state.box.dataset.ndFilled = String(chosen.length > 0);
  for (const { item, option } of state.rows) {
    if (option) item.setAttribute("aria-selected", String(option.selected));
  }
}

/** Tells anything listening that the answer moved, as a real change would. */
function announce(state) {
  state.native.dispatchEvent(new Event("input", { bubbles: true }));
  state.native.dispatchEvent(new Event("change", { bubbles: true }));
}

function visibleRows(state) {
  return state.rows.filter((row) => !row.item.hidden);
}

function firstVisible(state) {
  const chosen = state.rows.findIndex((row) => row.option?.selected && !row.item.hidden);
  if (chosen >= 0) return chosen;
  return state.rows.findIndex((row) => !row.item.hidden);
}

/** Hides the rows the typed term does not reach, and says so if none is left. */
function applyFilter(state) {
  const term = (state.search?.value ?? "").trim().toLowerCase();
  // Last pass's offer, gone before this pass decides whether to make one.
  for (const row of state.rows.filter((row) => !row.option)) row.item.remove();
  state.rows = state.rows.filter((row) => row.option);
  for (const { item, option } of state.rows) {
    const haystack = `${option.textContent} ${option.dataset.ndDetail ?? ""}`.toLowerCase();
    item.hidden = term !== "" && !haystack.includes(term);
  }
  // A group with nothing left in it is a heading over a gap.
  for (const group of state.list.querySelectorAll(".nd-select-group-row")) {
    group.hidden = ![...group.querySelectorAll(".nd-select-item")].some((item) => !item.hidden);
  }
  // See `enhanceSelect`: a filter box is what makes this mean anything.
  const creatable = state.creatable;
  // BY VALUE AS WELL AS BY LABEL, AND THE VALUE IS THE ONE THAT MATTERS.
  //
  // This compared `textContent` only, while `create()` writes the term into
  // BOTH `value` and `textContent` — so wherever the two differ, and the
  // shipped example `SelectOption { value: "ochre-balanced", label: "Ochre" }`
  // is exactly that, typing the value found no label to match, offered to
  // create, and appended a SECOND option carrying a value the control already
  // had. The form then submitted `palette=ochre-balanced` twice from one
  // single-valued select. Measured, not theorised.
  const exact = state.rows.some(
    (row) => sameValue(row.option.textContent, term) || sameValue(row.option.value, term),
  );
  const offer = creatable && term !== "" && !exact;
  if (offer) {
    const item = document.createElement("li");
    item.className = "nd-select-item";
    item.id = `${state.id}-create`;
    item.setAttribute("role", "option");
    item.setAttribute("aria-selected", "false");
    item.dataset.ndCreate = "";
    const text = document.createElement("span");
    text.className = "nd-select-item-text";
    text.textContent = message(state.field.dataset.ndCreateLabel ?? "", {
      term: state.search.value.trim(),
    });
    item.append(text);
    state.list.append(item);
    // A row like any other, so the arrow keys reach it and Enter takes it.
    state.rows.push({ item, option: null });
  }
  say(state, visibleRows(state).length === 0 ? state.field.dataset.ndEmptyLabel : "");
}

/**
 * Whether two option strings are the same answer.
 *
 * `Intl.Collator` rather than `toLowerCase()`: that is locale-independent and
 * gets Turkish dotted and dotless I wrong, and it does not fold the difference
 * between `é` typed as one code point and `é` pasted as two — which are
 * different strings, identical on the screen, and would become two options
 * nobody can tell apart. `base` sensitivity ignores case and accent, which is
 * what "the reader meant the same thing" means here.
 */
const SAME = new Intl.Collator(undefined, { sensitivity: "base", usage: "search" });
const sameValue = (a, b) => SAME.compare((a ?? "").trim(), (b ?? "").trim()) === 0;

/** Adds what was typed to the REAL select, which is what makes it submittable. */
function create(state) {
  const term = state.search.value.trim().normalize("NFC");
  if (!term) return;
  // The offer should never have been drawn for a value that exists, and this
  // is the second half of that: `applyFilter` decides what to show, and the
  // store refuses a duplicate whatever anything upstream believed.
  const existing = [...state.native.options].find((option) => sameValue(option.value, term));
  if (existing) {
    existing.selected = true;
    state.search.value = "";
    rebuild(state);
    applyFilter(state);
    announce(state);
    paint(state);
    if (!state.native.multiple) closeList(state);
    return;
  }
  const option = document.createElement("option");
  option.value = term;
  option.textContent = term;
  // WHAT THE SCRIPT ADDED, MARKED AS SUCH — so a reset can take it away again.
  // `form.reset()` restores `defaultSelected`, which is `false` on anything
  // created here, so without this the option is deselected and then lives in
  // the list for ever, reappearing as a suggestion nobody declared and growing
  // by one on every reset.
  option.dataset.ndCreated = "";
  state.native.append(option);
  option.selected = true;
  state.search.value = "";
  rebuild(state);
  applyFilter(state);
  announce(state);
  paint(state);
  if (!state.native.multiple) closeList(state);
}

/** Takes an option, and closes unless several may be chosen. */
function takeOption(state, index) {
  const row = state.rows[index];
  if (!row || row.item.getAttribute("aria-disabled") === "true") return;
  // The offer to add what was typed is a row with nothing behind it yet.
  if (!row.option) {
    create(state);
    return;
  }
  if (state.native.multiple) {
    row.option.selected = !row.option.selected;
  } else {
    for (const option of state.native.options) option.selected = false;
    row.option.selected = true;
  }
  announce(state);
  paint(state);
  // Several may be chosen means several will be, and closing after each one is
  // a reader reopening the list for every answer they have.
  if (!state.native.multiple) closeList(state);
}

/** Moves the highlight, and says where it went. */
function moveTo(state, index, scroll = true) {
  if (index < 0 || index >= state.rows.length) return;
  for (const { item } of state.rows) delete item.dataset.ndActive;
  const item = state.rows[index].item;
  item.dataset.ndActive = "";
  state.active = index;
  focusHolder(state).setAttribute("aria-activedescendant", item.id);
  if (scroll) item.scrollIntoView({ block: "nearest" });
}

/** The element the keyboard is on, which is the one that owns the highlight. */
function focusHolder(state) {
  return state.search ?? state.control;
}

function step(state, delta) {
  const rows = visibleRows(state);
  if (rows.length === 0) return;
  const here = rows.findIndex((row) => row === state.rows[state.active]);
  const next = here < 0 ? (delta > 0 ? 0 : rows.length - 1) : (here + delta + rows.length) % rows.length;
  moveTo(state, state.rows.indexOf(rows[next]));
}

function openList(state) {
  // Through the shared registry, so a `TagPicker`'s list closes this one and
  // the other way round.
  state.close ??= () => closeList(state, false);
  claimOpenPanel(state.close);
  openSelect = state;
  state.panel.hidden = false;
  state.control.setAttribute("aria-expanded", "true");
  if (state.search) {
    state.search.value = "";
    applyFilter(state);
    state.search.focus();
  }
  // The host's turn: a page that must fetch something before the list is
  // legible hears this, and the component does not need to know what.
  state.field.dispatchEvent(new CustomEvent("nd-select-open", { bubbles: true }));
  anchorSelect(state);
  const at = firstVisible(state);
  if (at >= 0) moveTo(state, at);
}

function closeList(state, restore = true) {
  if (state.panel.hidden) return;
  releaseAnchor(state.panel);
  state.panel.hidden = true;
  state.control.setAttribute("aria-expanded", "false");
  focusHolder(state).removeAttribute("aria-activedescendant");
  if (openSelect === state) openSelect = null;
  if (state.close) releaseOpenPanel(state.close);
  if (restore) state.control.focus();
}

function onSelectKey(state, event) {
  const open = !state.panel.hidden;
  switch (event.key) {
    case "ArrowDown":
    case "ArrowUp":
      event.preventDefault();
      if (!open) openList(state);
      else step(state, event.key === "ArrowDown" ? 1 : -1);
      return;
    case "Home":
    case "End": {
      if (!open) return;
      // With something TYPED these are the caret's, not the list's: a reader
      // fixing a typo at the start of what they typed is not asking for the
      // first option. With the box empty there is no typo and no caret worth
      // keeping, so the keys reach the list — which is what the contract
      // promises and what an empty filter box used to swallow.
      if (state.search && state.search.value !== "") return;
      event.preventDefault();
      const rows = visibleRows(state);
      if (rows.length) moveTo(state, state.rows.indexOf(event.key === "Home" ? rows[0] : rows.at(-1)));
      return;
    }
    case "Enter":
      if (!open) {
        event.preventDefault();
        openList(state);
        return;
      }
      event.preventDefault();
      takeOption(state, state.active);
      return;
    case " ":
      // A FILTER BOX NEEDS ITS SPACES, AND THAT IS NOT NEGOTIABLE. Which
      // leaves a `multiple` searchable select with no Space, so Enter is what
      // chooses without closing there — `takeOption` keeps the list open
      // whenever several may be chosen, whichever key opened it. The keyboard
      // table says so now rather than promising a key this branch refuses.
      if (state.search) return;
      event.preventDefault();
      if (!open) openList(state);
      else takeOption(state, state.active);
      return;
    case "Escape":
      if (!open) return;
      event.preventDefault();
      closeList(state);
      return;
    case "Tab":
      // Closes and leaves. What was highlighted is NOT chosen by leaving, which
      // is the difference between a combobox and a radiogroup.
      if (open) closeList(state, false);
      return;
    default:
      break;
  }
  // Type-ahead, for the list with no filter box. One printable character at a
  // time, accumulated while they keep coming.
  if (state.search || event.key.length !== 1 || event.metaKey || event.ctrlKey || event.altKey) {
    return;
  }
  const now = performance.now();
  state.typed = now - state.typedAt > TYPE_AHEAD ? event.key : state.typed + event.key;
  state.typedAt = now;
  const term = state.typed.toLowerCase();
  const at = state.rows.findIndex((row) => row.option?.textContent.toLowerCase().startsWith(term));
  if (at < 0) return;
  event.preventDefault();
  if (!open) openList(state);
  moveTo(state, at);
}

/** The Select's panel: the second of `anchorPanel`'s three call sites. */
function anchorSelect(state) {
  keepAnchored(state.panel, state.control, { width: "--nd-select-width" }, () =>
    closeList(state, false),
  );
}

// A click anywhere else closes it. Not `blur`: a click on the panel's own
// scrollbar blurs the input, and a list that closed when you reached for its
// scrollbar would be unusable with a mouse.
document.addEventListener("pointerdown", (event) => {
  if (!openSelect) return;
  const field = event.target.closest?.(SELECT_FIELD);
  if (field !== openSelect.field) closeList(openSelect, false);
});

// The scroll and resize listeners the Select used to carry are gone: the
// `anchored` registry above holds one of each for every floating panel, and it
// is what closes this one when its control leaves the window.

