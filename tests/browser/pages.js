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
//
// And the walks over all of it, `walker` at the bottom, so that "every page"
// is written in one place too.

import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

import { expect, test } from "@playwright/test";

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
  // The four controls that were one `/forms` page. Each is on this list because
  // the width x locale sweep, the radius sweep and the no-shadow sweep only
  // cover the pages named here — and a page dropped from this list is a page
  // three suites quietly stop looking at.
  dateInput: "date-input/",
  // The only `role="grid"` outside `/tables/`, the only page that paints a
  // range across cells, and the only control whose panel holds a second roving
  // tab stop. A page dropped from this list is a page four sweeps stop looking
  // at, and there is nowhere else those three are measured.
  datePicker: "date-picker/",
  checkbox: "checkbox/",
  radio: "radio/",
  fieldset: "fieldset/",
  // The select, its groups and its options, on one page. `select-group/` and
  // `select-option/` were rows of their own until the three were merged: an
  // `<optgroup>` outside a `<select>` is not a control, so neither was ever a
  // page a reader looked up on its own.
  select: "select/",
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
  // The assistant's mark, and the note it opens. On this list because it is the
  // only control on the site whose trigger is two letters of TRANSLATED text —
  // two glyphs in English, two different ones in Portuguese and six in `qps` —
  // so the width x locale sweep has a control here whose own name triples, and
  // the 320px audit has a hit target that has to survive it.
  aiLabel: "ai-label/",
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
  // clicked *Table* in the sidebar got the whole of `Pagination` first. The
  // other half of that argument — that a component on a shared page was absent
  // from the site's own search — is fixed in `index.rs` rather than still true.
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
  // The switch. On this list because every switch on it is a control the
  // settings model has never heard of, which is the one arrangement where
  // `shell.js` has to answer for a control it cannot route — and it did not,
  // so all eight of them moved nothing.
  switchControl: "switch/",
  // The bar of controls. On this list because it is the only place where a
  // roving tab stop is claimed over children the component never sees, so it
  // is where the claim and the markup can be held against each other.
  toolbar: "toolbar/",
  // THE RADIO GROUP AND ITS ITEMS, AND THEY WERE MISSING. Five whole-site
  // sweeps — the width x locale walk, the radius sweep, the no-shadow sweep,
  // the 320px audit and the floating-surface check — only ever open the pages
  // named here, so a page absent from this list is a page they have never
  // looked at. These two are the one place a roving tab stop is server
  // rendered, which is exactly the claim a sweep with the script blocked can
  // settle and nothing else on the list can.
  // `segmented-item/` was a second row here and is one page with this one now.
  segmented: "segmented/",
  // The list and its items. On this list because it is the one component whose
  // whole subject is DEPTH — a list inside a list inside a list — so the width
  // x locale sweep and the 320px audit have indentation here that nothing else
  // on the site can give them.
  list: "list/",
  // The name a query matched. On this list because it is the one component
  // whose whole drawing is a run of letters inside a word, so the readability
  // sweep has three tints here with ink on each that nothing else gives it.
  matchText: "match-text/",
  // The list a reader walks with the keyboard. On this list because it is the
  // only multi-column box in the system, so the width x locale sweep and the
  // 320px audit have a layout here that changes its own column count.
  optionList: "option-list/",
  // The footer of an application window. On this list because it is the one
  // component drawn edge to edge in four shades of one colour, and the only
  // one whose corners take a fixed width out of a 320px line.
  footer: "footer/",
  // The drawer of an application window. On this list because it is the one
  // component whose size is a share of the box around it, and the one whose
  // two rails take a fixed width out of a 320px window from both sides.
  drawer: "drawer/",
  // The split. On this list because it is the one component laid out by a
  // query on its own box, so the width x locale sweep has a layout here that
  // changes with the column it is in and not with the window.
  split: "split/",
  // The line a reader types on. On this list because it is the one control
  // whose cursor this system draws, over a second copy of its own text, so the
  // width x locale sweep has a box here whose two layers must wrap as one.
  prompt: "prompt/",
  // The control that records a shortcut. On this list because its label, its
  // box of key caps and its remove control share one row that wraps, and the
  // caller's sentences in it run longest in `qps`, so the width x locale sweep
  // has a row here that must break rather than spill.
  keyCapture: "key-capture/",
  // The log. On this list because it is the one region whose height is the
  // box around it and whose scroll position a script keeps at the end, so the
  // width x locale sweep has lines here that must wrap inside a window.
  log: "log/",
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
 * Every link the built sidebar carries, per locale, as `[slug, href]` pairs.
 *
 * READ FROM THE FILE THE SERVER HANDS THE BROWSER — the built `ANY` page, out
 * of the directory `playwright.config.js` serves — because a walk over the
 * sidebar is generated as one test per link, and the tests have to exist
 * before any browser does. The walk that uses it holds this list against the
 * sidebar the browser actually reads, so a link this misreads is a failing
 * test rather than a link nobody walks.
 *
 * A site that was never built reads as no links at all, and a walk's guard
 * refuses a walk with nothing to visit, so that is a failure that says what it
 * is rather than a file that will not load.
 */
export const SIDEBAR = Object.fromEntries(LOCALES.map((locale) => [locale, sidebar(locale)]));

function sidebar(locale) {
  const file = fileURLToPath(
    new URL(`../../docs-site/dist/${locale}/${ANY}index.html`, import.meta.url),
  );
  let html;
  try {
    html = readFileSync(file, "utf8");
  } catch {
    return [];
  }
  const links = [];
  // Each tag on its own and each attribute by name, so the order a renderer
  // writes them in is not part of the claim.
  for (const [tag] of html.matchAll(/<a\b[^>]*>/g)) {
    const classes = (/\sclass="([^"]*)"/.exec(tag)?.[1] ?? "").split(/\s+/);
    const href = /\shref="([^"]*)"/.exec(tag)?.[1];
    if (!classes.includes("nd-nav-link") || href === undefined) continue;
    links.push([href.replace(/^(\.\.\/)+/, "").replace(/\/$/, "") || href, href]);
  }
  return links;
}

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

// --- Whole-site walks ---------------------------------------------------------

/**
 * A question asked of every page, declared as one test per page.
 *
 * A WALK WAS ONE TEST WITH A LOOP IN IT, and the budget is per test. Every
 * page — or every page three times, one round per locale — shared thirty
 * seconds, so every page added took margin from the rest, and every process
 * the machine was running took more. The walk then failed as a timeout on
 * whichever page the clock ran out on, which reads as a defect in that page and
 * is not one. One test per page gives every page the whole budget, and gives
 * the workers the walk to share.
 *
 * - `name` is the walk's own title. `title` gives each combination of `axes`
 *   its heading, which by default is `name`: the tests are grouped under the
 *   title the walk had as one test, so a grep for it still finds all of them.
 * - `axes` are crossed with every page: `{ locale: LOCALES }` is one test per
 *   page per locale. `body` receives each axis by its name, beside `page`,
 *   `info` (the test's info), and `name` and `path` of the page it is on.
 * - `pages` is what the walk visits, as `[name, path]` pairs, or a function of
 *   one combination of axes returning them. Left out, it is every page in
 *   `PAGE`. It is the walk's CLAIM as well as its list: what the guard below
 *   holds the generated tests to. It may add pages to `PAGE` and never takes
 *   one away: a path may be written relative (`../icons/`, as a sidebar
 *   writes it) and is read as the page it names.
 * - `tally`, when a claim is about the site rather than about any one page —
 *   a floor, mostly, and `NOCTUA.md §19.5` is why a walk keeps one: the count
 *   of things examined is itself an assertion. It receives every page's
 *   `body` result, in page order, and asserts. A floor needs every page's
 *   number, so a combination with a tally runs its pages one after another on
 *   one worker and asserts after the last — in order, never in one test: each
 *   page still has the whole budget to itself.
 *
 * AND THE WALK IS HELD TO ITS CLAIM. A page dropped from a loop is a line
 * somebody can read; a page dropped from a generated set is a test that was
 * never made, which nobody sees at all. So every walk carries one more test,
 * which counts the tests that were actually registered, as they were
 * registered, against the claim's size times the product of the axes — worked
 * out again from the declarations rather than from the loop that did the
 * registering — and then holds every combination to every page it names, by
 * name, and to every page of `PAGE`, by path.
 *
 * A spec gets its `walk` from `walker`, once, by handing it a function of its
 * own that declares a test:
 *
 *     const walk = walker((title, fn) => test(title, fn));
 *
 * BECAUSE A TEST IS LOCATED WHERE `test` WAS CALLED. Playwright takes the
 * frame that called it, and called from here every walk in the suite reported
 * itself as a line of this file — in the list, in a failure's heading and in
 * the annotation CI writes. Called through a line of the spec, it is the spec
 * that is named.
 */
export function walker(declare) {
  return (name, options, body) => walk(declare, name, options, body);
}

function walk(declare, name, options, body) {
  const { axes = {}, title = () => name, pages = null, tally = null } = options;
  const claim = (combo) =>
    typeof pages === "function" ? pages(combo) : (pages ?? Object.entries(PAGE));
  const combos = Object.entries(axes).reduce(
    (all, [axis, values]) =>
      all.flatMap((combo) => values.map((value) => ({ ...combo, [axis]: value }))),
    [{}],
  );

  // What was declared, by heading, as `[name, path]`. Written straight after
  // the call that declares a test and nowhere else, so a page that never
  // reached that call is a page missing here.
  const made = new Map();
  for (const combo of combos) {
    const heading = title(combo);
    const declared = [];
    made.set(heading, declared);
    test.describe(heading, () => {
      if (tally) test.describe.configure({ mode: "serial" });
      const results = [];
      for (const [page_, path] of claim(combo)) {
        declare(page_, async ({ page }, info) => {
          results.push(await body({ ...combo, page, info, name: page_, path }));
        });
        declared.push([page_, path]);
      }
      if (tally) {
        declare("across every page", async () => {
          expect(results.length, "a page of the walk did not report").toBe(declared.length);
          await tally(results, combo);
        });
      }
    });
  }

  declare(`the walk "${name}" leaves no page out`, async () => {
    // The combinations, counted from the axes as declared.
    const product = Object.values(axes).reduce((count, values) => count * values.length, 1);
    expect(made.size, "a combination of the walk's axes was never generated").toBe(product);

    let claimed = 0;
    for (const combo of combos) {
      const heading = title(combo);
      const want = claim(combo).map(([page_]) => page_);
      const got = made.get(heading) ?? [];
      expect(want.length, `${heading} has no page to visit`).toBeGreaterThan(0);
      expect(
        got.map(([page_]) => page_),
        `${heading} did not generate one test per page it names`,
      ).toEqual(want);
      // AND WHATEVER ELSE A WALK NAMES, IT NAMES THE SITE. Every walk here is a
      // walk over every page of `PAGE` and sometimes more — the built
      // directory, the sidebar, a page `PAGE` leaves out on purpose — so a
      // walk handed a list with a page taken out of it is a walk that left
      // that page out, whatever its own list says.
      const walked = new Set(got.map(([, path]) => String(path).replace(/^(\.\.\/)+/, "")));
      expect(
        Object.values(PAGE).filter((path) => !walked.has(path)),
        `${heading} leaves out pages the site has`,
      ).toEqual([]);
      claimed += want.length;
    }

    // And the number the claim comes to: its size times the axes.
    const generated = [...made.values()].reduce((count, declared) => count + declared.length, 0);
    expect(generated, "the tests generated are not one per page per combination").toBe(claimed);
    if (typeof pages !== "function") {
      expect(generated, "the tests generated are not the page list times the axes").toBe(
        product * (pages ?? Object.keys(PAGE)).length,
      );
    }
  });
}
