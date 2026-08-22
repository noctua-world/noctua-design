// THE ONE RULE THE ASSISTANT'S MARK EXISTS UNDER: it never sits on content.
//
// Forty-five components reserve room for it, and every one of them says where
// in its own body. What none of them can say is whether the room is real, and
// that is what this file measures — two different ways, because either alone
// passes on a defect the other catches:
//
// 1. NOTHING ELSE MOVED. The marks are hidden with `visibility`, which does not
//    reflow, so every other box has to keep the geometry it had. A box that
//    moved is a mark whose layout is entangled with its neighbour's: a row that
//    grew a line, a grid track that took width from the one beside it. Cheap,
//    so it runs over every page at both widths.
// 2. AND IT PAINTED NOWHERE BUT ITS OWN RECTANGLES. Every pixel that differs
//    between a shot with the mark and a shot without it has to fall inside a
//    rectangle the mark reported. A pixel outside one is the rule broken,
//    stated as bytes.
//
// THE SECOND IS THE ONE THAT CATCHES AN OVERLAY, and the first cannot: a mark
// given `position: absolute` is out of the flow in BOTH shots, so hiding it
// moves nothing and check 1 passes on it. Measured, by putting that rule in
// `shell.css` and watching only check 2 fail.
//
// THE SECOND ONE IS PER MARK AND CLIPPED. A viewport screenshot misses every
// mark below the fold, and `outside` answers `Infinity` rather than zero when
// the two shots are identical — which is the floor working, and which is how
// the first draft of this file was caught measuring nothing on nine pages. A
// full-page shot would be correct and would put twenty million pixels through
// a loop per page.
//
// `audit.spec.js` CANNOT DO EITHER. Its overlap sweep filters candidates
// through a block-display list on purpose: an inline element's bounding box is
// the union of its lines, and reading one as a rectangle reported a thousand
// pairs there, none of them a defect. An inline wordmark is precisely what that
// filter excludes.

import { expect, test } from "@playwright/test";

import { LOCALES, PAGE, url, showParts } from "./pages.js";
import { outside } from "./pixels.js";

/** Every box under the main column, as integers, marks excluded. */
const geometry = () =>
  [...document.querySelectorAll(".nd-main *")]
    .filter((el) => !el.closest(".nd-ai-label"))
    .map((el) => {
      const r = el.getBoundingClientRect();
      return [Math.round(r.x), Math.round(r.y), Math.round(r.width), Math.round(r.height)];
    });

for (const width of [320, 1120]) {
  for (const locale of LOCALES) {
    test(`hiding the mark moves nothing, in ${locale} at ${width}px`, async ({ page }) => {
      // A WHOLE-SITE SWEEP, DECLARED AS ONE. Fifty pages a run, six runs, and
      // the site's other whole-site sweep does a third of that in one locale.
      // Under eight workers this passes the default ceiling and fails as a
      // timeout, which reads as a defect in the mark rather than in the clock.
      test.slow();
      await page.setViewportSize({ width, height: 900 });
      let swept = 0;

      for (const [name, path] of Object.entries(PAGE)) {
        await page.goto(url(path, locale));
        await showParts(page);
        if (!(await page.locator(".nd-ai-label").count())) continue;
        swept += 1;

        const before = await page.evaluate(geometry);
        await page.addStyleTag({ content: ".nd-ai-label { visibility: hidden !important; }" });
        const after = await page.evaluate(geometry);
        expect(after, `${name}: hiding the mark moved something else`).toEqual(before);
      }

      // A FLOOR, BECAUSE A SWEEP THAT FOUND NOTHING PASSES. Forty-five
      // components host a mark and every one of them shows it in an example,
      // so a run that visited fewer pages than this visited a site where the
      // marks had stopped rendering.
      expect(swept, `no page in ${locale} carries a mark`).toBeGreaterThan(30);
    });
  }
}

for (const locale of LOCALES) {
  test(`the mark paints inside its own box, in ${locale}`, async ({ page }) => {
    // Two screenshots per mark, sixty-odd marks. See above.
    test.slow();
    await page.setViewportSize({ width: 1120, height: 900 });
    let swept = 0;

    for (const [name, path] of Object.entries(PAGE)) {
      await page.goto(url(path, locale));
      await showParts(page);
      const marks = page.locator(".nd-ai-label");
      const count = await marks.count();

      for (let i = 0; i < count; i += 1) {
        const mark = marks.nth(i);
        // A MARK INSIDE SOMETHING SHUT IS NOT A MARK PAINTING ANYWHERE. A
        // dialog's surface, a popover's panel and a toast all ship hidden, and
        // the marks in them have no boxes at all until something opens them.
        if (!(await mark.isVisible())) continue;
        await mark.scrollIntoViewIfNeeded();
        const rects = await mark.evaluate((el) =>
          [...el.getClientRects()].map((r) => [r.left, r.top, r.right, r.bottom]),
        );
        if (!rects.length) continue;

        // A window around it, so the loop reads thousands of pixels rather
        // than millions and still covers everything the mark could reach.
        const clip = await mark.evaluate((el) => {
          const r = el.getBoundingClientRect();
          const pad = 160;
          const x = Math.max(0, r.left - pad);
          const y = Math.max(0, r.top - pad);
          return {
            x,
            y,
            width: Math.min(window.innerWidth - x, r.width + pad * 2),
            height: Math.min(window.innerHeight - y, r.height + pad * 2),
          };
        });
        if (clip.width < 8 || clip.height < 8) continue;

        const shown = await page.screenshot({ clip });
        await mark.evaluate((el) => {
          el.style.visibility = "hidden";
        });
        const gone = await page.screenshot({ clip });
        await mark.evaluate((el) => {
          el.style.visibility = "";
        });

        // The rectangles are in viewport coordinates and the shots are in the
        // clip's, so the clip's origin comes off them.
        const local = rects.map(([l, t, r, b]) => [
          l - clip.x,
          t - clip.y,
          r - clip.x,
          b - clip.y,
        ]);
        const strays = await outside(page, shown, gone, local);
        expect(strays, `${name}: mark ${i} painted outside its own box`).toBe(0);
        swept += 1;
      }
    }

    expect(swept, `no mark was measured in ${locale}`).toBeGreaterThan(40);
  });
}

test("no mark overlaps the run of text it stands beside", async ({ page }) => {
  // RANGE RECTANGLES, NOT ELEMENT BOXES, for the reason in this file's header:
  // an inline element's box is the union of its lines. A `Range` over a text
  // node reports the line boxes, which are the ink.
  await page.setViewportSize({ width: 1120, height: 900 });
  let checked = 0;
  for (const locale of LOCALES) {
    // `component` is `/heading/`, the page a spec about what a component page
    // IS uses. Between them these five carry every inline placement there is.
    for (const path of [PAGE.aiLabel, PAGE.component, PAGE.list, PAGE.tag, PAGE.label]) {
      await page.goto(url(path, locale));
      await showParts(page);
      const hits = await page.evaluate(() => {
        const overlaps = (a, b) =>
          a.left < b.right - 1 &&
          b.left < a.right - 1 &&
          a.top < b.bottom - 1 &&
          b.top < a.bottom - 1;
        const found = [];
        let seen = 0;
        for (const mark of document.querySelectorAll(".nd-ai-label")) {
          const mine = [...mark.getClientRects()];
          if (!mine.length) continue;
          seen += 1;
          for (const node of mark.parentElement.childNodes) {
            if (node.nodeType !== Node.TEXT_NODE || !node.textContent.trim()) continue;
            const range = document.createRange();
            range.selectNode(node);
            for (const theirs of range.getClientRects()) {
              for (const ours of mine) {
                if (overlaps(ours, theirs)) {
                  found.push(mark.parentElement.className || mark.parentElement.tagName);
                }
              }
            }
          }
        }
        return { found, seen };
      });
      expect(hits.found, `${path} in ${locale}`).toEqual([]);
      checked += hits.seen;
    }
  }
  expect(checked, "no inline mark was measured").toBeGreaterThan(10);
});
