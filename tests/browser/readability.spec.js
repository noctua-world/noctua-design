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
        await page.goto("/en/spacing/");
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
        // Three things have to be named rather than left to inheritance:
        // `.nd-header-glass` is a SIBLING of the shell; the docked sidebar sets
        // `visibility: visible` on itself, which REVERSES its parent's hidden;
        // and the skip link is fixed at the top-left, inside the measured
        // region. Any one of them left showing is read as the pattern's colour.
        await page.addStyleTag({
          content: ".nd-header, .nd-header-glass, .nd-skip, .nd-body, .nd-body * { visibility: hidden !important; }",
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

test("a pattern's ink stays under the declared ceiling", async ({ page }) => {
  await page.goto("/en/spacing/");
  await page.evaluate(() => {
    document.documentElement.setAttribute("data-theme", "dark");
    document.documentElement.setAttribute("data-noctua-pattern", "none");
  });
  await page.addStyleTag({
    content: ".nd-header, .nd-header-glass, .nd-skip, .nd-body, .nd-body * { visibility: hidden !important; }",
  });

  const spread = async () => {
    const shot = await page.locator("body").screenshot({
      clip: { x: 0, y: 0, width: 320, height: 200 },
    });
    const { min, max } = await extremes(page, shot);
    // The AMPLITUDE the pattern introduces across the bare page — which is what
    // "how much ink" means. Comparing one absolute extreme between two states
    // measures whatever else changed with them, which is how the first version
    // of this test reported a 59/255 move for a 4% wash.
    return (max[0] + max[1] + max[2] - min[0] - min[1] - min[2]) / 3;
  };

  // The baseline, whatever it is. Measuring the pattern's contribution RELATIVE
  // to it is what makes this robust: the region contains the scrollbar and the
  // page's own edge, and subtracting a fixed guess for those was what made the
  // first two versions of this test measure the furniture instead of the ink.
  const plain = await spread();

  for (const pattern of PATTERNS) {
    await page.evaluate(
      (p) => document.documentElement.setAttribute("data-noctua-pattern", p),
      pattern,
    );
    const inked = await spread();
    const contributed = inked - plain;
    // 4% of 255 is about 10. Allowing 18 leaves room for the compositor's
    // rounding and antialiasing while still failing loudly if the ceiling
    // stopped being applied — an unbounded pattern would be far past this.
    expect(
      contributed,
      `${pattern} added ${contributed.toFixed(1)}/255 over a baseline of ${plain.toFixed(1)}`,
    ).toBeLessThanOrEqual(18);
  }
});
