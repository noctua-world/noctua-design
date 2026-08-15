// Paging through a set, and the one claim that matters about it.
//
// It has a page of its own now. It never was part of the table family — paging
// is a question about the DATASET rather than about the table showing it, and a
// list of search results wants exactly the same control — and sharing `/tables/`
// meant a reader who clicked *Table* in the sidebar met the whole of
// `Pagination` first, because `pagination.toml` sorts before `table.toml`.

import { expect, test } from "@playwright/test";

import { PAGE, url } from "./pages.js";

test("pagination is links, and the ends are absent rather than dead", async ({ page }) => {
  await page.goto(url(PAGE.pagination));
  const nav = page.locator('[data-nd-pagination="palettes"]');
  await expect(nav).toBeVisible();
  expect(await nav.evaluate((el) => el.tagName)).toBe("NAV");
  expect(await nav.getAttribute("aria-label")).toBeTruthy();

  // A PAGE IS A PLACE. Every one of them is a real `<a href>`, so a reader can
  // open one in a new tab, and the browser's own history works.
  for (const link of await nav.locator("a").all()) {
    expect(await link.getAttribute("href")).toBeTruthy();
  }

  // Exactly one says it is the page in force, and it says so with the
  // attribute a screen reader reads rather than with a class.
  const current = nav.locator('[aria-current="page"]');
  await expect(current).toHaveCount(1);

  // The first page has no *previous* and the last has no *next*: a control that
  // goes nowhere is worse than no control, because a reader presses it before
  // learning that.
  const first = page.locator('[data-nd-pagination="palettes"]');
  expect(await first.getAttribute("data-nd-page")).toBe("1");
  await expect(first.locator('[data-nd-pagination-step="previous"]')).toHaveCount(0);

  const last = page.locator('[data-nd-pagination="palettes-last"]');
  expect(await last.getAttribute("data-nd-page")).toBe(await last.getAttribute("data-nd-pages"));
  await expect(last.locator('[data-nd-pagination-step="next"]')).toHaveCount(0);
});
