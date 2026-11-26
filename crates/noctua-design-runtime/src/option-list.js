// Everything an `OptionList` does once a script is running.
//
// IT SHIPS, because the markup writes `role="listbox"` and a listbox owes a
// keyboard (`NOCTUA.md §9.4`).
//
// THE FOCUS NEVER ENTERS THE LIST'S OPTIONS. One element holds it the whole
// time, and which option the keyboard is on is `data-nd-active` on the option
// and `aria-activedescendant` on the element with the focus. That element is
// one of two:
//
//   * THE LIST ITSELF, where it is a tab stop (`focusable`, the default). The
//     arrows, Home and End, the page keys, Enter, Space and a typed letter are
//     all its own.
//   * A CONTROL THAT NAMES THE LIST IN `aria-controls`: a prompt or a search
//     box a reader goes on typing in while the list is filtered. The arrows,
//     the page keys and Enter are heard on that control and answered here, so
//     a control gains the walk by naming the list and writes none of it. Home,
//     End, Space and letters stay the control's, because there they move the
//     caret and type.
//
// IT MOVES AND IT SAYS. IT CHOOSES NOTHING.
//
// Two events leave the list's root, both bubbling, each with
// `detail: { name, value }`:
//
//   nd-option-active   the keyboard or a pointer moved to another option
//   nd-option-choose   Enter, Space or a press asked for the active option
//
// What `selected` is afterwards is the caller's to render. A list of commands
// runs one and has no chosen row; a list of settings pages has one its caller
// already keeps. A script that kept a second copy of that answer would be a
// second answer.
//
// THE CALLER MAY MOVE IT TOO, AND THE NEWEST MOVE WINS. An interface that is a
// view of state kept elsewhere renders `OptionItem { active: true }` itself.
// This file watches the list, takes an option that gained the attribute from
// outside as the active one, and takes the attribute off the one it had. It
// reads its own writes back off the observer before returning, so it never
// answers itself.
//
// WHAT IT NEVER DOES:
//
// 1. It never wraps. A reader holding the Down arrow stops at the last option,
//    which is how they learn the list has ended.
// 2. It never skips a disabled option. A refused option is reached, announced
//    and not chosen.
// 3. It never takes the focus. A press on an option of a list that is not a
//    tab stop leaves the focus in the control the reader was typing in.
// 4. It never filters. The caller holds the matcher's answer and renders the
//    options it found, with `hits` on each (`NOCTUA.md §12.4`).
// 5. It never moves the page further than the active option needs. A move
//    scrolls the box that holds the option, and the page only where the
//    option would otherwise leave the window, each the least way. An option
//    already active when the list is enhanced is left where the page put it.
// 6. It never answers a key an input method is using, or a key with Shift or
//    Alt held on a control that names the list.

/** Every enhanced list, by its root. */
const lists = new Map();

/** The root every list hangs from. */
const LIST = "[data-nd-option-list]";

/** How long a run of keystrokes counts as one word, in milliseconds. */
const TYPING = 700;

export function enhanceOptionLists(root = document) {
  for (const root_ of root.querySelectorAll(LIST)) {
    if (lists.has(root_)) continue;
    const state = buildList(root_);
    if (state) lists.set(root_, state);
  }
}

/**
 * Takes the behaviour back off every enhanced list under `root`.
 *
 * The state is held in a strong `Map` and holds an observer, so a list nothing
 * released is a list, its options and its observer kept for the life of the
 * page.
 */
export function releaseOptionLists(root = document) {
  for (const element of root.querySelectorAll(LIST)) {
    const state = lists.get(element);
    if (!state) continue;
    state.binding.abort();
    state.observer.disconnect();
    lists.delete(element);
  }
}

function buildList(root_) {
  const list = root_.querySelector(":scope > .nd-option-list-items");
  if (!list) return null;
  const binding = new AbortController();
  const signal = binding.signal;
  const state = {
    binding,
    signal,
    root: root_,
    list,
    name: root_.dataset.ndOptionList,
    id: list.id,
    active: null,
    numbered: 0,
    typed: "",
    typedAt: 0,
    observer: null,
  };

  // IT READS PRINTABLE KEYS, AND SAYS SO: `search.js` leaves `/` alone on
  // anything carrying this, as it does for a tree.
  list.dataset.ndTypeahead = "";

  list.addEventListener("keydown", (event) => onListKey(state, event), { signal });
  list.addEventListener("focus", () => onListFocus(state), { signal });
  list.addEventListener("mousedown", (event) => {
    // A press on a list that is no tab stop must not blur the control the
    // reader is typing in. The click still arrives.
    if (!list.hasAttribute("tabindex")) event.preventDefault();
  }, { signal });
  list.addEventListener("click", (event) => {
    const option = event.target.closest?.(".nd-option-item");
    if (!option || !list.contains(option)) return;
    setActive(state, option);
    choose(state);
  }, { signal });

  state.observer = new MutationObserver(() => adopt(state, true));
  state.observer.observe(list, {
    childList: true,
    subtree: true,
    attributes: true,
    attributeFilter: ["data-nd-active", "hidden"],
  });

  // THE PAGE IS WHERE ITS READER LEFT IT. An option the caller rendered as
  // active is the active one from the start, and it is not brought into view:
  // a list enhanced as a page loads would otherwise scroll the page to itself.
  adopt(state, false);
  root_.dataset.ndEnhanced = "true";
  return state;
}

/** Every option a reader can see, in the order they see them. */
function options(state) {
  return [...state.list.querySelectorAll(".nd-option-item")].filter(
    (option) => !option.closest("[hidden]"),
  );
}

/**
 * Gives an id to every option that has none.
 *
 * A counter that only rises, never the option's index: a filtered list is
 * rendered again on every keystroke, and an index handed to the option now in
 * third place is an id two elements have held within one frame of each other.
 */
function number(state) {
  for (const option of state.list.querySelectorAll(".nd-option-item:not([id])")) {
    option.id = `${state.id}-option-${state.numbered}`;
    state.numbered += 1;
  }
}

/** Every control that names this list, wherever it is on the page. */
function controllers(state) {
  return document.querySelectorAll(`[aria-controls~="${CSS.escape(state.id)}"]`);
}

/** Says which option is active, on the list and on whatever names it. */
function point(state) {
  const id = state.active?.id ?? "";
  for (const holder of [state.list, ...controllers(state)]) {
    if (id) holder.setAttribute("aria-activedescendant", id);
    else holder.removeAttribute("aria-activedescendant");
  }
}

/**
 * Takes the list as it now is: what the page shipped, or what a caller just
 * rendered, which `show` says, and brings an option that became active into
 * view.
 *
 * AN OPTION THAT GAINED THE ATTRIBUTE FROM OUTSIDE IS THE ACTIVE ONE. Where two
 * carry it, one of them is the one this file last wrote and the other is the
 * caller's newer answer.
 */
function adopt(state, show) {
  number(state);
  const marked = options(state).filter((option) => option.dataset.ndActive === "true");
  const next = marked.find((option) => option !== state.active) ?? marked[0] ?? null;
  for (const option of state.list.querySelectorAll("[data-nd-active]")) {
    if (option !== next) option.removeAttribute("data-nd-active");
  }
  const moved = next !== state.active;
  state.active = next;
  point(state);
  if (moved && show) reveal(next);
  // Its own writes, read back so the observer does not report them to itself.
  state.observer.takeRecords();
}

/**
 * Brings an option that the keyboard or the caller moved to into view, the
 * least way: the box that scrolls it, and every box around that one, the page
 * included, each only as far as the option needs.
 *
 * THE PAGE MOVES WHEN THE OPTION WOULD LEAVE THE WINDOW. The active option is
 * the only sign of where the keyboard is, since the focus stays on the list or
 * the control that names it, so a walk that left it below the window would
 * leave the reader with nothing to read. An option the window already shows
 * moves nothing. A move is the only time it is called: the option active when
 * the list is enhanced is left where the page put it, or a list on a page as it
 * loaded would scroll the page to itself.
 */
function reveal(option) {
  option?.scrollIntoView({ block: "nearest", inline: "nearest" });
}

function say(state, type, option) {
  state.root.dispatchEvent(
    new CustomEvent(type, {
      bubbles: true,
      detail: { name: state.name, value: option?.dataset.ndValue ?? null },
    }),
  );
}

function setActive(state, option) {
  if (option === state.active) return;
  for (const other of state.list.querySelectorAll("[data-nd-active]")) {
    if (other !== option) other.removeAttribute("data-nd-active");
  }
  if (option) option.dataset.ndActive = "true";
  state.active = option ?? null;
  point(state);
  reveal(option);
  state.observer.takeRecords();
  say(state, "nd-option-active", option);
}

/** Asks for the active option. `false` when there is none to ask for. */
function choose(state) {
  const option = state.active;
  if (!option || !state.list.contains(option)) return false;
  // Reached, announced, and not chosen.
  if (option.getAttribute("aria-disabled") !== "true") say(state, "nd-option-choose", option);
  return true;
}

/** One step along the list, stopping at the ends. */
function step(state, delta) {
  const all = options(state);
  if (all.length === 0) return;
  const at = all.indexOf(state.active);
  const next = at < 0 ? (delta > 0 ? 0 : all.length - 1) : at + delta;
  setActive(state, all[Math.min(all.length - 1, Math.max(0, next))]);
}

function edge(state, last) {
  const all = options(state);
  if (all.length > 0) setActive(state, all[last ? all.length - 1 : 0]);
}

/** How many options fit in what scrolls them, which is what a page means. */
function pageSize(state) {
  const first = options(state)[0];
  if (!first) return 1;
  const height = first.getBoundingClientRect().height || 1;
  let box = state.list;
  while (box.parentElement && box.scrollHeight <= box.clientHeight) box = box.parentElement;
  return Math.max(1, Math.floor(box.clientHeight / height) - 1);
}

/**
 * The option beside the active one, in the next column that way.
 *
 * BY WHERE THE OPTIONS ARE, never by counting. How many columns there are is
 * the width's to decide and how many options are in each is the browser's, so
 * the only honest answer to *what is to the right of this* is the nearest
 * column that starts further right and, in it, the option whose middle is
 * nearest this one's.
 */
function across(state, direction) {
  const from = state.active;
  if (!from) {
    step(state, direction);
    return;
  }
  const here = from.getBoundingClientRect();
  const middle = (box) => box.top + box.height / 2;
  const beyond = options(state)
    .map((option) => ({ option, box: option.getBoundingClientRect() }))
    .filter(({ box }) => (direction > 0 ? box.left > here.left + 1 : box.left < here.left - 1));
  if (beyond.length === 0) return;
  const column = beyond.reduce(
    (best, { box }) => (direction > 0 ? Math.min(best, box.left) : Math.max(best, box.left)),
    direction > 0 ? Infinity : -Infinity,
  );
  let nearest = null;
  for (const one of beyond) {
    if (Math.abs(one.box.left - column) > 1) continue;
    const gap = Math.abs(middle(one.box) - middle(here));
    if (!nearest || gap < nearest.gap) nearest = { option: one.option, gap };
  }
  if (nearest) setActive(state, nearest.option);
}

/** The words an option is found by when a reader types its first letters. */
function spelt(option) {
  return (option.querySelector(".nd-option-item-label")?.textContent ?? "").trim().toLowerCase();
}

/**
 * Jumps to the next option whose name starts with what was typed.
 *
 * One letter looks from the option after the active one, so pressing it again
 * walks every option that starts with it. A second letter within the pause
 * makes a word, which looks from the active option itself.
 */
function typeAhead(state, event) {
  if (event.timeStamp - state.typedAt > TYPING) state.typed = "";
  state.typed += event.key.toLowerCase();
  state.typedAt = event.timeStamp;
  const all = options(state);
  const at = all.indexOf(state.active);
  const from = state.typed.length === 1 ? at + 1 : Math.max(at, 0);
  for (let turn = 0; turn < all.length; turn += 1) {
    const option = all[(from + turn) % all.length];
    if (spelt(option).startsWith(state.typed)) {
      setActive(state, option);
      return;
    }
  }
}

/** A tab stop with nothing active says where it is: the answer, or the top. */
function onListFocus(state) {
  if (state.active && state.list.contains(state.active)) return;
  const all = options(state);
  const start = all.find((option) => option.getAttribute("aria-selected") === "true") ?? all[0];
  if (start) setActive(state, start);
}

/** The keys both holders of the focus share. `true` when one was answered. */
function walk(state, event) {
  switch (event.key) {
    case "ArrowDown":
      step(state, 1);
      return true;
    case "ArrowUp":
      step(state, -1);
      return true;
    case "PageDown":
      step(state, pageSize(state));
      return true;
    case "PageUp":
      step(state, -pageSize(state));
      return true;
    case "Enter":
      return choose(state);
    default:
      return false;
  }
}

/**
 * Whether an input method has the key. `isComposing` is the standard's word
 * for it, and 229 is what an engine reports for a key the input method took
 * before the event was made: WebKit sends the Enter that commits a
 * composition after `compositionend`, not composing, with that code.
 */
function composing(event) {
  return event.isComposing || event.keyCode === 229;
}

function onListKey(state, event) {
  if (event.defaultPrevented || composing(event) || event.target !== state.list) return;
  let answered = walk(state, event);
  if (!answered) {
    const plain = !event.ctrlKey && !event.metaKey && !event.altKey;
    const columns = state.root.dataset.ndLayout === "columns";
    if (event.key === "Home" || event.key === "End") {
      edge(state, event.key === "End");
      answered = true;
    } else if (columns && (event.key === "ArrowRight" || event.key === "ArrowLeft")) {
      across(state, event.key === "ArrowRight" ? 1 : -1);
      answered = true;
    } else if (event.key === " " && plain) {
      // Answered even with nothing to choose, or the page scrolls instead.
      choose(state);
      answered = true;
    } else if (plain && [...event.key].length === 1) {
      typeAhead(state, event);
      answered = true;
    }
  }
  if (answered) event.preventDefault();
}

// THE CONTROL THAT NAMES A LIST, HEARD ON THE DOCUMENT.
//
// One listener for every list, and no list binds to a control: the control may
// arrive after the list, be rendered again, or name another list on the next
// keystroke, and a listener bound to an element is a listener on the element
// that was there. `aria-controls` is read when the key is pressed.
//
// A KEY WITH A MODIFIER IS ANOTHER KEY, and it stays the control's. Shift with
// Enter breaks a prompt's line and Shift with an arrow selects, Alt is the
// system's, and Control or Command with Enter or an arrow is a shortcut. Home
// and End belong to the caret there, so the list takes the pair only with
// Control or Command.
document.addEventListener("keydown", (event) => {
  if (event.defaultPrevented || composing(event) || event.shiftKey || event.altKey) return;
  const named = event.target?.getAttribute?.("aria-controls");
  if (!named) return;
  const ids = named.split(/\s+/);
  for (const state of lists.values()) {
    if (!ids.includes(state.id) || state.list.contains(event.target)) continue;
    let answered = false;
    if (event.ctrlKey || event.metaKey) {
      if (event.key === "Home" || event.key === "End") {
        edge(state, event.key === "End");
        answered = true;
      }
    } else {
      answered = walk(state, event);
    }
    if (answered) event.preventDefault();
    return;
  }
});
