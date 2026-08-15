// The one label in the system, and the three elements it renders.
//
// WHY THE THREE ARE THE WHOLE TEST. `kind` is the only prop here that changes
// what the browser does rather than what it draws: a `<label for>` is the one
// attachment an engine acts on, a `<legend>` is the one construct that reaches
// every control in a set, and a `<p>` is what a group that is neither has to be
// named by. Getting one of them wrong produces a page that looks finished and
// announces nothing, which is why each is asserted against the platform rather
// than against a class.
//
// It also asserts the takeover: `Field` renders this component now, so a page
// full of form controls has exactly one drawing of a question on it.

import { expect, test } from "@playwright/test";

import { PAGE, url, showParts } from "./pages.js";

async function labels(page, which = PAGE.label) {
  await page.goto(url(which));
  await showParts(page);
  return page.locator(".nd-label");
}

test("a label names one control, and the browser agrees that it does", async ({ page }) => {
  await labels(page);
  const named = page.locator("label.nd-label[for]").first();
  await expect(named).toBeVisible();
  const target = await named.getAttribute("for");

  // THE CLAIM, MADE BY THE PLATFORM RATHER THAN BY A CLASS. `control.labels`
  // is the browser's own answer to "what names this", and it is empty for a
  // `for` pointing at something that is not labelable.
  const attached = await page.evaluate((id) => {
    const control = document.getElementById(id);
    return [...(control?.labels ?? [])].map((one) => one.className.includes("nd-label"));
  }, target);
  expect(attached, `nothing labelable answers to ${target}`).toContain(true);
});

test("a group's name is a real <legend> inside a real <fieldset>", async ({ page }) => {
  await labels(page);
  const legend = page.locator("legend.nd-label").first();
  await expect(legend).toBeVisible();
  // A `<legend>` outside a `<fieldset>` names nothing at all and validates as
  // nothing either.
  expect(await legend.evaluate((el) => el.parentElement.tagName)).toBe("FIELDSET");
  // And it is the FIRST child, which is what makes it the group's name rather
  // than a sentence inside the group.
  expect(await legend.evaluate((el) => el.parentElement.firstElementChild === el)).toBe(true);
});

test("a group that is not labelable is named by reference, never by a for", async ({ page }) => {
  await labels(page);
  // Inside an example's stage, which is the only place on this page that is
  // documenting the component. The shell's locale menu and every frame's own
  // theme panel ship shut, and each of them is now built from this component
  // too — a shut panel is not what the page is showing.
  const byName = page.locator(".nd-example-stage p.nd-label[id]").first();
  await expect(byName).toBeVisible();
  // Never a `for`: a `<p>` cannot carry one, and a `<label>` aimed at a
  // radiogroup names nothing.
  expect(await byName.getAttribute("for")).toBeNull();

  const id = await byName.getAttribute("id");
  // The id is derived, so it is `{for_control}-label` and the group points at
  // exactly it. A pointer at nothing is the failure this rule exists to stop.
  expect(id).toMatch(/-label$/);
  const pointed = page.locator(`[aria-labelledby="${id}"]`);
  await expect(pointed.first()).toBeAttached();
});

test("one mark or the other, and never an asterisk", async ({ page }) => {
  await labels(page);
  const marks = page.locator(".nd-label-mark");
  expect(await marks.count()).toBeGreaterThan(0);

  const words = await marks.allTextContents();
  for (const word of words) {
    // A WORD, in the reader's language. An asterisk means *required* only to
    // somebody who already knows it does.
    expect(word.trim()).not.toBe("*");
    expect(word.trim().length).toBeGreaterThan(1);
  }

  // Never two marks on one label: a question is required or it is optional.
  const both = await page.evaluate(
    () =>
      [...document.querySelectorAll(".nd-label")].filter(
        (el) => el.querySelectorAll(".nd-label-mark").length > 1,
      ).length,
  );
  expect(both, "one label carries two marks").toBe(0);

  // And the mark is INSIDE the label, so it is part of the accessible name
  // rather than a sentence beside it.
  const outside = await marks.evaluateAll(
    (els) => els.filter((el) => !el.closest(".nd-label")).length,
  );
  expect(outside).toBe(0);
});

test("the three sizes are three different type sizes", async ({ page }) => {
  await labels(page);
  const drawn = await page.evaluate(() => {
    const seen = {};
    for (const size of ["small", "medium", "large"]) {
      const el = document.querySelector(`.nd-label[data-nd-size="${size}"]`);
      if (el) seen[size] = parseFloat(getComputedStyle(el).fontSize);
    }
    return seen;
  });
  expect(Object.keys(drawn).sort()).toEqual(["large", "medium", "small"]);
  expect(drawn.small).toBeLessThan(drawn.medium);
  expect(drawn.medium).toBeLessThan(drawn.large);
});

test("Field draws no question of its own any more", async ({ page }) => {
  await labels(page, PAGE.input);
  // THE TAKEOVER, ASSERTED. Two drawings of one thing is what got `Choice` and
  // `IconButton` deleted, and a class left behind is how the second one comes
  // back. Every question on a page of form controls is this component.
  const stale = await page.evaluate(
    () => document.querySelectorAll(".nd-field-label, .nd-field-required").length,
  );
  expect(stale, "Field is still drawing its own label").toBe(0);

  const questions = page.locator(".nd-field > .nd-label");
  expect(await questions.count()).toBeGreaterThan(0);
  // Each one is a `<label for>` or a `<legend>`, never the `<p>` form: a field
  // wraps a control, and a control is labelable.
  const shapes = await questions.evaluateAll((els) => [...new Set(els.map((el) => el.tagName))]);
  expect(shapes.sort()).not.toContain("P");
});
