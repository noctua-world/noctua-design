// The example consumer, measured in a browser.
//
// `examples/consumer-rust` asks whether the classes and the stylesheet agree;
// this asks the question only a rendering engine can answer — *is the page
// styled?* Nothing here compares against a baseline. It asserts NON-INITIAL:
// a browser's own default for a `<button>` is not a pill with a tone on it,
// and for a `<span>` it is not a rounded chip, so a value that differs from
// the initial one is a rule that arrived.
//
// It also opens a tab, because the behaviour ships from `dist/` too and a page
// that is painted but inert would pass every assertion above it.

import { expect, test } from "@playwright/test";

import { pixelAt } from "./pixels.js";

const PAGE = "http://127.0.0.1:8732/examples/consumer-web/index.html";

/** The colour actually painted at the middle of an element. */
async function paintedAt(page, locator) {
  const box = await locator.boundingBox();
  const shot = await page.screenshot();
  return pixelAt(page, shot, box.x + box.width / 2, box.y + box.height / 2);
}

test("nine links and no build step is a styled page", async ({ page }) => {
  await page.goto(PAGE);

  // Shape first, because a browser's own default for a `<button>` is not a
  // rounded box two and a half rems tall.
  const button = page.locator(".nd-button").first();
  const radius = await button.evaluate((el) => getComputedStyle(el).borderRadius);
  const height = await button.evaluate((el) => el.getBoundingClientRect().height);
  expect(radius, "a button with no rules is square").not.toBe("0px");
  expect(height, "a button with no rules is text-height").toBeGreaterThan(28);

  // Then paint, read from pixels because that is the only honest way here.
  // A solid button is filled; the page behind it is not.
  const filled = await paintedAt(page, button);
  const page_ground = await pixelAt(page, await page.screenshot(), 4, 4);
  expect(filled, "a solid button is painted in its tone").not.toEqual(page_ground);

  // The track is a shape a `<span>` has no reason to be.
  const box = await page.locator(".nd-switch-track").first().boundingBox();
  expect(box.width).toBeGreaterThan(20);
  expect(box.height).toBeGreaterThan(10);

  // A tone is a NAME, and the two tags here name two different families —
  // `overdue` and `shipped`, neither of which is one of four states.
  const tags = page.locator(".nd-tag");
  const overdue = await paintedAt(page, tags.nth(0));
  const shipped = await paintedAt(page, tags.nth(1));
  expect(overdue, "a toned tag is painted").not.toEqual(page_ground);
  expect(overdue, "two tone names are two colours").not.toEqual(shipped);
});

test("the behaviour ships too, so a tab strip is a tab strip", async ({ page }) => {
  await page.goto(PAGE);

  const strip = page.locator("[data-nd-tabs]");
  await expect(strip).toHaveAttribute("role", "tablist");

  const tabs = page.locator(".nd-tab");
  await expect(tabs.first()).toHaveAttribute("aria-selected", "true");
  await expect(page.locator("#panel-two")).toBeHidden();

  await tabs.nth(1).click();
  await expect(tabs.nth(1)).toHaveAttribute("aria-selected", "true");
  await expect(page.locator("#panel-one")).toBeHidden();
  await expect(page.locator("#panel-two")).toBeVisible();

  // One tab stop, and the arrows move within it — the promise `role="tablist"`
  // makes and `roving.js` is what keeps.
  await tabs.nth(1).focus();
  await page.keyboard.press("ArrowLeft");
  await expect(tabs.first()).toBeFocused();
});
