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
import { contrast, extremes } from "./pixels.js";

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
  const surfaces = await page.locator(".nd-dialog-surface").evaluateAll((els) => ({
    count: els.length,
    open: els.filter((el) => el.open || el.checkVisibility()).length,
  }));
  // THE FLOOR, AND IT MATTERS MOST HERE. This is the only guard for the bug
  // above, and a count of nothing has nothing open in it. A renamed surface
  // class read as every dialog being shut. The page ships five of them, and
  // three are named by the tests below.
  expect(surfaces.count, "the page drew no dialog to check").toBeGreaterThan(2);
  expect(surfaces.open, "a closed dialog is being drawn over the page").toBe(0);
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
  // POLLED, NOT READ ONCE. The browser hides the surface and dispatches
  // `close` on a later task, and the state is taken off in that listener — so
  // a single read between the two saw `data-nd-open=""` on a dialog that had
  // shut. It failed about one run in three, which is the worst rate there is.
  await expect(root).not.toHaveAttribute("data-nd-open");
  // Restored by `close`, which is the other half of what `showModal` buys.
  await expect
    .poll(() => page.evaluate(() => document.activeElement.className))
    .toContain("nd-dialog-open");
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

test("a non-modal one still answers Escape after Tab has walked out of it", async ({ page }) => {
  // THE HOLE THIS CLOSES. `keydown` was bound to the SURFACE, and `dialog.toml`
  // says in as many words that Tab may walk out of a non-modal dialog and leave
  // it open. The first place Tab lands on the way out is the trigger — outside
  // the surface, inside the root — and from there Escape reached nothing at
  // all, so the one key the contract offers a keyboard reader did nothing.
  // `popover.js` binds on the root and has never had this hole.
  await dialogs(page);
  const root = named(page, "dialog-notes");
  const surface = root.locator(".nd-dialog-surface");
  await root.locator(".nd-dialog-open").click();
  await expect(surface).toBeVisible();

  // Focus moved deliberately out of the surface and onto the trigger, which is
  // exactly where the reader's own Tab takes them.
  await root.locator(".nd-dialog-open").focus();
  const outside = await page.evaluate(() => {
    const dialog = document.querySelector('[data-nd-dialog="dialog-notes"] .nd-dialog-surface');
    return !dialog.contains(document.activeElement);
  });
  expect(outside, "the focus never left the surface, so this proves nothing").toBe(true);

  await page.keyboard.press("Escape");
  await expect(surface, "Escape did nothing once the focus had left the surface").toBeHidden();
});

test("Escape on a shut dialog is left for whatever else wants it", async ({ page }) => {
  // The other half: the key is answered only while the surface is open, or a
  // closed dialog would swallow the Escape meant for something around it.
  await dialogs(page);
  const root = named(page, "dialog-notes");
  await expect(root.locator(".nd-dialog-surface")).toBeHidden();
  const swallowed = await root.evaluate((el) => {
    let reached = false;
    const listen = () => {
      reached = true;
    };
    document.addEventListener("keydown", listen);
    el.querySelector(".nd-dialog-open").dispatchEvent(
      new KeyboardEvent("keydown", { key: "Escape", bubbles: true }),
    );
    document.removeEventListener("keydown", listen);
    return !reached;
  });
  expect(swallowed, "a shut dialog stopped an Escape that was not its own").toBe(false);
});

// --- What the layer behind it is for ----------------------------------------

/**
 * How readable the page is, from the pixels of a strip beside the dialog.
 *
 * The extremes of a strip of the page are its ink and its ground, so the
 * contrast between them is how legible whatever is written there still is. The
 * strip is taken clear of the surface itself, because the question is about
 * what the veil did to the page and not about the dialog drawn over it.
 */
async function legibility(page, strip) {
  const shot = await page.screenshot({ clip: strip });
  const { min, max } = await extremes(page, shot);
  return contrast(min, max);
}

for (const scheme of ["light", "dark"]) {
  test(`the page behind a modal one stops being readable in ${scheme}`, async ({ page }) => {
    // THE VEIL WAS A MARK WEARING A SCRIM'S HAT. `Dialog` painted its backdrop
    // from `--nd-veil-checked` — the fourteen percent that marks which value
    // is in force in a group — over the INK, whose colour inverts with the
    // mode. In light that was a wash nobody had to read around; in dark it was
    // a white haze at the same fourteen percent, which is fainter still.
    //
    // Measured from the pixels of the page beside the dialog, before and
    // after, and in both appearances — because a veil that works in one is
    // routinely invisible in the other.
    await page.emulateMedia({ colorScheme: scheme });
    await dialogs(page);
    const surface = named(page, "dialog-remove").locator(".nd-dialog-surface");
    await named(page, "dialog-remove").locator(".nd-dialog-open").click();
    await expect(surface).toBeVisible();

    // ONE STRIP, MEASURED TWICE. It is named while the surface is open, so it
    // is the same pixels either way — the page beside the dialog, clear of the
    // dialog itself, at the height the dialog covers.
    const box = await surface.boundingBox();
    const strip = {
      x: 8,
      y: box.y,
      width: Math.max(Math.round(box.x) - 24, 24),
      height: Math.round(box.height),
    };
    const veiled = await legibility(page, strip);
    await page.keyboard.press("Escape");
    await expect(surface).toBeHidden();
    const plain = await legibility(page, strip);

    // The page was readable a moment ago, so the strip has something in it.
    expect(plain, "the page beside the dialog carries no text to hide").toBeGreaterThan(4);
    // And behind an open dialog it is not. 3:1 is the floor for large text, so
    // a page under this is a page nothing on it can be read from.
    expect(veiled, `the page behind the dialog is still readable in ${scheme}`).toBeLessThan(3);
  });
}
