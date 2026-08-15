// The stars, and the number that has to be beside them.
//
// WHAT THIS FILE IS ACTUALLY ASKING. `Rating` is the one control in this system
// whose visible value follows a reader's own choice with **no script running**,
// and every claim in its contract turns on that: real radios, one name per
// option, and a sentence per step with the stylesheet showing exactly one.
//
// The other half is the defect `Meter` and `Progress` both record. A shape a
// reader has to estimate from is a value they were not given, so the number is
// asserted to be on the screen rather than assumed to be.

import { expect, test } from "@playwright/test";

import { PAGE, url, showParts } from "./pages.js";
import { ink } from "./pixels.js";

async function ratings(page) {
  await page.goto(url(PAGE.rating));
  await showParts(page);
}

test("a reading writes its number out and hides its stars", async ({ page }) => {
  await ratings(page);
  const reading = page.locator('.nd-rating[data-nd-readonly="true"]').first();
  await expect(reading).toBeVisible();

  // The picture says nothing to a screen reader, because the sentence beside it
  // already does.
  expect(await reading.locator(".nd-rating-stars").getAttribute("aria-hidden")).toBe("true");

  const said = (await reading.locator(".nd-rating-value").textContent())?.trim();
  expect(said, "a reading with no number is a shape").toBeTruthy();
  // And it is the value the component was given, not a rounding of it.
  const value = await reading.getAttribute("data-nd-value");
  expect(said).toContain(value);
});

test("a fraction is clipped rather than rounded", async ({ page }) => {
  await ratings(page);
  const reading = page.locator('.nd-rating[data-nd-readonly="true"]').first();
  const measured = await reading.evaluate((el) => {
    const track = el.querySelector(".nd-rating-track");
    const level = el.querySelector(".nd-rating-level");
    return {
      value: Number.parseFloat(el.getAttribute("data-nd-value")),
      max: Number.parseFloat(el.getAttribute("data-nd-max")),
      track: track.getBoundingClientRect().width,
      level: level.getBoundingClientRect().width,
      clipped: getComputedStyle(level).overflow,
    };
  });
  // The example is deliberately a fraction, or this test proves nothing.
  expect(Number.isInteger(measured.value)).toBe(false);
  expect(measured.clipped).not.toBe("visible");
  const share = measured.level / measured.track;
  expect(share).toBeGreaterThan(measured.value / measured.max - 0.05);
  expect(share).toBeLessThan(measured.value / measured.max + 0.05);
});

test("the question is real radios that submit under one name", async ({ page }) => {
  await ratings(page);
  const asked = page.locator('.nd-rating[data-nd-readonly="false"]').first();
  await expect(asked).toBeVisible();

  const inputs = asked.locator("input.nd-rating-input");
  const count = await inputs.count();
  expect(count).toBe(Number.parseInt(await asked.getAttribute("data-nd-max"), 10));

  const shape = await asked.evaluate((el) => {
    const boxes = [...el.querySelectorAll("input.nd-rating-input")];
    return {
      types: [...new Set(boxes.map((one) => one.type))],
      names: [...new Set(boxes.map((one) => one.name))],
      values: boxes.map((one) => one.value),
      // A star is a drawing, so the only name each option has is its label.
      named: boxes.every((one) => {
        const label = el.querySelector(`label[for="${CSS.escape(one.id)}"]`);
        return Boolean(label?.textContent?.trim());
      }),
      // The fieldset is the group, so the question is heard once.
      group: el.closest("fieldset") !== null,
    };
  });
  expect(shape.types).toEqual(["radio"]);
  expect(shape.names).toHaveLength(1);
  expect(shape.values).toEqual(Array.from({ length: count }, (_, at) => String(at + 1)));
  expect(shape.named, "an option whose only name is a drawing").toBe(true);
  expect(shape.group).toBe(true);
});

test("choosing a star writes the number beside it, with no script involved", async ({ page }) => {
  await ratings(page);
  const asked = page.locator('.nd-rating[data-nd-readonly="false"]').first();
  const stars = asked.locator("label.nd-rating-star");

  // Nothing chosen yet, so there is nothing to say.
  const before = await asked.locator(".nd-rating-count:visible").count();
  expect(before).toBe(0);

  await stars.nth(2).click();
  const shown = asked.locator(".nd-rating-count:visible");
  await expect(shown).toHaveCount(1);
  expect((await shown.textContent())?.trim()).toContain("3");

  // And it MOVES with the choice rather than accumulating.
  await stars.nth(0).click();
  await expect(asked.locator(".nd-rating-count:visible")).toHaveCount(1);
  expect((await asked.locator(".nd-rating-count:visible").textContent())?.trim()).toContain("1");
});

test("the stars below the chosen one are filled and the rest are not", async ({ page }) => {
  await ratings(page);
  const asked = page.locator('.nd-rating[data-nd-readonly="false"]').first();
  const stars = asked.locator("label.nd-rating-star");
  await stars.nth(2).click();

  // MEASURED AS INK, NOT ASKED OF THE DOM. `getComputedStyle(…).fill` is a
  // colour read back out of the engine, which this repository refuses on the
  // grounds that the string proves nothing about the pixels. A filled star and
  // an outline of the same shape differ by how much of the box is drawn on, so
  // that is what is counted.
  const drawn = [];
  for (const at of [0, 2, 3, 4]) {
    drawn.push(await ink(page, await stars.nth(at).screenshot()));
  }
  const [first, chosen, after, last] = drawn;
  // Three filled, then the rest hollow. Never the only signal: the number is
  // beside them, which the test above asserts.
  expect(chosen).toBeGreaterThan(after * 1.5);
  expect(first).toBeGreaterThan(last * 1.5);
});

test("the arrow keys move between options because the platform says so", async ({ page }) => {
  await ratings(page);
  const asked = page.locator('.nd-rating[data-nd-readonly="false"]').first();
  await asked.locator("label.nd-rating-star").nth(0).click();
  await page.keyboard.press("ArrowRight");
  // A real radio group, so this is the browser's own behaviour and the reason
  // this component ships no script at all.
  const chosen = await asked.evaluate(
    (el) => el.querySelector("input.nd-rating-input:checked")?.value,
  );
  expect(chosen).toBe("2");
});

test("every star clears the minimum target size", async ({ page }) => {
  await ratings(page);
  const small = await page.evaluate(() =>
    [...document.querySelectorAll("label.nd-rating-star")]
      .filter((el) => el.offsetParent !== null)
      .map((el) => el.getBoundingClientRect())
      .filter((box) => box.width < 24 || box.height < 24).length,
  );
  // WCAG 2.5.8 asks for 24px, and a star is the target here because the radio
  // beside it is clipped to nothing.
  expect(small, "a star smaller than the minimum target").toBe(0);
});
