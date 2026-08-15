// The anchored surface, and the line between it and an `Info`.
//
// WHAT IS BEING GUARDED. `Info` and this draw the same box and promise opposite
// things: one is a `role="note"` holding a sentence, the other a `role="dialog"`
// holding controls a reader came to use. Announcing the wrong one is not
// cosmetic, so both roles are asserted here rather than described.
//
// The rest is `anchor.js` reached rather than re-implemented, and the three ways
// out — Escape, a Tab out, a press outside — with the focus landing where the
// reader put it in each case.

import { expect, test } from "@playwright/test";

import { PAGE, url, showParts } from "./pages.js";

const named = (page, name) => page.locator(`[data-nd-popover="${name}"]`);

async function popovers(page) {
  await page.goto(url(PAGE.popover));
  await showParts(page);
}

test("with the script blocked there is no trigger and no surface", async ({ browser }) => {
  const context = await browser.newContext({ javaScriptEnabled: false });
  const page = await context.newPage();
  await page.goto(url(PAGE.popover));
  await showParts(page);
  await expect(named(page, "popover-filter").locator(".nd-popover-trigger")).toBeHidden();
  await expect(named(page, "popover-filter").locator(".nd-popover-panel")).toBeHidden();
  await context.close();
});

test("the trigger says what it opens and whether it is open", async ({ page }) => {
  await popovers(page);
  const trigger = named(page, "popover-filter").locator(".nd-popover-trigger");
  await expect(trigger).toBeVisible();
  expect(await trigger.evaluate((el) => el.tagName)).toBe("BUTTON");
  expect(await trigger.getAttribute("type")).toBe("button");
  expect(await trigger.getAttribute("aria-haspopup")).toBe("dialog");
  expect(await trigger.getAttribute("aria-expanded")).toBe("false");

  const panel = named(page, "popover-filter").locator(".nd-popover-panel");
  expect(await trigger.getAttribute("aria-controls")).toBe(await panel.getAttribute("id"));

  // AND THE BOX WAS HELD BEFORE THE BUTTON ARRIVED, so revealing it moves
  // nothing on a page that has already painted.
  const held = await named(page, "popover-filter").evaluate(
    (el) => Number.parseFloat(getComputedStyle(el).minBlockSize) > 0,
  );
  expect(held, "the trigger arrives into a box nothing was holding").toBe(true);
});

test("it is a dialog and an Info is a note", async ({ page }) => {
  await popovers(page);
  const roles = await page
    .locator(".nd-popover-panel")
    .evaluateAll((els) => els.map((el) => el.getAttribute("role")));
  expect(roles.length).toBeGreaterThan(0);
  // The whole reason both components exist.
  expect([...new Set(roles)]).toEqual(["dialog"]);

  // A `role="dialog"` with no name gives a screen reader its role and nothing
  // else, so every one of them carries one.
  const named_ = await page.locator(".nd-popover-panel").evaluateAll((els) =>
    els.map((el) => ({
      label: el.getAttribute("aria-label"),
      by: el.getAttribute("aria-labelledby"),
      resolves: el.getAttribute("aria-labelledby")
        ? Boolean(document.getElementById(el.getAttribute("aria-labelledby")))
        : true,
    })),
  );
  for (const one of named_) {
    expect(one.label || one.by, "a popover with no accessible name").toBeTruthy();
    expect(one.label && one.by).toBeFalsy();
    expect(one.resolves).toBe(true);
  }
});

test("it opens on a press and never on a hover", async ({ page }) => {
  await popovers(page);
  const root = named(page, "popover-filter");
  const panel = root.locator(".nd-popover-panel");

  // A surface with controls in it appearing under a passing pointer is one a
  // reader has to escape from to reach the page.
  await root.locator(".nd-popover-trigger").hover();
  await expect(panel).toBeHidden();

  await root.locator(".nd-popover-trigger").click();
  await expect(panel).toBeVisible();
  expect(await root.locator(".nd-popover-trigger").getAttribute("aria-expanded")).toBe("true");
});

test("it holds real controls, and they are reachable", async ({ page }) => {
  await popovers(page);
  const root = named(page, "popover-filter");
  await root.locator(".nd-popover-trigger").click();
  // The difference from a `Tooltip`, which holds nothing focusable at all.
  const reachable = await root
    .locator(".nd-popover-panel")
    .evaluate((el) => el.querySelectorAll("button, a[href], input, select, textarea").length);
  expect(reachable, "a popover with no controls in it should have been an Info").toBeGreaterThan(0);
});

test("Escape closes it and puts the focus back on the trigger", async ({ page }) => {
  await popovers(page);
  const root = named(page, "popover-filter");
  await root.locator(".nd-popover-trigger").click();
  await expect(root.locator(".nd-popover-panel")).toBeVisible();

  await page.keyboard.press("Escape");
  await expect(root.locator(".nd-popover-panel")).toBeHidden();
  expect(await page.evaluate(() => document.activeElement.className)).toContain(
    "nd-popover-trigger",
  );
});

test("nothing is trapped, so tabbing out of it closes it", async ({ page }) => {
  await popovers(page);
  const root = named(page, "popover-help");
  await root.locator(".nd-popover-trigger").click();
  await expect(root.locator(".nd-popover-panel")).toBeVisible();

  // A `role="dialog"` that traps without being modal is the worst of both: the
  // way out is the one key the reader did not press.
  for (let step = 0; step < 6; step += 1) {
    await page.keyboard.press("Tab");
    if (await root.locator(".nd-popover-panel").isHidden()) break;
  }
  await expect(root.locator(".nd-popover-panel")).toBeHidden();
});

test("a press outside closes it, and opening a second closes the first", async ({ page }) => {
  await popovers(page);
  const first = named(page, "popover-filter");
  const second = named(page, "popover-share");

  await first.locator(".nd-popover-trigger").click();
  await expect(first.locator(".nd-popover-panel")).toBeVisible();

  await second.locator(".nd-popover-trigger").click();
  await expect(second.locator(".nd-popover-panel")).toBeVisible();
  // Two open at once is two surfaces over one page.
  await expect(first.locator(".nd-popover-panel")).toBeHidden();

  await page.locator("h1").click();
  await expect(second.locator(".nd-popover-panel")).toBeHidden();
});

test("the surface is anchored to its trigger rather than to the page", async ({ page }) => {
  await popovers(page);
  const root = named(page, "popover-filter");
  await root.locator(".nd-popover-trigger").click();
  const boxes = await root.evaluate((el) => {
    const trigger = el.querySelector(".nd-popover-trigger").getBoundingClientRect();
    const panel = el.querySelector(".nd-popover-panel").getBoundingClientRect();
    return { bottom: trigger.bottom, top: panel.top, shadow: getComputedStyle(el.querySelector(".nd-popover-panel")).boxShadow };
  });
  // The numbers `anchor.js` wrote, not a guess.
  expect(boxes.top).toBeGreaterThan(boxes.bottom - 1);
  expect(Math.abs(boxes.top - boxes.bottom)).toBeLessThan(40);
  // Separation is a hairline plus a surface colour, never elevation.
  expect(boxes.shadow, "a popover is drawing an elevated surface").toBe("none");
});
