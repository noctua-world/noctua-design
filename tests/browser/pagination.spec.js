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
  //
  // THE ATTRIBUTE THIS ASKED ABOUT HAS NEVER BEEN EMITTED. Both counts read
  // `[data-nd-pagination-step="previous"]`, and what the component writes is a
  // CLASS with a `rel`. Nothing matched either selector, on any page, in any
  // state, so both claims were true of a page drawing every control there is.
  const step = (nav, way) => nav.locator(`a.nd-pagination-step[rel="${way}"]`);

  // The floor, and the reason this is a real claim now. A page in the middle
  // draws both steps, so the selector finds something when there is something
  // to find.
  const middle = page.locator('[data-nd-pagination="palettes-compact"]');
  await expect(step(middle, "prev"), "the selector matches nothing at all").toHaveCount(1);
  await expect(step(middle, "next"), "the selector matches nothing at all").toHaveCount(1);

  // The last page, which is the end that can be proved. It was given both
  // words and drew one control: a *next* here is a link to a page that does
  // not exist. The first page of the set comes from an example that never asked
  // for a *previous*, so its absence there proves nothing.
  const last = page.locator('[data-nd-pagination="palettes-last"]');
  expect(await last.getAttribute("data-nd-page")).toBe(await last.getAttribute("data-nd-pages"));
  await expect(step(last, "prev"), "the last page cannot go back").toHaveCount(1);
  await expect(step(last, "next"), "the last page offers a page after it").toHaveCount(0);

  // And the rule over every control of the kind on the page, from each one's
  // own numbers rather than from a name written here.
  const steps = await page.locator("a.nd-pagination-step").evaluateAll((all) =>
    all.map((one) => {
      const nav = one.closest("[data-nd-pagination]");
      return {
        way: one.getAttribute("rel"),
        page: Number(nav.getAttribute("data-nd-page")),
        pages: Number(nav.getAttribute("data-nd-pages")),
        href: one.getAttribute("href"),
        // Absent rather than dead is the whole claim, and a link that says it
        // is disabled is the dead one wearing the live one's markup.
        off: one.getAttribute("aria-disabled"),
      };
    }),
  );
  expect(steps.length, "the page draws no steps at all").toBeGreaterThan(4);
  for (const one of steps) {
    expect(one.href, `a ${one.way} step goes nowhere`).toBeTruthy();
    expect(one.off, `a ${one.way} step is drawn dead`).toBeNull();
    if (one.way === "prev") expect(one.page, "a first page offers a page before it").toBeGreaterThan(1);
    if (one.way === "next") expect(one.page, "a last page offers a page after it").toBeLessThan(one.pages);
  }
});
