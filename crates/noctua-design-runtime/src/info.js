// Everything a note beside a word does once a script is running.
//
// TWO COMPONENTS, ONE FILE, ONE OPEN NOTE. `Info` explains a word and `AILabel`
// says an assistant wrote the value beside it, and past the letters on the
// control they are the same surface: a `role="note"` holding one sentence,
// opened on a press, closed by Escape with the focus put back. `KINDS` below is
// the whole of the difference between them.
//
// A SECOND MODULE WOULD BE A SECOND `showing`, and two of those is an `Info`
// note and an `AILabel` note open at the same time over one page, which is the
// one thing a note beside a word must not do.
//
// IT SHIPS, for `ColorPicker`'s reason: a component whose consumer cannot use
// it is a component that lies. A note that only opens inside this repository's
// own documentation site is a marker everywhere else.
//
// THE ANCHORING IS NOT WRITTEN HERE. `anchor.js` already positions four
// floating surfaces — the frame theme menu, `Select`, `Search`, `TagPicker`
// and `DateInput` — and its own header says it was extracted precisely because
// a third would have made three. This is the sixth caller, not a sixth copy.
//
// WHAT IT NEVER DOES:
//
// 1. It never opens on a hover. A bubble that only appears under a pointer does
//    not exist on a telephone, cannot be reached with a keyboard, and vanishes
//    when a reader with a tremor drifts a pixel off the marker.
// 2. It never traps focus. This is a note beside a word, not a dialog: nothing
//    behind it is inert, the page keeps its scroll, and Tab walks out of it —
//    which is also what closes it.
// 3. It never leaves focus where the reader did not put it. Escape closes and
//    puts focus back on the marker, because a reader who dismissed a note and
//    lost their place was moved by the thing they were getting rid of.

import { keepAnchored, releaseAnchor } from "./anchor.js";

/** How much room a note wants below the marker before it flips above. */
const ROOM = 200;

/**
 * The two markers this file enhances, and the parts each one is made of.
 *
 * `reveal` is the only behavioural difference. `Info` ships its marker hidden
 * because with no script it opens nothing, and a marker that opens nothing is
 * worse than no marker. An `AILabel`'s mark says a value came from an
 * assistant, which is true with a script and without one, so it is drawn either
 * way and the note is the extra.
 */
const KINDS = [
  {
    hook: "[data-nd-info]",
    mark: "info",
    marker: ".nd-info-marker",
    note: ".nd-info-note",
    close: "nd-note-close nd-info-close",
    reveal: true,
  },
  {
    hook: "[data-nd-ai-note]",
    mark: "ai-label",
    marker: ".nd-ai-label-control",
    note: ".nd-ai-label-note",
    close: "nd-note-close nd-ai-label-close",
    reveal: false,
  },
];

// THE NOTE THAT IS OPEN, SO OPENING A SECOND CLOSES THE FIRST.
//
// IT CARRIES ITS OWN ROOT, WHICH IS WHAT A RELEASE NEEDS. Holding the closure
// alone meant nothing could ask which marker a note belonged to. A note open
// when its subtree went away left this pointing at a dead marker, and the next
// note opened anywhere on the page called it.
let showing = null;

/** Every enhanced note's teardown, so a subtree can be released. */
const infos = new WeakMap();

export function enhanceInfos(root = document) {
  for (const kind of KINDS) {
    for (const element of root.querySelectorAll(kind.hook)) {
      if (element.dataset.ndEnhanced) continue;
      element.dataset.ndEnhanced = kind.mark;
      const binding = new AbortController();
      infos.set(element, binding);
      enhance(element, kind, binding.signal);
    }
  }
}

/**
 * Takes the behaviour back off every note under `root`.
 *
 * A BEHAVIOUR THAT CANNOT BE RELEASED IS A DEFECT FOR EVERY CONSUMER WITH A
 * CLIENT-SIDE ROUTER. `enhance` registers a `pointerdown` listener on the
 * DOCUMENT, and that listener outlives the marker it closes: every visit to the
 * route adds another one, holding a detached subtree nothing can collect.
 *
 * The close control this file DREW is taken away again, because a second pass
 * over the same markup would otherwise draw a second one beside it.
 */
export function releaseInfos(root = document) {
  for (const kind of KINDS) {
    for (const element of root.querySelectorAll(kind.hook)) {
      if (element.dataset.ndEnhanced !== kind.mark) continue;
      if (showing?.root === element) showing = null;
      // AND THE NOTE GOES BACK TO WHAT THE MARKUP SHIPS. One left open with its
      // listeners gone is a bubble over the page that Escape cannot dismiss.
      const note = element.querySelector(kind.note);
      if (note) {
        releaseAnchor(note);
        note.hidden = true;
      }
      const marker = element.querySelector(kind.marker);
      if (marker) {
        marker.setAttribute("aria-expanded", "false");
        // Only the kind whose markup ships hidden goes back to hidden. Hiding
        // an `AILabel`'s control on release would take the mark off the page,
        // and the mark is true whether or not a script is running.
        if (kind.reveal) marker.hidden = true;
      }
      element.querySelector(`.${kind.close.split(" ")[0]}`)?.remove();
      infos.get(element)?.abort();
      infos.delete(element);
      delete element.dataset.ndEnhanced;
    }
  }
}

function enhance(root, kind, signal) {
  const marker = root.querySelector(kind.marker);
  const note = root.querySelector(kind.note);
  if (!marker || !note) return;

  // REVEALED, NEVER BUILT. `Info`'s markup ships hidden so a page with no
  // script carries no marker that opens nothing; this is the script arriving.
  if (kind.reveal) marker.hidden = false;

  const closeLabel = root.getAttribute("data-nd-close-label");
  if (closeLabel) {
    // Drawn here rather than in the markup because it is the one control that
    // means nothing without a script: with none, there is nothing to close.
    const close = document.createElement("button");
    close.type = "button";
    close.className = kind.close;
    close.setAttribute("aria-label", closeLabel);
    close.title = closeLabel;
    close.addEventListener("click", () => hide(true), { signal });
    note.append(close);
  }

  function show() {
    if (showing && showing.close !== hide) showing.close();
    note.hidden = false;
    marker.setAttribute("aria-expanded", "true");
    keepAnchored(note, marker, { room: ROOM }, () => hide(false));
    showing = { root, close: hide };
  }

  /** Closes it, and puts focus back only when the reader asked it to close. */
  function hide(returnFocus) {
    if (note.hidden) return;
    releaseAnchor(note);
    note.hidden = true;
    marker.setAttribute("aria-expanded", "false");
    if (showing?.close === hide) showing = null;
    if (returnFocus) marker.focus();
  }

  marker.addEventListener("click", () => {
    if (note.hidden) show();
    else hide(true);
  }, { signal });

  root.addEventListener("keydown", (event) => {
    if (event.key !== "Escape" || note.hidden) return;
    // Stopped, so a note inside a dialog closes itself first rather than taking
    // the dialog down with it.
    event.stopPropagation();
    hide(true);
  }, { signal });

  // TAB OUT AND IT CLOSES. A note that stays open behind a reader who has moved
  // on is a note floating over content it is not about.
  root.addEventListener("focusout", (event) => {
    if (root.contains(event.relatedTarget)) return;
    hide(false);
  }, { signal });

  document.addEventListener("pointerdown", (event) => {
    if (note.hidden || root.contains(event.target)) return;
    hide(false);
  }, { signal });
}
