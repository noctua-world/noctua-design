// The page shape, and the defects a picture shows that no other test asks for.
//
// WHY THIS FILE EXISTS. The developer's report was two sentences: the
// components carry visual errors, and the pages do not agree with each other.
// Neither was catchable by anything in this suite — every spec here asks about
// ONE component or ONE page, and "these fifty-six pages have one shape" and
// "nothing on any of them is clipped, overlapping or invisible" are claims
// about all of them at once.
//
// The apparatus half is measured rather than judged. Before this change, at
// 1120px: `/button/` spent 1,416px on twenty-two permanent control bars against
// 1,543px of actual example, and `/inline-text/` spent 2,961px against 2,105px.
// A documentation page whose furniture is taller than its content is a defect
// with a number, so it gets a test with a number.

import { expect, test } from "@playwright/test";

import { PAGE, showParts, url } from "./pages.js";

/** The widths every page is measured at: the floor, a tablet, a laptop. */
const WIDTHS = [320, 768, 1120];

/**
 * Elements allowed to reach past the page's content box.
 *
 * A scroller is SUPPOSED to hold something wider than itself — that is what it
 * is. The test is not "nothing is ever wider", it is "nothing is wider without
 * a scroller to hold it", which is the case a reader sees as a cut-off.
 */
const SCROLLS = /^(auto|scroll|hidden|clip)$/;

/** Everything a reader can press, in the vocabulary this site actually writes. */
const CONTROLS =
  'button, a[href], input, select, textarea, summary, [role="button"], [role="menuitem"], [role="tab"], [role="option"]';

/**
 * The displays that stack as boxes, which is where overlap is a question.
 *
 * An inline element that wraps runs from where it starts on one line to where
 * it ends on the next. Its one rectangle covers the whole column and every
 * word beside it, and that rectangle is the union of its lines rather than its
 * ink. Read as a box it reported 1,065 different pairs across the site. Not
 * one of them was two things drawn on top of each other.
 */
const BLOCKS =
  /^(block|flow-root|flex|grid|list-item|table|table-row|table-row-group|table-header-group|table-footer-group|table-cell|table-caption)$/;

test.describe("one page shape", () => {
  for (const [name, path] of Object.entries(PAGE)) {
    test(`${name} is built from the same parts as every other page`, async ({ page }) => {
      await page.goto(url(path));
      const shape = await page.evaluate(() => {
        const doc = document.querySelector(".nd-doc");
        const head = document.querySelector(".nd-doc-head");
        return {
          doc: !!doc,
          head: !!head,
          // The head is the FIRST thing in the article, on every page. A page
          // that opens on something else is a page a reader has to re-learn.
          headFirst: doc?.firstElementChild === head,
          // THE PAGE'S OWN TITLE, of which there is exactly one.
          //
          // Not "one `h1` in the document": `Fieldset { heading: true }` makes
          // its legend the page's heading, for the shape where a page asks
          // one question — so `/fieldset/` demonstrates a component whose
          // whole purpose is to be an `h1`, and a documentation page that
          // renders it honestly has two. The claim worth holding is that the
          // page has one title of its own and that it is where it belongs.
          h1: document.querySelectorAll("h1.nd-title").length,
          titleInHead: !!head?.querySelector("h1.nd-title"),
          leadInHead: !!head?.querySelector("p.nd-lead"),
          // Sections carry the scroll margin that clears the sticky header.
          sections: document.querySelectorAll(".nd-doc-section").length,
          // A page of several sections says so at the top. A page of one has
          // nothing to offer an index that the page itself does not.
          index: !!document.querySelector(".nd-doc-index"),
        };
      });
      expect(shape.doc, "the page is not inside an article.nd-doc").toBe(true);
      expect(shape.head, "the page has no head").toBe(true);
      expect(shape.headFirst, "something is above the page's own title").toBe(true);
      expect(shape.h1, "a page has exactly one h1").toBe(1);
      expect(shape.titleInHead, "the title is not in the head").toBe(true);
      expect(shape.leadInHead, "the page does not say what it is in one line").toBe(true);
      expect(shape.sections, "the page has no sections").toBeGreaterThan(0);
      if (shape.sections > 1) {
        expect(shape.index, "a page of several sections has no index").toBe(true);
      }
    });
  }

  test("every jump in every index lands on something that exists", async ({ page }) => {
    // A jump index on fifty-six pages is fifty-six chances to point at nothing,
    // and a link that scrolls nowhere reads as a page that is broken.
    const broken = [];
    for (const [name, path] of Object.entries(PAGE)) {
      await page.goto(url(path));
      const dead = await page.evaluate(() =>
        [...document.querySelectorAll(".nd-doc-index-list a")]
          .map((link) => link.getAttribute("href"))
          .filter((href) => href?.startsWith("#") && !document.getElementById(href.slice(1))),
      );
      for (const href of dead) broken.push(`${name}: ${href}`);
    }
    expect(broken, "a jump index points at nothing").toEqual([]);
  });

  test("a folded section still has its heading in the outline", async ({ page }) => {
    // THE WHOLE RISK OF FOLDING. Closed, a section's title is the only thing
    // left of it — and a `<span>` for a title is a section that has vanished
    // from the outline a screen reader navigates by. `Disclosure { heading }`
    // is what answers that, and this is the claim it exists to make.
    await page.goto(url(PAGE.component));
    const folds = await page.evaluate(() =>
      [...document.querySelectorAll(".nd-doc-fold")].map((fold) => ({
        open: fold.hasAttribute("open"),
        heading: !!fold.querySelector(":scope > summary > :is(h2, h3, h4)"),
      })),
    );
    expect(folds.length, "the reference is not folded at all").toBeGreaterThan(0);
    expect(
      folds.filter((fold) => !fold.heading),
      "a folded section has no heading",
    ).toEqual([]);
    expect(
      folds.every((fold) => !fold.open),
      "the reference is folded but shipped open",
    ).toBe(true);
  });
});

/**
 * Everything this test asks of one page, in whatever state the page is in.
 *
 * A function of its own rather than a body written inline, because it runs
 * twice on every page. Once on what ships, then again with every fold opened.
 * It answers with findings as strings, and the caller says which of the two
 * states each finding came from.
 */
const SWEEP = ([SCROLLS_SOURCE, CONTROLS_SELECTOR, BLOCKS_SOURCE]) => {
  const scrolls = new RegExp(SCROLLS_SOURCE);
  const blocks = new RegExp(BLOCKS_SOURCE);
  const out = [];
  const where = (el) =>
    `${el.tagName.toLowerCase()}.${String(el.className).split(" ").filter(Boolean).slice(0, 2).join(".")}`;

  const main = document.querySelector(".nd-main");
  if (!main) return ["no .nd-main"];
  const edge = main.getBoundingClientRect().right;

  // ONE STYLE RESOLUTION PER ELEMENT, KEPT. The five checks below ask about
  // the same elements, and resolving the style afresh in each is five walks of
  // the page. On `/lineage/` that is 16,593 elements a walk, and twice over
  // for the two fold states it put this test past its own timeout.
  const inside = [...main.querySelectorAll("*")];
  const styles = new Map(inside.map((el) => [el, getComputedStyle(el)]));
  const styleOf = (el) => styles.get(el) ?? getComputedStyle(el);

  for (const el of inside) {
    const style = styleOf(el);
    if (style.display === "none" || style.visibility === "hidden") continue;
    // TEXT FOR A SCREEN READER IS SUPPOSED TO BE CLIPPED — that is the whole
    // recipe, and `VisuallyHidden` is a documented component of this system.
    // It is also, by a wide margin, the commonest element on every page:
    // unfiltered, one page reported seven hundred findings and none of them
    // was a defect.
    if (style.clipPath !== "none") continue;
    if (el.closest(".nd-visually-hidden, [aria-hidden='true']")) continue;
    const box = el.getBoundingClientRect();
    if (box.width < 1 || box.height < 1) continue;

    // 1. TEXT CUT OFF BY A BOX THAT DOES NOT SCROLL. `Truncate` and the code
    //    block clip on purpose and say so with a line clamp; everything else
    //    that clips is losing a word.
    if (
      scrolls.test(style.overflowX) &&
      style.overflowX !== "auto" &&
      style.overflowX !== "scroll" &&
      el.scrollWidth > el.clientWidth + 1 &&
      style.webkitLineClamp === "none" &&
      style.textOverflow !== "ellipsis"
    ) {
      out.push(`clipped: ${where(el)} (${el.scrollWidth} in ${el.clientWidth})`);
    }

    // 2. PAST THE PAGE'S OWN EDGE, with nothing holding it. The carousel and
    //    the table are wider than the page and are supposed to be — they sit
    //    in a scroller. Nothing else may.
    if (box.right > edge + 1) {
      let held = false;
      for (let up = el.parentElement; up && up !== main; up = up.parentElement) {
        const s = styleOf(up);
        if (scrolls.test(s.overflowX) || scrolls.test(s.overflow)) {
          held = true;
          break;
        }
      }
      if (!held) out.push(`overflows: ${where(el)} to ${Math.round(box.right - edge)}px past`);
    }
  }

  // 3. A CONTROL TOO SMALL TO HIT. 24px is the AA floor and this system's own
  //    choice controls are 44.
  for (const el of main.querySelectorAll(CONTROLS_SELECTOR)) {
    const style = styleOf(el);
    if (style.display === "none" || style.visibility === "hidden") continue;
    // `VisuallyHidden`'s recipe again, and here it is a control on purpose: a
    // native file input is clipped rather than removed so it keeps its place
    // in the tab order, and the thing a reader aims at is the drop zone
    // beside it.
    if (style.clipPath !== "none") continue;
    // A TRANSPARENT CONTROL OVER A DRAWN ONE IS THIS SYSTEM'S OWN TECHNIQUE,
    // and it is the reason the choices survive a screen magnifier: the
    // `<input>` is `opacity: 0` and the box beside it is the picture. Its
    // size is the picture's business, and where it is genuinely 1x1 — a file
    // input a label opens — the target is the label.
    if (style.opacity === "0") continue;
    // A specimen is a PICTURE of a control, not a control. Nobody is meant to
    // press the `link` variant in a grid of variants.
    if (el.closest(".nd-variant-stage")) continue;
    // THE SAME RECIPE, APPLIED WITHOUT THE CLASS. A stacked table keeps its
    // heading row off-screen rather than out of the layout, so every cell
    // stays associated with the column that named it — and the controls
    // inside it inherit that. Their size is the recipe's, not a decision
    // anybody made about a target.
    const head = el.closest("thead");
    if (head && styleOf(head).clipPath !== "none") continue;
    const box = el.getBoundingClientRect();
    if (box.width === 0 && box.height === 0) continue;
    // An inline link inside a sentence is text, not a target: its height is
    // the line's and the line is the reader's own setting.
    if (style.display.startsWith("inline") && el.tagName === "A") continue;
    if (box.width < 24 || box.height < 24) {
      out.push(`too small: ${where(el)} (${Math.round(box.width)}x${Math.round(box.height)})`);
    }
  }

  // 4. A STAGE OR A SPECIMEN WITH NOTHING IN IT. An example that renders
  //    nothing looks exactly like an example nobody wrote.
  //
  //    A CLOSED FOLD IS NOT AN EMPTY STAGE. The variant grid ships folded,
  //    and a shut `<details>` hides what is in it rather than failing to draw
  //    it. An engine that hides it with `display: none` measures every one of
  //    those stages at 0x0, which would report every component page at every
  //    width. What is folded is asserted on its own, above: it has a heading
  //    and it is shut. What is inside it is swept by the second pass, which
  //    is the only pass that ever sees 361 of the site's 702 stages.
  for (const el of main.querySelectorAll(".nd-example-stage, .nd-variant-stage")) {
    if (el.closest("details:not([open])")) continue;
    const box = el.getBoundingClientRect();
    if (box.width < 4 || box.height < 4) out.push(`empty: ${where(el)}`);
  }

  // 5. TWO THINGS DRAWN ON TOP OF EACH OTHER. This is the word in the test's
  //    own name, and nothing in its body used to look for it.
  //
  //    SIBLINGS, AND ONLY SIBLINGS. A child sits inside its parent by
  //    definition, so a check that compares any two elements reports every
  //    element on the page against its own ancestors.
  //
  //    THE OVERLAPS THIS SYSTEM MEANS ARE DECLARED, and both ways of
  //    declaring one are read here rather than named. `AvatarGroup` lays each
  //    face over the one before it with a negative margin, which is the one
  //    component that overlaps on purpose. The two-thumb slider puts both of
  //    its rails in a single grid cell, which is how a range keeps a keyboard
  //    at each end.
  const tilted = new Set();
  for (const el of inside) {
    if (styleOf(el).transform === "none") continue;
    tilted.add(el);
    for (const kid of el.querySelectorAll("*")) tilted.add(kid);
  }
  for (const parent of [main, ...inside]) {
    // A TILTED ANCESTOR IS NOT A LAYOUT. `/lineage/` draws its scene under a
    // perspective, where a rectangle is the bounding box of a projected quad.
    // A 28px portrait measures 32 there and reaches 5px into the name below
    // it, which the layout leaves 4px clear. Nothing is drawn over anything,
    // so the finding is the measurement rather than the page.
    if (tilted.has(parent)) continue;
    const laid = /grid$/.test(styleOf(parent).display);
    const kids = [];
    for (const el of parent.children) {
      const style = styleOf(el);
      if (style.display === "none" || style.visibility === "hidden") continue;
      // Invisible is not drawn, and only what is drawn can be drawn over.
      if (style.opacity === "0" || style.clipPath !== "none") continue;
      if (el.closest(".nd-visually-hidden, [aria-hidden='true']")) continue;
      // OUT OF THE FLOW IS A PLACEMENT, and a placement is a decision
      // somebody made. The avatar's picture covers its own initials from
      // `inset: 0`, and the table's resize handle sits on the heading it
      // resizes.
      if (/^(absolute|fixed|sticky)$/.test(style.position)) continue;
      if (!blocks.test(style.display)) continue;
      const box = el.getBoundingClientRect();
      if (box.width < 1 || box.height < 1) continue;
      kids.push({
        el,
        box,
        lapped: [style.marginTop, style.marginBottom, style.marginLeft, style.marginRight].some(
          (side) => side.startsWith("-"),
        ),
        cell: laid ? style.gridArea : "auto",
      });
    }
    for (let i = 0; i < kids.length; i += 1) {
      for (let j = i + 1; j < kids.length; j += 1) {
        const first = kids[i];
        const second = kids[j];
        if (first.lapped || second.lapped) continue;
        if (!first.cell.includes("auto") && first.cell === second.cell) continue;
        const across =
          Math.min(first.box.right, second.box.right) - Math.max(first.box.left, second.box.left);
        const down =
          Math.min(first.box.bottom, second.box.bottom) - Math.max(first.box.top, second.box.top);
        // A pixel of tolerance, because two boxes that touch can each round
        // to the edge they share.
        if (across > 1 && down > 1) {
          out.push(
            `overlaps: ${where(first.el)} over ${where(second.el)} by ${Math.round(across)}x${Math.round(down)}px`,
          );
        }
      }
    }
  }

  return out;
};

test.describe("nothing is drawn wrong", () => {
  for (const width of WIDTHS) {
    test(`nothing is clipped, overlapping or invisible at ${width}px`, async ({ page }) => {
      await page.setViewportSize({ width, height: 900 });
      const found = [];
      for (const [name, path] of Object.entries(PAGE)) {
        await page.goto(url(path));
        // ONE PAGE IS TWO PAGES, and this sweep used to see only the first.
        // Every variant grid and every contract table ships behind a fold,
        // and a reader opens it. Across the site that is 313 folds holding
        // 361 of the 702 stages. The empty-stage check above steps over every
        // one of them while the fold is shut, so it saw half the site.
        //
        // The other four checks do reach inside a shut fold in this browser.
        // Chrome lays out a `content-visibility: hidden` subtree as soon as
        // anything measures it, so the 83,866 elements the site keeps folded
        // answer with a real box. That is the engine's choice rather than
        // this suite's, and it is not what the coverage should rest on.
        //
        // The state rides along in the finding. A report of "clipped at
        // 320px" is not actionable until you know whether a reader had to
        // open something first.
        for (const state of ["as shipped", "folds opened"]) {
          if (state === "folds opened") await showParts(page);
          const bad = await page.evaluate(SWEEP, [SCROLLS.source, CONTROLS, BLOCKS.source]);
          for (const one of bad) found.push(`${name} [${state}] — ${one}`);
        }
      }
      expect(found, `drawn wrong at ${width}px`).toEqual([]);
    });
  }
});

test("an example spends one strip on its own apparatus, and it is narrow", async ({ page }) => {
  // THE NUMBER THE DEVELOPER WAS REACTING TO. Every example carried a
  // permanent six-control bar across the top: measured at 1120px, `/button/`
  // spent 1,416px on twenty-two of them against 1,543px of actual example.
  // Folding it behind a gear traded that for a control you had to press to
  // find the controls, which was worse.
  //
  // The strip is a rail down the frame's trailing edge now, so it spends WIDTH
  // — of which every frame has 413 to 815px going unused — rather than height,
  // which every frame is short of. The budget moves to the other axis with it.
  const heavy = [];
  for (const [name, path] of Object.entries(PAGE)) {
    await page.goto(url(path));
    const frames = await page.evaluate(() =>
      [...document.querySelectorAll(".nd-frame")].map((frame) => {
        const rail = frame.querySelector(":scope > .nd-frame-bar");
        return {
          strips: frame.querySelectorAll(":scope > .nd-frame-bar").length,
          wide: rail ? Math.round(rail.getBoundingClientRect().width) : 0,
          // Every control in it is a real target at the 24-pixel floor. The rail
      // is the one place on
          // this site where a row of small controls sits beside content, so it
          // is the one most likely to shrink them.
          small: rail
            ? [...rail.querySelectorAll(":scope > button, :scope > details > summary")].filter(
                (el) => {
                  const box = el.getBoundingClientRect();
                  return box.width < 24 || box.height < 24;
                },
              ).length
            : 0,
        };
      }),
    );
    for (const frame of frames) {
      if (frame.strips !== 1) heavy.push(`${name}: a frame carries ${frame.strips} control strips`);
      // ITS OWN WIDTH, NOT ITS SHARE OF THE FRAME. The rail is a fixed strip,
      // so on a narrow frame it is a bigger fraction without being any wider —
      // and `/example-frame/` nests a frame inside a frame, where the share
      // reads 28% and the rail is the same 112px it is everywhere else. What
      // "compact" means is the absolute width.
      if (frame.wide > 128) heavy.push(`${name}: the rail is ${frame.wide}px wide`);
      if (frame.small) heavy.push(`${name}: ${frame.small} rail control(s) too small`);
    }
  }
  expect(heavy, "a frame spends too much of itself on its own furniture").toEqual([]);
});

// --- Four contracts that named something nothing did -------------------------

test("a list restates the role its own stylesheet takes away", async ({ page }) => {
  // `list.toml` has always said the role is restated explicitly, and nothing
  // emitted `role="list"` — while the stylesheet does both things that drop it:
  // every list is a flex column and `marker: none` removes the marker. So a
  // reader was told "three items" by the markup and nothing by the browser.
  //
  // NOT THE VARIANT GRID, for the reason `segmented.spec.js` gives about
  // radiogroups: a specimen whose contract carries MARKUP is the contract's
  // element with that markup inside, drawn to be looked at rather than a
  // component that rendered itself. Those pictures deliberately carry no ARIA
  // at all — `aria-sort="none"` on eight still `<th>` pictures is what
  // inventing it cost last time — so asking one to restate a role is asking
  // the wrong element. `/list/` is where this shows: seven real lists on it
  // claim the role, and eight pictures of a list do not.
  let seen = 0;
  for (const path of Object.values(PAGE)) {
    await page.goto(url(path));
    const wrong = await page.evaluate(() => {
      const out = { lists: 0, unnamed: 0, items: 0, unnamedItems: 0 };
      const drawn = (el) => el.closest(".nd-variant-stage") !== null;
      for (const list of document.querySelectorAll("ul.nd-list, ol.nd-list")) {
        if (drawn(list)) continue;
        out.lists += 1;
        if (list.getAttribute("role") !== "list") out.unnamed += 1;
      }
      for (const item of document.querySelectorAll("li.nd-list-item")) {
        if (drawn(item)) continue;
        out.items += 1;
        if (item.getAttribute("role") !== "listitem") out.unnamedItems += 1;
      }
      return out;
    });
    seen += wrong.lists;
    expect(wrong.unnamed, `${path} has a list that claims no role`).toBe(0);
    expect(wrong.unnamedItems, `${path} has an item that claims no role`).toBe(0);
  }
  expect(seen, "no list anywhere on the site").toBeGreaterThan(0);
});

test("a specimen is named only where the component owes a name", async ({ page }) => {
  // `Truncate` declared `needs_name` on a role-less `<span>` with no naming
  // prop, so the grid stamped `aria-labelledby` over every specimen — replacing
  // the clipped text in the accessible tree with the caption beside it, which
  // is the one thing that component promises not to do.
  // A literal path rather than a `PAGE` entry: adding one puts the page into
  // five whole-site sweeps, which is a decision about those sweeps and not
  // about this claim.
  await page.goto(url("truncate/"));
  const named = await page.evaluate(
    () => document.querySelectorAll(".nd-variant-stage [aria-labelledby]").length,
  );
  expect(named).toBe(0);
});

test("no variant grid renders a live control nothing has named", async ({ page }) => {
  // `TagPicker` recorded `needs_name: false` and its specimen was a real
  // `<input>`, so its grid shipped three focusable text boxes with no `name`,
  // no `id`, no label and no `aria-label` — the only nameless live control in
  // the system, on the page of a component that needs a name.
  for (const path of Object.values(PAGE)) {
    await page.goto(url(path));
    const nameless = await page.evaluate(() =>
      [...document.querySelectorAll(".nd-variant-stage input, .nd-variant-stage select")]
        // LIVE, WHICH IS THE WORD IN THE TITLE. A control the runtime owns
        // ships `hidden` and is replaced by the one it built: `Select` hides
        // its native `<select>` and moves the label's `for` onto the button.
        // Reading the hidden one reports every enhanced control in every grid
        // as nameless, which is a claim about the enhancement rather than
        // about the grid.
        .filter((el) => !el.hidden && getComputedStyle(el).display !== "none")
        .filter(
          (el) =>
            !el.getAttribute("aria-label") &&
            !el.getAttribute("aria-labelledby") &&
            !(el.id && document.querySelector(`label[for="${CSS.escape(el.id)}"]`)) &&
            !el.closest("label"),
        )
        .map((el) => el.className),
    );
    expect(nameless, path).toEqual([]);
  }
});

test("opening a disclosure turns its chevron and never the caller's icon", async ({ page }) => {
  await page.goto(url("disclosure/"));
  // The rule read `.nd-icon-glyph:last-child`, and in the icon-only form the
  // caller's own icon IS the last glyph — so opening a
  // `Disclosure { icon: "settings" }` turned the gear upside down.
  const turned = await page.evaluate(() => {
    const out = [];
    for (const details of document.querySelectorAll(".nd-disclosure")) {
      details.open = true;
    }
    for (const glyph of document.querySelectorAll(
      ".nd-disclosure[open] > .nd-disclosure-summary .nd-icon-glyph",
    )) {
      const rotated = getComputedStyle(glyph).transform !== "none";
      if (rotated && !glyph.classList.contains("nd-disclosure-marker")) {
        out.push(glyph.getAttribute("data-nd-icon") ?? "?");
      }
    }
    return out;
  });
  expect(turned).toEqual([]);
});
