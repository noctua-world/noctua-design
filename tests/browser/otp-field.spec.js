// One code, in a row of boxes.
//
// WHAT THIS FILE IS ACTUALLY ASKING. Three of the four claims in this
// component's contract are invisible when broken: a row that posts under six
// names instead of one still looks right, an `autocomplete` on every box still
// looks right, and a refusal said between two boxes of an unfinished answer
// still looks like a working field. So each is measured rather than reviewed.
//
// The fourth is the walk itself, and it is `TimeInput`'s: `segmentWalk` in
// `crates/noctua-design-runtime/src/inputs.js` serves both, so a regression
// here is a regression there.

import { expect, test } from "@playwright/test";

import { PAGE, url, showParts } from "./pages.js";

/** The first field on the page, and its boxes. */
async function field(page) {
  await page.goto(url(PAGE.otpField));
  await showParts(page);
  const root = page.locator(".nd-otp-field").first();
  return { root, boxes: root.locator("input.nd-otp-field-input") };
}

test("every box posts under one name, and each has its own label", async ({ page }) => {
  const { root, boxes } = await field(page);
  const count = await boxes.count();
  expect(count).toBeGreaterThan(1);

  const shape = await root.evaluate((el) => {
    const all = [...el.querySelectorAll("input.nd-otp-field-input")];
    return {
      names: [...new Set(all.map((one) => one.name))],
      lengths: [...new Set(all.map((one) => one.maxLength))],
      // The legend names the group; each box needs a name of its own, hidden
      // from the screen and kept for a screen reader.
      labelled: all.every((one) => {
        const label = el.querySelector(`label[for="${CSS.escape(one.id)}"]`);
        return Boolean(label?.textContent?.trim()) && label.clientWidth <= 1;
      }),
      legend: el.closest("fieldset")?.querySelector("legend")?.textContent?.trim(),
    };
  });
  // ONE VALUE. The code arrives as `length` values under one field, in order.
  expect(shape.names).toHaveLength(1);
  expect(shape.lengths).toEqual([1]);
  expect(shape.labelled, "a box with no name of its own").toBe(true);
  expect(shape.legend).toBeTruthy();
});

test("one-time-code is on the first box and on no other", async ({ page }) => {
  const { root } = await field(page);
  const tokens = await root.evaluate((el) =>
    [...el.querySelectorAll("input.nd-otp-field-input")].map((one) =>
      one.getAttribute("autocomplete"),
    ),
  );
  // Repeated, the platform offers the whole code to each of them.
  expect(tokens[0]).toBe("one-time-code");
  expect(tokens.slice(1).every((one) => one === null)).toBe(true);
});

test("typing fills a box and moves on, and Backspace in an empty one goes back", async ({
  page,
}) => {
  const { root, boxes } = await field(page);
  await boxes.nth(0).click();
  await page.keyboard.type("42");

  expect(await boxes.nth(0).inputValue()).toBe("4");
  expect(await boxes.nth(1).inputValue()).toBe("2");
  // The caret has moved on by itself, which is the whole gesture.
  expect(await root.evaluate((el) => document.activeElement.dataset.ndOtpFieldSegment)).toBe("3");

  await page.keyboard.press("Backspace");
  expect(await root.evaluate((el) => document.activeElement.dataset.ndOtpFieldSegment)).toBe("2");
  await page.keyboard.press("Backspace");
  expect(await boxes.nth(1).inputValue()).toBe("");
});

test("a pasted code splits itself across the row", async ({ page }) => {
  const { root, boxes } = await field(page);
  const count = await boxes.count();
  await boxes.nth(0).click();
  await boxes.nth(0).evaluate((el, code) => {
    const data = new DataTransfer();
    data.setData("text", code);
    el.dispatchEvent(new ClipboardEvent("paste", { clipboardData: data, bubbles: true }));
  }, "421337");

  const filled = await root.evaluate((el) =>
    [...el.querySelectorAll("input.nd-otp-field-input")].map((one) => one.value).join(""),
  );
  expect(filled).toBe("421337".slice(0, count));
});

test("a box that already holds a character is typed over", async ({ page }) => {
  const { boxes } = await field(page);
  await boxes.nth(0).click();
  await page.keyboard.type("4");
  await boxes.nth(0).click();
  await page.keyboard.type("7");
  // Without the replace, `maxlength` drops the keystroke and a reader
  // correcting one digit has to delete it first.
  expect(await boxes.nth(0).inputValue()).toBe("7");
});

test("an unfinished answer is not refused between two boxes", async ({ page }) => {
  const { root, boxes } = await field(page);
  const note = root.locator("[data-nd-otp-field-status]");
  await boxes.nth(0).click();
  await page.keyboard.type("42");
  // Still inside the row, so nothing has been said and nothing is marked.
  expect((await note.textContent())?.trim()).toBe("");
  expect(await root.getAttribute("data-nd-invalid")).toBe("false");

  // Leaving the field is when a short code becomes a wrong one.
  await page.locator("h1").first().click();
  await expect(note).not.toBeEmpty();
  expect(await root.getAttribute("data-nd-invalid")).toBe("true");
});

test("the alphanumeric row takes letters and the numeric one does not", async ({ page }) => {
  await page.goto(url(PAGE.otpField));
  await showParts(page);
  const letters = page.locator('.nd-otp-field[data-nd-kind="alphanumeric"]').first();
  await expect(letters).toBeVisible();
  const first = letters.locator("input.nd-otp-field-input").first();
  await first.click();
  await first.fill("");
  await page.keyboard.type("z");
  expect((await first.inputValue()).toLowerCase()).toBe("z");

  const digits = page.locator('.nd-otp-field[data-nd-kind="numeric"]').first();
  const box = digits.locator("input.nd-otp-field-input").first();
  await box.click();
  await box.fill("");
  await page.keyboard.type("z");
  // Dropped as it is typed rather than refused after the fact.
  expect(await box.inputValue()).toBe("");
});

test("every box clears the minimum target size", async ({ page }) => {
  await page.goto(url(PAGE.otpField));
  await showParts(page);
  const small = await page.evaluate(() =>
    [...document.querySelectorAll("input.nd-otp-field-input")]
      .filter((el) => el.offsetParent !== null)
      .map((el) => el.getBoundingClientRect())
      .filter((box) => box.width < 24 || box.height < 24).length,
  );
  expect(small, "a box smaller than the minimum target").toBe(0);
});
