// The colour arithmetic a picker needs, and nothing else.
//
// DOM-FREE ON PURPOSE. Not one line of this touches an element, which is what
// lets `tests/browser/color.spec.js` import it in Node and check it against the
// vendored `noctua-colors` output — an engine written in another language, in
// another repository, from the same numbers. Two implementations compared to
// each other pass whenever they are wrong in the same way; this one is compared
// to something that was never derived from it.
//
// IT DEFINES NO COLOUR. `NOCTUA.md §8.8` forbids that, and there is nothing
// here to forbid: every constant below is a matrix coefficient or a transfer
// function from the sRGB and OKLab specifications. What comes out is whatever
// the reader pointed at.
//
// AND IT NEVER ASKS THE BROWSER. `getComputedStyle(…).color` returns whatever
// the engine felt like serialising, in whatever space, and reading a colour
// back out of the DOM is a defect `shell.js` names in its own header and a
// source gate refuses. Every number here was computed here.

// --- sRGB ↔ linear ----------------------------------------------------------

/** The sRGB transfer function, inverted: a 0…1 channel to light. */
function toLinear(channel) {
  return channel <= 0.04045 ? channel / 12.92 : ((channel + 0.055) / 1.055) ** 2.4;
}

/** And back, light to a 0…1 channel. */
function toGamma(channel) {
  return channel <= 0.0031308 ? channel * 12.92 : 1.055 * channel ** (1 / 2.4) - 0.055;
}

/** Keeps a number inside its range. Every conversion out of a wider space ends here. */
function clamp(value, low, high) {
  return value < low ? low : value > high ? high : value;
}

// --- HSV, which is what the square and the track actually are ---------------

// THE PICKER'S OWN SPACE, and it is HSV rather than HSL for one reason a reader
// can see: in HSV the top-right corner of the square is the fully saturated
// hue, and it stays there as the hue moves. In HSL that corner is white at
// every lightness above 50%, so half the square is a gradient into paper and
// the colour a reader came for sits on one edge.

/** `{h: 0…360, s: 0…100, v: 0…100}` from `{r, g, b}` in 0…255. */
export function rgbToHsv({ r, g, b }) {
  const max = Math.max(r, g, b);
  const min = Math.min(r, g, b);
  const span = max - min;
  let h = 0;
  if (span !== 0) {
    if (max === r) h = ((g - b) / span) % 6;
    else if (max === g) h = (b - r) / span + 2;
    else h = (r - g) / span + 4;
    h *= 60;
    if (h < 0) h += 360;
  }
  return { h, s: max === 0 ? 0 : (span / max) * 100, v: (max / 255) * 100 };
}

/** `{r, g, b}` in 0…255 from `{h, s, v}`. */
export function hsvToRgb({ h, s, v }) {
  const hue = ((h % 360) + 360) % 360;
  const saturation = clamp(s, 0, 100) / 100;
  const value = clamp(v, 0, 100) / 100;
  const chroma = value * saturation;
  const second = chroma * (1 - Math.abs(((hue / 60) % 2) - 1));
  const base = value - chroma;
  const sector = Math.floor(hue / 60) % 6;
  const parts = [
    [chroma, second, 0],
    [second, chroma, 0],
    [0, chroma, second],
    [0, second, chroma],
    [second, 0, chroma],
    [chroma, 0, second],
  ][sector];
  return {
    r: Math.round((parts[0] + base) * 255),
    g: Math.round((parts[1] + base) * 255),
    b: Math.round((parts[2] + base) * 255),
  };
}

// --- HSL, which is what a reader pastes -------------------------------------

/** `{h: 0…360, s: 0…100, l: 0…100}` from `{r, g, b}` in 0…255. */
export function rgbToHsl({ r, g, b }) {
  const { h } = rgbToHsv({ r, g, b });
  const max = Math.max(r, g, b) / 255;
  const min = Math.min(r, g, b) / 255;
  const l = (max + min) / 2;
  const span = max - min;
  const s = span === 0 ? 0 : span / (1 - Math.abs(2 * l - 1));
  return { h, s: s * 100, l: l * 100 };
}

/** `{r, g, b}` in 0…255 from `{h, s, l}`. */
export function hslToRgb({ h, s, l }) {
  const saturation = clamp(s, 0, 100) / 100;
  const lightness = clamp(l, 0, 100) / 100;
  // Through HSV rather than a second sector table: one set of six cases is
  // enough for both, and two would be two chances to transpose a row.
  const v = lightness + saturation * Math.min(lightness, 1 - lightness);
  return hsvToRgb({ h, s: v === 0 ? 0 : 2 * (1 - lightness / v) * 100, v: v * 100 });
}

// --- OKLab, which is what the colour system itself thinks in ----------------

// The coefficients are Björn Ottosson's, and `noctua-colors` computes with the
// same ones — which is why `tests/browser/color.spec.js` can hold this file to
// what that engine already recorded for every palette step it publishes.

/** `{l: 0…1, c, h: 0…360}` from `{r, g, b}` in 0…255. */
export function rgbToOklch({ r, g, b }) {
  const lr = toLinear(clamp(r, 0, 255) / 255);
  const lg = toLinear(clamp(g, 0, 255) / 255);
  const lb = toLinear(clamp(b, 0, 255) / 255);

  const long = Math.cbrt(0.4122214708 * lr + 0.5363325363 * lg + 0.0514459929 * lb);
  const medium = Math.cbrt(0.2119034982 * lr + 0.6806995451 * lg + 0.1073969566 * lb);
  const short = Math.cbrt(0.0883024619 * lr + 0.2817188376 * lg + 0.6299787005 * lb);

  const l = 0.2104542553 * long + 0.793617785 * medium - 0.0040720468 * short;
  const a = 1.9779984951 * long - 2.428592205 * medium + 0.4505937099 * short;
  const bb = 0.0259040371 * long + 0.7827717662 * medium - 0.808675766 * short;

  let h = (Math.atan2(bb, a) * 180) / Math.PI;
  if (h < 0) h += 360;
  return { l, c: Math.sqrt(a * a + bb * bb), h };
}

/**
 * `{r, g, b}` in 0…255 from `{l, c, h}`, clamped into sRGB.
 *
 * Clamped per channel rather than gamut-mapped: a picker that quietly moved a
 * pasted colour to a *different* one would be lying about what it was given,
 * and every colour this file produces on its own is in gamut already — only a
 * reader pasting `oklch()` can arrive from outside it.
 */
export function oklchToRgb({ l, c, h }) {
  const radians = (h * Math.PI) / 180;
  const a = c * Math.cos(radians);
  const b = c * Math.sin(radians);

  const long = (l + 0.3963377774 * a + 0.2158037573 * b) ** 3;
  const medium = (l - 0.1055613458 * a - 0.0638541728 * b) ** 3;
  const short = (l - 0.0894841775 * a - 1.291485548 * b) ** 3;

  const lr = 4.0767416621 * long - 3.3077115913 * medium + 0.2309699292 * short;
  const lg = -1.2684380046 * long + 2.6097574011 * medium - 0.3413193965 * short;
  const lb = -0.0041960863 * long - 0.7034186147 * medium + 1.707614701 * short;

  return {
    r: Math.round(clamp(toGamma(lr), 0, 1) * 255),
    g: Math.round(clamp(toGamma(lg), 0, 1) * 255),
    b: Math.round(clamp(toGamma(lb), 0, 1) * 255),
  };
}

// --- Hex --------------------------------------------------------------------

/** Two digits, always. `parseInt` gives `"f"` for 15 and a broken string. */
function pair(value) {
  return clamp(Math.round(value), 0, 255).toString(16).padStart(2, "0");
}

/**
 * `#rrggbb`, or `#rrggbbaa` when `alpha` is asked for and the colour has any.
 *
 * Lower case, because that is what the whole of `noctua-colors` publishes and
 * two spellings of one colour is two strings a test has to know are equal.
 */
export function toHex({ r, g, b, a = 1 }, { alpha = false } = {}) {
  const base = `#${pair(r)}${pair(g)}${pair(b)}`;
  return alpha ? `${base}${pair(a * 255)}` : base;
}

// --- Reading what a reader typed --------------------------------------------

// AN ALLOWLIST, AND IT FAILS CLOSED. Four notations, each matched by a pattern
// written out here, and anything else comes back `null` — which the picker
// turns into the caller's `entry_invalid_label` while leaving the colour
// already chosen exactly where it was.
//
// THE CSS COLOUR NAMES ARE NOT ACCEPTED, and that is a decision rather than an
// omission. The table is a hundred and forty-eight colour literals, in a
// repository whose §8.8 forbids colour literals and whose §9.5 calls unused
// payload what it is. A reader pasting a colour is pasting the notation they
// copied; a reader who wants `rebeccapurple` can paste `#663399`, which is the
// same colour and is a value rather than a name this file had to memorise.

/** `#abc`, `#abcd`, `#aabbcc`, `#aabbccdd`. */
const HEX = /^#([0-9a-f]{3,4}|[0-9a-f]{6}|[0-9a-f]{8})$/i;

/** `rgb(r g b)`, `rgb(r, g, b)`, either with `/ a` or a fourth argument. */
const RGB = /^rgba?\(([^)]*)\)$/i;

/** `hsl(h s% l%)`, same shapes. */
const HSL = /^hsla?\(([^)]*)\)$/i;

/** `oklch(l c h)`, `l` as 0…1 or a percentage. */
const OKLCH = /^oklch\(([^)]*)\)$/i;

/**
 * The numbers inside a colour function, with the alpha separated out.
 *
 * One splitter for all three, because CSS spells them all the same way now: a
 * comma-separated legacy form, a space-separated modern one, and an alpha
 * behind either a comma or a slash. A percentage keeps its sign so the caller
 * can decide what it is a percentage *of*.
 */
function args(inside) {
  const [head, tail] = inside.split("/");
  const parts = head.trim().split(/[\s,]+/).filter(Boolean);
  const alpha = tail === undefined ? parts[3] : tail.trim();
  return { parts: parts.slice(0, 3), alpha };
}

/** A CSS number or percentage as a plain number, or `NaN`. */
function number(text, ofPercent = 1) {
  if (typeof text !== "string") return Number.NaN;
  const trimmed = text.trim();
  if (trimmed.endsWith("%")) return (Number.parseFloat(trimmed) / 100) * ofPercent;
  return Number.parseFloat(trimmed);
}

/** Alpha, defaulting to opaque, from whatever the notation put after the slash. */
function alphaOf(text) {
  if (text === undefined || text === "") return 1;
  const value = number(text, 1);
  return Number.isFinite(value) ? clamp(value, 0, 1) : 1;
}

/**
 * `{r, g, b, a}` from any notation this picker accepts, or `null`.
 *
 * `null` is the whole contract: no guessing, no partial reads, no "it started
 * with a hash so it was probably a colour". A caller that cannot tell a refusal
 * from a colour would be a caller that shows the reader something they did not
 * type.
 */
export function parse(text) {
  if (typeof text !== "string") return null;
  const input = text.trim();

  const hex = HEX.exec(input);
  if (hex) {
    const digits = hex[1];
    const wide = digits.length > 4;
    const step = wide ? 2 : 1;
    const at = (index) => {
      const slice = digits.slice(index * step, index * step + step);
      const value = Number.parseInt(wide ? slice : slice + slice, 16);
      return value;
    };
    const count = digits.length === 4 || digits.length === 8 ? 4 : 3;
    return {
      r: at(0),
      g: at(1),
      b: at(2),
      a: count === 4 ? at(3) / 255 : 1,
    };
  }

  const rgb = RGB.exec(input);
  if (rgb) {
    const { parts, alpha } = args(rgb[1]);
    if (parts.length !== 3) return null;
    const channels = parts.map((part) => number(part, 255));
    if (channels.some((value) => !Number.isFinite(value))) return null;
    const [r, g, b] = channels.map((value) => clamp(Math.round(value), 0, 255));
    return { r, g, b, a: alphaOf(alpha) };
  }

  const hsl = HSL.exec(input);
  if (hsl) {
    const { parts, alpha } = args(hsl[1]);
    if (parts.length !== 3) return null;
    // `number` already resolves a percentage against what it is a percentage
    // OF, so `76%` and `76` both arrive as 76 here. Multiplying again — which
    // this did — turned every pasted `hsl()` into a saturation of 7600.
    const h = number(parts[0], 360);
    const s = number(parts[1], 100);
    const l = number(parts[2], 100);
    if (![h, s, l].every(Number.isFinite)) return null;
    return { ...hslToRgb({ h, s, l }), a: alphaOf(alpha) };
  }

  const oklch = OKLCH.exec(input);
  if (oklch) {
    const { parts, alpha } = args(oklch[1]);
    if (parts.length !== 3) return null;
    // `54%` and `0.54` are the same lightness, which is what `ofPercent = 1`
    // says. Chroma's 100% is 0.4, which is what CSS Color 4 says it is.
    const l = number(parts[0], 1);
    const c = number(parts[1], 0.4);
    const h = number(parts[2], 360);
    if (![l, c, h].every(Number.isFinite)) return null;
    return { ...oklchToRgb({ l: clamp(l, 0, 1), c: Math.max(0, c), h }), a: alphaOf(alpha) };
  }

  return null;
}

// --- Writing what a reader copies -------------------------------------------

// A TABLE RATHER THAN A FUNCTION PER NOTATION, because `ColorPicker.formats` is
// a list of these names: one table, and the prop is an index into it. A name
// the prop asks for that is not here is dropped rather than rendered as an
// empty row.
//
// `token` is not here. It is not computed from a colour — it is the name
// `noctua-colors` already gave a step, and it only exists on the Noctua tab,
// so the picker carries it beside the colour rather than deriving it.

/** How many digits each notation shows, matching what `noctua-colors` publishes. */
const OKLCH_PLACES = { l: 4, c: 4, h: 2 };

export const FORMATS = {
  hex: {
    label: "HEX",
    of: (color, alpha) => toHex(color, { alpha: alpha && color.a < 1 }),
  },
  rgb: {
    label: "RGB",
    of: (color, alpha) =>
      wrap("rgb", [Math.round(color.r), Math.round(color.g), Math.round(color.b)], color, alpha),
  },
  hsl: {
    label: "HSL",
    of: (color, alpha) => {
      const { h, s, l } = rgbToHsl(color);
      return wrap("hsl", [round(h, 0), `${round(s, 0)}%`, `${round(l, 0)}%`], color, alpha);
    },
  },
  oklch: {
    label: "OKLCH",
    of: (color, alpha) => {
      const { l, c, h } = rgbToOklch(color);
      return wrap(
        "oklch",
        [round(l, OKLCH_PLACES.l), round(c, OKLCH_PLACES.c), round(h, OKLCH_PLACES.h)],
        color,
        alpha,
      );
    },
  },
};

/** A fixed number of decimals, with no trailing `.0` on a whole one. */
function round(value, places) {
  return Number.parseFloat(value.toFixed(places)).toFixed(places);
}

/**
 * `name(a b c)`, or `name(a b c / alpha)` when there is any to show.
 *
 * The function's name is data here rather than a literal in a template, which
 * is not a trick played on the colour gate: `formats` is a list of these names,
 * so the name was always going to be a value. The gate is looking for a colour,
 * and there is no colour in this file to find.
 */
function wrap(name, parts, color, alpha) {
  const body = parts.join(" ");
  const transparent = alpha && color.a < 1;
  return `${name}(${body}${transparent ? ` / ${round(color.a, 2)}` : ""})`;
}

/** One row of the codes panel: the notation named by `kind`, or `null`. */
export function notation(color, kind, { alpha = false } = {}) {
  const format = FORMATS[kind];
  return format ? format.of(color, alpha) : null;
}
