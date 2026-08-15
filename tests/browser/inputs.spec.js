// The seven input controls, and the claims only a browser can settle.
//
// Their behaviour SHIPS — `dist/inputs.js`, like `ColorPicker`'s — so what is
// asserted here is asserted about a file a consumer of the packages receives.

import { expect, test } from "@playwright/test";

import { LOCALES, PAGE, showParts, url } from "./pages.js";
import { extremes } from "./pixels.js";

/** Every page a control of this family is documented on.
 *
 * The four that predate `Field` are on it too, now that they use it: the
 * describedby check below is the only thing standing between the field's ids
 * and the control's, and it has already caught one component pointing at ids
 * the field never drew.
 */
const PAGES = [
  PAGE.select,
  PAGE.tagPicker,
  PAGE.dateInput,
  PAGE.colorPicker,
  PAGE.field,
  PAGE.input,
  PAGE.textarea,
  PAGE.numberInput,
  PAGE.slider,
  PAGE.fileInput,
  PAGE.timeInput,
];

// --- What `Field` promises, on every page that uses it ----------------------

test("every describedby points at something, on every page, in every locale", async ({ page }) => {
  // THE ONE PIECE OF DUPLICATION THE EXTRACTION COULD NOT REMOVE. `Field`
  // derives `{control}-hint` and `{control}-error`; the control computes the
  // same two strings for its own `aria-describedby`, because a component
  // cannot reach into its children to write an attribute on one of them.
  //
  // It had already gone wrong: `TimeInput` pointed at `{id}-hint` while the
  // field drew `{id}-hour-hint`, so every sentence under one was described by
  // an element that did not exist. Nothing reports that — not a browser, not a
  // validator — which is why this is a test and not a comment asking for care.
  for (const locale of LOCALES) {
    for (const slug of PAGES) {
      await page.goto(url(slug, locale));
      const dangling = await page.evaluate(() => {
        const out = [];
        for (const element of document.querySelectorAll("[aria-describedby]")) {
          for (const id of element.getAttribute("aria-describedby").split(/\s+/).filter(Boolean)) {
            if (!document.getElementById(id)) out.push(id);
          }
        }
        return out;
      });
      expect(dangling, `${locale}/${slug}`).toEqual([]);
    }
  }
});

test("every label points at a control that exists", async ({ page }) => {
  for (const slug of PAGES) {
    await page.goto(url(slug));
    const dangling = await page.evaluate(() =>
      [...document.querySelectorAll("label[for]")]
        .map((one) => one.getAttribute("for"))
        .filter((id) => !document.getElementById(id)),
    );
    expect(dangling, slug).toEqual([]);
  }
});

test("a group is a fieldset with a legend, and a single field is not", async ({ page }) => {
  await page.goto(url(PAGE.timeInput));
  // A `<label>` points at ONE element, so three boxes under one would be three
  // boxes with one of them named.
  const group = page.locator("fieldset.nd-field").first();
  await expect(group).toBeVisible();
  await expect(group.locator("legend.nd-label").first()).toBeVisible();

  await page.goto(url(PAGE.input));
  await expect(page.locator("fieldset.nd-field")).toHaveCount(0);
  await expect(page.locator("div.nd-field label.nd-label").first()).toBeVisible();
});

test("the required mark is a word, not an asterisk", async ({ page }) => {
  await page.goto(url(PAGE.field));
  const mark = page.locator(".nd-label-mark").first();
  await expect(mark).toBeVisible();
  // An asterisk means *required* only to a reader who already knows it does,
  // needs a legend elsewhere on the page, and reaches nobody who cannot see
  // it. The word is inside the label, so it is announced with the question.
  expect((await mark.textContent()).trim()).not.toBe("*");
  expect(await mark.evaluate((el) => el.closest("label, legend") !== null)).toBe(true);
});

// --- Input ---------------------------------------------------------------

test("with the script blocked it is an input that posts a value", async ({ browser }) => {
  const context = await browser.newContext({ javaScriptEnabled: false });
  const page = await context.newPage();
  await page.goto(url(PAGE.input));

  const box = page.locator('[data-nd-input="who"] .nd-input-box');
  await expect(box).toBeVisible();
  expect(await box.getAttribute("name")).toBe("who");
  expect(await box.getAttribute("required")).not.toBeNull();
  // And nothing claims what it cannot keep: the two controls the script owns
  // are not there at all.
  await expect(page.locator("[data-nd-input-clear]").first()).toBeHidden();
  await expect(page.locator("[data-nd-input-reveal]").first()).toBeHidden();

  // AND IT POSTS, WHICH IS THE HALF THE TITLE NAMES AND NOTHING ASKED. Every
  // line above reads an attribute off the markup, so a box the form could not
  // carry passed. The example is a real `get` form, so it is submitted here.
  // The value is read back out of the address the browser went to.
  await box.fill("Grace Hopper");
  await page.locator('form:has([data-nd-input="who"]) button[type="submit"]').click();
  await page.waitForURL(/\?/);
  expect(new URL(page.url()).searchParams.get("who")).toBe("Grace Hopper");
  await context.close();
});

test("the clear control appears with the text and moves nothing", async ({ page }) => {
  await page.goto(url(PAGE.input));
  const box = page.locator('[data-nd-input="handle"] .nd-input-box');
  const clear = page.locator('[data-nd-input="handle"] [data-nd-input-clear]');
  // A control that is there when the box is empty is a control that does
  // nothing, so it appears with the text and nowhere else.
  await expect(clear).toBeHidden();
  // AND THE BOX DOES NOT MOVE. This test used to repeat `clearable`'s own
  // self-contradicting sentence — "a control that appears as a reader types is
  // one that moves the layout under their hand" — while asserting exactly that
  // arrangement. The control was `display: none`, so the box widened on the
  // first keystroke. Both widths are read, and they are the same width.
  const width = () => box.evaluate((el) => Math.round(el.getBoundingClientRect().width));
  const empty = await width();
  await box.fill("ada");
  await expect(clear).toBeVisible();
  expect(await width()).toBe(empty);
  await clear.click();
  expect(await box.inputValue()).toBe("");
  // Focus goes back to the box, because emptying a field is something a reader
  // does in order to type in it.
  expect(await box.evaluate((el) => el === document.activeElement)).toBe(true);
});

test("Escape clears, and does nothing at all when there is nothing to clear", async ({ page }) => {
  await page.goto(url(PAGE.input));
  const box = page.locator('[data-nd-input="handle"] .nd-input-box');
  await box.fill("something");
  await box.press("Escape");
  expect(await box.inputValue()).toBe("");
  // Swallowing the key when it has nothing to do would take Escape away from
  // whatever else on the page wanted it.
  await box.press("Escape");
  expect(await box.inputValue()).toBe("");
});

test("the counter counts, and does not speak on every keystroke", async ({ page }) => {
  await page.goto(url(PAGE.input));
  const counter = page.locator('[data-nd-input="handle"]')
    .locator("xpath=..")
    .locator("[data-nd-input-counter]");
  const box = page.locator('[data-nd-input="handle"] .nd-input-box');
  await box.fill("ada");
  await expect(counter).toContainText("3");
  await expect(counter).toContainText("24");
  // A live region that speaks per character is a live region a reader turns
  // off: it goes quiet while they type and polite once they stop.
  expect(await counter.getAttribute("aria-live")).toBe("off");
  await expect(counter).toHaveAttribute("aria-live", "polite", { timeout: 3000 });
});

test("a password can be shown, and the control renames itself", async ({ page }) => {
  await page.goto(url(PAGE.input));
  const box = page.locator('[data-nd-input="secret"] .nd-input-box');
  const reveal = page.locator('[data-nd-input="secret"] [data-nd-input-reveal]');
  expect(await box.getAttribute("type")).toBe("password");
  const before = await reveal.getAttribute("aria-label");

  await reveal.click();
  expect(await box.getAttribute("type")).toBe("text");
  expect(await reveal.getAttribute("aria-pressed")).toBe("true");
  // A control that says *show password* while the password is showing has
  // lied about a state the reader cannot see.
  expect(await reveal.getAttribute("aria-label")).not.toBe(before);
});

// --- Textarea ----------------------------------------------------------------

test("an autosizing box grows with its content and stops at its ceiling", async ({ page }) => {
  await page.goto(url(PAGE.textarea));
  const box = page.locator('[data-nd-textarea="note"]');
  const start = (await box.boundingBox()).height;
  await box.fill("one\ntwo\nthree\nfour\nfive");
  const grown = (await box.boundingBox()).height;
  expect(grown).toBeGreaterThan(start);

  // A box with no ceiling pushes the submit control somewhere nobody will
  // find it, so `max_rows` is where the growing stops and the scrolling
  // starts.
  await box.fill(Array.from({ length: 40 }, (_, i) => `line ${i}`).join("\n"));
  const capped = (await box.boundingBox()).height;
  expect(await box.evaluate((el) => el.scrollHeight > el.clientHeight)).toBe(true);
  expect(capped).toBeLessThan(2000);
});

// --- NumberInput -------------------------------------------------------------

test("it says it is a spinbutton, and shows no stepper, until a script can spin it", async ({
  browser,
}) => {
  const bare = await browser.newContext({ javaScriptEnabled: false });
  const still = await bare.newPage();
  await still.goto(url(PAGE.numberInput));
  const box = still.locator('[data-nd-number-input="tickets"] .nd-number-input-box');
  // A role claimed without its contract is a promise the markup does not keep.
  expect(await box.getAttribute("role")).toBeNull();
  // AND NEITHER ARE THE STEPPERS. They shipped visible and inert: two buttons
  // that move nothing are two buttons a reader presses twice and then reads
  // the value again to check. They ship `hidden`, like `Input`'s clear.
  await expect(
    still.locator('[data-nd-number-input="tickets"] [data-nd-number-input-steppers]'),
  ).toBeHidden();
  await bare.close();
});

test("what is not a number is refused by the markup, with a script or without", async ({
  browser,
  page,
}) => {
  const bare = await browser.newContext({ javaScriptEnabled: false });
  const still = await bare.newPage();
  await still.goto(url(PAGE.numberInput));
  // `min`, `max` and `step` are mirrored into `data-nd-*` for the script to
  // read, so with no script the `pattern` is the only thing between the form
  // and a value that is not a number at all.
  expect(
    await still.locator('[data-nd-number-input="tickets"] .nd-number-input-box').getAttribute("pattern"),
  ).toBeTruthy();
  await bare.close();

  await page.goto(url(PAGE.numberInput));
  const box = page.locator('[data-nd-number-input="tickets"] .nd-number-input-box');
  const validity = (value) =>
    box.evaluate((el, text) => {
      el.value = text;
      return el.checkValidity();
    }, value);
  expect(await validity("twelve")).toBe(false);
  expect(await validity("12")).toBe(true);
  // AND WHAT THE SCRIPT ITSELF DREW IS NEVER REFUSED. `group` writes the
  // page's own separators into the box, so a pattern that took digits alone
  // would fail every grouped field the moment it was submitted.
  const price = page.locator('[data-nd-number-input="price"] .nd-number-input-box');
  expect(await price.evaluate((el) => el.checkValidity())).toBe(true);
});

test("the keyboard a phone opens is the one the step asks for", async ({ page }) => {
  await page.goto(url(PAGE.numberInput));
  // A decimal key on a field counting tickets types a character the field
  // refuses. The contract used to say `numeric` while both implementations
  // wrote `decimal`, so neither could be right for both fields.
  expect(
    await page.locator('[data-nd-number-input="tickets"] .nd-number-input-box').getAttribute("inputmode"),
  ).toBe("numeric");
  expect(
    await page.locator('[data-nd-number-input="weight"] .nd-number-input-box').getAttribute("inputmode"),
  ).toBe("decimal");
});

test("the steppers and the arrows move it, and the bounds hold", async ({ page }) => {
  await page.goto(url(PAGE.numberInput));
  const root = '[data-nd-number-input="tickets"]';
  const box = page.locator(`${root} .nd-number-input-box`);
  expect(await box.getAttribute("role")).toBe("spinbutton");
  expect(await box.getAttribute("aria-valuemax")).toBe("20");
  // The script is what can answer for them, so the script is what shows them.
  await expect(page.locator(`${root} [data-nd-number-input-steppers]`)).toBeVisible();

  await page.locator(`${root} [data-nd-number-input-step="up"]`).click();
  expect(await box.inputValue()).toBe("3");
  await box.focus();
  await page.keyboard.press("PageUp");
  expect(await box.inputValue()).toBe("8");
  await page.keyboard.press("Home");
  expect(await box.inputValue()).toBe("1");
  // The floor holds against the stepper as well as against the keyboard.
  await page.locator(`${root} [data-nd-number-input-step="down"]`).click();
  expect(await box.inputValue()).toBe("1");
  await box.focus();
  await page.keyboard.press("End");
  expect(await box.inputValue()).toBe("20");
});

test("the steppers are not tab stops", async ({ page }) => {
  await page.goto(url(PAGE.numberInput));
  // Two extra stops per number field is a form that takes three times as many
  // presses to cross, and the arrow keys already do what they do.
  const stops = await page
    .locator('[data-nd-number-input="tickets"] [data-nd-number-input-step]')
    .evaluateAll((all) => all.map((one) => one.tabIndex));
  expect(stops).toEqual([-1, -1]);
});

test("a spinbutton carries its value before anything touches it", async ({ page }) => {
  await page.goto(url(PAGE.numberInput));
  // `aria-valuenow` was written inside the move, so every number field on the
  // site was a spinbutton with no current value until a reader moved it —
  // which is the one moment they did not need to be told what it holds.
  const tickets = page.locator('[data-nd-number-input="tickets"] .nd-number-input-box');
  expect(await tickets.getAttribute("aria-valuenow")).toBe("2");

  // AND WHAT IS ANNOUNCED IS WHAT IS DRAWN, where those differ. `Slider` has
  // carried an `aria-valuetext` since it shipped and this had none, so the
  // currency, the unit and the separators reached nobody using a screen
  // reader.
  const price = page.locator('[data-nd-number-input="price"] .nd-number-input-box');
  expect(await price.getAttribute("aria-valuenow")).toBe("1499.90");
  expect(await price.getAttribute("aria-valuetext")).toBe(`R$ ${await price.inputValue()}`);
  const weight = page.locator('[data-nd-number-input="weight"] .nd-number-input-box');
  expect(await weight.getAttribute("aria-valuenow")).toBe("4.5");
  expect(await weight.getAttribute("aria-valuetext")).toBe("4.5 kg");
});

test("grouping is drawn at rest, and the form posts the plain number", async ({ page }) => {
  await page.goto(url(PAGE.numberInput));
  const box = page.locator('[data-nd-number-input="price"] .nd-number-input-box');
  // AT REST, NOT ONCE TOUCHED. Formatting only ever ran on focus and blur, so
  // a grouped field rendered the raw `1499.9` until a reader entered it.
  expect(await box.inputValue()).not.toBe("1499.9");
  expect(await box.inputValue()).toMatch(/^1[.,]499[.,]90$/);

  // Never submitted, which is what the contract promises: the separators are
  // a drawing, and the entry the form posts is the number.
  const posted = await box.evaluate((el) => new FormData(el.form).get(el.name));
  expect(posted).toBe("1499.90");

  // And they are gone while the caret is in the box, so no separator is a
  // character the reader has to delete.
  await box.focus();
  expect(await box.inputValue()).toBe("1499.90");

  // A press moves the value the box is showing. What the script drew cannot
  // be read back out of it — `1,499.90` is `NaN` to `Number` — so a stepper
  // that parsed the box would have fallen back to `min` on every grouped
  // field on the site.
  await page.locator('[data-nd-number-input="price"] [data-nd-number-input-step="up"]').click();
  expect(await box.inputValue()).toBe("1499.91");
});

test("focusing a box does not rewrite what it holds", async ({ page }) => {
  await page.goto(url(PAGE.numberInput));
  const box = page.locator('[data-nd-number-input="tickets"] .nd-number-input-box');
  // `step` IS ONE HERE, AND 7.5 IS STILL 7.5. Focus rounded the value to the
  // decimals `step` implied, so clicking into a box turned an answer into a
  // different answer before a key was pressed.
  await box.fill("7.5");
  await box.blur();
  expect(await box.inputValue()).toBe("7.5");
  await box.focus();
  expect(await box.inputValue()).toBe("7.5");
});

test("a value out of range is refused with a sentence, and not rewritten", async ({ page }) => {
  await page.goto(url(PAGE.numberInput));
  const root = '[data-nd-number-input="boxes"]';
  const box = page.locator(`${root} .nd-number-input-box`);
  const said = page.locator(`.nd-field:has(${root}) [data-nd-field-error] [data-nd-field-message]`);
  await box.fill("40");
  await box.blur();
  // REFUSED AND EXPLAINED, NEVER REWRITTEN — a control that changes a reader's
  // number to a different number has answered a question they did not ask.
  expect(await box.inputValue()).toBe("40");
  expect(await box.getAttribute("aria-invalid")).toBe("true");
  // AND THE SENTENCE IS THE POINT OF THE PROP. The slot arrived with
  // `required`, so this field marked itself wrong and said nothing at all —
  // and this test passed anyway, because it never asked what was said.
  await expect(said).toHaveText(/10/);
  await expect(said).toBeVisible();

  // A value that is not a number at all gets the third sentence, with what
  // was typed inside it.
  await box.fill("twelve");
  await box.blur();
  await expect(said).toHaveText(/twelve/);

  // And an accepted value takes the sentence away again.
  await box.fill("6");
  await box.blur();
  expect(await box.getAttribute("aria-invalid")).toBe("false");
  await expect(said).toHaveText("");
});

test("a read only field refuses its steppers and its arrow keys", async ({ page }) => {
  await page.goto(url(PAGE.numberInput));
  const root = '[data-nd-number-input="packed"]';
  const box = page.locator(`${root} .nd-number-input-box`);
  const up = page.locator(`${root} [data-nd-number-input-step="up"]`);
  // A value a press can still change is a value that is still editable, and
  // the contract says so. Both halves shipped taking `disabled` alone.
  await expect(up).toBeDisabled();
  const before = await box.inputValue();
  await up.click({ force: true });
  expect(await box.inputValue()).toBe(before);
  await box.focus();
  await page.keyboard.press("ArrowUp");
  expect(await box.inputValue()).toBe(before);
});

test("the mouse wheel is refused", async ({ page }) => {
  await page.goto(url(PAGE.numberInput));
  const box = page.locator('[data-nd-number-input="tickets"] .nd-number-input-box');
  await box.focus();
  const before = await box.inputValue();
  await box.hover();
  await page.mouse.wheel(0, 200);
  // A reader scrolling the page with the pointer resting here would otherwise
  // change an answer they had already given, silently.
  expect(await box.inputValue()).toBe(before);
});

test("no two number fields on the page ask the same question", async ({ page }) => {
  for (const locale of LOCALES) {
    await page.goto(url(PAGE.numberInput, locale));
    // THREE OF FOUR EXAMPLES WERE CALLED "How many", including a weight in
    // kilograms. An example is a component used, and a set of fields with one
    // label between them shows a page rather than a use.
    const questions = await page.evaluate(() =>
      [...document.querySelectorAll("[data-nd-number-input]")].map((one) =>
        one.closest(".nd-field")?.querySelector(".nd-label")?.textContent?.trim(),
      ),
    );
    expect(questions.length, locale).toBeGreaterThan(4);
    expect(questions.filter(Boolean).length, locale).toBe(questions.length);
    expect(new Set(questions).size, locale).toBe(questions.length);
  }
});

// --- Slider ------------------------------------------------------------------

test("the value is drawn beside the track and announced with its unit", async ({ page }) => {
  await page.goto(url(PAGE.slider));
  const thumb = page.locator('[data-nd-slider="volume"] [data-nd-slider-thumb="start"]');
  await expect(page.locator('[data-nd-slider="volume"] [data-nd-slider-output]')).toHaveText("35%");
  // `37` is a number; *37 per cent* is an answer — and the two must not say
  // different things about one value.
  expect(await thumb.getAttribute("aria-valuetext")).toBe("35%");
  await thumb.focus();
  await page.keyboard.press("ArrowRight");
  await expect(page.locator('[data-nd-slider="volume"] [data-nd-slider-output]')).toHaveText("40%");
});

test("a range is two named controls, and neither crosses the other", async ({ page }) => {
  await page.goto(url(PAGE.slider));
  const root = '[data-nd-slider="budget"]';
  const thumbs = page.locator(`${root} [data-nd-slider-thumb]`);
  await expect(thumbs).toHaveCount(2);
  // Two values under one name is one value a server reads twice.
  const names = await thumbs.evaluateAll((all) => all.map((one) => one.name));
  expect(names).toEqual(["budget-start", "budget-end"]);

  const reading = () => thumbs.evaluateAll((all) => all.map((one) => Number(one.value)));
  const [was, ceiling] = await reading();

  const start = thumbs.first();
  await start.focus();
  for (let press = 0; press < 30; press += 1) await page.keyboard.press("ArrowRight");
  const [low, high] = await reading();
  // WALKED, AND STOPPED AT THE OTHER THUMB. This asserted `low <= high`, which
  // was already true before a key was pressed. It stayed true of a thumb that
  // never moved, of a thumb nothing could focus, and of a thumb that shoved the
  // other one up the scale. All three are named below.
  expect(low, "the start thumb did not move at all").toBeGreaterThan(was);
  expect(low, "the start thumb walked past the end thumb").toBe(ceiling);
  expect(high, "the start thumb pushed the end thumb up the scale").toBe(ceiling);
});

// --- FileInput ---------------------------------------------------------------

test("the drop area is the label of a real file input", async ({ page }) => {
  await page.goto(url(PAGE.fileInput));
  const area = page.locator('[data-nd-file-input="one-attachment"] [data-nd-file-input-area]');
  const native = page.locator('[data-nd-file-input="one-attachment"] [data-nd-file-input-native]');
  // That one decision is what makes it focusable, pressable, announced and
  // usable by somebody who has never dragged a file.
  expect(await area.evaluate((el) => el.tagName)).toBe("LABEL");
  expect(await area.getAttribute("for")).toBe(await native.getAttribute("id"));
  // And the input is clipped rather than `display: none`, which would take it
  // out of the accessibility tree and out of the tab order.
  expect(await native.evaluate((el) => getComputedStyle(el).display)).not.toBe("none");

  // FOCUSED, RATHER THAN CALLED FOCUSABLE. Nothing here ever took the focus, so
  // the sentence above it was a claim no line under it made. The pairing is
  // read from the platform's own `labels` as well. Two strings that match are
  // not proof that the browser joined the two elements.
  await native.focus();
  expect(await native.evaluate((el) => el === document.activeElement)).toBe(true);
  expect(
    await native.evaluate((el) =>
      [...el.labels].some((one) => one.matches("[data-nd-file-input-area]")),
    ),
    "the browser does not read the area as this input's label",
  ).toBe(true);
});

test("what was chosen is listed, named, and can be taken off", async ({ page }) => {
  await page.goto(url(PAGE.fileInput));
  const root = '[data-nd-file-input="attachments"]';
  await page.locator(`${root} [data-nd-file-input-native]`).setInputFiles([
    { name: "report.pdf", mimeType: "application/pdf", buffer: Buffer.from("one") },
    { name: "photo.png", mimeType: "image/png", buffer: Buffer.from("two") },
  ]);
  const rows = page.locator(`${root} [data-nd-index]`);
  await expect(rows).toHaveCount(2);
  await expect(rows.first()).toContainText("report.pdf");
  // *3 files* is not something a reader can correct; a named row with a
  // control on it is.
  const remove = rows.first().locator('[data-nd-action="file-input-remove"]');
  expect(await remove.getAttribute("aria-label")).toContain("report.pdf");
  await remove.click();
  await expect(rows).toHaveCount(1);
  // The control that did it has just left the page.
  expect(await page.evaluate(() => document.activeElement?.tagName)).not.toBe("BODY");
});

test("a file of the wrong kind is refused with a sentence", async ({ page }) => {
  await page.goto(url(PAGE.fileInput));
  const root = '[data-nd-file-input="pictures"]';
  await page.locator(`${root} [data-nd-file-input-native]`).setInputFiles([
    { name: "notes.txt", mimeType: "text/plain", buffer: Buffer.from("no") },
  ]);
  // A drop that silently does nothing is the shape of failure the whole
  // contract is written against.
  await expect(page.locator(`${root} [data-nd-file-input-status]`)).toContainText("notes.txt");
  await expect(page.locator(`${root} [data-nd-index]`)).toHaveCount(0);
});

// --- TimeInput ---------------------------------------------------------------

test("it is served on a twenty-four hour clock and re-laid to the reader's", async ({
  browser,
  page,
}) => {
  // THE FIELD THAT TELLS THE TWO CLOCKS APART. This read `starts`, whose
  // `09:30` is `09:30` on either clock, and an attribute the runtime used to
  // write for every field on the page. Neither line could fail. An afternoon
  // reading can: it is 14 where the server keeps it and 02 with PM where the
  // reader was shown it.
  const root = '[data-nd-time-input="meeting"]';

  // What ships, with nothing running: the reading the server sent, whole, and
  // no meridiem control at all.
  const bare = await browser.newContext({ javaScriptEnabled: false });
  const still = await bare.newPage();
  await still.goto(url(PAGE.timeInput));
  const served = await still.locator(root).getAttribute("data-nd-value");
  const [hours, minutes] = served.split(":");
  expect(Number(hours), "no example is served an afternoon reading any more").toBeGreaterThan(12);
  await expect(still.locator(`${root} [data-nd-time-input-meridiem]`)).toBeHidden();
  await bare.close();

  // And what a reader on a twelve-hour clock is given.
  await page.goto(url(PAGE.timeInput));
  await expect(page.locator(root)).toHaveAttribute("data-nd-resolved", "true");
  expect(
    await page.locator(`${root} [data-nd-time-input-segment="hour"]`).inputValue(),
    "the hour box was left on the clock the server keeps",
  ).toBe(String(Number(hours) - 12).padStart(2, "0"));
  expect(await page.locator(`${root} [data-nd-time-input-segment="minute"]`).inputValue()).toBe(
    minutes,
  );
  // The half of the reading the boxes no longer carry is a control of its own,
  // with the right end of it chosen.
  const chosen = page.locator(`${root} .nd-time-input-meridiem-group [aria-checked="true"]`);
  await expect(chosen).toHaveCount(1);
  expect(await chosen.getAttribute("data-nd-value"), "the afternoon went missing").toBe("pm");
  // And the reading the server keeps is untouched by any of it.
  expect(await page.locator(root).getAttribute("data-nd-value")).toBe(served);
});

test("the boxes step, wrap, and carry the reader on", async ({ page }) => {
  await page.goto(url(PAGE.timeInput));
  const root = '[data-nd-time-input="starts"]';
  const hour = page.locator(`${root} [data-nd-time-input-segment="hour"]`);
  const minute = page.locator(`${root} [data-nd-time-input-segment="minute"]`);

  await hour.focus();
  await page.keyboard.press("ArrowUp");
  expect(await hour.inputValue()).toBe("10");

  // A full box carries on to the next one, so `0930` is four keystrokes and
  // not five.
  await hour.fill("");
  await hour.type("09");
  expect(await minute.evaluate((el) => el === document.activeElement)).toBe(true);
});

test("half a time is refused with the sentence the platform has none for", async ({ page }) => {
  await page.goto(url(PAGE.timeInput));
  const root = '[data-nd-time-input="booking"]';
  await page.locator(`${root} [data-nd-time-input-segment="hour"]`).fill("09");
  await page.locator(`${root} [data-nd-time-input-segment="minute"]`).fill("");
  await page.locator(`${root} [data-nd-time-input-segment="minute"]`).blur();
  // `required` on three boxes gives three identical bubbles, none of which
  // says *you gave me an hour and no minute*.
  await expect(page.locator(`${root} [data-nd-time-input-status]`)).toContainText("minute");
});

// --- The shell they sit in --------------------------------------------------

test("no id appears twice on any of the seven pages, in any locale", async ({ page }) => {
  for (const locale of LOCALES) {
    for (const slug of PAGES) {
      await page.goto(url(slug, locale));
      const twice = await page.evaluate(() => {
        const seen = new Map();
        for (const element of document.querySelectorAll("[id]")) {
          seen.set(element.id, (seen.get(element.id) ?? 0) + 1);
        }
        return [...seen].filter(([, count]) => count > 1).map(([id]) => id);
      });
      expect(twice, `${locale}/${slug}`).toEqual([]);
    }
  }
});

// --- What a wrong answer looks like -----------------------------------------

test("a wrong answer is marked by a rule down the whole question", async ({ page }) => {
  await page.goto(url(PAGE.field));
  const field = page.locator('.nd-field:has([data-nd-input="field-wrong"])');
  // The bar runs alongside the words, the hint, the sentence and the
  // control, so a reader scanning a long form finds the question rather than
  // the box.
  // MEASURED AGAINST THE TOKEN, not against a number. `--noctua-border-rule`
  // is a step on a scale, and a test that says `5px` is a test that has to be
  // edited the day the scale moves — which is the whole reason the scale
  // exists.
  const bar = await field.evaluate((el) => {
    const style = getComputedStyle(el);
    return {
      width: style.borderInlineStartWidth,
      token: getComputedStyle(document.documentElement)
        .getPropertyValue("--noctua-border-rule")
        .trim(),
      pad: style.paddingInlineStart,
    };
  });
  expect(bar.token).not.toBe("");
  expect(bar.width).toBe(bar.token);
  expect(Number.parseFloat(bar.pad)).toBeGreaterThan(0);
});

test("the question does not go red — only the sentence and the control do", async ({ page }) => {
  await page.goto(url(PAGE.field));
  const field = page.locator('.nd-field:has([data-nd-input="field-wrong"])');
  // A clip outside the viewport is not a screenshot, so the thing being read
  // has to be on screen before it can be read.
  await field.scrollIntoViewIfNeeded();

  // FROM PIXELS, WHICH IS THE ONLY WAY THIS REPOSITORY ALLOWS. A computed
  // `color` is whatever the engine felt like serialising; what a reader sees
  // is what got painted. `pixels.js` says so in its own header.
  const ink = async (selector) => {
    const box = await field.locator(selector).boundingBox();
    return (await extremes(page, await page.screenshot({ clip: box }))).min;
  };
  const label = await ink(".nd-label");
  const error = await ink(".nd-field-error");

  // The complaint is red — its darkest pixel is redder than it is green or
  // blue by a clear margin.
  expect(error[0] - error[1], "the sentence is not red").toBeGreaterThan(40);
  // A label the same colour as the complaint reads as part of the complaint,
  // and a reader scanning back up to find what they were asked reads past it.
  // Its darkest pixel is near-neutral: the three channels stay together.
  expect(Math.abs(label[0] - label[1]), "the question went red").toBeLessThan(20);
  expect(Math.abs(label[1] - label[2]), "the question went red").toBeLessThan(20);
});

test("a control the script refuses gets the mark too, not just a rendered error", async ({
  page,
}) => {
  await page.goto(url(PAGE.dateInput));
  const field = page.locator('.nd-field:has([data-nd-date-input="birth"])');
  const before = await field.evaluate((el) => getComputedStyle(el).borderInlineStartWidth);
  expect(before).toBe("0px");

  // THE 31ST OF APRIL. `data-nd-invalid` lands on the control the script
  // refused, not on the field around it — so a mark that only answered to a
  // server-rendered `error` would be missing exactly when a reader has just
  // typed something wrong.
  const box = (part) => `[data-nd-date-input="birth"] [data-nd-part="${part}"] input`;
  await page.locator(box("day")).fill("31");
  await page.locator(box("month")).fill("04");
  await page.locator(box("year")).fill("1815");
  await page.locator(box("year")).blur();

  const rule = await page.evaluate(() =>
    getComputedStyle(document.documentElement).getPropertyValue("--noctua-border-rule").trim(),
  );
  await expect
    .poll(() => field.evaluate((el) => getComputedStyle(el).borderInlineStartWidth))
    .toBe(rule);
});

// --- What the seven owe with no script at all -------------------------------

test("the slider draws its value and its scale before any script runs", async ({ browser }) => {
  // `output`'s own doc says it is on by default "so users can read the value
  // they have set", and the `<output>` shipped EMPTY: the single promise the
  // prop makes was the one thing it did not do without a script. The marks
  // were the same — five ticks a caller asked for, none of them drawn, under a
  // prop whose doc calls them decoration. Both are rendered now, and
  // `inputs.js` still rewrites the value on every move.
  const context = await browser.newContext({ javaScriptEnabled: false });
  const page = await context.newPage();
  await page.goto(url(PAGE.slider));

  await expect(page.locator('[data-nd-slider="volume"] [data-nd-slider-output]')).toHaveText("35%");
  const marks = page.locator('[data-nd-slider="marked"] .nd-slider-mark');
  expect(await marks.count(), "a slider asked for ticks and drew none").toBeGreaterThan(1);
  // The words the caller supplied, position by position against the values.
  expect((await marks.first().textContent()).trim().length).toBeGreaterThan(0);

  // A range says both ends, because half an answer is not the answer.
  const span = await page
    .locator('[data-nd-slider="budget"] [data-nd-slider-output]')
    .textContent();
  expect(span, "a range showed one bound or none").toMatch(/\d.*\d/);
  await context.close();
});

test("the counter says how much room is left before any script runs", async ({ browser }) => {
  // `maxlength` stops accepting characters in silence, and this sentence is
  // the only thing that says why. The `<p>` was rendered, put into
  // `aria-describedby`, and left blank — which is the exact failure the
  // `counter` prop's own doc exists to prevent.
  const context = await browser.newContext({ javaScriptEnabled: false });
  const page = await context.newPage();

  for (const [slug, hook] of [
    [PAGE.input, "[data-nd-input-counter]"],
    [PAGE.textarea, "[data-nd-textarea-counter]"],
  ]) {
    await page.goto(url(slug));
    const counter = page.locator(hook).first();
    await expect(counter).toBeAttached();
    const said = (await counter.textContent()).trim();
    expect(said.length, `${slug} rendered a counter with nothing in it`).toBeGreaterThan(0);
    // The limit is the half a reader cannot work out for themselves.
    const limit = await page
      .locator(`${hook.replace("-counter", "")}`)
      .first()
      .evaluate((el) => el.closest(".nd-field")?.querySelector("[maxlength]")?.maxLength ?? 0);
    if (limit > 0) expect(said, `${slug} does not say the limit`).toContain(String(limit));
  }
  await context.close();
});

test("the script keeps the counter right, and the sentence agrees with itself", async ({
  page,
}) => {
  // The other half: what is rendered is a starting point the script owns from
  // the first keystroke, and the plural form is the reader's language's.
  await page.goto(url(PAGE.input));
  const box = page.locator('[data-nd-input="handle"] [data-nd-input-box]');
  const counter = page.locator("[data-nd-input-counter]").first();
  const before = (await counter.textContent()).trim();
  await box.fill("ada");
  await expect(counter).not.toHaveText(before);
  expect((await counter.textContent()).trim()).toContain("3");
});

test("the now control is hidden until something can read the clock", async ({ browser }) => {
  // `NumberInput`'s steppers and `Input`'s clear and reveal controls are all
  // rendered hidden and revealed by `inputs.js`, and the comments beside them
  // cite the rule. This one shipped visible and inert: reading the clock is
  // the script's and nothing else's.
  const context = await browser.newContext({ javaScriptEnabled: false });
  const page = await context.newPage();
  await page.goto(url(PAGE.timeInput));
  const now = page.locator('[data-nd-action="time-input-now"]').first();
  await expect(now).toBeAttached();
  await expect(now, "a control that fills nothing is on the page").toBeHidden();
  await context.close();
});

test("the script reveals the now control, and it fills the boxes", async ({ page }) => {
  await page.goto(url(PAGE.timeInput));
  const root = '[data-nd-time-input="meeting"]';
  const now = page.locator(`${root} [data-nd-action="time-input-now"]`);
  await expect(now).toBeVisible();
  await now.click();
  const hour = page.locator(`${root} [data-nd-time-input-segment="hour"]`);
  expect((await hour.inputValue()).length, "the now control filled nothing").toBeGreaterThan(0);
});

// --- The two directions of a range, and the one that was never tried --------

test("pulling the end thumb below the start stops it, and keeps the lower bound", async ({
  page,
}) => {
  // THE DIRECTION THE OLD TEST NEVER PRESSED. `paint` clamped with two lines
  // that asked the same question — `start > end` and `end < start` — so the
  // first always fired: dragging the END down dragged the START down with it
  // and threw away a bound the reader had chosen, in silence. Both the comment
  // beside it and `slider.toml` say neither thumb may cross the other, and
  // neither says the other one moves.
  await page.goto(url(PAGE.slider));
  const root = '[data-nd-slider="budget"]';
  const thumbs = page.locator(`${root} [data-nd-slider-thumb]`);
  const [low] = await thumbs.evaluateAll((all) => all.map((one) => Number(one.value)));

  await thumbs.nth(1).focus();
  for (let press = 0; press < 40; press += 1) await page.keyboard.press("ArrowLeft");

  const [after, end] = await thumbs.evaluateAll((all) => all.map((one) => Number(one.value)));
  expect(after, "the end thumb dragged the start thumb down with it").toBe(low);
  expect(end, "the end thumb crossed the start thumb").toBe(low);
});

// --- The one focusable part of a file field ---------------------------------

test("the file field shows its focus, and a refused one is dimmed", async ({ page }) => {
  // The rule was `.nd-file-input-native:focus-visible + .nd-file-input-area`,
  // and the body renders the LABEL first — so the sibling combinator was aimed
  // backwards and matched nothing, ever. The only focusable part of the
  // control showed no focus at all, which is WCAG 2.4.7 failed in silence by a
  // rule that looked right.
  await page.goto(url(PAGE.fileInput));
  const root = '[data-nd-file-input="one-attachment"]';
  const area = page.locator(`${root} [data-nd-file-input-area]`);

  // THE STYLE, NOT THE WIDTH. `outline-width` computes to its declared value
  // even where `outline-style` is `none`, so a width comparison would have
  // passed against a ring nobody can see.
  const ring = () => area.evaluate((el) => getComputedStyle(el).outlineStyle);
  expect(await ring(), "a file field is outlined before anything focuses it").toBe("none");
  await page.locator(`${root} [data-nd-file-input-native]`).focus();
  expect(await ring(), "the one focusable part of a file field shows no focus").not.toBe("none");

  // And a refused one is dimmed, which was the same selector and the same miss.
  const refused = page.locator(".nd-file-input:has([data-nd-file-input-native]:disabled)").first();
  if ((await refused.count()) > 0) {
    const dim = await refused
      .locator("[data-nd-file-input-area]")
      .evaluate((el) => Number.parseFloat(getComputedStyle(el).opacity));
    expect(dim, "a refused file field is drawn as though it were not").toBeLessThan(1);
  }
});

// --- A refusal that has been answered has to go away ------------------------

test("a refusal is cleared once the value is right again", async ({ page }) => {
  // `say` returned early on an empty sentence, so `say(status, "")` — which is
  // what `TimeInput` and `OtpField` both call on their valid paths — did
  // nothing at all. A reader who corrected their answer was left looking at
  // the complaint about the answer before it.
  await page.goto(url(PAGE.timeInput));
  const root = '[data-nd-time-input="booking"]';
  const hour = page.locator(`${root} [data-nd-time-input-segment="hour"]`);
  const minute = page.locator(`${root} [data-nd-time-input-segment="minute"]`);
  const status = page.locator(`${root} [data-nd-time-input-status]`);

  await hour.fill("09");
  await minute.fill("");
  await minute.blur();
  await expect(status).toContainText("minute");

  await minute.fill("30");
  await minute.blur();
  await expect(status, "a stale refusal stayed beside a control that is now right").toHaveText("");
});

// --- The time field's own keyboard, and the boxes it refuses -----------------

test("the arrows walk the boxes and a separator moves on", async ({ page }) => {
  await page.goto(url(PAGE.timeInput));
  await showParts(page);
  const hour = page.locator('[data-nd-time-input] [data-nd-time-input-segment="hour"]').first();
  const minute = page
    .locator('[data-nd-time-input] [data-nd-time-input-segment="minute"]')
    .first();

  // A SEPARATOR MOVES ON, AND IT NEVER DID. `segmentWalk` strips every
  // non-digit before the input handler sees one, so a typed `:` disappeared
  // and only a second digit ever left the hour box — under a contract saying
  // `9:30` fills both boxes with no keystroke in between.
  await hour.click();
  await hour.fill("");
  await page.keyboard.type("9:30");
  await expect(hour).toHaveValue(/^0?9$/);
  await expect(minute).toHaveValue("30");

  // AND THE ARROWS WALK THE ROW, which `OtpField` binds and a time did not.
  await minute.click();
  await page.keyboard.press("Home");
  await page.keyboard.press("ArrowLeft");
  await expect(hour).toBeFocused();
  await page.keyboard.press("End");
  await page.keyboard.press("ArrowRight");
  await expect(minute).toBeFocused();
});

test("a refused time marks the boxes, not only the fieldset", async ({ page }) => {
  await page.goto(url(PAGE.timeInput));
  await showParts(page);
  const hour = page.locator('[data-nd-time-input] [data-nd-time-input-segment="hour"]').first();
  await hour.click();
  await hour.fill("99");
  await hour.blur();
  // `[state.invalid]` declares `aria-invalid` and the rule that paints a
  // refused box is `.nd-time-input-segment[aria-invalid="true"]` — and only
  // `data-nd-invalid` on the wrapper was ever written, so a refused time said
  // its sentence and every box stayed its ordinary colour.
  await expect(hour).toHaveAttribute("aria-invalid", "true");
});

test("only a clock the script re-laid says it was resolved", async ({ page }) => {
  await page.goto(url(PAGE.timeInput));
  await showParts(page);
  // The state exists to tell a re-laid clock from a served one. It was written
  // outside the twelve-hour branch, so it was true of every field on the page.
  const fields = await page.evaluate(() =>
    [...document.querySelectorAll("[data-nd-time-input]")].map((root) => ({
      name: root.dataset.ndTimeInput,
      resolved: root.hasAttribute("data-nd-resolved"),
      twelve: root.querySelector(".nd-time-input-meridiem-group") !== null,
    })),
  );
  expect(fields.length).toBeGreaterThan(0);
  for (const field of fields) {
    expect(field.resolved, `${field.name} claims a clock it never re-laid`).toBe(field.twelve);
  }
});

test("an empty number field is a spinbutton with no value, not a stale one", async ({ page }) => {
  await page.goto(url(PAGE.numberInput));
  await showParts(page);
  const box = page.locator('[data-nd-number-input] .nd-number-input-box').first();
  await box.click();
  await box.fill("42");
  await box.blur();
  // The drawn precision is the field's, so the number is compared and not the
  // string: a currency field says `42.00`.
  expect(Number(await box.getAttribute("aria-valuenow"))).toBe(42);
  // `settle()` returned before touching the value when there was no reading,
  // so a field a reader emptied went on announcing what they had just deleted.
  await box.click();
  await box.fill("");
  await box.blur();
  expect(await box.getAttribute("aria-valuenow")).toBeNull();
});
