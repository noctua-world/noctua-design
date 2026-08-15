// Everything a `Dialog` does once a script is running.
//
// IT SHIPS, for `ColorPicker`'s reason: a component whose consumer cannot use
// it is a component that lies. And this one has no second way of being true —
// a `<dialog>` opens from `showModal()` or `show()` and from nothing else, so a
// package shipping the surface without this would be shipping a box with no
// door on it.
//
// WHAT THE PLATFORM DOES, AND IS THEREFORE NOT WRITTEN HERE:
//
// The focus trap, the inert background, the focus restoration, Escape and the
// `::backdrop` all come with `showModal()`. `Search { presentation: dialog }`
// established that in this repository and the reasoning stands: a hand-written
// trap around a platform trap is a second implementation of the half that goes
// wrong first, and it is the half a keyboard reader is standing in.
//
// WHAT IT NEVER DOES:
//
// 1. It never traps focus. See above. The one thing it adds is Escape for a
//    NON-modal surface, which the platform answers only for a modal one.
// 2. It never removes the element. A caller who closed a dialog has closed it,
//    and a surface taken out of the document is one nothing can reopen.
// 3. It never moves focus on open. `showModal()` already puts it inside, and a
//    second `focus()` on top of that lands wherever this file guessed rather
//    than where the platform put it.

/** The attribute a stylesheet outside the surface can act on. */
const OPEN = "data-nd-open";

/** Every enhanced dialog's teardown, so a subtree can be released. */
const dialogs = new WeakMap();

export function enhanceDialogs(root = document) {
  for (const element of root.querySelectorAll("[data-nd-dialog]")) {
    if (element.dataset.ndEnhanced) continue;
    element.dataset.ndEnhanced = "dialog";
    const binding = new AbortController();
    dialogs.set(element, binding);
    enhance(element, binding.signal);
  }
}

/**
 * Takes the behaviour back off every dialog under `root`.
 *
 * A BEHAVIOUR THAT CANNOT BE RELEASED IS A DEFECT FOR EVERY CONSUMER WITH A
 * CLIENT-SIDE ROUTER. This file binds nothing outside the element it was given,
 * so what it leaks is smaller than its neighbours: the listeners go with the
 * subtree. What does NOT go is a modal surface left open. `showModal()` marks
 * the rest of the document inert, so a dialog unmounted while open leaves a
 * page a reader cannot touch and cannot see the reason for.
 */
export function releaseDialogs(root = document) {
  for (const element of root.querySelectorAll("[data-nd-dialog]")) {
    if (element.dataset.ndEnhanced !== "dialog") continue;
    const surface = element.querySelector(".nd-dialog-surface");
    if (surface?.open) surface.close();
    element.removeAttribute(OPEN);
    // The trigger goes back to hidden, which is what the markup ships. A
    // control that opens nothing is the defect this file exists to avoid.
    const trigger = element.querySelector(".nd-dialog-open");
    if (trigger) trigger.hidden = true;
    dialogs.get(element)?.abort();
    dialogs.delete(element);
    delete element.dataset.ndEnhanced;
  }
}

function enhance(root, signal) {
  const trigger = root.querySelector(".nd-dialog-open");
  const surface = root.querySelector(".nd-dialog-surface");
  if (!trigger || !surface) return;

  // `showModal()` or `show()`, and the two differ in the accessibility tree
  // rather than only on the screen: the first marks the document behind it
  // inert and the second leaves it reachable.
  const modal = root.getAttribute("data-nd-modal") !== "false";

  // REVEALED, NEVER BUILT. The markup ships hidden so a page with no script
  // carries no control that opens nothing, and the stylesheet holds its box
  // from the first paint so the page does not move when this line runs.
  trigger.hidden = false;

  function open() {
    if (surface.open) return;
    root.setAttribute(OPEN, "");
    if (modal) surface.showModal();
    else surface.show();
  }

  function close() {
    if (surface.open) surface.close();
  }

  trigger.addEventListener("click", open, { signal });
  root.querySelector("[data-nd-dialog-close]")?.addEventListener("click", close, { signal });

  // THE BACKDROP IS THE `<dialog>` ITSELF, which is where a press on it lands.
  // Only for a modal surface: a non-modal one has no backdrop at all, so the
  // same test would close it on a press against its own padding.
  surface.addEventListener("click", (event) => {
    if (modal && event.target === surface) close();
  }, { signal });

  // ESCAPE, FOR THE HALF THE PLATFORM LEAVES OUT. `showModal()` answers it and
  // fires `cancel`; `show()` answers nothing, so a non-modal surface would be a
  // dialog a keyboard reader can only leave by finding the close control.
  //
  // ON THE ROOT, NOT ON THE SURFACE, which is where `popover.js` binds it and
  // why that file has no hole here. The contract lets Tab walk out of an open
  // non-modal dialog and leave it open, and the first place Tab lands on the
  // way out is the trigger — outside the surface, inside the root. Bound to
  // the surface, Escape answered nothing from there, so a reader who tabbed
  // one stop past the close control had no key left that shut it.
  //
  // Only while it is open, or a shut dialog would swallow the Escape meant for
  // something around it. Stopped, so a `Menu` or an `Info` inside the surface
  // closes itself first rather than taking the dialog down with it.
  root.addEventListener("keydown", (event) => {
    if (modal || event.key !== "Escape" || !surface.open) return;
    event.stopPropagation();
    close();
  }, { signal });

  surface.addEventListener("close", () => {
    root.removeAttribute(OPEN);
    // WHERE FOCUS GOES. `showModal()` records where the reader was and puts
    // them back. `show()` records nothing, so focus falls to `<body>` and the
    // next Tab restarts at the top of the document — which is the one thing
    // this file exists to prevent everywhere else.
    if (!modal && document.activeElement === document.body) trigger.focus();
  }, { signal });
}
