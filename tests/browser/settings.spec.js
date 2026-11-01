// The header's controls, and the panel behind the gear.
//
// Every assertion here corresponds to something that shipped broken and was
// invisible: an accent row of thirteen identical swatches, four controls at
// four different heights, a panel that would not close, and a default palette
// whose stylesheet applied to nothing.

import { test, expect } from "@playwright/test";
import { ANY, url } from "./pages.js";

/** The pixels a serialised color actually paints. */
async function paint(page, value) {
  return page.evaluate((v) => {
    const c = document.createElement("canvas");
    c.width = c.height = 1;
    const ctx = c.getContext("2d");
    ctx.fillStyle = v;
    ctx.fillRect(0, 0, 1, 1);
    return [...ctx.getImageData(0, 0, 1, 1).data].slice(0, 3);
  }, value);
}

test("every control in the header is one box on one centre line", async ({ page }) => {
  await page.goto(url(ANY));

  const boxes = await page
    .locator(".nd-header-controls > *")
    .evaluateAll((els) =>
      els.map((el) => {
        const r = el.getBoundingClientRect();
        return { cls: el.className, height: r.height, centre: r.top + r.height / 2 };
      }),
    );

  // Four: the implementation toggle, the repository link, and the two
  // `<details>` — the language menu and the theme menu.
  //
  // It was three, and it was three because a fourth had been removed: a mode
  // control the theme menu already had a row for. This fourth is not that. It
  // is the one control here that is not an appearance, and the theme panel is
  // titled *Theme*, so a row for it there would be a category error in a panel
  // generated from the theme model. An icon, like the rest, because the budget
  // is one line at 320 px.
  expect(boxes.length, "the header lost a control").toBe(4);

  // Four controls, four elements — two buttons and two `<details>` — and each
  // used to size itself. `align-items: center` then faithfully centred three
  // different heights, which is what "slightly off" looks like.
  const heights = new Set(boxes.map((b) => Math.round(b.height)));
  expect([...heights], `the controls disagree about their height: ${JSON.stringify(boxes)}`)
    .toHaveLength(1);

  const centres = new Set(boxes.map((b) => Math.round(b.centre * 100)));
  expect([...centres], "the controls sit on different centre lines").toHaveLength(1);
});

test("every accent swatch shows its own accent", async ({ page }) => {
  await page.goto(url(ANY));
  await page.locator(".nd-settings > summary").click();

  // The stylesheets are fetched when the panel opens, and a swatch cannot show
  // a palette whose file has not landed.
  //
  // Waiting for the ATTRIBUTE is not enough, and the difference is a flake
  // rather than a failure: the attribute is stamped in the same tick the link
  // is created, so a swatch whose sheet is still in flight paints the page's
  // accent and reads as a duplicate of whichever accent the page is on.
  // `link.sheet` is null until the file is parsed, which is the question.
  await page.waitForFunction(
    () => {
      const swatches = [...document.querySelectorAll("[data-nd-group='accent'] .nd-swatch")];
      if (!swatches.every((s) => s.hasAttribute("data-palette"))) return false;
      const links = [...document.querySelectorAll("link[data-nd-palette]")];
      return links.length >= swatches.length && links.every((l) => l.sheet !== null);
    },
    null,
    { timeout: 10_000 },
  );

  const swatches = page.locator("[data-nd-group='accent'] .nd-swatch");
  const count = await swatches.count();
  expect(count, "the accent row is empty").toBe(13);

  // Read from PIXELS, never from a serialised string (`NOCTUA.md §9.6`, and
  // the `color_read_from_dom` gate). Two accents that differ only in hue
  // serialise differently in every engine and the same in none.
  const painted = new Set();
  for (let i = 0; i < count; i++) {
    const value = await swatches.nth(i).evaluate((el) => getComputedStyle(el).backgroundColor); // allow-literal: read only to be painted into a canvas by `paint` on the next line, which is the recipe the gate prescribes
    painted.add((await paint(page, value)).join(","));
  }

  // The whole defect in one number. The script's hook was lost when the panel
  // became a generated `Disclosure`, so no swatch was ever given a
  // `data-palette` and all thirteen painted the page's accent: this was 1.
  expect(painted.size, "the swatches are not showing their own palettes").toBe(13);
});

test("the settings panel closes when the reader clicks past it", async ({ page }) => {
  await page.goto(url(ANY));
  const panel = page.locator(".nd-settings");

  await panel.locator("summary").click();
  await expect(panel).toHaveAttribute("open", "");

  await page.locator(".nd-title").click();
  await expect(panel).not.toHaveAttribute("open", "");
});

test("the default palette applies before any script runs", async ({ page }) => {
  // `noctua-colors` anchors one palette at `:root` and every other at
  // `[data-palette]`. This system's default is not that one, so the attribute
  // is what makes the base stylesheet apply at all — and without it the page
  // has no palette whatsoever rather than the wrong one.
  await page.route("**/assets/shell.js", (route) => route.abort());
  await page.goto(url(ANY));

  const palette = await page.locator("html").getAttribute("data-palette");
  expect(palette).toBe("blue-balanced");

  const declared = await page
    .locator("body")
    .evaluate((el) => getComputedStyle(el).getPropertyValue("--nc-color-accent")); // allow-literal: read only to be painted into a canvas on the next line, which is the recipe the gate prescribes
  const accent = await paint(page, declared);
  // Blue, not the vendored default's ochre: blue's channel ordering is
  // b > g > r, and ochre's is the reverse.
  expect(accent[2], `the accent painted ${accent}`).toBeGreaterThan(accent[0]);
});

test("the language menu offers three answers and remembers one", async ({ page }) => {
  await page.goto(url(ANY));

  const options = page.locator("[data-nd-locale]");
  await expect(options).toHaveCount(3);

  // Automatic is server-rendered as the one in force. Every option shipping
  // `aria-current` absent would tell a screen reader that none of three is
  // chosen, which is a confident wrong answer rather than no answer.
  await expect(page.locator("[data-nd-locale='auto']")).toHaveAttribute("aria-current", "true");

  await page.locator(".nd-locale > summary").click();
  await page.locator("[data-nd-locale='en']").click();
  expect(await page.evaluate(() => localStorage.getItem("noctua-locale"))).toBe("en");
});
