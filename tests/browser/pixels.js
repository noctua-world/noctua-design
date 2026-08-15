// Colour, read the only way this repository allows: from pixels.
//
// `getComputedStyle(…).color` returns whatever the engine chose to serialise,
// in whatever space and precision, so comparing two of those strings proves
// nothing about what a reader sees. Everything here decodes a SCREENSHOT —
// actual pixels, after compositing, which includes the one thing a computed
// style can never tell you: what a translucent surface ended up looking like
// over whatever happened to be behind it.
//
// Lifted out of `readability.spec.js`, which had them to itself while the only
// readability claim in the repository was about the background pattern. The
// glass makes controls translucent too, and a second copy of a contrast
// function is a second answer to the same question.

/** WCAG 2.1 relative luminance, from an sRGB triple. */
export function luminance([r, g, b]) {
  const channel = (v) => {
    const s = v / 255;
    return s <= 0.03928 ? s / 12.92 : ((s + 0.055) / 1.055) ** 2.4;
  };
  return 0.2126 * channel(r) + 0.7152 * channel(g) + 0.0722 * channel(b);
}

export function contrast(a, b) {
  const [hi, lo] = [luminance(a), luminance(b)].sort((x, y) => y - x);
  return (hi + 0.05) / (lo + 0.05);
}

/**
 * The darkest and lightest pixel in a PNG screenshot, decoded in the browser.
 *
 * The extremes are what matters: a pattern's worst case for a glyph is the
 * pattern's own peak ink, and averaging would hide exactly that.
 */
export async function extremes(page, buffer) {
  const base64 = buffer.toString("base64");
  return page.evaluate(async (data) => {
    const blob = await (await fetch(`data:image/png;base64,${data}`)).blob();
    const bitmap = await createImageBitmap(blob);
    const canvas = new OffscreenCanvas(bitmap.width, bitmap.height);
    const ctx = canvas.getContext("2d");
    ctx.drawImage(bitmap, 0, 0);
    const { data: px } = ctx.getImageData(0, 0, bitmap.width, bitmap.height);

    let min = [255, 255, 255];
    let max = [0, 0, 0];
    let minSum = Infinity;
    let maxSum = -Infinity;
    for (let i = 0; i < px.length; i += 4) {
      const sum = px[i] + px[i + 1] + px[i + 2];
      if (sum < minSum) {
        minSum = sum;
        min = [px[i], px[i + 1], px[i + 2]];
      }
      if (sum > maxSum) {
        maxSum = sum;
        max = [px[i], px[i + 1], px[i + 2]];
      }
    }
    return { min, max };
  }, base64);
}

/**
 * One pixel of a screenshot, at a point inside it.
 *
 * The extremes answer "is anything in this box readable against anything else";
 * they cannot answer "what colour is this surface", because the lightest pixel
 * in a control's box may be an antialiased edge or a light shape showing
 * through it. A named point can: a few pixels in from a control's leading edge,
 * at half its height, is its pane and nothing else.
 */
export async function pixelAt(page, buffer, x, y) {
  const base64 = buffer.toString("base64");
  return page.evaluate(
    async ([data, px, py]) => {
      const blob = await (await fetch(`data:image/png;base64,${data}`)).blob();
      const bitmap = await createImageBitmap(blob);
      const canvas = new OffscreenCanvas(bitmap.width, bitmap.height);
      const ctx = canvas.getContext("2d");
      ctx.drawImage(bitmap, 0, 0);
      const [r, g, b] = ctx.getImageData(
        Math.min(px, bitmap.width - 1),
        Math.min(py, bitmap.height - 1),
        1,
        1,
      ).data;
      return [r, g, b];
    },
    [base64, Math.round(x), Math.round(y)],
  );
}

/**
 * How much of a screenshot is ink, as a fraction of its pixels.
 *
 * `extremes` answers *is anything here readable against anything else* and
 * `pixelAt` answers *what colour is this surface*. Neither answers *how much of
 * this box is drawn on*, which is the one question a filled shape and an
 * outline of the same shape differ by. A `Rating`'s stars are exactly that
 * pair, and asking the DOM instead would be reading a colour back out of it.
 *
 * The lightest pixel is taken as the ground, so this needs no token and no
 * knowledge of the theme in force.
 */
export async function ink(page, buffer) {
  const base64 = buffer.toString("base64");
  return page.evaluate(async (data) => {
    const blob = await (await fetch(`data:image/png;base64,${data}`)).blob();
    const bitmap = await createImageBitmap(blob);
    const canvas = new OffscreenCanvas(bitmap.width, bitmap.height);
    const ctx = canvas.getContext("2d");
    ctx.drawImage(bitmap, 0, 0);
    const { data: px } = ctx.getImageData(0, 0, bitmap.width, bitmap.height);

    let ground = -Infinity;
    for (let i = 0; i < px.length; i += 4) {
      ground = Math.max(ground, px[i] + px[i + 1] + px[i + 2]);
    }
    // Far enough from the ground to skip antialiasing, near enough to count a
    // stroke that is not the darkest ink on the page.
    let drawn = 0;
    for (let i = 0; i < px.length; i += 4) {
      if (ground - (px[i] + px[i + 1] + px[i + 2]) > 60) drawn += 1;
    }
    return drawn / (px.length / 4);
  }, base64);
}
