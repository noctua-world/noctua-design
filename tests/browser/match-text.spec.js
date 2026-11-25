// The name a query matched, and the three ways a word marks it.
//
// WHAT THE CASES CANNOT SAY. `spec/components/match_text.cases.toml` holds the
// two bodies to one cut of the text. What is left is what a reader SEES: that
// the three tones are three pictures with the colour taken out of them, that a
// mark moves no letter, and that the ink on each tint can be read.

import { expect, test } from "@playwright/test";

import { PAGE, showParts, url } from "./pages.js";
import { contrast, differing, extremes } from "./pixels.js";

async function open(page, locale = "en") {
  await page.goto(url(PAGE.matchText, locale));
  await showParts(page);
}

/** The example whose caption is this call, by the hits it sets. */
const example = (page, hits) =>
  page.locator(`.nd-example-stage .nd-match-text:has(mark)`).filter({
    has: page.locator(hits),
  });

test("the three tones are three pictures with every colour taken away", async ({ page }) => {
  await open(page);
  // ONE RUN OF LETTERS, DRAWN THREE TIMES, IN ONE INK ON NO TINT. What is left
  // to tell two of them apart is the line under them and nothing else, which is
  // the claim: `NOCTUA.md §8.8.6` measures that the palette cannot separate
  // two families for every reader, so a tone that is only a colour is no tone.
  // The keywords are the platform's, so this names no colour.
  await page.addStyleTag({
    content: `.nd-match {
      color: inherit !important;
      background: transparent !important;
      text-decoration-color: currentColor !important;
    }`,
  });
  const shots = {};
  for (const word of ["1", "2", "3"]) {
    const id = await page.evaluate((tone) => {
      const host = document.querySelector(".nd-example-stage .nd-match-text");
      const probe = document.createElement("span");
      probe.id = `probe-${tone}`;
      probe.className = "nd-match-text";
      probe.style.display = "inline-block";
      const mark = document.createElement("mark");
      mark.className = "nd-match";
      mark.dataset.ndWord = tone;
      mark.textContent = "Button";
      probe.append(mark);
      host.parentElement.append(probe);
      return probe.id;
    }, word);
    shots[word] = await page.locator(`#${id}`).screenshot();
    await page.evaluate((probe) => document.getElementById(probe).remove(), id);
  }
  for (const [a, b] of [["1", "2"], ["1", "3"], ["2", "3"]]) {
    expect(
      await differing(page, shots[a], shots[b]),
      `words ${a} and ${b} are one picture once the colour is gone`,
    ).toBeGreaterThan(0);
  }
});

test("each tone draws a line under its letters", async ({ page }) => {
  await open(page);
  // The style is what the first test tells apart. This asks that there is a
  // line at all, on the marks a page really shipped: take it away and the
  // picture has to change.
  for (const word of ["1", "2", "3"]) {
    const mark = page.locator(`.nd-example-stage mark.nd-match[data-nd-word="${word}"]`).first();
    await expect(mark, `no example draws word ${word}`).toBeVisible();
    const before = await mark.screenshot();
    await mark.evaluate((element) => element.style.setProperty("text-decoration-line", "none"));
    const after = await mark.screenshot();
    await mark.evaluate((element) => element.style.removeProperty("text-decoration-line"));
    expect(await differing(page, before, after), `word ${word} draws no line`).toBeGreaterThan(0);
  }
});

test("a mark moves no letter", async ({ page }) => {
  for (const locale of ["en", "pt-BR"]) {
    await open(page, locale);
    // The same name, marked and unmarked, has to be the same width: a match
    // changes on every keystroke, and a list whose names shift as a reader
    // types is a list they cannot read.
    const widths = await page.evaluate(() =>
      [...document.querySelectorAll(".nd-example-stage .nd-match-text")]
        .filter((one) => one.textContent === "ButtonGroup")
        .map((one) => ({
          marks: one.querySelectorAll("mark").length,
          width: one.getBoundingClientRect().width,
          height: one.getBoundingClientRect().height,
        })),
    );
    const plain = widths.find((one) => one.marks === 0);
    expect(plain, "the page shows the name unmarked").toBeTruthy();
    expect(widths.some((one) => one.marks >= 3), "the page shows three marks in one name").toBe(true);
    for (const one of widths) {
      // Under a twentieth of a pixel: an engine places a run that starts a new
      // element on its own subpixel, and a padded or heavier mark moves the
      // name by whole pixels.
      expect(Math.abs(one.width - plain.width), `${locale}: ${one.marks} mark(s) changed the width`).toBeLessThan(0.05);
      expect(Math.abs(one.height - plain.height), `${locale}: ${one.marks} mark(s) changed the height`).toBeLessThan(0.05);
    }
  }
});

for (const scheme of ["light", "dark"]) {
  test(`the ink on each tint can be read, in ${scheme}`, async ({ browser }) => {
    const context = await browser.newContext({ colorScheme: scheme });
    const page = await context.newPage();
    await open(page);
    for (const word of ["1", "2", "3"]) {
      const mark = page.locator(`.nd-example-stage mark.nd-match[data-nd-word="${word}"]`).first();
      // From pixels: the darkest and the lightest thing in the mark are its
      // ink and its tint, whichever way round the scheme puts them.
      const { min, max } = await extremes(page, await mark.screenshot());
      expect(contrast(min, max), `word ${word} in ${scheme}`).toBeGreaterThanOrEqual(4.5);
    }
    await context.close();
  });
}

test("the whole name is one text to a screen reader", async ({ page }) => {
  await open(page);
  const marked = example(page, 'mark[data-nd-word="3"]').first();
  await expect(marked).toBeVisible();
  // No element inside it is hidden, labelled or given a role, so what is read
  // is the name: the runs are a drawing over it.
  expect(await marked.evaluate((el) => el.textContent)).toBe("ButtonGroup");
  expect(
    await marked.evaluate((el) => el.querySelectorAll("[aria-hidden], [aria-label], [role]").length),
  ).toBe(0);
});
