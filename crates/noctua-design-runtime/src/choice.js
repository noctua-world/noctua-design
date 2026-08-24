// What a `Switch` and a `Segmented` do when they are pressed.
//
// IT SHIPS, and it is the plainest case of the rule `menu.js` states: a
// component whose consumer cannot use it is a component that lies. A `Switch`
// is a control whose entire purpose is to be turned on, and until this moved
// out of the documentation site's own script a consumer who installed the
// packages got `role="switch"`, `aria-checked="false"`, and a control that
// stayed false whatever a reader did to it. `Segmented` was the same.
//
// WHAT IT DOES NOT DO: decide where the chosen value goes. That is the host's,
// and the host hears about it — every change dispatches a `change` event that
// bubbles, carrying the group's name and the value now in force. The
// documentation site listens for it and routes the setting into a theme scope
// or into storage; a consumer listens for it and does whatever it is their
// application does. A component that knew either would be a component with an
// application inside it.
//
// THE KEYBOARD IS FREE, AND THAT IS WHY BOTH ARE BUTTONS. `Space` and `Enter`
// fire a click on a `<button>`, so the contract's `[keyboard]` is kept by the
// platform rather than by a handler here — which is also what makes it survive
// a reader who uses neither.

import { roving } from "./roving.js";

/** Every root this is installed on, so it is installed once. */
const installed = new WeakMap();

/**
 * The clicks already answered, so a nested root does not answer twice.
 *
 * A HOST MAY ENHANCE A SUBTREE AND THE DOCUMENT, and this is delegated: React
 * mounts an example and enhances the slot it mounted into, while the page
 * enhanced the document once at load. One click then reached two listeners, the
 * switch flipped twice, and it read as a control that does nothing at all —
 * which is a worse defect than no behaviour, because it looks like the reader
 * missed. The event itself is the flag: it is one object, whatever it passes.
 */
const answered = new WeakSet();

/**
 * Turns a switch on or off, and marks the chosen item in a group.
 *
 * One delegated listener rather than one per control: a group's items are
 * markup the host may replace, and a listener per item is a listener per item
 * to remove.
 */
export function enhanceChoices(root = document) {
  if (installed.has(root)) return;
  const binding = new AbortController();
  installed.set(root, binding);

  root.addEventListener(
    "click",
    (event) => {
      if (answered.has(event)) return;
      const flip = event.target.closest?.('[role="switch"][data-nd-group]');
      if (flip) {
        // A refused control is reachable on purpose — `aria-disabled` rather
        // than `disabled` keeps its tab stop so a reader hears what it is — and
        // it must still refuse.
        if (flip.getAttribute("aria-disabled") === "true") return;
        answered.add(event);
        const on = flip.getAttribute("aria-checked") === "true";
        flip.setAttribute("aria-checked", String(!on));
        // IT CARRIES BOTH VALUES, so the event says which is in force rather
        // than composing one: `on` and `off` are the host's words.
        announce(flip, flip.dataset.ndGroup, on ? flip.dataset.ndOff : flip.dataset.ndOn);
        return;
      }

      const item = event.target.closest?.("[data-nd-value]");
      const group = item?.closest("[data-nd-group]");
      if (!item || !group) return;
      if (item.getAttribute("aria-disabled") === "true") return;
      answered.add(event);
      for (const other of group.querySelectorAll("[data-nd-value]")) {
        other.setAttribute("aria-checked", String(other === item));
      }
      // THE TAB STOP FOLLOWS THE CHOICE. A radio group is one stop and it sits
      // on the chosen radio, so choosing moves it — `roving.js` is what knows
      // that, and this is its eighth caller rather than a second copy.
      roving(group);
      announce(group, group.dataset.ndGroup, item.dataset.ndValue);
    },
    { signal: binding.signal },
  );
}

/** Takes the behaviour back off. */
export function releaseChoices(root = document) {
  installed.get(root)?.abort();
  installed.delete(root);
}

/**
 * Says what changed, on the control that changed.
 *
 * A `change` event because that is the name the platform already uses for
 * exactly this, so a host that knows the web knows this without being told.
 * `detail` carries the group's name and the value, which is what a host needs
 * and what reading the DOM back would otherwise cost them.
 */
function announce(target, name, value) {
  target.dispatchEvent(
    new CustomEvent("change", { bubbles: true, detail: { name, value } }),
  );
}
