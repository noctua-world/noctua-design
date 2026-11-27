// The page's colour, as data, held to the pixels a page paints.
//
// A native host paints its window before its page has a first frame, and it
// has no stylesheet to ask. `dist/palettes/<name>.json` carries `page`, the
// colour `dist/theme.css` gives the page for each mode and backdrop, and the
// theme crate carries the same table (`examples/consumer-rust` holds the two
// to each other). This file holds the table to the engine: the consumer page
// is put in each mode and backdrop, its background is painted into a canvas
// and read back, and the bytes must be the ones the file names.
//
// The light page was a literal in the sheet and data nowhere, so a host could
// paint a dark window from the palette's steps and a light one from nothing.

import { expect, test } from "@playwright/test";

const ORIGIN = "http://127.0.0.1:8732";
const PAGE = `${ORIGIN}/examples/consumer-web/index.html`;

/** The page's painted background, as `#rrggbb`. */
async function painted(page) {
  return page.evaluate(() => {
    const canvas = document.createElement("canvas");
    canvas.width = canvas.height = 1;
    const ctx = canvas.getContext("2d");
    ctx.fillStyle = getComputedStyle(document.documentElement).backgroundColor; // allow-literal: painted into the canvas on this line, the recipe the gate prescribes
    ctx.fillRect(0, 0, 1, 1);
    return `#${[...ctx.getImageData(0, 0, 1, 1).data]
      .slice(0, 3)
      .map((byte) => byte.toString(16).padStart(2, "0"))
      .join("")}`;
  });
}

for (const palette of ["blue-balanced", "amber-vivid"]) {
  test(`the page colours ${palette} names are the ones its page paints`, async ({ page, request }) => {
    const data = await (await request.get(`${ORIGIN}/dist/palettes/${palette}.json`)).json();
    expect(Object.keys(data.page), "the palette names no page colour").toEqual(["light", "dark"]);

    await page.goto(PAGE);
    // The palette's own sheet, after the default's, as the bootstrap adds it.
    await page.evaluate(async (name) => {
      const link = document.createElement("link");
      link.rel = "stylesheet";
      link.href = `../../packages/tokens/vendor/css/theme-${name}.css`;
      const loaded = new Promise((resolve, reject) => {
        link.addEventListener("load", resolve);
        link.addEventListener("error", reject);
      });
      document.getElementById("noctua-base-palette").after(link);
      await loaded;
      document.documentElement.setAttribute("data-palette", name);
    }, palette);

    let asked = 0;
    for (const mode of ["light", "dark"]) {
      const backdrops = Object.keys(data.page[mode]);
      expect(backdrops.length, `${mode} names fewer backdrops than there are`).toBeGreaterThanOrEqual(6);
      for (const backdrop of backdrops) {
        await page.evaluate(
          ([theme, value]) => {
            const root = document.documentElement;
            root.setAttribute("data-theme", theme);
            root.setAttribute("data-noctua-backdrop", value);
          },
          [mode, backdrop],
        );
        expect(await painted(page), `${palette}, ${mode}, ${backdrop}`).toBe(data.page[mode][backdrop]);
        asked += 1;
      }
    }
    expect(asked).toBe(12);

    // And with no backdrop named at all, which is how a page starts.
    for (const mode of ["light", "dark"]) {
      await page.evaluate((theme) => {
        const root = document.documentElement;
        root.setAttribute("data-theme", theme);
        root.removeAttribute("data-noctua-backdrop");
      }, mode);
      expect(await painted(page), `${palette}, ${mode}, no backdrop`).toBe(data.page[mode].plain);
    }
  });
}

test("a light page is paper and two palettes' dark pages differ", async ({ request }) => {
  // The table is not one row written thirty-nine times.
  const blue = await (await request.get(`${ORIGIN}/dist/palettes/blue-balanced.json`)).json();
  const amber = await (await request.get(`${ORIGIN}/dist/palettes/amber-vivid.json`)).json();
  expect(blue.page.light.plain).toBe("#ffffff"); // allow-literal: the paper the sheet declares, asserted
  expect(amber.page.light.plain).toBe("#ffffff"); // allow-literal: as above
  expect(blue.page.dark.tinted).not.toBe(amber.page.dark.tinted);
});
