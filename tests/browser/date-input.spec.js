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

// --- The order is the READER's, and this is the test that says so ----------

/** What each forced locale writes a numeric date in, and what CLDR says. */
const ORDERS = [
  ["en-GB", ["day", "month", "year"], "dmy"],
  ["en-US", ["month", "day", "year"], "mdy"],
  ["pt-BR", ["day", "month", "year"], "dmy"],
];

for (const [locale, expected, stamp] of ORDERS) {
  test(`a reader whose browser says ${locale} gets ${stamp}`, async ({ browser }) => {
    // THE TEST THAT WOULD HAVE CAUGHT IT, AND THE ONE THAT WAS THERE INSTEAD.
    // The old pair asserted `mdy` on the English page and `dmy` on the
    // Portuguese one, which is the defect written down as the requirement:
    // the detection was asking `document.documentElement.lang`, so it
    // answered with the language the PAGE is written in. A bare `en` has no
    // region and CLDR resolves it to the United States, so every reader of
    // the English build was re-laid month-first — including the ones whose
    // browser says `en-GB`, who had been served the right order and were
    // moved off it.
    //
    // Forcing the locale is the whole point: it is the one thing the page
    // cannot influence, so a detection that reads the page cannot pass this.
    const context = await browser.newContext({ locale });
    const page = await context.newPage();
    await page.goto(url(PAGE.dateInput));
    const field = page.locator(BIRTH);
    await expect(field).toHaveAttribute("data-nd-resolved", stamp);
    // Document order agrees with the stamp…
    const parts = await field
      .locator(".nd-date-input-row [data-nd-part]")
      .evaluateAll((all) => all.map((one) => one.dataset.ndPart));
    expect(parts).toEqual(expected);
    // …and Tab walks it: from the first box, Tab lands on the second. Three
    // orders — eyes, Tab, reader — were the defect this contract forbids.
    await page.focus(box(BIRTH, expected[0]));
    await page.keyboard.press("Tab");
    await expect(page.locator(box(BIRTH, expected[1]))).toBeFocused();
    await context.close();
  });
}

test("the page's own language does not decide the order", async ({ browser }) => {
  // The other half of the same claim, and the one a single-locale run cannot
  // make: the Portuguese page read by an American browser is month-first,
  // because the order is a convention the READER keeps and not a word this
  // page is written in. The month NAMES in the calendar stay Portuguese,
  // which is why `dateWords` and `readerLocales` are two functions.
  const context = await browser.newContext({ locale: "en-US" });
  const page = await context.newPage();
  await page.goto(url(PAGE.dateInput, "pt-BR"));
  await expect(page.locator(BIRTH)).toHaveAttribute("data-nd-resolved", "mdy");
  await page.locator(`${VISIT} .nd-date-input-suggest-button`).click();
  // ASSERTED POSITIVELY. This read `not.toHaveText(/Jan|Feb|Mar/)`, which
  // passes in April and says nothing: a test that asserts a consequence rather
  // than the property. The caption has to be written in the page's language,
  // so name that language's months and look for one.
  const caption = await page.locator(`${VISIT} .nd-date-picker-month`).textContent();
  const months = [
    "janeiro", "fevereiro", "março", "abril", "maio", "junho",
    "julho", "agosto", "setembro", "outubro", "novembro", "dezembro",
  ];
  expect(
    months.some((one) => caption.toLowerCase().includes(one)),
    `the caption "${caption}" is not written in the page's language`,
  ).toBe(true);
  await context.close();
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


test("an empty required submit is named, and focus lands on the first box", async ({ page }) => {
  await page.goto(url(PAGE.dateInput));
  await page.focus(box(BIRTH, "year"));
  await page.keyboard.press("Enter");
  const slot = page.locator(`.nd-field:has(${BIRTH}) [data-nd-field-error] [data-nd-field-message]`);
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
  await expect(page.locator(`.nd-field:has(${BIRTH}) [data-nd-field-error] [data-nd-field-message]`)).toContainText(
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
  await expect(page.locator(`.nd-field:has(${BIRTH}) [data-nd-field-error] [data-nd-field-message]`)).toContainText(
    "a date that exists",
  );
  await fill("29", "2", "2026");
  await expect(page.locator(`.nd-field:has(${BIRTH}) [data-nd-field-error] [data-nd-field-message]`)).toContainText(
    "a date that exists",
  );
  await fill("29", "2", "2024");
  await expect(page.locator(`.nd-field:has(${BIRTH}) [data-nd-field-error] [data-nd-field-message]`)).toHaveText("");
  await expect(page.locator(BIRTH)).toHaveAttribute("data-nd-invalid", "false");
});

test("a bound is spoken in the reader's words, never the wire format", async ({ page }) => {
  await page.goto(url(PAGE.dateInput));
  await page.fill(box(VISIT, "day"), "1");
  await page.fill(box(VISIT, "month"), "1");
  await page.fill(box(VISIT, "year"), "2020");
  await page.locator("h1").click();
  const said = await page.locator(`.nd-field:has(${VISIT}) [data-nd-field-error] [data-nd-field-message]`).textContent();
  expect(said).not.toContain("2026-01-01");
  expect(said).toContain("2026");
});

test("a caller's sentence is not the script's to clear", async ({ page }) => {
  await page.goto(url(PAGE.dateInput));
  const slot = page.locator(`.nd-field:has(${BOOKED}) [data-nd-field-error] [data-nd-field-message]`);
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

test("the panel holds a real picker, bounded by the field it sits in", async ({ page }) => {
  await page.goto(url(PAGE.dateInput));
  await page.locator(`${VISIT} .nd-date-input-suggest-button`).click();
  // WHAT THIS TEST IS FOR NOW. Walking the grid by day and by week, turning the
  // month, refusing a day out of range and every other claim about the calendar
  // itself belong to `date-picker.spec.js`, which asks them harder: it asserts
  // the arrow moved exactly one day and exactly seven, and that a refusal is
  // the platform's own `disabled` rather than an attribute nothing enforces.
  //
  // What only this file can ask is the COMPOSITION: that the field renders a
  // real picker rather than a second month grid, that the bounds reach it, and
  // that a day chosen there lands in the three boxes that actually submit.
  const picker = page.locator(`${VISIT} [data-nd-date-picker]`);
  await expect(picker).toHaveCount(1);
  await expect(picker.locator(".nd-date-picker-pane").first()).toHaveAttribute("role", "grid");
  for (const bound of ["data-nd-min", "data-nd-max"]) {
    expect(await picker.getAttribute(bound)).toBe(await page.locator(VISIT).getAttribute(bound));
  }
  // AND NOT ONE OF ITS DAYS POSTS. Two carriers for one date is what a picker
  // inside a field would otherwise be: three text boxes and a month of
  // checkboxes, both under the same field, both reaching the server.
  await expect(page.locator(`${VISIT} .nd-date-picker-input[name]`)).toHaveCount(0);

  const day = await picker
    .locator(".nd-date-picker-cell:not([data-nd-outside])")
    .first()
    .getAttribute("data-nd-day");
  await picker.locator(`.nd-date-picker-cell[data-nd-day="${day}"]`).click();
  const [year, month, at] = day.split("-");
  await expect(page.locator(box(VISIT, "year"))).toHaveValue(year);
  await expect(page.locator(box(VISIT, "month"))).toHaveValue(month);
  await expect(page.locator(box(VISIT, "day"))).toHaveValue(at);
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
    // AND A FLOOR FOR THE OTHER FIELD, because everything below it is an
    // absence and a field that rendered an empty box passes every absence
    // there is. The three boxes that submit are three, on both fields.
    await expect(page.locator(`${VISIT} input[name]`)).toHaveCount(3);
    // No promise the page cannot keep: no listbox, no grid, no expanded, and
    // none of the four a picker inside the panel could now leak. The list grew
    // with the nested component, because a coverage list is a claim of its own
    // and a stale one passes on exactly what it stopped naming.
    for (const claim of [
      '[role="listbox"]',
      '[role="grid"]',
      "[aria-expanded]",
      "[aria-activedescendant]",
      "[data-nd-phase]",
      ".nd-date-picker-input[tabindex]",
      ".nd-date-picker-trigger",
    ]) {
      await expect(page.locator(`${VISIT} ${claim}`), `${claim} is promised with no script`).toHaveCount(0);
    }
    // `aria-current="date"` is NOT on that list, and the distinction is the
    // whole rule. Which day is today is a fact the server knows and states.
    // `role="grid"` is a promise about the arrow keys, and nothing is
    // answering them.
    // The picker itself is served, and served hidden: the field's calendar is
    // an aid to three boxes that already work, so with no script it is simply
    // not there.
    await expect(page.locator(`${VISIT} [data-nd-date-input-calendar]`)).toBeHidden();
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
