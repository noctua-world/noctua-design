// The date input, measured.
//
// THE CLAIMS IT STAKES EVERYTHING ON: three real boxes post three real
// values; the row stands in the reader's own order and that order is ONE
// order — document, Tab and eyes; the validator speaks the caller's
// sentences and only when there is something to judge; and everything the
// script keeps — reorder, stepping, paste, suggestions, the month grid — is
// promised nowhere until the script exists. Every test is phrased as the
// defect it catches.

import { expect, test } from "@playwright/test";

import { PAGE, url } from "./pages.js";

const BIRTH = '[data-nd-date-input="birth"]';
const VISIT = '[data-nd-date-input="visit"]';
const ISSUED = '[data-nd-date-input="issued"]';
const BOOKED = '[data-nd-date-input="booked"]';

const box = (field, part) => `${field} input[name$="-${part}"]`;

test("the three boxes post three values, under the field's own names", async ({ page }) => {
  await page.goto(url(PAGE.dateInput));
  await page.fill(box(BIRTH, "day"), "27");
  await page.fill(box(BIRTH, "month"), "3");
  await page.fill(box(BIRTH, "year"), "2007");
  const posted = await page.evaluate(() => {
    const form = document.querySelector('[data-nd-date-input="birth"]').closest("form");
    return Object.fromEntries(new FormData(form));
  });
  expect(posted["birth-day"]).toBe("27");
  expect(posted["birth-month"]).toBe("3");
  expect(posted["birth-year"]).toBe("2007");
});

test("the submit control does not touch the field above it", async ({ page }) => {
  await page.goto(url(PAGE.dateInput));
  // The button sat flush against the fieldset's edge — the example form is
  // a column with the same gap every other example form breathes with.
  const apart = await page.evaluate(() => {
    const field = document.querySelector('[data-nd-date-input="birth"]');
    const send = field.closest("form").querySelector('button[type="submit"]');
    return send.getBoundingClientRect().top - field.getBoundingClientRect().bottom;
  });
  expect(apart).toBeGreaterThanOrEqual(8);
});

test("the English page re-lays the row month-first, and it is ONE order", async ({ page }) => {
  await page.goto(url(PAGE.dateInput));
  const field = page.locator(BIRTH);
  await expect(field).toHaveAttribute("data-nd-resolved", "mdy");
  // Document order agrees with the stamp…
  const parts = await field
    .locator(".nd-date-input-row [data-nd-part]")
    .evaluateAll((all) => all.map((one) => one.dataset.ndPart));
  expect(parts).toEqual(["month", "day", "year"]);
  // …and Tab walks it: from the month box, Tab lands on the day box. Three
  // orders — eyes, Tab, reader — were the defect this contract forbids.
  await page.focus(box(BIRTH, "month"));
  await page.keyboard.press("Tab");
  await expect(page.locator(box(BIRTH, "day"))).toBeFocused();
});

test("every label still points at its own box after the move", async ({ page }) => {
  await page.goto(url(PAGE.dateInput));
  const broken = await page.locator(BIRTH).evaluate((field) => {
    return [...field.querySelectorAll("label")].filter(
      (label) => !field.querySelector(`#${CSS.escape(label.htmlFor)}`),
    ).length;
  });
  expect(broken, "a label naming an id nothing carries").toBe(0);
  await page.locator(`${BIRTH} [data-nd-part="year"] label`).click();
  await expect(page.locator(box(BIRTH, "year"))).toBeFocused();
});

test("a declared order is never second-guessed by detection", async ({ page }) => {
  await page.goto(url(PAGE.dateInput));
  const field = page.locator(ISSUED);
  await expect(field).toHaveAttribute("data-nd-resolved", "ymd");
  const first = field.locator(".nd-date-input-row [data-nd-part]").first();
  await expect(first).toHaveAttribute("data-nd-part", "year");
});

test("the Portuguese page keeps day-first", async ({ page }) => {
  await page.goto(url(PAGE.dateInput, "pt-BR"));
  await expect(page.locator(BIRTH)).toHaveAttribute("data-nd-resolved", "dmy");
});

test("an empty required submit is named, and focus lands on the first box", async ({ page }) => {
  await page.goto(url(PAGE.dateInput));
  await page.focus(box(BIRTH, "year"));
  await page.keyboard.press("Enter");
  const slot = page.locator(`.nd-field:has(${BIRTH}) [data-nd-field-error]`);
  await expect(slot).toContainText("Enter your date of birth");
  await expect(page.locator(BIRTH)).toHaveAttribute("data-nd-invalid", "true");
  // The first box in VISUAL order — month, on this page.
  await expect(page.locator(box(BIRTH, "month"))).toBeFocused();
});

test("one empty box is named as itself, and only it is marked", async ({ page }) => {
  await page.goto(url(PAGE.dateInput));
  await page.fill(box(BIRTH, "day"), "27");
  await page.fill(box(BIRTH, "year"), "2007");
  await page.focus(box(BIRTH, "year"));
  await page.keyboard.press("Enter");
  await expect(page.locator(`.nd-field:has(${BIRTH}) [data-nd-field-error]`)).toContainText(
    "must include a Month",
  );
  await expect(page.locator(box(BIRTH, "month"))).toHaveAttribute("aria-invalid", "true");
  expect(await page.locator(box(BIRTH, "day")).getAttribute("aria-invalid")).toBeNull();
});

test("the 31st of April and an off-leap 29th of February are refused; a leap one is not", async ({
  page,
}) => {
  await page.goto(url(PAGE.dateInput));
  const fill = async (day, month, year) => {
    await page.fill(box(BIRTH, "day"), day);
    await page.fill(box(BIRTH, "month"), month);
    await page.fill(box(BIRTH, "year"), year);
    await page.locator("h1").click();
  };
  await fill("31", "4", "2001");
  await expect(page.locator(`.nd-field:has(${BIRTH}) [data-nd-field-error]`)).toContainText(
    "a date that exists",
  );
  await fill("29", "2", "2026");
  await expect(page.locator(`.nd-field:has(${BIRTH}) [data-nd-field-error]`)).toContainText(
    "a date that exists",
  );
  await fill("29", "2", "2024");
  await expect(page.locator(`.nd-field:has(${BIRTH}) [data-nd-field-error]`)).toHaveText("");
  await expect(page.locator(BIRTH)).toHaveAttribute("data-nd-invalid", "false");
});

test("a bound is spoken in the reader's words, never the wire format", async ({ page }) => {
  await page.goto(url(PAGE.dateInput));
  await page.fill(box(VISIT, "day"), "1");
  await page.fill(box(VISIT, "month"), "1");
  await page.fill(box(VISIT, "year"), "2020");
  await page.locator("h1").click();
  const said = await page.locator(`.nd-field:has(${VISIT}) [data-nd-field-error]`).textContent();
  expect(said).not.toContain("2026-01-01");
  expect(said).toContain("2026");
});

test("a caller's sentence is not the script's to clear", async ({ page }) => {
  await page.goto(url(PAGE.dateInput));
  const slot = page.locator(`.nd-field:has(${BOOKED}) [data-nd-field-error]`);
  await expect(slot).toContainText("has to be booked");
  // The prefilled value is a real date; leaving the field must not settle
  // the caller's complaint.
  await page.focus(box(BOOKED, "day"));
  await page.locator("h1").click();
  await expect(slot).toContainText("has to be booked");
  await expect(page.locator(BOOKED)).toHaveAttribute("data-nd-invalid", "true");
});

test("the typing aids act on leaving the box, and only when asked for", async ({ page }) => {
  await page.goto(url(PAGE.dateInput));
  // `issued` opts in to both.
  await page.fill(box(ISSUED, "day"), "7");
  await page.locator("h1").click();
  await expect(page.locator(box(ISSUED, "day"))).toHaveValue("07");
  await page.fill(box(ISSUED, "year"), "26");
  await page.locator("h1").click();
  await expect(page.locator(box(ISSUED, "year"))).toHaveValue("2026");
  // `birth` did not ask.
  await page.fill(box(BIRTH, "day"), "7");
  await page.locator("h1").click();
  await expect(page.locator(box(BIRTH, "day"))).toHaveValue("7");
});

test("arrows step and wrap, and typing afterwards still works", async ({ page }) => {
  await page.goto(url(PAGE.dateInput));
  const day = page.locator(box(BIRTH, "day"));
  await day.fill("31");
  await day.press("ArrowUp");
  await expect(day).toHaveValue("1");
  await day.press("ArrowDown");
  await expect(day).toHaveValue("31");
  await day.fill("12");
  await expect(day).toHaveValue("12");
});

test("a pasted whole date lands across all three boxes", async ({ page }) => {
  await page.goto(url(PAGE.dateInput));
  const paste = (target, text) =>
    page.locator(target).evaluate((input, value) => {
      const data = new DataTransfer();
      data.setData("text/plain", value);
      input.dispatchEvent(new ClipboardEvent("paste", { clipboardData: data, bubbles: true }));
    }, text);
  await paste(box(BIRTH, "day"), "2007-03-27");
  await expect(page.locator(box(BIRTH, "year"))).toHaveValue("2007");
  await expect(page.locator(box(BIRTH, "month"))).toHaveValue("03");
  await expect(page.locator(box(BIRTH, "day"))).toHaveValue("27");
  // Slashed groups read by the order in force — month-first on this page.
  await paste(box(BIRTH, "month"), "3/27/2007");
  await expect(page.locator(box(BIRTH, "month"))).toHaveValue("03");
  await expect(page.locator(box(BIRTH, "day"))).toHaveValue("27");
});

test("the suggestions exist only where they were asked for", async ({ page }) => {
  await page.goto(url(PAGE.dateInput));
  await expect(page.locator(`${VISIT} .nd-date-input-suggest-button`)).toHaveCount(1);
  await expect(page.locator(`${BIRTH} .nd-date-input-suggest-button`)).toHaveCount(0);
});

test("a quick pick fills all three boxes and focus returns whence it came", async ({ page }) => {
  await page.goto(url(PAGE.dateInput));
  const year = page.locator(box(VISIT, "year"));
  await year.focus();
  await year.press("Alt+ArrowDown");
  const list = page.locator(`${VISIT} .nd-date-input-list`);
  await expect(list).toBeFocused();
  await list.press("ArrowDown");
  await list.press("Enter");
  await expect(page.locator(box(VISIT, "year"))).toHaveValue("2027");
  await expect(page.locator(box(VISIT, "month"))).toHaveValue("01");
  await expect(page.locator(box(VISIT, "day"))).toHaveValue("01");
  await expect(year).toBeFocused();
});

test("Escape closes the panel and abandons none of the three numbers", async ({ page }) => {
  await page.goto(url(PAGE.dateInput));
  await page.fill(box(VISIT, "day"), "14");
  const day = page.locator(box(VISIT, "day"));
  await day.press("Alt+ArrowDown");
  await page.locator(`${VISIT} .nd-date-input-list`).press("Escape");
  await expect(page.locator(`${VISIT} .nd-date-input-panel`)).toBeHidden();
  await expect(day).toHaveValue("14");
});

test("the month grid walks by day and week, and refuses days out of range", async ({ page }) => {
  await page.goto(url(PAGE.dateInput));
  await page.locator(`${VISIT} .nd-date-input-suggest-button`).click();
  const grid = page.locator(`${VISIT} [role="grid"]`);
  await expect(grid).toBeVisible();
  // The quick picks hold focus first; Tab walks through the month controls
  // to the grid's one roving day.
  const focused = () => page.evaluate(() => document.activeElement.dataset.ndValue);
  for (let hop = 0; hop < 5 && !(await focused()); hop += 1) {
    await page.keyboard.press("Tab");
  }
  const before = await focused();
  await page.keyboard.press("ArrowRight");
  const after = await focused();
  expect(after > before, "ArrowRight did not move a day forward").toBe(true);
  await page.keyboard.press("ArrowDown");
  // `min` is 2026-01-01: December 2025 must be reachable and refused.
  const month = page.locator(`${VISIT} .nd-date-input-month`);
  const start = await month.textContent();
  await page.keyboard.press("PageUp");
  expect(await month.textContent()).not.toBe(start);
  for (let back = 0; back < 12; back += 1) await page.keyboard.press("PageUp");
  const refused = await grid
    .locator('.nd-date-input-day[aria-disabled="true"]')
    .count();
  expect(refused, "days before the bound are not refused").toBeGreaterThan(0);
});

test.describe("with the script blocked", () => {
  test.use({ javaScriptEnabled: false });

  test("it is smaller, not broken, and promises nothing", async ({ page }) => {
    await page.goto(url(PAGE.dateInput));
    const field = page.locator(BIRTH);
    // Three labelled boxes, the real `required`, day-first as served.
    await expect(field.locator("input")).toHaveCount(3);
    expect(await page.locator(box(BIRTH, "day")).getAttribute("required")).not.toBeNull();
    await expect(field).toHaveAttribute("data-nd-resolved", "dmy");
    const parts = await field
      .locator(".nd-date-input-row [data-nd-part]")
      .evaluateAll((all) => all.map((one) => one.dataset.ndPart));
    expect(parts).toEqual(["day", "month", "year"]);
    // No promise the page cannot keep: no listbox, no grid, no expanded.
    for (const claim of ['[role="listbox"]', '[role="grid"]', "[aria-expanded]", "[aria-activedescendant]"]) {
      await expect(page.locator(`${VISIT} ${claim}`)).toHaveCount(0);
    }
    // The quick picks stay inert and hidden.
    await expect(page.locator(`${VISIT} [data-nd-date-input-suggest]`)).toBeHidden();
    await expect(page.locator(`${VISIT} .nd-date-input-suggest-button`)).toHaveCount(0);
  });
});

test("every offered locale asks in its own words", async ({ page }) => {
  await page.goto(url(PAGE.dateInput, "pt-BR"));
  await expect(page.locator(`${VISIT} .nd-date-input-suggest-button`)).toHaveAttribute(
    "aria-label",
    "Sugestões",
  );
  await expect(page.locator(`.nd-field:has(${BIRTH}) .nd-label`)).toContainText(
    "data de nascimento",
  );
});
