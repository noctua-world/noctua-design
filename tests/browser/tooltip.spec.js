// The hint, and the two rules that keep it from becoming something else.
//
// WHAT IS BEING GUARDED. A tooltip is the one component in this system whose
// content some readers will never see, and that is only acceptable while two
// things hold:
//
// 1. IT IS NEVER THE CONTROL'S ONLY NAME. `aria-describedby` is a description,
//    and assistive technology may read it late, read it after the name, or skip
//    it on a reader's own setting. A control named by its tooltip is a control
//    with no name for anybody who has done that.
// 2. IT HOLDS NOTHING INTERACTIVE. The bubble is not a tab stop and closes when
//    the pointer leaves, so a link inside it is a link that cannot be reached.
//
// Both are asserted over every tooltip on the page rather than over one, because
// the failure is a caller's rather than the component's.
//
// The rest is the three ways in — hover with a wait, focus with none, a held
// touch — and Escape, which hides the bubble without moving the focus.

import { expect, test } from "@playwright/test";

import { PAGE, url, showParts } from "./pages.js";

const named = (page, name) => page.locator(`[data-nd-tooltip="${name}"]`);

async function tooltips(page) {
  await page.goto(url(PAGE.tooltip));
  await showParts(page);
}

test("with the script blocked the control is whole and the bubble is not there", async ({
  browser,
}) => {
  const context = await browser.newContext({ javaScriptEnabled: false });
  const page = await context.newPage();
  await page.goto(url(PAGE.tooltip));
  await showParts(page);
  // The control is the caller's own, so it is untouched. The hint is not.
  await expect(named(page, "tooltip-copy").locator("button")).toBeVisible();
  await expect(named(page, "tooltip-copy").locator(".nd-tooltip-bubble")).toBeHidden();
  await context.close();
});

test("it describes the control and never names it", async ({ page }) => {
  await tooltips(page);
  const wrong = await page.locator("[data-nd-tooltip]").evaluateAll((els) =>
    els
      .map((el) => {
        const bubble = el.querySelector(".nd-tooltip-bubble");
        const control = el.querySelector("a[href], button, input, select, textarea, [tabindex]");
        if (!bubble || !control) return `${el.dataset.ndTooltip}: no control`;
        if (control.getAttribute("aria-describedby") !== bubble.id) {
          return `${el.dataset.ndTooltip}: not described by its own bubble`;
        }
        // The two ways a hint could become the name, and both are refused.
        if ((control.getAttribute("aria-labelledby") ?? "").includes(bubble.id)) {
          return `${el.dataset.ndTooltip}: the hint is the accessible name`;
        }
        const own = control.getAttribute("aria-label") ?? control.textContent.trim();
        if (!own) return `${el.dataset.ndTooltip}: the control has no name of its own`;
        return null;
      })
      .filter(Boolean),
  );
  expect(wrong, "a tooltip is doing a name's job").toEqual([]);
});

test("nothing inside a bubble can be reached", async ({ page }) => {
  await tooltips(page);
  const holding = await page.locator(".nd-tooltip-bubble").evaluateAll((els) =>
    els
      .filter(
        (el) => el.querySelectorAll("a[href], button, input, select, textarea, [tabindex]").length,
      )
      .map((el) => el.id),
  );
  // A control in a bubble that closes when the pointer leaves is a control
  // nothing can press. `Popover` is the component for a surface with controls.
  expect(holding, "a tooltip is holding something interactive").toEqual([]);

  // And the bubble itself is not in anybody's way.
  const events = await page
    .locator(".nd-tooltip-bubble")
    .first()
    .evaluate((el) => getComputedStyle(el).pointerEvents);
  expect(events).toBe("none");
});

test("a pointer opens it, and it is a tooltip", async ({ page }) => {
  await tooltips(page);
  const root = named(page, "tooltip-copy");
  const bubble = root.locator(".nd-tooltip-bubble");
  expect(await bubble.getAttribute("role")).toBe("tooltip");

  await root.locator("button").hover();
  await expect(bubble).toBeVisible();
  expect(await root.getAttribute("data-nd-shown")).toBe("");

  await page.locator("h1").hover();
  await expect(bubble).toBeHidden();
});

test("focus opens it with no wait at all", async ({ page }) => {
  await tooltips(page);
  // `long` is the slowest wait there is, so a bubble that is already visible
  // the moment focus lands is a bubble that did not wait.
  const root = named(page, "tooltip-export");
  await root.locator("button").focus();
  await expect(root.locator(".nd-tooltip-bubble")).toBeVisible();

  await page.locator("h1").click();
  await expect(root.locator(".nd-tooltip-bubble")).toBeHidden();
});

test("a held touch opens it and lifting the finger takes it away", async ({ page }) => {
  await tooltips(page);
  const root = named(page, "tooltip-copy");
  const bubble = root.locator(".nd-tooltip-bubble");
  const box = await root.locator("button").boundingBox();
  const at = { x: box.x + box.width / 2, y: box.y + box.height / 2 };

  // A screen with no hover gets nothing from `pointerenter`, so this is the
  // path that serves it.
  const touch = await root.locator("button").evaluate((el, at) => {
    const send = (kind) =>
      el.dispatchEvent(
        new PointerEvent(kind, {
          bubbles: true,
          pointerType: "touch",
          clientX: at.x,
          clientY: at.y,
        }),
      );
    send("pointerdown");
    return true;
  }, at);
  expect(touch).toBe(true);
  await expect(bubble).toBeVisible();

  await root.locator("button").evaluate((el) => {
    el.dispatchEvent(new PointerEvent("pointerup", { bubbles: true, pointerType: "touch" }));
  });
  await expect(bubble).toBeHidden();
});

test("Escape hides it and leaves the focus on the control", async ({ page }) => {
  await tooltips(page);
  const root = named(page, "tooltip-refresh");
  await root.locator("button").focus();
  await expect(root.locator(".nd-tooltip-bubble")).toBeVisible();

  await page.keyboard.press("Escape");
  await expect(root.locator(".nd-tooltip-bubble")).toBeHidden();
  // The reader was getting rid of the bubble, not of the control.
  const still = await page.evaluate(
    () => document.activeElement === document.querySelector('[data-nd-tooltip="tooltip-refresh"] button'),
  );
  expect(still, "Escape moved the focus off the control").toBe(true);
});

test("the bubble is anchored to its control and carries no shadow", async ({ page }) => {
  await tooltips(page);
  const root = named(page, "tooltip-copy");
  await root.locator("button").hover();
  // The wait `delay` names has to pass before there is a box to measure: a
  // hidden bubble answers `getBoundingClientRect` with zeroes.
  await expect(root.locator(".nd-tooltip-bubble")).toBeVisible();
  const shape = await root.evaluate((el) => {
    const control = el.querySelector("button").getBoundingClientRect();
    const bubble = el.querySelector(".nd-tooltip-bubble");
    const box = bubble.getBoundingClientRect();
    return {
      bottom: control.bottom,
      top: box.top,
      shadow: getComputedStyle(bubble).boxShadow,
      position: getComputedStyle(bubble).position,
    };
  });
  expect(shape.position).toBe("fixed");
  expect(Math.abs(shape.top - shape.bottom)).toBeLessThan(40);
  expect(shape.shadow, "a tooltip is drawing an elevated surface").toBe("none");
});
