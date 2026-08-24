// Everything a `Search` does once a script is running.
//
// IT SHIPS, and it was the last one that did not. `Search` writes
// `role="combobox"` and `aria-expanded`, which promise a reader a listbox, the
// arrows, Enter and Escape — and until this moved out of the documentation
// site's own script, a consumer who installed the packages got the promise and
// none of it.
//
// WHAT WAS SUPPOSED TO BE IN THE WAY, AND WAS NOT. The recorded reason for
// leaving it behind was that its index came from a path the documentation site
// decided. It never did: `[prop.src]` has always written `data-nd-src`, and
// this reads that attribute. The note was true of an older shape and outlived
// it, which is the failure mode a gap list has.
//
// THE RANKING IS NOT WRITTEN HERE. `search-match.js` touches no DOM, carries no
// word in any language and imports nothing — and its own header said it would
// move into a behaviour package unchanged on the day one existed. It did.
//
// THE ANCHORING IS NOT WRITTEN HERE EITHER: `anchor.js` places the panel, as it
// does for the six other floating surfaces.

import { keepAnchored, releaseAnchor } from "./anchor.js";
import { message } from "./messages.js";
import { prepare, rank } from "./search-match.js";


// WHAT THIS REPLACES.
//
// One `input` listener that ran `dataset.ndIndex.toLowerCase().includes(needle)`
// over three hundred and forty-seven elements per keystroke and toggled `hidden`
// on each. No ranking, no reason a hit was a hit, no keyboard, no combobox
// semantics — the box and the results were unrelated in the accessibility tree,
// so a reader heard "twelve results" and had no announced way to reach one.
//
// WHAT THE COMPONENT OWES AND WHAT THIS OWES.
//
// The markup carries the box, the rows, the words and the ids. Every one of
// `role="combobox"`, `aria-expanded`, `aria-controls`, `aria-autocomplete`,
// `aria-activedescendant`, `role="listbox"`, `role="option"` and `role="group"`
// is written HERE, because a box claiming to control a listbox that nothing can
// open is a promise the markup does not keep.
//
// TWO SUPPLY FORMS. Children are rows already in the document — real links that
// work with this file blocked. `src` is a JSON index fetched on first open,
// filled into a fixed pool of rows. The pool is not an optimisation: rebuilding
// the list per keystroke is how `aria-activedescendant` comes to name a removed
// node, which is the commonest defect in this pattern and invisible without a
// screen reader.

/** Every enhanced search, by its root. */
const searches = new Map();

/** How long after the last keystroke the count is spoken. */
const ANNOUNCE_AFTER = 400;

/** How many recent queries are kept, whatever the reader typed. */
const RECENT_KEEP = 6;

export function enhanceSearches(root = document) {
  for (const root_ of root.querySelectorAll("[data-nd-search]")) {
    if (searches.has(root_)) continue;
    const state = buildSearch(root_);
    if (state) searches.set(root_, state);
  }
}

/**
 * Takes the behaviour back off every enhanced search under `root`.
 *
 * A BEHAVIOUR THAT CANNOT BE RELEASED IS A DEFECT FOR EVERY CONSUMER WITH A
 * CLIENT-SIDE ROUTER, and this one holds its state in a STRONG `Map` keyed by
 * the element. Every search ever enhanced stays reachable from that map for the
 * life of the page, with its rows, its pool and its recent queries. A `WeakMap`
 * would not have fixed it either, because the state points back at the element.
 */
export function releaseSearches(root = document) {
  for (const element of root.querySelectorAll("[data-nd-search]")) {
    const state = searches.get(element);
    if (!state) continue;
    releaseAnchor(state.panel);
    state.binding.abort();
    searches.delete(element);
  }
}

function buildSearch(root_) {
  const binding = new AbortController();
  const signal = binding.signal;
  const box = root_.querySelector("[data-nd-search-input]");
  const panel = root_.querySelector("[data-nd-search-panel]");
  const list = root_.querySelector("[data-nd-search-results]");
  if (!box || !panel || !list) return null;

  const name = root_.dataset.ndSearch;
  const id = `nd-search-${name}`;
  const state = {
    // CARRIED ON THE STATE, because every helper below already takes it. A
    // parallel argument threaded through a dozen call sites is a dozen places
    // for one of them to be forgotten.
    binding,
    signal,
    root: root_,
    name,
    id,
    mode: root_.dataset.ndPresentation ?? "inline",
    box,
    panel,
    list,
    note: root_.querySelector("[data-nd-search-status]"),
    dialog: root_.querySelector("[data-nd-search-dialog]"),
    trigger: root_.querySelector("[data-nd-search-open]"),
    clear: root_.querySelector("[data-nd-search-clear]"),
    recentBox: root_.querySelector("[data-nd-search-recent]"),
    recentList: root_.querySelector("[data-nd-search-recent-list]"),
    src: root_.dataset.ndSrc ?? "",
    cap: Number.parseInt(root_.dataset.ndCap ?? "", 10) || 0,
    filter: root_.dataset.ndFilter === "true",
    recentKey: root_.dataset.ndRecent ?? "",
    words: {
      status: root_.dataset.ndStatus ?? "",
      empty: root_.dataset.ndEmptyLabel ?? "",
      loading: root_.dataset.ndLoadingLabel ?? "",
      error: root_.dataset.ndErrorLabel ?? "",
      cap: root_.dataset.ndCapLabel ?? "",
    },
    candidates: null,
    pool: [],
    hits: [],
    active: -1,
    frame: 0,
    sayAt: 0,
    lastSaid: null,
    // THE CALLER'S CLAIM, and it was read by nothing until this line.
    //
    // `data-nd-loading` was stamped by the component, documented in the
    // contract and never looked at here — so `Search { loading: true }`
    // rendered a box indistinguishable from one whose results had all
    // arrived, which is the exact confusion the prop exists to prevent.
    //
    // It is separate from `fetching` below on purpose. This one is what the
    // caller says about their own data; that one is whether a request for
    // `src` is in the air. They produce the same sentence and they are not the
    // same fact — collapsing them into one field is what hid this, because the
    // fetch guard read the field as "a request is running" and the attribute
    // never got a chance to mean anything.
    loading: root_.dataset.ndLoading === "true",
    fetching: false,
    loaded: false,
    failed: false,
  };

  box.setAttribute("role", "combobox");
  box.setAttribute("aria-autocomplete", "list");
  box.setAttribute("aria-controls", `${id}-results`);
  box.setAttribute("aria-expanded", "false");
  list.setAttribute("role", "listbox");
  for (const group of list.querySelectorAll("[data-nd-search-group]")) {
    group.setAttribute("role", "group");
    group.setAttribute("aria-label", group.dataset.ndSearchGroup);
    // A `listbox` holds groups and options and nothing else, so the inner list
    // has to stop being a list as far as the tree is concerned.
    group.querySelector("ul")?.setAttribute("role", "none");
  }

  if (state.src) {
    state.pool = buildPool(state);
  } else {
    state.candidates = childCandidates(state);
  }

  box.addEventListener("input", () => runQuery(state), { signal });
  box.addEventListener("keydown", (event) => onSearchKey(state, event), { signal });
  box.addEventListener("focus", () => {
    if (state.mode === "popover" && (state.box.value || recentQueries(state).length)) {
      openSearch(state);
    }
  }, { signal });
  state.clear?.addEventListener("click", () => clearQuery(state), { signal });
  state.trigger?.addEventListener("click", () => openSearch(state), { signal });
  root_.querySelector("[data-nd-search-forget]")?.addEventListener("click", () => {
    forgetQueries(state);
    showRecent(state);
  }, { signal });

  // Choosing a result is what makes it recent, and it is the only thing that
  // does: a query somebody typed and abandoned is not one they meant.
  //
  // AND IT ANSWERS THE QUESTION, which is what closes and clears the palette.
  // Until `navigate.js` existed, following a result took the whole document
  // with it and both were free; with the document kept, the dialog stayed open
  // over the page the reader had just asked for, and the box stayed full — so
  // reopening the palette showed the results of the query they had already
  // answered instead of the recents that are the point of an empty box.
  //
  // Here rather than in the router, because the dialog is `Search`'s. A router
  // that reached in to close somebody else's surface would need a second line
  // for every surface a link can sit in.
  list.addEventListener("click", (event) => {
    if (!event.target.closest("a")) return;
    rememberQuery(state);
    if (!state.dialog) return;
    clearQuery(state);
    state.dialog.close();
  }, { signal });

  if (state.dialog) {
    // A trigger that opens a modal nothing can fill is a button that does
    // nothing, so the markup ships it hidden and this is what reveals it.
    if (state.trigger) state.trigger.hidden = false;
    state.dialog.addEventListener("cancel", (event) => {
      // THE FIRST ESCAPE CLEARS AND THE SECOND CLOSES, implemented WITH the
      // platform rather than around it: the browser's default here IS the
      // close, so preventing it is the feature.
      if (state.box.value !== "") {
        event.preventDefault();
        clearQuery(state);
      }
    }, { signal });
    state.dialog.addEventListener("close", () => {
      state.root.removeAttribute("data-nd-open");
      // A stale "twelve results" read out after the panel is gone is a lie.
      note(state, "");
    }, { signal });
    // The backdrop is the dialog element itself, which is where a click on it
    // lands.
    state.dialog.addEventListener("click", (event) => {
      if (event.target === state.dialog) state.dialog.close();
    }, { signal });
    // And the one exit a reader can see. Escape and the backdrop are the other
    // two, and neither of them is visible on a touch screen.
    state.root
      .querySelector("[data-nd-search-close]")
      ?.addEventListener("click", () => state.dialog.close(), { signal });
  }

  root_.dataset.ndEnhanced = "true";
  if (state.mode === "inline") runQuery(state);
  return state;
}

/** The rows the page already shipped, prepared once. */
function childCandidates(state) {
  return [...state.list.querySelectorAll(".nd-search-result")].map((row, index) => {
    const labelEl = row.querySelector("[data-nd-search-label]");
    const text = labelEl?.textContent ?? "";
    row.setAttribute("role", "option");
    row.id = `${state.id}-option-${index}`;
    row.querySelector("a")?.setAttribute("tabindex", "-1");
    return {
      row,
      labelEl,
      text,
      match: prepare(text),
      // Matched but never drawn, and a row matched only on these is
      // deliberately not highlighted: the letters that matched are not letters
      // on the screen, and a highlight over the wrong ones is worse than none.
      keywords: row.dataset.ndKeywords ? prepare(row.dataset.ndKeywords) : null,
      group: row.closest("[data-nd-search-group]") ?? null,
    };
  });
}

/** A fixed set of rows the fetched index is written into. */
function buildPool(state) {
  const size = state.cap > 0 ? state.cap : 20;
  const pool = [];
  for (let index = 0; index < size; index += 1) {
    const row = document.createElement("li");
    row.className = "nd-search-result";
    row.setAttribute("role", "option");
    row.id = `${state.id}-option-${index}`;
    row.hidden = true;
    const link = document.createElement("a");
    link.className = "nd-search-result-link";
    link.tabIndex = -1;
    const label = document.createElement("span");
    label.className = "nd-search-result-label";
    label.dataset.ndSearchLabel = "";
    const group = document.createElement("span");
    group.className = "nd-search-result-detail";
    link.append(label, group);
    row.append(link);
    state.list.append(row);
    pool.push({ row, link, labelEl: label, detailEl: group });
  }
  return pool;
}

/** The index at `src`, fetched once, the first time somebody searches. */
async function loadIndex(state) {
  if (state.loaded || state.fetching || !state.src) return;
  state.fetching = true;
  setLoading(state, true);
  note(state, state.words.loading);
  try {
    const response = await fetch(state.src);
    if (!response.ok) throw new Error(String(response.status));
    const data = await response.json();
    state.candidates = (data.entries ?? []).map((entry) => ({
      entry,
      text: entry.label ?? "",
      match: prepare(entry.label ?? ""),
      keywords: entry.keywords ? prepare(entry.keywords) : null,
    }));
    state.loaded = true;
    // And the claim is over: whatever the caller said was on its way is here.
    setLoading(state, false);
    // The children were the no-script fallback; the script has arrived.
    for (const row of state.list.querySelectorAll(".nd-search-result:not([role])")) {
      row.hidden = true;
    }
  } catch {
    // A failed fetch and an empty result look identical and mean opposite
    // things, which is why they are two sentences rather than one.
    state.failed = true;
    setLoading(state, false);
  } finally {
    state.fetching = false;
    runQuery(state);
  }
}

/**
 * Says it in the state and on the element, which are one fact in two places.
 *
 * The attribute is what the stylesheet reads for the progress cursor, and a
 * cursor that keeps spinning after the list arrived is the same lie in the
 * other direction.
 */
function setLoading(state, loading) {
  state.loading = loading;
  state.root.dataset.ndLoading = String(loading);
}

// --- Matching and drawing ---------------------------------------------------

function runQuery(state) {
  const query = state.box.value.trim();
  if (state.src && !state.loaded && !state.failed) {
    if (query) loadIndex(state);
    // AN EMPTY BOX IS EXACTLY WHEN THE RECENTS ARE WORTH SHOWING, and this
    // return swallowed them: `showRecent` is called from `drawResults`, which
    // is downstream of here, so a palette that had not fetched its index yet
    // — which is every palette, on the way in — drew nothing until a letter
    // was typed. The one moment the history is useful is the one moment it
    // was not on the screen.
    else showRecent(state);
    return;
  }
  if (!state.candidates) return;

  if (query === "") {
    state.hits = state.src ? [] : state.candidates.map((item) => ({ item, positions: [] }));
    state.total = state.hits.length;
  } else {
    const { hits, total } = rank(query, state.candidates, state.cap);
    // A row nothing matched by its label may still answer to its keywords, and
    // then it is shown without a highlight.
    const missed = [];
    if (state.cap === 0 || hits.length < state.cap) {
      const found = new Set(hits.map((hit) => hit.item));
      for (const item of state.candidates) {
        if (found.has(item) || !item.keywords) continue;
        const hit = rank(query, [{ match: item.keywords }], 0).hits[0];
        if (hit) missed.push({ item, positions: [], score: hit.score - 1_000_000 });
      }
    }
    missed.sort((a, b) => b.score - a.score);
    const room = state.cap > 0 ? Math.max(0, state.cap - hits.length) : missed.length;
    state.hits = [...hits, ...missed.slice(0, room)];
    state.total = total + missed.length;
  }

  state.active = -1;
  schedule(state);
}

/**
 * Render and anchor inside one coalesced frame.
 *
 * `requestAnimationFrame` and not a microtask: a microtask runs before layout,
 * so reading `panel.offsetWidth` for the anchor from one forces a synchronous
 * reflow on every keystroke. This puts the read and the write in the same
 * frame, once.
 */
function schedule(state) {
  if (state.frame) return;
  state.frame = requestAnimationFrame(() => {
    state.frame = 0;
    drawResults(state);
  });
}

function drawResults(state) {
  const query = state.box.value.trim();
  const shown = new Set(state.hits.map((hit) => hit.item));

  if (state.src) {
    state.hits.forEach((hit, index) => {
      const slot = state.pool[index];
      if (!slot) return;
      slot.row.hidden = false;
      slot.link.href = hit.item.entry.href ?? "";
      paintLabel(slot.labelEl, hit.item.match.chars, hit.positions);
      slot.detailEl.textContent = hit.item.entry.detail ?? hit.item.entry.group ?? "";
    });
    for (let index = state.hits.length; index < state.pool.length; index += 1) {
      state.pool[index].row.hidden = true;
    }
  } else {
    for (const item of state.candidates) {
      item.row.hidden = !shown.has(item);
      if (item.group) item.group.hidden = true;
    }
    // Reordered by moving rather than by cloning, and only the rows that are
    // shown: the rest keep the author's order, which is also the order they
    // have with no script.
    for (const hit of state.hits) {
      if (hit.positions.length) markInPlace(hit.item.labelEl, hit.positions);
      else clearMarks(hit.item.labelEl);
      if (hit.item.group) {
        hit.item.group.hidden = false;
      } else if (query) {
        state.list.append(hit.item.row);
      }
    }
  }

  const has = state.hits.length > 0;
  state.box.setAttribute("aria-expanded", String(has));
  state.root.toggleAttribute("data-nd-has-results", has);

  if (state.filter) filterAround(state, query);

  // The sentence, and the same one the live region says.
  let words = "";
  if (state.failed) words = state.words.error;
  else if (state.loading || state.fetching) words = state.words.loading;
  else if (query === "" && state.src) words = "";
  else if (!has && query) words = state.words.empty;
  else if (state.cap > 0 && state.total > state.hits.length) {
    words = message(
      state.words.cap,
      { shown: state.hits.length, total: state.total },
      state.total,
    );
  } else if (query) {
    words = message(state.words.status, { count: state.total }, state.total);
  }
  note(state, words);
  showRecent(state);
}

/**
 * The panel's one sentence — shown and announced by the same element, at most
 * once per pause, and never twice the same.
 *
 * ONE ELEMENT, BECAUSE A COUNT WORTH TELLING A SCREEN READER IS A COUNT A
 * READER CAN SEE. It is `role="status"`, so writing it *is* announcing it —
 * which is why the write is what gets debounced rather than a second hidden
 * copy of it. The line this replaces wrote on every `input` event, so typing
 * `button` queued six utterances, five of them stale before they were spoken.
 *
 * Clearing is immediate and appearing is not: a sentence that lingers 400ms
 * after it stopped being true is the one case where the delay is visible.
 */
function note(state, words) {
  if (!state.note) return;
  clearTimeout(state.sayAt);
  if (words === "") {
    state.lastSaid = "";
    state.note.textContent = "";
    return;
  }
  // THE FIRST SENTENCE IS NOT PACED. The delay exists to collapse a burst of
  // keystrokes into one utterance, and on the first write there is no burst to
  // collapse — only a panel that says nothing for four hundred milliseconds
  // while claiming to be still loading.
  if (state.lastSaid === null) {
    state.lastSaid = words;
    state.note.textContent = words;
    return;
  }
  state.sayAt = setTimeout(() => {
    if (words === state.lastSaid) return;
    state.lastSaid = words;
    state.note.textContent = words;
  }, ANNOUNCE_AFTER);
}

/** The rest of the page, filtered by the same query. Off unless asked for. */
function filterAround(state, query) {
  const scope = state.root.closest("nav, main, aside, section, body");
  if (!scope) return;
  for (const element of scope.querySelectorAll("[data-nd-index]")) {
    if (state.root.contains(element)) continue;
    const hit = query === "" || rank(query, [{ match: prepare(element.dataset.ndIndex) }], 0).hits.length > 0;
    const row = element.closest("li") ?? element;
    row.hidden = !hit;
  }
}

// --- The highlight ----------------------------------------------------------

// `<mark class="nd-mark">`, which is the component whose summary is literally
// "Text highlighted for the reader's current purpose, such as a search hit".
//
// The CSS Custom Highlight API would do this without touching the DOM and is
// rejected anyway: a highlight pseudo-element accepts no layout-affecting
// property, so it cannot set `font-weight` — and `Mark`'s own accessibility
// note says the weight is why the highlight is legible to a reader who cannot
// distinguish the colour. It would render a `Mark` that fails `Mark`'s contract.

/** A label the script owns, rebuilt with the matched runs marked. */
function paintLabel(into, chars, positions) {
  into.textContent = "";
  const wanted = new Set(positions);
  let run = [];
  let marked = false;
  const flush = () => {
    if (run.length === 0) return;
    if (marked) {
      const mark = document.createElement("mark");
      mark.className = "nd-mark";
      mark.dataset.ndHit = "";
      mark.textContent = run.join("");
      into.append(mark);
    } else {
      into.append(document.createTextNode(run.join("")));
    }
    run = [];
  };
  for (let index = 0; index < chars.length; index += 1) {
    const on = wanted.has(index);
    if (on !== marked) {
      flush();
      marked = on;
    }
    run.push(chars[index]);
  }
  flush();
}

/**
 * A label the PAGE shipped, wrapped where it matched and put back exactly.
 *
 * Rebuilding it would destroy classes, drawings and anything a consumer
 * attached. Splitting the text nodes and undoing with `normalize()` — which
 * merges them back — leaves every element untouched.
 */
function markInPlace(label, positions) {
  if (!label) return;
  clearMarks(label);
  if (positions.length === 0) return;
  const wanted = [...positions].sort((a, b) => a - b);
  const walker = document.createTreeWalker(label, NodeFilter.SHOW_TEXT);
  const jobs = [];
  let seen = 0;
  let next = 0;
  for (let node = walker.nextNode(); node && next < wanted.length; node = walker.nextNode()) {
    const chars = [...node.data];
    let unit = 0;
    let start = -1;
    let units = 0;
    for (let index = 0; index < chars.length; index += 1) {
      if (next < wanted.length && wanted[next] === seen + index) {
        if (start < 0) {
          start = unit;
          units = 0;
        }
        units += chars[index].length;
        next += 1;
      } else if (start >= 0) {
        jobs.push({ node, start, end: start + units });
        start = -1;
      }
      unit += chars[index].length;
    }
    if (start >= 0) jobs.push({ node, start, end: start + units });
    seen += chars.length;
  }
  // Back to front, so splitting an earlier run cannot move a later offset.
  for (const job of jobs.reverse()) {
    const range = document.createRange();
    range.setStart(job.node, job.start);
    range.setEnd(job.node, job.end);
    const mark = document.createElement("mark");
    mark.className = "nd-mark";
    mark.dataset.ndHit = "";
    range.surroundContents(mark);
  }
}

/** Puts a label back exactly as the page shipped it. */
function clearMarks(label) {
  if (!label) return;
  // `[data-nd-hit]`, not `mark`: a caller's children may hold a `Mark` of their
  // own, and unwrapping that would be this file editing the page's content
  // rather than annotating it.
  for (const mark of label.querySelectorAll("mark[data-nd-hit]")) {
    mark.replaceWith(...mark.childNodes);
  }
  label.normalize();
}

// --- Opening, closing and the keyboard --------------------------------------

function openSearch(state) {
  if (state.mode === "dialog") {
    if (state.dialog?.open) return;
    state.root.dataset.ndOpen = "true";
    // Whatever the box already holds, drawn before the modal is shown — which
    // for an empty box is the recents.
    runQuery(state);
    state.dialog?.showModal();
    // `showModal` moves focus; this only makes sure it lands in the box rather
    // than on whatever the browser picked first.
    state.box.focus();
    state.box.select();
    return;
  }
  if (state.mode !== "popover") return;
  state.root.dataset.ndOpen = "true";
  runQuery(state);
  keepAnchored(state.panel, state.box.closest(".nd-search-field") ?? state.box, { room: 320, width: "--nd-search-width" }, () =>
    closeSearch(state),
  );
}

function closeSearch(state) {
  if (state.mode === "dialog") {
    state.dialog?.close();
    return;
  }
  if (state.mode !== "popover") return;
  releaseAnchor(state.panel);
  state.root.removeAttribute("data-nd-open");
  setActive(state, -1);
}

function clearQuery(state) {
  state.box.value = "";
  runQuery(state);
  state.box.focus();
}

/** The rows a reader can move through, in the order they are drawn. */
function searchRows(state) {
  return state.src
    ? state.pool.filter((slot) => !slot.row.hidden).map((slot) => slot.row)
    : state.hits.map((hit) => hit.item.row).filter((row) => !row.hidden);
}

function setActive(state, index) {
  const rows = searchRows(state);
  for (const row of rows) {
    row.removeAttribute("aria-selected");
    row.removeAttribute("data-nd-active");
  }
  state.active = index;
  const row = rows[index];
  if (!row) {
    state.box.removeAttribute("aria-activedescendant");
    return;
  }
  // On the active option ONLY. `aria-selected="false"` on three hundred rows is
  // noise a screen reader reads out.
  row.setAttribute("aria-selected", "true");
  row.dataset.ndActive = "";
  state.box.setAttribute("aria-activedescendant", row.id);
  row.scrollIntoView({ block: "nearest" });
}

function moveActive(state, delta) {
  const rows = searchRows(state);
  if (rows.length === 0) return;
  const next = state.active < 0 ? (delta > 0 ? 0 : rows.length - 1) : state.active + delta;
  setActive(state, (next + rows.length) % rows.length);
}

/** How many rows fit in the panel, which is what a page means here. */
function pageSize(state) {
  const row = searchRows(state)[0];
  if (!row) return 1;
  const height = row.getBoundingClientRect().height || 1;
  return Math.max(1, Math.floor(state.list.clientHeight / height) - 1);
}

function onSearchKey(state, event) {
  const rows = searchRows(state);
  switch (event.key) {
    case "ArrowDown":
      event.preventDefault();
      if (state.mode === "popover" && !state.root.hasAttribute("data-nd-open")) openSearch(state);
      moveActive(state, 1);
      break;
    case "ArrowUp":
      event.preventDefault();
      if (state.mode === "popover" && !state.root.hasAttribute("data-nd-open")) openSearch(state);
      moveActive(state, -1);
      break;
    case "Home":
    case "End":
      // THE CARET'S, NOT THE LIST'S. A reader fixing a typo at the start of
      // what they typed is not asking for the first result — the same call
      // `onSelectKey` already made, for the same reason. The list keeps the
      // modified pair.
      if (event.metaKey || event.ctrlKey) {
        event.preventDefault();
        setActive(state, event.key === "Home" ? 0 : rows.length - 1);
      }
      break;
    case "PageDown":
    case "PageUp": {
      if (rows.length === 0) break;
      event.preventDefault();
      const step = pageSize(state) * (event.key === "PageDown" ? 1 : -1);
      const from = state.active < 0 ? 0 : state.active;
      setActive(state, Math.min(rows.length - 1, Math.max(0, from + step)));
      break;
    }
    case "Enter": {
      const row = rows[state.active];
      if (!row) return; // A form, if there is one, submits.
      event.preventDefault();
      const link = row.querySelector("a");
      if (!link) return;
      rememberQuery(state);
      if (event.metaKey || event.ctrlKey) window.open(link.href, "_blank", "noopener");
      else link.click();
      break;
    }
    case "Escape":
      event.preventDefault();
      if (state.box.value !== "") clearQuery(state);
      else closeSearch(state);
      break;
    case "Tab":
      // A POPOVER IS LEFT BY TABBING OUT OF IT. A DIALOG IS NOT.
      //
      // This closed on Tab whatever the presentation was, which threw a reader
      // out of the palette on the first press — and did it by fighting the
      // modality the browser was already providing. Inside a `<dialog>` the
      // browser cycles focus within the top layer, which is the behaviour the
      // contract promises and the reason no focus trap is written here.
      if (state.mode !== "dialog") closeSearch(state);
      break;
    default:
      break;
  }
}

// --- The hotkey -------------------------------------------------------------

/** Whether a key belongs to whatever has focus rather than to the page. */
function isTyping(target) {
  return Boolean(
    target?.closest?.(
      // AND ANYTHING THAT SAID IT LISTENS FOR LETTERS. A `role="tree"` node
      // with focus is not a text field, so `/` would have opened the palette
      // out from under a reader who was typing a node's name to jump to it —
      // and the hotkey listener is registered before the roving one, so it
      // would have won. A component that reads printable keys says so with
      // `data-nd-typeahead` and this stops taking them.
      "input, textarea, select, [contenteditable]:not([contenteditable='false']), [data-nd-typeahead]",
    ),
  );
}

/**
 * The first search in document order that asked for the chord.
 *
 * Two claimants would be two things answering one keystroke, so the first wins
 * and it wins deterministically.
 */
function hotkeyOwner() {
  const root_ = document.querySelector('[data-nd-search][data-nd-hotkey="true"]');
  return root_ ? searches.get(root_) : null;
}

document.addEventListener("keydown", (event) => {
  if (event.isComposing || event.defaultPrevented) return;
  const state = hotkeyOwner();
  if (!state) return;

  // Command-K on Apple hardware, Control-K everywhere else. It fires from
  // inside a text field on purpose — that is the convention a palette is judged
  // against — and it toggles, so the same keystroke puts it away.
  if (event.key.toLowerCase() === "k" && (event.metaKey || event.ctrlKey) && !event.altKey) {
    event.preventDefault();
    if (state.mode === "dialog" && state.dialog?.open) closeSearch(state);
    else {
      openSearch(state);
      state.box.focus();
    }
    return;
  }

  // `/` is the other convention, and it must never fire while somebody is
  // typing — including into this component's own box, where a slash is a slash.
  if (event.key === "/" && !event.metaKey && !event.ctrlKey && !event.altKey) {
    if (isTyping(event.target)) return;
    event.preventDefault();
    openSearch(state);
    state.box.focus();
  }
});

// --- Recent queries ---------------------------------------------------------

// Opt-in, by a prop carrying the key. Absent means nothing is stored — not a
// shorter history, none — because what somebody typed into a search is a
// different category of thing from the appearance they chose.

function recentQueries(state) {
  if (!state.recentKey) return [];
  try {
    const stored = JSON.parse(localStorage.getItem(state.recentKey) ?? "[]");
    // A hostile or stale value is not a reason to break the box. The theme
    // layer applies the same discipline to every setting it reads.
    if (!Array.isArray(stored)) return [];
    return stored.filter((entry) => typeof entry === "string").slice(0, RECENT_KEEP);
  } catch {
    return [];
  }
}

function rememberQuery(state) {
  const query = state.box.value.trim();
  if (!state.recentKey || query === "") return;
  const kept = [query, ...recentQueries(state).filter((entry) => entry !== query)];
  try {
    localStorage.setItem(state.recentKey, JSON.stringify(kept.slice(0, RECENT_KEEP)));
  } catch {
    // A full or refused store is not a reason to lose the search.
  }
}

function forgetQueries(state) {
  if (!state.recentKey) return;
  try {
    localStorage.removeItem(state.recentKey);
  } catch {
    // As above.
  }
}

function showRecent(state) {
  if (!state.recentBox || !state.recentList) return;
  const queries = state.box.value.trim() === "" ? recentQueries(state) : [];
  state.recentBox.hidden = queries.length === 0;
  state.recentList.textContent = "";
  for (const query of queries) {
    const item = document.createElement("li");
    const button = document.createElement("button");
    button.type = "button";
    button.className = "nd-search-recent-item";
    button.textContent = query;
    button.addEventListener("click", () => {
      state.box.value = query;
      state.box.focus();
      runQuery(state);
    }, { signal: state.signal });
    item.append(button);
    state.recentList.append(item);
  }
}

