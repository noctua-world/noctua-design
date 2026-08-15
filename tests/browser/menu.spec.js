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

test("with the script blocked neither the menu nor its button renders", async ({ browser }) => {
  // THE RULE THIS FAMILY IS BUILT ON, AND THE HALF THAT USED TO BE MISSING.
  //
  // `menu.toml` has always opened with "neither the menu nor its button renders
  // without a script". Only the menu was hidden. The button shipped visible and
  // focusable, carrying `aria-haspopup="menu"` and `aria-expanded="false"` —
  // announced to a screen reader as a menu button, over nothing that could
  // open. That is the exact defect this component was written to remove,
  // committed by the component itself, and the sentence at the top of
  // `menu.js` names it.
  const context = await browser.newContext({ javaScriptEnabled: false });
  const page = await context.newPage();
  await page.goto(`http://127.0.0.1:8731${url(PAGE.menu)}`);
  await showParts(page);
  await expect(page.locator(".nd-menu[id]").first()).toBeHidden();

  // Not one menu button anywhere on the page, in an example or in the site's
  // own furniture. Every one of them opens a `Menu`, and there is no script to
  // open it with.
  const buttons = page.locator('[aria-haspopup="menu"]');
  const count = await buttons.count();
  expect(count, "the page carries no menu buttons at all").toBeGreaterThan(0);
  for (let at = 0; at < count; at += 1) {
    const button = buttons.nth(at);
    await expect(button, `a menu button ${at} is on the page with nothing to open`).toBeHidden();
  }
  await context.close();
});

test("the script reveals the button, and only once it has found the menu", async ({ page }) => {
  await page.goto(url(PAGE.menu));
  await showParts(page);
  const { button } = parts(page);
  await expect(button).toBeVisible();
  // Revealed by `menu.js`, which marks what it enhanced. A button it skipped
  // is a button that stays hidden, because it still opens nothing.
  expect(await button.evaluate((el) => el.dataset.ndEnhanced)).toBe("menu");
  expect(await button.getAttribute("aria-expanded")).toBe("false");
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
  const items = menu.locator('[role="menuitem"]');
  const refused = menu.locator('[aria-disabled="true"]').first();
  // Where it sits, so the walk below is counted rather than guessed.
  const at = await menu.evaluate((root) =>
    [...root.querySelectorAll('[role="menuitem"]')].findIndex(
      (item) => item.getAttribute("aria-disabled") === "true",
    ),
  );
  expect(at, "there is no refused item to walk to").toBeGreaterThan(0);

  // THE ARROWS, PRESSED. The check before this one put the focus on the item
  // itself, which proves the item can hold focus and says nothing about the
  // walk. Down opens onto the first item, and one press per item after that
  // has to arrive on the refused one.
  await button.focus();
  await page.keyboard.press("ArrowDown");
  await expect(items.first()).toBeFocused();
  for (let step = 0; step < at; step += 1) {
    await page.keyboard.press("ArrowDown");
  }
  await expect(refused, "the arrows walked straight past the refused item").toBeFocused();

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

test("a menu opened with the pointer is a tab stop, and Tab closes it", async ({ page }) => {
  await page.goto(url(PAGE.menu));
  await showParts(page);
  // OPENING BY POINTER CALLED `open(false)`, WHICH SET NO STOP. Every item
  // ships `tabindex="-1"` and only `move` ever wrote a `0`, and the click path
  // never called it: Tab skipped the whole menu, `focusout` never fired, and
  // the menu stayed open behind a reader who had moved on. `open`'s `focus`
  // parameter was dead — no call site ever passed `true`.
  const button = page.locator('[aria-haspopup="menu"]').first();
  await button.click();
  const menu = page.locator('[role="menu"]:not([hidden])').first();
  await expect(menu).toBeVisible();

  const stops = await menu.evaluate(
    (el) => [...el.querySelectorAll('[role="menuitem"]')].filter((one) => one.tabIndex === 0).length,
  );
  expect(stops).toBe(1);
  // The focus stays where the reader put it: a pointer open is a reader
  // reading, not a reader walking.
  expect(await menu.evaluate((el) => el.contains(document.activeElement))).toBe(false);

  // Tab reaches the menu, which is what the stop is for.
  await page.keyboard.press("Tab");
  expect(await menu.evaluate((el) => el.contains(document.activeElement))).toBe(true);
  // And leaving it closes it, which is what `focusout` never got the chance to
  // do while nothing inside the menu could hold focus.
  await page.keyboard.press("Tab");
  await expect(page.locator('[role="menu"]:not([hidden])')).toHaveCount(0);
});
