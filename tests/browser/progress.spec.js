// The progress bar, and the one state that is easy to get wrong.
//
// An absent `value` is the indeterminate bar — a state rather than an omission.
// It is also the only animation in this system that is not the carousel's, so
// the reduced-motion claim is asserted here rather than assumed.

import { expect, test } from "@playwright/test";

import { PAGE, url } from "./pages.js";

const named = (page, name) => page.locator(`[data-nd-progress="${name}"]`);

test.describe("with the script blocked", () => {
  test("it is a real progress element with a real label", async ({ browser }) => {
    const context = await browser.newContext({ javaScriptEnabled: false });
    const page = await context.newPage();
    await page.goto(url(PAGE.progress));

    const bar = named(page, "upload").locator("progress");
    expect(await bar.evaluate((el) => el.tagName)).toBe("PROGRESS");
    expect(await bar.getAttribute("value")).toBe("7.4");
    const label = named(page, "upload").locator("label");
    expect(await label.getAttribute("for")).toBe(await bar.getAttribute("id"));
    await context.close();
  });
});

test("an absent value is the indeterminate bar, not a bar at zero", async ({ page }) => {
  await page.goto(url(PAGE.progress));
  const busy = named(page, "import");
  expect(await busy.getAttribute("data-nd-busy")).not.toBe(null);

  const bar = busy.locator("progress");
  // NO ATTRIBUTE AT ALL. An empty one is a bar at nothing, which looks broken.
  expect(await bar.getAttribute("value")).toBe(null);
  expect(await bar.evaluate((el) => el.position)).toBe(-1);
  // And it says what it is busy with rather than announcing a bare busy state.
  expect(await bar.getAttribute("aria-valuetext")).toMatch(/\S/);
  // No number is drawn, because there is none.
  await expect(busy.locator(".nd-progress-value")).toBeHidden();
});

test("the sweep moves, and stops when a reader asks for less movement", async ({ page }) => {
  await page.goto(url(PAGE.progress));
  const bar = named(page, "import").locator("progress");
  await bar.scrollIntoViewIfNeeded();

  const running = await bar.evaluate(
    (el) => getComputedStyle(el).animationName !== "none" && getComputedStyle(el).animationName !== "",
  );
  expect(running).toBe(true);

  // `appearance: none` takes the platform's own animation away, so this
  // promise is the stylesheet's to keep rather than the browser's.
  await page.emulateMedia({ reducedMotion: "reduce" });
  const stopped = await bar.evaluate((el) => getComputedStyle(el).animationName);
  expect(stopped).toBe("none");
  await page.emulateMedia({ reducedMotion: null });
});

test("what is read and what is heard are the same sentence", async ({ page }) => {
  await page.goto(url(PAGE.progress));
  for (const name of ["upload", "failed"]) {
    const shown = await named(page, name).locator(".nd-progress-value").innerText();
    const heard = await named(page, name).locator("progress").getAttribute("aria-valuetext");
    expect(heard).toBe(shown);
  }
});

test("size is a measurement, not a word", async ({ page }) => {
  await page.goto(url(PAGE.progress));
  const height = async (name) =>
    (await named(page, name).locator("progress").boundingBox()).height;
  const large = await height("steps-large");
  const small = await height("steps-small");
  // Two examples that differ only in a prop nobody can see is the defect this
  // page was written to avoid.
  expect(large).toBeGreaterThan(small * 2);
});
