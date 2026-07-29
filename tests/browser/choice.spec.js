// The radio group's keyboard contract, which is what the role promises.
//
// The settings panel had twelve of these written as raw markup, and they were
// forty-two tab stops: every option was focusable, so a reader crossing the
// panel with Tab pressed it forty-two times. A radio group is ONE tab stop and
// the arrows move within it — not a nicety, but what the markup claims the
// moment it says `role="radiogroup"`.

import { test, expect } from "@playwright/test";
import { ANY, url } from "./pages.js";

const openSettings = (page) =>
  page.evaluate(() => {
    document.querySelector(".nd-settings").open = true;
  });

test("a group is one tab stop, not one per option", async ({ page }) => {
  await page.goto(url(ANY));
  await openSettings(page);

  const stops = await page.evaluate(() =>
    [...document.querySelectorAll('[role="radiogroup"]')].map((group) => ({
      name: group.dataset.ndSetting,
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
      name: group.dataset.ndSetting,
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
  const group = page.locator(".nd-segmented");
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
      .map((group) => group.dataset.ndSetting ?? "(unnamed)"),
  );
  // A radiogroup with neither announces its role and nothing else.
  expect(nameless, "a radiogroup with no accessible name").toEqual([]);
});
