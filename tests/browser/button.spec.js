// `Button`, and the four promises its contract makes about a real page.
//
// Every one of these was a defect somewhere before the component existed. The
// site shipped a 44×44 header link with an empty icon and, to a screen reader,
// nothing but "link"; the derived variant grid shipped twenty-three buttons
// with no accessible name at all; and six loose `<button>`s wore whichever
// class was nearest.

import { test, expect } from "@playwright/test";
import { LOCALES, PAGE, showParts, url } from "./pages.js";

const BUTTONS = "button/";
// The split button lives here, and this page is not in `PAGE`.
const GROUPS = "button-group/";

test("no control anywhere on the button page is nameless", async ({ page }) => {
  for (const locale of LOCALES) {
    await page.goto(url(BUTTONS, locale));
    const nameless = await page.evaluate(() =>
      [...document.querySelectorAll("button, a[href], [role=radio]")]
        .filter((el) => {
          const label = el.getAttribute("aria-label")?.trim();
          if (label) return false;
          const by = el.getAttribute("aria-labelledby");
          if (by && document.getElementById(by)?.textContent.trim()) return false;
          return !el.textContent.trim();
        })
        .map((el) => el.outerHTML.slice(0, 90)),
    );
    // `hide_label` hides the text; it can never hide the name. The variant grid
    // names its specimens after the caption beside them, which is the one
    // string that cannot drift from what the reader sees.
    expect(nameless, `${locale}: a control with no accessible name`).toEqual([]);
  }
});

test("no button anywhere is left without a type", async ({ page }) => {
  for (const [, path] of Object.entries(PAGE)) {
    await page.goto(url(path));
    const typeless = await page.evaluate(
      () => [...document.querySelectorAll("button:not([type])")].map((el) => el.className),
    );
    // An unset `type` inside a form is `submit`, which is how a button that
    // meant to do something else sends the page. Nothing here is in a form
    // today, and "today" is the word that makes it worth writing down.
    expect(typeless, "a button with no type").toEqual([]);
  }
});

test("a link-form button is a link, and a menu button says it opens one", async ({ page }) => {
  await page.goto(url(BUTTONS));

  // A control that changes the address is an `<a>`, whatever it is painted
  // like: the middle mouse button, the context menu and a screen reader's list
  // of links all expect one, and none can be given by a handler.
  const linked = page.locator('.nd-button[href]').first();
  await expect(linked).toHaveJSProperty("tagName", "A");

  // And `target` never travels without `rel`, whether or not the caller
  // remembered — browsers imply `noopener` for `_blank` now, and "now" is a
  // browser version rather than a guarantee.
  // `noreferrer` implies `noopener`, so either satisfies it. What is refused
  // is a `target` with neither — including one where the caller set some OTHER
  // `rel` and a rule keyed on absence would have stepped aside.
  const targeted = page.locator(".nd-button[target]").first();
  expect(await targeted.getAttribute("rel")).toMatch(/noopener|noreferrer/);

  await expect(page.locator('.nd-button[aria-haspopup="menu"]').first()).toBeVisible();
});

test("no false ARIA state is written on any button", async ({ page }) => {
  await page.goto(url(BUTTONS));
  const lying = await page.evaluate(() =>
    [...document.querySelectorAll(".nd-button")]
      .filter(
        (el) =>
          // `aria-expanded="false"` IS TRUE ON A MENU BUTTON, and only there: a
          // control that opens something opens something whether or not it is
          // open right now, so absence is the lie and `"false"` is the fact.
          (el.getAttribute("aria-expanded") === "false" &&
            el.getAttribute("aria-haspopup") === null) ||
          el.getAttribute("aria-current") === "false" ||
          (el.getAttribute("aria-pressed") === "false" && !el.dataset.ndLabelAlt),
      )
      .map((el) => el.outerHTML.slice(0, 80)),
  );
  // Absence is meaningful on all three. `aria-pressed="false"` makes a plain
  // button announce itself as a toggle that is off, which is a different
  // control from the one it is — the exceptions are a real toggle and a real
  // menu button, each of which says `false` because it has a state to report.
  expect(lying, "a button announcing a state it does not have").toEqual([]);
});

test("every radio group on every page announces exactly one chosen", async ({ page }) => {
  for (const [name, path] of Object.entries(PAGE)) {
    await page.goto(url(path));
    const wrong = await page.evaluate(() =>
      [...document.querySelectorAll('[role="radiogroup"]')]
        .map((group) => {
          const radios = [...group.querySelectorAll('[role="radio"]')];
          return {
            group: group.dataset.ndGroup ?? "(unnamed)",
            // A SETTING IS IN FORCE AND A QUESTION MAY BE UNANSWERED, and the
            // structure says which this is: a group inside a `Field` is a
            // question the reader has been asked. `TimeInput`'s morning and
            // afternoon control is the first, and it ships with NEITHER half
            // marked when the field was given no time — a control that arrives
            // already saying *morning* has answered for the reader, and a
            // twelve-hour reading whose half nobody stated is half a time.
            asks: group.closest(".nd-field") !== null,
            missing: radios.filter((r) => !r.hasAttribute("aria-checked")).length,
            chosen: radios.filter((r) => r.getAttribute("aria-checked") === "true").length,
          };
        })
        .filter((row) => row.missing > 0 || row.chosen > 1 || (row.chosen === 0 && !row.asks)),
    );
    // A radio with no `aria-checked` is a radio whose state is unknown, and a
    // group of those announces nothing chosen. Server-rendered, so it is true
    // before any script runs — the thirteen accent swatches used to hardcode
    // `false` on all thirteen and rely on a script to repair it. Two chosen is
    // wrong everywhere, including in a question.
    expect(wrong, `${name}: a group with no single chosen option`).toEqual([]);
  }
});

test("no button anywhere draws the same chevron twice", async ({ page }) => {
  for (const [name, path] of [...Object.entries(PAGE), ["buttonGroup", GROUPS]]) {
    await page.goto(url(path));
    await showParts(page);
    const doubled = await page.evaluate(() =>
      [...document.querySelectorAll(".nd-button")]
        .filter((el) => el.querySelectorAll('[data-nd-icon="chevron-down"]').length > 1)
        .map((el) => el.outerHTML.slice(0, 100)),
    );
    // `menu` draws the chevron, and a caller who names the same drawing on
    // `icon` or `icon_end` gets theirs rather than both. The split button on
    // this page and the one in the `ButtonGroup` hero each asked for a chevron
    // beside `menu` and each shipped a control with two of them side by side.
    expect(doubled, `${name}: a button drawing two chevrons`).toEqual([]);
  }
});

test("an attached group keeps its outer corners and squares every shared edge", async ({
  page,
}) => {
  await page.goto(url(GROUPS));
  await showParts(page);
  const wrong = await page.evaluate(() => {
    const round = (value) => Number.parseFloat(value) > 0;
    const bad = [];
    for (const group of document.querySelectorAll(
      '.nd-button-group[data-nd-attached="true"]',
    )) {
      const vertical = group.getAttribute("aria-orientation") === "vertical";
      const members = [...group.querySelectorAll(":scope > .nd-button")];
      members.forEach((member, index) => {
        const first = index === 0;
        const last = index === members.length - 1;
        const style = getComputedStyle(member);
        // Which corners belong to the group's own outside, on this axis. A
        // vertical group owns both top corners of its first member and both
        // bottom corners of its last; a horizontal one owns both leading
        // corners and both trailing ones.
        const want = {
          borderTopLeftRadius: first,
          borderTopRightRadius: vertical ? first : last,
          borderBottomRightRadius: last,
          borderBottomLeftRadius: vertical ? last : first,
        };
        for (const [corner, expected] of Object.entries(want)) {
          if (round(style[corner]) !== expected) {
            bad.push(
              `${vertical ? "vertical" : "horizontal"} member ${index}: ` +
                `${corner} is ${style[corner]} and should be ${expected ? "round" : "square"}`,
            );
          }
        }
      });
    }
    return bad;
  });
  // The two horizontal rules used to carry no orientation, so a vertical group
  // ran them as well as its own and kept two of its eight corners. Reading the
  // radius is reading a LENGTH, which is the one thing `getComputedStyle` is
  // allowed to answer here.
  expect(wrong, "a corner on the wrong side of a shared edge").toEqual([]);
});
