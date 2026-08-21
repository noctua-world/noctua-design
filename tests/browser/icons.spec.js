// The icons, and the class of bug they replaced.
//
// The header shipped a link to the repository whose icon was `<path d="">` — a
// 44×44 clickable void, on every page, in every locale. It compiled because the
// icon was asked for by name, as a string, against a `match` with a `_ => ""`
// arm: "an icon called repository" and "no icon at all" were the same value.
//
// Each drawing is a generated constant now, so that call does not compile, and
// in JavaScript it is a module whose path does not resolve. These are the
// assertions for what a type cannot catch: that a drawing reached the page, and
// that a control made of nothing but a drawing still has a name.

import { test, expect } from "@playwright/test";
import { modeControl } from "./pages.js";
import { ANY, LOCALES, PAGE, url } from "./pages.js";

test("no icon anywhere is empty", async ({ page }) => {
  for (const locale of LOCALES) {
    for (const name of Object.values(PAGE)) {
      await page.goto(url(name, locale));
      const empty = await page.evaluate(() =>
        [...document.querySelectorAll("svg")]
          .filter((svg) => svg.children.length === 0 || svg.innerHTML.includes('d=""'))
          .map((svg) => svg.getAttribute("data-nd-icon") ?? "(unnamed)"),
      );
      expect(empty, `${locale}/${name} renders an icon that draws nothing`).toEqual([]);
    }
  }
});

test("a control that is only an icon still has a name", async ({ page }) => {
  await page.goto(url(ANY));
  // Two ways to fail this: no name at all, or a name that is the empty string.
  // Both leave a screen reader announcing "button" and nothing else.
  const unnamed = await page.evaluate(() =>
    [...document.querySelectorAll("button, a, summary")]
      .filter((el) => {
        const onlyIcon =
          el.children.length === 1 &&
          el.children[0].tagName.toLowerCase() === "svg" &&
          el.textContent.trim() === "";
        if (!onlyIcon) return false;
        const label = el.getAttribute("aria-label") ?? "";
        return label.trim() === "" && !el.getAttribute("aria-labelledby");
      })
      .map((el) => el.outerHTML.slice(0, 120)),
  );
  expect(unnamed, "an icon-only control with no accessible name").toEqual([]);
});

test("the icon page lists the whole set, and lists it from the set", async ({ page }) => {
  await page.goto(url(PAGE.icons));
  const cells = page.locator(".nd-icon-cell");
  const count = await cells.count();
  expect(count, "the gallery is empty").toBeGreaterThan(0);

  // Every cell draws something and names it. A gallery that lists names with no
  // drawings is exactly as useless as the bug it documents.
  const names = await page.evaluate(() =>
    [...document.querySelectorAll(".nd-icon-cell")].map((cell) => ({
      name: cell.querySelector("code")?.textContent?.trim(),
      drawn: (cell.querySelector("svg")?.children.length ?? 0) > 0,
      attribute: cell.querySelector("svg")?.getAttribute("data-nd-icon"),
    })),
  );
  for (const cell of names) {
    expect(cell.drawn, `${cell.name} is listed but draws nothing`).toBe(true);
    expect(cell.attribute, `${cell.name} is labelled with a different icon`).toBe(cell.name);
  }

  // And the count in the prose comes from the same place the cells do.
  const stated = await page.locator(".nd-note").first().textContent();
  expect(stated).toContain(String(count));
});

test("the filter narrows the gallery, and hides rather than removes", async ({ page }) => {
  await page.goto(url(PAGE.icons));
  const cells = page.locator(".nd-icon-cell");
  const total = await cells.count();
  // A thousand-odd drawings with no way to narrow them is a gallery nobody
  // finds anything in, which is why the field exists at all.
  expect(total).toBeGreaterThan(1000);

  const field = page.locator('input[name="icon-filter"]');
  await field.fill("chevron");
  const shown = page.locator(".nd-icon-cell:not([hidden])");
  const narrowed = await shown.count();
  expect(narrowed).toBeGreaterThan(0);
  expect(narrowed).toBeLessThan(total);

  // HIDES, DOES NOT REMOVE. The count in the prose above is the size of the
  // set, and it stays true while a filter is on because the set is still there.
  expect(await cells.count(), "a filtered cell was removed from the document").toBe(total);
  for (const name of await shown.evaluateAll((els) =>
    els.map((el) => el.getAttribute("data-nd-icon-name")),
  )) {
    expect(name).toContain("chevron");
  }

  // A name nothing has says so, rather than leaving an empty grid that looks
  // like a page that failed to load.
  await field.fill("zzzz");
  await expect(page.locator(".nd-icon-empty")).toBeVisible();
  expect(await shown.count()).toBe(0);

  await field.fill("");
  await expect(page.locator(".nd-icon-empty")).toBeHidden();
  expect(await shown.count(), "clearing the filter did not bring the set back").toBe(total);
});

test("no page fetches the whole manifest", async ({ page }) => {
  // `dist/icons.json` is every drawing in the system, and it is for tooling —
  // an icon picker that wants the set fetches it once and caches it. A PAGE
  // that fetched it would be loading six hundred kilobytes of geometry to draw
  // the handful it shows, which is the cost this whole layer exists to avoid.
  const asked = [];
  page.on("request", (request) => {
    if (request.url().includes("icons.json")) asked.push(request.url());
  });
  for (const name of [PAGE.icons, ANY]) {
    await page.goto(url(name));
  }
  expect(asked, "a page requested the icon manifest").toEqual([]);
});

test("the repository link has a visible icon", async ({ page }) => {
  await page.goto(url(ANY));
  // The specific control that shipped invisible. Named, not found by position:
  // a test that asserted "the third header control" would have passed then too.
  const link = page.locator('.nd-header a[href*="github.com"]');
  await expect(link).toBeVisible();

  const drawn = await link.locator("svg").evaluate((svg) => ({
    children: svg.children.length,
    // A path is not enough — the one that shipped had a path with no data.
    ink: [...svg.querySelectorAll("path")].some((p) => (p.getAttribute("d") ?? "").length > 0),
    box: svg.getBoundingClientRect().width,
  }));
  expect(drawn.children).toBeGreaterThan(0);
  expect(drawn.ink, "the icon is a path with no data — the exact bug").toBe(true);
  expect(drawn.box).toBeGreaterThan(0);
});

test("the mode control is three radios, and choosing one always moves it", async ({ page }) => {
  await page.goto(url(ANY));
  // In the theme menu, which is the only place it is. The header carried a
  // second copy of this control until the menu was made one component; a reader
  // had two ways to change one setting and the header spent a third of its
  // 320px budget on the duplicate.
  const group = await modeControl(page);
  const options = group.locator('[role="radio"]');
  await expect(options).toHaveCount(3);

  // The whole reason this is three controls rather than one that cycles: on a
  // system already in dark, "dark" and "system" paint the page identically, so
  // a cycling button gives the reader no feedback at all. The indicator has to
  // be the feedback.
  for (const value of ["light", "dark", "system"]) {
    await group.locator(`[data-nd-value="${value}"]`).click();
    await expect(group.locator(`[data-nd-value="${value}"]`)).toHaveAttribute(
      "aria-checked",
      "true",
    );
    expect(
      await options.evaluateAll((els) =>
        els.filter((el) => el.getAttribute("aria-checked") === "true").length,
      ),
      "more than one mode is announced as chosen",
    ).toBe(1);
  }
});
