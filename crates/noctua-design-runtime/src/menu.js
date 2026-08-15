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

import { keepAnchored, releaseAnchor } from "./anchor.js";

/** How much room a menu wants below its button before it flips above. */
const ROOM = 220;

/** How long a run of keystrokes counts as one word, in milliseconds. */
const TYPING = 700;

/** The menu that is open, so opening a second closes the first. */
let showing = null;

export function enhanceMenus(root = document) {
  for (const button of root.querySelectorAll('[aria-haspopup="menu"][aria-controls]')) {
    if (button.dataset.ndEnhanced) continue;
    const menu = document.getElementById(button.getAttribute("aria-controls"));
    // A button pointing at nothing is left exactly as it is. It is already
    // announcing a menu that does not exist, and hiding that by silently
    // dropping the attributes would remove the evidence rather than the defect.
    if (!menu || !menu.classList.contains("nd-menu")) continue;
    button.dataset.ndEnhanced = "menu";
    enhance(button, menu);
  }
}

function enhance(button, menu) {
  /** Every item the arrows may land on — refused ones included. */
  const items = () => [...menu.querySelectorAll('[role="menuitem"]')];

  let typed = "";
  let typedAt = 0;

  function open(focus) {
    if (showing && showing !== close) showing();
    menu.hidden = false;
    button.setAttribute("aria-expanded", "true");
    keepAnchored(menu, button, { room: ROOM }, () => close(false));
    showing = close;
    // OPENING WITH THE KEYBOARD LANDS ON AN ITEM; opening with a pointer does
    // not. A reader who pressed a key is walking, and one who clicked is
    // reading — moving the focus for the second is taking their place from them.
    if (focus) move(0, 1);
  }

  /** Closes it, and puts focus back only when the reader asked it to close. */
  function close(returnFocus) {
    if (menu.hidden) return;
    releaseAnchor(menu);
    menu.hidden = true;
    button.setAttribute("aria-expanded", "false");
    if (showing === close) showing = null;
    if (returnFocus) button.focus();
  }

  /**
   * Puts focus on one item and takes it off the rest.
   *
   * THE ROVING `tabindex`, which is the same one `Tree` and the data grid use:
   * the menu is one tab stop, and within it the arrows move. Wrapping at both
   * ends is what a menu does and a listbox does not.
   */
  function move(from, step) {
    const all = items();
    if (all.length === 0) return;
    const at = ((from % all.length) + all.length) % all.length;
    for (const item of all) item.tabIndex = -1;
    all[at].tabIndex = 0;
    all[at].focus();
    return step;
  }

  /** Where the focus is now, or the first item. */
  const at = () => Math.max(0, items().indexOf(document.activeElement));

  button.addEventListener("click", () => {
    if (menu.hidden) open(false);
    else close(true);
  });

  button.addEventListener("keydown", (event) => {
    // DOWN OPENS ONTO THE FIRST ITEM AND UP ONTO THE LAST, which is the one
    // keyboard convention a menu button has that an ordinary button does not.
    if (event.key !== "ArrowDown" && event.key !== "ArrowUp") return;
    event.preventDefault();
    open(false);
    move(event.key === "ArrowDown" ? 0 : items().length - 1, 1);
  });

  menu.addEventListener("keydown", (event) => {
    const all = items();
    switch (event.key) {
      case "ArrowDown":
        event.preventDefault();
        move(at() + 1, 1);
        return;
      case "ArrowUp":
        event.preventDefault();
        move(at() - 1, -1);
        return;
      case "Home":
        event.preventDefault();
        move(0, 1);
        return;
      case "End":
        event.preventDefault();
        move(all.length - 1, -1);
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
        move(index, 1);
        return;
      }
    }
  });

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
  });

  // TAB OUT AND IT CLOSES. A menu left open behind a reader who has moved on is
  // a surface floating over content it is not about.
  menu.addEventListener("focusout", (event) => {
    if (menu.contains(event.relatedTarget) || event.relatedTarget === button) return;
    close(false);
  });

  document.addEventListener("pointerdown", (event) => {
    if (menu.hidden || menu.contains(event.target) || button.contains(event.target)) return;
    close(false);
  });
}
