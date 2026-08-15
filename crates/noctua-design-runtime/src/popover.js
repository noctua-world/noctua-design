// Everything a `Popover` does once a script is running.
//
// IT SHIPS, for `ColorPicker`'s reason and `Info`'s: a component whose consumer
// cannot use it is a component that lies.
//
// THE ANCHORING IS NOT WRITTEN HERE. `anchor.js` already positions eight
// floating surfaces, flips them when there is no room below and closes one
// whose control has scrolled off the screen. This is the ninth caller, not a
// ninth copy.
//
// IT IS `info.js` WITH ONE DIFFERENCE, AND THE DIFFERENCE IS THE POINT OF THE
// COMPONENT. An `Info` note is a `role="note"` holding a sentence. This surface
// is a `role="dialog"` holding controls a reader came to use, so it is named,
// and a screen reader enters it rather than reading it in passing.
//
// WHAT IT NEVER DOES:
//
// 1. It never opens on a hover. `info.js` says why at length, and the reasons
//    are stronger here: a surface with controls in it that appears under a
//    passing pointer is one a reader has to escape from to reach the page.
// 2. It never traps focus. Nothing behind it is inert and the page keeps its
//    scroll, so Tab walks out of it — which is also what closes it. A
//    `role="dialog"` that traps without being modal is the worst of both,
//    because the way out is the one key the reader did not press.
// 3. It never leaves focus where the reader did not put it. Escape and the
//    close control both put it back on the trigger. A press outside does not,
//    because the reader has already chosen where they are going.

import { keepAnchored, releaseAnchor } from "./anchor.js";

/** How much room a surface wants below its trigger before it flips above. */
const ROOM = 240;

// THE SURFACE THAT IS OPEN, SO OPENING A SECOND CLOSES THE FIRST.
//
// IT CARRIES ITS OWN ROOT, WHICH IS WHAT A RELEASE NEEDS. Holding the closure
// alone meant nothing could ask which trigger a surface belonged to. One open
// when its subtree went away left this pointing at a dead trigger for ever, and
// the next surface opened anywhere on the page called it.
let showing = null;

/** Every enhanced surface's teardown, so a subtree can be released. */
const popovers = new WeakMap();

export function enhancePopovers(root = document) {
  for (const element of root.querySelectorAll("[data-nd-popover]")) {
    if (element.dataset.ndEnhanced) continue;
    element.dataset.ndEnhanced = "popover";
    const binding = new AbortController();
    popovers.set(element, binding);
    enhance(element, binding.signal);
  }
}

/**
 * Takes the behaviour back off every surface under `root`.
 *
 * A BEHAVIOUR THAT CANNOT BE RELEASED IS A DEFECT FOR EVERY CONSUMER WITH A
 * CLIENT-SIDE ROUTER. `enhance` registers a `pointerdown` listener on the
 * DOCUMENT, and that listener outlives the trigger it closes: every visit adds
 * another one, holding a detached subtree nothing can collect.
 */
export function releasePopovers(root = document) {
  for (const element of root.querySelectorAll("[data-nd-popover]")) {
    if (element.dataset.ndEnhanced !== "popover") continue;
    if (showing?.root === element) showing = null;
    // AND THE SURFACE GOES BACK TO WHAT THE MARKUP SHIPS. A `role="dialog"`
    // left open with nothing listening is a surface a keyboard cannot leave.
    const panel = element.querySelector(".nd-popover-panel");
    if (panel) {
      releaseAnchor(panel);
      panel.hidden = true;
    }
    const trigger = element.querySelector(".nd-popover-trigger");
    if (trigger) {
      trigger.setAttribute("aria-expanded", "false");
      trigger.hidden = true;
    }
    popovers.get(element)?.abort();
    popovers.delete(element);
    delete element.dataset.ndEnhanced;
  }
}

function enhance(root, signal) {
  const trigger = root.querySelector(".nd-popover-trigger");
  const panel = root.querySelector(".nd-popover-panel");
  if (!trigger || !panel) return;

  // REVEALED, NEVER BUILT. The markup ships hidden so a page with no script
  // carries no control that opens nothing, and the stylesheet holds its box
  // from the first paint so the page does not move when this line runs.
  trigger.hidden = false;

  function show() {
    if (showing && showing.close !== hide) showing.close();
    panel.hidden = false;
    trigger.setAttribute("aria-expanded", "true");
    keepAnchored(panel, trigger, { room: ROOM }, () => hide(false));
    showing = { root, close: hide };
  }

  /** Closes it, and puts focus back only when the reader asked it to close. */
  function hide(returnFocus) {
    if (panel.hidden) return;
    releaseAnchor(panel);
    panel.hidden = true;
    trigger.setAttribute("aria-expanded", "false");
    if (showing?.close === hide) showing = null;
    if (returnFocus) trigger.focus();
  }

  trigger.addEventListener("click", () => {
    if (panel.hidden) show();
    else hide(true);
  }, { signal });

  panel.querySelector("[data-nd-popover-close]")?.addEventListener("click", () => hide(true), { signal });

  root.addEventListener("keydown", (event) => {
    if (event.key !== "Escape" || panel.hidden) return;
    // Stopped, so a popover inside a dialog closes itself first rather than
    // taking the dialog down with it.
    event.stopPropagation();
    hide(true);
  }, { signal });

  // TAB OUT AND IT CLOSES. A surface left open behind a reader who has moved on
  // is a surface floating over content it is not about.
  root.addEventListener("focusout", (event) => {
    if (root.contains(event.relatedTarget)) return;
    hide(false);
  }, { signal });

  document.addEventListener("pointerdown", (event) => {
    if (panel.hidden || root.contains(event.target)) return;
    hide(false);
  }, { signal });
}
