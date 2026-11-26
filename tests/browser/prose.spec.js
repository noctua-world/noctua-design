// The reading column, and the document rhythm `flow` gives it.
//
// WHAT IS BEING GUARDED. `Prose { flow }` is how the interface draws a help
// page from its closed Markdown subset (`NOCTUA.md §12.16`): a run of blocks
// nobody spaced by hand. The example on `/prose/` holds every construct of the
// subset, and it is the fixture a change to any of them is looked at against,
// so the first claim here is that it still holds all of them. The rest is
// geometry, measured rather than read off a stylesheet: where each block lands.

import { expect, test } from "@playwright/test";

import { url } from "./pages.js";

const PAGE = "prose/";
const FLOW = ".nd-prose-block[data-nd-flow='true']";

/** A token's length in pixels, measured on a box and not parsed off a string. */
async function token(page, name) {
  return page.evaluate((property) => {
    const probe = document.createElement("div");
    probe.style.position = "absolute";
    probe.style.blockSize = `var(${property})`;
    document.body.append(probe);
    const size = probe.getBoundingClientRect().height;
    probe.remove();
    return size;
  }, name);
}

/** Every block of the flow, with its edges and what it is. */
async function blocks(page) {
  return page.locator(FLOW).first().evaluate((flow) => {
    const box = flow.getBoundingClientRect();
    return {
      top: box.top,
      blocks: [...flow.children].map((el) => {
        const r = el.getBoundingClientRect();
        return {
          what: el.matches(".nd-heading")
            ? "heading"
            : el.matches(".nd-divider")
              ? "break"
              : "block",
          top: r.top,
          bottom: r.bottom,
        };
      }),
    };
  });
}

test("the flow example holds every construct of the Markdown subset", async ({ page }) => {
  await page.goto(url(PAGE));
  const flow = page.locator(FLOW).first();
  await expect(flow).toBeVisible();
  // THE CLOSED SUBSET, ONE SELECTOR EACH. A construct dropped from the
  // fixture is a construct whose drawing nobody looks at again.
  const constructs = {
    "a heading": ":scope > h3.nd-heading",
    "a second level of heading": ":scope > h4.nd-heading",
    "a paragraph": ":scope > p.nd-paragraph",
    emphasis: "em",
    "strong text": "strong",
    "inline code": "p code, li code",
    "a link": "a[href]",
    "a list inside a list": "ul ul",
    "a block quote": "blockquote",
    "a fenced block of code": ".nd-codeblock",
    "a table": "table",
    "a thematic break": ":scope > .nd-divider",
  };
  for (const [name, selector] of Object.entries(constructs)) {
    expect(await flow.locator(selector).count(), `the fixture has no ${name}`).toBeGreaterThan(0);
  }
});

test("a heading sits nearer what it heads than what it ends, and nothing is above the first block", async ({
  page,
}) => {
  await page.goto(url(PAGE));
  const { top, blocks: all } = await blocks(page);
  expect(all.length, "the flow drew too few blocks to say anything").toBeGreaterThan(6);
  expect(Math.abs(all[0].top - top), "room was left above the first block").toBeLessThanOrEqual(1);

  // THE ROOM ITSELF, NOT ONLY WHICH SIDE HAS MORE. A column with no flow
  // also leaves a heading more room above than below, because its gap and the
  // heading's own margin add up there, so the claim is the two tokens: a
  // heading's room above a third level or deeper, and a gap's below.
  const [above, below] = [
    await token(page, "--noctua-space-heading"),
    await token(page, "--noctua-space-gap"),
  ];
  let headings = 0;
  for (let at = 1; at < all.length - 1; at += 1) {
    if (all[at].what !== "heading") continue;
    headings += 1;
    const over = all[at].top - all[at - 1].bottom;
    const under = all[at + 1].top - all[at].bottom;
    expect(under, "a heading is no nearer what it heads than what it ends").toBeLessThan(over);
    expect(Math.abs(over - above), "the room above a heading is not a heading's").toBeLessThanOrEqual(1);
    expect(Math.abs(under - below), "the room under a heading is not a gap's").toBeLessThanOrEqual(1);
  }
  expect(headings, "no heading inside the flow to measure").toBeGreaterThan(0);
});

test("two blocks are a paragraph's room apart, and a break keeps at least that", async ({ page }) => {
  await page.goto(url(PAGE));
  const room = await token(page, "--noctua-space-paragraph");
  const { blocks: all } = await blocks(page);
  let measured = 0;
  for (let at = 1; at < all.length; at += 1) {
    const [before, after] = [all[at - 1], all[at]];
    const gap = after.top - before.bottom;
    if (before.what === "break" || after.what === "break") {
      expect(gap, "a break is closer to a block than a paragraph's room").toBeGreaterThanOrEqual(
        room - 1,
      );
    } else if (before.what === "block" && after.what === "block") {
      expect(Math.abs(gap - room), "two blocks are not a paragraph's room apart").toBeLessThanOrEqual(
        1,
      );
      measured += 1;
    }
  }
  expect(measured, "no two plain blocks in a row to measure").toBeGreaterThan(2);
});

test("without flow the column keeps its one gap", async ({ page }) => {
  await page.goto(url(PAGE));
  const room = await token(page, "--noctua-space-paragraph");
  const gaps = await page
    .locator(".nd-prose-block:not([data-nd-flow='true'])")
    .evaluateAll((els) => els.map((el) => Number.parseFloat(getComputedStyle(el).rowGap)));
  expect(gaps.length).toBeGreaterThan(0);
  for (const gap of gaps) expect(Math.abs(gap - room)).toBeLessThanOrEqual(0.5);
});
