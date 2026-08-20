// The `i`, and the two things a hover bubble cannot do.
//
// Its behaviour SHIPS — `dist/info.js` — and it is the sixth caller of
// `anchor.js` rather than a sixth copy of it.

import { expect, test } from "@playwright/test";

import { PAGE, url, showParts } from "./pages.js";
const named = (page, name) => page.locator(`[data-nd-info="${name}"]`);

/**
 * The most coloured pixel of a screenshot.
 *
 * `pixels.js` answers *is this readable*, *what colour is this surface* and
 * *how much of this is ink*, and none of the three answers *which colour is
 * the ink here* — the darkest pixel of a box holding a blue link and a line of
 * body text is the body text. Everything else on this page is neutral, so the
 * pixel furthest from grey is the one thing in the box that has a colour, and
 * it lands on the token exactly rather than on an antialiased edge.
 */
async function ink(page, shot) {
  return page.evaluate(async (data) => {
    const blob = await (await fetch(`data:image/png;base64,${data}`)).blob();
    const bitmap = await createImageBitmap(blob);
    const canvas = new OffscreenCanvas(bitmap.width, bitmap.height);
    const context = canvas.getContext("2d");
    context.drawImage(bitmap, 0, 0);
    const { data: px } = context.getImageData(0, 0, bitmap.width, bitmap.height);
    let best = [0, 0, 0];
    let spread = -1;
    for (let i = 0; i < px.length; i += 4) {
      const [r, g, b] = [px[i], px[i + 1], px[i + 2]];
      const range = Math.max(r, g, b) - Math.min(r, g, b);
      if (range > spread) {
        spread = range;
        best = [r, g, b];
      }
    }
    return { colour: best, spread };
  }, shot.toString("base64"));
}

test("with the script blocked there is no marker and no note", async ({ browser }) => {
  const context = await browser.newContext({ javaScriptEnabled: false });
  const page = await context.newPage();
  await page.goto(url(PAGE.info));
  await showParts(page);
  // A marker that opens nothing is worse than no marker.
  await expect(named(page, "deadline").locator(".nd-info-marker")).toBeHidden();
  await expect(named(page, "deadline").locator(".nd-info-note")).toBeHidden();
  await context.close();
});

test("it is a real button that says what it opens and whether it is open", async ({ page }) => {
  await page.goto(url(PAGE.info));
  await showParts(page);
  const marker = named(page, "deadline").locator(".nd-info-marker");
  await expect(marker).toBeVisible();
  expect(await marker.evaluate((el) => el.tagName)).toBe("BUTTON");
  expect(await marker.getAttribute("type")).toBe("button");
  // Not *Info*: a table of eight of those is eight controls a screen reader
  // reads identically.
  expect(await marker.getAttribute("aria-label")).toMatch(/\S\s\S/);
  expect(await marker.getAttribute("aria-expanded")).toBe("false");

  const note = named(page, "deadline").locator(".nd-info-note");
  expect(await marker.getAttribute("aria-controls")).toBe(await note.getAttribute("id"));
  expect(await note.getAttribute("role")).toBe("note");
});

test("it opens on a press and not on a hover", async ({ page }) => {
  await page.goto(url(PAGE.info));
  await showParts(page);
  const info = named(page, "deadline");
  const note = info.locator(".nd-info-note");

  // A HOVER OPENS NOTHING. That is the whole reason this is a button: a bubble
  // a pointer reveals does not exist on a telephone.
  await info.locator(".nd-info-marker").hover();
  await expect(note).toBeHidden();

  await info.locator(".nd-info-marker").click();
  await expect(note).toBeVisible();
  expect(await info.locator(".nd-info-marker").getAttribute("aria-expanded")).toBe("true");
});

test("Escape closes it and puts the focus back", async ({ page }) => {
  await page.goto(url(PAGE.info));
  await showParts(page);
  const info = named(page, "closable");
  const marker = info.locator(".nd-info-marker");
  await marker.click();
  await expect(info.locator(".nd-info-note")).toBeVisible();

  await page.keyboard.press("Escape");
  await expect(info.locator(".nd-info-note")).toBeHidden();
  // A reader who dismissed a note and lost their place was moved by the thing
  // they were getting rid of.
  expect(await page.evaluate(() => document.activeElement.className)).toContain("nd-info-marker");
});

test("a click outside closes it, and opening a second closes the first", async ({ page }) => {
  await page.goto(url(PAGE.info));
  await showParts(page);
  const first = named(page, "deadline");
  const second = named(page, "usage");

  await first.locator(".nd-info-marker").click();
  await expect(first.locator(".nd-info-note")).toBeVisible();

  await second.locator(".nd-info-marker").click();
  await expect(second.locator(".nd-info-note")).toBeVisible();
  // Two notes open at once is two notes over one page.
  await expect(first.locator(".nd-info-note")).toBeHidden();

  await page.locator("h1").click();
  await expect(second.locator(".nd-info-note")).toBeHidden();
});

test("the note is anchored to its marker rather than to the page", async ({ page }) => {
  await page.goto(url(PAGE.info));
  await showParts(page);
  const info = named(page, "usage");
  await info.locator(".nd-info-marker").click();
  const boxes = await info.evaluate((el) => {
    const marker = el.querySelector(".nd-info-marker").getBoundingClientRect();
    const note = el.querySelector(".nd-info-note").getBoundingClientRect();
    return { marker: { x: marker.x, bottom: marker.bottom }, note: { x: note.x, y: note.y } };
  });
  // Below it, and near it — the numbers `anchor.js` wrote, not a guess.
  expect(boxes.note.y).toBeGreaterThan(boxes.marker.bottom - 1);
  expect(Math.abs(boxes.note.y - boxes.marker.bottom)).toBeLessThan(40);
});

test("a close control exists only where a caller named one", async ({ page }) => {
  await page.goto(url(PAGE.info));
  await showParts(page);
  await named(page, "closable").locator(".nd-info-marker").click();
  await expect(named(page, "closable").locator(".nd-info-close")).toBeVisible();

  await named(page, "deadline").locator(".nd-info-marker").click();
  // Drawn by the script, so it exists only where the words for it do.
  await expect(named(page, "deadline").locator(".nd-info-close")).toHaveCount(0);
});

test("auto reads the measurement and start ignores it", async ({ page }) => {
  // `start` WAS BYTE-IDENTICAL TO `auto`. Only an `="end"` rule existed and
  // nothing read the attribute, so two of the three values drew the same note
  // and a caller choosing between them was choosing nothing. `anchor.js`
  // writes the edge a measurement picks, and only `auto` reads it: a stated
  // edge is a decision, not a guess.
  await page.setViewportSize({ width: 1280, height: 800 });
  await page.goto(url(PAGE.info));
  await showParts(page);

  const root = named(page, "deadline");
  await root.locator(".nd-info-marker").click();
  const note = root.locator(".nd-info-note");
  await expect(note).toBeVisible();
  // The measurement is written, which it never was.
  expect(await note.getAttribute("data-nd-anchor")).toMatch(/^(start|end)$/);

  const at = (placement, anchor) =>
    root.evaluate(
      (el, [where, edge]) => {
        el.setAttribute("data-nd-placement", where);
        el.querySelector(".nd-info-note").setAttribute("data-nd-anchor", edge);
        return Math.round(el.querySelector(".nd-info-note").getBoundingClientRect().left);
      },
      [placement, anchor],
    );

  // Under `auto` the note hangs from whichever edge the measurement named.
  expect(await at("auto", "start")).not.toBe(await at("auto", "end"));
  // Under `start` it hangs from the leading edge whatever the measurement says.
  expect(await at("start", "start")).toBe(await at("start", "end"));
});

test("the marker is painted the colour a reader already reads as a control", async ({ page }) => {
  // IT WAS THE MUTED INK AT REST. The one thing on the line that does
  // something looked like the one thing that had been switched off, and it
  // only became a control on hover — which a touch screen never reports, and
  // this control exists so that a touch screen can reach the sentence.
  //
  // Read from the pixels of both, because a colour compared through
  // `getComputedStyle` is two serialisations being compared and not two
  // colours. The claim is that a reader sees one colour, so it is asked of the
  // screen.
  await page.goto(url(PAGE.info));
  await showParts(page);

  // AT REST, which is the whole of the claim: the marker already turns accent
  // on hover and while its note is open, so a reading taken in either of those
  // states would pass whatever the resting colour was.
  const marker = named(page, "deadline").locator(".nd-info-marker");
  await marker.scrollIntoViewIfNeeded();
  await expect(marker).toHaveAttribute("aria-expanded", "false");
  const glyph = await ink(page, await marker.screenshot());

  // A link on the same page, drawn from `--nc-color-accent` and nothing else.
  // It lives inside a note, so the note is opened to get at it — a different
  // marker from the one measured above, so nothing about this reading touches
  // the reading that matters.
  const usage = named(page, "overview-usage");
  await usage.locator(".nd-info-marker").click();
  const link = usage.locator(".nd-info-note a").first();
  await expect(link).toBeVisible();
  const anchor = await ink(page, await link.screenshot());

  // The marker has a colour at all, which the muted grey it shipped with did
  // not: a neutral's channels are within a few levels of each other however
  // dark it is.
  expect(glyph.spread, "the marker is painted a neutral").toBeGreaterThan(40);
  // And it is the same colour a link on this page is painted. Compared against
  // the page rather than against a value written down here, because the token
  // is the palette's and the palette can be changed.
  for (const channel of [0, 1, 2]) {
    expect(
      Math.abs(glyph.colour[channel] - anchor.colour[channel]),
      `the marker and a link disagree: ${glyph.colour} against ${anchor.colour}`,
    ).toBeLessThanOrEqual(6);
  }
});
