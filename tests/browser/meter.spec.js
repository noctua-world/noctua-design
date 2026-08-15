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
  for (const name of ["storage", "freezer", "tokens-low"]) {
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
  // A meter with no thresholds makes no claim. `freezer` is the one on the
  // page that declares none, and it is the one that declares a `min` below
  // zero, which is the only thing that prop is for.
  expect(await named(page, "freezer").getAttribute("data-nd-level")).toBe("middle");
  expect(await named(page, "freezer").locator("meter").getAttribute("min")).toBe("-20");
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
    const bars = await page.locator("[data-nd-meter]").evaluateAll((all) =>
      all.map((one, at) => ({
        name: one.getAttribute("data-nd-meter"),
        reading: one.querySelector("meter")?.getAttribute("aria-valuetext") ?? null,
        label: one.querySelector("label")?.textContent?.trim() ?? null,
        // Which worked example this bar belongs to. Two figures may measure
        // the same thing at different readings; two bars in ONE figure may
        // not share a name, because they are read one after the other.
        figure: [...document.querySelectorAll("figure")].indexOf(one.closest("figure")) || at,
      })),
    );

    // EVERY, WHICH IS WHAT THE TITLE SAYS. This asserted `distinct > 4` over
    // eleven meters, so three bars could share one reading and two more could
    // share another while the sweep stayed green, and both pairs did. A count
    // above a floor is not the claim "every one differs".
    expect(bars.length, `${locale}: the page drew almost no meter`).toBeGreaterThan(10);
    expect(bars.every((one) => one.reading), `${locale}: a meter announces no reading`).toBe(true);

    // Two examples showing the same number is two pictures of one thing, which
    // is the defect this whole page was rebuilt to stop shipping.
    const readings = bars.map((one) => one.reading);
    const twice = readings.filter((one, at) => readings.indexOf(one) !== at);
    expect(twice, `${locale}: one reading is drawn twice`).toEqual([]);

    // AND THE NAME BESIDE IT. Three threshold bars shipped under one label,
    // four lines under this page's own note saying three bars with one label
    // is three bars nobody can use. A distinct reading a reader cannot tell
    // apart by name is the same defect wearing different numbers.
    const stacks = new Map();
    for (const bar of bars) {
      if (!stacks.has(bar.figure)) stacks.set(bar.figure, []);
      stacks.get(bar.figure).push(bar.label);
    }
    const shared = [...stacks.values()].flatMap((labels) =>
      labels.filter((one, at) => labels.indexOf(one) !== at),
    );
    expect(shared, `${locale}: two bars in one example answer to one name`).toEqual([]);
    // The floor for the line above: an example holding several bars has to
    // exist, or the check runs over eleven groups of one and proves nothing.
    expect(
      Math.max(...[...stacks.values()].map((one) => one.length)),
      `${locale}: no example draws more than one bar`,
    ).toBeGreaterThan(2);
  }
});
