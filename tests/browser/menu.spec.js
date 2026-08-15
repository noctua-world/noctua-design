// The menu: the surface a button opens, and every key it owes.
//
// WHY THIS PAGE EXISTS AT ALL. `Button { menu: true }` shipped for months
// writing `aria-haspopup="menu"` and `aria-controls` over a hidden paragraph —
// announcing a menu to a screen reader, drawing a chevron, and doing nothing on
// every press, because the system had no menu for it to open. The developer
// found it by pressing the button. Every test below is a promise that markup
// was already making.

import { expect, test } from "@playwright/test";

import { PAGE, url, showParts } from "./pages.js";

/**
 * The first menu IN AN EXAMPLE, and the button that opens it.
 *
 * Scoped to the stages, because the example frame's own rail carries a menu of
 * its own on every page of the site — the five viewport widths — and it comes
 * first in the document. A spec about this component would otherwise have been
 * measuring the documentation's furniture.
 *
 * `[id]`, because the page also carries variant-grid SPECIMENS: pictures of a
 * menu, with no id, never hidden, and nothing to open them.
 */
const parts = (page) => ({
  button: page.locator('.nd-example-stage [aria-haspopup="menu"]').first(),
  menu: page.locator(".nd-example-stage .nd-menu[id]").first(),
});

test("with the script blocked there is a menu nobody is invited to open", async ({ browser }) => {
  // THE RULE THIS FAMILY IS BUILT ON. A marker that opens nothing is worse than
  // no marker, so the menu ships hidden — and the button, which is a `Button`
  // like any other, is left saying it opens something. The half that matters is
  // that nothing is VISIBLE and nothing is reachable: a reader with no script
  // sees a page with one dead control, not a page with a panel stuck open.
  const context = await browser.newContext({ javaScriptEnabled: false });
  const page = await context.newPage();
  await page.goto(`http://127.0.0.1:8731${url(PAGE.menu)}`);
  await showParts(page);
  await expect(page.locator(".nd-menu[id]").first()).toBeHidden();
  await context.close();
});

test("it opens on a press and closes on the next one", async ({ page }) => {
  await page.goto(url(PAGE.menu));
  await showParts(page);
  const { button, menu } = parts(page);
  await expect(menu).toBeHidden();
  await expect(button).toHaveAttribute("aria-expanded", "false");
  await button.click();
  await expect(menu).toBeVisible();
  await expect(button).toHaveAttribute("aria-expanded", "true");
  await button.click();
  await expect(menu).toBeHidden();
  await expect(button).toHaveAttribute("aria-expanded", "false");
});

test("the arrows walk the items, and the menu is one tab stop", async ({ page }) => {
  await page.goto(url(PAGE.menu));
  await showParts(page);
  const { button, menu } = parts(page);
  // DOWN OPENS ONTO THE FIRST ITEM. It is the one keyboard convention a menu
  // button has that an ordinary button does not.
  await button.focus();
  await page.keyboard.press("ArrowDown");
  const items = menu.locator('[role="menuitem"]');
  await expect(items.first()).toBeFocused();

  await page.keyboard.press("ArrowDown");
  await expect(items.nth(1)).toBeFocused();
  await page.keyboard.press("End");
  await expect(items.last()).toBeFocused();
  // Wrapping is what a menu does and a listbox does not.
  await page.keyboard.press("ArrowDown");
  await expect(items.first()).toBeFocused();

  // ONE TAB STOP: exactly one item is reachable by Tab, and it is wherever the
  // focus was left. A Tab per item is a menu a keyboard reader walks THROUGH.
  const reachable = await menu.evaluate(
    (root) => [...root.querySelectorAll('[role="menuitem"]')].filter((i) => i.tabIndex === 0).length,
  );
  expect(reachable, "the roving tabindex is not roving").toBe(1);
});

test("a letter jumps, and a repeated letter spells a word", async ({ page }) => {
  await page.goto(url(PAGE.menu));
  await showParts(page);
  const { button, menu } = parts(page);
  await button.click();
  const items = menu.locator('[role="menuitem"]');
  const names = (await items.allTextContents()).map((one) => one.trim().toLowerCase());
  const first = names[0][0];
  await page.keyboard.press("ArrowDown");
  await page.keyboard.press(first);
  const landed = await menu.evaluate(
    (root) => document.activeElement?.textContent?.trim().toLowerCase() ?? "",
  );
  expect(landed.startsWith(first), `typing ${first} landed on ${landed}`).toBe(true);
});

test("Escape closes it and puts the focus back where it came from", async ({ page }) => {
  await page.goto(url(PAGE.menu));
  await showParts(page);
  const { button, menu } = parts(page);
  await button.focus();
  await page.keyboard.press("ArrowDown");
  await expect(menu).toBeVisible();
  await page.keyboard.press("Escape");
  await expect(menu).toBeHidden();
  // A reader who dismissed a menu and found themselves at the top of the
  // document was moved by the thing they were getting rid of.
  await expect(button).toBeFocused();
});

test("choosing an item closes the menu and returns the focus", async ({ page }) => {
  await page.goto(url(PAGE.menu));
  await showParts(page);
  const { button, menu } = parts(page);
  await button.click();
  await menu.locator('[role="menuitem"]').first().click();
  await expect(menu).toBeHidden();
  await expect(button).toBeFocused();
});

test("a refused item stays in the walk and does nothing", async ({ page }) => {
  await page.goto(url(PAGE.menu));
  await showParts(page);
  // THE ONE PLACE THIS FAMILY DISAGREES WITH `Button`. A disabled `<button>`
  // leaves the tab order, and in a menu that means the arrows skip it — an item
  // that is skipped is an action a reader concludes does not exist.
  const menu = page.locator('.nd-menu:has([aria-disabled="true"])').first();
  const button = page.locator(`[aria-controls="${await menu.getAttribute("id")}"]`);
  await button.click();
  const refused = menu.locator('[aria-disabled="true"]').first();
  await refused.focus();
  await expect(refused).toBeFocused();
  // `force`, because Playwright reads `aria-disabled` as unactionable — which
  // is the whole point of the attribute, and not a reason to skip the check
  // that pressing it anyway does nothing.
  await refused.click({ force: true });
  await expect(menu, "a refused item closed the menu").toBeVisible();
});

test("a click outside closes it, and the focus is left alone", async ({ page }) => {
  await page.goto(url(PAGE.menu));
  await showParts(page);
  const { button, menu } = parts(page);
  await button.click();
  await expect(menu).toBeVisible();
  await page.locator("h1").click();
  await expect(menu).toBeHidden();
  // Not returned: the reader moved the focus themselves by clicking, and moving
  // it back would be taking their place from them.
  await expect(button).not.toBeFocused();
});

test("only one menu is open at a time", async ({ page }) => {
  await page.goto(url(PAGE.menu));
  await showParts(page);
  const buttons = page.locator('[aria-haspopup="menu"]');
  await buttons.nth(0).click();
  await buttons.nth(1).click();
  const open = await page.evaluate(
    () => [...document.querySelectorAll(".nd-menu[id]")].filter((one) => !one.hidden).length,
  );
  expect(open, "two menus were open over each other").toBe(1);
});

test("the button on the Button page opens the menu it names", async ({ page }) => {
  // THE DEFECT THE DEVELOPER REPORTED, in its own place. The example lived on
  // `/button/` and the component that could answer it did not exist.
  await page.goto(url(PAGE.button));
  await showParts(page);
  const button = page.locator('[aria-controls="example-button-actions"]');
  const menu = page.locator("#example-button-actions");
  await expect(menu).toBeHidden();
  await button.click();
  await expect(menu).toBeVisible();
  await expect(menu.locator('[role="menuitem"]')).toHaveCount(3);
});

test("no id appears twice on the menu page, in any locale", async ({ page }) => {
  for (const locale of ["en", "pt-BR"]) {
    await page.goto(url(PAGE.menu, locale));
    await showParts(page);
    const twins = await page.evaluate(() => {
      const seen = new Set();
      const bad = [];
      for (const element of document.querySelectorAll("[id]")) {
        if (seen.has(element.id)) bad.push(element.id);
        seen.add(element.id);
      }
      return bad;
    });
    expect(twins, `duplicate ids in ${locale}`).toEqual([]);
  }
});
