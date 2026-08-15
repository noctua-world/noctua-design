// The bar of controls, and the two states its contract now describes.
//
// WHY THIS FILE EXISTS. `toolbar.toml` stated, as a fact about the markup,
// that exactly one control inside carries `tabindex="0"` and the rest carry
// `-1`. Nothing emitted a `tabindex` at all. `SegmentedItem` renders its own
// roving stop because `selected` tells it which one it is; a toolbar takes
// arbitrary controls as opaque children and cannot write on any of them, in
// either language. So the contract was corrected to describe what really
// happens, and this file holds it to both halves of what it now says:
//
//   1. Once `shell.js` runs, the bar is ONE tab stop.
//   2. With no script every control is its own stop, and none is unreachable.
//
// The second is the honest degradation. It is not a trap, and it is the thing
// a sentence claiming otherwise was hiding.

import { expect, test } from "@playwright/test";

import { PAGE, url, showParts } from "./pages.js";

/** The first bar inside an example, which is the one the page is about. */
const bar = (page) => page.locator('.nd-example-stage [role="toolbar"]').first();

/** Every control the roving order runs over, in document order. */
const stops = (bar_) =>
  bar_.evaluate((el) =>
    [...el.querySelectorAll('[role="radio"], button, a[href], summary')].map((one) => ({
      name: one.getAttribute("aria-label") ?? one.textContent.trim(),
      tab: one.tabIndex,
    })),
  );

test("the script leaves exactly one tab stop on the bar", async ({ page }) => {
  await page.goto(url(PAGE.toolbar));
  await showParts(page);

  const list = await stops(bar(page));
  expect(list.length, "the bar holds no controls").toBeGreaterThan(2);
  const held = list.filter((one) => one.tab === 0);
  expect(held.length, `${held.length} controls claim the bar's single tab stop`).toBe(1);
  // And every other one is reachable by the arrows rather than by Tab.
  expect(list.filter((one) => one.tab === -1).length).toBe(list.length - 1);
});

test("the arrows walk it, and Home and End reach both ends", async ({ page }) => {
  await page.goto(url(PAGE.toolbar));
  await showParts(page);
  const bar_ = bar(page);

  const inside = () =>
    bar_.evaluate((el) => {
      const all = [...el.querySelectorAll('[role="radio"], button, a[href], summary')];
      return all.findIndex((one) => one === document.activeElement || one.contains(document.activeElement));
    });

  await bar_.locator("button").first().focus();
  expect(await inside()).toBe(0);
  await page.keyboard.press("ArrowRight");
  expect(await inside(), "the arrows do not move inside the bar").toBe(1);
  await page.keyboard.press("End");
  const last = (await stops(bar_)).length - 1;
  expect(await inside(), "End did not reach the far end").toBe(last);
  await page.keyboard.press("Home");
  expect(await inside(), "Home did not reach the near end").toBe(0);

  // The stop follows the reader, so leaving the bar and coming back lands
  // where they left it.
  const list = await stops(bar_);
  expect(list.filter((one) => one.tab === 0).length).toBe(1);
});

test("with the script blocked the contract says what the markup does", async ({ browser }) => {
  // THE SENTENCE THAT USED TO BE HERE WAS FALSE, and this is the shape of the
  // one that replaced it. With no script nothing has placed a roving stop, so
  // no control carries `tabindex` at all and each is its own stop. That is
  // more stops than the role recommends and it is not a trap: nothing in the
  // bar is unreachable, which is what a claim of one stop was concealing.
  const context = await browser.newContext({ javaScriptEnabled: false });
  const page = await context.newPage();
  await page.goto(url(PAGE.toolbar));
  await showParts(page);

  const bar_ = bar(page);
  const written = await bar_.evaluate((el) =>
    [...el.querySelectorAll('[role="radio"], button, a[href], summary')].map((one) =>
      one.getAttribute("tabindex"),
    ),
  );
  expect(written.length).toBeGreaterThan(2);
  // A `Segmented` inside the bar renders its own roving stop, which is the one
  // thing here that a component CAN answer for. Everything else writes none.
  expect(
    written.filter((one) => one !== null && one !== "0" && one !== "-1"),
    "a control carries a tabindex nobody can explain",
  ).toEqual([]);

  // And every control is reachable: none of them is taken out of the tab
  // order by a rule that only a script could undo.
  const reachable = await bar_.evaluate((el) =>
    [...el.querySelectorAll("button, a[href], summary")].every(
      (one) => !one.disabled && one.getAttribute("tabindex") !== "-1" || one.getAttribute("role") === "radio",
    ),
  );
  expect(reachable, "a control in the bar cannot be reached at all").toBe(true);
  await context.close();
});
