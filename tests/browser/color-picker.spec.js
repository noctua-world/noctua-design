// The colour picker, and the claims only a browser can settle.
//
// Its behaviour SHIPS — `dist/color-picker.js`, like `Lineage`'s — so what is
// asserted here is asserted about a file a consumer of the packages receives,
// not about this site's shell. The site is only where it is rendered.

import { expect, test } from "@playwright/test";

import { LOCALES, PAGE, url } from "./pages.js";

// EVERY COLOUR LITERAL IN THIS FILE IS IN THE BLOCK BELOW, AND NOWHERE ELSE.
//
// `NOCTUA.md §8.8` names tests explicitly and offers the marker as the way
// through; scattering ten markers through a file is how a hatch stops being an
// exception. These are the values the page is rendered with and the notations
// a reader types into it — test data for a component whose value is the
// consumer's, never a colour this system defines.

/** What the plain picker is rendered with, and what its rows must therefore say. */
const RENDERED = "#7c3aed"; // allow-literal: the `value` prop of the first example, asserted end to end
const RENDERED_RGB = "rgb(124 58 237)"; // allow-literal: the same colour, in the row beside it

/** What a reader types into the entry, and what must come back. */
const TYPED_RGB = "rgb(14 165 233)"; // allow-literal: a notation pasted into the box
const TYPED_RGB_HEX = "#0ea5e9"; // allow-literal: the same colour, as the hex row must then read
const TYPED_OKLCH = "oklch(0.65 0.13 145)"; // allow-literal: a third notation, to prove all four are read

/** What must be refused, and must change nothing. */
const INJECTION = "red; background: url(https://example.test/x)"; // allow-literal: a colour with CSS after it
const PROPERTY_INJECTION = "#0ea5e9; --nd-color-picker-pure: url(https://example.test/x)"; // allow-literal: the same trick aimed at the custom property the panel paints from

/** What is put into the store by hand, to be read back with no trust at all. */
const STORED = ["#0ea5e9", "#22c55e"]; // allow-literal: two colours a reader might have chosen
const STORED_DIRTY = ["#0ea5e9", "url(https://example.test/x)", { nope: 1 }, "#22c55e"]; // allow-literal: the same, with two entries that are not colours

/** The plain picker, the one with alpha, the list, the Noctua tab, the form. */
const PICKERS = "[data-nd-color-picker]";
const PLAIN = `${PICKERS}[data-nd-color-picker="label-colour"]`;
const EXTRAS = `${PICKERS}[data-nd-color-picker="brand-colour"]`;
const MANY = `${PICKERS}[data-nd-color-picker="tag-colours"]`;
const NOCTUA = `${PICKERS}[data-nd-color-picker="noctua-colour"]`;

/** Opens one picker's panel and hands back the panel. */
async function open(page, picker) {
  const root = page.locator(picker);
  await root.locator("[data-nd-color-picker-trigger]").click();
  const panel = root.locator("[data-nd-color-picker-panel]");
  await expect(panel).toBeVisible();
  return panel;
}

// --- What ships, and what the script adds -----------------------------------

test("with the script blocked it is a colour input that posts a value", async ({ browser }) => {
  const context = await browser.newContext({ javaScriptEnabled: false });
  const page = await context.newPage();
  await page.goto(url(PAGE.colorPicker));

  // THE HALF THAT NEEDS NO SCRIPT. A real control, with the platform's own
  // picker behind it, under the real field name.
  const native = page.locator(`${PLAIN} input[data-nd-color-picker-native]`);
  await expect(native).toBeVisible();
  expect(await native.getAttribute("type")).toBe("color");
  expect(await native.getAttribute("name")).toBe("label-colour");
  expect(await native.inputValue()).toBe(RENDERED);

  // And nothing claims what it cannot keep: the square that opens a panel is
  // not there at all, and neither is the panel.
  await expect(page.locator(`${PLAIN} [data-nd-color-picker-trigger]`)).toBeHidden();
  await expect(page.locator(`${PLAIN} [data-nd-color-picker-panel]`)).toBeHidden();

  await context.close();
});

test("the script hides the native control before it shows the square", async ({ page }) => {
  await page.goto(url(PAGE.colorPicker));
  // Never two controls for one colour. The order is what makes that true, and
  // this is the assertion that the order held.
  await expect(page.locator(`${PLAIN} [data-nd-color-picker-values]`)).toBeHidden();
  await expect(page.locator(`${PLAIN} [data-nd-color-picker-trigger]`)).toBeVisible();
  expect(
    await page.locator(`${PLAIN} [data-nd-color-picker-trigger]`).getAttribute("aria-expanded"),
  ).toBe("false");
});

test("the panel opens under the square and closes on Escape", async ({ page }) => {
  await page.goto(url(PAGE.colorPicker));
  const trigger = page.locator(`${PLAIN} [data-nd-color-picker-trigger]`);
  const panel = await open(page, PLAIN);
  expect(await trigger.getAttribute("aria-expanded")).toBe("true");
  // Anchored rather than positioned in CSS — the panel is `fixed`, so this is
  // the only thing that could have put it under the control it belongs to.
  const control = await trigger.boundingBox();
  const box = await panel.boundingBox();
  expect(box.y).toBeGreaterThan(control.y);

  await page.keyboard.press("Escape");
  await expect(panel).toBeHidden();
  // AND FOCUS COMES BACK. A reader who closed a panel with the keyboard is
  // otherwise left at the top of the document with nothing focused.
  expect(await trigger.evaluate((el) => el === document.activeElement)).toBe(true);
});

test("a click outside closes it, and the colour survives", async ({ page }) => {
  await page.goto(url(PAGE.colorPicker));
  const panel = await open(page, PLAIN);
  const before = await page.locator(`${PLAIN} [data-nd-color-picker-entry]`).inputValue();
  await page.locator("h1").click();
  await expect(panel).toBeHidden();
  await open(page, PLAIN);
  // Escape and a click outside abandon the PANEL, never the value: a picker
  // that reverted would throw away the drag a reader had just finished.
  expect(await page.locator(`${PLAIN} [data-nd-color-picker-entry]`).inputValue()).toBe(before);
});

// --- The field, and what announces it ---------------------------------------

test("the saturation field is two real sliders that announce a colour", async ({ page }) => {
  await page.goto(url(PAGE.colorPicker));
  await open(page, PLAIN);
  const axes = page.locator(`${PLAIN} [data-nd-color-picker-axis]`);
  await expect(axes).toHaveCount(2);
  for (const axis of await axes.all()) {
    // Real `<input type="range">`, which is where the role, the keyboard and
    // the value all come from — a `<div role="slider">` would have to
    // re-implement every one of them.
    expect(await axis.getAttribute("type")).toBe("range");
    // THE COLOUR, NOT THE COORDINATE. `42` is a number a reader cannot act on.
    expect(await axis.getAttribute("aria-valuetext")).toMatch(/^#[0-9a-f]{6}$/);
  }
  const hue = page.locator(`${PLAIN} [data-nd-color-picker-hue]`);
  expect(await hue.getAttribute("type")).toBe("range");
  expect(await hue.getAttribute("aria-label")).toBeTruthy();
});

test("an arrow moves the colour, and Shift moves it ten times as far", async ({ page }) => {
  await page.goto(url(PAGE.colorPicker));
  await open(page, PLAIN);
  const hue = page.locator(`${PLAIN} [data-nd-color-picker-hue]`);
  await hue.focus();
  const start = Number(await hue.inputValue());

  await page.keyboard.press("ArrowRight");
  expect(Number(await hue.inputValue())).toBe(start + 1);

  // Sixteen million colours behind a one-step arrow is a control a keyboard
  // cannot cross, and the platform offers no modifier for a coarse step.
  await page.keyboard.press("Shift+ArrowRight");
  expect(Number(await hue.inputValue())).toBe(start + 11);
});

// --- Transparency, and the control that may not be there --------------------

test("alpha reaches every notation, and the native input as well", async ({ page }) => {
  await page.goto(url(PAGE.colorPicker));
  await open(page, EXTRAS);
  // The platform's own attribute, so a browser that knows it gives a reader
  // transparency with no script at all — and one that does not gives an opaque
  // colour, which is smaller rather than broken.
  expect(
    await page.locator(`${EXTRAS} input[type=color]`).first().evaluate((el) => el.hasAttribute("alpha")),
  ).toBe(true);

  await page.locator(`${EXTRAS} details summary`).click();
  const track = page.locator(`${EXTRAS} [data-nd-color-picker-alpha]`);
  await track.focus();
  for (let step = 0; step < 20; step += 1) await page.keyboard.press("ArrowLeft");

  const rows = await page.locator(`${EXTRAS} [data-nd-color-picker-codes] li code`).allTextContents();
  // Eight digits in the hex, and a slash in the other three — the forms CSS
  // gives a colour that is not opaque.
  expect(rows[0]).toMatch(/^#[0-9a-f]{8}$/);
  for (const row of rows.slice(1)) expect(row).toContain("/");
});

test("the eyedropper is absent where the browser has no EyeDropper", async ({ page }) => {
  await page.goto(url(PAGE.colorPicker));
  await open(page, EXTRAS);
  // A CONTROL THAT DOES NOTHING IS WORSE THAN ONE THAT WAS NEVER OFFERED, and
  // the API is not everywhere. Whichever this browser is, the button and the
  // capability agree — which is the claim, rather than either alone.
  const supported = await page.evaluate(() => "EyeDropper" in window);
  const drawn = await page.locator(`${EXTRAS} [data-nd-action="color-picker-eyedropper"]`).count();
  expect(drawn > 0).toBe(supported);
});

// --- The codes, which is what the panel is for ------------------------------

test("the codes are folded away, and open to one row per notation", async ({ page }) => {
  await page.goto(url(PAGE.colorPicker));
  await open(page, PLAIN);
  const codes = page.locator(`${PLAIN} details`);
  // Closed until asked: four rows of text under the square would put the
  // colour off the bottom of a telephone.
  expect(await codes.getAttribute("open")).toBeNull();
  await codes.locator("summary").click();

  const rows = page.locator(`${PLAIN} [data-nd-color-picker-codes] li`);
  await expect(rows).toHaveCount(4);
  const text = (await rows.allTextContents()).join("|");
  expect(text).toContain(RENDERED);
  expect(text).toContain(RENDERED_RGB);
  // The other two by their row names, which the copy controls also carry.
  expect(await rows.nth(2).locator(".nd-color-picker-code-name").textContent()).toBe("HSL");
  expect(await rows.nth(3).locator(".nd-color-picker-code-name").textContent()).toBe("OKLCH");
});

test("each copy control names its own notation", async ({ page }) => {
  await page.goto(url(PAGE.colorPicker));
  await open(page, PLAIN);
  await page.locator(`${PLAIN} details summary`).click();
  const labels = await page
    .locator(`${PLAIN} [data-nd-color-copy]`)
    .evaluateAll((all) => all.map((one) => one.getAttribute("aria-label")));
  // Four identical *Copy* buttons in a column is four controls a screen-reader
  // user cannot tell apart.
  expect(new Set(labels).size).toBe(labels.length);
  expect(labels.join(" ")).toContain("HEX");
  expect(labels.join(" ")).toContain("OKLCH");
});

test("a notation reaches the clipboard, and says so once it is there", async ({
  page,
  context,
}) => {
  await context.grantPermissions(["clipboard-read", "clipboard-write"]);
  await page.goto(url(PAGE.colorPicker));
  await open(page, PLAIN);
  await page.locator(`${PLAIN} details summary`).click();

  const row = page.locator(`${PLAIN} [data-nd-color-picker-codes] li`).nth(1);
  const shown = (await row.locator("code").textContent()).trim();
  await row.locator("[data-nd-color-copy]").click();

  expect(await page.evaluate(() => navigator.clipboard.readText())).toBe(shown);
  // The two halves a reader gets: the glyph a sighted one sees, and the
  // sentence a screen reader hears. Neither is claimed when the copy fails.
  await expect(row.locator("[data-nd-color-copy][data-nd-done]")).toHaveCount(1);
  await expect(page.locator(`${PLAIN} [data-nd-color-picker-status]`)).toHaveText("Copied.");
});

// --- Typing one -------------------------------------------------------------

test("a typed colour is accepted in any notation", async ({ page }) => {
  await page.goto(url(PAGE.colorPicker));
  await open(page, PLAIN);
  const entry = page.locator(`${PLAIN} [data-nd-color-picker-entry]`);
  await page.locator(`${PLAIN} details summary`).click();
  const hex = page.locator(`${PLAIN} [data-nd-color-picker-codes] li`).first().locator("code");

  await entry.fill(TYPED_RGB);
  await entry.press("Enter");
  await expect(hex).toHaveText(TYPED_RGB_HEX);

  await entry.fill(TYPED_OKLCH);
  await entry.press("Enter");
  expect(await hex.textContent()).toMatch(/^#[0-9a-f]{6}$/);
});

test("a string that is not a colour is refused and the colour does not move", async ({ page }) => {
  await page.goto(url(PAGE.colorPicker));
  await open(page, PLAIN);
  await page.locator(`${PLAIN} details summary`).click();
  const hex = page.locator(`${PLAIN} [data-nd-color-picker-codes] li`).first().locator("code");
  const before = await hex.textContent();

  const entry = page.locator(`${PLAIN} [data-nd-color-picker-entry]`);
  // Not a colour, and — the part that matters — not something to guess at
  // either. A picker that reset itself on a typo throws away the thing the
  // reader was adjusting.
  await entry.fill(INJECTION);
  await entry.press("Enter");

  expect(await hex.textContent()).toBe(before);
  expect(await entry.getAttribute("aria-invalid")).toBe("true");
  await expect(page.locator(`${PLAIN} [data-nd-color-picker-status]`)).toContainText("not a colour");
});

test("nothing a reader types reaches the stylesheet", async ({ page }) => {
  await page.goto(url(PAGE.colorPicker));
  await open(page, PLAIN);
  const entry = page.locator(`${PLAIN} [data-nd-color-picker-entry]`);
  await entry.fill(PROPERTY_INJECTION);
  await entry.press("Enter");
  // Refused outright, so the custom property still holds what the script last
  // computed rather than anything that arrived as text.
  const written = await page
    .locator(`${PLAIN} [data-nd-color-picker-panel]`)
    .evaluate((el) => el.style.getPropertyValue("--nd-color-picker-pure"));
  expect(written).toMatch(/^#[0-9a-f]{6}$/);
});

// --- Several colours --------------------------------------------------------

test("the trigger shows one square per committed colour, and no spare", async ({ page }) => {
  await page.goto(url(PAGE.colorPicker));
  // THE DEFECT THIS CATCHES: the markup ships a spare `<input type="color">`
  // for the reader with no script, an empty colour input reports as opaque
  // black, and the script counted it — so a three-colour picker showed four,
  // one of them nobody had chosen.
  await expect(page.locator(`${MANY} .nd-color-picker-chips .nd-color-picker-chip`)).toHaveCount(3);
  await open(page, MANY);
  await expect(page.locator(`${MANY} [data-nd-color-picker-committed] li`)).toHaveCount(3);
  await expect(page.locator(`${MANY} [data-nd-color-picker-status]`)).toHaveText("3 colours");
});

test("adding and removing are announced, because neither moves focus", async ({ page }) => {
  await page.goto(url(PAGE.colorPicker));
  await open(page, MANY);
  const rows = page.locator(`${MANY} [data-nd-color-picker-committed] li`);
  const status = page.locator(`${MANY} [data-nd-color-picker-status]`);

  await page.locator(`${MANY} [data-nd-action="color-picker-add"]`).click();
  await expect(rows).toHaveCount(4);
  await expect(status).toContainText("added");

  await rows.first().locator('[data-nd-action="color-picker-remove"]').click();
  await expect(rows).toHaveCount(3);
  await expect(status).toContainText("removed");
});

test("at the limit it says why rather than doing nothing", async ({ page }) => {
  await page.goto(url(PAGE.colorPicker));
  await open(page, MANY);
  const add = page.locator(`${MANY} [data-nd-action="color-picker-add"]`);
  await add.click();
  await expect(page.locator(`${MANY} [data-nd-color-picker-committed] li`)).toHaveCount(4);
  await add.click();
  await expect(page.locator(`${MANY} [data-nd-color-picker-committed] li`)).toHaveCount(4);
  await expect(page.locator(`${MANY} [data-nd-color-picker-status]`)).toContainText("Four");
});

test("each remove control names the colour it removes", async ({ page }) => {
  await page.goto(url(PAGE.colorPicker));
  await open(page, MANY);
  const labels = await page
    .locator(`${MANY} [data-nd-action="color-picker-remove"]`)
    .evaluateAll((all) => all.map((one) => one.getAttribute("aria-label")));
  // A row of *Remove*, *Remove*, *Remove* is a row a screen-reader user cannot
  // navigate. `NOCTUA.md §8.8.6`: never by colour alone.
  expect(labels.every((label) => /#[0-9a-f]{6}/.test(label))).toBe(true);
  expect(new Set(labels).size).toBe(labels.length);
});

test("removing the last colour never drops focus to the body", async ({ page }) => {
  await page.goto(url(PAGE.colorPicker));
  await open(page, MANY);
  const rows = page.locator(`${MANY} [data-nd-color-picker-committed] li`);
  // Down to one, then take that one off: the control that did it has just left
  // the page, and focus falling to `<body>` puts a screen-reader user back at
  // the top of the document with no idea why.
  for (let left = 3; left > 0; left -= 1) {
    await rows.last().locator('[data-nd-action="color-picker-remove"]').click();
    await expect(rows).toHaveCount(left - 1);
    expect(await page.evaluate(() => document.activeElement?.tagName)).not.toBe("BODY");
  }
});

// --- The Noctua tab ---------------------------------------------------------

test("the Noctua tab shows the palette the page is in, with its tokens", async ({ page }) => {
  await page.goto(url(PAGE.colorPicker));
  await open(page, NOCTUA);
  await page.locator(`${NOCTUA} [data-nd-value="palette"]`).click();

  const swatches = page.locator(`${NOCTUA} [data-nd-color-picker-grid] .nd-color-picker-swatch`);
  await expect(swatches.first()).toBeVisible();
  expect(await swatches.count()).toBeGreaterThan(100);
  // ITS NAME IS ITS TOKEN, which is the whole reason this tab is worth having
  // and `§8.8.6` applied to a control made of nothing but colour.
  expect(await swatches.first().getAttribute("aria-label")).toMatch(/^--nc-/);
});

test("a step chosen there carries the one notation worth pasting", async ({ page }) => {
  await page.goto(url(PAGE.colorPicker));
  await open(page, NOCTUA);
  await page.locator(`${NOCTUA} [data-nd-value="palette"]`).click();
  const swatches = page.locator(`${NOCTUA} [data-nd-color-picker-grid] .nd-color-picker-swatch`);
  await expect(swatches.first()).toBeVisible();
  const token = await swatches.nth(20).getAttribute("aria-label");
  await swatches.nth(20).click();

  await page.locator(`${NOCTUA} details summary`).click();
  const rows = page.locator(`${NOCTUA} [data-nd-color-picker-codes] li`);
  await expect(rows).toHaveCount(3);
  expect((await rows.allTextContents()).join("|")).toContain(token);

  // And it goes away the moment the colour stops being that step: a token
  // beside a colour it no longer names is the worst kind of wrong.
  await page.locator(`${NOCTUA} [data-nd-value="spectrum"]`).click();
  await page.locator(`${NOCTUA} [data-nd-color-picker-hue]`).focus();
  await page.keyboard.press("ArrowRight");
  await expect(rows).toHaveCount(2);
});

// --- Remembering ------------------------------------------------------------

test("the recent row keeps what was chosen and forgets on request", async ({ page }) => {
  await page.goto(url(PAGE.colorPicker));
  await page.evaluate(() => localStorage.removeItem("noctua.docs.colours"));
  await page.reload();

  await open(page, EXTRAS);
  // Nothing remembered yet, so nothing is shown — a heading over an empty row
  // is a promise of something that is not there.
  await expect(page.locator(`${EXTRAS} [data-nd-color-picker-recent]`)).toBeHidden();

  // `multiple` is what commits a colour, so the memory is exercised on the
  // list rather than here; what this asserts is that the row appears from the
  // store and can be emptied.
  await page.evaluate(
    (stored) => localStorage.setItem("noctua.docs.colours", JSON.stringify(stored)),
    STORED,
  );
  await page.reload();
  await open(page, EXTRAS);
  await expect(page.locator(`${EXTRAS} [data-nd-color-picker-recent-row] li`)).toHaveCount(2);

  await page.locator(`${EXTRAS} [data-nd-action="color-picker-forget"]`).click();
  await expect(page.locator(`${EXTRAS} [data-nd-color-picker-recent]`)).toBeHidden();
});

test("anything in the store that is not a colour is dropped rather than trusted", async ({
  page,
}) => {
  await page.goto(url(PAGE.colorPicker));
  // A `localStorage` key is writable by any script on the origin, so what
  // comes back out of it is an input like any other.
  await page.evaluate(
    (stored) => localStorage.setItem("noctua.docs.colours", JSON.stringify(stored)),
    STORED_DIRTY,
  );
  await page.reload();
  await open(page, EXTRAS);
  await expect(page.locator(`${EXTRAS} [data-nd-color-picker-recent-row] li`)).toHaveCount(2);

  // And a store that is not even a list is not a reason to fail.
  await page.evaluate(() => localStorage.setItem("noctua.docs.colours", "{not json"));
  await page.reload();
  await open(page, EXTRAS);
  await expect(page.locator(`${EXTRAS} [data-nd-color-picker-recent]`)).toBeHidden();
});

// --- The shell it sits in ---------------------------------------------------

test("no id appears twice on the page, in any locale", async ({ page }) => {
  // Six pickers on one page, every id derived from `name`. The check that
  // makes `name` required worth requiring.
  for (const locale of LOCALES) {
    await page.goto(url(PAGE.colorPicker, locale));
    const twice = await page.evaluate(() => {
      const seen = new Map();
      for (const element of document.querySelectorAll("[id]")) {
        seen.set(element.id, (seen.get(element.id) ?? 0) + 1);
      }
      return [...seen].filter(([, count]) => count > 1).map(([id]) => id);
    });
    expect(twice, locale).toEqual([]);
  }
});

test("the panel does not make the page scroll sideways", async ({ page }) => {
  await page.setViewportSize({ width: 380, height: 720 });
  await page.goto(url(PAGE.colorPicker));
  await open(page, PLAIN);
  const overflow = await page.evaluate(
    () => document.documentElement.scrollWidth - document.documentElement.clientWidth,
  );
  expect(overflow).toBeLessThanOrEqual(0);
});

test("the field starts one gap under the row above it, not two", async ({ page }) => {
  // THE ROW AT THE TOP HELD ONE SMALL CONTROL AND RESERVED A BAND. It is
  // sticky, so it carries the panel's own block-start padding to cover the
  // inset while the palette scrolls under it — and it carried the same padding
  // at the BOTTOM, where there is nothing to cover and the panel's own `gap`
  // had already separated it. The two stacked: the saturation field opened
  // 69px down a 322px panel, most of that empty.
  //
  // Geometry rather than a declaration, because the padding was never wrong on
  // its own — it was wrong beside the gap.
  await page.goto(url(PAGE.colorPicker));
  const panel = await open(page, PLAIN);
  const space = await panel.evaluate((el) => {
    const head = el.querySelector(".nd-color-picker-head");
    const field = el.querySelector(".nd-color-picker-area");
    const control = head.querySelector(".nd-button");
    const gap = parseFloat(getComputedStyle(el).rowGap);
    return {
      control: control.getBoundingClientRect().height,
      headToField:
        field.getBoundingClientRect().top - head.getBoundingClientRect().bottom,
      topToField: field.getBoundingClientRect().top - el.getBoundingClientRect().top,
      gap,
    };
  });
  // Between the row and the field there is exactly the panel's own gap, and
  // never a second copy of it added by the row's own padding.
  expect(
    space.headToField,
    "the row under the close control is padded as well as gapped",
  ).toBeLessThanOrEqual(space.gap + 1);
  // And the whole band above the field is the panel's inset, the control, and
  // that one gap. A band larger than the sum is space nothing asked for.
  expect(space.topToField, "the panel reserves a band above its field").toBeLessThanOrEqual(
    space.control + space.gap * 2 + 2,
  );
});

/** The chip beside the error message, which is the one a reader meets refused. */
const REFUSED = `${PICKERS}[data-nd-color-picker="page-colour"]`;

test("the chip under an error takes the corner preset, square included", async ({ page }) => {
  // THE ONE SURFACE ON THE SITE THAT ANSWERED NOBODY. A reader who changes
  // *Corners* moves `--noctua-radius-control` and every control in the system
  // follows it — except this chip, which asked for `shape: square` in the
  // example's own call and therefore resolved `--noctua-radius-none` under all
  // four presets. Set the page to round corners and the trigger around it
  // became a capsule with a hard square inside, which is what was reported.
  //
  // FOUR PRESETS, NOT ONE, because `square` is itself a preset and the chip
  // has to follow that too. A chip that is square because somebody wrote
  // `shape: square` and a chip that is square because the reader asked for
  // square corners look identical, and only the sweep tells them apart.
  //
  // THE CORNER PIXEL, NOT THE RADIUS. `border-radius` is a length so reading
  // it would be allowed, but it would prove the declaration is present and
  // nothing about which rule won — and losing on specificity is exactly how
  // this got here. What is asked instead is whether the chip painted its own
  // top-left corner.
  await page.goto(url(PAGE.colorPicker));
  const chip = page.locator(`${REFUSED} .nd-color-picker-chip`).first();
  await chip.scrollIntoViewIfNeeded();
  const painted = async () => {
    const shot = await chip.screenshot();
    return page.evaluate(async (data) => {
      const blob = await (await fetch(`data:image/png;base64,${data}`)).blob();
      const bitmap = await createImageBitmap(blob);
      const canvas = new OffscreenCanvas(bitmap.width, bitmap.height);
      const ctx = canvas.getContext("2d");
      ctx.drawImage(bitmap, 0, 0);
      // One pixel in from the corner, and the middle of the same edge to
      // compare it with. A rounded box has cut the first away and kept the
      // second; a square one has both. The answer is how far apart they are
      // rather than whether they match: a corner a single pixel from a curve
      // is antialiased, and two colours one unit apart are the same colour.
      const at = (x, y) => [...ctx.getImageData(x, y, 1, 1).data];
      const corner = at(1, 1);
      const edge = at(Math.floor(bitmap.width / 2), 1);
      return Math.hypot(...corner.map((value, band) => value - edge[band]));
    }, shot.toString("base64"));
  };

  for (const preset of ["square", "soft", "round", "circular"]) {
    await page.evaluate(
      (value) => document.documentElement.setAttribute("data-noctua-radius", value),
      preset,
    );
    const apart = await painted();
    if (preset === "square") {
      expect(apart, "the square preset cut a corner off the chip").toBeLessThan(16);
    } else {
      expect(apart, `the chip kept its corner under the ${preset} preset`).toBeGreaterThan(60);
    }
  }
});
