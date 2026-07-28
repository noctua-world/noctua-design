// Does the glass actually sample?
//
// This is the test neither P0 had, and its absence is why both shipped. The
// 49 specs before it asserted that `backdrop-filter` was *declared* and that
// its computed value contained `blur` — which was true of the settings panel
// the whole time it was rendering a flat fill.
//
// A declared `backdrop-filter` is not a sampling one. It stops sampling if any
// ANCESTOR has `filter`, `opacity` below 1, `mask` or a `backdrop-filter` of
// its own, and the computed value on the element itself is unchanged when that
// happens. The only way to know is to look at the pixels.

import { test, expect } from "@playwright/test";

/** Variance of the luminance across a screenshot's pixels. */
async function variance(page, buffer) {
  return page.evaluate(async (data) => {
    const blob = await (await fetch(`data:image/png;base64,${data}`)).blob();
    const bitmap = await createImageBitmap(blob);
    const canvas = new OffscreenCanvas(bitmap.width, bitmap.height);
    const ctx = canvas.getContext("2d");
    ctx.drawImage(bitmap, 0, 0);
    const { data: px } = ctx.getImageData(0, 0, bitmap.width, bitmap.height);

    const values = [];
    for (let i = 0; i < px.length; i += 4) values.push((px[i] + px[i + 1] + px[i + 2]) / 3);
    const mean = values.reduce((a, b) => a + b, 0) / values.length;
    return values.reduce((a, v) => a + (v - mean) ** 2, 0) / values.length;
  }, buffer.toString("base64"));
}

/** No ancestor may carry anything that stops the backdrop sampling. */
async function filteredAncestors(page, selector) {
  return page.evaluate((sel) => {
    const el = document.querySelector(sel);
    const found = [];
    for (let node = el?.parentElement; node; node = node.parentElement) {
      const s = getComputedStyle(node);
      if (
        (s.backdropFilter && s.backdropFilter !== "none") ||
        (s.filter && s.filter !== "none") ||
        Number(s.opacity) < 1 ||
        (s.maskImage && s.maskImage !== "none")
      ) {
        found.push(`${node.className || node.tagName}: ${s.backdropFilter}/${s.filter}/${s.opacity}`);
      }
    }
    return found;
  }, selector);
}

test("the settings panel's glass has no filtered ancestor", async ({ page }) => {
  await page.goto("/en/spacing.html");
  await page.locator(".nd-settings > summary").click();
  // The header used to be exactly this: a `backdrop-filter` ancestor of the
  // panel, which silently turned the panel's own into a flat fill.
  expect(await filteredAncestors(page, ".nd-panel")).toEqual([]);
});

test("every glass surface has no filtered ancestor", async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto("/en/scoped-themes.html");
  await page.locator("[data-nd-menu]").click();
  await page.locator(".nd-settings > summary").click();

  for (const selector of [".nd-header-glass", ".nd-nav", ".nd-panel", "[data-noctua-scope]"]) {
    expect(await filteredAncestors(page, selector), selector).toEqual([]);
  }
});

test("a frosted surface sees what is behind it", async ({ page }) => {
  await page.goto("/en/spacing.html");
  // A hard-edged pattern behind the page. Through real glass it survives as a
  // blurred gradient with measurable variance; through a flat fill it does not
  // survive at all.
  await page.evaluate(() => {
    document.documentElement.setAttribute("data-noctua-glass", "full");
    const stripes = document.createElement("div");
    stripes.id = "probe";
    stripes.style.cssText =
      "position:fixed;inset:0;z-index:0;background:repeating-linear-gradient(90deg,#000 0 12px,#fff 12px 24px)"; // allow-literal: a test probe deliberately outside the system — maximum-contrast stripes at a fixed pitch are what make "did the blur sample this" measurable, and a palette colour would weaken the very signal being read
    document.body.prepend(stripes);
  });
  await page.locator(".nd-settings > summary").click();

  const panel = await page.locator(".nd-panel").screenshot();
  const spread = await variance(page, panel);

  // A flat fill over the stripes gives variance close to zero. Anything that
  // actually samples carries the pattern through, softened.
  expect(spread, "the panel rendered a flat fill — its backdrop is not sampling").toBeGreaterThan(20);
});

test("the colours page paints thirty-nine distinct palettes", async ({ page }) => {
  await page.goto("/en/colours.html");
  await page.waitForLoadState("networkidle");

  const distinct = await page.evaluate(() => {
    const paint = (v) => {
      const c = document.createElement("canvas");
      c.width = c.height = 1;
      const x = c.getContext("2d");
      x.fillStyle = v;
      x.fillRect(0, 0, 1, 1);
      return [...x.getImageData(0, 0, 1, 1).data].slice(0, 3).join(",");
    };
    const cells = [...document.querySelectorAll(".nd-palette")];
    const accents = cells.map((c) =>
      paint(getComputedStyle(c.querySelector(".nd-chip--accent")).backgroundColor), // allow-literal: read only to be painted into a canvas by `paint` on this same line, which is the recipe the gate prescribes
    );
    return { cells: cells.length, distinct: new Set(accents).size };
  });

  // The grid rendered thirty-nine cells and painted ONE palette until the
  // stylesheets were linked. The cell count passed every check; the colour did
  // not, because nothing looked at it.
  expect(distinct.cells).toBe(39);
  expect(distinct.distinct, "the palette stylesheets are not being linked").toBe(39);
});
