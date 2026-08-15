// Everything an `Info` does once a script is running.
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

/** The note that is open, so opening a second closes the first. */
let showing = null;

export function enhanceInfos(root = document) {
  for (const element of root.querySelectorAll("[data-nd-info]")) {
    if (element.dataset.ndEnhanced) continue;
    element.dataset.ndEnhanced = "info";
    enhance(element);
  }
}

function enhance(root) {
  const marker = root.querySelector(".nd-info-marker");
  const note = root.querySelector(".nd-info-note");
  if (!marker || !note) return;

  // REVEALED, NEVER BUILT. The markup ships hidden so a page with no script
  // carries no marker that opens nothing; this is the script arriving.
  marker.hidden = false;

  const closeLabel = root.getAttribute("data-nd-close-label");
  if (closeLabel) {
    // Drawn here rather than in the markup because it is the one control that
    // means nothing without a script: with none, there is nothing to close.
    const close = document.createElement("button");
    close.type = "button";
    close.className = "nd-info-close";
    close.setAttribute("aria-label", closeLabel);
    close.title = closeLabel;
    close.addEventListener("click", () => hide(true));
    note.append(close);
  }

  function show() {
    if (showing && showing !== hide) showing();
    note.hidden = false;
    marker.setAttribute("aria-expanded", "true");
    keepAnchored(note, marker, { room: ROOM }, () => hide(false));
    showing = hide;
  }

  /** Closes it, and puts focus back only when the reader asked it to close. */
  function hide(returnFocus) {
    if (note.hidden) return;
    releaseAnchor(note);
    note.hidden = true;
    marker.setAttribute("aria-expanded", "false");
    if (showing === hide) showing = null;
    if (returnFocus) marker.focus();
  }

  marker.addEventListener("click", () => {
    if (note.hidden) show();
    else hide(true);
  });

  root.addEventListener("keydown", (event) => {
    if (event.key !== "Escape" || note.hidden) return;
    // Stopped, so an `Info` inside a dialog closes its own note first rather
    // than taking the dialog down with it.
    event.stopPropagation();
    hide(true);
  });

  // TAB OUT AND IT CLOSES. A note that stays open behind a reader who has moved
  // on is a note floating over content it is not about.
  root.addEventListener("focusout", (event) => {
    if (root.contains(event.relatedTarget)) return;
    hide(false);
  });

  document.addEventListener("pointerdown", (event) => {
    if (note.hidden || root.contains(event.target)) return;
    hide(false);
  });
}
