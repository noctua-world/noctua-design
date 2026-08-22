// The two letters, and the note they open.
//
// Its behaviour SHIPS — `dist/info.js`, which enhances this hook and `Info`'s
// from one table — so most of what is asserted here is asserted in
// `info.spec.js` for the same reasons.
//
// WHAT IS NOT THE SAME IS THE TRIGGER. `Info`'s is a drawing at a fixed size.
// This one is two letters out of the catalogue: two glyphs in English, two
// different ones in Portuguese and six in `qps`. Every geometry claim below is
// therefore made in all three, and the target has to survive the widest of them
// at the narrowest viewport.

import { expect, test } from "@playwright/test";

import { LOCALES, PAGE, url, showParts } from "./pages.js";
import { contrast, extremes } from "./pixels.js";

/** Every mark on the page that has a note to open. */
const opens = (page) => page.locator("[data-nd-ai-note]");

test("the letters come from the catalogue and not from the component", async ({ page }) => {
  const seen = [];
  for (const locale of LOCALES) {
    await page.goto(url(PAGE.aiLabel, locale));
    await showParts(page);
    seen.push(await page.locator(".nd-ai-label-mark").first().textContent());
  }
  const [en, pt, qps] = seen;
  // THREE DIFFERENT STRINGS. A component that wrote `AI` itself would give
  // three identical ones, and the pseudo-locale sweep would never see it: the
  // run would be there, in English, on every page in every language.
  expect(en).not.toBe(pt);
  expect(qps).toMatch(/^⟦.*⟧$/);
  expect(qps.length).toBeGreaterThan(en.length);
});

test("it is a real button that says whether it is open, and the letters are hidden", async ({
  page,
}) => {
  await page.goto(url(PAGE.aiLabel));
  await showParts(page);
  const root = opens(page).first();
  const control = root.locator(".nd-ai-label-control");
  expect(await control.evaluate((el) => el.tagName)).toBe("BUTTON");
  expect(await control.getAttribute("type")).toBe("button");
  expect(await control.getAttribute("aria-expanded")).toBe("false");
  // The letters are a picture of the phrase, and the phrase is what is read.
  expect(await root.locator(".nd-ai-label-mark").getAttribute("aria-hidden")).toBe("true");
  const named = await root.locator(".nd-visually-hidden").first().textContent();
  expect(named).toMatch(/\S\s\S/);
  const note = root.locator(".nd-ai-label-note");
  expect(await note.getAttribute("role")).toBe("note");
  await expect(note).toBeHidden();
});

test("a mark with no note is not a control and not a tab stop", async ({ page }) => {
  await page.goto(url(PAGE.aiLabel));
  await showParts(page);
  const quiet = page.locator(".nd-ai-label:not([data-nd-ai-note])").first();
  await expect(quiet).toBeVisible();
  expect(await quiet.locator(".nd-ai-label-control").count()).toBe(0);
  expect(await quiet.locator("button, a[href], [tabindex]").count()).toBe(0);
});

test("Enter and Space open it, from the keyboard alone", async ({ page }) => {
  await page.goto(url(PAGE.aiLabel));
  await showParts(page);
  const root = opens(page).first();
  const control = root.locator(".nd-ai-label-control");
  const note = root.locator(".nd-ai-label-note");

  for (const key of ["Enter", " "]) {
    // NOT `.click()`. A click passes on a `<div onclick>`, which is exactly
    // the thing this assertion exists to refuse.
    await control.focus();
    await page.keyboard.press(key);
    await expect(note).toBeVisible();
    expect(await control.getAttribute("aria-expanded")).toBe("true");
    await page.keyboard.press(key);
    await expect(note).toBeHidden();
  }
});

test("Escape closes it and puts the focus back on the mark", async ({ page }) => {
  await page.goto(url(PAGE.aiLabel));
  await showParts(page);
  // THE ONE WITH A CLOSE CONTROL, AND TAB IN FIRST. Pressing Escape while the
  // focus is still on the mark asserts nothing: the focus never left, so it is
  // where the assertion wants it whether or not anything put it there. Written
  // that way this passed with `marker.focus()` deleted.
  const root = page.locator("[data-nd-ai-note][data-nd-close-label]").first();
  const note = root.locator(".nd-ai-label-note");
  await root.locator(".nd-ai-label-control").focus();
  await page.keyboard.press("Enter");
  await expect(note).toBeVisible();
  await page.keyboard.press("Tab");
  expect(await note.evaluate((el) => el.contains(document.activeElement))).toBe(true);

  await page.keyboard.press("Escape");
  await expect(note).toBeHidden();
  // A reader who dismissed a note and lost their place was moved by the thing
  // they were getting rid of.
  expect(await page.evaluate(() => document.activeElement.className)).toContain(
    "nd-ai-label-control",
  );
});

test("Tab walks into the note and out of it, and leaving closes it", async ({ page }) => {
  await page.goto(url(PAGE.aiLabel));
  await showParts(page);
  // The one with a close control in it, so there is something to walk into.
  const root = page.locator("[data-nd-ai-note][data-nd-close-label]").first();
  const note = root.locator(".nd-ai-label-note");
  await root.locator(".nd-ai-label-control").focus();
  await page.keyboard.press("Enter");
  await expect(note).toBeVisible();

  await page.keyboard.press("Tab");
  expect(await note.evaluate((el) => el.contains(document.activeElement))).toBe(true);

  // Nothing is trapped, so this terminates. The ceiling is the assertion: a
  // trap would hang the walk rather than fail it.
  let presses = 0;
  while ((await note.isVisible()) && presses < 5) {
    await page.keyboard.press("Tab");
    presses += 1;
  }
  await expect(note).toBeHidden();
  expect(presses).toBeLessThan(5);
});

test("opening a second note closes the first, across both kinds", async ({ page }) => {
  await page.goto(url(PAGE.aiLabel));
  await showParts(page);
  const marks = opens(page);
  expect(await marks.count()).toBeGreaterThan(1);
  const first = marks.nth(0);
  const second = marks.nth(1);

  await first.locator(".nd-ai-label-control").click();
  await expect(first.locator(".nd-ai-label-note")).toBeVisible();
  await second.locator(".nd-ai-label-control").click();
  await expect(second.locator(".nd-ai-label-note")).toBeVisible();
  // ONE OPEN NOTE ON A PAGE, which is why `info.js` enhances both hooks from
  // one table rather than being copied into a second module with a second
  // `showing`.
  await expect(first.locator(".nd-ai-label-note")).toBeHidden();
});

test("with the script blocked the mark is still drawn and the note stays shut", async ({
  browser,
}) => {
  const context = await browser.newContext({ javaScriptEnabled: false });
  const page = await context.newPage();
  await page.goto(url(PAGE.aiLabel));
  await showParts(page);
  // UNLIKE `Info`'s MARKER. That one ships hidden because with no script it
  // opens nothing. This one says a value came from an assistant, which is true
  // either way, so the mark is drawn and the note is the extra.
  await expect(page.locator(".nd-ai-label-mark").first()).toBeVisible();
  await expect(page.locator(".nd-ai-label-note").first()).toBeHidden();
  await context.close();
});

test("the target clears the touch minimum at the narrowest viewport", async ({ browser }) => {
  // A TOUCH CONTEXT, which nothing else in this suite asks for — so the
  // `@media (pointer: coarse)` half of the stylesheet has never been measured.
  const context = await browser.newContext({
    hasTouch: true,
    isMobile: true,
    viewport: { width: 320, height: 720 },
  });
  const page = await context.newPage();
  for (const locale of LOCALES) {
    await page.goto(url(PAGE.aiLabel, locale));
    await showParts(page);
    const control = page.locator(".nd-ai-label-control").first();
    await control.scrollIntoViewIfNeeded();
    const box = await control.boundingBox();
    expect(box.height, `${locale}: the mark is ${box.height} tall`).toBeGreaterThanOrEqual(24);
    expect(box.width, `${locale}: the mark is ${box.width} wide`).toBeGreaterThanOrEqual(24);
  }
  await context.close();
});

test("the letters are readable against every variant the contract declares", async ({ page }) => {
  // READ FROM THE VARIANT GRID, NOT FROM A LIST HERE. The page draws one
  // specimen per declared value of every drawn enum prop, so the set swept
  // below IS the contract's own and stays in step with it. A list written here
  // would be a second copy of the same fact.
  //
  // AND FROM PIXELS. A computed style is a serialisation, and the tint is a
  // mix over whatever happens to be behind it — the one thing a computed style
  // can never report.
  await page.setViewportSize({ width: 1120, height: 900 });
  for (const mode of ["light", "dark"]) {
    await page.goto(url(PAGE.aiLabel));
    await showParts(page);
    await page.evaluate((m) => document.documentElement.setAttribute("data-theme", m), mode);

    // A SOLID FILL IS THE PALETTE'S OWN PAIR AND IS EXCLUDED HERE. `on-<family>`
    // over `<family>` measures 2.5:1 to 4.6:1 across the palette in the light
    // appearance, and that is a fact about the colour system rather than about
    // this component — every solid-filled thing in the catalogue sits on it.
    // Holding one component to a bar the palette cannot meet would only mean
    // this component may not be solid.
    const cells = page.locator(
      ".nd-ai-label:not([data-nd-variant='solid']) .nd-ai-label-mark",
    );
    const count = await cells.count();
    expect(count, `no variant specimen was measured in ${mode}`).toBeGreaterThan(2);

    const seen = new Set();
    for (let i = 0; i < count; i += 1) {
      const cell = cells.nth(i);
      await cell.scrollIntoViewIfNeeded();
      const shape = await cell.evaluate((el) => {
        const root = el.closest(".nd-ai-label");
        return `${root.getAttribute("data-nd-variant")}/${root.getAttribute("data-nd-tone") ?? "none"}`;
      });
      seen.add(shape);
      const { min, max } = await extremes(page, await cell.screenshot());
      expect(
        contrast(min, max),
        `${shape} in ${mode} ranged ${min} to ${max}`,
      ).toBeGreaterThanOrEqual(4.5);
    }
    // BOTH THE UNTONED AND THE TONED FORM. A sweep that only ever met the
    // resting state would pass a tinted mark painted in the tone's own ink,
    // which is the pair this component was measured away from.
    expect([...seen].some((shape) => shape.endsWith("/none")), `only toned marks in ${mode}`).toBe(
      true,
    );
    expect(
      [...seen].some((shape) => !shape.endsWith("/none")),
      `no toned mark was measured in ${mode}`,
    ).toBe(true);
  }
});
