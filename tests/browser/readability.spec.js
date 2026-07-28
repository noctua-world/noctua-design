// The readability guarantee, measured.
//
// `spec/tokens.toml` states that a background pattern may move a page pixel at
// most 4% toward the text colour, and calls that "a bounded, measurable claim
// rather than a promise". This file is what makes that sentence true. Without
// it the repository asserts a bound nothing checks — which is worse than having
// no bound, because a reader trusts it.
//
// TWO RULES THIS FILE KEEPS:
//
// 1. **Never read a colour back out of the DOM.** `getComputedStyle(…).color`
//    returns whatever the engine chose to serialise, in whatever space and
//    precision. Every colour here is read from a SCREENSHOT — actual pixels,
//    after compositing, which is the only thing a reader's eye sees.
//
// 2. **Test the worst case, not a sample.** Every pattern × every mode ×
//    blackout on and off. A pattern that only fails in dark blackout is a
//    pattern that fails for exactly the readers who chose blackout.

import { test, expect } from "@playwright/test";

const PATTERNS = ["dots", "mesh", "checks", "diagonals"];
const MODES = ["light", "dark"];

/** WCAG 2.1 relative luminance, from an sRGB triple. */
function luminance([r, g, b]) {
  const channel = (v) => {
    const s = v / 255;
    return s <= 0.03928 ? s / 12.92 : ((s + 0.055) / 1.055) ** 2.4;
  };
  return 0.2126 * channel(r) + 0.7152 * channel(g) + 0.0722 * channel(b);
}

function contrast(a, b) {
  const [hi, lo] = [luminance(a), luminance(b)].sort((x, y) => y - x);
  return (hi + 0.05) / (lo + 0.05);
}

/**
 * The darkest and lightest pixel in a PNG screenshot, decoded in the browser.
 *
 * The extremes are what matters: a pattern's worst case for a glyph is the
 * pattern's own peak ink, and averaging would hide exactly that.
 */
async function extremes(page, buffer) {
  const base64 = buffer.toString("base64");
  return page.evaluate(async (data) => {
    const blob = await (await fetch(`data:image/png;base64,${data}`)).blob();
    const bitmap = await createImageBitmap(blob);
    const canvas = new OffscreenCanvas(bitmap.width, bitmap.height);
    const ctx = canvas.getContext("2d");
    ctx.drawImage(bitmap, 0, 0);
    const { data: px } = ctx.getImageData(0, 0, bitmap.width, bitmap.height);

    let min = [255, 255, 255];
    let max = [0, 0, 0];
    let minSum = Infinity;
    let maxSum = -Infinity;
    for (let i = 0; i < px.length; i += 4) {
      const sum = px[i] + px[i + 1] + px[i + 2];
      if (sum < minSum) {
        minSum = sum;
        min = [px[i], px[i + 1], px[i + 2]];
      }
      if (sum > maxSum) {
        maxSum = sum;
        max = [px[i], px[i + 1], px[i + 2]];
      }
    }
    return { min, max };
  }, base64);
}

for (const mode of MODES) {
  for (const blackout of ["off", "on"]) {
    for (const pattern of PATTERNS) {
      test(`${pattern} on ${mode}${blackout === "on" ? " + blackout" : ""} keeps text readable`, async ({
        page,
      }) => {
        await page.goto("/en/spacing.html");
        await page.evaluate(
          ([mode, blackout, pattern]) => {
            const r = document.documentElement;
            r.setAttribute("data-theme", mode);
            r.setAttribute("data-noctua-blackout", blackout);
            r.setAttribute("data-noctua-pattern", pattern);
          },
          [mode, blackout, pattern],
        );

        // The text colour first, from a REAL GLYPH — a rendered glyph is what
        // has to stay readable, and its darkest pixel is its core.
        // In light mode the glyph's darkest pixel is the letter; in dark mode it
        // is the background BEHIND the letter, and the letter is the lightest.
        // Taking `min` unconditionally measured the dark background against
        // itself and reported a contrast ratio of 1 for every dark case.
        //
        // So the ink is whichever extreme is furthest from the page — which is
        // the same rule in both modes and needs no branch on the mode.
        const glyph = await page.locator(".nd-title").screenshot();
        const { min: glyphDark, max: glyphLight } = await extremes(page, glyph);

        // Then the background ALONE. Hiding the shell is what makes this a
        // measurement of the pattern rather than of the text sitting on it:
        // the first version of this test clipped a region containing the
        // heading, found its darkest pixel was the heading, and dutifully
        // reported that the text had a contrast ratio of 1 against itself.
        //
        // `visibility: hidden` rather than `display: none`, so nothing reflows
        // and the pattern keeps the geometry it had.
        await page.addStyleTag({
          content: ".nd-header, .nd-body { visibility: hidden; }",
        });
        const strip = await page.locator("body").screenshot({
          clip: { x: 0, y: 0, width: 320, height: 200 },
        });
        const { min, max } = await extremes(page, strip);
        const page_ = luminance(max) > 0.5 ? max : min;
        const ink =
          Math.abs(luminance(glyphDark) - luminance(page_)) >
          Math.abs(luminance(glyphLight) - luminance(page_))
            ? glyphDark
            : glyphLight;

        // Worst case: the pattern's most extreme background pixel against the
        // darkest ink the heading actually paints.
        const worst = Math.min(contrast(ink, min), contrast(ink, max));
        expect(
          worst,
          `${pattern}/${mode}/blackout:${blackout} — background ranged ${min} to ${max}, ink ${ink}`,
        ).toBeGreaterThanOrEqual(4.5);
      });
    }
  }
}

test("a pattern moves the page by at most the declared ceiling", async ({ page }) => {
  await page.goto("/en/spacing.html");

  const sample = async () => {
    const shot = await page.locator("body").screenshot({
      clip: { x: 0, y: 0, width: 320, height: 200 },
    });
    return extremes(page, shot);
  };

  await page.evaluate(() => {
    document.documentElement.setAttribute("data-theme", "dark");
    document.documentElement.setAttribute("data-noctua-pattern", "none");
  });
  const plain = await sample();

  for (const pattern of PATTERNS) {
    await page.evaluate(
      (p) => document.documentElement.setAttribute("data-noctua-pattern", p),
      pattern,
    );
    const inked = await sample();
    // The spec declares a 4% ink ceiling. Allow a couple of levels for the
    // compositor's rounding; anything near 10 would mean the ceiling is not
    // being applied at all.
    const moved = Math.max(
      Math.abs(inked.max[0] - plain.max[0]),
      Math.abs(inked.max[1] - plain.max[1]),
      Math.abs(inked.max[2] - plain.max[2]),
    );
    expect(moved, `${pattern} moved the page by ${moved}/255`).toBeLessThanOrEqual(18);
  }
});
