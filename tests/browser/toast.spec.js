// The message that floats, and the timer that has to be interruptible.
//
// WHAT IS BEING GUARDED. A message that removes itself on a timer is unreadable
// to a reader who reads slowly, to one using a magnifier who has not reached
// that corner, and to one who looked away. So three things are asserted here:
// the timer is absent unless a caller asked for one, it holds while a pointer or
// the focus is on the message, and the element survives its own dismissal
// because it is a live region.
//
// And the quiet one: `role="status"` rather than `role="alert"`, for
// `MessageBar`'s reason. An alert cuts a screen reader off mid-sentence, and a
// component whose default is the rude one gets the rude one everywhere.

import { expect, test } from "@playwright/test";

import { PAGE, url, showParts } from "./pages.js";

const named = (page, name) => page.locator(`[data-nd-toast="${name}"]`);

async function toasts(page) {
  await page.goto(url(PAGE.toast));
  await showParts(page);
  return page.locator(".nd-toast");
}

test("a message is a status, and its region is armed after the page settles", async ({ page }) => {
  const all = await toasts(page);
  expect(await all.count()).toBeGreaterThan(0);
  const roles = await all.evaluateAll((els) => els.map((el) => el.getAttribute("role")));
  expect([...new Set(roles)]).toEqual(["status"]);

  // QUIET WHILE THE PAGE PAINTS, ARMED AFTERWARDS. A message that shipped with
  // the page has already been read by the time a reader reaches it, so a page
  // of them announcing themselves on load is a page talking about itself.
  // `inputs.js` and `carousel.js` make the same switch for the same reason.
  await page.waitForFunction(() =>
    [...document.querySelectorAll(".nd-toast")].every(
      (el) => el.getAttribute("aria-live") !== "off",
    ),
  );

  // And what comes back is exactly what the caller wrote, including nothing:
  // `role="status"` already implies polite, so an attribute written at that
  // value would be the component overruling the role it just chose.
  const live = await all.evaluateAll((els) => els.map((el) => el.getAttribute("aria-live")));
  expect(live.filter((one) => one === "polite"), "a status was told to be polite").toEqual([]);
  expect(live.some((one) => one === null), "every message overrides its own role").toBe(true);
  expect(live.some((one) => one === "assertive"), "the loud one is not demonstrated").toBe(true);
});

test("with the script blocked the message is whole and closes nothing", async ({ browser }) => {
  const context = await browser.newContext({ javaScriptEnabled: false });
  const page = await context.newPage();
  await page.goto(url(PAGE.toast));
  await showParts(page);
  // The words are in the markup, so a page that never ran a script still says
  // what happened. The control is not, because it would close nothing.
  await expect(named(page, "toast-saved").locator(".nd-toast-body")).toBeVisible();
  await expect(named(page, "toast-saved").locator(".nd-toast-dismiss")).toBeHidden();
  await context.close();
});

test("the script reveals the control, and pressing it takes the message away", async ({ page }) => {
  await toasts(page);
  const toast = named(page, "toast-saved");
  const control = toast.locator(".nd-toast-dismiss");
  await expect(control).toBeVisible();
  expect(await control.evaluate((el) => el.tagName)).toBe("BUTTON");
  expect(await control.getAttribute("type")).toBe("button");
  // A glyph, so the name is the only thing a screen reader has.
  expect(await control.getAttribute("aria-label")).toMatch(/\S\s\S/);
  const box = await control.boundingBox();
  expect(box.width).toBeGreaterThanOrEqual(24);
  expect(box.height).toBeGreaterThanOrEqual(24);

  await control.click();
  await expect(toast).toBeHidden();
  // THE ELEMENT STAYS. A live region taken out of the document announces
  // nothing the next time a caller writes into it.
  await expect(toast).toBeAttached();
  expect(await toast.getAttribute("data-nd-dismissed")).toBe("");
  // AND FOCUS IS NOT LEFT WHERE THE READER CANNOT SEE IT.
  const landed = await page.evaluate(() => document.activeElement !== document.body);
  expect(landed, "focus fell to the body when a message was dismissed").toBe(true);
});

test("no timer unless a caller asked for one", async ({ page }) => {
  const all = await toasts(page);
  const timers = await all.evaluateAll((els) =>
    els.map((el) => ({ name: el.dataset.ndToast, every: el.dataset.ndDuration ?? null })),
  );
  // The default is a message that stays, which is the component's one opinion.
  expect(timers.some((one) => one.every === null), "no message without a timer").toBe(true);
  expect(timers.some((one) => one.every), "the timer is not demonstrated").toBe(true);

  // And a message with no `duration` is still there after longer than the
  // longest one on the page.
  await page.waitForTimeout(1200);
  await expect(named(page, "toast-saved")).toBeVisible();
});

test("the timer holds under a pointer and under the focus", async ({ page }) => {
  await toasts(page);
  const toast = named(page, "toast-sent");
  await expect(toast).toBeVisible();

  await toast.hover();
  await expect(toast).toHaveAttribute("data-nd-paused", "");
  // A reader who moved onto the message is a reader still reading it, so the
  // wait must not run out underneath them.
  await page.waitForTimeout(400);
  await expect(toast).toBeVisible();

  // The focus holds it too, and counting the two separately is what stops one
  // of them leaving from restarting the wait under the other.
  await toast.locator(".nd-toast-dismiss").focus();
  await page.locator("h1").hover();
  await expect(toast).toHaveAttribute("data-nd-paused", "");

  await page.locator("h1").click();
  await expect(toast).not.toHaveAttribute("data-nd-paused", "");
});

test("the tone paints an edge and never carries the message alone", async ({ page }) => {
  const all = await toasts(page);
  const silent = await all.evaluateAll((els) =>
    els
      .filter((el) => el.dataset.ndTone)
      .filter((el) => !el.querySelector(".nd-toast-body")?.textContent?.trim())
      .map((el) => el.dataset.ndTone),
  );
  // `NOCTUA.md §8.8.6`: the words are the signal and the colour is a second one.
  expect(silent, "a message is saying it in colour alone").toEqual([]);

  const titles = await page.locator(".nd-toast-title").evaluateAll((els) => [
    ...new Set(els.map((el) => el.tagName)),
  ]);
  // A `<strong>` and never a heading: a message arriving after the page has
  // loaded must not rewrite the outline under a reader navigating by it.
  expect(titles).toEqual(["STRONG"]);
});

test("it floats in a corner it was told to, and carries no shadow", async ({ page }) => {
  const all = await toasts(page);
  const shapes = await all.evaluateAll((els) =>
    els.map((el) => ({
      name: el.dataset.ndToast,
      position: getComputedStyle(el).position,
      shadow: getComputedStyle(el).boxShadow,
    })),
  );
  for (const shape of shapes) {
    expect(shape.position, `${shape.name} is not floating`).toBe("fixed");
    expect(shape.shadow, `${shape.name} is drawing an elevated surface`).toBe("none");
  }

  // TOP AND BOTTOM ARE DIFFERENT CORNERS, which is the whole of `placement`.
  const where = await page.evaluate(() => {
    const box = (name) =>
      document.querySelector(`[data-nd-toast="${name}"]`).getBoundingClientRect();
    return { top: box("toast-failed").top, bottom: box("toast-offline").top };
  });
  expect(where.top).toBeLessThan(where.bottom);
});

test("a floating message stays inside the page it is documented on", async ({ page }) => {
  await page.setViewportSize({ width: 320, height: 800 });
  const all = await toasts(page);
  // `.nd-example-toasts` is `contain: layout`, so a `position: fixed` message is
  // pinned to its own example rather than to the window. Without it, four of
  // them stack in one corner over the documentation about them.
  const outside = await all.evaluateAll((els) => {
    const main = document.querySelector(".nd-main").getBoundingClientRect();
    return els
      .filter((el) => {
        const box = el.getBoundingClientRect();
        return box.right > main.right + 1 || box.left < main.left - 1;
      })
      .map((el) => el.dataset.ndToast);
  });
  expect(outside, "a message escaped the column it belongs to").toEqual([]);
});
