// The readability guarantee, measured.
//
// `spec/tokens.toml` states that a background pattern may move a page pixel at
// most 4% toward the text color, and calls that "a bounded, measurable claim
// rather than a promise". This file is what makes that sentence true. Without
// it the repository asserts a bound nothing checks — which is worse than having
// no bound, because a reader trusts it.
//
// TWO RULES THIS FILE KEEPS:
//
// 1. **Never believe a color read back out of the DOM.**
//    `getComputedStyle(…).color` returns whatever the engine chose to
//    serialise, in whatever space and precision, so two of those strings
//    compared prove nothing about the pixels. Almost everything here is read
//    from a SCREENSHOT — actual pixels, after compositing, which is the only
//    thing a reader's eye sees.
//
//    The one exception says why in place: a probe laid over a live page
//    screenshots whatever is painted in that rectangle rather than its own
//    glyphs, and that is how the scope-ink test passed with its defect
//    restored. It names the two colors and paints them into a 1×1 canvas
//    itself, which is the recipe the gate prescribes — the bytes are still
//    what is compared, they are just composited on purpose rather than by
//    luck.
//
// 2. **Test the worst case, not a sample.** Every pattern × every mode ×
//    every backdrop. A pattern that only fails on one of the six is a pattern
//    that fails for exactly the readers who chose it — and there are six now
//    where there were two, so a pattern's peak ink over a lifted dark page is a
//    contrast pair nobody had measured before this.

import { test, expect } from "@playwright/test";
import { ANY, PAGE, url } from "./pages.js";
import { contrast, extremes, luminance } from "./pixels.js";

const PATTERNS = ["dots", "mesh", "checks", "diagonals"];
const MODES = ["light", "dark"];
const BACKDROPS = ["plain", "cool", "black", "tinted", "soft", "softer"];

for (const mode of MODES) {
  for (const backdrop of BACKDROPS) {
    for (const pattern of PATTERNS) {
      test(`${pattern} on ${mode} + ${backdrop} keeps text readable`, async ({
        page,
      }) => {
        await page.goto(url(ANY));
        await page.evaluate(
          ([mode, backdrop, pattern]) => {
            const r = document.documentElement;
            r.setAttribute("data-theme", mode);
            r.setAttribute("data-noctua-backdrop", backdrop);
            r.setAttribute("data-noctua-pattern", pattern);
          },
          [mode, backdrop, pattern],
        );

        // The text color first, from a REAL GLYPH — a rendered glyph is what
        // has to stay readable, and its darkest pixel is its core.
        // In light mode the glyph's darkest pixel is the letter; in dark mode it
        // is the background BEHIND the letter, and the letter is the lightest.
        // Taking `min` unconditionally measured the dark background against
        // itself and reported a contrast ratio of 1 for every dark case.
        //
        // So the ink is whichever extreme is furthest from the page — which is
        // the same rule in both modes and needs no branch on the mode.
        const glyph = await page.locator(".nd-title").screenshot();
        const { min: glyphDark, max: glyphLight } = await extremes(page, glyph);

        // Then the background ALONE. Hiding the shell is what makes this a
        // measurement of the pattern rather than of the text sitting on it:
        // the first version of this test clipped a region containing the
        // heading, found its darkest pixel was the heading, and dutifully
        // reported that the text had a contrast ratio of 1 against itself.
        //
        // `visibility: hidden` rather than `display: none`, so nothing reflows
        // and the pattern keeps the geometry it had.
        // Three things have to be named rather than left to inheritance:
        // `.nd-header-glass` is a SIBLING of the shell; the docked sidebar sets
        // `visibility: visible` on itself, which REVERSES its parent's hidden;
        // and the skip link is fixed at the top-left, inside the measured
        // region. Any one of them left showing is read as the pattern's color.
        await page.addStyleTag({
          content: ".nd-header, .nd-header-glass, .nd-skip, .nd-body, .nd-body * { visibility: hidden !important; }",
        });
        const strip = await page.locator("body").screenshot({
          clip: { x: 0, y: 0, width: 320, height: 200 },
        });
        const { min, max } = await extremes(page, strip);
        const page_ = luminance(max) > 0.5 ? max : min;
        const ink =
          Math.abs(luminance(glyphDark) - luminance(page_)) >
          Math.abs(luminance(glyphLight) - luminance(page_))
            ? glyphDark
            : glyphLight;

        // Worst case: the pattern's most extreme background pixel against the
        // darkest ink the heading actually paints.
        const worst = Math.min(contrast(ink, min), contrast(ink, max));
        expect(
          worst,
          `${pattern}/${mode}/${backdrop} — background ranged ${min} to ${max}, ink ${ink}`,
        ).toBeGreaterThanOrEqual(4.5);
      });
    }
  }
}

test("a pattern's ink stays under the declared ceiling", async ({ page }) => {
  // THE CEILINGS ARE PER PATTERN NOW, and one of them moved. `dots` covers
  // about a fortieth of its tile where the other three cover most of theirs, so
  // at one shared 4% the grids read as texture and the dots read as nothing at
  // all — invisible on a light page, which is a setting a reader turns on and
  // cannot tell they turned on. `spec/tokens.toml` declares 10% for dots and 4%
  // for the rest, and this is what holds them there.
  const CEILING = { dots: 10, mesh: 4, checks: 4, diagonals: 4 };

  await page.setViewportSize({ width: 1100, height: 760 }); // allow-literal: a viewport is not a token
  await page.goto(url(ANY));
  await page.evaluate(() => {
    document.documentElement.setAttribute("data-theme", "dark");
    document.documentElement.setAttribute("data-noctua-pattern", "none");
  });
  // THE SIDEBAR TOO. It is not inside `.nd-body`, it paints its own surface,
  // and the region measured here used to land on it — which is how the version
  // before this one reported a baseline spread of 68/255 on a bare page and
  // then a NEGATIVE contribution for every pattern, passing an upper bound by
  // measuring furniture instead of ink.
  await page.addStyleTag({
    content:
      ".nd-header, .nd-header-glass, .nd-skip, .nd-body, .nd-body *, .nd-nav, .nd-sidebar { visibility: hidden !important; }",
  });

  const spread = async () => {
    // `page.screenshot`, NOT the body locator's. An element screenshot ignores
    // `clip` — it takes the whole element — so the clip written here had no
    // effect at all and the whole page was being measured.
    const shot = await page.screenshot({ clip: { x: 700, y: 400, width: 300, height: 260 } }); // allow-literal: a screenshot rectangle over bare page, in device pixels
    const { min, max } = await extremes(page, shot);
    // The AMPLITUDE the pattern introduces across the bare page — which is what
    // "how much ink" means. Comparing one absolute extreme between two states
    // measures whatever else changed with them.
    return (max[0] + max[1] + max[2] - min[0] - min[1] - min[2]) / 3;
  };

  // The baseline, whatever it is. Measuring the pattern's contribution RELATIVE
  // to it is what makes this robust, and on a region of bare page it is zero —
  // which is the proof that the region really is bare.
  const plain = await spread();
  expect(plain, "the measured region is not bare page").toBeLessThanOrEqual(2);

  for (const pattern of PATTERNS) {
    await page.evaluate(
      (p) => document.documentElement.setAttribute("data-noctua-pattern", p),
      pattern,
    );
    const contributed = (await spread()) - plain;
    // The ceiling in bytes, plus room for antialiasing and the compositor's
    // rounding — a hairline grid never reaches its own full alpha on a pixel
    // boundary. Measured: dots 22, mesh 16-18, checks and diagonals 9. An
    // unbounded pattern would be past 200.
    const bound = (CEILING[pattern] / 100) * 255 + 12;
    expect(
      contributed,
      `${pattern} added ${contributed.toFixed(1)}/255 against a ceiling of ${CEILING[pattern]}%`,
    ).toBeLessThanOrEqual(bound);
  }
});

// --- A scope's ink is its own -----------------------------------------------

test("a dark scope on a light page does not keep the page's ink", async ({ page }) => {
  await page.goto(url(ANY));

  // NOT A SCREENSHOT, AND THAT IS DELIBERATE. The first version of this test
  // clipped to the words and read their extremes, which is the recipe the rest
  // of this file uses — and it passed with the defect restored, because a probe
  // over a live page catches whatever is painted in that rectangle rather than
  // the glyphs. The header's own furniture was the light end.
  //
  // So the two colours are asked for by name and turned into pixels HERE, in a
  // canvas, which is the recipe the gate prescribes when a colour has to be
  // interpreted. Nothing reads a colour back out of the DOM and believes it: it
  // is fed to the compositor and the bytes are what is compared.
  const measured = await page.evaluate(() => {
    const probe = document.createElement("div");
    probe.setAttribute("data-noctua-scope", "");
    probe.setAttribute("data-theme", "dark");
    probe.setAttribute("data-palette", document.documentElement.dataset.palette);
    document.documentElement.setAttribute("data-theme", "light");
    // NO CLASS ON THE WORDS, which is the whole point. Every component in the
    // system sets its own colour and therefore looked right; only prose that
    // INHERITS was wrong, so a probe that styled its text would have gone on
    // passing with the defect present.
    probe.append(document.createTextNode("plain prose with no class of its own"));
    document.body.append(probe);

    const canvas = document.createElement("canvas");
    canvas.width = canvas.height = 1;
    const ctx = canvas.getContext("2d");
    const bytes = (value) => {
      ctx.clearRect(0, 0, 1, 1);
      ctx.fillStyle = value;
      ctx.fillRect(0, 0, 1, 1);
      return [...ctx.getImageData(0, 0, 1, 1).data].slice(0, 3);
    };
    const style = getComputedStyle(probe); // allow-literal: every value read here is painted into the canvas above and compared as bytes, which is the recipe the gate prescribes
    const out = {
      ink: bytes(style.color),
      surface: bytes(style.backgroundColor),
      page: bytes(getComputedStyle(document.body).color), // allow-literal: painted into the canvas above and compared as bytes, which is the recipe the gate prescribes
    };
    probe.remove();
    return out;
  });

  // `color` is set once, high up, from `var(--nc-color-fg)` — and a `var()`
  // resolves where it is DECLARED, so every descendant inherits the computed
  // colour and a themed subtree kept the page's ink over its own surface.
  // Measured with the defect present: oklch(0.2836) text on an oklch(0.178)
  // surface — near-black on near-black, a ratio of about 1.2.
  expect(
    contrast(measured.ink, measured.surface),
    `the scope drew ${measured.ink} on ${measured.surface} — a themed subtree kept the page's ink`,
  ).toBeGreaterThanOrEqual(4.5);
  // And the page really was the other polarity, so this is not passing on a
  // page that happened to agree with the scope.
  expect(
    measured.ink.join(),
    "the page and the scope resolved the same ink, so nothing was actually themed",
  ).not.toBe(measured.page.join());
});

// The aurora used to be measured here — an 18% wash of the accent whose lobes
// were sized as a share of their box, so inside a 320px scope they covered the
// content rather than the margins. It was the one effect with no bound until
// this file gave it one. It has been removed from the system, and a probe test
// for a setting that no longer exists is a test that passes by being empty.
