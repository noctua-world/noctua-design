// The platform's `<dialog>`, and the two halves of it this system writes.
//
// WHAT IS BEING GUARDED. Almost everything a modal dialog does is the browser's
// — the focus trap, the inert background, the focus restoration, Escape and the
// backdrop — and the value of using `<dialog>` at all is that none of it is
// reimplemented here. So these tests assert the platform's behaviour is reached
// rather than replaced, and then assert the two things that ARE written: the
// trigger the script reveals, and the Escape a non-modal surface is owed.
//
// The `[open]` scoping is asserted too, because it is the bug this system has
// already shipped once: a closed `<dialog>` is `display: none` by the
// user-agent stylesheet, and one layout property on the class un-hides every
// closed dialog in the document.

import { expect, test } from "@playwright/test";

import { PAGE, url, showParts } from "./pages.js";

const named = (page, name) => page.locator(`[data-nd-dialog="${name}"]`);

async function dialogs(page) {
  await page.goto(url(PAGE.dialog));
  await showParts(page);
}

test("with the script blocked there is no control that opens nothing", async ({ browser }) => {
  const context = await browser.newContext({ javaScriptEnabled: false });
  const page = await context.newPage();
  await page.goto(url(PAGE.dialog));
  await showParts(page);
  const trigger = named(page, "dialog-remove").locator(".nd-dialog-open");
  await expect(trigger).toBeAttached();
  await expect(trigger).toBeHidden();
  await context.close();
});

test("every dialog on the page ships shut", async ({ page }) => {
  await dialogs(page);
  // THE BUG THIS EXISTS FOR. `display: flex` on the class rather than on
  // `[open]` un-hides every closed dialog in the document, and each one then
  // draws its box over the page with nothing else anywhere saying so.
  const open = await page
    .locator(".nd-dialog-surface")
    .evaluateAll((els) => els.filter((el) => el.open || el.checkVisibility()).length);
  expect(open, "a closed dialog is being drawn over the page").toBe(0);
});

test("the script reveals the trigger, and its box was there before it", async ({ page }) => {
  await dialogs(page);
  const trigger = named(page, "dialog-remove").locator(".nd-dialog-open");
  await expect(trigger).toBeVisible();
  expect(await trigger.evaluate((el) => el.tagName)).toBe("BUTTON");
  expect(await trigger.getAttribute("type")).toBe("button");
  expect(await trigger.getAttribute("aria-haspopup")).toBe("dialog");

  // 24px is the AA floor for a target.
  const box = await trigger.boundingBox();
  expect(box.height).toBeGreaterThanOrEqual(24);

  // AND THE SPACE WAS RESERVED BEFORE THE SCRIPT RAN. The container holds a
  // minimum height under `[data-nd-scripting]`, which the inline head script
  // stamps before the first paint — so unhiding the button moves nothing.
  const held = await named(page, "dialog-remove").evaluate(
    (el) => Number.parseFloat(getComputedStyle(el).minBlockSize) > 0,
  );
  expect(held, "the trigger arrives into a box nothing was holding").toBe(true);
});

test("a modal one takes the focus, marks the page inert and gives the focus back", async ({
  page,
}) => {
  await dialogs(page);
  const root = named(page, "dialog-remove");
  const surface = root.locator(".nd-dialog-surface");

  await root.locator(".nd-dialog-open").click();
  await expect(surface).toBeVisible();
  expect(await root.getAttribute("data-nd-open")).toBe("");

  // The platform's own doing, and the reason none of it is written by hand:
  // the surface is in the top layer and the document behind it is inert.
  const inside = await page.evaluate(() =>
    document
      .querySelector('[data-nd-dialog="dialog-remove"] .nd-dialog-surface')
      .contains(document.activeElement),
  );
  expect(inside, "focus is outside a modal surface").toBe(true);

  await page.keyboard.press("Escape");
  await expect(surface).toBeHidden();
  expect(await root.getAttribute("data-nd-open")).toBeNull();
  // Restored by `close`, which is the other half of what `showModal` buys.
  expect(await page.evaluate(() => document.activeElement.className)).toContain("nd-dialog-open");
});

test("a non-modal one leaves the page reachable and still answers Escape", async ({ page }) => {
  await dialogs(page);
  const root = named(page, "dialog-notes");
  const surface = root.locator(".nd-dialog-surface");
  await root.locator(".nd-dialog-open").click();
  await expect(surface).toBeVisible();

  // `show()` puts nothing in the top layer, so the page behind it is ordinary.
  const modal = await surface.evaluate((el) => el.matches(":modal"));
  expect(modal, "a `modal: false` dialog reached the top layer").toBe(false);

  // AND THE PLATFORM ANSWERS NOTHING HERE. `cancel` fires for a modal surface
  // only, so this key is `dist/dialog.js` doing the half that was left out.
  await surface.locator(".nd-dialog-close").focus();
  await page.keyboard.press("Escape");
  await expect(surface).toBeHidden();
  // And focus is not left on `<body>`, where the next Tab restarts the document.
  const landed = await page.evaluate(() => document.activeElement !== document.body);
  expect(landed, "focus fell to the body when a non-modal dialog closed").toBe(true);
});

test("the close control has a name, and a caller may refuse one", async ({ page }) => {
  await dialogs(page);
  const close = named(page, "dialog-remove").locator(".nd-dialog-close");
  await named(page, "dialog-remove").locator(".nd-dialog-open").click();
  await expect(close).toBeVisible();
  expect(await close.getAttribute("aria-label")).toMatch(/\S\s\S/);
  const box = await close.boundingBox();
  expect(box.width).toBeGreaterThanOrEqual(24);
  expect(box.height).toBeGreaterThanOrEqual(24);
  await page.keyboard.press("Escape");

  // `dismissible: false` draws none. Escape and the backdrop are still the way
  // out, and both of those are the platform's.
  await expect(named(page, "dialog-terms").locator(".nd-dialog-close")).toHaveCount(0);
});

test("it is named, and the visible title is what names it", async ({ page }) => {
  await dialogs(page);
  const shapes = await page.locator(".nd-dialog-surface").evaluateAll((els) =>
    els.map((el) => ({
      labelledby: el.getAttribute("aria-labelledby"),
      label: el.getAttribute("aria-label"),
      titled: Boolean(el.querySelector(".nd-dialog-title")),
      resolves: el.getAttribute("aria-labelledby")
        ? Boolean(document.getElementById(el.getAttribute("aria-labelledby")))
        : true,
    })),
  );
  expect(shapes.length).toBeGreaterThan(0);
  for (const shape of shapes) {
    expect(shape.labelledby || shape.label, "a dialog with no accessible name").toBeTruthy();
    // Never both: a name written twice can drift from itself.
    expect(shape.labelledby && shape.label).toBeFalsy();
    expect(shape.resolves, "aria-labelledby points at nothing").toBe(true);
    if (shape.titled) expect(shape.labelledby).toBeTruthy();
  }
});

test("the surface clips and the body scrolls, and only one of them is round", async ({ page }) => {
  await dialogs(page);
  await page.evaluate(() => document.documentElement.setAttribute("data-noctua-radius", "circular"));
  await named(page, "dialog-terms").locator(".nd-dialog-open").click();

  const shape = await named(page, "dialog-terms").evaluate((el) => {
    const surface = getComputedStyle(el.querySelector(".nd-dialog-surface"));
    const body = getComputedStyle(el.querySelector(".nd-dialog-body"));
    return {
      clips: surface.overflow,
      radius: surface.borderRadius,
      scrolls: body.overflowY,
      bodyRadius: body.borderRadius,
      shadow: surface.boxShadow,
    };
  });
  // A scrollable element never carries a radius, and a radius element never
  // scrolls. `.nd-panel` is two elements for this reason and so is this.
  expect(shape.clips).toBe("hidden");
  expect(shape.scrolls).toBe("auto");
  expect(shape.bodyRadius).toBe("0px");
  // And separation is a hairline plus a surface colour, never elevation.
  expect(shape.shadow, "a dialog is drawing an elevated surface").toBe("none");
});
