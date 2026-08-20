// `Select`, and the listbox it becomes when the script runs.
//
// The native control is what submits, and everything below is a check that the
// enhancement never takes that away: the real `<select>` stays in the form, the
// value it holds is the value that was chosen, and with the script blocked the
// page is a form that works.
//
// Split out of `forms.spec.js` when `/forms` became six component pages. What
// each test opens matters — a select test on a page with no select passes by
// measuring nothing.

import { test, expect } from "@playwright/test";
import { PAGE, showParts, url } from "./pages.js";
import { extremes } from "./pixels.js";

// THE ONE ASSERTION THAT MATTERS MOST IS THAT THE NATIVE VALUE FOLLOWS.
//
// A listbox that looks right and submits nothing is this whole category's
// signature bug, and it is invisible from the outside: the control says the
// right word, the highlight moves, and the form posts an empty field. Every
// test below that chooses something also reads `<select>.value`.

/** The field around a control, by the name its select carries. */
const field = (page, name) => page.locator(`[data-nd-select]:has(#nd-select-${name})`);

test("the enhancement keeps the real control and hides it properly", async ({ page }) => {
  await page.goto(url(PAGE.select));
  await showParts(page);
  const native = page.locator("#nd-select-palette");

  // Still in the page, still named, still what submits.
  await expect(native).toHaveCount(1);
  expect(await native.getAttribute("name")).toBe("palette");
  // `hidden`, not `aria-hidden`: a tabbable element carrying `aria-hidden` is
  // the `aria-hidden-focus` violation, and `hidden` removes it from the tree
  // and from the tab order at once.
  expect(await native.evaluate((el) => el.hidden)).toBe(true);
  expect(await native.getAttribute("aria-hidden")).toBeNull();

  // And the label follows the control that is now the control.
  const control = page.locator("#nd-select-palette-control");
  expect(await page.locator("#nd-select-palette-label").getAttribute("for")).toBe(
    "nd-select-palette-control",
  );
  expect(await control.getAttribute("aria-labelledby")).toContain("nd-select-palette-label");
});

test("a select is the same box as a button, at every size", async ({ page }) => {
  await page.goto(url(PAGE.select));
  await showParts(page);
  // MEASURED AGAINST THE BUTTON, NOT AGAINST THREE NUMBERS.
  //
  // The contract says `Button`'s scale step for step, and the first version of
  // this test asserted 32/48/64 — the `min-block-size` each rule declares. A
  // small button is 34: four pixels of padding either side of a 24px line and
  // two of border, which is more than its own minimum. So the constants were
  // wrong and the stylesheet was right, and a test that says 32 would go on
  // being wrong every time the type scale moved. Both are read off the page.
  const measure = (target, selector) =>
    target.evaluate((sel) => {
      const of = (size) => {
        const el = document.querySelector(sel.replace("{size}", size));
        return el ? Math.round(el.getBoundingClientRect().height) : null;
      };
      return { small: of("small"), medium: of("medium"), large: of("large") };
    }, selector);

  const selects = await measure(page, '.nd-select[data-nd-size="{size}"] .nd-select-control');
  await page.goto(url(PAGE.button));
  const buttons = await measure(page, '.nd-button[data-nd-size="{size}"]');

  expect(Object.values(buttons).every(Boolean), "no buttons to compare against").toBe(true);
  expect(selects, "a select beside a button nearly lines up").toEqual(buttons);

  // And the arrow is there at every size: `appearance: none` takes the
  // browser's away, so a control that lost the replacement reads as a text box.
  const chevrons = await page
    .locator('.nd-select-field[data-nd-enhanced="true"] .nd-select-chevron')
    .evaluateAll((nodes) => nodes.every((node) => node.getBoundingClientRect().width > 0));
  expect(chevrons, "an enhanced control lost its chevron").toBe(true);
});

test("the combobox keeps its ARIA contract while it is open", async ({ page }) => {
  await page.goto(url(PAGE.select));
  await showParts(page);
  const control = page.locator("#nd-select-palette-control");
  await control.scrollIntoViewIfNeeded();

  expect(await control.getAttribute("aria-expanded")).toBe("false");
  expect(await control.getAttribute("aria-controls")).toBe("nd-select-palette-listbox");
  await expect(page.locator("#nd-select-palette-listbox")).toBeHidden();

  await control.click();
  expect(await control.getAttribute("aria-expanded")).toBe("true");
  await expect(page.locator("#nd-select-palette-listbox")).toBeVisible();

  // `aria-activedescendant` NAMES AN ELEMENT THAT EXISTS.
  //
  // The failure it guards is the quiet one: a stale id, or an id built from an
  // index that no longer matches, leaves a screen reader announcing nothing at
  // all while the highlight moves perfectly well on screen.
  const active = await control.getAttribute("aria-activedescendant");
  expect(active).toBeTruthy();
  await expect(page.locator(`#${active}`)).toHaveAttribute("role", "option");
  await expect(page.locator(`#${active}`)).toBeVisible();
});

test("choosing writes the value into the select that submits", async ({ page }) => {
  await page.goto(url(PAGE.select));
  await showParts(page);
  const control = page.locator("#nd-select-palette-control");
  const native = page.locator("#nd-select-palette");
  await control.scrollIntoViewIfNeeded();

  expect(await native.inputValue(), "the placeholder is not an answer").toBe("");
  await control.click();
  await page.locator("#nd-select-palette-listbox .nd-select-item").nth(2).click();

  expect(await native.inputValue()).toBe("clay-sober");
  expect((await control.textContent()).trim()).toBe("Clay");
  expect(await control.getAttribute("aria-expanded"), "one answer, so it closes").toBe("false");
});

test("the keyboard opens it, moves in it and never chooses by moving", async ({ page }) => {
  await page.goto(url(PAGE.select));
  await showParts(page);
  const control = page.locator("#nd-select-palette-control");
  const native = page.locator("#nd-select-palette");
  await control.scrollIntoViewIfNeeded();
  await control.focus();

  await page.keyboard.press("ArrowDown");
  expect(await control.getAttribute("aria-expanded")).toBe("true");
  const first = await control.getAttribute("aria-activedescendant");
  await page.keyboard.press("ArrowDown");
  expect(await control.getAttribute("aria-activedescendant")).not.toBe(first);
  // Moving is not choosing. A combobox that chose as it moved would answer the
  // question every time a reader passed an option on the way to another.
  expect(await native.inputValue()).toBe("");

  await page.keyboard.press("End");
  const last = await control.getAttribute("aria-activedescendant");
  await page.keyboard.press("Home");
  expect(await control.getAttribute("aria-activedescendant")).not.toBe(last);

  await page.keyboard.press("End");
  await page.keyboard.press("Enter");
  expect(await native.inputValue()).toBe("indigo-sober");
});

test("Escape closes, restores focus and changes nothing", async ({ page }) => {
  await page.goto(url(PAGE.select));
  await showParts(page);
  const control = page.locator("#nd-select-palette-control");
  const native = page.locator("#nd-select-palette");
  await control.scrollIntoViewIfNeeded();
  await control.focus();

  await page.keyboard.press("ArrowDown");
  await page.keyboard.press("ArrowDown");
  await page.keyboard.press("Escape");
  expect(await control.getAttribute("aria-expanded")).toBe("false");
  await expect(control).toBeFocused();
  expect(await native.inputValue(), "Escape answered the question").toBe("");
});

test("Tab leaves without taking what was highlighted", async ({ page }) => {
  await page.goto(url(PAGE.select));
  await showParts(page);
  const control = page.locator("#nd-select-palette-control");
  const native = page.locator("#nd-select-palette");
  await control.scrollIntoViewIfNeeded();
  await control.focus();

  await page.keyboard.press("ArrowDown");
  await page.keyboard.press("ArrowDown");
  await page.keyboard.press("Tab");
  expect(await control.getAttribute("aria-expanded")).toBe("false");
  // The difference between a combobox and a radiogroup, in one assertion.
  expect(await native.inputValue()).toBe("");
});

test("type-ahead lands on the option the letters name", async ({ page }) => {
  await page.goto(url(PAGE.select));
  await showParts(page);
  const control = page.locator("#nd-select-palette-control");
  await control.scrollIntoViewIfNeeded();
  await control.focus();

  await page.keyboard.press("ArrowDown");
  await page.keyboard.type("cl");
  const active = await control.getAttribute("aria-activedescendant");
  expect((await page.locator(`#${active}`).textContent()).trim()).toBe("Clay");
});

test("filtering hides what does not match, and says so when nothing does", async ({ page }) => {
  await page.goto(url(PAGE.select));
  await showParts(page);
  const control = page.locator("#nd-select-palette-search-control");
  await control.scrollIntoViewIfNeeded();
  await control.click();

  const search = field(page, "palette-search").locator(".nd-select-search");
  await expect(search).toBeFocused();
  await search.type("az");
  const shown = await page
    .locator("#nd-select-palette-search-listbox .nd-select-item:not([hidden])")
    .allTextContents();
  expect(shown.map((text) => text.trim())).toEqual(["Azure"]);

  await search.press("Enter");
  expect(await page.locator("#nd-select-palette-search").inputValue()).toBe("azure-vivid");

  // And the nothing-matches line, which is a prop rather than a word this
  // system invented in one language.
  await control.click();
  await search.type("zzzz");
  await expect(field(page, "palette-search").locator(".nd-select-note")).toBeVisible();
  expect(
    (await field(page, "palette-search").locator(".nd-select-note").textContent()).trim(),
  ).toBe("No palette matches that.");
});

test("clearing puts the field back to nothing", async ({ page }) => {
  await page.goto(url(PAGE.select));
  await showParts(page);
  const control = page.locator("#nd-select-palette-search-control");
  const native = page.locator("#nd-select-palette-search");
  await control.scrollIntoViewIfNeeded();

  // The control that clears is hidden while there is nothing to clear, because
  // a control that does nothing is a control that lies.
  await expect(field(page, "palette-search").locator(".nd-select-clear")).toBeHidden();
  await control.click();
  await page.locator("#nd-select-palette-search-listbox .nd-select-item").first().click();
  expect(await native.inputValue()).toBe("ochre-balanced");

  await field(page, "palette-search").locator(".nd-select-clear").click();
  expect(await native.inputValue()).toBe("");
});

test("several may be chosen, and the control counts rather than lists", async ({ page }) => {
  await page.goto(url(PAGE.select));
  await showParts(page);
  const control = page.locator("#nd-select-palette-compare-control");
  const native = page.locator("#nd-select-palette-compare");
  await control.scrollIntoViewIfNeeded();
  expect((await control.textContent()).trim(), "an unanswered control said nothing").toBe(
    "Choose a palette",
  );
  await control.click();

  expect(await page.locator("#nd-select-palette-compare-listbox").getAttribute("aria-multiselectable")).toBe("true");
  const items = page.locator("#nd-select-palette-compare-listbox .nd-select-item");
  await items.nth(0).click();
  // It stays open: closing after each one is a reader reopening the list for
  // every answer they have.
  expect(await control.getAttribute("aria-expanded")).toBe("true");
  await items.nth(2).click();

  const chosen = await native.evaluate((el) => [...el.selectedOptions].map((o) => o.value));
  expect(chosen).toEqual(["ochre-balanced", "clay-sober"]);
  // AND IT SAID SOMETHING BEFORE ANY OF THAT.
  //
  // A `multiple` select carries no empty `<option>` — nothing chosen already
  // submits nothing — so a control that read its empty state off one showed a
  // blank box, which reads as broken rather than as unanswered.
  expect(
    await native.evaluate((el) => [...el.options].some((o) => o.value === "")),
    "a multiple select carries a blank option",
  ).toBe(false);
  expect((await control.textContent()).trim()).toBe("2 chosen");
  await expect(items.nth(0)).toHaveAttribute("aria-selected", "true");

  // And clicking a chosen one takes it back off.
  await items.nth(0).click();
  expect(await native.evaluate((el) => [...el.selectedOptions].map((o) => o.value))).toEqual([
    "clay-sober",
  ]);
});

test("what is created is appended to the real select", async ({ page }) => {
  await page.goto(url(PAGE.select));
  await showParts(page);
  const control = page.locator("#nd-select-palette-new-control");
  const native = page.locator("#nd-select-palette-new");
  await control.scrollIntoViewIfNeeded();
  await control.click();

  const search = field(page, "palette-new").locator(".nd-select-search");
  await search.type("Umber");
  const offer = page.locator("#nd-select-palette-new-listbox [data-nd-create]");
  await expect(offer).toBeVisible();
  expect((await offer.textContent()).trim(), "the offer is a prop with a hole in it").toBe(
    "Add Umber",
  );

  await search.press("Enter");
  // The whole point: it is an `<option>` in the control that submits, not a
  // chip in a list beside one.
  expect(await native.evaluate((el) => [...el.options].map((o) => o.value))).toContain("Umber");
  expect(await native.inputValue()).toBe("Umber");
});

test("a list that has not arrived says so instead of looking empty", async ({ page }) => {
  await page.goto(url(PAGE.select));
  await showParts(page);
  // OPENABLE, which is the point of saying anything at all.
  //
  // This example used to be `disabled` as well, so the one sentence the loading
  // state exists to say was in the page and unreachable.
  const control = page.locator("#nd-select-palette-loading-control");
  await control.scrollIntoViewIfNeeded();
  await expect(control).toBeEnabled();
  await control.click();
  const note = field(page, "palette-loading").locator(".nd-select-note");
  await expect(note).toBeVisible();
  expect((await note.textContent()).trim()).toBe("The palettes are on their way.");
  // And the two sentences really are different props: an empty list and a list
  // still loading look identical and mean opposite things.
  expect(await note.textContent()).not.toBe(
    await field(page, "palette-search").locator(".nd-select-note").textContent(),
  );
});

test("a refused select refuses the value it holds", async ({ page }) => {
  await page.goto(url(PAGE.select));
  await showParts(page);
  // The real attribute rather than `aria-disabled`: only the real one stops a
  // value being submitted, which is the whole difference on a form control.
  await expect(page.locator("#nd-select-palette-off")).toBeDisabled();
  await expect(page.locator("#nd-select-palette-off-control")).toBeDisabled();
});

test("a group is a group, and its options belong to it", async ({ page }) => {
  await page.goto(url(PAGE.select));
  await showParts(page);
  const control = page.locator("#nd-select-palette-control");
  await control.scrollIntoViewIfNeeded();
  await control.click();

  const groups = await page.evaluate(() =>
    [...document.querySelectorAll("#nd-select-palette-listbox [role='group']")].map((group) => ({
      name: group.getAttribute("aria-label"),
      options: group.querySelectorAll("[role='option']").length,
    })),
  );
  expect(groups).toEqual([
    { name: "Warm", options: 3 },
    { name: "Cool", options: 3 },
  ]);
  // The heading is drawn and not announced twice: the group already carries the
  // name, and a heading read as an option is an option nobody can choose.
  const heading = page.locator("#nd-select-palette-listbox .nd-select-group-label").first();
  expect(await heading.getAttribute("aria-hidden")).toBe("true");
});

test("every drawing in the list came out of the markup", async ({ page }) => {
  await page.goto(url(PAGE.select));
  await showParts(page);
  const control = page.locator("#nd-select-option-icon-control");
  await control.scrollIntoViewIfNeeded();
  await control.click();

  // An `<option>` holds text, so the glyph rides in a `<template>` beside it —
  // which is the one element a `<select>` accepts that can hold a drawing. If
  // that stopped working the row would simply lose its icon, silently.
  const icons = await page
    // A direct child: the tick that marks a chosen option is an icon too, and
    // it lives inside `.nd-select-item-mark`.
    .locator("#nd-select-option-icon-listbox .nd-select-item > .nd-icon-glyph")
    .evaluateAll((nodes) => nodes.map((node) => node.dataset.ndIcon));
  expect(icons).toEqual(["sun", "moon", "monitor"]);

  // And the tick, which comes out of a template the field carries rather than
  // out of this file's option markup.
  await expect(
    page.locator("#nd-select-option-icon-listbox .nd-select-item-mark .nd-icon-glyph").first(),
  ).toHaveAttribute("data-nd-icon", "check");
});

test("the chip beside an option is that option's palette", async ({ page }) => {
  await page.goto(url(PAGE.select));
  await showParts(page);
  const control = page.locator("#nd-select-option-rich-control");
  await control.scrollIntoViewIfNeeded();
  await control.click();
  // The stylesheet a palette needs is fetched on open, so WAIT FOR IT rather
  // than for a number of milliseconds. This was 400ms, which was enough while
  // `SelectOption` had a page of its own and not always enough on `/select/`,
  // where the same control sits under two other components' worth of examples:
  // a chip whose sheet has not arrived paints the reader's own accent, so the
  // slow machine reads three chips as two colours and reports a paint bug.
  await page.waitForLoadState("networkidle");

  // SIX IDENTICAL CHIPS IS WHAT THE FAILURE LOOKS LIKE.
  //
  // A `Swatch` takes its colour from the palette it declares, and a palette is
  // a stylesheet this site loads on demand — so a chip whose sheet has not
  // arrived paints the reader's own accent instead. The theme menu's accent row
  // shipped thirteen of those once. Read from pixels, because the question is
  // what the chips ended up looking like.
  const rows = page.locator("#nd-select-option-rich-listbox .nd-select-item");
  const seen = new Set();
  for (let index = 0; index < 3; index += 1) {
    const chip = await rows.nth(index).locator(".nd-swatch").boundingBox();
    const shot = await page.screenshot({ clip: chip });
    const { min } = await extremes(page, shot);
    seen.add(min.join(","));
  }
  expect(seen.size, "the chips are all the same colour").toBe(3);
});

test("with the script blocked the select is still a select", async ({ browser }) => {
  const context = await browser.newContext({ javaScriptEnabled: false });
  const noScript = await context.newPage();
  await noScript.goto(`http://127.0.0.1:8731${url(PAGE.select)}`);
  await showParts(noScript);

  const native = noScript.locator("#nd-select-palette");
  await expect(native).toBeVisible();
  expect(await native.evaluate((el) => el.tagName)).toBe("SELECT");
  // Its options are real options, in real groups, with the values a form posts.
  expect(await native.evaluate((el) => el.options.length)).toBe(7);
  expect(
    await native.evaluate((el) => [...el.querySelectorAll("optgroup")].map((g) => g.label)),
  ).toEqual(["Warm", "Cool"]);
  // And it can be answered.
  await native.selectOption("teal-balanced");
  expect(await native.inputValue()).toBe("teal-balanced");

  // Nothing the script would have built is in the page claiming to work.
  await expect(noScript.locator(".nd-select-control")).toHaveCount(0);
  await expect(noScript.locator(".nd-select-clear").first()).toBeHidden();
  await context.close();
});

// --- What a form actually receives ------------------------------------------

// NOTHING IN THIS REPOSITORY HAD EVER READ A `FormData`.
//
// Every claim this component makes is a claim about what a server gets — "the
// value that reaches a server is the browser's own", "a hidden form control
// still submits", "a form that submits the sentence submits a different string
// in every locale" — and every test asserted the DOM instead. That is how a
// select came to submit the same value twice with nothing failing.
//
// These read the form.

/** The form the `Select` page ships, and what it would post right now. */
const posted = (page, name) =>
  page.evaluate(
    (field) => [...new FormData(document.querySelector(`#${field}`).form)].map(([k, v]) => `${k}=${v}`),
    name,
  );

const FORM = ".nd-select:has(#nd-select-palette-form)";

test("a hidden select still submits, and submits the value rather than the label", async ({ page }) => {
  await page.goto(url(PAGE.select));
  await showParts(page);
  await page.locator(`${FORM} .nd-select-control`).click();
  await page.locator(`${FORM} .nd-select-item`).filter({ hasText: "Teal" }).click();
  // `SelectOption { value: "teal-balanced", label: "Teal" }` — a form that
  // posted the label would post a different string in every locale.
  expect(await posted(page, "nd-select-palette-form")).toEqual(["palette-form=teal-balanced"]);
});

test("creating a value the control already has submits it once", async ({ page }) => {
  await page.goto(url(PAGE.select));
  await showParts(page);
  await page.locator(`${FORM} .nd-select-control`).click();
  // THE DEFECT: the create offer compared `textContent` while `create()` wrote
  // the term into `value` AND `textContent`. Where the two differ — and
  // `value: "ochre-balanced", label: "Ochre"` is exactly that — typing the
  // value found no label to match, offered to create, and appended a second
  // option carrying a value the control already had. One select, one answer,
  // `palette=ochre-balanced` twice.
  await page.locator(`${FORM} .nd-select-search`).fill("ochre-balanced");
  await expect(page.locator(`${FORM} [data-nd-create]`)).toHaveCount(0);
  await page.keyboard.press("Enter");
  expect(await posted(page, "nd-select-palette-form")).toEqual(["palette-form=ochre-balanced"]);
  expect(
    await page.evaluate(() => document.querySelector("#nd-select-palette-form").options.length),
  ).toBe(4);
});

test("a refused submit says why, where the reader is looking", async ({ page }) => {
  const complaints = [];
  page.on("console", (message) => {
    if (message.type() === "error") complaints.push(message.text());
  });
  await page.goto(url(PAGE.select));
  await showParts(page);
  await page.evaluate(() => {
    document.querySelector("#nd-select-palette-form").selectedIndex = -1;
  });
  await page.locator("form.nd-example-form button[type=submit]").click();

  // THE DEFECT: `required` on a control the script has hidden. The browser has
  // to focus an invalid control to anchor its bubble, cannot, and gives up —
  // Chrome logged "An invalid form control with name=… is not focusable" and
  // the submit did nothing at all, with nothing on the screen to say so.
  await expect(page.locator("#nd-select-palette-form-error")).not.toHaveText("");
  await expect(page.locator(`${FORM} .nd-select-control`)).toBeFocused();
  await expect(page).toHaveURL(new RegExp(`${PAGE.select}$`));
  expect(complaints.filter((text) => text.includes("not focusable"))).toEqual([]);
});

test("a reset takes back what the script added", async ({ page }) => {
  await page.goto(url(PAGE.select));
  await showParts(page);
  await page.locator(`${FORM} .nd-select-control`).click();
  await page.locator(`${FORM} .nd-select-search`).fill("brand new");
  await page.keyboard.press("Enter");
  expect(await posted(page, "nd-select-palette-form")).toEqual(["palette-form=brand new"]);

  await page.locator("form.nd-example-form button[type=reset]").click();
  // THE DEFECT: `reset()` fires no `change`, so the control went on showing an
  // answer the select no longer held; and a created option's `defaultSelected`
  // is false, so reset deselected it and left it in the list for ever — one
  // more phantom suggestion on every reset.
  await expect(page.locator(`${FORM} .nd-select-value`)).toHaveText("Choose a palette");
  expect(
    await page.evaluate(() => document.querySelector("#nd-select-palette-form").options.length),
  ).toBe(4);
  expect(await posted(page, "nd-select-palette-form")).toEqual(["palette-form="]);
});

// --- What the enhanced control says about the field --------------------------

test("a required select says so on the control a reader can reach", async ({ page }) => {
  await page.goto(url(PAGE.select));
  await showParts(page);
  // THE NATIVE CONTROL IS HIDDEN, SO ITS `required` IS UNREACHABLE. The string
  // `aria-required` did not exist anywhere in this repository, under a contract
  // saying it follows from the prop — so an enhanced compulsory field
  // announced itself exactly like an optional one.
  const compulsory = await page.evaluate(() =>
    [...document.querySelectorAll(".nd-select-native[required]")].map((native) => {
      const field = native.closest("[data-nd-select]");
      const control = field.querySelector(".nd-select-control");
      const search = field.querySelector(".nd-select-search");
      return {
        name: native.name,
        control: control?.getAttribute("aria-required"),
        search: search ? search.getAttribute("aria-required") : "true",
      };
    }),
  );
  expect(compulsory.length).toBeGreaterThan(0);
  for (const field of compulsory) {
    expect(field.control, `${field.name}'s button`).toBe("true");
    expect(field.search, `${field.name}'s filter box`).toBe("true");
  }
});

test("a creatable select always has the filter box creating depends on", async ({ page }) => {
  await page.goto(url(PAGE.select));
  await showParts(page);
  // `creatable` is consumed inside `applyFilter`, which runs only where there
  // is a box to filter with, and `create()` reads that box. Without one the
  // attribute reached nothing at all.
  const wrong = await page.evaluate(
    () =>
      document.querySelectorAll('[data-nd-creatable="true"]:not([data-nd-searchable="true"])')
        .length,
  );
  expect(wrong).toBe(0);
});

test("the browser's own refusal arrives with the word that introduces it", async ({ page }) => {
  await page.goto(url(PAGE.select));
  await showParts(page);
  // THE CASE `error_slot` EXISTS FOR. `Field` used to render `error_label`
  // only beside a caller's own `error`, so the one path the slot was built
  // for — a script writing the browser's `validationMessage` into an empty
  // paragraph — never got its visually hidden prefix.
  const said = await page.evaluate(() => {
    const native = document.querySelector(
      '[data-nd-select] .nd-select-native[required][name="palette-compare"]',
    );
    const field = native.closest(".nd-field");
    const slot = field.querySelector("[data-nd-field-error]");
    native.form?.requestSubmit?.();
    return {
      prefix: slot.querySelector(".nd-visually-hidden")?.textContent.trim() ?? "",
      message: slot.querySelector("[data-nd-field-message]")?.textContent.trim() ?? "",
    };
  });
  expect(said.prefix).not.toBe("");
});

test("Home and End reach the list once the filter box is empty", async ({ page }) => {
  await page.goto(url(PAGE.select));
  await showParts(page);
  // BOTH USED TO BAIL OUT WHENEVER THERE WAS A FILTER BOX, and the only
  // `multiple` select on the site is searchable — so the two ways of moving
  // across a long list did not work anywhere it mattered. With something typed
  // they are still the caret's, which is the case the bail-out was written for.
  const field = page.locator('[data-nd-select]:has(#nd-select-palette-compare)');
  await field.locator(".nd-select-control").click();
  const search = field.locator(".nd-select-search");
  await expect(search).toBeVisible();

  await page.keyboard.press("End");
  const last = await field.evaluate(
    (el) => el.querySelector("[aria-activedescendant]")?.getAttribute("aria-activedescendant"),
  );
  await page.keyboard.press("Home");
  const first = await field.evaluate(
    (el) => el.querySelector("[aria-activedescendant]")?.getAttribute("aria-activedescendant"),
  );
  expect(first).not.toBe(last);
  expect(first).toBeTruthy();

  // And with something typed the caret keeps them.
  await search.fill("a");
  const before = await field.evaluate(
    (el) => el.querySelector("[aria-activedescendant]")?.getAttribute("aria-activedescendant"),
  );
  await page.keyboard.press("End");
  const after = await field.evaluate(
    (el) => el.querySelector("[aria-activedescendant]")?.getAttribute("aria-activedescendant"),
  );
  expect(after).toBe(before);
});
