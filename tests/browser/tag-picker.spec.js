// The token input, and the three claims it stakes everything on.
//
// 1. The store is the browser's own — a real `<select multiple>` — so what a
//    server receives is what the browser sends and a value cannot be committed
//    twice.
// 2. A separator is a keystroke and never an encoding, so a value may contain
//    one. That is the whole reason this is not one text field holding a
//    comma-joined string.
// 3. With no script it is smaller, not broken: the composer ships a real
//    `name` and the script's first act is to remove it.
//
// Every one of those is a claim about what arrives, so most of these read a
// real `FormData` rather than the DOM.

import { test, expect } from "@playwright/test";
import { LOCALES, PAGE, url } from "./pages.js";

const P = '[data-nd-tag-picker="topics"]';
const FORM = '[data-nd-tag-picker="labels"]';
const ADDRESSES = '[data-nd-tag-picker="addresses"]';

const words = (page, picker) => page.locator(`${picker} .nd-tag-label`).allTextContents();

const posted = (page, picker) =>
  page.evaluate((selector) => {
    const form = document.querySelector(`${selector} select`).form;
    return [...new FormData(form)].map(([k, v]) => `${k}=${v}`);
  }, picker);

/** Types into the composer and commits with Enter. */
async function add(page, picker, value) {
  const box = page.locator(`${picker} .nd-tag-picker-input`);
  await box.fill(value);
  await box.press("Enter");
}

test("what a reader types becomes a value the form posts", async ({ page }) => {
  await page.goto(url(PAGE.tagPicker));
  await add(page, FORM, "alpha");
  await add(page, FORM, "beta");
  expect(await words(page, FORM)).toEqual(["alpha", "beta"]);
  expect(await posted(page, FORM)).toEqual(["labels=alpha", "labels=beta"]);
});

test("a value may contain the separator", async ({ page }) => {
  await page.goto(url(PAGE.tagPicker));
  // THE WHOLE ARGUMENT FOR THE SHAPE OF THIS COMPONENT. One text field holding
  // a comma-joined string has three ways to answer this and all three are
  // disqualifying: escape it and the package ships a wire format, forbid it and
  // a legal string is refused with no way to say why, split it and the data is
  // corrupted in silence. Here the comma is a keystroke, so a pasted value that
  // contains one survives as one value.
  await page.locator(`${FORM} .nd-tag-picker-input`).fill("Lima, Ana");
  await page.locator(`${FORM} .nd-tag-picker-input`).press("Enter");
  expect(await posted(page, FORM)).toEqual(["labels=Lima, Ana"]);
});

test("the order is the order the reader chose in", async ({ page }) => {
  await page.goto(url(PAGE.tagPicker));
  await add(page, P, "hello");
  await add(page, P, "world");
  // THE DEFECT: `selectedOptions` is DOM order, and a declared suggestion sits
  // above everything typed — so choosing `rust` after typing two words put it
  // FIRST in the tags and first in what the form posts. Both are the reader's
  // list, and neither is sorted by where a suggestion happened to be declared.
  await add(page, P, "rust");
  expect(await words(page, P)).toEqual(["hello", "world", "rust"]);
});

test("the same value twice is one tag, whatever case it is typed in", async ({ page }) => {
  await page.goto(url(PAGE.tagPicker));
  await add(page, P, "rust");
  await add(page, P, "RUST");
  await add(page, P, "Rust");
  // The option set IS the index — there is no second list to keep in step —
  // and the comparison ignores case and accent, so two tags nobody could tell
  // apart cannot both exist.
  expect(await words(page, P)).toEqual(["rust"]);
});

test("a half-typed value is not thrown away when the form goes", async ({ page }) => {
  await page.goto(url(PAGE.tagPicker));
  await add(page, FORM, "alpha");
  // Typed and not committed. This is the single most-complained-about bug in
  // every token input ever shipped: the reader typed it, so they meant it.
  await page.locator(`${FORM} .nd-tag-picker-input`).fill("beta");
  expect(
    await page.evaluate((selector) => {
      const form = document.querySelector(`${selector} select`).form;
      form.dispatchEvent(new Event("submit", { bubbles: true, cancelable: true }));
      return [...new FormData(form)].map(([k, v]) => `${k}=${v}`);
    }, FORM),
  ).toEqual(["labels=alpha", "labels=beta"]);
});

test("a pasted list is one batch, not one commit each", async ({ page }) => {
  await page.goto(url(PAGE.tagPicker));
  let changes = 0;
  await page.exposeFunction("countChange", () => {
    changes += 1;
  });
  await page.evaluate((selector) => {
    document
      .querySelector(`${selector} select`)
      .addEventListener("change", () => window.countChange());
  }, ADDRESSES);

  await page.evaluate((selector) => {
    const box = document.querySelector(`${selector} .nd-tag-picker-input`);
    box.focus();
    const data = new DataTransfer();
    data.setData("text", "a@b.com, c@d.com, e@f.com");
    box.dispatchEvent(new ClipboardEvent("paste", { clipboardData: data, bubbles: true, cancelable: true }));
  }, ADDRESSES);

  await expect.poll(() => words(page, ADDRESSES)).toEqual(["a@b.com", "c@d.com", "e@f.com"]);
  // Five hundred separate commits would be five hundred rebuilds and a thousand
  // dispatched events — every consumer's listener running five hundred times.
  expect(changes, `${changes} change events for one paste`).toBeLessThanOrEqual(1);
});

test("a value the pattern refuses is refused, and says which", async ({ page }) => {
  await page.goto(url(PAGE.tagPicker));
  await add(page, ADDRESSES, "not-an-address");
  expect(await words(page, ADDRESSES)).toEqual([]);
  // A field that stops accepting and gives no reason reads as broken.
  await expect(page.locator(`${ADDRESSES} .nd-tag-picker-note`)).toContainText("not-an-address");
});

test("backspace on an empty box takes the last one off", async ({ page }) => {
  await page.goto(url(PAGE.tagPicker));
  await add(page, P, "one");
  await add(page, P, "two");
  await page.locator(`${P} .nd-tag-picker-input`).press("Backspace");
  expect(await words(page, P)).toEqual(["one"]);
});

test("removing a tag leaves focus somewhere a reader can find it", async ({ page }) => {
  await page.goto(url(PAGE.tagPicker));
  await add(page, P, "one");
  await add(page, P, "two");
  await page.locator(`${P} .nd-tag-remove`).first().click();
  // THE CONTROL THAT DID IT HAS JUST LEFT THE PAGE. Focus falling to `<body>`
  // puts a screen-reader user back at the top of the document with no idea why.
  const landed = await page.evaluate((selector) => {
    const active = document.activeElement;
    if (!active || active === document.body) return "body";
    return document.querySelector(selector).contains(active) ? "inside" : "elsewhere";
  }, P);
  expect(landed).toBe("inside");
});

test("forty tags are not forty tab stops", async ({ page }) => {
  await page.goto(url(PAGE.tagPicker));
  for (const value of ["one", "two", "three", "four"]) await add(page, P, value);
  // The list is one stop and the arrows walk it — the trade `Toolbar` already
  // makes. Four values would already be four presses between the composer and
  // whatever is after it.
  const stops = await page.locator(`${P} .nd-tag-remove`).evaluateAll((els) =>
    els.filter((el) => el.tabIndex === 0).length,
  );
  expect(stops).toBe(0);
});

test("the box is a combobox only once there is something to open", async ({ page }) => {
  await page.goto(url(PAGE.tagPicker));
  const box = page.locator(`${P} .nd-tag-picker-input`);
  await expect(box).toHaveAttribute("role", "combobox");
  await expect(box).toHaveAttribute("aria-expanded", "false");

  await box.fill("ru");
  await box.press("ArrowDown");
  await expect(box).toHaveAttribute("aria-expanded", "true");
  // And the row it names exists. Rebuilding the list per keystroke is how this
  // attribute comes to name a node that was thrown away.
  const named = await box.getAttribute("aria-activedescendant");
  expect(named).toBeTruthy();
  await expect(page.locator(`#${named}`)).toHaveAttribute("role", "option");
});

test.describe("with the script blocked", () => {
  test.use({ javaScriptEnabled: false });

  test("it is smaller, not broken", async ({ page }) => {
    await page.goto(url(PAGE.tagPicker));
    // The native listbox is visible and answerable...
    const native = page.locator(`${P} select`);
    await expect(native).toBeVisible();
    expect(await native.evaluate((el) => el.multiple)).toBe(true);
    expect(await native.evaluate((el) => el.options.length)).toBeGreaterThan(3);

    // ...and the composer still submits, under the same name, so a reader with
    // no script picks any number of suggestions AND types one free value.
    const box = page.locator(`${P} .nd-tag-picker-input`);
    expect(await box.getAttribute("name")).toBe(await native.getAttribute("name"));

    // Nothing the script would have built is in the page claiming to work.
    await expect(page.locator(`${P} .nd-tag-picker-panel`)).toHaveCount(0);
    expect(await box.getAttribute("role")).toBeNull();
  });

  test("both halves of a no-script answer arrive under one name", async ({ page }) => {
    await page.goto(url(PAGE.tagPicker));
    await page.locator(`${FORM} select`).selectOption(["docs"]);
    await page.locator(`${FORM} .nd-tag-picker-input`).fill("typed");
    expect(await posted(page, FORM)).toEqual(["labels=docs", "labels=typed"]);
  });
});

test("inside a picker, a suggestion's value is its label", async ({ page }) => {
  // NOT A BUILD GATE, BECAUSE THE EMITTER NEVER SEES THE PARENT. A reader can
  // produce the label by typing it, so if `value` and the label differ there
  // are two ways to make one tag and they submit two different strings —
  // identical on the screen, different on the server, and the reader cannot
  // tell which one they got.
  for (const locale of LOCALES) {
    await page.goto(url(PAGE.tagPicker, locale));
    const wrong = await page.evaluate(() =>
      [...document.querySelectorAll("[data-nd-tag-picker] option")]
        .filter((option) => option.value !== option.textContent.trim())
        .map((option) => `${option.value} / ${option.textContent.trim()}`),
    );
    expect(wrong, locale).toEqual([]);
  }
});

test("no id appears twice on the picker page, in any locale", async ({ page }) => {
  for (const locale of LOCALES) {
    await page.goto(url(PAGE.tagPicker, locale));
    const twice = await page.evaluate(() => {
      const seen = new Set();
      const out = [];
      for (const el of document.querySelectorAll("[id]")) {
        if (seen.has(el.id)) out.push(el.id);
        seen.add(el.id);
      }
      return out;
    });
    expect(twice, locale).toEqual([]);
  }
});
