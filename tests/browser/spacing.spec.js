// The spacing page, measured against the tokens it documents.
//
// THE HOLE THIS FILLS.
//
// Every bar on this page has its width written as `var(--noctua-space-{step})`,
// composed at render time — and `gates::unknown_token` says in its own comment
// that it cannot check those: "the value is composed at runtime, so there is
// nothing static to check." A typo would produce a declaration the browser
// drops at computed-value time with no error, and the bar would be zero pixels
// wide on a page whose whole subject is width.
//
// So the check is here instead, and it is the strong version: every bar is
// measured and compared against the property it claims to be.

import { test, expect } from "@playwright/test";
import { PAGE, url } from "./pages.js";

/** A custom property's value, in pixels, as the page itself resolves it. */
const resolve = (page, name) =>
  page.evaluate((property) => {
    const root = document.documentElement;
    const value = getComputedStyle(root).getPropertyValue(property).trim();
    const rem = Number.parseFloat(getComputedStyle(root).fontSize);
    if (value.endsWith("rem")) return Number.parseFloat(value) * rem;
    return Number.parseFloat(value);
  }, name);

test("every bar is the token it is labelled with", async ({ page }) => {
  await page.setViewportSize({ width: 1440, height: 1000 });
  await page.goto(url(PAGE.spacing));

  const rows = await page.evaluate(() =>
    [...document.querySelectorAll(".nd-table-row")]
      .filter((row) => row.querySelector(".nd-bar"))
      .map((row) => ({
        name: row.querySelector("th code")?.textContent.trim(),
        drawn: row.querySelector(".nd-bar").getBoundingClientRect().width,
      })),
  );
  // Thirteen steps and seven named gaps, on two tables.
  expect(rows.length, "the page draws no bars").toBe(20);

  for (const row of rows) {
    expect(row.name, "a bar has no token beside it").toMatch(/^--noctua-space-/);
    const wanted = await resolve(page, row.name);
    expect(
      Math.abs(row.drawn - wanted),
      `${row.name} draws ${row.drawn}px and resolves to ${wanted}px`,
    ).toBeLessThan(1);
  }
});

test("the lengths the page prints are the lengths the tokens resolve to", async ({ page }) => {
  await page.goto(url(PAGE.spacing));
  // A column of numbers is the one thing on this page a reader will copy, so
  // it is the one thing that must not be arithmetic somebody did once.
  const printed = await page.evaluate(() =>
    [...document.querySelectorAll('[data-nd-table="spacing-scale"] tbody tr')].map((row) => ({
      name: row.querySelector("th code").textContent.trim(),
      // The `code` inside, not the cell: a cell also carries the column's name
      // for the stacked layout, so `textContent` reads `rem0rem`.
      inRem: row.children[1].querySelector("code").textContent.trim(),
      inPixels: row.children[2].querySelector("code").textContent.trim(),
    })),
  );
  expect(printed.length).toBe(13);

  const root = await page.evaluate(() =>
    Number.parseFloat(getComputedStyle(document.documentElement).fontSize),
  );
  for (const row of printed) {
    const resolved = await resolve(page, row.name);
    expect(Number.parseFloat(row.inRem) * root, `${row.name} prints ${row.inRem}`).toBeCloseTo(
      resolved,
      1,
    );
    expect(Number.parseFloat(row.inPixels), `${row.name} prints ${row.inPixels}`).toBeCloseTo(resolved, 1);
  }
});

test("each gap in the rhythm is its own token's height", async ({ page }) => {
  await page.goto(url(PAGE.spacing));
  const gaps = await page.evaluate(() =>
    [...document.querySelectorAll(".nd-rhythm-gap")].map((gap) => ({
      name: gap.querySelector(".nd-rhythm-gap-name").textContent.trim(),
      drawn: gap.getBoundingClientRect().height,
    })),
  );
  expect(gaps.length, "the rhythm draws no gaps").toBeGreaterThan(3);

  for (const gap of gaps) {
    const wanted = await resolve(page, gap.name);
    expect(Math.abs(gap.drawn - wanted), `${gap.name} is ${gap.drawn}px`).toBeLessThan(1);
  }
  // And they really are different from each other, or the demonstration
  // demonstrates nothing: the whole claim is that a heading's gap, a
  // paragraph's and a section's read as a hierarchy.
  expect(new Set(gaps.map((gap) => gap.drawn)).size).toBeGreaterThan(2);
});

test("every name the scale emits is searchable, and lands here", async ({ page }) => {
  await page.goto(url(PAGE.spacing));
  // `index.rs` says a token with no page is not listed, and that adding the
  // names back means writing the page first. This is the other half of that
  // sentence: the page exists, so the names are findable again.
  //
  // The mechanics changed and the claim did not. It used to fill a box in the
  // sidebar and read the visible one of 313 hidden anchors; it opens the
  // palette and reads the first RANKED result, because the anchors are gone and
  // the index is a file. `--noctua-space-heading` is also the query that found
  // nothing at all under the old matcher, which is why it is the one asked here.
  await page.locator('[data-nd-search="docs"] [data-nd-search-open]').click();
  await page.locator("#nd-search-docs").fill("--noctua-space-heading");
  const hit = page.locator("#nd-search-docs-results .nd-search-result:not([hidden])").first();
  await expect(hit).toBeVisible();
  await expect(hit.locator(".nd-search-result-label")).toHaveText("--noctua-space-heading");
  expect(await hit.locator("a").getAttribute("href")).toContain("spacing/");
});
