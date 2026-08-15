// The radio group's keyboard contract, which is what the role promises.
//
// The settings panel had twelve of these written as raw markup, and they were
// forty-two tab stops: every option was focusable, so a reader crossing the
// panel with Tab pressed it forty-two times. A radio group is ONE tab stop and
// the arrows move within it — not a nicety, but what the markup claims the
// moment it says `role="radiogroup"`.

import { test, expect } from "@playwright/test";
import { ANY, PAGE, modeControl, showParts, url } from "./pages.js";

const openSettings = (page) =>
  page.evaluate(() => {
    document.querySelector(".nd-settings").open = true;
  });

test("a group is one tab stop, not one per option", async ({ page }) => {
  await page.goto(url(ANY));
  await openSettings(page);

  const stops = await page.evaluate(() =>
    [...document.querySelectorAll('[role="radiogroup"]')].map((group) => ({
      name: group.dataset.ndGroup,
      focusable: [...group.querySelectorAll('[role="radio"]')].filter(
        (option) => option.tabIndex >= 0,
      ).length,
      options: group.querySelectorAll('[role="radio"]').length,
    })),
  );

  expect(stops.length, "there are no radiogroups on the page").toBeGreaterThan(5);
  for (const group of stops) {
    expect(group.options, `${group.name} has no options`).toBeGreaterThan(1);
    expect(group.focusable, `${group.name} is ${group.focusable} tab stops`).toBe(1);
  }
});

test("every option announces a state, chosen or not", async ({ page }) => {
  await page.goto(url(ANY));
  await openSettings(page);
  // Unlike `aria-pressed` on a button, absence here is not "not pressed" — it
  // is "state unknown", and a group of those announces nothing chosen.
  const missing = await page.evaluate(() =>
    [...document.querySelectorAll('[role="radio"]')]
      .filter((option) => !option.hasAttribute("aria-checked"))
      .map((option) => option.dataset.ndValue ?? "(unnamed)"),
  );
  expect(missing, "a radio with no aria-checked has no state").toEqual([]);

  const groups = await page.evaluate(() =>
    [...document.querySelectorAll('[role="radiogroup"]')].map((group) => ({
      name: group.dataset.ndGroup,
      chosen: [...group.querySelectorAll('[role="radio"]')].filter(
        (option) => option.getAttribute("aria-checked") === "true",
      ).length,
    })),
  );
  for (const group of groups) {
    expect(group.chosen, `${group.name} announces ${group.chosen} chosen`).toBe(1);
  }
});

test("arrows move within a group and choose as they go", async ({ page }) => {
  await page.goto(url(ANY));
  const group = await modeControl(page);
  const chosen = () =>
    group.evaluate(
      (el) => el.querySelector('[aria-checked="true"]')?.getAttribute("data-nd-value"),
    );

  await group.locator('[data-nd-value="light"]').click();
  expect(await chosen()).toBe("light");

  // A radio group moves AND selects — it does not merely move focus, which is
  // what a toolbar does.
  await page.keyboard.press("ArrowRight");
  expect(await chosen()).toBe("dark");
  await page.keyboard.press("ArrowRight");
  expect(await chosen()).toBe("system");
  await page.keyboard.press("ArrowLeft");
  expect(await chosen()).toBe("dark");
});

test("a group has a name, visible or not", async ({ page }) => {
  await page.goto(url(ANY));
  await openSettings(page);
  const nameless = await page.evaluate(() =>
    [...document.querySelectorAll('[role="radiogroup"]')]
      .filter((group) => {
        const label = group.getAttribute("aria-label")?.trim();
        const by = group.getAttribute("aria-labelledby");
        return !label && !(by && document.getElementById(by));
      })
      .map((group) => group.dataset.ndGroup ?? "(unnamed)"),
  );
  // A radiogroup with neither announces its role and nothing else.
  expect(nameless, "a radiogroup with no accessible name").toEqual([]);
});

// --- One stop, answered or not -----------------------------------------------

test("a group with nothing chosen is still one tab stop, with no script", async ({ browser }) => {
  // THE GUARANTEE, MEASURED WHERE IT USED TO FAIL. The stop was rendered on
  // the chosen item — `0` when `selected` and `-1` otherwise — which is one
  // stop for a group that has an answer and ZERO for one that does not. A
  // `Segmented` nothing had chosen was unreachable by keyboard, and the first
  // example on `/segmented-item/` was exactly that shape. An item cannot fix
  // it, because an item cannot see its siblings, so the group holds the stop.
  const context = await browser.newContext({ javaScriptEnabled: false });
  const page = await context.newPage();

  let unanswered = 0;
  // THE THIRD PAGE IS WHERE THE CASE LIVES. `/theme-scope/`'s scope controls
  // are three groups a reader has not answered — the state a scope is in comes
  // from above it until they choose — so it is the page that would have shown
  // zero tab stops.
  for (const target of [PAGE.segmented, PAGE.segmentedItem, PAGE.themeScope]) {
    await page.goto(url(target));
    await showParts(page);
    const groups = await page.evaluate(() =>
      // NOT THE VARIANT GRID. A specimen is raw markup drawn to be looked at
      // rather than a component, and its buttons are focusable because a
      // `<button>` is — which is a question about specimens, not about this.
      [...document.querySelectorAll('[role="radiogroup"]')]
        .filter((group) => !group.closest(".nd-variant-stage"))
        .map((group) => ({
          name: group.dataset.ndGroup,
          chosen: group.querySelectorAll('[role="radio"][aria-checked="true"]').length,
          // Every element in or at the group that a keyboard can land on.
          stops: [group, ...group.querySelectorAll("*")].filter((el) => el.tabIndex === 0).length,
        })),
    );
    expect(groups.length, `no radiogroup on ${target}`).toBeGreaterThan(0);
    for (const group of groups) {
      expect(group.stops, `${group.name} is ${group.stops} tab stops`).toBe(1);
    }
    unanswered += groups.filter((group) => group.chosen === 0).length;
  }
  // And the pages really do carry the case this is about, or the count
  // above proves only that a group with an answer has one stop.
  expect(unanswered, "no group on either page has nothing chosen").toBeGreaterThan(0);
  await context.close();
});

test("the script moves the stop onto a radio and leaves the group alone", async ({ page }) => {
  await page.goto(url(PAGE.segmented));
  await showParts(page);
  const groups = await page.evaluate(() =>
    // A GROUP INSIDE A TOOLBAR DOES NOT OWN ITS STOPS: the bar's list already
    // holds these radios, so the bar places one stop across the whole of it
    // and a group within it may legitimately hold none.
    [...document.querySelectorAll('[role="radiogroup"]')]
      .filter((group) => !group.closest('[role="toolbar"], .nd-variant-stage'))
      .map((group) => ({
        name: group.dataset.ndGroup,
        // The markup's fallback stop is the script's to take away: a group
        // that kept it would be a second stop, and inside a toolbar it would
        // join the bar's own walk through `[tabindex]`.
        onGroup: group.hasAttribute("tabindex"),
        onRadios: [...group.querySelectorAll('[role="radio"]')].filter((el) => el.tabIndex === 0)
          .length,
      })),
  );
  for (const group of groups) {
    expect(group.onGroup, `${group.name} kept the markup's own stop`).toBe(false);
    expect(group.onRadios, `${group.name} has ${group.onRadios} stops`).toBe(1);
  }
});
