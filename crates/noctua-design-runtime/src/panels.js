// The one open floating list on the page, whatever kind it is.
//
// `Select` held its open list in a module-level `openSelect`, which was right
// while it was the only thing with a list. A second one — a `TagPicker`'s
// suggestions, a `DateInput`'s month — is a second global, and then two panels
// are open at once, both claiming Escape and both claiming the arrow keys.
//
// One registry, and whoever opens closes the other. It lives here rather than
// inside any one of them because the rule is about the page, not about a
// component: three modules that each kept their own would be three that each
// believe theirs is the only one.
//
// This is NOT `anchor.js`. That places a panel and closes it when its control
// scrolls out of the window; this decides which single panel may be open. A
// panel usually wants both.

/** Every close function whose panel is open. */
const openPanels = new Set();

/**
 * Closes every open list except this one's.
 *
 * The caller passes the function that closes it, so this needs to know nothing
 * about what a panel is or which component drew it.
 */
export function claimOpenPanel(close) {
  for (const other of [...openPanels]) {
    if (other !== close) {
      openPanels.delete(other);
      other();
    }
  }
  openPanels.add(close);
}

/** Gives it up again. Safe to call when it was never held. */
export function releaseOpenPanel(close) {
  openPanels.delete(close);
}
