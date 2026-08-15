// The variant grids, on every component page that has one.
//
// THE DEFECT THIS NAMES, in the developer's words: side-by-side examples that
// should show different results showing exactly the same thing. `lineage.spec.js`
// has asked this of ONE page since the scene shipped, and it was the right
// question asked in the wrong place — the failure it caught is not about
// drawing in three dimensions, it is about what a specimen IS.
//
// The button page is what proved that. Four tones drew four identical grey
// squares because a specimen wrote exactly one attribute and every tone rule in
// this system is compound — `.nd-button[data-nd-variant="solid"][data-nd-tone="danger"]`
// — so a specimen carrying `data-nd-tone` alone matched no rule and fell back to
// the browser's own button. The same shape sits on `.nd-select-field`,
// `.nd-table` and `.nd-lineage-piece`. One page's test could not have found any
// of it.
//
// A grid that draws one picture under two names is worse than no grid: it says
// the prop does nothing, and a reader is right to believe it.

import { expect, test } from "@playwright/test";

import { PAGE, url, showParts } from "./pages.js";

/**
 * Everything about how one specimen looks, as a string.
 *
 * The stage's own box is part of the picture — `size` and `width` vary nothing
 * else — so the root is read along with its descendants.
 *
 * IT IS LONG ON PURPOSE. Every property missing from this list is a pair of
 * pictures the test calls identical when a reader can tell them apart: the
 * first draft left out `text-align` and reported all four values of
 * `TableCell.align` as one picture, which they visibly are not. A false alarm
 * here costs a real prop its grid, so the list errs toward saying *different*.
 */
const FINGERPRINT = `(root) => {
  // THE PLATFORM'S OWN PARTS COUNT. \`<meter>\`, \`<progress>\` and a range
  // \`<input>\` are painted almost entirely into pseudo-elements with a
  // different name per engine, and \`getComputedStyle(el)\` cannot see any of
  // them — so a slider whose four tones really are four colours reported as one
  // picture four times, and the test was blind exactly where the drawing is.
  const PARTS = [
    "::-webkit-slider-thumb",
    "::-webkit-slider-runnable-track",
    "::-webkit-meter-bar",
    "::-webkit-meter-optimum-value",
    "::-webkit-progress-bar",
    "::-webkit-progress-value",
    "::before",
    "::after",
  ];
  return [root, ...root.querySelectorAll("*")]
    .map((el) => {
      const style = getComputedStyle(el);
      const box = el.getBoundingClientRect();
      const parts = PARTS.map((part) => {
        const s = getComputedStyle(el, part);
        return [part, s.backgroundColor, s.backgroundImage, s.borderColor, s.blockSize, s.inlineSize, s.content].join("~");
      }).join("/");
      return [
        el.className,
        el.tagName,
        style.transform,
        style.display,
        style.justifyContent,
        style.alignItems,
        style.flexDirection,
        style.borderColor,
        style.borderRadius,
        style.borderStyle,
        style.borderWidth,
        style.backgroundColor,
        style.backgroundImage,
        style.color,
        style.opacity,
        style.boxShadow,
        style.textAlign,
        style.textDecorationLine,
        style.textTransform,
        style.letterSpacing,
        style.whiteSpace,
        style.overflow,
        style.resize,
        style.padding,
        style.gap,
        style.fontSize,
        style.fontFamily,
        style.fontWeight,
        style.fontVariantNumeric,
        parts,
        Math.round(box.width),
        Math.round(box.height),
      ].join("|");
    })
    .join("\\n");
}`;

for (const [name, path] of Object.entries(PAGE)) {
  test(`no two values of one prop draw the same picture on ${name}`, async ({ page }) => {
    await page.goto(url(path));
    await showParts(page);
    const twins = await page.evaluate((source) => {
      // eslint-disable-next-line no-eval
      const fingerprint = eval(source);
      const found = [];
      // GROUPED BY THE PROP, which is what `data-nd-prop` on the stage is for.
      // A page's grids sit in one flow, so two values of DIFFERENT props being
      // alike is not a defect — `size: medium` and `shape: rounded` are both
      // the resting component, and they should be.
      const byProp = new Map();
      for (const item of document.querySelectorAll(".nd-variant-item")) {
        const stage = item.querySelector(".nd-variant-stage");
        const prop = stage?.getAttribute("data-nd-prop");
        if (!prop) continue;
        // The component too: two components on one collection page may both
        // declare `align`, and those are two grids rather than one.
        const key = `${stage.getAttribute("data-nd-slug") ?? ""}.${prop}`;
        if (!byProp.has(key)) byProp.set(key, []);
        byProp.get(key).push({
          value: item.querySelector(".nd-variant-label")?.textContent?.trim() ?? "?",
          print: fingerprint(stage),
        });
      }
      for (const [prop, values] of byProp) {
        const seen = new Map();
        for (const one of values) {
          const first = seen.get(one.print);
          if (first) found.push(`${prop}: ${first} and ${one.value} are one picture`);
          else seen.set(one.print, one.value);
        }
      }
      return found;
    }, FINGERPRINT);

    expect(twins, "a variant grid lies about its own prop").toEqual([]);
  });
}

test("the grids are actually there to be checked", async ({ page }) => {
  // THE SWEEP ABOVE PASSES ON A PAGE WITH NO GRIDS AT ALL, which is how a test
  // over every page comes to prove nothing. This is the floor.
  await page.goto(url(PAGE.button));
  await showParts(page);
  const stages = await page.evaluate(
    () => document.querySelectorAll(".nd-variant-stage[data-nd-prop]").length,
  );
  expect(stages, "the button page rendered no variant grid").toBeGreaterThan(10);
});
