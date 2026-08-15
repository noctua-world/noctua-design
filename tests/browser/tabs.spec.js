// The strip, its tabs and its panels.
//
// WHAT THIS FILE IS ACTUALLY ASKING. Two claims, and both are invisible when
// broken:
//
// 1. **Nothing claims a role it cannot honour.** With the script blocked there
//    is no `role="tablist"`, no `role="tab"` and no `role="tabpanel"`, every
//    panel is on the page, and each tab is a link down to the one it names.
//    A strip that shipped the roles and could not hide a panel would look
//    exactly the same and lie to every screen reader that read it.
// 2. **Manual activation does not choose.** An arrow key that shows a panel
//    under `manual` looks like a working tab strip and is the one behaviour
//    that prop exists to change.
//
// The walk itself is `stops()` and `roving()` in `shell.js`, the same pair a
// toolbar and a radio group use, so a regression here is a regression there.

import { expect, test } from "@playwright/test";

import { PAGE, url, showParts } from "./pages.js";

/** The page, enhanced, and the first strip on it. */
async function strip(page, selector = "[data-nd-tabs]") {
  await page.goto(url(PAGE.tabs));
  await showParts(page);
  const set = page.locator(selector).first();
  await expect(set).toHaveAttribute("role", "tablist");
  return set;
}

test("with no script every panel is on the page and no role is claimed", async ({ browser }) => {
  const context = await browser.newContext({ javaScriptEnabled: false });
  const page = await context.newPage();
  await page.goto(url(PAGE.tabs));

  const shape = await page.evaluate(() => {
    const strips = [...document.querySelectorAll("[data-nd-tabs]")];
    const panels = [...document.querySelectorAll(".nd-tab-panel")];
    return {
      strips: strips.length,
      roles: strips.map((el) => el.getAttribute("role")).filter(Boolean).length,
      tabRoles: [...document.querySelectorAll(".nd-tab")]
        .map((el) => el.getAttribute("role"))
        .filter(Boolean).length,
      panels: panels.length,
      hidden: panels.filter((el) => el.hidden).length,
      // The strip is a table of contents: every tab points at a panel that
      // really is on the page.
      dangling: [...document.querySelectorAll(".nd-tab[href]")].filter(
        (el) => !document.querySelector(el.getAttribute("href")),
      ).length,
    };
  });
  await context.close();

  expect(shape.strips).toBeGreaterThan(0);
  expect(shape.roles, "a role nothing can honour").toBe(0);
  expect(shape.tabRoles, "a tab that cannot show a panel").toBe(0);
  expect(shape.panels).toBeGreaterThan(0);
  expect(shape.hidden, "a panel hidden behind a control that cannot work").toBe(0);
  expect(shape.dangling, "a tab pointing at nothing").toBe(0);
});

test("with a script the roles and the wiring are complete both ways", async ({ page }) => {
  const set = await strip(page);
  const wired = await set.evaluate((el) => {
    const tabs = [...el.querySelectorAll('[role="tab"]')];
    return tabs.map((tab) => {
      const panel = document.getElementById(tab.getAttribute("aria-controls"));
      return {
        selected: tab.getAttribute("aria-selected"),
        panelRole: panel?.getAttribute("role"),
        // Both ways round: the tab controls the panel and the panel is named
        // by the tab.
        back: panel?.getAttribute("aria-labelledby") === tab.id,
        hidden: panel?.hidden,
      };
    });
  });
  expect(wired.length).toBeGreaterThan(1);
  expect(wired.every((one) => one.panelRole === "tabpanel")).toBe(true);
  expect(wired.every((one) => one.back), "a panel not named by its tab").toBe(true);
  // Exactly one shown, and it is the one that says so.
  expect(wired.filter((one) => one.selected === "true")).toHaveLength(1);
  expect(wired.filter((one) => one.hidden === false)).toHaveLength(1);
});

test("the strip is one tab stop, not one per tab", async ({ page }) => {
  const set = await strip(page);
  const stops = await set.evaluate(
    (el) => [...el.querySelectorAll('[role="tab"]')].filter((tab) => tab.tabIndex === 0).length,
  );
  expect(stops, "a strip of six tabs is not six tab stops").toBe(1);
});

test("an arrow key moves and, by default, shows the panel it moved to", async ({ page }) => {
  const set = await strip(page, '[data-nd-tabs][data-nd-activation="automatic"]');
  const tabs = set.locator('[role="tab"]');
  await tabs.nth(0).focus();
  await page.keyboard.press("ArrowRight");

  const after = await set.evaluate((el) => {
    const all = [...el.querySelectorAll('[role="tab"]')];
    return {
      focused: all.indexOf(document.activeElement),
      chosen: all.findIndex((tab) => tab.getAttribute("aria-selected") === "true"),
    };
  });
  expect(after.focused).toBe(1);
  expect(after.chosen, "automatic activation follows the focus").toBe(1);
});

test("manual activation moves the focus and waits to be told", async ({ page }) => {
  const set = await strip(page, '[data-nd-tabs][data-nd-activation="manual"]');
  const tabs = set.locator('[role="tab"]');
  await tabs.nth(0).focus();
  await page.keyboard.press("ArrowRight");

  const moved = await set.evaluate((el) => {
    const all = [...el.querySelectorAll('[role="tab"]')];
    return {
      focused: all.indexOf(document.activeElement),
      chosen: all.findIndex((tab) => tab.getAttribute("aria-selected") === "true"),
    };
  });
  // THE ONE PLACE A TAB DIFFERS FROM A RADIO. The focus moved and the panel
  // did not, which is the whole of `manual`.
  expect(moved.focused).toBe(1);
  expect(moved.chosen).toBe(0);

  await page.keyboard.press("Enter");
  const told = await set.evaluate((el) =>
    [...el.querySelectorAll('[role="tab"]')].findIndex(
      (tab) => tab.getAttribute("aria-selected") === "true",
    ),
  );
  expect(told).toBe(1);
});

test("Home and End reach the ends of the strip", async ({ page }) => {
  const set = await strip(page, '[data-nd-tabs][data-nd-activation="automatic"]');
  const tabs = set.locator('[role="tab"]');
  const count = await tabs.count();
  await tabs.nth(0).focus();
  await page.keyboard.press("End");
  expect(
    await set.evaluate((el) => [...el.querySelectorAll('[role="tab"]')].indexOf(document.activeElement)),
  ).toBe(count - 1);
  await page.keyboard.press("Home");
  expect(
    await set.evaluate((el) => [...el.querySelectorAll('[role="tab"]')].indexOf(document.activeElement)),
  ).toBe(0);
});

test("a vertical strip is walked with the other pair of arrows", async ({ page }) => {
  const set = await strip(page, '[data-nd-tabs][aria-orientation="vertical"]');
  const tabs = set.locator('[role="tab"]');
  await tabs.nth(0).focus();
  // A horizontal bar that answered Up and Down would swallow the keys a reader
  // scrolls with, and a vertical one that answered Left and Right would be
  // unreachable by the keys pointing along it.
  await page.keyboard.press("ArrowRight");
  expect(
    await set.evaluate((el) => [...el.querySelectorAll('[role="tab"]')].indexOf(document.activeElement)),
  ).toBe(0);
  await page.keyboard.press("ArrowDown");
  expect(
    await set.evaluate((el) => [...el.querySelectorAll('[role="tab"]')].indexOf(document.activeElement)),
  ).toBe(1);
});

test("a refused tab keeps its place and shows nothing", async ({ page }) => {
  const set = await strip(page, '[data-nd-tabs]:has(.nd-tab[aria-disabled="true"])');
  const refused = set.locator('.nd-tab[aria-disabled="true"]').first();
  await expect(refused).toBeVisible();
  // It stays in the strip and in the walk: a tab a reader cannot find is a
  // panel they conclude does not exist.
  expect(await refused.getAttribute("role")).toBe("tab");
  // NOT VACUOUS. Both of these are written by the script. A comparison below
  // is then a statement about a state that exists, rather than about two
  // absent attributes. The check before this one passed on a page where
  // nothing had run at all.
  expect(await refused.getAttribute("aria-selected"), "the script wrote no state").toBe("false");
  const panel = page.locator(`#${await refused.getAttribute("aria-controls")}`);
  await expect(panel, "a refused tab points at no panel").toBeAttached();
  await expect(panel).toBeHidden();

  /** Where it sits in the strip, and how long the strip is. */
  const place = () =>
    set.evaluate((el) => {
      const tabs = [...el.querySelectorAll(".nd-tab")];
      const at = tabs.findIndex((tab) => tab.getAttribute("aria-disabled") === "true");
      return `${at} of ${tabs.length}`;
    });
  const before = await place();

  // `force`, because Playwright reads `aria-disabled` as unactionable, which
  // is the point of the attribute rather than a reason to skip the press.
  await refused.click({ force: true });
  expect(await place(), "a refused tab moved or left the strip").toBe(before);
  expect(await refused.getAttribute("aria-selected"), "a refused tab chose itself").toBe("false");
  await expect(panel, "a refused tab showed its panel").toBeHidden();
});

test("the chosen tab is marked by more than a colour", async ({ page }) => {
  const set = await strip(page);
  const marked = await set.evaluate((el) => {
    const tabs = [...el.querySelectorAll('[role="tab"]')];
    const on = tabs.find((tab) => tab.getAttribute("aria-selected") === "true");
    const off = tabs.find((tab) => tab.getAttribute("aria-selected") !== "true");
    if (!on || !off) return null;
    const read = (node) => {
      const style = getComputedStyle(node);
      return {
        weight: Number.parseFloat(style.fontWeight),
        edge: style.borderBottomColor,
        edgeWidth: Number.parseFloat(style.borderBottomWidth),
        inline: style.borderInlineEndColor,
        inlineWidth: Number.parseFloat(style.borderInlineEndWidth),
      };
    };
    return { on: read(on), off: read(off) };
  });
  expect(marked, "the strip has no chosen tab, or nothing to compare it with").not.toBeNull();

  // THE SIGNAL THAT IS NOT A COLOUR, per `NOCTUA.md §8.8.6`, and the one the
  // check before this one measured and then never asserted. The chosen tab is
  // heavier than the rest, so a reader who cannot tell two hues apart still
  // knows which tab they are on.
  expect(marked.on.weight, "the chosen tab is no heavier than the rest").toBeGreaterThan(
    marked.off.weight,
  );
  // And the mark under it is a rule with real thickness. A rule of no width
  // leaves the colour doing all of the work again, which is the defect this
  // title names.
  const rule = Math.max(marked.on.edgeWidth, marked.on.inlineWidth);
  expect(rule, "the mark under the chosen tab has no thickness").toBeGreaterThan(0);
  // The colour is the second signal rather than the only one.
  expect(marked.on.edge === marked.off.edge && marked.on.inline === marked.off.inline).toBe(false);
});
