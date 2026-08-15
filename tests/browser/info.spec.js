// The `i`, and the two things a hover bubble cannot do.
//
// Its behaviour SHIPS — `dist/info.js` — and it is the sixth caller of
// `anchor.js` rather than a sixth copy of it.

import { expect, test } from "@playwright/test";

import { PAGE, url, showParts } from "./pages.js";

const named = (page, name) => page.locator(`[data-nd-info="${name}"]`);

test("with the script blocked there is no marker and no note", async ({ browser }) => {
  const context = await browser.newContext({ javaScriptEnabled: false });
  const page = await context.newPage();
  await page.goto(url(PAGE.info));
  await showParts(page);
  // A marker that opens nothing is worse than no marker.
  await expect(named(page, "deadline").locator(".nd-info-marker")).toBeHidden();
  await expect(named(page, "deadline").locator(".nd-info-note")).toBeHidden();
  await context.close();
});

test("it is a real button that says what it opens and whether it is open", async ({ page }) => {
  await page.goto(url(PAGE.info));
  await showParts(page);
  const marker = named(page, "deadline").locator(".nd-info-marker");
  await expect(marker).toBeVisible();
  expect(await marker.evaluate((el) => el.tagName)).toBe("BUTTON");
  expect(await marker.getAttribute("type")).toBe("button");
  // Not *Info*: a table of eight of those is eight controls a screen reader
  // reads identically.
  expect(await marker.getAttribute("aria-label")).toMatch(/\S\s\S/);
  expect(await marker.getAttribute("aria-expanded")).toBe("false");

  const note = named(page, "deadline").locator(".nd-info-note");
  expect(await marker.getAttribute("aria-controls")).toBe(await note.getAttribute("id"));
  expect(await note.getAttribute("role")).toBe("note");
});

test("it opens on a press and not on a hover", async ({ page }) => {
  await page.goto(url(PAGE.info));
  await showParts(page);
  const info = named(page, "deadline");
  const note = info.locator(".nd-info-note");

  // A HOVER OPENS NOTHING. That is the whole reason this is a button: a bubble
  // a pointer reveals does not exist on a telephone.
  await info.locator(".nd-info-marker").hover();
  await expect(note).toBeHidden();

  await info.locator(".nd-info-marker").click();
  await expect(note).toBeVisible();
  expect(await info.locator(".nd-info-marker").getAttribute("aria-expanded")).toBe("true");
});

test("Escape closes it and puts the focus back", async ({ page }) => {
  await page.goto(url(PAGE.info));
  await showParts(page);
  const info = named(page, "closable");
  const marker = info.locator(".nd-info-marker");
  await marker.click();
  await expect(info.locator(".nd-info-note")).toBeVisible();

  await page.keyboard.press("Escape");
  await expect(info.locator(".nd-info-note")).toBeHidden();
  // A reader who dismissed a note and lost their place was moved by the thing
  // they were getting rid of.
  expect(await page.evaluate(() => document.activeElement.className)).toContain("nd-info-marker");
});

test("a click outside closes it, and opening a second closes the first", async ({ page }) => {
  await page.goto(url(PAGE.info));
  await showParts(page);
  const first = named(page, "deadline");
  const second = named(page, "usage");

  await first.locator(".nd-info-marker").click();
  await expect(first.locator(".nd-info-note")).toBeVisible();

  await second.locator(".nd-info-marker").click();
  await expect(second.locator(".nd-info-note")).toBeVisible();
  // Two notes open at once is two notes over one page.
  await expect(first.locator(".nd-info-note")).toBeHidden();

  await page.locator("h1").click();
  await expect(second.locator(".nd-info-note")).toBeHidden();
});

test("the note is anchored to its marker rather than to the page", async ({ page }) => {
  await page.goto(url(PAGE.info));
  await showParts(page);
  const info = named(page, "usage");
  await info.locator(".nd-info-marker").click();
  const boxes = await info.evaluate((el) => {
    const marker = el.querySelector(".nd-info-marker").getBoundingClientRect();
    const note = el.querySelector(".nd-info-note").getBoundingClientRect();
    return { marker: { x: marker.x, bottom: marker.bottom }, note: { x: note.x, y: note.y } };
  });
  // Below it, and near it — the numbers `anchor.js` wrote, not a guess.
  expect(boxes.note.y).toBeGreaterThan(boxes.marker.bottom - 1);
  expect(Math.abs(boxes.note.y - boxes.marker.bottom)).toBeLessThan(40);
});

test("a close control exists only where a caller named one", async ({ page }) => {
  await page.goto(url(PAGE.info));
  await showParts(page);
  await named(page, "closable").locator(".nd-info-marker").click();
  await expect(named(page, "closable").locator(".nd-info-close")).toBeVisible();

  await named(page, "deadline").locator(".nd-info-marker").click();
  // Drawn by the script, so it exists only where the words for it do.
  await expect(named(page, "deadline").locator(".nd-info-close")).toHaveCount(0);
});

test("auto reads the measurement and start ignores it", async ({ page }) => {
  // `start` WAS BYTE-IDENTICAL TO `auto`. Only an `="end"` rule existed and
  // nothing read the attribute, so two of the three values drew the same note
  // and a caller choosing between them was choosing nothing. `anchor.js`
  // writes the edge a measurement picks, and only `auto` reads it: a stated
  // edge is a decision, not a guess.
  await page.setViewportSize({ width: 1280, height: 800 });
  await page.goto(url(PAGE.info));
  await showParts(page);

  const root = named(page, "deadline");
  await root.locator(".nd-info-marker").click();
  const note = root.locator(".nd-info-note");
  await expect(note).toBeVisible();
  // The measurement is written, which it never was.
  expect(await note.getAttribute("data-nd-anchor")).toMatch(/^(start|end)$/);

  const at = (placement, anchor) =>
    root.evaluate(
      (el, [where, edge]) => {
        el.setAttribute("data-nd-placement", where);
        el.querySelector(".nd-info-note").setAttribute("data-nd-anchor", edge);
        return Math.round(el.querySelector(".nd-info-note").getBoundingClientRect().left);
      },
      [placement, anchor],
    );

  // Under `auto` the note hangs from whichever edge the measurement named.
  expect(await at("auto", "start")).not.toBe(await at("auto", "end"));
  // Under `start` it hangs from the leading edge whatever the measurement says.
  expect(await at("start", "start")).toBe(await at("start", "end"));
});
