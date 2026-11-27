// What a state drawn in colour still draws when the system takes the colours.
//
// WHAT IS BEING GUARDED. Under forced colours the system paints every fill as
// the window and every border, a transparent one included, in the text colour
// (`NOCTUA.md §9.8`). A state a component drew with a fill, or with a border it
// kept transparent until the state arrived, is then drawn the same as no state
// at all: every row of a list wears the bar the active one wears, a line drawn
// as a background is gone, and every tab of a rail has the chosen one's edge.
// Each of those components names the state again in the system's own colours,
// which forced colours keep, and each is asked here the question a reader
// would ask, in pixels, with the colours forced.

import { expect, test } from "@playwright/test";

import { PAGE, forceColours, showParts, url } from "./pages.js";
import { contrast, differing, extremes } from "./pixels.js";

// Asked for before the page loads, as a reader's system would, and again once
// it is there: the emulation can miss a new document (`forceColours`).
test.beforeEach(async ({ page }) => {
  await page.emulateMedia({ forcedColors: "active" });
});

/** A strip of an element's box, `width` pixels wide, at its left or right edge. */
async function strip(page, locator, side, width) {
  const at = await locator.boundingBox();
  const x = side === "left" ? at.x : at.x + at.width - width;
  return page.screenshot({ clip: { x, y: at.y + 2, width, height: at.height - 4 } });
}

test("the active option of a list is told from its neighbour, where the keyboard never goes", async ({
  page,
}) => {
  await page.goto(url(PAGE.optionList));
  await forceColours(page);
  await showParts(page);
  const root = page.locator('[data-nd-option-list="walk"][data-nd-enhanced]');
  await root.locator(".nd-option-list-items").focus();
  await page.keyboard.press("ArrowDown");
  const active = root.locator('.nd-option-item[data-nd-active="true"]');
  await expect(active).toHaveCount(1);
  // THE FOCUS GOES, AND THE MARK STAYS. The list's ring is drawn inside its
  // box, across the top of the first row and the foot of the last, and a
  // strip of either would differ from the active row's for the ring alone.
  await page.evaluate(() => document.activeElement.blur());
  await page.mouse.move(0, 0);
  const other = root.locator('.nd-option-item:not([data-nd-active="true"])').first();
  // The leading strip of each row is the bar's, and nothing else is drawn in
  // it: the name starts after the row's padding.
  expect(
    await differing(page, await strip(page, active, "left", 6), await strip(page, other, "left", 6)),
    "the active row looks like every other row",
  ).toBeGreaterThan(0);
});

test("a second seat's option is told from its neighbour too", async ({ page }) => {
  await page.goto(url(PAGE.optionList));
  await forceColours(page);
  await showParts(page);
  const root = page.locator('[data-nd-option-list="walk"][data-nd-enhanced]');
  await expect(root).toBeAttached();
  await page.mouse.move(0, 0);
  // No keyboard of the window's own is in the list: the seat's is all there is.
  await expect(root.locator('.nd-option-item[data-nd-active="true"]')).toHaveCount(0);
  const seated = root.locator('.nd-option-item[data-nd-value="help"]');
  await seated.evaluate(async (element) => {
    const tag = document.querySelector('script[type="module"][src*="shell.js"]');
    const seats = await import(new URL("seats.js", tag.src).href);
    seats.seatOption("2", element);
  });
  await expect(seated).toHaveAttribute("data-nd-seat-option", "2");
  // Nothing focused the list, so nothing brought it into the picture.
  await root.locator(".nd-option-list-items").scrollIntoViewIfNeeded();
  const other = root.locator(".nd-option-item:not([data-nd-seat-option])").first();
  expect(
    await differing(page, await strip(page, seated, "left", 6), await strip(page, other, "left", 6)),
    "a second seat's option looks like every other row",
  ).toBeGreaterThan(0);
});

test("the line between two panes is still drawn", async ({ page }) => {
  await page.goto(url(PAGE.split));
  await forceColours(page);
  await showParts(page);
  const handle = page.locator('[data-nd-split="split-basic"] > .nd-split-track > .nd-split-handle').first();
  await expect(handle).toBeVisible();
  await handle.scrollIntoViewIfNeeded();
  await page.mouse.move(0, 0);
  // The handle's box is a target, and the line down its middle is all that
  // is drawn in it: something there stands out from the ground.
  const { min, max } = await extremes(page, await handle.screenshot());
  expect(contrast(min, max), "nothing shows that a control is there").toBeGreaterThan(1.5);
});

test("the chosen tab of a rail is told from the others", async ({ page }) => {
  await page.goto(url(PAGE.drawer));
  await forceColours(page);
  await showParts(page);
  const rail = page.locator('.nd-tabs[data-nd-appearance="rail"]').first();
  await expect(rail).toBeVisible();
  await rail.scrollIntoViewIfNeeded();
  await page.mouse.move(0, 0);
  const chosen = rail.locator('.nd-tab[aria-selected="true"]');
  const other = rail.locator('.nd-tab[aria-selected="false"]').first();
  await expect(chosen).toHaveCount(1);
  // The mark is a border on one inline edge, the one that faces the panels.
  // The strips at both edges of the chosen tab are held to the same strips of
  // another tab: at least one of the two differs.
  const edge = await page.evaluate(() => {
    const probe = document.createElement("div");
    probe.style.position = "absolute";
    probe.style.inlineSize = "var(--noctua-border-strong)";
    document.body.append(probe);
    const size = probe.getBoundingClientRect().width;
    probe.remove();
    return Math.max(1, Math.round(size));
  });
  let differ = 0;
  for (const side of ["left", "right"]) {
    differ += await differing(page, await strip(page, chosen, side, edge), await strip(page, other, side, edge));
  }
  expect(differ, "every tab of the rail wears the chosen one's edge").toBeGreaterThan(0);
});
