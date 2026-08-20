// The switch, and the one thing it owes: moving when it is pressed.
//
// WHY THIS FILE EXISTS. Every switch on `/switch/` is a demonstration, so its
// `name` is a setting the theme model has never heard of. `shell.js` sent all
// of them through `route`, which asks a scope or the document to store a
// setting and returns early when neither recognises the name — so the eight
// controls on the page about switches announced a state and never changed it.
// `refresh` then skipped them too, so nothing anywhere put the state right.
//
// The `Segmented` branch three lines below the switch's had the missing answer
// from the day it was written, under the comment "a control on a page about
// controls has to work". This is that sentence, made mechanical.

import { expect, test } from "@playwright/test";

import { PAGE, url, showParts } from "./pages.js";

/** Every switch inside an example, which is every switch on this page. */
const switches = (page) => page.locator('.nd-example-stage [role="switch"]');

/**
 * The knob's box inside the track, and the gap on each of the four sides,
 * READ FROM THE PIXELS.
 *
 * Not from a computed style. Every one of the three sums that used to place
 * the knob read back exactly what it had been told, which is why nothing
 * reported that they disagreed with each other — the knob sat 5px from the
 * leading edge when it was off, 7px from the trailing edge when it was on,
 * and 4px from the top either way. A screenshot is the only witness that
 * answers the question a reader asked.
 *
 * How it finds the knob without being told where to look: the middle row of
 * the track is a border pixel, a run of track, the knob, more track, and
 * another border pixel. The commonest colour on that row is therefore the
 * track, and the one run of something else that does NOT touch an edge of the
 * image is the knob. The same walk down the knob's own centre column gives its
 * top and bottom. Nothing here knows a token, a size or a state.
 */
async function knobIn(track) {
  const shot = (await track.screenshot()).toString("base64");
  return track.page().evaluate(async (data) => {
    const blob = await (await fetch(`data:image/png;base64,${data}`)).blob();
    const bitmap = await createImageBitmap(blob);
    const canvas = new OffscreenCanvas(bitmap.width, bitmap.height);
    const ctx = canvas.getContext("2d");
    ctx.drawImage(bitmap, 0, 0);
    const { data: px, width: w, height: h } = ctx.getImageData(0, 0, bitmap.width, bitmap.height);
    const at = (x, y) => {
      const i = (y * w + x) * 4;
      return [px[i], px[i + 1], px[i + 2]];
    };
    const apart = (a, b) => Math.max(Math.abs(a[0] - b[0]), Math.abs(a[1] - b[1]), Math.abs(a[2] - b[2]));

    const mid = Math.floor(h / 2);
    const seen = new Map();
    for (let x = 0; x < w; x += 1) {
      const key = at(x, mid).join(",");
      seen.set(key, (seen.get(key) ?? 0) + 1);
    }
    const fill = [...seen.entries()].sort((a, b) => b[1] - a[1])[0][0].split(",").map(Number);

    // Half of the largest departure from the track, so an antialiased edge is
    // counted at its own midpoint rather than at whichever side of it the
    // threshold happened to fall.
    const walk = (count, colour) => {
      let peak = 0;
      for (let i = 0; i < count; i += 1) peak = Math.max(peak, apart(colour(i), fill));
      const runs = [];
      let open = null;
      for (let i = 0; i < count; i += 1) {
        const off = apart(colour(i), fill) > peak / 2;
        if (off && open === null) open = i;
        if (!off && open !== null) {
          runs.push([open, i]);
          open = null;
        }
      }
      if (open !== null) runs.push([open, count]);
      // The border is the run that touches an edge, and the outermost column
      // of an element screenshot is a blend of the page behind it whenever the
      // control does not land on a whole pixel. The knob touches neither, and
      // is the only thing here more than two pixels across — a gap of zero
      // would merge it into the border, and this reports that as *not found*
      // rather than as a knob.
      return runs.filter(([from, to]) => from > 0 && to < count && to - from > 2);
    };

    const across = walk(w, (x) => at(x, mid));
    if (across.length !== 1) return { found: across.length, axis: "across the track" };
    const [left, right] = across[0];
    const down = walk(h, (y) => at(Math.floor((left + right) / 2), y));
    if (down.length !== 1) return { found: down.length, axis: "down the knob" };
    const [top, bottom] = down[0];
    return {
      found: 1,
      knob: [right - left, bottom - top],
      start: left,
      end: w - right,
      top,
      bottom: h - bottom,
    };
  }, shot);
}

test("a switch the settings model does not know still moves when pressed", async ({ page }) => {
  await page.goto(url(PAGE.switchControl));
  await showParts(page);

  const all = switches(page);
  const count = await all.count();
  expect(count, "the page shows no switches at all").toBeGreaterThan(4);

  for (let at = 0; at < count; at += 1) {
    const flip = all.nth(at);
    if ((await flip.getAttribute("aria-disabled")) === "true") continue;
    const before = await flip.getAttribute("aria-checked");
    // Always written, `true` or `false`. A switch whose state is unknown is
    // not a switch, and this is the attribute the whole defect hid behind.
    expect(["true", "false"], "a switch shipped with no state").toContain(before);
    await flip.click();
    const label = await flip.textContent();
    expect(
      await flip.getAttribute("aria-checked"),
      `\`${label?.trim()}\` did not move when it was pressed`,
    ).toBe(before === "true" ? "false" : "true");
    // And back, so the page is left as it was found and the second press is
    // asserted as well as the first.
    await flip.click();
    expect(await flip.getAttribute("aria-checked")).toBe(before);
  }
});

test("a refused switch stays reachable and refuses", async ({ page }) => {
  await page.goto(url(PAGE.switchControl));
  await showParts(page);

  const refused = page.locator('.nd-example-stage [role="switch"][aria-disabled="true"]').first();
  await expect(refused).toBeVisible();
  // `aria-disabled` rather than `disabled`, so the setting can still be found.
  expect(await refused.evaluate((el) => el.disabled)).toBe(false);
  const before = await refused.getAttribute("aria-checked");
  // Dispatched rather than clicked: the test runner refuses to press something
  // marked `aria-disabled`, and the question here is what the handler does when
  // a press reaches it anyway.
  await refused.dispatchEvent("click");
  expect(await refused.getAttribute("aria-checked"), "a refused switch moved").toBe(before);
});

test("the keyboard moves it, because it is a button", async ({ page }) => {
  await page.goto(url(PAGE.switchControl));
  await showParts(page);

  const flip = switches(page).first();
  const before = await flip.getAttribute("aria-checked");
  await flip.focus();
  await page.keyboard.press("Space");
  expect(await flip.getAttribute("aria-checked"), "Space did nothing").toBe(
    before === "true" ? "false" : "true",
  );
  await page.keyboard.press("Enter");
  expect(await flip.getAttribute("aria-checked"), "Enter did nothing").toBe(before);
});

test("the two sizes are two boxes, not two type sizes", async ({ page }) => {
  await page.goto(url(PAGE.switchControl));
  await showParts(page);
  // `size`'s doc says "the size of the control's box". The only rule keyed on
  // it set a font size and a row height, so the track and the knob were the
  // same 48x24 at both values and the variant grid showed one control twice.
  const boxes = await page.evaluate(() =>
    ["small", "medium"].map((size) => {
      const track = document.querySelector(
        `[role="switch"][data-nd-size="${size}"] .nd-switch-track`,
      );
      const box = track?.getBoundingClientRect();
      return box ? [Math.round(box.width), Math.round(box.height)] : null;
    }),
  );
  expect(boxes[0]).not.toBeNull();
  expect(boxes[1]).not.toBeNull();
  expect(boxes[0][0]).toBeLessThan(boxes[1][0]);
  expect(boxes[0][1]).toBeLessThan(boxes[1][1]);
});

test("the gap around the knob is one gap, on four sides, in both states", async ({ page }) => {
  // WHAT WAS REPORTED, AND WHAT IT ACTUALLY WAS. A switch that is on was said
  // to have a smaller knob than one that is off. The knob was the same circle
  // in both — what differed was the room around it: 5px from the leading edge
  // when off, 7px from the trailing edge when on, 4px above and below either
  // way. Three sums placed it, one per state and one per size, and each of
  // them was right about the number it had been given.
  //
  // A press moves the knob to the other end and changes the track's colour.
  // Nothing else about it may change.
  await page.goto(url(PAGE.switchControl));
  await showParts(page);

  const all = switches(page);
  const total = await all.count();
  let measured = 0;

  for (let index = 0; index < total; index += 1) {
    const flip = all.nth(index);
    const track = flip.locator(".nd-switch-track");
    const name = `${await flip.getAttribute("data-nd-size")} \`${(await flip.textContent()).trim()}\``;
    const refused = (await flip.getAttribute("aria-disabled")) === "true";

    // The pointer is parked away from the control: hover repaints the track,
    // and a measurement taken with the mouse on one switch and off the next is
    // a measurement of two different pictures.
    await page.mouse.move(0, 0);
    const states = [await knobIn(track)];
    if (!refused) {
      await flip.click();
      await page.mouse.move(0, 0);
      // The knob slides, and a screenshot taken mid-transition catches it
      // between the two ends.
      await page.waitForTimeout(400);
      states.push(await knobIn(track));
      await flip.click();
      await page.mouse.move(0, 0);
      await page.waitForTimeout(400);
    }

    for (const [which, shape] of states.entries()) {
      expect(
        shape.found,
        `${name}: ${which ? "after it moved, " : ""}${shape.found} shapes ${shape.axis}, not one knob`,
      ).toBe(1);
      // A circle, and the same circle at both ends.
      expect(Math.abs(shape.knob[0] - shape.knob[1]), `${name}: the knob is not round`).toBeLessThanOrEqual(1);
      // Three of the four gaps ARE the gap; the fourth is the gap plus the
      // travel, and which side that is depends on the state.
      const near = Math.min(shape.start, shape.end);
      const far = Math.max(shape.start, shape.end);
      expect(Math.abs(shape.top - shape.bottom), `${name}: the knob is not centred between the track's edges`).toBeLessThanOrEqual(1);
      expect(Math.abs(near - shape.top), `${name}: ${near}px beside the knob and ${shape.top}px above it`).toBeLessThanOrEqual(1);
      expect(far, `${name}: the knob is not at either end`).toBeGreaterThan(near);
    }

    if (states.length === 2) {
      const [off, on] = states;
      expect(Math.abs(off.knob[0] - on.knob[0]), `${name}: the knob changed size when it moved`).toBeLessThanOrEqual(1);
      expect(
        Math.abs(Math.min(off.start, off.end) - Math.min(on.start, on.end)),
        `${name}: the gap beside the knob changed when the switch moved`,
      ).toBeLessThanOrEqual(1);
      // And it really did move, so nothing above was measured twice.
      expect(off.start < off.end, `${name}: the knob sat at the same end in both states`).toBe(
        on.start > on.end,
      );
    }
    measured += 1;
  }
  expect(measured, "the page shows no switches at all").toBeGreaterThan(4);
});
