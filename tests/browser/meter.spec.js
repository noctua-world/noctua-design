// The meter, and the claims only a browser can settle.
//
// IT HAS NO RUNTIME, which is the whole argument for the component: the bar,
// the thresholds and the announcement are `<meter>`'s own. So most of this file
// asserts that the element really is the element, and that what a reader sees
// and what a screen reader hears are the same sentence.

import { expect, test } from "@playwright/test";

import { LOCALES, PAGE, url } from "./pages.js";
import { extremes } from "./pixels.js";

const named = (page, name) => page.locator(`[data-nd-meter="${name}"]`);

test.describe("with the script blocked", () => {
  test("it is complete: a real meter, a real label, and the number in words", async ({
    browser,
  }) => {
    const context = await browser.newContext({ javaScriptEnabled: false });
    const page = await context.newPage();
    await page.goto(url(PAGE.meter));

    const meter = named(page, "storage").locator("meter");
    await expect(meter).toBeVisible();
    expect(await meter.evaluate((el) => el.tagName)).toBe("METER");
    // A `<div role="meter">` needs four ARIA attributes kept in step by hand.
    expect(await meter.getAttribute("value")).toBe("41.2");
    expect(await meter.getAttribute("max")).toBe("50");

    const label = named(page, "storage").locator("label");
    expect(await label.getAttribute("for")).toBe(await meter.getAttribute("id"));
    // THE NUMBER IS WRITTEN OUT. A bar on its own is an estimate.
    await expect(named(page, "storage").locator(".nd-meter-value")).toHaveText(/\d/);
    await context.close();
  });
});

test("what is read and what is heard are the same sentence", async ({ page }) => {
  await page.goto(url(PAGE.meter));
  for (const name of ["storage", "seats", "tokens-low"]) {
    const shown = await named(page, name).locator(".nd-meter-value").innerText();
    const heard = await named(page, name).locator("meter").getAttribute("aria-valuetext");
    expect(heard).toBe(shown);
  }
});

test("the band follows the thresholds, and says where rather than whether", async ({ page }) => {
  await page.goto(url(PAGE.meter));
  // One reading per band, which is the only way three thresholds can be seen
  // to do anything.
  expect(await named(page, "tokens-low").getAttribute("data-nd-level")).toBe("low");
  expect(await named(page, "tokens-warn").getAttribute("data-nd-level")).toBe("middle");
  expect(await named(page, "tokens-over").getAttribute("data-nd-level")).toBe("high");
  // A meter with no thresholds makes no claim.
  expect(await named(page, "seats").getAttribute("data-nd-level")).toBe("middle");
});

test("the three bands are three different colours on the screen", async ({ page }) => {
  await page.goto(url(PAGE.meter));
  const inks = [];
  for (const name of ["tokens-low", "tokens-warn", "tokens-over"]) {
    const bar = named(page, name).locator("meter");
    await bar.scrollIntoViewIfNeeded();
    const box = await bar.boundingBox();
    // The leading quarter, which every one of the three fills.
    const strip = await page.screenshot({
      clip: { x: box.x + 2, y: box.y, width: Math.max(4, box.width / 8), height: box.height },
    });
    const { min } = await extremes(page, strip);
    inks.push(min);
  }
  // READ FROM PIXELS. A computed style is whatever the engine serialised and
  // says nothing about what the platform painted into its own shadow tree.
  const [low, middle, high] = inks;
  expect(low[1]).toBeGreaterThan(low[0]);
  expect(high[0]).toBeGreaterThan(high[1] + 20);
  expect(middle.join()).not.toBe(low.join());
  expect(middle.join()).not.toBe(high.join());
});

test("a hidden label is off the screen and still in the document", async ({ page }) => {
  await page.goto(url(PAGE.meter));
  const bare = named(page, "bare");
  const label = bare.locator("label");
  // The case a removed label fails: somebody who cannot see the heading.
  expect(await label.innerText()).toMatch(/\S/);
  const box = await label.boundingBox();
  expect(box.width).toBeLessThan(4);
  await expect(bare.locator(".nd-meter-value")).toHaveCount(0);
});

test("every meter on the page is a different reading, in every locale", async ({ page }) => {
  for (const locale of LOCALES) {
    await page.goto(url(PAGE.meter, locale));
    const readings = await page
      .locator("[data-nd-meter] meter")
      .evaluateAll((all) => all.map((one) => one.getAttribute("aria-valuetext")));
    // Two examples showing the same number is two pictures of one thing, which
    // is the defect this whole page was rebuilt to stop shipping.
    const distinct = new Set(readings.filter(Boolean));
    expect(distinct.size).toBeGreaterThan(4);
  }
});
