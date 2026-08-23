// The month, the range painted across it, and the keyboard that walks both.
//
// WHAT THIS FILE IS ACTUALLY ASKING. Every day here is a real radio or
// checkbox, so the claims worth pinning are the ones that follow from that and
// from nothing else: one tab stop with no script, a value that submits with no
// script, a bound that is the platform's own `disabled` rather than a
// decoration, and a range whose two ends are two ticked boxes.
//
// AND THE MOVES ARE DATE ARITHMETIC, NOT AN INDEX INTO A ROW OF CELLS. That is
// the difference this component was written around: an index breaks the day a
// week-number column joins the row, and again the day a second month is drawn
// beside the first. Both are on this page, so both are asked here.
//
// Every assertion about a date compares ISO strings rather than naming a day,
// so nothing in this file starts failing because the calendar turned over.

import { expect, test } from "@playwright/test";

import { LOCALES, OFFERED, PAGE, url, showParts } from "./pages.js";
import { contrast, extremes, pixelAt } from "./pixels.js";

const VISIT = '[data-nd-date-picker="visit"]';
const STAY = '[data-nd-date-picker="stay"]';
const BOOKING = '[data-nd-date-picker="booking"]';
const FIELD = '[data-nd-date-picker="arrival"]';
const TRIP = '[data-nd-date-picker="trip"]';
const ATTACHED = '[data-nd-date-picker="flight"]';
const STACKED = '[data-nd-date-picker="nights"]';
const INSIDE = '[data-nd-date-picker="rental"]';
const TERM = '[data-nd-date-picker="term"]';
// The small and large pickers from the states example. `small` is where a
// column first stops being wide enough for the word above it, so a sweep that
// leaves it out is a sweep that cannot see the defect it was written for.
const SMALL = '[data-nd-date-picker="compact"]';
const LARGE = '[data-nd-date-picker="roomy"]';

/**
 * A page whose BROWSER says `locale`, which is a different question from the
 * language the page is written in.
 *
 * The order a box asks its three numbers in is a convention, so it follows the
 * reader. Every assertion about what a box reads has to force one, or it is
 * asserting whatever locale the test runner happened to start in — which is
 * how the first version of the typing test came to expect day-first on a
 * runner that says `en-US`.
 */
async function reader(browser, locale) {
  const context = await browser.newContext({ locale });
  const page = await context.newPage();
  await page.goto(url(PAGE.datePicker));
  await showParts(page);
  return { page, context };
}

async function pickers(page, locale = "en") {
  await page.goto(url(PAGE.datePicker, locale));
  await showParts(page);
}

/** The days actually ticked, which is the value and not the paint. */
const ticked = (page, root) =>
  page.locator(`${root} .nd-date-picker-input`).evaluateAll((all) =>
    all.filter((one) => one.checked).map((one) => one.value),
  );

/** The day the reader is on, read off the control rather than off the label. */
const held = (page) => page.evaluate(() => document.activeElement?.value ?? null);

const press = (page, root, day) =>
  page.locator(`${root} .nd-date-picker-cell[data-nd-day="${day}"]`).click();

// --- The grid and its keyboard ----------------------------------------------

test("the month is one tab stop, and it is on the control rather than the label", async ({
  page,
}) => {
  await pickers(page);
  const stops = await page
    .locator(`${VISIT} .nd-date-picker-input`)
    .evaluateAll((all) => all.filter((one) => one.tabIndex === 0).length);
  // ONE STOP for a month, not forty-two. A stop per day is a month a keyboard
  // reader walks through rather than into.
  expect(stops, "the roving tabindex is not roving").toBe(1);
  // AND NOT ON THE LABEL. The input is what carries `checked` and what a screen
  // reader announces, so a tab stop anywhere else is a stop on a thing that
  // answers no key.
  const strays = await page
    .locator(`${VISIT} .nd-date-picker-cell[tabindex]`)
    .count();
  expect(strays, "a label carries a tab stop").toBe(0);
});

test("the arrows move by a day and by a week, and cross the edge of the month", async ({
  page,
}) => {
  await pickers(page);
  await page.locator(`${VISIT} .nd-date-picker-input[tabindex="0"]`).focus();
  const start = await held(page);
  await page.keyboard.press("ArrowRight");
  const across = await held(page);
  await page.keyboard.press("ArrowDown");
  const down = await held(page);
  // A DAY, AND THEN EXACTLY SEVEN. Asserting the delta rather than "it changed"
  // is what catches a move that walks the row of cells instead of the calendar.
  expect(days(start, across), "ArrowRight did not move one day").toBe(1);
  expect(days(across, down), "ArrowDown did not move one week").toBe(7);
});

test("an arrow off the end of the month steps into the next one", async ({ page }) => {
  await pickers(page);
  // The last day drawn in the first pane's own month, found by description
  // rather than by position.
  const last = await page.locator(`${VISIT} .nd-date-picker-cell`).evaluateAll((all) => {
    const inside = all.filter((one) => one.dataset.ndOutside !== "true");
    return inside[inside.length - 1].dataset.ndDay;
  });
  await page.locator(`${VISIT} .nd-date-picker-input[value="${last}"]`).focus();
  await page.keyboard.press("ArrowRight");
  expect(days(last, await held(page)), "the month edge stopped the walk").toBe(1);
});

test("Home and End walk the row the reader is in, every row of it", async ({ page }) => {
  await pickers(page);
  // WHY EVERY ROW AND NOT ONE. This started as a single press in the middle of
  // the grid, and restoring the defect it was written for left it green: with
  // one month, the day cells are forty-two contiguous controls and counting
  // seven of them lands on a row boundary by arithmetic accident.
  //
  // Three panes break that accident, because a day another pane already draws
  // is an empty cell here, so a pane no longer contributes a multiple of seven.
  // Any row after the first blank is where an index and a row disagree, and
  // only sweeping them all is sure to include one.
  const rows = await page.locator(`${TERM} tbody tr`).count();
  let swept = 0;
  for (let at = 0; at < rows; at += 1) {
    const row = page.locator(`${TERM} tbody tr`).nth(at);
    const days = await row.locator(".nd-date-picker-input").evaluateAll((all) =>
      all.map((one) => one.value),
    );
    if (days.length === 0) continue;
    await row.locator(".nd-date-picker-input").first().focus();
    await page.keyboard.press("End");
    expect(await held(page), `End left row ${at}`).toBe(days[days.length - 1]);
    await row.locator(".nd-date-picker-input").last().focus();
    await page.keyboard.press("Home");
    expect(await held(page), `Home left row ${at}`).toBe(days[0]);
    swept += 1;
  }
  expect(swept, "no row was walked").toBeGreaterThan(12);
});

test("Ctrl with Home and End reaches the ends of everything on show", async ({ page }) => {
  await pickers(page);
  const all = await page
    .locator(`${TERM} .nd-date-picker-input`)
    .evaluateAll((one) => one.map((each) => each.value));
  await page.locator(`${TERM} .nd-date-picker-input`).nth(30).focus();
  await page.keyboard.press("Control+Home");
  expect(await held(page)).toBe(all[0]);
  await page.keyboard.press("Control+End");
  expect(await held(page)).toBe(all[all.length - 1]);
});

test("Page Up turns a month and Shift turns a year", async ({ page }) => {
  await pickers(page);
  const caption = page.locator(`${VISIT} .nd-date-picker-month`);
  const first = await caption.textContent();
  await page.locator(`${VISIT} .nd-date-picker-input[tabindex="0"]`).focus();
  await page.keyboard.press("PageUp");
  const back = await caption.textContent();
  expect(back).not.toBe(first);
  await page.keyboard.press("Shift+PageUp");
  const year = await caption.textContent();
  expect(year).not.toBe(back);
  expect(year).not.toBe(first);
});

test("an arrow across two panes does not turn the page under the reader", async ({ page }) => {
  await pickers(page);
  const captions = page.locator(`${STAY} .nd-date-picker-month`);
  const before = await captions.allTextContents();
  // The last day of the first pane's own month, by description.
  const edge = await page.locator(`${STAY} .nd-date-picker-cell`).evaluateAll((all) => {
    const inside = all.filter((one) => one.dataset.ndOutside !== "true");
    return inside[inside.length - 1].dataset.ndDay;
  });
  await page.locator(`${STAY} .nd-date-picker-input[value="${edge}"]`).focus();
  await page.keyboard.press("ArrowRight");
  // The day moved, and the months did NOT. Re-seeding on every crossing is what
  // makes the day a reader moved to jump from the second pane to the first.
  expect(days(edge, await held(page))).toBe(1);
  expect(await captions.allTextContents(), "the panes turned under the reader").toEqual(before);
});

test("the week number is a row heading and never a tab stop", async ({ page }) => {
  await pickers(page);
  const weeks = page.locator(`${TERM} .nd-date-picker-week`);
  const count = await weeks.count();
  expect(count, "no week numbers were drawn").toBeGreaterThan(5);
  const scopes = await weeks.evaluateAll((all) => all.map((one) => one.getAttribute("scope")));
  expect(new Set(scopes)).toEqual(new Set(["row"]));
  expect(await page.locator(`${TERM} .nd-date-picker-week[tabindex]`).count()).toBe(0);
});

// --- The range, with a pointer ----------------------------------------------

test("the first press posts nothing, because half a range is not an answer", async ({ page }) => {
  await pickers(page);
  await page.locator(`${STAY} .nd-date-picker-clear`).click();
  const day = await firstInside(page, STAY);
  await press(page, STAY, day);
  expect(await page.locator(STAY).getAttribute("data-nd-phase")).toBe("anchored");
  // One box, and the component says so rather than the paint saying so.
  expect(await ticked(page, STAY)).toEqual([day]);
});

test("a second press before the first completes the range rather than restarting", async ({
  page,
}) => {
  await pickers(page);
  await page.locator(`${STAY} .nd-date-picker-clear`).click();
  const first = await firstInside(page, STAY);
  const later = shift(first, 6);
  // The LATER day first, then the earlier one.
  await press(page, STAY, later);
  await press(page, STAY, first);
  // WHAT THIS ACTUALLY PROVES, because the first title overstated it. The
  // ORDER is structural: the days are checkboxes in document order and document
  // order is date order, so a value read off them cannot come back backwards
  // and no defect restored here could make it. What can break is the rule about
  // the second press. Reading it as "before the anchor, so start again" is a
  // real and tempting reading, and it would leave one day ticked instead of two.
  expect(await ticked(page, STAY)).toEqual([first, later]);
  expect(await page.locator(STAY).getAttribute("data-nd-phase")).toBe("complete");
});

test("the same day twice is a range of one day, not a clear", async ({ page }) => {
  await pickers(page);
  await page.locator(`${STAY} .nd-date-picker-clear`).click();
  const day = await firstInside(page, STAY);
  await press(page, STAY, day);
  await press(page, STAY, day);
  // A stay of one night is a legitimate answer, and reading equality as a
  // toggle puts it out of reach of a pointer altogether.
  expect(await page.locator(STAY).getAttribute("data-nd-phase")).toBe("complete");
  expect(await ticked(page, STAY)).toEqual([day]);
  expect(
    await page.locator(`${STAY} .nd-date-picker-cell[data-nd-in-range="both"]`).count(),
  ).toBe(1);
});

test("a third press starts over rather than moving whichever end is nearer", async ({ page }) => {
  await pickers(page);
  await page.locator(`${STAY} .nd-date-picker-clear`).click();
  const first = await firstInside(page, STAY);
  await press(page, STAY, first);
  await press(page, STAY, shift(first, 4));
  await press(page, STAY, shift(first, 2));
  expect(await ticked(page, STAY)).toEqual([shift(first, 2)]);
  expect(await page.locator(STAY).getAttribute("data-nd-phase")).toBe("anchored");
});

test("hovering previews the range and takes nothing", async ({ page }) => {
  await pickers(page);
  await page.locator(`${STAY} .nd-date-picker-clear`).click();
  const first = await firstInside(page, STAY);
  await press(page, STAY, first);
  const before = await ticked(page, STAY);
  await page.locator(`${STAY} .nd-date-picker-cell[data-nd-day="${shift(first, 5)}"]`).hover();
  const previewed = await page.locator(`${STAY} [data-nd-preview]`).count();
  // A FLOOR, because a component that previews nothing would otherwise pass
  // the assertion that matters most here.
  expect(previewed, "nothing was previewed").toBeGreaterThan(0);
  expect(await ticked(page, STAY), "hovering chose a day").toEqual(before);
});

test("a bound is the platform's own refusal, not a decoration", async ({ page }) => {
  await pickers(page);
  const refused = await page
    .locator(`${BOOKING} .nd-date-picker-input`)
    .evaluateAll((all) => all.filter((one) => one.disabled).map((one) => one.value));
  expect(refused.length, "no day was refused").toBeGreaterThan(0);
  const min = await page.locator(BOOKING).getAttribute("data-nd-min");
  const max = await page.locator(BOOKING).getAttribute("data-nd-max");
  for (const day of refused) {
    expect(day < min || day > max, `${day} is inside the bounds and still refused`).toBe(true);
  }
  // A disabled control cannot be reached, so there is no sentence to say and no
  // click handler re-checking an attribute nothing enforced.
  expect(await page.locator(`${BOOKING} .nd-date-picker-input:disabled`).first().isDisabled()).toBe(
    true,
  );
});

test("clearing empties the answer and never drops focus onto the body", async ({ page }) => {
  await pickers(page);
  const day = await firstInside(page, STAY);
  await press(page, STAY, day);
  await page.locator(`${STAY} .nd-date-picker-clear`).click();
  expect(await ticked(page, STAY)).toEqual([]);
  expect(await page.locator(STAY).getAttribute("data-nd-phase")).toBe("empty");
  const landed = await page.evaluate(() => document.activeElement.tagName);
  expect(landed, "focus fell to the body").not.toBe("BODY");
});

// --- The range, from the keyboard -------------------------------------------

test("Enter sets a start, then an end, and moving between them chooses nothing", async ({
  page,
}) => {
  await pickers(page);
  await page.locator(`${STAY} .nd-date-picker-clear`).click();
  const first = await firstInside(page, STAY);
  await page.locator(`${STAY} .nd-date-picker-input[value="${first}"]`).focus();
  await page.keyboard.press("Enter");
  expect(await ticked(page, STAY)).toEqual([first]);
  for (let step = 0; step < 3; step += 1) await page.keyboard.press("ArrowRight");
  // MOVING IS NOT CHOOSING. A picker that chose as it moved would answer the
  // question every time a reader passed a day on the way to another.
  expect(await ticked(page, STAY), "an arrow chose a day").toEqual([first]);
  await page.keyboard.press("Enter");
  expect(await ticked(page, STAY)).toEqual([first, shift(first, 3)]);
});

test("Escape abandons a half-made range and leaves a finished one alone", async ({ page }) => {
  await pickers(page);
  await page.locator(`${STAY} .nd-date-picker-clear`).click();
  const first = await firstInside(page, STAY);
  await page.locator(`${STAY} .nd-date-picker-input[value="${first}"]`).focus();
  await page.keyboard.press("Enter");
  await page.keyboard.press("Escape");
  expect(await ticked(page, STAY), "a half-made range survived Escape").toEqual([]);
  // And a finished one does not move.
  await press(page, STAY, first);
  await press(page, STAY, shift(first, 3));
  const made = await ticked(page, STAY);
  await page.locator(`${STAY} .nd-date-picker-input[value="${first}"]`).focus();
  await page.keyboard.press("Escape");
  expect(await ticked(page, STAY)).toEqual(made);
});

test("Shift with an arrow extends a finished range from the end that has focus", async ({
  page,
}) => {
  await pickers(page);
  await page.locator(`${STAY} .nd-date-picker-clear`).click();
  const first = await firstInside(page, STAY);
  await press(page, STAY, first);
  await press(page, STAY, shift(first, 4));
  // From the LATER end, so the later end is the one that moves.
  await page.locator(`${STAY} .nd-date-picker-input[value="${shift(first, 4)}"]`).focus();
  await page.keyboard.press("Shift+ArrowRight");
  expect(await ticked(page, STAY)).toEqual([first, shift(first, 5)]);
  // AND THE OTHER END DID NOT MOVE. That is the whole of "from the end that
  // has focus": a chord that moved whichever end was nearer would be the third
  // press rule again, wearing a different key.
  await page.locator(`${STAY} .nd-date-picker-input[value="${first}"]`).focus();
  await page.keyboard.press("Shift+ArrowLeft");
  expect(await ticked(page, STAY)).toEqual([shift(first, -1), shift(first, 5)]);
});

test("a held Shift says one sentence, not one per press", async ({ page }) => {
  await pickers(page);
  await page.locator(`${STAY} .nd-date-picker-clear`).click();
  const first = await firstInside(page, STAY);
  await press(page, STAY, first);
  await press(page, STAY, shift(first, 2));
  const status = page.locator(`${STAY} [data-nd-date-picker-status]`);
  // Count what the live region actually does, rather than what it ends up
  // saying. A region rewritten five times says five things to a reader.
  await status.evaluate((region) => {
    window.ndSaid = 0;
    new MutationObserver(() => {
      window.ndSaid += 1;
    }).observe(region, { childList: true, characterData: true, subtree: true });
  });
  await page.locator(`${STAY} .nd-date-picker-input[value="${shift(first, 2)}"]`).focus();
  await page.keyboard.down("Shift");
  for (let step = 0; step < 4; step += 1) await page.keyboard.press("ArrowRight");
  await page.keyboard.up("Shift");
  const said = await page.evaluate(() => window.ndSaid);
  // A FLOOR AS WELL AS A CEILING, because a region that says nothing at all
  // would otherwise pass the assertion that matters.
  expect(said, "the range was never announced").toBeGreaterThan(0);
  expect(said, `the region was rewritten ${said} times for one held Shift`).toBeLessThanOrEqual(2);
});

test("no sentence the script writes escapes the pseudo-locale untranslated", async ({ page }) => {
  await pickers(page, "qps");
  // THE RUST SWEEP CANNOT SEE THIS. `every_sentence_on_every_page_is_translated`
  // reads rendered HTML, so a sentence hard-coded in English inside an enhancer
  // is invisible to it, and nothing else in this suite looks either.
  //
  // Every word a reader can see here is the caller's, so every one has to carry
  // the pseudo-locale's brackets. The exceptions are the day numerals and the
  // month caption, which are dates rather than sentences.
  const runs = await page.locator(`${VISIT}`).evaluate((root) => {
    const words = [];
    for (const el of root.querySelectorAll(
      ".nd-date-picker-weekday, .nd-date-picker-control[aria-label], legend",
    )) {
      const text = el.getAttribute("aria-label") ?? el.textContent;
      if (text && text.trim()) words.push(text.trim());
    }
    return words;
  });
  expect(runs.length, "no script-written word was examined").toBeGreaterThan(8);
  for (const run of runs) {
    expect(run, `"${run}" reached the reader untranslated`).toContain("\u27e6");
  }
});

// --- The box, and the month behind it ---------------------------------------

test("a date can be typed, and what was typed is read back canonically", async ({ browser }) => {
  const { page, context } = await reader(browser, "en-GB");
  const box = page.locator(`${FIELD} .nd-date-picker-entry`);
  // SPLIT ON WHATEVER IS NOT A DIGIT. A reader who typed dots meant the three
  // numbers a reader who typed slashes meant.
  await box.fill("18.8.2026");
  await box.blur();
  await expect(box).toHaveValue("18/08/2026");
  expect(await ticked(page, FIELD)).toEqual(["2026-08-18"]);
  await context.close();
});

const ORDERS = [
  ["en-GB", "dmy"],
  ["pt-BR", "dmy"],
  ["en-US", "mdy"],
];

for (const [locale, order] of ORDERS) {
  test(`a reader whose browser says ${locale} is asked for ${order}`, async ({ browser }) => {
    // THE PLATFORM'S OWN DEFAULT IS DAY FIRST, and a reader whose conventions
    // say otherwise is served otherwise. The page cannot influence this, which
    // is the whole reason it is forced here.
    const { page, context } = await reader(browser, locale);
    await expect(page.locator(FIELD)).toHaveAttribute("data-nd-resolved", order);
    const shown = await page.locator(`${FIELD} .nd-date-picker-entry`).getAttribute("placeholder");
    // The letters are the page's own, so only their ORDER is asserted: the
    // first thing an empty box asks for is the first thing the order names.
    expect(shown[0].toLowerCase(), `the box shows "${shown}"`).toBe(order[0]);
    // And a date typed in that order is the date it names.
    const box = page.locator(`${FIELD} .nd-date-picker-entry`);
    await box.fill(order === "mdy" ? "08/18/2026" : "18/08/2026");
    await box.blur();
    expect(await ticked(page, FIELD)).toEqual(["2026-08-18"]);
    await context.close();
  });
}

test("a date already in a box keeps its meaning when the order is re-read", async ({ browser }) => {
  // The markup serves day first and the script may move it. A date served as
  // `18/08/2026` and re-read as month-first has to become `08/18/2026`, not
  // stay put and mean the eighteenth month.
  const { page, context } = await reader(browser, "en-US");
  await expect(page.locator(TRIP)).toHaveAttribute("data-nd-resolved", "mdy");
  const boxes = page.locator(`${TRIP} .nd-date-picker-entry`);
  await boxes.nth(0).fill("08/18/2026");
  await boxes.nth(0).blur();
  expect(await ticked(page, TRIP)).toEqual(["2026-08-18"]);
  await context.close();
});

test("something that is not a date is refused and said out loud", async ({ browser }) => {
  const { page, context } = await reader(browser, "en-GB");
  const box = page.locator(`${FIELD} .nd-date-picker-entry`);
  // The 31st of February is three numbers and no day.
  await box.fill("31/02/2026");
  await box.blur();
  await expect(box).toHaveAttribute("aria-invalid", "true");
  await expect(page.locator(`${FIELD} [data-nd-date-picker-status]`)).toContainText("31/02/2026");
  expect(await ticked(page, FIELD), "a date that does not exist was taken").toEqual([]);
  await context.close();
});

test("pressing the box opens the month, and pressing a day fills it and closes", async ({
  browser,
}) => {
  const { page, context } = await reader(browser, "en-GB");
  const box = page.locator(`${FIELD} .nd-date-picker-entry`);
  const region = page.locator(`${FIELD} .nd-date-picker-region`);
  await box.click();
  await expect(region).toBeVisible();
  await expect(page.locator(`${FIELD} .nd-date-picker-open`)).toHaveAttribute(
    "aria-expanded",
    "true",
  );
  const day = await firstInside(page, FIELD);
  await press(page, FIELD, day);
  const [year, month, at] = day.split("-");
  await expect(box).toHaveValue(`${at}/${month}/${year}`);
  // ONE DATE IS DONE AT THE FIRST PRESS. A month left open after the last
  // press is a month the reader has to dismiss.
  await expect(region).toBeHidden();
  await context.close();
});

test("Tab past the box does not unfold the month, and Alt with Down asks for it", async ({
  page,
}) => {
  await pickers(page);
  const region = page.locator(`${FIELD} .nd-date-picker-region`);
  await page.locator(`${FIELD} .nd-date-picker-entry`).focus();
  await expect(region, "moving into the box unfolded the month").toBeHidden();
  await page.keyboard.press("Alt+ArrowDown");
  await expect(region).toBeVisible();
});

test("the days behind a box carry no name, so one date reaches the server once", async ({
  page,
}) => {
  await pickers(page);
  // THE DEFECT THIS IS WRITTEN FOR. The markup leaves them nameless and the
  // script draws them again on every turn, so both halves have to know: a
  // month of named checkboxes beside a named box is one date under two schemes.
  expect(await page.locator(`${FIELD} .nd-date-picker-input[name]`).count()).toBe(0);
  await page.locator(`${FIELD} .nd-date-picker-entry`).click();
  await page.locator(`${FIELD} .nd-date-picker-turn[data-nd-turn="ahead"]`).click();
  expect(
    await page.locator(`${FIELD} .nd-date-picker-input[name]`).count(),
    "the rebuild put the names back",
  ).toBe(0);
  // And the box itself is what posts.
  await expect(page.locator(`${FIELD} .nd-date-picker-entry`)).toHaveAttribute("name", "arrival");
});

test("two boxes share one month, and the start sends the reader to the end", async ({
  browser,
}) => {
  const { page, context } = await reader(browser, "en-GB");
  const boxes = page.locator(`${TRIP} .nd-date-picker-entry`);
  const region = page.locator(`${TRIP} .nd-date-picker-region`);
  await expect(boxes).toHaveCount(2);
  await expect(boxes.nth(0)).toHaveAttribute("name", "trip-start");
  await expect(boxes.nth(1)).toHaveAttribute("name", "trip-end");
  await boxes.nth(0).click();
  await expect(region).toBeVisible();
  const first = await firstInside(page, TRIP);
  await press(page, TRIP, first);
  // HALF A RANGE IS NOT AN ANSWER, so the month stays and the second box is
  // the one now waiting.
  await expect(region).toBeVisible();
  await expect(boxes.nth(1)).toHaveValue("");
  await expect(page.locator(`${TRIP} .nd-date-picker-open[aria-expanded="true"]`)).toHaveAttribute(
    "data-nd-open",
    "end",
  );
  await press(page, TRIP, shift(first, 5));
  await expect(region).toBeHidden();
  const [, month, at] = first.split("-");
  await expect(boxes.nth(0)).toHaveValue(`${at}/${month}/2026`);
  expect(await page.locator(`${TRIP} [data-nd-in-range]`).count()).toBe(6);
  await context.close();
});

test("a scroll does not throw the month away, and the day still lands", async ({ browser }) => {
  // THE DEFECT THAT WAS REPORTED, AND THE REASON NOTHING CAUGHT IT. Every other
  // test here presses the box and presses a day with nothing in between, and a
  // reader does not work that way: the calendar is three hundred pixels tall,
  // so they scroll to see the rest of it. `anchor.js` closes a panel whose
  // control has scrolled off the screen — right for a menu, wrong for a month —
  // and the calendar vanished with the box still empty.
  const { page, context } = await reader(browser, "en-GB");
  const box = page.locator(`${FIELD} .nd-date-picker-entry`);
  const region = page.locator(`${FIELD} .nd-date-picker-region`);
  await box.click();
  await expect(region).toBeVisible();
  await page.mouse.wheel(0, 600);
  await page.waitForTimeout(250);
  expect(await region.isVisible(), "the month was discarded by a scroll").toBe(true);
  // And it is still a working calendar afterwards.
  const day = await page
    .locator(`${FIELD} .nd-date-picker-cell:not([data-nd-outside])`)
    .first()
    .getAttribute("data-nd-day");
  await page.locator(`${FIELD} .nd-date-picker-cell[data-nd-day="${day}"]`).click();
  const [year, month, at] = day.split("-");
  await expect(box).toHaveValue(`${at}/${month}/${year}`);
  await context.close();
});

test("pressing the box leaves the caret in it, so a reader can still type", async ({ browser }) => {
  // OPENING USED TO TAKE THE CARET INTO THE GRID. A reader presses the box
  // meaning to type into it, and the next thing they typed went into a clipped
  // radio and nowhere a person could see.
  const { page, context } = await reader(browser, "en-GB");
  const box = page.locator(`${FIELD} .nd-date-picker-entry`);
  await box.click();
  await expect(page.locator(`${FIELD} .nd-date-picker-region`)).toBeVisible();
  await expect(box, "opening the month took the caret out of the box").toBeFocused();
  await page.keyboard.type("18/08/2026");
  await expect(box).toHaveValue("18/08/2026");
  // And pressing a day with a pointer leaves it there too.
  const day = await page
    .locator(`${FIELD} .nd-date-picker-cell:not([data-nd-outside])`)
    .first()
    .getAttribute("data-nd-day");
  await page.locator(`${FIELD} .nd-date-picker-cell[data-nd-day="${day}"]`).click();
  await expect(box).toBeFocused();
  await context.close();
});

/** Where a picker's two boxes are, and what their corners do. */
const arrangement = (page, root) =>
  page.locator(`${root} .nd-date-picker-box`).evaluateAll((all) =>
    all.map((one) => {
      const box = one.getBoundingClientRect();
      const style = getComputedStyle(one);
      return {
        left: Math.round(box.left),
        top: Math.round(box.top),
        right: Math.round(box.right),
        bottom: Math.round(box.bottom),
        width: Math.round(box.width),
        corners: [style.borderStartStartRadius, style.borderStartEndRadius].map((each) =>
          Math.round(Number.parseFloat(each)),
        ),
        blockCorners: [style.borderStartStartRadius, style.borderEndStartRadius].map((each) =>
          Math.round(Number.parseFloat(each)),
        ),
      };
    }),
  );

test("an attached pair shares one outline and one line between them", async ({ page }) => {
  await pickers(page);
  const [start, end] = await arrangement(page, ATTACHED);
  expect(start.top, "the two boxes are not on one row").toBe(end.top);
  // ONE LINE, NOT TWO. The boxes overlap by exactly the width of a hairline, so
  // the two adjacent edges are the single edge a reader sees.
  const overlap = start.right - end.left;
  expect(overlap, `the boxes are ${overlap}px apart rather than sharing an edge`).toBeGreaterThan(0);
  expect(overlap).toBeLessThanOrEqual(3);
  // The outer corners are the pair's and the inner ones are squared off.
  expect(start.corners[0], "the pair lost its leading corner").toBeGreaterThan(0);
  expect(start.corners[1], "the first box kept an inner corner").toBe(0);
  expect(end.corners[0], "the second box kept an inner corner").toBe(0);
  expect(end.corners[1], "the pair lost its trailing corner").toBeGreaterThan(0);
});

test("a stacked pair shares one outline down a column", async ({ page }) => {
  await pickers(page);
  const [start, end] = await arrangement(page, STACKED);
  expect(end.top, "the second box did not go under the first").toBeGreaterThan(start.top);
  expect(end.left, "the two boxes do not share a leading edge").toBe(start.left);
  expect(end.width, "the two boxes are different widths").toBe(start.width);
  // ONE LINE, NOT TWO, on the block axis exactly as the row does it on the
  // inline one. A column that merely stacked two boxes with a gap would be two
  // fields under one question rather than one control asking for both ends.
  const overlap = start.bottom - end.top;
  expect(overlap, `the boxes are ${overlap}px apart rather than sharing an edge`).toBeGreaterThan(0);
  expect(overlap).toBeLessThanOrEqual(3);
  expect(start.blockCorners[0], "the pair lost its top corner").toBeGreaterThan(0);
  expect(start.blockCorners[1], "the first box kept an inner corner").toBe(0);
  expect(end.blockCorners[0], "the second box kept an inner corner").toBe(0);
  expect(end.blockCorners[1], "the pair lost its bottom corner").toBeGreaterThan(0);
});

test("a hidden label is clipped, never dropped", async ({ page }) => {
  await pickers(page);
  const labels = page.locator(`${STACKED} .nd-date-picker-field-label`);
  // IT IS STILL A `<label for>` AND IT STILL NAMES THE BOX. A box a screen
  // reader reads as unnamed is a box nobody can answer, so this is `Table`'s
  // `hide_caption` and not a label somebody deleted.
  await expect(labels).toHaveCount(2);
  const bound = await labels.evaluateAll((all) =>
    all.map((one) => ({
      text: one.textContent.trim(),
      names: !!document.getElementById(one.htmlFor),
      drawn: one.getBoundingClientRect().width > 1,
    })),
  );
  for (const one of bound) {
    expect(one.text.length, "a hidden label has nothing to say").toBeGreaterThan(0);
    expect(one.names, `"${one.text}" points at no box`).toBe(true);
    expect(one.drawn, `"${one.text}" is still drawn`).toBe(false);
  }
});

test("an embedded label rides inside its own box", async ({ page }) => {
  await pickers(page);
  const labels = page.locator(`${INSIDE} .nd-date-picker-field-label`);
  await expect(labels).toHaveCount(2);
  const placed = await labels.evaluateAll((all) =>
    all.map((one) => ({
      inBox: !!one.closest(".nd-date-picker-box"),
      drawn: one.getBoundingClientRect().width > 1,
      beforeTheTyping:
        one.compareDocumentPosition(one.closest(".nd-date-picker-box").querySelector("input")) ===
        Node.DOCUMENT_POSITION_FOLLOWING,
    })),
  );
  for (const one of placed) {
    expect(one.inBox, "the label is not inside the box").toBe(true);
    expect(one.drawn, "the embedded label is not drawn").toBe(true);
    expect(one.beforeTheTyping, "the label comes after the typing").toBe(true);
  }
  // And the pair is still one outline, which is the whole reason it moved in.
  const [start, end] = await arrangement(page, INSIDE);
  expect(start.bottom - end.top).toBeGreaterThan(0);
});

test("both arrangements are still one picker with one month", async ({ browser }) => {
  const { page, context } = await reader(browser, "en-GB");
  for (const root of [ATTACHED, STACKED]) {
    const boxes = page.locator(`${root} .nd-date-picker-entry`);
    await expect(boxes).toHaveCount(2);
    // ONE MONTH SERVES BOTH BOXES, whichever way the pair sits.
    expect(await page.locator(`${root} .nd-date-picker-region`).count()).toBe(1);
    await boxes.nth(0).click();
    const region = page.locator(`${root} .nd-date-picker-region`);
    await expect(region).toBeVisible();
    const first = await firstInside(page, root);
    await press(page, root, first);
    await expect(region, "a half-made range closed the month").toBeVisible();
    await press(page, root, shift(first, 4));
    await expect(region).toBeHidden();
    const [, month, at] = first.split("-");
    await expect(boxes.nth(0)).toHaveValue(`${at}/${month}/2026`);
    expect(await ticked(page, root)).toHaveLength(2);
  }
  await context.close();
});

test("Escape closes the month and puts the focus back in the box", async ({ page }) => {
  await pickers(page);
  const box = page.locator(`${FIELD} .nd-date-picker-entry`);
  await box.click();
  await page.locator(`${FIELD} .nd-date-picker-input[tabindex="0"]`).focus();
  await page.keyboard.press("Escape");
  await expect(page.locator(`${FIELD} .nd-date-picker-region`)).toBeHidden();
  await expect(box).toBeFocused();
});

test("an inline picker claims no popup it cannot open", async ({ page }) => {
  await pickers(page);
  expect(await page.locator(`${VISIT} [aria-expanded]`).count()).toBe(0);
  expect(await page.locator(`${VISIT} .nd-date-picker-entry`).count()).toBe(0);
  const closers = await page
    .locator(`${VISIT} .nd-date-picker-close`)
    .evaluateAll((all) => all.filter((one) => !one.hidden).length);
  expect(closers, "a control that closes nothing was revealed").toBe(0);
});

test("the empty box shows the whole of what it is asking for", async ({ page }) => {
  // A HINT CLIPPED TO `DD/MM/` IS A HINT THAT TEACHES THE WRONG FORMAT, and it
  // is what shipped: the box was sized in `ch`, which is the width of a zero,
  // while the hint is letters. Measured against the box's own font rather than
  // guessed at, so the assertion survives a change of typeface.
  let swept = 0;
  for (const locale of OFFERED) {
    await pickers(page, locale);
    for (const root of [FIELD, TRIP]) {
      const boxes = page.locator(`${root} .nd-date-picker-entry`);
      const count = await boxes.count();
      for (let at = 0; at < count; at += 1) {
        const fits = await boxes.nth(at).evaluate((box) => {
          const style = getComputedStyle(box);
          const pen = document.createElement("canvas").getContext("2d");
          pen.font = `${style.fontWeight} ${style.fontSize} ${style.fontFamily}`;
          const room =
            box.getBoundingClientRect().width -
            Number.parseFloat(style.paddingLeft) -
            Number.parseFloat(style.paddingRight);
          return { hint: box.placeholder, needs: pen.measureText(box.placeholder).width, room };
        });
        expect(
          fits.room,
          `"${fits.hint}" needs ${fits.needs.toFixed(1)}px and has ${fits.room.toFixed(1)}px`,
        ).toBeGreaterThanOrEqual(fits.needs);
        swept += 1;
      }
    }
  }
  expect(swept, "no box was measured").toBeGreaterThan(4);
});

// --- With the script blocked ------------------------------------------------

test.describe("with the script blocked", () => {
  test.use({ javaScriptEnabled: false });

  test("it is smaller, not broken, and promises nothing it cannot keep", async ({ page }) => {
    await page.goto(url(PAGE.datePicker));
    // THE POSITIVE FLOOR FIRST. Without it this whole test passes on a page
    // that rendered an empty box, which is the shape §19.5 calls a sweep with
    // no floor.
    const days = await page.locator(`${VISIT} .nd-date-picker-input`).count();
    expect(days, "no month was drawn").toBe(42);
    expect(await page.locator(`${VISIT} .nd-date-picker-input[name]`).count()).toBe(42);
    // And it claims none of what only a script can answer.
    for (const claim of [
      '[role="grid"]',
      "[aria-expanded]",
      "[tabindex]",
      "[data-nd-phase]",
      "[data-nd-preview]",
      ".nd-date-picker-trigger",
    ]) {
      expect(
        await page.locator(`${VISIT} ${claim}`).count(),
        `${claim} is promised with no script`,
      ).toBe(0);
    }
    // The controls a script would drive stay hidden.
    const shown = await page
      .locator(`${VISIT} .nd-date-picker-control`)
      .evaluateAll((all) => all.filter((one) => !one.hidden).length);
    expect(shown, "a control that does nothing was drawn").toBe(0);
  });

  test("a day is chosen and submitted by the browser alone", async ({ page }) => {
    await page.goto(url(PAGE.datePicker));
    // The folds are `<details>`, which a reader opens with no script at all.
    // Opening them is setup rather than the assertion.
    await showParts(page);
    const first = await page
      .locator(`${VISIT} .nd-date-picker-input`)
      .evaluateAll((all) => all.find((one) => !one.disabled).value);
    await page.locator(`${VISIT} .nd-date-picker-cell[data-nd-day="${first}"]`).click();
    expect(await ticked(page, VISIT)).toEqual([first]);
    // A radio group is one tab stop and its arrows are the platform's own.
    expect(
      await page.locator(`${VISIT} .nd-date-picker-input[type="radio"]`).count(),
      "a single-date picker did not draw radios",
    ).toBe(42);
  });

  test("a box is a box, and the month behind it is simply not there", async ({ page }) => {
    await page.goto(url(PAGE.datePicker));
    await showParts(page);
    // THE FLOOR FIRST. Everything below is an absence, and a picker that
    // rendered nothing satisfies every absence there is.
    const box = page.locator(`${FIELD} .nd-date-picker-entry`);
    await expect(box).toHaveCount(1);
    await expect(box).toHaveAttribute("name", "arrival");
    // A reader can type into it, and what they type is what submits.
    await box.fill("18/08/2026");
    await expect(box).toHaveValue("18/08/2026");
    // The month is served, hidden, claiming nothing, and posting nothing.
    await expect(page.locator(`${FIELD} .nd-date-picker-region`)).toBeHidden();
    expect(await page.locator(`${FIELD} .nd-date-picker-input[name]`).count()).toBe(0);
    for (const claim of ['[role="grid"]', '[role="dialog"]', "[aria-expanded]"]) {
      expect(
        await page.locator(`${FIELD} ${claim}`).count(),
        `${claim} is promised with no script`,
      ).toBe(0);
    }
    // And the control that opens it is not drawn, because it opens nothing.
    const openers = await page
      .locator(`${FIELD} .nd-date-picker-open`)
      .evaluateAll((all) => all.filter((one) => !one.hidden).length);
    expect(openers, "a control that opens nothing was drawn").toBe(0);
  });

  test("a bound refuses its days with no script running", async ({ page }) => {
    await page.goto(url(PAGE.datePicker));
    await showParts(page);
    const refused = await page.locator(`${BOOKING} .nd-date-picker-input:disabled`).count();
    expect(refused, "a bound refused nothing without a script").toBeGreaterThan(0);
  });
});

// --- The reader's conventions, and the page's words -------------------------

const WEEKS = [
  ["en-GB", 1],
  ["en-US", 7],
  ["pt-BR", 7],
];

for (const [locale, first] of WEEKS) {
  test(`a reader whose browser says ${locale} starts the week on day ${first}`, async ({
    browser,
  }) => {
    // Forcing the locale is the whole point: it is the one thing the page
    // cannot influence, so a detection that reads the page cannot pass this.
    const context = await browser.newContext({ locale });
    const page = await context.newPage();
    await page.goto(url(PAGE.datePicker));
    await showParts(page);
    await expect(page.locator(VISIT)).toHaveAttribute("data-nd-week-start", String(first));
    const columns = await page
      .locator(`${VISIT} .nd-date-picker-weekday`)
      .evaluateAll((all) => all.map((one) => Number(one.dataset.ndWeekday)));
    expect(columns[0], "the columns disagree with the stamp").toBe(first);
    await context.close();
  });
}

test("the page's language decides the month name and not the week's first day", async ({
  browser,
}) => {
  // THE ONE TEST THAT CAN SEPARATE THE TWO CHAINS. A convention follows the
  // reader and a word follows the page, and reading one from the other is a
  // defect this repository has shipped once already.
  const context = await browser.newContext({ locale: "en-US" });
  const page = await context.newPage();
  await page.goto(url(PAGE.datePicker, "pt-BR"));
  await showParts(page);
  await expect(page.locator(VISIT)).toHaveAttribute("data-nd-week-start", "7");
  const caption = await page.locator(`${VISIT} .nd-date-picker-month`).textContent();
  // Asserted POSITIVELY against the Portuguese months, because "not January"
  // passes in February and says nothing.
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

test("a declared week start is never second-guessed", async ({ browser }) => {
  const context = await browser.newContext({ locale: "en-GB" });
  const page = await context.newPage();
  await page.goto(url(PAGE.datePicker));
  await showParts(page);
  // The three-month example asks for Sunday outright, and the reader's own
  // conventions say Monday. A named value is served as written.
  await expect(page.locator(TERM)).toHaveAttribute("data-nd-week-start", "7");
  await context.close();
});

test("every locale draws the month in its own words", async ({ page }) => {
  const seen = new Set();
  for (const locale of LOCALES) {
    await pickers(page, locale);
    seen.add(await page.locator(`${VISIT} .nd-date-picker-month`).textContent());
    const columns = await page
      .locator(`${VISIT} .nd-date-picker-weekday`)
      .allTextContents();
    expect(columns).toHaveLength(7);
  }
  expect(seen.size, "the month reads the same in every locale").toBe(LOCALES.length);
});

// --- Geometry and colour ----------------------------------------------------

test("every day clears the target size at the narrowest width", async ({ page }) => {
  test.slow();
  await page.setViewportSize({ width: 320, height: 720 });
  await pickers(page);
  let measured = 0;
  for (const root of [VISIT, STAY, BOOKING, TERM]) {
    // ONLY THE PANES A READER CAN SEE. Below the breakpoint the second and
    // third months are `display: none`, and a box of nothing is not a target
    // anybody has to hit.
    const boxes = await page.locator(`${root} .nd-date-picker-cell`).evaluateAll((all) =>
      all
        .filter((one) => one.offsetParent !== null)
        .map((one) => {
          const box = one.getBoundingClientRect();
          return [Math.round(box.width), Math.round(box.height)];
        }),
    );
    for (const [width, height] of boxes) {
      expect(width, `a day is ${width}px wide`).toBeGreaterThanOrEqual(24);
      expect(height, `a day is ${height}px tall`).toBeGreaterThanOrEqual(24);
      measured += 1;
    }
  }
  expect(measured, "no day was measured").toBeGreaterThan(100);
});

test("every state a day can be in reads against its own ground", async ({ browser }) => {
  test.slow();
  // AT THREE TIMES THE SCALE, AND THAT IS NOT A TRICK. `extremes` reports the
  // most extreme pixel a box actually paints, and a two-digit numeral at one
  // device pixel per CSS pixel never paints one: every stroke is antialiased
  // toward its own background, so the measurement is of the blend rather than
  // of the colours. At three the core of a stroke is the ink itself, which is
  // the pair the contrast rule is defined on. The page is the shipped one
  // either way.
  const context = await browser.newContext({ deviceScaleFactor: 3 });
  const page = await context.newPage();
  for (const mode of ["light", "dark"]) {
    await page.emulateMedia({ colorScheme: mode });
    await pickers(page);
    const seen = new Set();
    for (const [what, locator] of [
      ["start", page.locator(`${STAY} .nd-date-picker-cell[data-nd-in-range="start"]`).first()],
      ["end", page.locator(`${STAY} .nd-date-picker-cell[data-nd-in-range="end"]`).first()],
      ["between", page.locator(`${STAY} .nd-date-picker-cell[data-nd-in-range="between"]`).first()],
      ["today", page.locator(`${VISIT} .nd-date-picker-cell[aria-current="date"]`).first()],
      ["outside", page.locator(`${VISIT} .nd-date-picker-cell[data-nd-outside="true"]`).first()],
      ["plain", page.locator(`${VISIT} .nd-date-picker-cell`).nth(20)],
    ]) {
      await locator.scrollIntoViewIfNeeded();
      const { min, max } = await extremes(page, await locator.screenshot());
      expect(
        contrast(min, max),
        `${what} in ${mode} reads at ${contrast(min, max).toFixed(2)}:1`,
      ).toBeGreaterThanOrEqual(4.5);
      seen.add(what);
    }
    // A STATE PRESENT NOWHERE IS A STATE THIS SWEEP SILENTLY SKIPPED, and a
    // sweep with no floor is the shape that passes on an empty selector.
    expect(seen.size, `only ${seen.size} states were measured in ${mode}`).toBe(6);
  }
  await context.close();
});

test("a refused day is never also a chosen one", async ({ page }) => {
  await pickers(page);
  // Asserted rather than believed. The two are meant to be impossible together
  // by construction, and "by construction" is a claim like any other.
  let examined = 0;
  for (const root of [VISIT, STAY, BOOKING, FIELD, TERM]) {
    const clashes = await page.locator(`${root} .nd-date-picker-input`).evaluateAll((all) => {
      return all.filter((one) => one.disabled && one.checked).map((one) => one.value);
    });
    expect(clashes, `a refused day is ticked in ${root}`).toEqual([]);
    examined += await page.locator(`${root} .nd-date-picker-input`).count();
  }
  expect(examined, "no day was examined").toBeGreaterThan(100);
});

test("every day in a month is the same size as every other", async ({ page }) => {
  // THE GUARD THE UNEVEN COLUMNS GOT PAST. A table laid out automatically sizes
  // each column to its widest cell, and the widest cell is the weekday heading:
  // seven columns came out between 25 and 36px in Portuguese and between 32 and
  // 49 in the pseudo-locale. Nothing asked, because every other measurement
  // here is of one cell rather than of one cell against its neighbours.
  //
  // It also catches the other way a cell can be the odd one out: today carried
  // a border no other day did, so today's row was three pixels taller than the
  // five around it.
  let swept = 0;
  for (const locale of LOCALES) {
    await pickers(page, locale);
    for (const root of [VISIT, STAY, BOOKING, TERM, SMALL, LARGE]) {
      const panes = await page.locator(`${root} .nd-date-picker-pane`).count();
      for (let at = 0; at < panes; at += 1) {
        const boxes = await page
          .locator(`${root} .nd-date-picker-pane`)
          .nth(at)
          .locator(".nd-date-picker-cell")
          .evaluateAll((all) =>
            all
              .filter((one) => one.offsetParent !== null)
              .map((one) => {
                const box = one.getBoundingClientRect();
                return [Math.round(box.width), Math.round(box.height)];
              }),
          );
        if (boxes.length === 0) continue;
        const widths = new Set(boxes.map(([width]) => width));
        const heights = new Set(boxes.map(([, height]) => height));
        expect(
          widths.size,
          `${root} in ${locale} draws days at ${[...widths].join(", ")}px wide`,
        ).toBe(1);
        expect(
          heights.size,
          `${root} in ${locale} draws days at ${[...heights].join(", ")}px tall`,
        ).toBe(1);
        swept += boxes.length;
      }
    }
  }
  expect(swept, "no day was measured").toBeGreaterThan(400);
});

test("the band is visible against a day outside it", async ({ browser }) => {
  test.slow();
  // THE QUESTION THE OTHER CONTRAST TEST CANNOT ASK. That one screenshots one
  // cell and reads its own extremes, which is the numeral against the fill
  // under it — and a fill that is not there at all still gives a numeral on the
  // page's own surface at 15:1. So the band shipped at 1.06:1 against the days
  // around it, which is a range a reader has to hunt for.
  //
  // The palette has no stronger tint that keeps a numeral legible in both
  // appearances, so the measured ceiling here is 2.47:1 in dark and 1.81:1 in
  // light. That is a recorded gap, not a target: the floor below is set to
  // catch a return to the invisible tint, and the two ends carry a ring and a
  // weight so the range is a shape as well as a colour.
  const context = await browser.newContext({ deviceScaleFactor: 2 });
  const page = await context.newPage();
  for (const mode of ["light", "dark"]) {
    await page.emulateMedia({ colorScheme: mode });
    await pickers(page);
    const band = page.locator(`${STAY} .nd-date-picker-cell[data-nd-in-range="between"]`).first();
    const plain = page.locator(`${VISIT} .nd-date-picker-cell:not([data-nd-in-range])`).nth(20);
    await band.scrollIntoViewIfNeeded();
    // A CORNER, NOT THE MIDDLE. The middle of a day is its numeral, and this is
    // a question about the fill the numeral sits on.
    const inside = await pixelAt(page, await band.screenshot(), 3, 3);
    await plain.scrollIntoViewIfNeeded();
    const outside = await pixelAt(page, await plain.screenshot(), 3, 3);
    expect(
      contrast(inside, outside),
      `in ${mode} the band reads at ${contrast(inside, outside).toFixed(2)}:1 against a day outside it`,
    ).toBeGreaterThan(1.5);
  }
  await context.close();
});

// --- The arithmetic these assertions lean on --------------------------------

/** How many days from one wire-format day to another. */
function days(from, to) {
  return Math.round((Date.parse(`${to}T00:00:00Z`) - Date.parse(`${from}T00:00:00Z`)) / 86400000);
}

/** A wire-format day this many days along. */
function shift(day, by) {
  const at = new Date(Date.parse(`${day}T00:00:00Z`));
  at.setUTCDate(at.getUTCDate() + by);
  return at.toISOString().slice(0, 10);
}

/** The first day of the month a picker is actually showing, and not a spill. */
async function firstInside(page, root) {
  return page.locator(`${root} .nd-date-picker-cell`).evaluateAll((all) => {
    const inside = all.filter((one) => one.dataset.ndOutside !== "true");
    return inside[7].dataset.ndDay;
  });
}
