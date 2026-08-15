// The site's URLs, in one place.
//
// Every spec imports from here rather than typing a path, so changing the URL
// scheme is one edit instead of forty-eight. It was forty-eight, spread over
// six files, and that is what made a wrong URL cheap to ship and expensive to
// correct.
//
// There is deliberately no entry for `/<locale>/`. It is a redirect document —
// a door, not a page — and a spec that navigates to it loses its execution
// context the moment the meta refresh fires, which surfaces as the useless
// "Execution context was destroyed" rather than as anything about the site.

/** The locales that are built, including the pseudo-locale. */
export const LOCALES = ["en", "pt-BR", "qps"];

/** The locales a reader is offered. `qps` exists to be inspected, not read. */
export const OFFERED = ["en", "pt-BR"];

/** Every real page, by the name a spec wants to say. */
export const PAGE = {
  intro: "intro/",
  icons: "icons/",
  typography: "typography/",
  // The spacing scale. It is on this list for the same reason `tables` is: the
  // width x locale sweep and the no-shadow sweep only cover the pages named
  // here, and this one draws a bar whose width IS a token.
  spacing: "spacing/",
  // The code block. On this list because the width x locale sweep, the radius
  // sweep and the no-shadow sweep only cover the pages named here — and this
  // one has a box that scrolls, which is what `material.spec.js` is looking
  // for when it asserts that a scrollable element carries no radius.
  codeBlock: "code-block/",
  // The scene. On this list because the width x locale sweep and the no-shadow
  // sweep only cover the pages named here — and this is the one page that
  // draws in three dimensions, where "nothing scrolls sideways" is a claim
  // about a scroll container rather than about a document.
  lineage: "lineage/",
  themeScope: "theme-scope/",
  exampleFrame: "example-frame/",
  // The page whose examples are controls: several of them declare `glass`, so
  // this is where "frost everything that can be frosted" has anything to do.
  button: "button/",
  // One component page, for a spec about what a component page IS rather than
  // about any particular component.
  component: "heading/",
  // The six controls that were one `/forms` page. Each is on this list because
  // the width x locale sweep, the radius sweep and the no-shadow sweep only
  // cover the pages named here — and a page dropped from this list is a page
  // three suites quietly stop looking at.
  dateInput: "date-input/",
  checkbox: "checkbox/",
  radio: "radio/",
  fieldset: "fieldset/",
  select: "select/",
  selectGroup: "select-group/",
  selectOption: "select-option/",
  // The search, and the same argument: it is the one page with three floating
  // presentations on it, a `<dialog>`, and a box whose panel scrolls — so the
  // radius sweep, the width x locale sweep and the no-shadow sweep all have
  // something here they can find nowhere else.
  search: "search/",
  // The pill and the count. Both are on the list because the width x locale
  // sweep, the radius sweep and the no-shadow sweep only cover the pages named
  // here — and `Tag` is the one component that renders as three different
  // elements depending on what it was asked for.
  tag: "tag/",
  badge: "badge/",
  // The people. On this list because the width x locale sweep and the radius
  // sweep only cover the pages named here, and because an `AvatarGroup` is the
  // one component whose members deliberately overlap — which is a geometry
  // question those sweeps are the only thing asking.
  avatars: "avatars/",
  // The token input. On this list because it is the one control on the site
  // whose field GROWS — the sweeps that watch for overflow and overlap have
  // nothing else to look at that changes its own height as a reader types.
  tagPicker: "tag-picker/",
  // The colour picker. On this list because it is the one control whose panel
  // is painted from values a script computes rather than from tokens — so the
  // width x locale sweep, the radius sweep and the no-shadow sweep have
  // something here that nothing else on the site can give them, and because
  // it is the second component whose behaviour ships from `dist/`.
  colorPicker: "color-picker/",
  // The seven input controls. Every one of them is on this list because the
  // width x locale sweep, the radius sweep and the no-shadow sweep only cover
  // the pages named here — and because `Field` is what draws the question
  // above all of them, so a page missing here is a page those sweeps stop
  // asking about the arrangement every form in this system uses.
  field: "field/",
  input: "input/",
  textarea: "textarea/",
  numberInput: "number-input/",
  slider: "slider/",
  fileInput: "file-input/",
  timeInput: "time-input/",
  // The carousel. On this list because it is the only component on the site
  // that SCROLLS ITS OWN CONTENT sideways and the only one that moves without
  // being touched — so `material.spec.js`'s scroll-and-radius sweep and
  // `theme.spec.js`'s reduced-motion sweep have something here they can find
  // nowhere else.
  carousel: "carousel/",
  // The error summary. On this list because it is the only component that
  // takes the focus without being asked, and the only box on the site drawn
  // entirely in the danger tone — so the width x locale sweep, the radius
  // sweep and the no-shadow sweep have something here they can find nowhere
  // else, and the contrast sweep has the one surface where a whole component
  // is red on the page's own background.
  errorSummary: "error-summary/",
  // The hierarchy. On this list because it is the only component whose whole
  // contract is what it does NOT claim without a script, and the no-script
  // sweeps only look at the pages named here.
  tree: "tree/",
  // The `i` and its note. On this list because it is the SIXTH floating
  // surface in the system and the only one that hangs off a word in running
  // text rather than off a form control — the radius sweep and the no-shadow
  // sweep have a panel here that nothing else on the site gives them.
  info: "info/",
  // The menu and its items — the SEVENTH floating surface, and the only one
  // whose contents are a walk: `role="menu"` takes the arrow keys, so this is
  // the one page where the width x locale sweep is measuring a panel that a
  // reader moves through rather than reads.
  menu: "menu/",
  // The two bars. On this list because they are the only components on the
  // site whose fill is painted by the PLATFORM rather than by this stylesheet —
  // `<meter>` and `<progress>` have separate shadow trees and no engine agrees
  // with another about their pseudo-element names — and because the
  // indeterminate sweep is the one animation `theme.spec.js`'s reduced-motion
  // sweep has to find outside the carousel.
  meter: "meter/",
  progress: "progress/",
  // Paging, on a page of its own. It left `/tables/` because a reader who
  // clicked *Table* in the sidebar got the whole of `Pagination` first — and
  // because a component on a collection page is absent from the site's own
  // search index, so nobody could find `spread` by searching for it.
  pagination: "pagination/",
  // The tables. Naming it here is not only for
  // the specs that are about tables: it is what puts the family into
  // `responsive.spec.js`'s width x locale sweep, `material.spec.js`'s
  // scroll-and-radius sweep and `shell.spec.js`'s no-shadow sweep. An
  // assertion over "every element on the page" only ever covers the elements
  // that are on it — and this page now carries the only `role="grid"` in the
  // system, the only container query, and the only component that writes into
  // a `<colgroup>`.
  tables: "tables/",
  // The rule between two blocks. On this list because the width x locale sweep
  // and the no-shadow sweep only cover the pages named here — and this is the
  // one component in the system whose whole drawing IS a border, so the
  // contrast sweep has an edge here with no fill behind it.
  divider: "divider/",
  // The one label. On this list because `Field` composes it, so a page missing
  // here is a page those sweeps stop asking about the question above every
  // control in the system — and because it is the only component that renders
  // three different elements from one enum.
  label: "label/",
  // The picture. On this list because it is the only component that reserves a
  // box for something that has not arrived, and the width x locale sweep is
  // the only thing that can tell a held box from a collapsed one.
  image: "image/",
  // The bar. On this list because it is the second component painted entirely
  // from a tone — `ErrorSummary` is the first — and the only one that carries
  // a live region a reader can dismiss, so the contrast sweep and the
  // no-shadow sweep both have something here they find nowhere else.
  messageBar: "message-bar/",
  // The component that draws the drawings. `icons` above is the GALLERY —
  // which drawing means what — and this is the component's own page: the three
  // sizes, and what an unlabelled one promises assistive technology. Both are
  // on this list because the width x locale sweep and the no-shadow sweep only
  // cover the pages named here.
  icon: "icon/",
  // The stars. On this list because it is the only control in the system whose
  // value follows a reader's choice with NO script at all, so the no-script
  // sweeps have something here they can find nowhere else.
  rating: "rating/",
  // The row of boxes. On this list because it is the only field whose boxes all
  // post under one name, and the second control that walks a row of segments.
  otpField: "otp-field/",
  // The strip and its panels. On this list because it is the only component
  // that claims three roles a script writes, and the only one whose scriptless
  // form is a table of contents rather than the component itself.
  tabs: "tabs/",
  // The four floating surfaces. Each is on this list because the width x locale
  // sweep, the radius sweep and the no-script sweep only cover the pages named
  // here, and because between them they carry the only things of their kind on
  // the site: the second `<dialog>` in the system and the only non-modal one,
  // the only `role="dialog"` that is anchored rather than centred, the only
  // `role="tooltip"`, and the only element that is `position: fixed` and
  // visible from the moment the page loads.
  dialog: "dialog/",
  popover: "popover/",
  tooltip: "tooltip/",
  toast: "toast/",
};

/**
 * The page carrying the scoped-theme demonstration.
 *
 * `ThemeScope`'s own page, which is where it always belonged: it is a
 * component, and the sidebar is a list of components. The alias stays so a spec
 * about scoping reads as though it is about scoping.
 */
export const SCOPED = PAGE.themeScope;

/** The URL of a page in a locale. */
export const url = (page, locale = "en") => `/${locale}/${page}`;

/**
 * A page for tests about the *shell* rather than about content.
 *
 * The introduction on purpose: it is the lightest of them — one paragraph, no
 * examples, no font specimens — so a test about the header is not also a test
 * of everything else.
 */
export const ANY = PAGE.intro;

/**
 * The page's theme menu, opened.
 *
 * There used to be a mode control in the header *and* a mode row in the panel,
 * and a spec could pick either. There is one now, and every spec that changes
 * the page's appearance comes through here — so when the menu moves again,
 * this is the edit.
 */
export async function openTheme(page) {
  const panel = page.locator(".nd-settings");
  if ((await panel.getAttribute("open")) === null) {
    await panel.locator("summary").click();
  }
  return panel.locator(".nd-panel");
}

/** The page's mode control, which lives in that menu. */
export async function modeControl(page) {
  return (await openTheme(page)).locator('[data-nd-group="mode"]');
}

/**
 * Every folded section of a page, opened.
 *
 * A collection page — `/tables/`, `/search/`, `/carousel/` — ships one folded
 * section per component, and the contract tables under every component fold
 * too. That is what makes `/tables/` a page rather than sixty-three screens.
 *
 * A spec about what a component DOES is not a spec about the fold in front of
 * it, so it opens everything first and then asks its own question. That the
 * folds are shut by default, that each keeps a real heading while shut, and
 * that following a link into one opens it are asserted on their own, in
 * `audit.spec.js` and `example.spec.js`.
 */
export async function showParts(page) {
  await page.evaluate(() => {
    for (const fold of document.querySelectorAll(".nd-doc-part, .nd-doc-fold")) {
      fold.setAttribute("open", "");
    }
  });
}

/**
 * Puts one frame's stage at a named width.
 *
 * THE WIDTH IS A MENU NOW, not a row of five stops. Five stops side by side is
 * the better shape and does not fit in a rail 112px wide, so the control is one
 * button that opens the list — which means setting a width is two interactions
 * rather than one, here as for a reader.
 */
export async function setFrameWidth(page, value, index = 0) {
  const frame = page.locator("[data-nd-frame]").nth(index);
  const trigger = frame.locator(".nd-frame-width").first();
  await trigger.click();
  const menu = page.locator(`#${await trigger.getAttribute("aria-controls")}`);
  await menu.locator(`[data-nd-value="${value}"]`).click();
  // The stage animates to its new width; the claim is about where it lands.
  await page.waitForTimeout(300);
}

/**
 * A frame's own theme menu, opened, and the panel it opens.
 *
 * The frame's menu is the same component on the same surface as the header's,
 * so a spec that wants "the menu" says which one and gets the same shape.
 */
export async function openFrameTheme(page, index = 0) {
  const menu = page.locator("[data-nd-disclosure='frame-theme']").nth(index);
  if ((await menu.getAttribute("open")) === null) {
    await menu.locator("summary").click();
  }
  return menu.locator(".nd-panel");
}

/** Closes it again. A floating panel swallows the clicks behind it. */
export async function closeFrameTheme(page, index = 0) {
  const menu = page.locator("[data-nd-disclosure='frame-theme']").nth(index);
  if ((await menu.getAttribute("open")) !== null) {
    await menu.locator("summary").click();
  }
}

/** Closes it again. A floating panel swallows the clicks behind it. */
export async function closeTheme(page) {
  const panel = page.locator(".nd-settings");
  if ((await panel.getAttribute("open")) !== null) {
    await panel.locator("summary").click();
  }
}
