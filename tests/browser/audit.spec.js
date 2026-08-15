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

import { PAGE, url } from "./pages.js";

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

test.describe("nothing is drawn wrong", () => {
  for (const width of WIDTHS) {
    test(`nothing is clipped, overlapping or invisible at ${width}px`, async ({ page }) => {
      await page.setViewportSize({ width, height: 900 });
      const found = [];
      for (const [name, path] of Object.entries(PAGE)) {
        await page.goto(url(path));
        const bad = await page.evaluate(
          ([SCROLLS_SOURCE, CONTROLS_SELECTOR]) => {
            const scrolls = new RegExp(SCROLLS_SOURCE);
            const out = [];
            const where = (el) =>
              `${el.tagName.toLowerCase()}.${String(el.className).split(" ").filter(Boolean).slice(0, 2).join(".")}`;

            const main = document.querySelector(".nd-main");
            if (!main) return ["no .nd-main"];
            const edge = main.getBoundingClientRect().right;

            for (const el of main.querySelectorAll("*")) {
              const style = getComputedStyle(el);
              if (style.display === "none" || style.visibility === "hidden") continue;
              // TEXT FOR A SCREEN READER IS SUPPOSED TO BE CLIPPED — that is
              // the whole recipe, and `VisuallyHidden` is a documented
              // component of this system. It is also, by a wide margin, the
              // commonest element on every page: unfiltered, one page reported
              // seven hundred findings and none of them was a defect.
              if (style.clipPath !== "none") continue;
              if (el.closest(".nd-visually-hidden, [aria-hidden='true']")) continue;
              const box = el.getBoundingClientRect();
              if (box.width < 1 || box.height < 1) continue;

              // 1. TEXT CUT OFF BY A BOX THAT DOES NOT SCROLL. `Truncate` and
              //    the code block clip on purpose and say so with a line
              //    clamp; everything else that clips is losing a word.
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

              // 2. PAST THE PAGE'S OWN EDGE, with nothing holding it. The
              //    carousel and the table are wider than the page and are
              //    supposed to be — they sit in a scroller. Nothing else may.
              if (box.right > edge + 1) {
                let held = false;
                for (let up = el.parentElement; up && up !== main; up = up.parentElement) {
                  const s = getComputedStyle(up);
                  if (scrolls.test(s.overflowX) || scrolls.test(s.overflow)) {
                    held = true;
                    break;
                  }
                }
                if (!held) out.push(`overflows: ${where(el)} to ${Math.round(box.right - edge)}px past`);
              }
            }

            // 3. A CONTROL TOO SMALL TO HIT. 24px is the AA floor and this
            //    system's own choice controls are 44.
            for (const el of main.querySelectorAll(CONTROLS_SELECTOR)) {
              const style = getComputedStyle(el);
              if (style.display === "none" || style.visibility === "hidden") continue;
              // `VisuallyHidden`'s recipe again, and here it is a control on
              // purpose: a native file input is clipped rather than removed so
              // it keeps its place in the tab order, and the thing a reader
              // aims at is the drop zone beside it.
              if (style.clipPath !== "none") continue;
              // A TRANSPARENT CONTROL OVER A DRAWN ONE IS THIS SYSTEM'S OWN
              // TECHNIQUE, and it is the reason the choices survive a screen
              // magnifier: the `<input>` is `opacity: 0` and the box beside it
              // is the picture. Its size is the picture's business, and where
              // it is genuinely 1x1 — a file input a label opens — the target
              // is the label.
              if (style.opacity === "0") continue;
              // A specimen is a PICTURE of a control, not a control. Nobody is
              // meant to press the `link` variant in a grid of variants.
              if (el.closest(".nd-variant-stage")) continue;
              // THE SAME RECIPE, APPLIED WITHOUT THE CLASS. A stacked table
              // keeps its heading row off-screen rather than out of the
              // layout, so every cell stays associated with the column that
              // named it — and the controls inside it inherit that. Their size
              // is the recipe's, not a decision anybody made about a target.
              const head = el.closest("thead");
              if (head && getComputedStyle(head).clipPath !== "none") continue;
              const box = el.getBoundingClientRect();
              if (box.width === 0 && box.height === 0) continue;
              // An inline link inside a sentence is text, not a target: its
              // height is the line's and the line is the reader's own setting.
              if (style.display.startsWith("inline") && el.tagName === "A") continue;
              if (box.width < 24 || box.height < 24) {
                out.push(`too small: ${where(el)} (${Math.round(box.width)}x${Math.round(box.height)})`);
              }
            }

            // 4. A STAGE OR A SPECIMEN WITH NOTHING IN IT. An example that
            //    renders nothing looks exactly like an example nobody wrote.
            //
            //    A CLOSED FOLD IS NOT AN EMPTY STAGE. The variant grid ships
            //    folded, and everything inside a shut `<details>` measures
            //    0x0 — which is the platform hiding it, not a page that
            //    rendered nothing. Reading it as empty would report every
            //    component page at every width. What is folded is asserted on
            //    its own, above: it has a heading and it is shut.
            for (const el of main.querySelectorAll(".nd-example-stage, .nd-variant-stage")) {
              if (el.closest("details:not([open])")) continue;
              const box = el.getBoundingClientRect();
              if (box.width < 4 || box.height < 4) out.push(`empty: ${where(el)}`);
            }

            return out;
          },
          [SCROLLS.source, CONTROLS],
        );
        for (const one of bad) found.push(`${name} — ${one}`);
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
