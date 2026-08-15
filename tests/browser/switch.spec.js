// The switch, and the one thing it owes: moving when it is pressed.
//
// WHY THIS FILE EXISTS. Every switch on `/switch/` is a demonstration, so its
// `name` is a setting the theme model has never heard of. `shell.js` sent all
// of them through `route`, which asks a scope or the document to store a
// setting and returns early when neither recognises the name — so the eight
// controls on the page about switches announced a state and never changed it.
// `refresh` then skipped them too, so nothing anywhere put the state right.
//
// The `Segmented` branch three lines below the switch's had the missing answer
// from the day it was written, under the comment "a control on a page about
// controls has to work". This is that sentence, made mechanical.

import { expect, test } from "@playwright/test";

import { PAGE, url, showParts } from "./pages.js";

/** Every switch inside an example, which is every switch on this page. */
const switches = (page) => page.locator('.nd-example-stage [role="switch"]');

test("a switch the settings model does not know still moves when pressed", async ({ page }) => {
  await page.goto(url(PAGE.switchControl));
  await showParts(page);

  const all = switches(page);
  const count = await all.count();
  expect(count, "the page shows no switches at all").toBeGreaterThan(4);

  for (let at = 0; at < count; at += 1) {
    const flip = all.nth(at);
    if ((await flip.getAttribute("aria-disabled")) === "true") continue;
    const before = await flip.getAttribute("aria-checked");
    // Always written, `true` or `false`. A switch whose state is unknown is
    // not a switch, and this is the attribute the whole defect hid behind.
    expect(["true", "false"], "a switch shipped with no state").toContain(before);
    await flip.click();
    const label = await flip.textContent();
    expect(
      await flip.getAttribute("aria-checked"),
      `\`${label?.trim()}\` did not move when it was pressed`,
    ).toBe(before === "true" ? "false" : "true");
    // And back, so the page is left as it was found and the second press is
    // asserted as well as the first.
    await flip.click();
    expect(await flip.getAttribute("aria-checked")).toBe(before);
  }
});

test("a refused switch stays reachable and refuses", async ({ page }) => {
  await page.goto(url(PAGE.switchControl));
  await showParts(page);

  const refused = page.locator('.nd-example-stage [role="switch"][aria-disabled="true"]').first();
  await expect(refused).toBeVisible();
  // `aria-disabled` rather than `disabled`, so the setting can still be found.
  expect(await refused.evaluate((el) => el.disabled)).toBe(false);
  const before = await refused.getAttribute("aria-checked");
  // Dispatched rather than clicked: the test runner refuses to press something
  // marked `aria-disabled`, and the question here is what the handler does when
  // a press reaches it anyway.
  await refused.dispatchEvent("click");
  expect(await refused.getAttribute("aria-checked"), "a refused switch moved").toBe(before);
});

test("the keyboard moves it, because it is a button", async ({ page }) => {
  await page.goto(url(PAGE.switchControl));
  await showParts(page);

  const flip = switches(page).first();
  const before = await flip.getAttribute("aria-checked");
  await flip.focus();
  await page.keyboard.press("Space");
  expect(await flip.getAttribute("aria-checked"), "Space did nothing").toBe(
    before === "true" ? "false" : "true",
  );
  await page.keyboard.press("Enter");
  expect(await flip.getAttribute("aria-checked"), "Enter did nothing").toBe(before);
});

test("the two sizes are two boxes, not two type sizes", async ({ page }) => {
  await page.goto(url(PAGE.switchControl));
  await showParts(page);
  // `size`'s doc says "the size of the control's box". The only rule keyed on
  // it set a font size and a row height, so the track and the knob were the
  // same 48x24 at both values and the variant grid showed one control twice.
  const boxes = await page.evaluate(() =>
    ["small", "medium"].map((size) => {
      const track = document.querySelector(
        `[role="switch"][data-nd-size="${size}"] .nd-switch-track`,
      );
      const box = track?.getBoundingClientRect();
      return box ? [Math.round(box.width), Math.round(box.height)] : null;
    }),
  );
  expect(boxes[0]).not.toBeNull();
  expect(boxes[1]).not.toBeNull();
  expect(boxes[0][0]).toBeLessThan(boxes[1][0]);
  expect(boxes[0][1]).toBeLessThan(boxes[1][1]);
});
