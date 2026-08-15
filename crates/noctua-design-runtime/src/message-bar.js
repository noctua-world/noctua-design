// Everything a `MessageBar` does once a script is running, which is one thing.
//
// IT SHIPS, for `ColorPicker`'s reason: a component whose consumer cannot use
// it is a component that lies. `dismissible` renders a control, and a control
// that closes nothing outside this repository's own documentation site is
// exactly the shape of that complaint.
//
// THE CONTROL SHIPS HIDDEN AND THIS REVEALS IT. `Info` established the
// arrangement and it is the same argument here: with no script there is
// nothing to take the bar away, so a page that loaded without one carries no
// button at all rather than a button that does nothing when it is pressed.
//
// WHAT IT NEVER DOES:
//
// 1. It never removes the element. The bar is a live region, and a live region
//    taken out of the document is a region that announces nothing the next
//    time a caller writes into it. `data-nd-dismissed` is a `data-` attribute
//    the stylesheet acts on, which is `ErrorSummary`'s empty state exactly.
// 2. It never leaves focus where the reader cannot see it. The control they
//    pressed is inside the thing they just dismissed, so focus goes to the bar
//    itself and then, on the next Tab, onward into the page.
// 3. It never announces the dismissal. A reader who pressed the control knows
//    what they pressed, and a live region saying so is the page talking about
//    itself.

/** The attribute the stylesheet reads, and the one thing this script writes. */
const DISMISSED = "data-nd-dismissed";

/**
 * Reveals the dismiss control on every bar that has one, and wires it.
 *
 * Idempotent, like every other `enhance*` here: a page that re-renders part of
 * itself calls this again, and a bar already wired is left alone.
 */
/** Every enhanced bar's teardown, so a subtree can be released. */
const bars = new WeakMap();

/**
 * Takes the behaviour back off every bar under `root`.
 *
 * A BEHAVIOUR THAT CANNOT BE RELEASED IS A DEFECT FOR EVERY CONSUMER WITH A
 * CLIENT-SIDE ROUTER. This bar binds one listener to its own control, so what
 * it leaks is small. It is here anyway, because the barrel that releases a
 * subtree may not have a hole in it: a list with one name missing is a list a
 * reader has to check rather than read.
 *
 * The control goes back to hidden, which is the state the markup ships. A
 * control revealed by a script that is no longer running is a control that
 * closes nothing.
 */
export function releaseMessageBars(root = document) {
  for (const element of root.querySelectorAll(".nd-message-bar[data-nd-dismissible='true']")) {
    if (element.dataset.ndEnhanced !== "message-bar") continue;
    const control = element.querySelector("[data-nd-dismiss]");
    if (control) control.hidden = true;
    bars.get(element)?.abort();
    bars.delete(element);
    delete element.dataset.ndEnhanced;
  }
}

export function enhanceMessageBars(root = document) {
  // THE CLASS IS PART OF THE SELECTOR, and it had to become part of it.
  //
  // `[data-nd-dismissible='true']` alone was true of a bar and of nothing else
  // until `Dialog`, `Popover` and `Toast` arrived carrying the same attribute
  // for the same reason. This function ran first, stamped `data-nd-enhanced` on
  // all four, and the three real enhancers then skipped every element it had
  // touched — so a dialog opened nothing and a toast never started its timer.
  // Nothing errored, in the browser or in the build.
  for (const element of root.querySelectorAll(".nd-message-bar[data-nd-dismissible='true']")) {
    if (element.dataset.ndEnhanced) continue;
    element.dataset.ndEnhanced = "message-bar";
    const binding = new AbortController();
    bars.set(element, binding);
    enhance(element, binding.signal);
  }
}

function enhance(bar, signal) {
  const control = bar.querySelector("[data-nd-dismiss]");
  if (!control) return;

  // REVEALED, NEVER BUILT. The markup ships hidden so a page with no script
  // carries no control that closes nothing; this is the script arriving.
  control.hidden = false;

  control.addEventListener("click", () => {
    // WHERE FOCUS GOES, decided BEFORE the bar is hidden. The control the
    // reader pressed is inside the thing they just dismissed, and an element
    // the stylesheet has taken out of the layout cannot be focused — so
    // leaving it alone means the browser sends focus to `<body>` and the next
    // Tab restarts at the top of the document.
    //
    // The bar's parent is the closest thing to "where the reader was", and
    // `tabindex="-1"` makes it focusable without making it a tab stop.
    // `ErrorSummary` uses the same attribute for the same reason.
    const anchor = bar.parentElement;
    if (anchor && !anchor.hasAttribute("tabindex")) anchor.setAttribute("tabindex", "-1");
    bar.setAttribute(DISMISSED, "");
    anchor?.focus({ preventScroll: true });
  }, { signal });

  // Both keys, because the control is a real `<button>` and the platform
  // already answers both. Nothing is added here, and that is the claim the
  // contract's `[keyboard]` table is making.
}
