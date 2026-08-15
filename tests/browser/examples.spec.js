// The documentation site's OWN examples, measured.
//
// WHAT THIS FILE IS FOR. `lineage.spec.js` holds the component's claims;
// these are the claims about things the docs site builds AROUND the
// component and never ships — the model stage (`model.js`), copied straight
// to the site and absent from `dist/`. Filing them under the component's
// own spec said the component depended on them, and it does not.

import { expect, test } from "@playwright/test";

import { PAGE, url, showParts } from "./pages.js";

test.describe("the model stage", () => {
  test("the model stage is a feature, and the owl is only its first file", async ({
    page,
  }) => {
    await page.goto(url(PAGE.lineage));
    await showParts(page);
    // THE INSTRUCTION THIS HOLDS: the viewer must receive any `.glb` plus
    // its parameters, and nothing in it may be shaped by the one model the
    // documentation happens to show. The first version was owl-shaped —
    // dispatched by an owl attribute, sized by an owl rule, in the owl's own
    // module — and the reader called it out in as many words.
    const stage = page.locator("[data-nd-model]").first();
    await expect(stage).toHaveAttribute("data-nd-model", /\.glb$/);
    expect(await stage.getAttribute("data-nd-owl"), "the stage is owl-dispatched").toBeNull();
    const module = await page.evaluate(async () => {
      const base = document.documentElement.getAttribute("data-noctua-base") || "";
      return (await fetch(`${base}assets/model.js`)).text();
    });
    expect(module.toLowerCase().includes("owl"), "model.js knows what an owl is").toBe(false);

    // And the defaults carry the example: no `data-nd-model-*` parameter is
    // set, the model hatches, and its animation runs.
    for (const knob of ["fit", "size", "clip", "gaze", "fov", "ratio"]) {
      expect(await stage.getAttribute(`data-nd-model-${knob}`)).toBeNull();
    }
    await stage.scrollIntoViewIfNeeded();
    await expect(stage.locator("canvas.nd-model")).toHaveCount(1, { timeout: 30000 });
  });

  test("no orientation puts the model outside its canvas", async ({ page }) => {
    await page.goto(url(PAGE.lineage));
    await showParts(page);
    // THE INSTRUCTION THIS HOLDS: the model turns with the scene, and no
    // reachable orientation may cut it at the canvas edge. The fit was once
    // computed for the resting orientation only — a box the model fit while
    // facing front — and orbiting the scene swung a long axis past it, so a
    // wing left the frame at the very angles a reader orbits to. Twice
    // photographed, twice reported. Only a bound that does not care which
    // way the model faces survives this test's orbit.
    const wild = page.locator('[data-nd-lineage="wild"]');
    await wild.scrollIntoViewIfNeeded();
    const stage = page.locator("[data-nd-model]").first();
    await expect(stage.locator("canvas.nd-model")).toHaveCount(1, { timeout: 30000 });
    await page.waitForTimeout(600);

    // Orbit hard to the reported family of angles, then let the billboard
    // settle into the new bearing.
    const field = wild.locator("[data-nd-lineage-field]");
    const box = await field.boundingBox();
    await page.mouse.move(box.x + 300, box.y + 300);
    await page.mouse.down({ button: "right" });
    await page.mouse.move(box.x + 560, box.y + 330, { steps: 8 });
    await page.mouse.up({ button: "right" });
    await page.waitForTimeout(500);

    // Watch ninety frames of flight — more than a full wingbeat — copying
    // the live canvas each frame (the buffer is readable until the frame
    // composites) and keeping the strongest alpha seen on a two-pixel ring
    // at the canvas edge. A clipped wing crosses that ring at full opacity.
    const worst = await stage.evaluate(async (element) => {
      const canvas = element.querySelector("canvas.nd-model");
      const flat = document.createElement("canvas");
      flat.width = canvas.width;
      flat.height = canvas.height;
      const brush = flat.getContext("2d", { willReadFrequently: true });
      let seen = 0;
      for (let frame = 0; frame < 90; frame += 1) {
        await new Promise((done) => window.requestAnimationFrame(done));
        brush.clearRect(0, 0, flat.width, flat.height);
        brush.drawImage(canvas, 0, 0);
        const ring = [
          brush.getImageData(0, 0, flat.width, 2).data,
          brush.getImageData(0, flat.height - 2, flat.width, 2).data,
          brush.getImageData(0, 0, 2, flat.height).data,
          brush.getImageData(flat.width - 2, 0, 2, flat.height).data,
        ];
        for (const band of ring) {
          for (let at = 3; at < band.length; at += 4) seen = Math.max(seen, band[at]);
        }
      }
      return seen;
    });
    expect(worst, "model pixels on the canvas edge").toBeLessThanOrEqual(8);
  });

  test("the model stands larger than its element, uncut", async ({ page }) => {
    await page.goto(url(PAGE.lineage));
    await showParts(page);
    // THE INSTRUCTION THIS HOLDS: fitting the whole pose envelope's sphere
    // inside the element made the model SMALL — the reader had asked for
    // LARGER than the axis fit ever showed, and got the opposite. The
    // element's box is a window, not a cage: the canvas overdraws it
    // (`--nd-model-grow`) so the performance can exceed the element while
    // the no-cut guarantee moves to the canvas's own, farther edge.
    const stage = page.locator("[data-nd-model]").first();
    await stage.scrollIntoViewIfNeeded();
    await expect(stage.locator("canvas.nd-model")).toHaveCount(1, { timeout: 30000 });
    await page.waitForTimeout(600);
    const seen = await stage.evaluate(async (element) => {
      const canvas = element.querySelector("canvas.nd-model");
      const grow =
        Number.parseFloat(getComputedStyle(element).getPropertyValue("--nd-model-grow")) || 1;
      const flat = document.createElement("canvas");
      flat.width = canvas.width;
      flat.height = canvas.height;
      const brush = flat.getContext("2d", { willReadFrequently: true });
      let left = Infinity;
      let right = -Infinity;
      let top = Infinity;
      let bottom = -Infinity;
      for (let frame = 0; frame < 75; frame += 1) {
        await new Promise((done) => window.requestAnimationFrame(done));
        brush.clearRect(0, 0, flat.width, flat.height);
        brush.drawImage(canvas, 0, 0);
        const read = brush.getImageData(0, 0, flat.width, flat.height).data;
        for (let at = 3; at < read.length; at += 4) {
          if (read[at] > 8) {
            const dot = (at - 3) / 4;
            const x = dot % flat.width;
            const y = (dot - x) / flat.width;
            if (x < left) left = x;
            if (x > right) right = x;
            if (y < top) top = y;
            if (y > bottom) bottom = y;
          }
        }
      }
      return {
        wide: right - left,
        tall: bottom - top,
        boxWide: flat.width / grow,
        boxTall: flat.height / grow,
        grow,
      };
    });
    expect(seen.grow, "the canvas does not overdraw its element").toBeGreaterThan(1);
    expect(
      Math.max(seen.wide / seen.boxWide, seen.tall / seen.boxTall),
      "a wingbeat never once exceeded the element's own box",
    ).toBeGreaterThan(1);
  });

  test("closing the camera in sharpens the canvas", async ({ page }) => {
    await page.goto(url(PAGE.lineage));
    await showParts(page);
    // THE INSTRUCTION THIS HOLDS: the scene's zoom scales the canvas up as
    // a PICTURE, and a picture enlarged past its pixels is pixels — the
    // reader photographed the mosaic. The canvas re-renders at the
    // magnification it is shown at, so closing in adds pixels instead of
    // enlarging them.
    const wild = page.locator('[data-nd-lineage="wild"]');
    await wild.scrollIntoViewIfNeeded();
    const stage = page.locator("[data-nd-model]").first();
    const canvas = stage.locator("canvas.nd-model");
    await expect(canvas).toHaveCount(1, { timeout: 30000 });
    await page.waitForTimeout(500);
    const before = await canvas.evaluate((c) => c.width);

    const field = wild.locator("[data-nd-lineage-field]");
    const box = await field.boundingBox();
    await page.mouse.move(box.x + 200, box.y + 200);
    await page.mouse.down();
    await page.mouse.move(box.x + 230, box.y + 210, { steps: 3 });
    await page.mouse.up();
    await page.mouse.wheel(0, -720);
    // The resolution follows on a deliberate delay — a dead band checked at
    // intervals, so a wheel's worth of notches costs one reallocation.
    await page.waitForTimeout(900);
    const after = await canvas.evaluate((c) => c.width);
    expect(after, "the zoomed canvas kept its resting resolution").toBeGreaterThan(before);
  });

  test("`size` doubles the presentation, and the field breathes to hold it", async ({
    page,
  }) => {
    await page.goto(url(PAGE.lineage));
    await showParts(page);
    // THE INSTRUCTION THIS HOLDS: the resting presentation is the default
    // for every model, and `data-nd-model-size="2"` is the dial that shows
    // the same file at exactly twice it — overdraw and all. And because a
    // doubled canvas is taller than the rows, the field must BREATHE: the
    // stage measures its own poke and asks for that much air, so the wings
    // stay inside the component instead of painting over the page above it.
    const queen = page.locator('[data-nd-lineage="queen"]');
    await queen.scrollIntoViewIfNeeded();
    await expect(queen.locator("canvas.nd-model")).toHaveCount(1, { timeout: 30000 });
    const wild = page.locator('[data-nd-lineage="wild"]');
    await wild.scrollIntoViewIfNeeded();
    await expect(wild.locator("canvas.nd-model")).toHaveCount(1, { timeout: 30000 });
    await expect(queen.locator("[data-nd-model]")).toHaveAttribute("data-nd-model-size", "2");
    // The field's air converges over a few measuring rounds once the
    // scene's resting tilt is applied.
    await page.waitForTimeout(1200);
    const seen = await page.evaluate(() => {
      const read = (name) => {
        const stage = document.querySelector(`[data-nd-lineage="${name}"] [data-nd-model]`);
        const canvas = stage.querySelector("canvas.nd-model");
        // Against the FIELD — the box the engaged camera clips to, so a
        // canvas inside it here stays whole in every camera mode.
        const edge = stage.closest("[data-nd-lineage-field]").getBoundingClientRect();
        const brim = canvas.getBoundingClientRect();
        return {
          grow: Number.parseFloat(getComputedStyle(stage).getPropertyValue("--nd-model-grow")),
          canvas: canvas.offsetWidth,
          box: stage.offsetWidth,
          over: brim.top - edge.top,
          under: edge.bottom - brim.bottom,
        };
      };
      return { wild: read("wild"), queen: read("queen") };
    });
    expect(
      seen.queen.grow / seen.wild.grow,
      "the doubled size does not double the overdraw",
    ).toBeCloseTo(2, 1);
    expect(
      seen.queen.canvas / seen.queen.box,
      "the css chain dropped the overdraw",
    ).toBeCloseTo(seen.queen.grow, 1);
    expect(seen.queen.over, "the model pokes past the field's top").toBeGreaterThanOrEqual(-1);
    expect(
      seen.queen.under,
      "the model pokes past the field's bottom",
    ).toBeGreaterThanOrEqual(-1);

    // AND THE WINDOW HOLDS WHEN THE CAMERA ENGAGES. The engaged field
    // CLIPS (`overflow: clip`) — air worn outside the field once became a
    // dead band above a model cut at the field's edge, photographed by the
    // reader at exactly this pose: engaged, orbited back to flat.
    // Back into view first — the wild measurement above scrolled away, and
    // a drag outside the viewport is a drag on nothing.
    await queen.scrollIntoViewIfNeeded();
    const box = await queen.locator("[data-nd-lineage-field]").boundingBox();
    const size = page.viewportSize();
    const grabX = box.x + box.width / 2;
    const grabY = Math.min(Math.max(box.y + box.height * 0.45, 10), size.height - 10);
    await page.mouse.move(grabX, grabY);
    await page.mouse.down({ button: "right" });
    await page.mouse.move(grabX + 120, grabY, { steps: 6 });
    await page.mouse.up({ button: "right" });
    await page.mouse.down({ button: "right" });
    await page.mouse.move(grabX, grabY, { steps: 6 });
    await page.mouse.up({ button: "right" });
    await page.waitForTimeout(400);
    await expect(queen).toHaveAttribute("data-nd-camera", "three");
    const engaged = await page.evaluate(() => {
      const stage = document.querySelector('[data-nd-lineage="queen"] [data-nd-model]');
      const edge = stage.closest("[data-nd-lineage-field]").getBoundingClientRect();
      const brim = stage.querySelector("canvas.nd-model").getBoundingClientRect();
      return { over: brim.top - edge.top, under: edge.bottom - brim.bottom };
    });
    expect(
      engaged.over,
      "the engaged window cuts the model at its top",
    ).toBeGreaterThanOrEqual(-2);
    expect(
      engaged.under,
      "the engaged window cuts the model at its bottom",
    ).toBeGreaterThanOrEqual(-2);
  });

  test("a figure takes any component, and the description stands under the name", async ({
    page,
  }) => {
    await page.goto(url(PAGE.lineage));
    await showParts(page);
    // THE INSTRUCTION THIS HOLDS: the figure's children are ANYTHING, in
    // order — so a description under the name is not a feature, it is one
    // more child a consumer handed over, and it stands exactly where the
    // order says: below the name.
    const queen = page.locator('[data-nd-lineage="queen"]');
    await queen.scrollIntoViewIfNeeded();
    const figure = queen.locator(".nd-lineage-figure");
    const name = figure.locator(".nd-identifier");
    const blurb = figure.locator(".nd-lineage-blurb");
    await expect(blurb).toHaveCount(1);
    const nameBox = await name.boundingBox();
    const blurbBox = await blurb.boundingBox();
    expect(
      blurbBox.y,
      "the description does not stand below the name",
    ).toBeGreaterThan(nameBox.y + nameBox.height - 2);
  });

});
