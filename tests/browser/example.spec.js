// The example frame, and the claim only a browser can settle.
//
// `NOCTUA.md §9.3` says the Dioxus and React implementations must be
// behaviourally identical. Generating both from one table makes their prop
// surfaces identical by construction — but "the same props" is not "the same
// behaviour", and the only way to show the second is to mount both and compare
// what they put in the DOM.

import { test, expect } from "@playwright/test";
import { ANY, LOCALES, PAGE, url } from "./pages.js";

// The frame lives on the scoped-themes page now, which is where `ThemeScope`
// is documented and where the React module has always mounted it. It went away
// for one commit with the single "Components" page it used to sit on; these
// five were `fixme` rather than deleted in the meantime, because they were the
// only written record of what it has to do.


const requests = (page) =>
  page.evaluate(() =>
    performance.getEntriesByType("resource").filter((r) => r.name.includes("example-react")).length,
  );

test("the React bundle is not fetched on a normal page view", async ({ page }) => {
  await page.goto("/en/scoped-themes/");
  await page.waitForLoadState("networkidle");
  // 190 KB that most readers never need. A lazy path that loads eagerly is not
  // a lazy path; it is a lazy path with a bug nobody notices.
  expect(await requests(page)).toBe(0);
});

test("switching to React fetches it once and mounts the same theme", async ({ page }) => {
  await page.goto("/en/scoped-themes/");
  const dioxus = page.locator("[data-nd-frame-stage] > [data-noctua-scope]");
  const expected = {
    palette: await dioxus.getAttribute("data-palette"),
    theme: await dioxus.getAttribute("data-theme"),
  };

  await page.locator(".nd-code .nd-disclosure-summary").click();
  await page.locator('[data-nd-setting="frame-impl"] [data-nd-value="React"]').click();
  await expect(page.locator("[data-nd-react-slot] [data-noctua-scope]")).toBeVisible();

  expect(await requests(page)).toBe(1);

  const react = page.locator("[data-nd-react-slot] [data-noctua-scope]");
  // Byte-identical attributes from two independent implementations. This is
  // prop-surface parity demonstrated rather than asserted.
  expect(await react.getAttribute("data-palette")).toBe(expected.palette);
  expect(await react.getAttribute("data-theme")).toBe(expected.theme);
  await expect(dioxus).toBeHidden();

  // And switching again does not fetch it a second time.
  await page.locator('[data-nd-setting="frame-impl"] [data-nd-value="Dioxus"]').click();
  await page.locator('[data-nd-setting="frame-impl"] [data-nd-value="React"]').click();
  expect(await requests(page)).toBe(1);
});

test("switching back unmounts React rather than hiding it", async ({ page }) => {
  await page.goto("/en/scoped-themes/");
  await page.locator(".nd-code .nd-disclosure-summary").click();
  await page.locator('[data-nd-setting="frame-impl"] [data-nd-value="React"]').click();
  await expect(page.locator("[data-nd-react-slot] [data-noctua-scope]")).toBeVisible();

  await page.locator('[data-nd-setting="frame-impl"] [data-nd-value="Dioxus"]').click();
  // A live React root behind a hidden div keeps its state and its effects, and
  // the next switch would show a component that had been running all along.
  await expect(page.locator("[data-nd-react-slot] [data-noctua-scope]")).toHaveCount(0);
  await expect(page.locator("[data-nd-frame-stage] > [data-noctua-scope]")).toBeVisible();
});

test("the code panel shows the implementation that is mounted", async ({ page }) => {
  await page.goto("/en/scoped-themes/");
  await page.locator(".nd-code .nd-disclosure-summary").click();
  await expect(page.locator('[data-nd-impl-source="Dioxus"]')).toBeVisible();

  await page.locator('[data-nd-setting="frame-impl"] [data-nd-value="React"]').click();
  await expect(page.locator('[data-nd-impl-source="React"]')).toBeVisible();
  await expect(page.locator('[data-nd-impl-source="Dioxus"]')).toBeHidden();
});

test("the width control resizes the stage and nothing else", async ({ page }) => {
  await page.goto("/en/scoped-themes/");
  const stage = page.locator("[data-nd-frame-stage]");
  const full = (await stage.boundingBox()).width;

  await page.locator('[data-nd-setting="frame-width"] [data-nd-value="narrow"]').click();
  await page.waitForTimeout(250);
  const narrow = (await stage.boundingBox()).width;
  expect(narrow).toBeLessThan(full);

  // The page around it did not move.
  const main = await page.locator(".nd-main").boundingBox();
  expect(main.width).toBeGreaterThan(narrow);

  await page.locator("[data-nd-frame-reset]").click();
  await page.waitForTimeout(250);
  expect((await stage.boundingBox()).width).toBeCloseTo(full, 0);
});

test("search finds the system's own names, and only when asked", async ({ page }) => {
  await page.goto(url(ANY));
  const names = page.locator(".nd-found-entry:visible");
  // Thirty-nine palettes under an empty search box would bury the navigation.
  expect(await names.count()).toBe(0);

  await page.locator("[data-nd-search]").fill("space-gap");
  await expect(names).toHaveCount(1);
  await expect(names.first()).toHaveText("--noctua-space-gap");

  await page.locator("[data-nd-search]").fill("jade-vivid");
  await expect(names.first()).toHaveText("jade-vivid");

  await page.locator("[data-nd-search]").fill("zzzz");
  await expect(page.locator("[data-nd-search-empty]")).toBeVisible();
});

test("every locale ships the same page and switches between them", async ({ page }) => {
  await page.goto("/en/scoped-themes/");
  await page.locator(".nd-locale").click();
  await expect(page).toHaveURL(/\/pt-BR\/scoped-themes\/$/);
  expect(await page.locator("html").getAttribute("lang")).toBe("pt-BR");

  // Foundations first, translated and in the reader's alphabet; then every
  // component, alphabetically, with nothing between them.
  const order = await page.locator(".nd-nav-text").allTextContents();
  expect(order[0]).toBe("Cores");
  expect(order).toContain("Tipografia");
  // The property behind the collation: `Efeitos` before `Espaçamento` folds the
  // `ç`, and the English order (`Effects`, `Spacing`) puts them nowhere near
  // each other.
  expect(order.indexOf("Efeitos")).toBeLessThan(order.indexOf("Espaçamento"));

  // And the components are a block of their own, in order, after them.
  const components = order.slice(order.indexOf("Abbr"));
  expect(components[0]).toBe("Abbr");
  expect(components.at(-1)).toBe("VisuallyHidden");
  expect([...components].sort()).toEqual(components);
});
