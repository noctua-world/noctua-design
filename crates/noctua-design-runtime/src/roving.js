// One roving-focus model, for a radio group, a toolbar and a tab list.
//
// IT SHIPS, for the reason `menu.js` gives: a component whose consumer cannot
// use it is a component that lies. `Segmented`, `Toolbar` and `Tabs` all write
// a role that PROMISES arrow keys — `radiogroup`, `toolbar`, `tablist` — and
// until this moved out of the documentation site's own script, a consumer who
// installed the packages got the promise and none of the keys.
//
// Twelve groups and forty-two buttons meant forty-two tab stops. A radiogroup
// is ONE stop: Tab enters it, arrows move within it, Tab leaves. That is what
// every native radio group does and what a reader expects from `role="radio"`.
// A `role="toolbar"` promises exactly the same thing one level up, and there is
// one implementation of it here rather than two that disagree at the boundary
// where a `Segmented` sits inside a `Toolbar`.
//
// THE STOP LIST IS FLAT, AND THAT IS THE WHOLE TRICK.
//
// A toolbar containing a radio group has two plausible readings — the group is
// one stop, or each radio is — and the ARIA practices settle it: the radios
// join the toolbar's list, and moving onto one CHOOSES it, because that is what
// a radio group does anyway. So the toolbar needs no special case for the group
// and the group needs none for the toolbar. `.click()` below is what makes
// selection follow focus, and it is a no-op on an ordinary button because
// nothing is listening for a click that did not come from a pointer.
//
// THERE IS NO `releaseRoving`, AND THAT IS NOT AN OMISSION. Every other module
// here holds state per element and has to give it back; this one holds none.
// What it writes is `tabindex` on elements that are being removed anyway, and
// its one listener is on the document, delegated, and does nothing when the
// event lands outside a container that asked for it.

// every native radio group does and what a reader expects from `role="radio"`.
// A `role="toolbar"` promises exactly the same thing one level up, and there is
// one implementation of it here rather than two that disagree at the boundary
// where a `Segmented` sits inside a `Toolbar`.
//
// THE STOP LIST IS FLAT, AND THAT IS THE WHOLE TRICK.
//
// A toolbar containing a radio group has two plausible readings — the group is
// one stop, or each radio is — and the ARIA practices settle it: the radios
// join the toolbar's list, and moving onto one CHOOSES it, because that is what
// a radio group does anyway. So the toolbar needs no special case for the group
// and the group needs none for the toolbar. `.click()` below is what makes
// selection follow focus, and it is a no-op on an ordinary button because
// nothing is listening for a click that did not come from a pointer.

/**
 * Every control inside `container` that the arrows should walk.
 *
 * A TOOLBAR'S LIST IS FLAT AND INCLUDES A NESTED GROUP'S RADIOS. That is the
 * whole reason there is one implementation here rather than two: the ARIA
 * practices settle the ambiguity by having the radios join the toolbar's list,
 * so the toolbar needs no special case for the group and the group needs none
 * for the toolbar. A nested TOOLBAR would own its own stops, and is the one
 * thing excluded.
 *
 * A radiogroup's list is its own radios, and only its own.
 */
export function stops(container) {
  const role = container.getAttribute("role");
  const toolbar = role === "toolbar";
  // A THIRD ROLE, THE SAME WALK. A tab list is one stop with arrows inside it,
  // exactly as a radio group is, and the only place the two differ is whether
  // moving also chooses — which is one line in the handler below rather than a
  // second implementation of everything above it.
  const selector = toolbar
    ? '[role="radio"], button, a[href], summary, [tabindex]'
    : role === "tablist"
      ? '[role="tab"]'
      : '[role="radio"]';
  return [...container.querySelectorAll(selector)].filter((el) => {
    if (el.hasAttribute("disabled")) return false;
    if (el.getAttribute("aria-hidden") === "true") return false;
    // A DISCLOSURE INSIDE A BAR IS ONE STOP: ITS SUMMARY.
    //
    // The example frame's toolbar opens a theme menu with thirty controls in
    // it. They are a region the reader tabs into, not more of the bar — a
    // toolbar whose arrows walked into a panel would be a bar with no end. The
    // summary is the stop; what it opens is not.
    const panel = el.closest("details");
    if (panel && container.contains(panel) && el.tagName !== "SUMMARY") return false;
    if (toolbar) {
      if (el.closest('[role="toolbar"]') !== container) return false;
      // AND A ROLE THAT WALKS ITS OWN CONTENT KEEPS IT. The selector above
      // collects `a[href]`, so every link of a list inside a bar joined the
      // bar's stop list — which nothing on this site did yet, and a `Tree` in a
      // toolbar would have done on the first try. A nested toolbar already got
      // this test; a nested listbox, tree or menu had none.
      const owner = el.closest(
        '[role="tree"], [role="listbox"], [role="menu"], [role="grid"], [role="tablist"]',
      );
      return !(owner && container.contains(owner));
    }
    return el.closest('[role="radiogroup"], [role="tablist"]') === container;
  });
}

/** Puts the one tab stop on the active control and `-1` on the rest. */
export function roving(container) {
  const list = stops(container);
  if (list.length === 0) return;
  // The chosen one if there is one, the one that already had focus otherwise,
  // and the first as a last resort — so leaving a bar and coming back lands
  // where the reader left it.
  // A radio says `aria-checked` and a tab says `aria-selected`. Same question.
  const checked = list.findIndex(
    (el) =>
      el.getAttribute("aria-checked") === "true" || el.getAttribute("aria-selected") === "true",
  );
  const held = list.findIndex((el) => el.tabIndex === 0);
  const active = checked !== -1 ? checked : held !== -1 ? held : 0;
  list.forEach((el, index) => {
    el.tabIndex = index === active ? 0 : -1;
  });
}

document.addEventListener("keydown", (event) => {
  const nearest = event.target?.closest?.('[role="radiogroup"], [role="toolbar"], [role="tablist"]');
  if (!nearest) return;
  // A group inside a toolbar does not own its arrows — the bar does, and its
  // list already contains these radios. Handing them to the group would stop
  // the arrows at the group's edge, halfway along the bar.
  const container = nearest.closest('[role="toolbar"]') ?? nearest;

  const vertical = container.getAttribute("aria-orientation") === "vertical";
  // The axis is honoured rather than ignored: a horizontal bar that answered
  // Up and Down would swallow the keys the reader uses to scroll past it.
  const step = {
    ArrowRight: vertical ? 0 : 1,
    ArrowLeft: vertical ? 0 : -1,
    ArrowDown: vertical ? 1 : 0,
    ArrowUp: vertical ? -1 : 0,
  }[event.key];

  const list = stops(container);
  // The event may land on the glyph inside a control rather than the control,
  // so the stop is whichever one CONTAINS the target.
  const at = list.findIndex((el) => el === event.target || el.contains(event.target));
  if (at === -1) return;

  let next = null;
  if (step) next = list[(at + step + list.length) % list.length];
  else if (event.key === "Home") next = list[0];
  else if (event.key === "End") next = list[list.length - 1];
  if (!next) return;

  event.preventDefault();
  next.focus();
  // Selection follows focus in a radio group, which is the contract the role
  // carries. On a plain toolbar button this reaches no handler.
  if (next.getAttribute("role") === "radio") next.click();
  // AND A TAB IS THE ONE PLACE IT MAY NOT. `manual` is what a panel that costs
  // something to show asks for: the arrows move, and the reader says when.
  else if (
    next.getAttribute("role") === "tab" &&
    container.getAttribute("data-nd-activation") !== "manual"
  ) {
    next.click();
  }
  roving(container);
});


/**
 * Places the single tab stop in every roving container under `root`.
 *
 * Call it after rendering, and again after anything that adds or removes a
 * control inside one: the stop is a property of the list, so a list that
 * changed length has to be counted again.
 */
export function enhanceRoving(root = document) {
  // THE GROUP'S OWN STOP IS THE MARKUP'S, AND IT ENDS HERE.
  //
  // `Segmented` renders `tabindex="0"` on the group, because with no script a
  // group whose items all say `-1` is a radiogroup no keyboard can reach — and
  // an item cannot know whether any of its siblings is chosen. Once this pass
  // has run a real radio holds the stop, so a group that kept its own would be
  // a second one. Inside a toolbar it would be worse: `stops()` collects
  // `[tabindex]`, so the group itself would join the bar's walk.
  for (const group of root.querySelectorAll('[role="radiogroup"][tabindex]')) {
    group.removeAttribute("tabindex");
  }
  // Toolbars first, then the radiogroups that are NOT inside one: a group
  // within a toolbar has already had its radios placed by the toolbar's pass,
  // and running the group's own pass afterwards would put a second `0` on the
  // bar — two tab stops where the role promises one.
  for (const bar of root.querySelectorAll('[role="toolbar"]')) roving(bar);
  for (const group of root.querySelectorAll('[role="radiogroup"], [role="tablist"]')) {
    if (!group.closest('[role="toolbar"]')) roving(group);
  }
}
