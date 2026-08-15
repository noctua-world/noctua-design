// The rule between two blocks, and the word on it.
//
// WHAT THIS FILE IS ACTUALLY ASKING. A divider is one border and a handful of
// ARIA, so every claim it makes is cheap to break and invisible when broken: a
// labelled rule whose word never reaches a screen reader looks identical to one
// whose word does, and a decorative rule left in the accessibility tree
// announces *separator* in the middle of a sentence with nothing to say.
//
// The one thing NOT asserted here is the argument in `guidance` — when a rule
// is right and when more space is right. That is a judgement, and a judgement
// is a review, not a test.

import { expect, test } from "@playwright/test";

import { PAGE, url, showParts } from "./pages.js";

/** Every divider on the page, once the folded sections are open. */
async function dividers(page) {
  await page.goto(url(PAGE.divider));
  await showParts(page);
  return page.locator(".nd-divider");
}

test("a plain divider is an <hr> that says it is a separator", async ({ page }) => {
  const all = await dividers(page);
  const plain = all.filter({ hasNot: page.locator(".nd-divider-text") }).first();
  expect(await plain.evaluate((el) => el.tagName)).toBe("HR");
  expect(await plain.getAttribute("role")).toBe("separator");
  // Always written, both ways round. A separator with no stated orientation is
  // read as horizontal, and half of these are not.
  expect(await plain.getAttribute("aria-orientation")).toMatch(/^(horizontal|vertical)$/);
});

test("a labelled divider is a <div>, because an <hr> holds no content", async ({ page }) => {
  const all = await dividers(page);
  const labelled = page.locator(".nd-divider[data-nd-labelled]").first();
  await expect(labelled).toBeVisible();
  expect(await labelled.evaluate((el) => el.tagName)).toBe("DIV");
  expect(await labelled.getAttribute("role")).toBe("separator");

  // THE CLAIM THAT MATTERS. ARIA gives `separator` children-presentational, so
  // the word inside is a drawing and nothing reads it. The same string has to
  // be the accessible name or the boundary announces itself as nothing.
  const shown = (await labelled.locator(".nd-divider-text").textContent())?.trim();
  expect(shown).toBeTruthy();
  expect(await labelled.getAttribute("aria-label")).toBe(shown);

  // And every divider on the page is one shape or the other, never both.
  const confused = await all.evaluateAll((els) =>
    els.filter((el) => (el.tagName === "HR") === el.hasAttribute("data-nd-labelled")).length,
  );
  expect(confused, "an <hr> with a label, or a <div> without one").toBe(0);
});

test("a decorative divider leaves the accessibility tree entirely", async ({ page }) => {
  await dividers(page);
  const hidden = page.locator('.nd-divider[aria-hidden="true"]').first();
  await expect(hidden).toBeAttached();
  // The three go together. A role on something nothing can reach is a claim
  // about an element that is not there.
  expect(await hidden.getAttribute("role")).toBeNull();
  expect(await hidden.getAttribute("aria-orientation")).toBeNull();
  expect(await hidden.getAttribute("aria-label")).toBeNull();
});

test("the three weights are three different thicknesses", async ({ page }) => {
  await dividers(page);
  const drawn = await page.evaluate(() => {
    const seen = {};
    for (const weight of ["hairline", "strong", "rule"]) {
      // The plain form: a labelled divider draws its two half-rules on the
      // spans inside it, so the root's own border is deliberately zero.
      const el = document.querySelector(
        `.nd-divider:not([data-nd-labelled])[data-nd-weight="${weight}"]`,
      );
      if (!el) continue;
      const style = getComputedStyle(el);
      // A horizontal rule is a block-start border and a vertical one is an
      // inline-start border, so the reading is whichever is not zero.
      seen[weight] = [style.borderTopWidth, style.borderLeftWidth, style.borderRightWidth]
        .map(parseFloat)
        .find((width) => width > 0);
    }
    return seen;
  });
  expect(Object.keys(drawn).sort()).toEqual(["hairline", "rule", "strong"]);
  expect(drawn.hairline).toBeLessThan(drawn.strong);
  expect(drawn.strong).toBeLessThan(drawn.rule);
});

test("a vertical divider takes its height from the row beside it", async ({ page }) => {
  await dividers(page);
  const upright = page.locator('.nd-divider[data-nd-orientation="vertical"]').first();
  await expect(upright).toBeVisible();
  const box = await upright.boundingBox();
  const beside = await upright.evaluate((el) => {
    const row = el.parentElement;
    return Math.max(
      ...[...row.children].filter((one) => one !== el).map((one) => one.getBoundingClientRect().height),
    );
  });
  // Not a number written into the stylesheet: it is as tall as what it stands
  // between, which is the only height that is still right tomorrow.
  expect(box.height).toBeGreaterThanOrEqual(beside - 1);
  expect(box.width).toBeLessThan(8);
});

test("it draws with a border and never with a colour of its own", async ({ page }) => {
  await dividers(page);
  // BOTH HALVES OF THE TITLE, because the sweep before this one asserted
  // neither. It read the shadow and the background image. A rule with no
  // border left to draw it passed, and so did one painted with a colour.
  //
  // A divider is a border and a gap. Anything else would make it a surface,
  // and `NOCTUA.md §9.7` has none.
  const drawn = await page.evaluate(() =>
    [...document.querySelectorAll(".nd-divider")].flatMap((el) => {
      // Whatever carries the border. A labelled divider draws two half-rules
      // on the spans beside the word, so its root deliberately has none.
      const parts = el.hasAttribute("data-nd-labelled")
        ? [...el.querySelectorAll(".nd-divider-line")]
        : [el];
      return parts.map((part) => {
        const style = getComputedStyle(part);
        return {
          name: `${el.dataset.ndOrientation} ${el.dataset.ndWeight}`,
          // The drawing edge is block-start when the rule lies across the
          // page, and inline-start when it stands beside a row. The reading is
          // whichever edge is not zero.
          edge: Math.max(
            ...[
              style.borderTopWidth,
              style.borderBottomWidth,
              style.borderLeftWidth,
              style.borderRightWidth,
            ].map(parseFloat),
          ),
          paint: style.backgroundColor, // allow-literal: read to ask whether anything is painted at all, never to compare one colour with another
          image: style.backgroundImage,
          shadow: style.boxShadow,
        };
      });
    }),
  );
  // A floor, because an empty page passes every loop below.
  expect(drawn.length, "no divider was measured at all").toBeGreaterThan(5);
  for (const one of drawn) {
    expect(one.edge, `${one.name} draws no border`).toBeGreaterThan(0);
    expect(one.paint, `${one.name} paints a colour of its own`).toBe("rgba(0, 0, 0, 0)"); // allow-literal: the CSS keyword for "no background", not a colour this system could take from a token
    expect(one.image, `${one.name} paints a gradient`).toBe("none");
    expect(one.shadow, `${one.name} carries a shadow`).toBe("none");
  }
});
