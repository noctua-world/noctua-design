// The colour arithmetic, held to an engine that never saw it.
//
// THE TEST THIS REFUSES TO BE. There is one implementation of these
// conversions, so there is no twin to compare it against — and comparing a
// thing to itself passes whenever it is wrong. What there IS, in this
// repository, is thousands of colours computed by `noctua-colors`: a different
// engine, in a different language, in a different repository, which published
// both the hex of every palette step and the OKLCH it computed that hex from.
//
// So `color.js` converts the hex and is held to the OKLCH beside it. Nothing
// here was derived from anything there; agreement is evidence.
//
// It runs in Node rather than in a page, for `scanner.spec.js`'s reason:
// `packages/tokens/color.js` is a plain ES module with no DOM in it, and
// loading a page to test a pure function only adds ways for it to fail for
// reasons that are not about the function.

import { readdirSync, readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

import { expect, test } from "@playwright/test";

import {
  FORMATS,
  hslToRgb,
  hsvToRgb,
  notation,
  oklchToRgb,
  parse,
  rgbToHsl,
  rgbToHsv,
  rgbToOklch,
  toHex,
} from "../../packages/tokens/color.js";

const root = fileURLToPath(new URL("../..", import.meta.url));
const THEMES = `${root}packages/tokens/vendor/json/themes`;

/** Every `(hex, oklch)` pair the colour engine published, from every palette. */
function published() {
  const pairs = [];
  for (const file of readdirSync(THEMES)) {
    if (!file.endsWith(".json")) continue;
    const theme = JSON.parse(readFileSync(`${THEMES}/${file}`, "utf8"));
    for (const mode of ["light", "dark"]) {
      for (const [family, ramp] of Object.entries(theme[mode]?.families ?? {})) {
        for (const step of ramp.steps ?? []) {
          // The sRGB rendition, not the wide-gamut one: `color.js` converts
          // sRGB, and holding it to a P3 colour would be holding it to a
          // question it was never asked.
          const srgb = (step.renditions ?? []).find((one) => one.gamut === "srgb");
          if (srgb) pairs.push({ file, mode, family, role: step.role, ...srgb });
        }
      }
    }
  }
  return pairs;
}

const PAIRS = published();

test("the vendored colours reached this file at all", () => {
  // A fixture that failed to load would make every test below vacuously true,
  // which is the one way a parity suite can be worse than no parity suite.
  expect(PAIRS.length).toBeGreaterThan(5000);
});

test("every published hex converts to the OKLCH published beside it", () => {
  // The vendor rounds to four places; a hex is eight bits per channel, so the
  // round trip cannot be exact and this is the width of that rounding rather
  // than a tolerance chosen to make the test pass.
  const NEAR = 0.002;

  for (const pair of PAIRS) {
    const colour = parse(pair.hex);
    expect(colour, `${pair.file} ${pair.mode} ${pair.role}: ${pair.hex}`).not.toBeNull();
    const ours = rgbToOklch(colour);
    const theirs = pair.oklch;
    const where = `${pair.file} ${pair.mode} ${pair.family} ${pair.role} (${pair.hex})`;
    expect(Math.abs(ours.l - theirs.l), `${where} lightness`).toBeLessThan(NEAR);
    expect(Math.abs(ours.c - theirs.c), `${where} chroma`).toBeLessThan(NEAR);

    // HUE IS COMPARED IN CARTESIAN COORDINATES, and that is not a dodge.
    // Hue is an angle around a point, so its precision degrades as the chroma
    // approaches that point: at c = 0.01, one step of eight-bit rounding is
    // nearly three degrees, and at c = 0 every angle names the same grey. A
    // degree tolerance therefore has to be either wrong near the axis or
    // useless away from it. `a` and `b` carry exactly the same claim — the
    // colour is where the other engine put it — with no singularity in it.
    const ab = (oklch) => [oklch.c * Math.cos((oklch.h * Math.PI) / 180), oklch.c * Math.sin((oklch.h * Math.PI) / 180)];
    const [ourA, ourB] = ab(ours);
    const [theirA, theirB] = ab(theirs);
    expect(Math.abs(ourA - theirA), `${where} a`).toBeLessThan(NEAR);
    expect(Math.abs(ourB - theirB), `${where} b`).toBeLessThan(NEAR);
  }
});

test("OKLCH goes back to the hex it came from", () => {
  for (const pair of PAIRS) {
    const colour = parse(pair.hex);
    const back = oklchToRgb(rgbToOklch(colour));
    // One step of eight-bit rounding in each direction, per channel.
    for (const channel of ["r", "g", "b"]) {
      expect(Math.abs(back[channel] - colour[channel]), `${pair.hex} ${channel}`).toBeLessThanOrEqual(1);
    }
  }
});

test("hex to HSV and back is the identity", () => {
  // Every palette hex, plus a sweep across the cube — the sampled version of
  // "all 16.7 million", which is the same claim without the ten-minute test.
  const sample = [];
  for (let r = 0; r < 256; r += 17) {
    for (let g = 0; g < 256; g += 51) {
      for (let b = 0; b < 256; b += 85) sample.push({ r, g, b, a: 1 });
    }
  }
  const cases = [...sample, ...PAIRS.map((pair) => parse(pair.hex))];
  for (const colour of cases) {
    expect(toHex(hsvToRgb(rgbToHsv(colour)))).toBe(toHex(colour));
  }
});

test("hex to HSL and back is the identity", () => {
  for (const pair of PAIRS) {
    const colour = parse(pair.hex);
    expect(toHex(hslToRgb(rgbToHsl(colour))), pair.hex).toBe(toHex(colour));
  }
});

// --- The literals, all of them, in one place --------------------------------

// EVERY COLOUR LITERAL IN THIS FILE IS BELOW, AND NOWHERE ELSE IN IT.
//
// `NOCTUA.md §8.8` names tests explicitly — no hardcoded colour outside the
// spec, in engines, tests, docs sites or examples — and offers the marker as
// the way through. Scattering thirty markers through a file is how a hatch
// stops being an exception, so they are together: a reviewer reads this block
// once and knows the rest of the file has none.
//
// What they are is test data for a component whose value is the CONSUMER's:
// notations a reader pastes, and notations this file asserts are produced.
// None of them is a colour this system defines, and none will ever be asked
// to follow a palette.

const VIOLET = { r: 124, g: 58, b: 237 };

/** One colour, in every spelling CSS allows for it. */
const SPELLINGS = [
  "#7c3aed", // allow-literal: the notation a reader pastes, asserted to parse to VIOLET
  "#7C3AED", // allow-literal: the same, upper case, which is how most tools write it
  "  #7c3aed  ", // allow-literal: the same, with the whitespace a paste brings
  "rgb(124 58 237)", // allow-literal: the modern form
  "rgb(124, 58, 237)", // allow-literal: the legacy comma form
  "rgba(124, 58, 237, 1)", // allow-literal: the legacy alpha form
];

/** The short hex forms, which are the ones a reader types by hand. */
const SHORT = "#abc"; // allow-literal: three digits, asserted to expand to six
const SHORT_ALPHA = "#abcd"; // allow-literal: four digits, the same with an alpha nibble

/** A saturated blue-violet, written the two ways that carry percentages. */
const IN_HSL = "hsl(263 76% 58%)"; // allow-literal: `76%` is 76, not 7600 — the bug this asserts is gone
const IN_OKLCH = "oklch(54% 60% 292)"; // allow-literal: lightness is a percentage of 1, chroma of 0.4

/** Half-transparent, in the one notation that carries alpha in the hex. */
const HALF = "#7c3aed80"; // allow-literal: eight digits, asserted to round-trip with its alpha

/** Everything that is not a colour, and must come back `null`. */
const REFUSED = [
  "",
  "   ",
  "purple", // allow-literal: a CSS name, which this deliberately does not carry a table of
  "rebeccapurple", // allow-literal: the same, and the one the contract names
  "#7c3ae", // allow-literal: five digits, which is no hex length
  "#gggggg", // allow-literal: six characters that are not hex digits
  "rgb(124 58)", // allow-literal: two channels
  "rgb(a b c)", // allow-literal: three non-numbers
  "hsl(263 76%)", // allow-literal: two components
  "oklch()", // allow-literal: none at all
  "red; background: url(https://example.test/x)", // allow-literal: a colour with CSS after it — the injection this fails closed on
  "var(--nc-accent-solid)", // allow-literal: a token reference, which is not a value this can read
  "javascript:alert(1)",
  null,
  undefined,
  42,
];


// --- What a reader may type -------------------------------------------------

test("every notation this picker writes is a notation it can read back", () => {
  // THE PROPERTY THAT MATTERS TO A READER: they copy a row and paste it into
  // the box, and the colour does not move. A formatter and a parser that
  // disagree is the defect this catches, and it is invisible in either half.
  for (const pair of PAIRS.slice(0, 400)) {
    const colour = parse(pair.hex);
    for (const kind of Object.keys(FORMATS)) {
      const text = notation(colour, kind);
      const back = parse(text);
      expect(back, `${kind}: ${text}`).not.toBeNull();
      for (const channel of ["r", "g", "b"]) {
        // HSL and OKLCH are written rounded, so a channel may move by the
        // width of what was printed rather than by the width of a defect.
        expect(Math.abs(back[channel] - colour[channel]), `${kind} ${text} ${channel}`).toBeLessThanOrEqual(3);
      }
    }
  }
});

test("the four notations are read in every spelling CSS allows", () => {
  for (const text of SPELLINGS) {
    const colour = parse(text);
    expect(colour, text).not.toBeNull();
    expect({ r: colour.r, g: colour.g, b: colour.b }, text).toEqual(VIOLET);
  }
  expect(parse(SHORT)).toEqual({ r: 170, g: 187, b: 204, a: 1 });
  expect(parse(SHORT_ALPHA)).toEqual({ r: 170, g: 187, b: 204, a: 221 / 255 });
});

test("a percentage means what it is a percentage of", () => {
  // `76%` saturation is 76, not 7600 — which is what a second multiplication
  // made of it, and what turned every pasted `hsl()` into a grey.
  const bright = parse(IN_HSL);
  expect(bright).not.toBeNull();
  const { s, l } = rgbToHsl(bright);
  expect(Math.round(s)).toBeGreaterThan(70);
  expect(Math.round(l)).toBeGreaterThan(50);
  // OKLCH lightness is 0…1, and CSS says its 100% is 1 while chroma's is 0.4.
  const same = parse(IN_OKLCH);
  expect(same).not.toBeNull();
  expect(rgbToOklch(same).l).toBeGreaterThan(0.5);
  expect(rgbToOklch(same).l).toBeLessThan(0.6);
});

test("alpha survives every notation that carries it", () => {
  const half = parse(HALF);
  expect(half.a).toBeCloseTo(128 / 255, 3);
  expect(toHex(half, { alpha: true })).toBe(HALF);
  expect(notation(half, "rgb", { alpha: true })).toContain("/");
  // And is dropped where it was not asked for, rather than leaking into a
  // notation a consumer pasted somewhere opaque.
  expect(notation(half, "rgb")).not.toContain("/");
  expect(toHex(half)).toBe(SPELLINGS[0]);
});

test("what is not a colour comes back null, and never a guess", () => {
  for (const refused of REFUSED) {
    expect(parse(refused), String(refused)).toBeNull();
  }
});

test("a notation name nothing recognises is dropped rather than rendered", () => {
  expect(notation({ r: 0, g: 0, b: 0, a: 1 }, "cmyk")).toBeNull();
  expect(notation({ r: 0, g: 0, b: 0, a: 1 }, "token")).toBeNull();
});
