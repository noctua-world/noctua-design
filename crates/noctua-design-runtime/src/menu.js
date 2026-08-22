// Everything a `Menu` does once a script is running.
//
// IT SHIPS, for `ColorPicker`'s reason and `Info`'s: a component whose consumer
// cannot use it is a component that lies. And this one lied louder than most —
// `Button { menu: true }` wrote `aria-haspopup="menu"`, drew a chevron and
// announced a menu to a screen reader while there was nothing in the system
// that could open one. A control that does nothing is worse than a missing one.
//
// THE ANCHORING IS NOT WRITTEN HERE. `anchor.js` already positions six floating
// surfaces and this is the seventh caller, not a seventh copy.
//
// WHAT IT NEVER DOES:
//
// 1. It never opens on a hover. `Info` says why at length, and a menu has the
//    same reasons plus one of its own: a menu that opens under a passing
//    pointer covers the thing the reader was on their way to.
// 2. It never traps focus. Nothing behind it is inert and the page keeps its
//    scroll; walking out with Tab is one of the ways it closes.
// 3. It never leaves focus where the reader did not put it. Escape, a chosen
//    item and a click outside all put focus back on the button — a reader who
//    acted and found themselves at the top of the document was moved by the
//    thing they were using.
// 4. It never makes an item a tab stop. A menu owns the arrow keys; a Tab per
//    item is a menu a keyboard reader walks THROUGH rather than past.
//
// THREE THINGS ABOUT THE MARKUP THAT ARE EASY TO GET BACKWARDS:
//
//   * `aria-disabled` ON AN ITEM, `disabled` ON A BUTTON. An item that leaves
//     the arrow walk is an action a reader concludes does not exist.
//   * `aria-expanded="false"` IS TRUE ON A MENU BUTTON — the one exception to
//     `Button`'s write-it-only-when-true rule. A button that announces a menu
//     has to say the menu is shut, not say nothing.
//   * ESCAPE HAS TO BE HEARD ON THE BUTTON, NOT ONLY ON THE SURFACE. Opening
//     with a pointer leaves focus on the button deliberately, so a handler only
//     on the menu leaves that reader no key that puts it away. It is on both,
//     with `stopPropagation` on each, so a menu inside a dialog closes itself
//     rather than the dialog.
//
// AND A MENU BUTTON DRAWS ONE CHEVRON, WHOEVER DREW IT. `menu` draws it and
// steps aside for an `icon` or an `icon_end` naming the same drawing — both
// positions, or a split button ships two side by side.

import { keepAnchored, releaseAnchor } from "./anchor.js";

/** How much room a menu wants below its button before it flips above. */
const ROOM = 220;

/** How long a run of keystrokes counts as one word, in milliseconds. */
const TYPING = 700;

// THE MENU THAT IS OPEN, SO OPENING A SECOND CLOSES THE FIRST.
//
// IT CARRIES ITS BUTTON, WHICH IS WHAT A RELEASE NEEDS. Holding the closure
// alone meant nothing could ask which button a menu belonged to. So a menu open
// when its subtree went away left this pointing at a dead button for ever, and
// the next menu opened anywhere on the page called it.
let showing = null;

/** Every enhanced button's teardown, so a subtree can be released. */
const menus = new WeakMap();

export function enhanceMenus(root = document) {
  for (const button of root.querySelectorAll('[aria-haspopup="menu"][aria-controls]')) {
    if (button.dataset.ndEnhanced) continue;
    const menu = document.getElementById(button.getAttribute("aria-controls"));
    // A button pointing at nothing is left exactly as it is, which now means
    // left hidden: the component renders a menu button with `hidden` and this
    // file takes it off. Nothing anywhere drops the attributes and pretends the
    // markup was right, so the evidence is still there for anyone reading it.
    if (!menu || !menu.classList.contains("nd-menu")) continue;
    button.dataset.ndEnhanced = "menu";
    const binding = new AbortController();
    menus.set(button, binding);
    enhance(button, menu, binding.signal);
  }
}

/**
 * Takes the behaviour back off every menu under `root`.
 *
 * A BEHAVIOUR THAT CANNOT BE RELEASED IS A DEFECT FOR EVERY CONSUMER WITH A
 * CLIENT-SIDE ROUTER. It is not a gap in one framework's story. `enhance`
 * registers a `pointerdown` listener on the DOCUMENT, which outlives the button
 * it closes: route away and back ten times and the document carries ten of
 * them, each holding a detached subtree nothing can ever collect.
 *
 * It is the exact mirror of `enhanceMenus`, selector included, so a menu the
 * one reached is a menu the other reaches.
 */
export function releaseMenus(root = document) {
  for (const button of root.querySelectorAll('[aria-haspopup="menu"][aria-controls]')) {
    if (button.dataset.ndEnhanced !== "menu") continue;
    // The open menu is forgotten BEFORE the listeners go. Left set, the next
    // menu opened anywhere calls a closure over a button nobody can see.
    if (showing?.button === button) showing = null;
    // AND THE SURFACE GOES BACK TO WHAT THE MARKUP SHIPS. An open menu whose
    // listeners have been taken off is worse than a closed one: Escape answers
    // nothing, a press outside answers nothing, and the reader is left with a
    // panel over the page that has no way out. The button goes back to hidden
    // for the reason this file opens with, which is that a control announcing a
    // menu nothing can open is a control that lies.
    const menu = document.getElementById(button.getAttribute("aria-controls"));
    if (menu) {
      releaseAnchor(menu);
      menu.hidden = true;
    }
    button.setAttribute("aria-expanded", "false");
    button.hidden = true;
    menus.get(button)?.abort();
    menus.delete(button);
    delete button.dataset.ndEnhanced;
  }
}

function enhance(button, menu, signal) {
  // REVEALED, NEVER BUILT, which is `Dialog`'s line and `Popover`'s. The
  // component renders the button hidden, so a page with no script carries no
  // control announcing a menu nothing can open — which is the defect named at
  // the top of this file, and which the button itself committed for as long as
  // only the menu was hidden.
  button.hidden = false;

  /** Every item the arrows may land on — refused ones included. */
  const items = () => [...menu.querySelectorAll('[role="menuitem"]')];

  let typed = "";
  let typedAt = 0;

  function open() {
    if (showing && showing.close !== close) showing.close();
    menu.hidden = false;
    button.setAttribute("aria-expanded", "true");
    keepAnchored(menu, button, { room: ROOM }, () => close(false));
    showing = { button, close };
    // AN OPEN MENU IS A TAB STOP, WHOEVER OPENED IT.
    //
    // Every item ships `tabindex="-1"` and only `move` ever wrote a `0` — and
    // the pointer path never called it. So a menu opened with a click had no
    // stop of its own: Tab walked straight past the whole surface, `focusout`
    // never fired, and the menu stayed open behind a reader who had moved on.
    // The stop is placed and the focus is NOT: a reader who pressed a key is
    // walking, and one who clicked is reading.
    //
    // This replaces a `focus` parameter nothing ever passed as true. A branch
    // no call site takes is not a decision, and the keyboard path hand-rolled
    // its own `move` around it.
    seed(0);
  }

  /** Closes it, and puts focus back only when the reader asked it to close. */
  function close(returnFocus) {
    if (menu.hidden) return;
    releaseAnchor(menu);
    menu.hidden = true;
    button.setAttribute("aria-expanded", "false");
    if (showing?.close === close) showing = null;
    if (returnFocus) button.focus();
  }

  /**
   * Puts the one tab stop on one item and takes it off the rest.
   *
   * THE ROVING `tabindex`, which is the same one `Tree` and the data grid use:
   * the menu is one tab stop, and within it the arrows move. Wrapping at both
   * ends is what a menu does and a listbox does not.
   *
   * It moves no focus, which is what lets `open` call it: placing the stop and
   * taking the reader's place are two decisions, and only one of them belongs
   * to a pointer.
   */
  function seed(from) {
    const all = items();
    if (all.length === 0) return -1;
    const index = ((from % all.length) + all.length) % all.length;
    for (const item of all) item.tabIndex = -1;
    all[index].tabIndex = 0;
    return index;
  }

  /** The same, and the focus goes with it. */
  function move(from) {
    const all = items();
    const index = seed(from);
    if (index >= 0) all[index].focus();
  }

  /** Where the focus is now, or the first item. */
  const at = () => Math.max(0, items().indexOf(document.activeElement));

  button.addEventListener("click", () => {
    if (menu.hidden) open();
    else close(true);
  }, { signal });

  button.addEventListener("keydown", (event) => {
    // ESCAPE CLOSES IT FROM THE BUTTON TOO, and it had nowhere to be heard
    // from there. Opening with a POINTER leaves the focus on the button, which
    // is deliberate — a reader who clicked is reading — and the only Escape
    // handler was on the menu, so that reader had no key that would put the
    // surface away. Stopped for the menu's reason: a menu inside a dialog
    // closes itself rather than taking the dialog down with it.
    if (event.key === "Escape" && !menu.hidden) {
      event.stopPropagation();
      close(true);
      return;
    }
    // DOWN OPENS ONTO THE FIRST ITEM AND UP ONTO THE LAST, which is the one
    // keyboard convention a menu button has that an ordinary button does not.
    if (event.key !== "ArrowDown" && event.key !== "ArrowUp") return;
    event.preventDefault();
    open();
    move(event.key === "ArrowDown" ? 0 : items().length - 1);
  }, { signal });

  menu.addEventListener("keydown", (event) => {
    const all = items();
    switch (event.key) {
      case "ArrowDown":
        event.preventDefault();
        move(at() + 1);
        return;
      case "ArrowUp":
        event.preventDefault();
        move(at() - 1);
        return;
      case "Home":
        event.preventDefault();
        move(0);
        return;
      case "End":
        event.preventDefault();
        move(all.length - 1);
        return;
      case "Escape":
        // Stopped, so a menu inside a dialog closes itself first rather than
        // taking the dialog down with it.
        event.stopPropagation();
        close(true);
        return;
      case "Tab":
        close(false);
        return;
      default:
        break;
    }
    // TYPE-AHEAD, and it is a WORD rather than a letter: pressing `d` twice in
    // a menu holding *Duplicate* and *Delete* should reach *Delete*, not walk
    // between them. `tree.js` reached the same answer for the same reason.
    if (event.key.length !== 1 || event.altKey || event.ctrlKey || event.metaKey) return;
    const now = event.timeStamp;
    typed = now - typedAt > TYPING ? event.key : typed + event.key;
    typedAt = now;
    const word = typed.toLowerCase();
    const from = at() + (typed.length > 1 ? 0 : 1);
    for (let step = 0; step < all.length; step += 1) {
      const index = (from + step) % all.length;
      if (all[index].textContent.trim().toLowerCase().startsWith(word)) {
        move(index);
        return;
      }
    }
  }, { signal });

  menu.addEventListener("click", (event) => {
    const item = event.target.closest?.('[role="menuitem"]');
    if (!item || !menu.contains(item)) return;
    // A REFUSED ITEM IS REACHABLE AND INERT. `aria-disabled` keeps it in the
    // walk so a reader is told the action is unavailable; the refusal itself
    // has to be made here, because the platform only enforces `disabled`.
    if (item.getAttribute("aria-disabled") === "true") {
      event.preventDefault();
      event.stopPropagation();
      return;
    }
    // Closed before whatever the caller bound to it runs, so an item that opens
    // a dialog does not leave a menu hanging behind it.
    close(true);
  }, { signal });

  // TAB OUT AND IT CLOSES. A menu left open behind a reader who has moved on is
  // a surface floating over content it is not about.
  menu.addEventListener("focusout", (event) => {
    if (menu.contains(event.relatedTarget) || event.relatedTarget === button) return;
    close(false);
  }, { signal });

  document.addEventListener("pointerdown", (event) => {
    if (menu.hidden || menu.contains(event.target) || button.contains(event.target)) return;
    close(false);
  }, { signal });
}
