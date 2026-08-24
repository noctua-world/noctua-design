// Everything a `TagPicker` does once a script is running.
//
// IT SHIPS, for the reason `menu.js` gives: a component whose consumer cannot
// use it is a component that lies. A picker with no script is a text field and
// a datalist — honest, and not what the component announces.
//
// THE ANCHORING IS NOT WRITTEN HERE, and neither is which panel may be open:
// `anchor.js` places the suggestions and `panels.js` decides that only one
// floating list on the page is open at a time.

import { keepAnchored, releaseAnchor } from "./anchor.js";
import { claimOpenPanel, releaseOpenPanel } from "./panels.js";
import { message } from "./messages.js";

/** How long to wait before saying what changed, so typing is not narrated. */
const ANNOUNCE_AFTER = 400;

/** Two values are the same tag when a reader would call them the same word. */
const SAME = new Intl.Collator(undefined, { sensitivity: "base", usage: "search" });
const sameValue = (a, b) => SAME.compare((a ?? "").trim(), (b ?? "").trim()) === 0;


// SEVERAL VALUES A READER WRITES, ON A REAL `<select multiple>`.
//
// The store is the browser's own — `selectedOptions`, with no mirror — so a
// value cannot be committed twice: the option set IS the index. A typed value
// becomes a real `<option selected>`, exactly as `Select`'s `create()` does,
// and what reaches a server is what the browser sends.
//
// THE THREE THINGS THIS FILE MAY NOT DO, and each was a temptation here:
//
// 1. It may not know what a `Tag` looks like. The picker ships one inside a
//    `<template>` and this clones it — the generalisation of the
//    `<template data-nd-glyph>` `Select` already ships for its tick. A second
//    place that knew a component's markup would drift from the emitter, which
//    is the whole thing the emitter exists to prevent.
// 2. It may not carry a word. Every sentence is a prop with a `data-nd-*`
//    twin, filled through `fillMessage` like every other one here.
// 3. It may not be the only way this works. The composer ships a real `name`
//    and the first thing done below is to remove it, so a reader with no
//    script picks from the native listbox AND types one free value.

const PICKER = "[data-nd-tag-picker]";
const pickers = new WeakMap();

export function enhanceTagPickers(root = document) {
  for (const root_ of root.querySelectorAll(PICKER)) {
    if (pickers.has(root_)) continue;
    const state = buildPicker(root_);
    if (state) pickers.set(root_, state);
  }
}

/**
 * Takes the behaviour back off every enhanced picker under `root`.
 *
 * A BEHAVIOUR THAT CANNOT BE RELEASED IS A DEFECT FOR EVERY CONSUMER WITH A
 * CLIENT-SIDE ROUTER. `buildPicker` registers a `pointerdown` on the DOCUMENT
 * per picker, so a page that mounts the same picker ten times carries ten of
 * them, each holding a detached composer alive through its closure.
 */
export function releaseTagPickers(root = document) {
  for (const element of root.querySelectorAll(PICKER)) {
    const state = pickers.get(element);
    if (!state) continue;
    if (state.close) releaseOpenPanel(state.close);
    releaseAnchor(state.panel);
    state.binding.abort();
    pickers.delete(element);
    delete element.dataset.ndEnhanced;
  }
}

function buildPicker(root_) {
  const binding = new AbortController();
  const signal = binding.signal;
  const native = root_.querySelector("[data-nd-tag-picker-native]");
  const box = root_.querySelector("[data-nd-tag-picker-input]");
  const list = root_.querySelector("[data-nd-tag-picker-tags]");
  const shape = root_.querySelector("[data-nd-tag-template]");
  if (!native || !box || !list || !shape) return null;

  const name = root_.dataset.ndTagPicker;
  const id = `nd-tag-picker-${name}`;
  const state = {
    // CARRIED ON THE STATE, because every helper below already takes it. A
    // parallel argument threaded through a dozen call sites is a dozen places
    // for one of them to be forgotten.
    binding,
    signal,
    root: root_,
    native,
    box,
    list,
    shape,
    id,
    note: root_.querySelector("[data-nd-tag-picker-status]"),
    clear: root_.querySelector("[data-nd-tag-picker-clear]"),
    separators: [...(root_.dataset.ndSeparators ?? ",")],
    creatable: root_.dataset.ndCreatable === "true",
    paste: root_.dataset.ndPaste === "true",
    duplicates: root_.dataset.ndDuplicates === "true",
    max: Number.parseInt(root_.dataset.ndMax ?? "", 10) || 0,
    pattern: root_.dataset.ndPattern ?? "",
    // Read from the composer, which is where the component puts it, and read
    // ONCE — `paintTags` writes the same property and would otherwise be
    // asking a question it had already answered.
    required: box.required,
    words: {
      create: root_.dataset.ndCreateLabel ?? "",
      max: root_.dataset.ndMaxLabel ?? "",
      invalid: root_.dataset.ndInvalidLabel ?? "",
      remove: root_.dataset.ndRemoveLabel ?? "",
      added: root_.dataset.ndAddedLabel ?? "",
      removed: root_.dataset.ndRemovedLabel ?? "",
      status: root_.dataset.ndStatus ?? "",
      empty: root_.dataset.ndEmptyLabel ?? "",
      loading: root_.dataset.ndLoadingLabel ?? "",
    },
    rows: [],
    active: -1,
    said: 0,
  };

  // THE COMPOSER STOPS SUBMITTING, AND THE SELECT STARTS BEING INVISIBLE.
  //
  // Both in the same breath, because they are one arrangement: with no script
  // the reader answers through a listbox and a text field, and with one they
  // answer through this. Two fields under one name after enhancement would
  // post the composer's leftovers beside the committed values.
  box.removeAttribute("name");
  native.hidden = true;

  const panel = document.createElement("div");
  panel.className = "nd-panel nd-tag-picker-panel";
  panel.hidden = true;
  const rows = document.createElement("ul");
  rows.className = "nd-tag-picker-list";
  rows.id = `${id}-listbox`;
  rows.setAttribute("role", "listbox");
  rows.setAttribute(
    "aria-label",
    root_.closest(".nd-field")?.querySelector(".nd-label")?.textContent ?? "",
  );
  const note = document.createElement("p");
  note.className = "nd-tag-picker-empty";
  panel.append(rows, note);
  root_.querySelector(".nd-tag-picker-field").append(panel);
  state.panel = panel;
  state.list_ = rows;
  state.empty = note;
  state.close = () => closePicker(state);

  // Every combobox attribute, written here and never by the markup: with no
  // script there is no popup, and a box claiming to control a listbox nothing
  // can open is a promise the markup does not keep.
  box.setAttribute("role", "combobox");
  box.setAttribute("aria-autocomplete", "list");
  box.setAttribute("aria-expanded", "false");
  box.setAttribute("aria-controls", rows.id);

  box.addEventListener("input", () => {
    filterPicker(state);
    openPicker(state);
  }, { signal });
  box.addEventListener("keydown", (event) => onPickerKey(state, event), { signal });
  box.addEventListener("paste", (event) => onPickerPaste(state, event), { signal });
  // Whatever is half-typed when the form goes is a value the reader meant.
  // This is the single most-complained-about bug in every token input ever
  // shipped, and it costs two lines.
  box.addEventListener("blur", () => commit(state, box.value, { quiet: true }), { signal });
  native.form?.addEventListener("submit", () => commit(state, box.value, { quiet: true }), { signal });
  native.form?.addEventListener("reset", () => {
    setTimeout(() => {
      for (const option of [...native.options]) {
        if (option.dataset.ndCreated !== undefined) option.remove();
      }
      paintTags(state);
    }, 0);
  }, { signal });

  state.clear?.addEventListener("click", () => {
    for (const option of native.selectedOptions) option.selected = false;
    announcePicker(state);
    paintTags(state);
    box.focus();
  }, { signal });

  // Not `blur`: a click on the panel's own scrollbar blurs the composer, and a
  // list that closed when you reached for its scrollbar would be unusable.
  document.addEventListener("pointerdown", (event) => {
    if (!event.target.closest?.(PICKER)) closePicker(state);
    else if (event.target.closest(PICKER) !== root_) closePicker(state);
  }, { signal });

  root_.dataset.ndEnhanced = "true";
  rebuildPicker(state);
  paintTags(state);
  return state;
}

/** The suggestion rows, from the options the native control actually has. */
function rebuildPicker(state) {
  state.list_.textContent = "";
  state.rows = [];
  if (state.root.dataset.ndLoading === "true") {
    state.empty.textContent = state.words.loading;
    return;
  }
  let index = 0;
  for (const option of state.native.options) {
    if (option.value === "" || option.selected) continue;
    const item = document.createElement("li");
    item.className = "nd-tag-picker-item";
    item.id = `${state.id}-option-${index}`;
    item.setAttribute("role", "option");
    item.textContent = option.textContent;
    item.addEventListener("pointerdown", (event) => {
      // Before the composer can blur, so the click is not a race.
      event.preventDefault();
      commit(state, option.value);
    }, { signal: state.signal });
    state.list_.append(item);
    state.rows.push({ item, option });
    index += 1;
  }
}

/** Hides what does not match, and offers to add what was typed. */
function filterPicker(state) {
  rebuildPicker(state);
  const term = state.box.value.trim();
  const folded = term.toLocaleLowerCase();
  for (const { item, option } of state.rows) {
    item.hidden = term !== "" && !option.textContent.toLocaleLowerCase().includes(folded);
  }
  const shown = state.rows.filter((row) => !row.item.hidden);
  // The offer to add what was typed, when nothing already IS it. Compared with
  // the same collator the store uses, so the row a reader is offered and the
  // value the store refuses are the same question.
  const exact = [...state.native.options].some((option) => sameValue(option.value, term));
  if (state.creatable && term !== "" && !exact) {
    const item = document.createElement("li");
    item.className = "nd-tag-picker-item";
    item.id = `${state.id}-create`;
    item.setAttribute("role", "option");
    item.dataset.ndCreate = "";
    item.textContent = message(state.words.create, { term });
    item.addEventListener("pointerdown", (event) => {
      event.preventDefault();
      commit(state, term);
    }, { signal: state.signal });
    state.list_.append(item);
    state.rows.push({ item, option: null });
  }
  state.empty.textContent = shown.length === 0 && !state.creatable ? state.words.empty : "";
  state.active = -1;
  state.box.removeAttribute("aria-activedescendant");
}

/**
 * Adds one value to the store, which is the real `<select>`.
 *
 * Everything a value has to survive is here, and every one of them was a way
 * this goes wrong: NFC so `é` typed and `é` pasted are one tag rather than two
 * that look identical; a stricter emptiness test than `trim()`, which does not
 * catch a zero-width space; the pattern the caller asked for; the ceiling; and
 * a duplicate check against the option set rather than against a second list.
 */
function commit(state, raw, { quiet = false } = {}) {
  const term = (raw ?? "").normalize("NFC").replace(/^[\s​﻿]+|[\s​﻿]+$/gu, "");
  if (term === "") return false;

  if (state.pattern) {
    let ok = false;
    try {
      ok = new RegExp(`^(?:${state.pattern})$`, "u").test(term);
    } catch {
      // A pattern this engine will not compile refuses nothing rather than
      // refusing everything: a caller's typo must not make a field unusable.
      ok = true;
    }
    if (!ok) {
      sayPicker(state, message(state.words.invalid, { term }));
      return false;
    }
  }

  const chosen = [...state.native.selectedOptions].filter((option) => option.value !== "");
  if (state.max > 0 && chosen.length >= state.max) {
    sayPicker(state, message(state.words.max, { max: state.max }, state.max));
    return false;
  }

  const existing = [...state.native.options].find((option) => sameValue(option.value, term));
  if (existing) {
    if (existing.selected && !state.duplicates) {
      state.box.value = "";
      filterPicker(state);
      closePicker(state);
      return false;
    }
    existing.selected = true;
    // MOVED TO THE END, so the order is the order the reader chose in.
    //
    // `selectedOptions` is DOM order, and a declared suggestion sits above
    // everything typed — so choosing `rust` after typing three words put it
    // FIRST in the tags and first in what the form posts. Both are the reader's
    // list, and neither should be sorted by where a suggestion happened to be
    // declared. The native listbox is hidden by now, so nothing a reader can
    // see is reordered.
    state.native.append(existing);
  } else {
    const option = document.createElement("option");
    option.value = term;
    option.textContent = term;
    // Marked, so a reset can take it away again: `defaultSelected` is false on
    // anything created here, so without this the option is deselected and then
    // lives in the list for ever.
    option.dataset.ndCreated = "";
    state.native.append(option);
    option.selected = true;
  }

  state.box.value = "";
  if (!quiet) announcePicker(state);
  paintTags(state);
  filterPicker(state);
  closePicker(state);
  if (!quiet) sayPicker(state, message(state.words.added, { label: term }));
  return true;
}

/** Takes one value off, and puts focus somewhere a reader can find it. */
function remove(state, option, from) {
  const label = option.textContent;
  option.selected = false;
  if (option.dataset.ndCreated !== undefined) option.remove();
  announcePicker(state);
  paintTags(state);
  filterPicker(state);
  sayPicker(state, message(state.words.removed, { label }));

  // THE CONTROL THAT DID THIS HAS JUST LEFT THE PAGE. Focus falling to `<body>`
  // puts a screen-reader user back at the top of the document with no idea why,
  // so it goes to the next tag's control or to the composer.
  const buttons = [...state.list.querySelectorAll(".nd-tag-remove")];
  const next = buttons[Math.min(from, buttons.length - 1)];
  (next ?? state.box).focus();
}

/**
 * Draws the tags, by cloning the one the component shipped.
 *
 * This file does not know what a tag looks like — see the header. The template
 * holds one empty `Tag` with its remove control already named `{label}`, and
 * everything here is filling two text nodes in a copy of it.
 */
function paintTags(state) {
  const chosen = [...state.native.selectedOptions].filter((option) => option.value !== "");
  state.list.textContent = "";
  chosen.forEach((option, at) => {
    const tag = state.shape.content.firstElementChild.cloneNode(true);
    const label = option.textContent;
    tag.querySelector(".nd-tag-label").textContent = label;
    tag.dataset.ndValue = option.value;
    const cross = tag.querySelector(".nd-tag-remove");
    if (cross) {
      const named = message(state.words.remove, { label });
      cross.setAttribute("aria-label", named);
      cross.setAttribute("title", named);
      cross.value = option.value;
      // NOT A TAB STOP. Forty tags would be forty presses between the composer
      // and the next control; the list is one stop and the arrows walk it.
      cross.tabIndex = -1;
      cross.addEventListener("click", () => remove(state, option, at), { signal: state.signal });
    }
    const item = document.createElement("li");
    item.append(tag);
    state.list.append(item);
  });
  state.list.tabIndex = chosen.length > 0 ? 0 : -1;
  // AND THE COMPOSER STOPS BEING COMPULSORY ONCE THERE IS AN ANSWER.
  //
  // `required` sits on the composer, because the browser has to focus an
  // invalid control to anchor its bubble and the real `<select>` is hidden.
  // Every commit clears the composer and nothing took the attribute off, so a
  // reader who entered three tags met "please fill in this field" pointing at
  // an empty box they had deliberately emptied — the field was unsubmittable
  // exactly when it had been answered. The requirement is about the field, so
  // it follows the tags rather than the box.
  if (state.required) state.box.required = chosen.length === 0;
  state.root.toggleAttribute("data-nd-filled", chosen.length > 0);
  if (state.clear) state.clear.hidden = chosen.length === 0;
  // The standing sentence. `commit` and `remove` say something more specific
  // right after this and win, because `sayPicker` replaces rather than queues —
  // one element, one sentence, the most recent thing worth saying, which is the
  // shape `Search`'s note already has.
  state.root.dataset.ndCount = String(chosen.length);
  // NOTHING TO REPORT IS REPORTED BY AN ABSENCE. A field with no tags in it
  // saying *0 topics* is a page telling a reader about something that has not
  // happened — the same argument `Badge { show_zero }` makes, and the same one
  // `Search` makes by saying nothing until there is a query.
  sayPicker(
    state,
    chosen.length === 0
      ? ""
      : message(state.words.status, { count: chosen.length }, chosen.length),
  );
}

/** The one sentence, at most one per pause. */
function sayPicker(state, words) {
  if (!state.note) return;
  clearTimeout(state.said);
  if (!words) {
    state.note.textContent = "";
    state.said = 0;
    return;
  }
  state.said = setTimeout(() => {
    state.note.textContent = words;
  }, ANNOUNCE_AFTER);
}

/** Real events on the real control, so a consumer's listener runs. */
function announcePicker(state) {
  state.native.dispatchEvent(new Event("input", { bubbles: true }));
  state.native.dispatchEvent(new Event("change", { bubbles: true }));
}

function pickerRows(state) {
  return state.rows.filter((row) => !row.item.hidden);
}

function movePicker(state, delta) {
  const rows = pickerRows(state);
  if (rows.length === 0) return;
  const here = rows.findIndex((row) => row === state.rows[state.active]);
  const next = here < 0 ? (delta > 0 ? 0 : rows.length - 1) : (here + delta + rows.length) % rows.length;
  setPickerActive(state, state.rows.indexOf(rows[next]));
}

function setPickerActive(state, index) {
  for (const { item } of state.rows) item.removeAttribute("aria-selected");
  state.active = index;
  const row = state.rows[index];
  if (!row) {
    state.box.removeAttribute("aria-activedescendant");
    return;
  }
  row.item.setAttribute("aria-selected", "true");
  state.box.setAttribute("aria-activedescendant", row.item.id);
  row.item.scrollIntoView({ block: "nearest" });
}

function openPicker(state) {
  if (!state.panel.hidden) return;
  if (pickerRows(state).length === 0 && !state.empty.textContent) return;
  claimOpenPanel(state.close);
  state.panel.hidden = false;
  state.box.setAttribute("aria-expanded", "true");
  keepAnchored(state.panel, state.root.querySelector(".nd-tag-picker-field"), {
    width: "--nd-tag-picker-width",
  }, state.close);
}

function closePicker(state) {
  if (state.panel.hidden) return;
  releaseAnchor(state.panel);
  releaseOpenPanel(state.close);
  state.panel.hidden = true;
  state.box.setAttribute("aria-expanded", "false");
  state.box.removeAttribute("aria-activedescendant");
  state.active = -1;
}

function onPickerKey(state, event) {
  if (event.isComposing) return;

  // A separator is a keystroke, never an encoding — which is the whole reason
  // this is a `<select multiple>` and not one string with commas in it.
  if (state.separators.includes(event.key)) {
    event.preventDefault();
    commit(state, state.box.value);
    return;
  }

  switch (event.key) {
    case "Enter": {
      const row = state.rows[state.active];
      if (row) {
        event.preventDefault();
        commit(state, row.option ? row.option.value : state.box.value);
        return;
      }
      // With something typed, Enter commits it rather than submitting: a
      // reader whose last keystroke was Enter did not mean to send the page.
      if (state.box.value.trim() !== "") {
        event.preventDefault();
        commit(state, state.box.value);
      }
      return;
    }
    case "Backspace": {
      if (state.box.value !== "") return;
      const chosen = [...state.native.selectedOptions].filter((option) => option.value !== "");
      const last = chosen.at(-1);
      if (last) {
        event.preventDefault();
        remove(state, last, chosen.length - 1);
      }
      return;
    }
    case "ArrowDown":
      event.preventDefault();
      openPicker(state);
      movePicker(state, 1);
      return;
    case "ArrowUp":
      event.preventDefault();
      openPicker(state);
      movePicker(state, -1);
      return;
    case "Escape":
      if (state.panel.hidden) return;
      event.preventDefault();
      closePicker(state);
      return;
    default:
  }
}

/**
 * A pasted list is many values, and it is committed as one batch.
 *
 * Five hundred separate commits would be five hundred rebuilds, five hundred
 * repaints and a thousand dispatched events — every consumer's `onChange`
 * running five hundred times. So the store is filled first and the page is
 * drawn once.
 */
function onPickerPaste(state, event) {
  if (!state.paste) return;
  const text = event.clipboardData?.getData("text") ?? "";
  // SPLIT WITHOUT A REGEX. Building a character class out of the separators
  // means escaping them, and `\,` inside one is an invalid escape under the
  // `u` flag — so the `RegExp` constructor threw, the listener died, and a
  // paste of five addresses added none of them with nothing in the console
  // that named the separator as the cause.
  const cuts = new Set([...state.separators, "\n", "\r"]);
  if (![...text].some((ch) => cuts.has(ch))) return;
  event.preventDefault();
  const quiet = { quiet: true };
  const pieces = [];
  let piece = "";
  for (const ch of text) {
    if (cuts.has(ch)) {
      pieces.push(piece);
      piece = "";
    } else {
      piece += ch;
    }
  }
  pieces.push(piece);
  let added = 0;
  for (const one of pieces) {
    if (commit(state, one, quiet)) added += 1;
  }
  if (added > 0) {
    announcePicker(state);
    sayPicker(state, message(state.words.status, { count: added }, added));
  }
}

