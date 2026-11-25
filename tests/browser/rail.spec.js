// The rail: a toolbar and a tab list drawn one rail wide.
//
// WHAT THE CASES CANNOT SAY. `spec/components/toolbar.cases.toml` and
// `spec/components/tabs.cases.toml` hold that a rail is announced as vertical
// and which arrows walk it. What `NOCTUA.md §12.11` asks of a rail is geometry:
// it is exactly as wide as a footer's corner, a rail of controls holds two to a
// row, and every hit area meets the smallest target even where the glyph in it
// is drawn smaller. That is asked here, with the two things a rail of icons owes
// a reader who sees it and one who does not: the name drawn, and the name said
// once.

import { expect, test } from "@playwright/test";

import { LOCALES, PAGE, showParts, url } from "./pages.js";
import { outside } from "./pixels.js";

/** A token's length in pixels, measured on a box and not parsed off a string. */
async function token(page, name) {
  return page.evaluate((property) => {
    const probe = document.createElement("div");
    probe.style.position = "absolute";
    probe.style.blockSize = `var(${property})`;
    document.body.append(probe);
    const size = probe.getBoundingClientRect().height;
    probe.remove();
    return size;
  }, name);
}

const box = (locator) =>
  locator.evaluate((one) => {
    const at = one.getBoundingClientRect();
    return { left: at.left, right: at.right, top: at.top, bottom: at.bottom, width: at.width, height: at.height };
  });

/** The rail of controls the toolbar page names its controls in. */
const controls = (page) => page.locator('.nd-toolbar[data-nd-density="rail"]:has(.nd-tooltip)').first();

/** The rail of tabs the tabs page names its tabs in. */
const strip = (page) => page.locator('.nd-tabs[data-nd-appearance="rail"]:has(.nd-tooltip)').first();

/**
 * How many pixels a keyboard's ring paints outside the control it is on.
 *
 * The bubble that draws the control's name opens on the same focus and is
 * meant to stand outside it, so it is taken out of the picture first.
 */
async function ringOutside(page, control) {
  await page.addStyleTag({ content: ".nd-tooltip-bubble { visibility: hidden; }" });
  await control.scrollIntoViewIfNeeded();
  const before = await page.screenshot();
  await control.focus();
  // A key press, so the engine has no doubt that the keyboard is in use.
  await page.keyboard.press("Shift");
  await expect(control).toBeFocused();
  const after = await page.screenshot();
  const rects = await control.evaluate((one) =>
    [...one.getClientRects()].map((at) => [at.left, at.top, at.right, at.bottom]),
  );
  return outside(page, before, after, rects);
}

for (const width of [320, 768, 1280]) {
  test(`a rail of controls is one rail wide and two to a row, at ${width}`, async ({ page }) => {
    await page.setViewportSize({ width, height: 800 });
    for (const locale of LOCALES) {
      await page.goto(url(PAGE.toolbar, locale));
      await showParts(page);
      const rail = await token(page, "--noctua-shell-rail");
      const floor = await token(page, "--noctua-target-min");
      expect(rail, "the page declares no rail width").toBeGreaterThan(0);
      expect(floor, "the page declares no smallest target").toBeGreaterThan(0);

      const bar = controls(page);
      const outer = await box(bar);
      expect(outer.width, `${locale}: the rail is not a rail wide`).toBeCloseTo(rail, 1);

      const buttons = await bar.locator("button").evaluateAll((all) =>
        all.map((one) => {
          const at = one.getBoundingClientRect();
          return { left: at.left, right: at.right, top: at.top, width: at.width, height: at.height };
        }),
      );
      expect(buttons.length, "the example lost its controls").toBe(3);
      for (const [at, one] of buttons.entries()) {
        expect(one.width, `${locale}: control ${at} is under the smallest target`).toBeGreaterThanOrEqual(floor - 0.05);
        expect(one.height, `${locale}: control ${at} is under the smallest target`).toBeGreaterThanOrEqual(floor - 0.05);
      }
      // TWO TO A ROW, AND THE TWO ARE THE RAIL. The first row runs from one
      // edge of the rail to the other with nothing between its halves.
      expect(buttons[1].top, `${locale}: the second control is not beside the first`).toBeCloseTo(buttons[0].top, 1);
      expect(buttons[0].left).toBeCloseTo(outer.left, 1);
      expect(buttons[0].right, `${locale}: a gap between the two controls of a row`).toBeCloseTo(buttons[1].left, 1);
      expect(buttons[1].right).toBeCloseTo(outer.right, 1);
      // And the third starts a row of its own, in a second group, under them.
      expect(buttons[2].top, `${locale}: the third control did not start a row`).toBeGreaterThan(buttons[0].top + buttons[0].height - 0.05);
      expect(buttons[2].left).toBeCloseTo(outer.left, 1);

      // A CONTROL WITH NO `Tooltip` AROUND IT FILLS ITS HALF AS WELL. The walk
      // over every density draws a rail of bare buttons, each of which has a
      // width of its own that is wider than half a rail.
      const bare = page.locator('.nd-toolbar[data-nd-density="rail"]:not(:has(.nd-tooltip)):has(.nd-toolbar-group)').first();
      const edge = await box(bare);
      const halves = await bare.locator("button").evaluateAll((all) =>
        all.slice(0, 2).map((one) => {
          const at = one.getBoundingClientRect();
          return { left: at.left, right: at.right, top: at.top };
        }),
      );
      expect(halves.length, "the page lost its rail of bare controls").toBe(2);
      expect(edge.width, `${locale}: the bare rail is not a rail wide`).toBeCloseTo(rail, 1);
      expect(halves[1].top).toBeCloseTo(halves[0].top, 1);
      expect(halves[0].left).toBeCloseTo(edge.left, 1);
      expect(halves[0].right, `${locale}: a bare control keeps its own width`).toBeCloseTo(halves[1].left, 1);
      expect(halves[1].right, `${locale}: a bare control hangs out of the rail`).toBeCloseTo(edge.right, 1);
    }
  });

  test(`a rail of tabs is one rail wide and a tab is its icon, at ${width}`, async ({ page }) => {
    await page.setViewportSize({ width, height: 800 });
    for (const locale of LOCALES) {
      await page.goto(url(PAGE.tabs, locale));
      await showParts(page);
      const rail = await token(page, "--noctua-shell-rail");
      const floor = await token(page, "--noctua-target-min");

      const tabs = strip(page);
      const outer = await box(tabs);
      expect(outer.width, `${locale}: the rail is not a rail wide`).toBeCloseTo(rail, 1);

      const drawn = await tabs.locator(".nd-tab").evaluateAll((all) =>
        all.map((one) => {
          const at = one.getBoundingClientRect();
          const glyph = one.querySelector(".nd-icon-glyph")?.getBoundingClientRect();
          const words = one.querySelector(".nd-tab-text");
          const text = words.getBoundingClientRect();
          return {
            left: at.left,
            right: at.right,
            top: at.top,
            bottom: at.bottom,
            height: at.height,
            glyph: glyph ? { centre: (glyph.left + glyph.right) / 2, width: glyph.width } : null,
            text: { width: text.width, height: text.height },
            name: words.textContent.trim(),
          };
        }),
      );
      expect(drawn.length).toBe(3);
      for (const [at, tab] of drawn.entries()) {
        expect(tab.left, `${locale}: tab ${at} does not start at the rail's edge`).toBeCloseTo(outer.left, 1);
        expect(tab.right, `${locale}: tab ${at} does not end at the rail's edge`).toBeCloseTo(outer.right, 1);
        expect(tab.height, `${locale}: tab ${at} is under the smallest target`).toBeGreaterThanOrEqual(floor - 0.05);
        // THE ICON IS DRAWN, ON THE RAIL'S CENTRE LINE. The rule that marks the
        // chosen tab stands on one edge, and a tab that leaned away from it
        // would put three icons off the line the controls rail draws its on.
        expect(tab.glyph, `${locale}: tab ${at} draws no icon`).not.toBeNull();
        expect(tab.glyph.width).toBeGreaterThan(0);
        expect(tab.glyph.centre, `${locale}: tab ${at}'s icon is off the centre line`).toBeCloseTo(
          (outer.left + outer.right) / 2,
          0,
        );
        // THE NAME IS KEPT AND NOT DRAWN: it is in the tab, and it has no box.
        expect(tab.name, `${locale}: tab ${at} lost its name`).not.toBe("");
        expect(tab.text.width, `${locale}: tab ${at} draws its words in the rail`).toBeLessThanOrEqual(1);
        expect(tab.text.height).toBeLessThanOrEqual(1);
      }
      // Stacked, with nothing between them.
      expect(drawn[1].top).toBeCloseTo(drawn[0].bottom, 1);
      expect(drawn[2].top).toBeCloseTo(drawn[1].bottom, 1);
    }
  });
}

test("a rail tab with no icon keeps its words", async ({ page }) => {
  // The grid of every variant draws a rail whose tabs have no icon. A rule
  // that hid the words there would leave three blank squares.
  await page.goto(url(PAGE.tabs));
  await showParts(page);
  const bare = page.locator('.nd-tabs[data-nd-appearance="rail"] .nd-tab:not(:has(.nd-icon-glyph)) .nd-tab-text');
  expect(await bare.count(), "no rail on the page has a tab without an icon").toBeGreaterThan(0);
  const size = await box(bare.first());
  expect(size.width, "a tab with no icon draws nothing").toBeGreaterThan(1);
  expect(size.height).toBeGreaterThan(1);
  // Cut at the rail's width, never hung out of it.
  const rail = await box(bare.first().locator("xpath=ancestor::*[contains(@class, 'nd-tabs')]"));
  expect(size.right).toBeLessThanOrEqual(rail.right + 0.5);
});

test("a rail of controls is one tab stop that all four arrows walk", async ({ page }) => {
  await page.goto(url(PAGE.toolbar));
  await showParts(page);
  const bar = controls(page);
  expect(await bar.getAttribute("aria-orientation"), "a rail is announced as horizontal").toBe("vertical");

  const stops = await bar.locator("button").evaluateAll((all) => all.map((one) => one.tabIndex));
  expect(stops.filter((one) => one === 0).length, "the rail is more than one tab stop").toBe(1);

  const at = () =>
    bar.evaluate((el) => [...el.querySelectorAll("button")].findIndex((one) => one === document.activeElement));
  await bar.locator("button").first().focus();
  await page.keyboard.press("ArrowRight");
  expect(await at(), "Right did not reach the control across").toBe(1);
  await page.keyboard.press("ArrowDown");
  expect(await at(), "Down did not reach the next row").toBe(2);
  await page.keyboard.press("ArrowLeft");
  expect(await at(), "Left did not go back").toBe(1);
  await page.keyboard.press("ArrowUp");
  expect(await at(), "Up did not go back").toBe(0);
});

test("a control in a rail shows its name to the eye and says it once", async ({ page }) => {
  await page.goto(url(PAGE.toolbar));
  await showParts(page);
  const first = controls(page).locator(".nd-tooltip").first();
  const button = first.locator("button");
  const bubble = first.locator(".nd-tooltip-bubble");
  const name = await button.getAttribute("aria-label");
  expect(name, "an icon-only control has no name").toBeTruthy();

  await expect(bubble).toBeHidden();
  await button.focus();
  await expect(bubble).toBeVisible();
  expect((await bubble.textContent()).trim(), "the bubble does not draw the name").toBe(name);
  // Said once: the name is the control's own, and nothing describes it with
  // the same words.
  expect(await button.getAttribute("aria-describedby")).toBeNull();
  expect(await bubble.getAttribute("aria-hidden")).toBe("true");
});

test("a rail of tabs is a tab list whose tabs are named, and Down walks it", async ({ page }) => {
  await page.goto(url(PAGE.tabs));
  await showParts(page);
  const tabs = strip(page);
  expect(await tabs.getAttribute("role")).toBe("tablist");
  expect(await tabs.getAttribute("aria-orientation"), "a rail is announced as horizontal").toBe("vertical");

  // Every accessibility child of the list is a tab, and each has a name. The
  // bubble that draws a name is hidden from this tree, so it is not among them.
  const tree = await tabs.ariaSnapshot();
  const lines = tree.split("\n").filter((line) => line.trim().startsWith("- "));
  expect(lines[0], tree).toMatch(/^- tablist "/u);
  const children = lines.slice(1).filter((line) => /^ {2}- /u.test(line));
  expect(children.length, tree).toBe(3);
  for (const line of children) expect(line, tree).toMatch(/^ {2}- tab "[^"]+"/u);

  const chosen = () => tabs.locator('.nd-tab[aria-selected="true"]').getAttribute("data-nd-value");
  await tabs.locator(".nd-tab").first().focus();
  expect(await chosen()).toBe("commands");
  await page.keyboard.press("ArrowDown");
  expect(await chosen(), "Down did not choose the next tab").toBe("settings");
  // Across the rail there is nothing: a column has no neighbour to the side.
  await page.keyboard.press("ArrowRight");
  expect(await chosen(), "Right moved along a column").toBe("settings");

  // And its bubble is open, drawing the name of the tab the keyboard is on.
  const bubble = tabs.locator('.nd-tooltip:has([data-nd-value="settings"]) .nd-tooltip-bubble');
  await expect(bubble).toBeVisible();
  expect(await bubble.getAttribute("aria-hidden")).toBe("true");
});

test("the keyboard's ring is drawn inside a rail's own controls", async ({ page }) => {
  // A rail stands at the edge of a window, or of a drawer that cuts what hangs
  // out of it, so a ring drawn outside a control would be cut on one side.
  await page.goto(url(PAGE.toolbar));
  await showParts(page);
  const strays = await ringOutside(page, controls(page).locator("button").first());
  expect(strays, "nothing changed when the control took the focus").not.toBe(Infinity);
  expect(strays, "the ring is drawn outside the control").toBe(0);

  await page.goto(url(PAGE.tabs));
  await showParts(page);
  const chosen = strip(page).locator('.nd-tab[aria-selected="true"]');
  const tabStrays = await ringOutside(page, chosen);
  expect(tabStrays, "nothing changed when the tab took the focus").not.toBe(Infinity);
  expect(tabStrays, "the ring is drawn outside the tab").toBe(0);
});
