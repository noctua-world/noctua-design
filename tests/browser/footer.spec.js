// The lines at the bottom of a window, and the corner at each end of them.
//
// WHAT THE CASES CANNOT SAY. `spec/components/footer.cases.toml` and
// `spec/components/footer_line.cases.toml` hold the markup: the role, the name,
// the attributes. Everything `NOCTUA.md §12.3` says about a footer is geometry
// and paint, and that is asked here: how tall a line is, how wide a corner, that
// nothing draws a border, that four shades are four, what a folded footer is
// and what opens it, and when the control that expands a line shows.

import { expect, test } from "@playwright/test";

import { LOCALES, PAGE, showParts, url } from "./pages.js";
import { contrast, extremes, pixelAt } from "./pixels.js";

async function open(page, locale = "en") {
  await page.goto(url(PAGE.footer, locale));
  await showParts(page);
}

/** The page's first example: three lines, two corners, one expanding control. */
const whole = (page) => page.locator('.nd-footer:has([data-nd-action="overview-footer-expand"])').first();

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

const box = (locator) => locator.evaluate((one) => {
  const at = one.getBoundingClientRect();
  return { left: at.left, right: at.right, top: at.top, bottom: at.bottom, width: at.width, height: at.height };
});

for (const width of [320, 768, 1280]) {
  test(`a line is one line tall and a corner one rail wide, at ${width}`, async ({ page }) => {
    await page.setViewportSize({ width, height: 800 });
    for (const locale of LOCALES) {
      await open(page, locale);
      const line = await token(page, "--noctua-shell-line");
      const rail = await token(page, "--noctua-shell-rail");
      expect(line, "the page declares no line height").toBeGreaterThan(0);
      expect(rail, "the page declares no rail width").toBeGreaterThan(0);

      const footer = whole(page);
      const outer = await box(footer);
      const lines = await footer.locator(".nd-footer-line").evaluateAll((all) =>
        all.map((one) => {
          const at = one.getBoundingClientRect();
          return { left: at.left, right: at.right, top: at.top, bottom: at.bottom, height: at.height };
        }),
      );
      expect(lines.length).toBe(3);
      // The first two are one line and no more. The third is the line a
      // reader types in, which grows with what is in it and never goes under.
      for (const at of [0, 1]) {
        expect(lines[at].height, `${locale}: line ${at} is not one line tall`).toBeCloseTo(line, 1);
      }
      expect(lines[2].height, `${locale}: the last line is under one line`).toBeGreaterThanOrEqual(line - 0.05);
      // STACKED WITH NOTHING BETWEEN THEM: a gap would be the border the
      // footer is not allowed to draw, in the colour of whatever is behind it.
      expect(lines[1].top, `${locale}: a gap under the first line`).toBeCloseTo(lines[0].bottom, 1);
      expect(lines[2].top, `${locale}: a gap under the second line`).toBeCloseTo(lines[1].bottom, 1);
      expect(outer.height, `${locale}: the footer is not its three lines`).toBeCloseTo(
        lines[0].height + lines[1].height + lines[2].height,
        1,
      );

      const start = await box(footer.locator('.nd-footer-corner[data-nd-side="start"]'));
      const end = await box(footer.locator('.nd-footer-corner[data-nd-side="end"]'));
      for (const [name, corner] of [["start", start], ["end", end]]) {
        expect(corner.width, `${locale}: the ${name} corner is not a rail wide`).toBeCloseTo(rail, 1);
        expect(corner.height, `${locale}: the ${name} corner is not as tall as the footer`).toBeCloseTo(outer.height, 1);
        expect(corner.top).toBeCloseTo(outer.top, 1);
      }
      // AT THE ENDS, AND THE LINES RUN FROM ONE TO THE OTHER. In every locale
      // here the start is the left.
      expect(start.left).toBeCloseTo(outer.left, 1);
      expect(end.right).toBeCloseTo(outer.right, 1);
      for (const one of lines) {
        expect(one.left, `${locale}: a line starts under the corner`).toBeCloseTo(start.right, 1);
        expect(one.right, `${locale}: a line ends under the corner`).toBeCloseTo(end.left, 1);
      }

      // And the footer is no wider than what holds it.
      const over = await footer.evaluate((one) => {
        const stage = one.closest(".nd-example-stage") ?? one.parentElement;
        return one.getBoundingClientRect().right - stage.getBoundingClientRect().right;
      });
      expect(over, `${locale}: the footer runs past its stage`).toBeLessThanOrEqual(0.5);
    }
  });
}

test("nothing in a footer draws a border", async ({ page }) => {
  await open(page);
  const widths = await page.evaluate(() =>
    [...document.querySelectorAll(".nd-footer, .nd-footer-line, .nd-footer-line-content, .nd-footer-corner")].map(
      (one) => {
        const style = getComputedStyle(one);
        return (
          Number.parseFloat(style.borderTopWidth) +
          Number.parseFloat(style.borderBottomWidth) +
          Number.parseFloat(style.borderLeftWidth) +
          Number.parseFloat(style.borderRightWidth) +
          (style.outlineStyle === "none" ? 0 : Number.parseFloat(style.outlineWidth))
        );
      },
    ),
  );
  expect(widths.length).toBeGreaterThan(20);
  expect(Math.max(...widths)).toBe(0);
});

for (const scheme of ["light", "dark"]) {
  test(`the three lines and the corners are four shades, in ${scheme}`, async ({ browser }) => {
    const context = await browser.newContext({ colorScheme: scheme });
    const page = await context.newPage();
    await open(page);
    const footer = whole(page);
    await footer.scrollIntoViewIfNeeded();
    const picture = await page.screenshot();
    // A POINT WITH NOTHING DRAWN ON IT: two pixels in from the trailing end of
    // each line, under the middle of its height, where no glyph reaches.
    const shade = async (locator, inset) => {
      const at = await locator.boundingBox();
      return pixelAt(page, picture, at.x + at.width - inset, at.y + at.height - 3);
    };
    const shades = {
      modeline: await shade(footer.locator('.nd-footer-line[data-nd-kind="modeline"]'), 3),
      info: await shade(footer.locator('.nd-footer-line[data-nd-kind="info"] .nd-footer-line-content'), 3),
      prompt: await shade(footer.locator('.nd-footer-line[data-nd-kind="prompt"]'), 3),
      corner: await shade(footer.locator('.nd-footer-corner[data-nd-side="end"]'), 3),
    };
    const names = Object.keys(shades);
    for (const [at, one] of names.entries()) {
      for (const other of names.slice(at + 1)) {
        expect(shades[one], `${one} and ${other} are one shade in ${scheme}`).not.toEqual(shades[other]);
      }
    }
    // VERY CLOSE, THOUGH: the lines are told apart, not set against each
    // other. No two of the four are as far apart as text is from its ground.
    for (const one of names) {
      for (const other of names) {
        expect(contrast(shades[one], shades[other]), `${one} against ${other}`).toBeLessThan(1.5);
      }
    }
    // And the words on each line are readable on its shade.
    for (const kind of ["modeline", "info", "prompt"]) {
      const text = footer.locator(`.nd-footer-line[data-nd-kind="${kind}"] .nd-footer-line-content > span`).first();
      const { min, max } = await extremes(page, await text.screenshot());
      expect(contrast(min, max), `the words on the ${kind} line, ${scheme}`).toBeGreaterThanOrEqual(4.5);
    }
    await context.close();
  });
}

test("a line cuts what does not fit, and an expanded one wraps and grows", async ({ page }) => {
  await page.setViewportSize({ width: 320, height: 800 });
  for (const locale of LOCALES) {
    await open(page, locale);
    const line = await token(page, "--noctua-shell-line");

    const cut = page.locator('.nd-footer-line:has([data-nd-action="example-footer-expand"])');
    const facts = await cut.evaluate((one) => {
      const content = one.querySelector(".nd-footer-line-content");
      const last = content.lastElementChild.getBoundingClientRect();
      return {
        height: one.getBoundingClientRect().height,
        hidden: content.scrollWidth - content.clientWidth,
        // Where the last item ends against where the box does: past it, so the
        // item is there and cut, not wrapped under the first.
        past: last.right - content.getBoundingClientRect().right,
        sameRow: Math.abs(last.top - content.firstElementChild.getBoundingClientRect().top) < last.height,
      };
    });
    expect(facts.height, `${locale}: a cut line grew`).toBeCloseTo(line, 1);
    expect(facts.past, `${locale}: nothing is cut at 320, so the example shows nothing`).toBeGreaterThan(0);
    expect(facts.sameRow, `${locale}: the content wrapped under itself`).toBe(true);

    const grown = page.locator('.nd-footer-line[data-nd-expanded="true"]');
    const wrapped = await grown.evaluate((one) => {
      const content = one.querySelector(".nd-footer-line-content");
      const edge = content.getBoundingClientRect();
      const runs = [...content.querySelectorAll("span")].map((run) => run.getBoundingClientRect());
      return {
        height: one.getBoundingClientRect().height,
        over: runs.filter((run) => run.right > edge.right + 0.5 || run.bottom > edge.bottom + 0.5).length,
      };
    });
    expect(wrapped.height, `${locale}: an expanded line is still one line`).toBeGreaterThan(line * 1.5);
    expect(wrapped.over, `${locale}: an expanded line still cuts its content`).toBe(0);
  }
});

/** Whether anything is drawn in the control's box: its glyph, against a flat ground. */
async function drawn(page, control) {
  const { min, max } = await extremes(page, await control.screenshot());
  return contrast(min, max) > 1.5;
}

test("the expanding control shows under a pointer and under the focus, and is a full target", async ({
  page,
}) => {
  await open(page);
  const line = page.locator('.nd-footer-line:has([data-nd-action="example-footer-expand"])');
  const control = line.locator(".nd-footer-line-toggle");
  await line.scrollIntoViewIfNeeded();
  await page.mouse.move(0, 0);
  expect(await drawn(page, control), "the control shows on a line nothing is on").toBe(false);

  // A target all the same, and no smaller for being unseen.
  const floor = await token(page, "--noctua-target-min");
  const size = await control.boundingBox();
  expect(Math.min(size.width, size.height)).toBeGreaterThanOrEqual(floor);

  // Anywhere on the line, not only on the control.
  const at = await line.boundingBox();
  await page.mouse.move(at.x + 8, at.y + at.height / 2);
  expect(await drawn(page, control), "a pointer on the line does not show the control").toBe(true);

  await page.mouse.move(0, 0);
  expect(await drawn(page, control)).toBe(false);
  await control.focus();
  expect(await drawn(page, control), "the focus does not show the control").toBe(true);
  await control.blur();

  // AND AN EXPANDED LINE KEEPS IT, with no pointer and no focus anywhere near.
  const kept = page.locator('.nd-footer-line[data-nd-expanded="true"] .nd-footer-line-toggle');
  await kept.scrollIntoViewIfNeeded();
  await page.mouse.move(0, 0);
  expect(await drawn(page, kept), "an expanded line hides the way back").toBe(true);
});

test("on a screen with no pointer that hovers, the control always shows", async ({ browser }) => {
  const context = await browser.newContext({ hasTouch: true, isMobile: true, viewport: { width: 390, height: 800 } });
  const page = await context.newPage();
  await open(page);
  expect(await page.evaluate(() => matchMedia("(hover: hover)").matches), "this context still hovers").toBe(false);
  const control = page.locator('[data-nd-action="example-footer-expand"]');
  await control.scrollIntoViewIfNeeded();
  expect(await drawn(page, control), "nothing would ever show the control on a touch screen").toBe(true);
  await context.close();
});

test("a folded footer is a strip, and a pointer, the focus and `shown` each open it", async ({ page }) => {
  await open(page);
  const line = await token(page, "--noctua-shell-line");
  // The one with a control in it, which is the example and not the picture of
  // the value under "Every variant".
  const folded = page.locator('.nd-footer[data-nd-visibility="auto"]:not([data-nd-shown="true"]):has(.nd-button)');
  await folded.scrollIntoViewIfNeeded();
  await page.mouse.move(0, 0);

  const strip = await box(folded);
  expect(strip.height, "a folded footer is not folded").toBeLessThan(line / 2);
  expect(strip.height, "a folded footer leaves nothing to point at").toBeGreaterThan(0);

  // IT OPENS IN THE FLOW. What holds it grows by what the footer grew, so the
  // footer took its height and lies over nothing.
  const holder = () => folded.evaluate((one) => one.parentElement.getBoundingClientRect().height);
  const before = await holder();
  await page.mouse.move(strip.left + strip.width / 2, strip.top + strip.height / 2);
  const opened = await box(folded);
  expect(opened.height, "a pointer on the strip does not open it").toBeCloseTo(3 * line, 1);
  expect((await holder()) - before, "the footer opened over the page").toBeCloseTo(opened.height - strip.height, 1);

  await page.mouse.move(0, 0);
  expect((await box(folded)).height, "it stays open with nothing on it").toBeCloseTo(strip.height, 1);

  // THE KEYBOARD REACHES IT FOLDED, and reaching it opens it: nothing that
  // holds the focus is ever cut away.
  await folded.locator(".nd-button").first().focus();
  expect((await box(folded)).height, "the focus inside does not open it").toBeCloseTo(3 * line, 1);
  await page.evaluate(() => document.activeElement.blur());
  expect((await box(folded)).height).toBeCloseTo(strip.height, 1);

  const shown = page.locator('.nd-footer[data-nd-visibility="auto"][data-nd-shown="true"]:has(.nd-button)');
  expect((await box(shown)).height, "`shown` does not open a folded footer").toBeCloseTo(3 * line, 1);
});

test("a ring in a footer is drawn inside the control it is on", async ({ page }) => {
  await open(page);
  const footer = whole(page);
  const control = footer.locator('.nd-footer-line[data-nd-kind="modeline"] .nd-button').first();
  await control.scrollIntoViewIfNeeded();
  const at = await control.boundingBox();
  const clip = { x: at.x - 4, y: at.y - 4, width: at.width + 8, height: at.height + 8 };
  const rest = await page.screenshot({ clip });
  // By the keyboard, so the ring is one `:focus-visible` draws.
  await page.keyboard.press("Tab");
  await control.focus();
  await page.keyboard.press("Shift+Tab");
  await page.keyboard.press("Tab");
  await expect(control).toBeFocused();
  const ringed = await page.screenshot({ clip });
  // Every pixel that changed is inside the control's own box: four in from
  // each side of the clip is where the box starts.
  const strays = await page.evaluate(
    async ([a, b, width, height]) => {
      const read = async (data) => {
        const blob = await (await fetch(`data:image/png;base64,${data}`)).blob();
        const bitmap = await createImageBitmap(blob);
        const canvas = new OffscreenCanvas(bitmap.width, bitmap.height);
        const ctx = canvas.getContext("2d");
        ctx.drawImage(bitmap, 0, 0);
        return { data: ctx.getImageData(0, 0, bitmap.width, bitmap.height).data, width: bitmap.width };
      };
      const one = await read(a);
      const two = await read(b);
      let changed = 0;
      let outside = 0;
      for (let i = 0; i < one.data.length; i += 4) {
        if (one.data[i] === two.data[i] && one.data[i + 1] === two.data[i + 1] && one.data[i + 2] === two.data[i + 2]) {
          continue;
        }
        changed += 1;
        const dot = i / 4;
        const x = dot % one.width;
        const y = (dot - x) / one.width;
        if (x < 3 || y < 3 || x > width + 4 || y > height + 4) outside += 1;
      }
      return { changed, outside };
    },
    [rest.toString("base64"), ringed.toString("base64"), at.width, at.height],
  );
  expect(strays.changed, "the focused control draws no ring at all").toBeGreaterThan(0);
  expect(strays.outside, "the ring is drawn outside its control, where the next line paints over it").toBe(0);
});
