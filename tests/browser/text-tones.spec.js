// The two tones a sentence takes when it says what went wrong or right.
//
// `Text` and `Paragraph` took `default`, `muted` and, for `Text`, `accent`, so
// an interface drawing a kernel's `danger` or `success` text drew it in the
// default tone. The tones are the colour system's `danger` and `success`
// contexts, the ones a field's own sentence is drawn in, and nothing here
// defines a colour.
//
// What is asked is asked of pixels: each tone is its own ink, and the ink is
// readable on the page in both modes. `variants.spec.js` asks, of every prop,
// that no two values draw one picture. This file names the pair that was
// missing and measures it against the page.

import { test, expect } from "@playwright/test";
import { url, showParts } from "./pages.js";
import { extremes, contrast } from "./pixels.js";

/** WCAG 1.4.3, for text at its body size. */
const FLOOR = 4.5;

const SUBJECTS = [
  ["Text", "inline-text/", ".nd-text"],
  ["Paragraph", "paragraph/", ".nd-paragraph"],
];

/** The ink and the ground of an element: the two extremes of its picture. */
async function drawn(page, locator) {
  await locator.scrollIntoViewIfNeeded();
  const { min, max } = await extremes(page, await locator.screenshot());
  return { min, max, ratio: contrast(min, max) };
}

// The page at its lightest and at the darkest a light page gets, in both modes.
const GROUNDS = ["plain", "softer"];

for (const [name, path, root] of SUBJECTS) {
  for (const mode of ["light", "dark"]) {
    for (const ground of GROUNDS) {
    test(`${name} draws danger and success in their own readable ink, ${mode} on ${ground}`, async ({ page }) => {
      await page.goto(url(path));
      await showParts(page);
      await page.evaluate(
        ([theme, backdrop]) => {
          document.documentElement.setAttribute("data-theme", theme);
          document.documentElement.setAttribute("data-noctua-backdrop", backdrop);
        },
        [mode, ground],
      );

      const plain = await drawn(page, page.locator(`${root}[data-nd-tone="default"]`).first());
      const seen = { default: plain };
      for (const tone of ["danger", "success"]) {
        const element = page.locator(`${root}[data-nd-tone="${tone}"]`).first();
        await expect(element, `no ${name} on the page takes the ${tone} tone`).toHaveCount(1);
        seen[tone] = await drawn(page, element);
        expect(
          seen[tone].ratio,
          `${name} in the ${tone} tone is ${seen[tone].ratio.toFixed(2)}:1 on a ${mode} page`,
        ).toBeGreaterThanOrEqual(FLOOR);
      }

      // The ink is the darker extreme on a light page and the lighter on a dark one.
      const ink = (picture) => (mode === "light" ? picture.min : picture.max).join(",");
      expect(ink(seen.danger), "danger is drawn in the default ink").not.toBe(ink(seen.default));
      expect(ink(seen.success), "success is drawn in the default ink").not.toBe(ink(seen.default));
      expect(ink(seen.danger), "danger and success are one ink").not.toBe(ink(seen.success));
    });
    }
  }
}
