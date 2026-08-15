// The error summary, and the claims only a browser can settle.
//
// Its behaviour SHIPS — `dist/error-summary.js` — so what is asserted here is
// asserted about a file a consumer of the packages receives.
//
// THE ONE COMPONENT ON THIS SITE THAT TAKES THE FOCUS WITHOUT BEING TOUCHED,
// which is why most of this file is about where focus ends up.

import { expect, test } from "@playwright/test";

import { LOCALES, PAGE, url, showParts } from "./pages.js";
import { extremes } from "./pixels.js";

const RENDERED = '[data-nd-error-summary="rendered"]';
const COLLECTED = '[data-nd-error-summary="collected"]';
const HIDDEN = '[data-nd-error-summary="grouped"]';
const PLAIN = '[data-nd-error-summary="plain"]';

/** The form the collecting summary watches. */
const FORM = "#nd-example-collect";

// --- What ships, and what the script adds ------------------------------------

test("with the script blocked the list is there and every entry works", async ({ browser }) => {
  const context = await browser.newContext({ javaScriptEnabled: false });
  const page = await context.newPage();
  await page.goto(url(PAGE.errorSummary));
  await showParts(page);

  // A server that refused a submission rendered this, and none of it needs us.
  const summary = page.locator(RENDERED);
  await expect(summary).toBeVisible();
  await expect(summary.locator("[data-nd-error-summary-link]")).toHaveCount(2);
  for (const href of await summary
    .locator("[data-nd-error-summary-link]")
    .evaluateAll((all) => all.map((one) => one.getAttribute("href")))) {
    expect(href.startsWith("#")).toBe(true);
    await expect(page.locator(href)).toHaveCount(1);
  }

  // And nothing claims what it cannot keep: the one that collects has nothing
  // to collect without a script, so it is not in the page at all.
  await expect(page.locator(COLLECTED)).toBeHidden();
  await context.close();
});

test("it is an alert in a box that can be focused but is not a tab stop", async ({ page }) => {
  await page.goto(url(PAGE.errorSummary));
  await showParts(page);
  const summary = page.locator(RENDERED);

  expect(await summary.getAttribute("tabindex")).toBe("-1");
  // The alert is INSIDE, and the element is in the document before it speaks —
  // a region inserted whole is a region most screen readers never see.
  await expect(summary.locator('[role="alert"]')).toHaveCount(1);

  // The words a reader hears are the words on the screen.
  const named = await summary.getAttribute("aria-labelledby");
  await expect(page.locator(`#${named}`)).toHaveText(/\S/);

  // IT IS NOT A LANDMARK. `role="region"` here would put *there is a problem*
  // in every landmark list on the page for the rest of the visit.
  expect(await summary.getAttribute("role")).toBe(null);
});

// --- Where the focus goes ----------------------------------------------------

test("following an entry puts focus on the control, not on the label above it", async ({
  page,
}) => {
  await page.goto(url(PAGE.errorSummary));
  await showParts(page);
  const entry = page.locator(`${RENDERED} [data-nd-error-summary-link]`).first();
  const wanted = (await entry.getAttribute("href")).slice(1);

  await entry.click();
  expect(await page.evaluate(() => document.activeElement.id)).toBe(wanted);
  // The address bar is left alone: the fragment would put the browser's own
  // scroll where ours already is, and back would then mean *un-follow*.
  expect(page.url()).not.toContain("#");
});

test("an entry reaching a control the script has hidden focuses the visible one", async ({
  page,
}) => {
  await page.goto(url(PAGE.errorSummary));
  await showParts(page);
  const entry = page.locator(`${HIDDEN} [data-nd-error-summary-link]`).first();
  const wanted = (await entry.getAttribute("href")).slice(1);

  // The `Select`'s real element, which its own script hides the moment it runs.
  await expect(page.locator(`#${wanted}`)).toBeHidden();

  await entry.click();
  const landed = await page.evaluate(() => ({
    id: document.activeElement.id,
    seen: document.activeElement.offsetParent !== null,
    inside: document.activeElement.closest(".nd-select") !== null,
  }));
  expect(landed.id).not.toBe(wanted);
  expect(landed.seen).toBe(true);
  expect(landed.inside).toBe(true);
});

test("an entry for a question asked as several boxes lands in the first of them", async ({
  page,
}) => {
  await page.goto(url(PAGE.errorSummary));
  await showParts(page);
  const entry = page.locator(`${HIDDEN} [data-nd-error-summary-link]`).nth(1);
  const wanted = (await entry.getAttribute("href")).slice(1);

  await entry.click();
  expect(await page.evaluate(() => document.activeElement.id)).toBe(wanted);
  expect(await page.evaluate(() => document.activeElement.tagName)).toBe("INPUT");
});

// --- The refusal the browser makes -------------------------------------------

test("a summary with nothing to say is not in the page", async ({ page }) => {
  await page.goto(url(PAGE.errorSummary));
  await showParts(page);
  // A box that says *there is a problem* above a form with no problems is a
  // box readers learn to ignore.
  await expect(page.locator(COLLECTED)).toBeHidden();
  expect(await page.locator(COLLECTED).getAttribute("data-nd-empty")).not.toBe(null);
  // And it did not take the focus on the way in.
  expect(await page.evaluate(() => document.activeElement.tagName)).toBe("BODY");
});

test("the browser's refusal fills the fields, the list and the document title", async ({ page }) => {
  await page.goto(url(PAGE.errorSummary));
  await showParts(page);
  const before = await page.title();

  await page.locator(`${FORM} button[type="submit"]`).click();

  const summary = page.locator(COLLECTED);
  await expect(summary).toBeVisible();
  await expect(summary.locator("[data-nd-error-summary-link]")).toHaveCount(2);

  // NOT ONE WORD OF LANGUAGE IS INVENTED. What the list says is what the
  // platform says, which is why it is in the reader's language without this
  // repository carrying a sentence of it.
  const said = await summary
    .locator("[data-nd-error-summary-link]")
    .evaluateAll((all) => all.map((one) => one.textContent.trim()));
  const platform = await page
    .locator(`${FORM} input`)
    .evaluateAll((all) => all.map((one) => one.validationMessage).filter(Boolean));
  expect(said).toEqual(platform);

  // The same sentence beside the answer, where a reader who has arrived needs
  // it, and the field marked wrong with it.
  await expect(page.locator(`${FORM} [data-nd-field-error]`).first()).toHaveText(platform[0]);
  expect(
    await page.locator(`${FORM} .nd-field`).first().getAttribute("data-nd-invalid"),
  ).toBe("true");

  // THE WHOLE POINT: the reader was at the bottom of the form, on the control
  // they pressed, and the reason is at the top.
  expect(
    await page.evaluate(() => document.activeElement.matches("[data-nd-error-summary]")),
  ).toBe(true);

  // And the tab says so, once.
  const after = await page.title();
  expect(after).not.toBe(before);
  expect(after).toContain(before);
});

test("every entry the browser's refusal wrote reaches the box it is about", async ({ page }) => {
  await page.goto(url(PAGE.errorSummary));
  await showParts(page);
  await page.locator(`${FORM} button[type="submit"]`).click();

  const entry = page.locator(`${COLLECTED} [data-nd-error-summary-link]`).last();
  const wanted = (await entry.getAttribute("href")).slice(1);
  await entry.click();
  expect(await page.evaluate(() => document.activeElement.id)).toBe(wanted);
});

test("answering takes the sentence back, and the title with it", async ({ page }) => {
  await page.goto(url(PAGE.errorSummary));
  await showParts(page);
  const before = await page.title();
  await page.locator(`${FORM} button[type="submit"]`).click();
  await expect(page.locator(COLLECTED)).toBeVisible();

  await page.locator(`${FORM} input`).first().fill("Ada Lovelace");
  await expect(page.locator(`${COLLECTED} [data-nd-error-summary-link]`)).toHaveCount(1);

  await page.locator(`${FORM} input`).nth(1).fill("ada@example.org");
  // Nothing left to say, so it stops saying it — in the box, in the field, and
  // in the tab.
  await expect(page.locator(COLLECTED)).toBeHidden();
  await expect(page.locator(`${FORM} [data-nd-field-error]`).first()).toHaveText("");
  expect(await page.title()).toBe(before);
});

test("the platform's bubble is refused, because the same sentence is already twice on the page", async ({
  page,
}) => {
  await page.goto(url(PAGE.errorSummary));
  await showParts(page);
  // A bubble anchors on the invalid control and closes the moment focus moves,
  // which is exactly what this component then does — so it is refused rather
  // than raced.
  const refused = await page.evaluate(() => {
    const box = document.querySelector("#nd-example-collect input");
    const event = new Event("invalid", { cancelable: true, bubbles: false });
    return !box.dispatchEvent(event);
  });
  expect(refused).toBe(true);
});

// --- What it looks like ------------------------------------------------------

test("the box is drawn in the danger tone, at the rule's own thickness", async ({ page }) => {
  await page.goto(url(PAGE.errorSummary));
  await showParts(page);
  const summary = page.locator(RENDERED);
  await summary.scrollIntoViewIfNeeded();

  // The token, read from the document rather than written here — a number in a
  // test is a second place the scale is decided.
  const rule = await page.evaluate(() =>
    getComputedStyle(document.documentElement).getPropertyValue("--noctua-border-rule").trim(),
  );
  const width = await summary.evaluate((el) => getComputedStyle(el).borderTopWidth);
  expect(width).toBe(rule);

  // AND IT IS ACTUALLY RED ON THE SCREEN. Read from pixels, because a computed
  // style is whatever the engine chose to serialise and says nothing about
  // what a reader sees.
  const box = await summary.boundingBox();
  const strip = await page.screenshot({
    clip: { x: box.x, y: box.y, width: box.width, height: Math.ceil(Number.parseFloat(rule)) },
  });
  // The darkest pixel in the strip: the rule itself, against a surface that
  // is lighter than it in every theme this site ships.
  const { min } = await extremes(page, strip);
  expect(min[0]).toBeGreaterThan(min[1] + 20);
  expect(min[0]).toBeGreaterThan(min[2] + 20);
});

test("it is not a list of nothing, in either locale", async ({ page }) => {
  for (const locale of LOCALES) {
    await page.goto(url(PAGE.errorSummary, locale));
    await showParts(page);
    // Every summary rendered on this page has entries, and the one that does
    // not is the one that is hidden.
    for (const summary of await page.locator("[data-nd-error-summary]").all()) {
      const entries = await summary.locator("[data-nd-error-summary-link]").count();
      const empty = (await summary.getAttribute("data-nd-empty")) !== null;
      expect(entries > 0).toBe(!empty);
    }
    await expect(page.locator(PLAIN)).toBeVisible();
  }
});
