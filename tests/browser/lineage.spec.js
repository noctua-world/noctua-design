// The scene, measured rather than described.
//
// WHAT THIS FILE IS FOR. `lineage-layout.spec.js` beside it checks the layout
// engine against the blessed cases, in Node, with no page — so nothing here
// re-checks arithmetic. These are the claims that only a browser can settle:
// that the scene is three-dimensional and correct before any script runs, that
// depth goes AWAY from the reader, that the camera hands over without taking
// the reading order with it, and that a connection can be singled out with
// nothing running.
//
// Every one of these was confirmed by restoring the defect it names. Three of
// them are named after defects this component actually shipped during the hour
// it was written.

import { expect, test } from "@playwright/test";

import { PAGE, url, showParts } from "./pages.js";

const SCENE = ".nd-lineage";
const at = (page, index = 0) => page.locator(SCENE).nth(index);

/**
 * The 4x4 of an element's own transform, or null when it has none.
 *
 * A `matrix()` of six numbers and a `matrix3d()` of sixteen are the same thing
 * — an engine serialises the short form whenever the z terms are zero — so a
 * check that only accepted sixteen would report "no transform at all" for every
 * run that happens to sit on the front plane.
 */
const matrix = (locator) =>
  locator.evaluate((el) => {
    const value = getComputedStyle(el).transform;
    if (!value || value === "none") return null;
    const n = value.slice(value.indexOf("(") + 1, -1).split(",").map(Number);
    if (n.length === 16) return n;
    if (n.length !== 6) return null;
    return [n[0], n[1], 0, 0, n[2], n[3], 0, 0, 0, 0, 1, 0, n[4], n[5], 0, 1];
  });

test.describe("the scene is drawn before anything runs", () => {
  test.use({ javaScriptEnabled: false });

  test("every figure, every connection and every place is server-rendered", async ({ page }) => {
    await page.goto(url(PAGE.lineage));
    await showParts(page);
    const scene = at(page);
    // THE CLAIM THE WHOLE COMPONENT RESTS ON. With the script blocked this is
    // still a family tree: the layout ran in Rust during pre-rendering, and
    // what a reader sees is the scene rather than a placeholder for one.
    await expect(scene.locator(".nd-lineage-piece").first()).toBeVisible();
    expect(await scene.locator(".nd-lineage-piece").count()).toBeGreaterThan(4);
    expect(await scene.locator(".nd-lineage-run").count()).toBeGreaterThan(4);
    // And the places are real numbers rather than a default.
    const places = await scene.locator(".nd-lineage-piece").evaluateAll((els) =>
      els.map((el) => el.style.getPropertyValue("--nd-x")),
    );
    expect(new Set(places).size, "every piece was placed at the same x").toBeGreaterThan(2);
  });

  test("a scene fills its box, and never decides how wide that box is", async ({ page }) => {
    await page.goto(url(PAGE.lineage));
    await showParts(page);
    // THE DEFECT THIS NAMES, AND IT TOOK THREE ATTEMPTS TO NAME IT RIGHT. Every
    // example on this site sits in a flex container that shrink-wraps its
    // items, so a scene with `width: auto` is sized by its own drawing: a
    // family wider than the page made the page wider than the window, and
    // anything that shrank the drawing shrank the box with it — which is how a
    // scene came to be seventy-five pixels wide until a pointer went near it.
    // EACH SCENE AGAINST ITS OWN BOX, not against every other scene. Comparing
    // them to each other was a proxy that held only while every frame on the
    // page was the same frame — the page opens on an overview now, whose frame
    // is drawn at the strong border width and is two pixels narrower inside.
    // The claim was never that the scenes match; it is that a scene does not
    // choose its own width.
    const fields = await page
      .locator(".nd-example-stage .nd-lineage-field:not(.nd-lineage-sample)")
      .evaluateAll((els) =>
        els.map((el) => ({
          scene: Math.round(el.getBoundingClientRect().width),
          box: el.parentElement.clientWidth,
        })),
      );
    expect(fields.length).toBeGreaterThan(4);
    expect(
      fields.filter((one) => Math.abs(one.scene - one.box) > 2),
      "a scene is not the width of the box it was given",
    ).toEqual([]);
    expect(Math.min(...fields.map((one) => one.scene))).toBeGreaterThan(300);
  });

  test("a scene wider than its window pans with the platform's own scrolling", async ({
    page,
  }) => {
    await page.goto(url(PAGE.lineage));
    await showParts(page);
    // THE SIZING DOCTRINE, in one assertion. Text never scales with the
    // layout, so a scene wider than the squeeze floor allows has to PAN — the
    // platform's own scrollbar with no script, the camera with one. The old
    // model shrank the cell until everything fit, and the page rendered
    // milestone notes at seven pixels. `lineage-audit.spec.js` owns the rest
    // of the geometry, at five widths, in both locales.
    const field = await at(page)
      .locator("[data-nd-lineage-field]")
      .evaluate((el) => {
        const style = getComputedStyle(el);
        return {
          overflowX: style.overflowX,
          scrollbar: style.scrollbarWidth,
          // No horizontal bar takes height from the box: what the borders do
          // not account for would be a classic scrollbar being worn.
          gutter:
            el.offsetHeight -
            el.clientHeight -
            Number.parseFloat(style.borderTopWidth) -
            Number.parseFloat(style.borderBottomWidth),
        };
      });
    expect(field.overflowX).toBe("auto");
    // IT PANS WITHOUT WEARING A BAR. A drawing under a permanent grey
    // scrollbar was reported outright; the scene still scrolls by wheel,
    // swipe and keyboard, it just does not dress as a scroll container.
    expect(field.scrollbar).toBe("none");
    expect(field.gutter).toBe(0);
  });

  test("a connection can be singled out with nothing running", async ({ page }) => {
    await page.goto(url(PAGE.lineage));
    await showParts(page);
    const picking = page.locator(`${SCENE}[data-nd-pick="both"]`).first();
    const box = picking.locator(".nd-lineage-pick").first();
    await expect(box).toHaveCount(1);
    // A REAL CHECKBOX, which is the whole reason picking survives here: it is
    // focusable, it posts with a form, and `:checked` is what draws the
    // connection heavier. Ticked through the DOM rather than clicked, because
    // the box is a hairline the reader never points at — the label over the
    // connection is what they press.
    const run = picking.locator(".nd-lineage-run").first();
    const rest = await run.evaluate((el) => getComputedStyle(el).height);
    await box.evaluate((el) => {
      el.checked = true;
    });
    const height = await run.evaluate((el) => getComputedStyle(el).height);
    expect(height, "a picked connection is drawn no heavier than an unpicked one").not.toBe(rest);
  });
});

test.describe("the third dimension", () => {
  test("a horizontal run carries a transform", async ({ page }) => {
    await page.goto(url(PAGE.lineage));
    await showParts(page);
    // THE DEFECT THIS NAMES. `translate3d(…) none` is not a valid transform
    // list, and an invalid transform drops the WHOLE declaration — so every
    // horizontal run piled up at the scene's origin while every vertical one,
    // which has a real rotation, was correct. It read as a routing bug.
    const run = page.locator('.nd-lineage-run[data-nd-axis="x"]').first();
    const values = await matrix(run);
    expect(values, "a horizontal run has no transform at all").not.toBeNull();
    expect(Math.abs(values[12]) + Math.abs(values[13]), "it is at the origin").toBeGreaterThan(1);
    // AND IT LIES BEHIND THE PLANE OF THE FACES. A run is drawn to its
    // piece's centre and the card covers the overlap — but coplanar, the
    // cover held only at rest: turned, the depth sort drew the line over the
    // portrait it entered. Two pixels back, the planes are parallel and the
    // card wins at every angle.
    expect(values[14], "the run lies in the faces' own plane").toBeLessThanOrEqual(-2);
  });

  test("a deep figure is never see-through", async ({ page }) => {
    await page.goto(url(PAGE.lineage));
    await showParts(page);
    // `opacity` fog once made every deeper card transparent, and the legs
    // drawn to its own centre read straight through its face. The fog is
    // deleted — a reader said the effect made no sense — and this stays so a
    // future depth cue cannot bring the transparency back with it.
    const deep = page.locator('.nd-lineage[data-nd-view="deep"]').first();
    await deep.scrollIntoViewIfNeeded();
    const cards = deep.locator(".nd-lineage-piece .nd-lineage-card");
    const seen = await cards.evaluateAll((all) =>
      all
        .filter((card) => card.getClientRects().length > 0)
        .map((card) => getComputedStyle(card).opacity),
    );
    expect(seen.length).toBeGreaterThan(3);
    for (const opacity of seen) expect(opacity, "a deep card went translucent").toBe("1");
  });

  test("depth goes away from the reader, never toward", async ({ page }) => {
    await page.goto(url(PAGE.lineage));
    await showParts(page);
    const deep = page.locator(`${SCENE}[data-nd-view="deep"]`).first();
    const far = deep.locator('.nd-lineage-piece:not([style*="--nd-z:0"])').first();
    const values = await matrix(far);
    expect(values, "nothing in the deep view is on a second plane").not.toBeNull();
    // THE DEFECT THIS NAMES. `+z` in CSS is toward the camera, so a plane the
    // engine counted as "further back" was drawn magnified by d / (d - z) —
    // and a scene of two planes was enough to make the compositor stop
    // answering a screenshot for thirty seconds.
    expect(values[14], "a deeper plane is drawn closer to the reader").toBeLessThan(0);
  });

  test("a figure is a box rather than a sticker", async ({ page }) => {
    await page.goto(url(PAGE.lineage));
    await showParts(page);
    const piece = at(page).locator(".nd-lineage-piece").first();
    // THE DEFECT THIS NAMES: the walls used to wait for a reader to turn the
    // camera, which left the resting view — the one everybody sees — drawing
    // flat cards with a rectangle behind them, and that is exactly what it was
    // called. A scene has a real perspective, so a figure away from the middle
    // of the view shows a wall with the camera straight on, the way a row of
    // boxes on a desk does.
    // AN EXTRUSION, NOT A STACK AND NOT FOUR SQUARE WALLS. The back face
    // repeats the card's rounded silhouette a slab behind it, and each edge
    // wall spans only the straight run between the corner radii, folded 90°
    // into the depth — the two bodies this shipped before were a deck of
    // visibly separate cards and a rounded card with square corners sticking
    // out of it.
    const back = piece.locator('.nd-lineage-body > [data-nd-face="back"]');
    await expect(back).toHaveCount(1);
    const backDepth = await back.evaluate(
      (el) => new DOMMatrix(getComputedStyle(el).transform).m43,
    );
    expect(backDepth, "the back face is not behind the card").toBeLessThan(0);
    const backRadius = await back.evaluate((el) => getComputedStyle(el).borderTopLeftRadius);
    expect(Number.parseFloat(backRadius), "the back face lost the card's corner").toBeGreaterThan(
      0,
    );
    for (const face of ["top", "bottom", "start", "end"]) {
      const wall = piece.locator(`.nd-lineage-body > [data-nd-face="${face}"]`);
      await expect(wall).toHaveCount(1);
      const folded = await wall.evaluate((el) => {
        const m = new DOMMatrix(getComputedStyle(el).transform);
        // A 90° fold about either axis zeroes one in-plane diagonal term.
        return Math.abs(m.m22) < 0.01 || Math.abs(m.m11) < 0.01;
      });
      expect(folded, `the ${face} wall is not folded into the depth`).toBe(true);
    }
    // A figure with no thickness has no walls, which is what `none` means.
    const flat = page.locator(`${SCENE}[data-nd-thickness="none"] .nd-lineage-body`).first();
    expect(await flat.evaluate((el) => getComputedStyle(el).display)).toBe("none");
  });

  test("a glance changes nothing, and a press changes only the camera", async ({ page }) => {
    await page.goto(url(PAGE.lineage));
    await showParts(page);
    // The FIRST scene only: the whole-page sweep also caught the owl hatching
    // into a scene far below as it scrolled into view — which is that example
    // doing its job, not this one failing.
    const where = () =>
      at(page)
        .locator(".nd-lineage-piece")
        .evaluateAll((pieces) =>
          pieces.map((piece) => {
            const box = piece.getBoundingClientRect();
            return [box.left, box.top + window.scrollY, box.width, box.height];
          }),
        );
    const before = await where();
    expect(before.length).toBeGreaterThan(5);

    // THE DEFECT THIS NAMES, in the reader's own words: hovering an example
    // moved the whole thing down and took its scrollbars with it. The camera
    // now mounts on an ACT — a press, a key, focus — and engages only at its
    // first movement, folding the platform's scroll into its own pan so the
    // pixels under the pointer never move.
    await page.evaluate(() => {
      for (const field of document.querySelectorAll("[data-nd-lineage-field]"))
        field.dispatchEvent(new PointerEvent("pointerenter", { bubbles: false }));
    });
    await page.waitForTimeout(600);
    await expect(at(page)).toHaveAttribute("data-nd-camera", "css");
    const hovered = await where();
    expect(hovered, "hovering changed the drawing").toEqual(before);

    // A tap mounts the module; the drawing still must not move. A real click
    // is never perfectly still — the wander here is the mouse settling as the
    // button goes down, and exactly this click once engaged the camera as a
    // one-pixel orbit and squeezed the scene to a strip two lines tall.
    // From a corner of the field, where there is air rather than a card — a
    // press on a card belongs to the card. The TOP corner, because on the
    // suite's 720-pixel viewport the field's bottom edge is below the fold and
    // a press there lands on nothing.
    const field = at(page).locator("[data-nd-lineage-field]");
    await field.scrollIntoViewIfNeeded();
    const box = await field.boundingBox();
    await page.mouse.move(box.x + box.width - 40, box.y + 30);
    await page.mouse.down();
    await page.mouse.move(box.x + box.width - 42, box.y + 31);
    await page.mouse.up();
    await page.waitForTimeout(800);
    await expect(at(page)).toHaveAttribute("data-nd-camera", "css");
    const tapped = await where();
    expect(tapped, "a tap without a drag moved the drawing").toEqual(before);

    // A drag engages the camera and turns the scene — that one is on purpose.
    // The WINDOW holds still while the scene inside it turns: engaging takes
    // the space out of the flow, and a field left to its padding collapsed.
    await page.mouse.down();
    await page.mouse.move(box.x + box.width - 110, box.y + 60, { steps: 3 });
    await page.mouse.up();
    await expect(at(page)).toHaveAttribute("data-nd-camera", "three");
    const engaged = await field.boundingBox();
    expect(engaged.height, "engaging the camera resized the window").toBeCloseTo(box.height, 0);
    expect(engaged.y, "engaging the camera moved the window").toBeCloseTo(box.y, 0);
  });

  test("the left button moves, the right button turns, the wheel closes in", async ({
    page,
  }) => {
    await page.goto(url(PAGE.lineage));
    await showParts(page);
    const scene = at(page);
    const field = scene.locator("[data-nd-lineage-field]");
    await field.scrollIntoViewIfNeeded();
    const box = await field.boundingBox();
    const state = () =>
      scene.evaluate((root) => {
        const style = getComputedStyle(root.querySelector("[data-nd-lineage-field]"));
        return {
          yaw: style.getPropertyValue("--nd-yaw"),
          pan: style.getPropertyValue("--nd-pan-x"),
          zoom: style.getPropertyValue("--nd-zoom"),
        };
      });

    // The wheel over a scene at rest belongs to the page: it scrolls, and the
    // scene neither zooms nor engages. `overscroll-behavior: contain` on both
    // axes once swallowed exactly this wheel, over every scene on the page.
    const scrolled = await page.evaluate(() => window.scrollY);
    await page.mouse.move(box.x + 60, box.y + 60);
    await page.mouse.wheel(0, 120);
    await page.waitForTimeout(300);
    expect((await state()).zoom, "a wheel over a resting scene zoomed it").toBe("1");
    await expect(scene).toHaveAttribute("data-nd-camera", "css");
    expect(
      await page.evaluate(() => window.scrollY),
      "a wheel over a resting scene no longer scrolls the page",
    ).not.toBe(scrolled);
    await page.evaluate((y) => window.scrollTo(0, y), scrolled);

    // The left button pans — moving the picture is the gesture a reader wants
    // first, so it is on the button that needs no knowledge — and does not turn.
    await page.mouse.move(box.x + 60, box.y + 60);
    await page.mouse.down();
    await page.mouse.move(box.x + 160, box.y + 100, { steps: 5 });
    await page.mouse.up();
    await page.waitForTimeout(600);
    const panned = await state();
    expect(panned.pan, "the left drag did not move the scene").not.toBe("0");
    expect(panned.yaw, "the left drag turned the scene").toBe("0deg");

    // Engaged, the wheel is the zoom, which is what every drawing tool does.
    await page.mouse.wheel(0, 240);
    await page.waitForTimeout(200);
    expect(Number.parseFloat((await state()).zoom), "the wheel did not zoom").toBeLessThan(1);

    // And the right button turns.
    await page.mouse.move(box.x + 200, box.y + 200);
    await page.mouse.down({ button: "right" });
    await page.mouse.move(box.x + 280, box.y + 240, { steps: 4 });
    await page.mouse.up({ button: "right" });
    await page.waitForTimeout(200);
    expect((await state()).yaw, "the right drag did not turn the scene").not.toBe("0deg");
  });

  test("solid lettering is a stack a reader can see", async ({ page }) => {
    await page.goto(url(PAGE.lineage));
    await showParts(page);
    // `lettering: solid` extrudes every name and date: five aria-hidden plies
    // recede behind the real text and turn with the scene. The wild example
    // is the one that sets it; everywhere else the plies must not exist —
    // or, in the variant specimen that carries them inertly, must not show.
    // BY NAME, not by first match: the variant grid's solid sample sits
    // earlier in the DOM and carries the specimen's three inert plies.
    const wild = page.locator('[data-nd-lineage="wild"]');
    const name = wild.locator(".nd-lineage-name").first();
    expect(await name.locator("[data-nd-ply]").count(), "no plies behind the name").toBe(5);
    const shown = await name
      .locator("[data-nd-ply]")
      .evaluateAll((plies) => plies.filter((ply) => getComputedStyle(ply).display !== "none").length);
    expect(shown, "the plies exist but are hidden").toBe(5);
    const flat = page.locator('.nd-lineage[data-nd-lettering="flat"][data-nd-lineage]').first();
    expect(await flat.locator(".nd-lineage-name [data-nd-ply]").count()).toBe(0);
  });

  test("a shy line never touches a figure", async ({ page }) => {
    await page.goto(url(PAGE.lineage));
    await showParts(page);
    // `reach: shy` trims every figure-bound run end by 0.35 of a pitch — in
    // the wild scene, whose figures are bare words, a line that touched one
    // would strike it through. The near side — that a shy end still stops
    // NEAR its figure — is the audit's landing rule, with the trim added to
    // its tolerance.
    const wild = page.locator('[data-nd-lineage="wild"]');
    const crossings = await wild.evaluate((scene) => {
      // WHAT A READER SEES, not the card's box: a bare figure's card is a
      // transparent anchor wider than its words, and a line clear of every
      // visible thing is what "never touches" means. Figures only — a union
      // mark sits on its own line by design.
      const boxes = [
        ...scene.querySelectorAll(
          ".nd-lineage-piece:not([data-nd-piece='mark']) :is(.nd-lineage-name, .nd-lineage-meta, .nd-lineage-portrait, .nd-lineage-count)",
        ),
      ].map((el) => el.getBoundingClientRect());
      let touching = 0;
      for (const run of scene.querySelectorAll('.nd-lineage-run[data-nd-axis="x"], .nd-lineage-run[data-nd-axis="y"]')) {
        const r = run.getBoundingClientRect();
        if (r.width < 1 && r.height < 1) continue;
        for (const b of boxes) {
          const x = Math.min(r.right, b.right) - Math.max(r.left, b.left);
          const y = Math.min(r.bottom, b.bottom) - Math.max(r.top, b.top);
          if (x > 1 && y > 1) touching += 1;
        }
      }
      return touching;
    });
    expect(crossings, "a shy line touches a figure").toBe(0);
  });

  test("a beam is a closed tube, and a ribbon is only itself", async ({ page }) => {
    await page.goto(url(PAGE.lineage));
    await showParts(page);
    // `line: Beam` closes every run into a square section — the run is the
    // front face and three children fold the back and the walls, sharing its
    // background so a dashed or coloured run dashes and colours all four.
    const beams = page.locator('.nd-lineage[data-nd-line="beam"]').first();
    const faces = await beams
      .locator(".nd-lineage-run")
      .first()
      .locator("[data-nd-face]")
      .count();
    expect(faces, "a beam run is missing its section").toBe(3);
    const ribbon = page.locator('.nd-lineage[data-nd-line="ribbon"]').first();
    expect(await ribbon.locator(".nd-lineage-run [data-nd-face]").count()).toBe(0);
  });

  test("zooming in becomes flight, and the flight passes through", async ({ page }) => {
    await page.goto(url(PAGE.lineage));
    await showParts(page);
    // THE INVISIBLE WALL THIS NAMES: `distance = lens / zoom` approaches the
    // front plane for ever without arriving, so the deeper planes' scale
    // converges and a reader pressing toward Fay and Dan stopped moving.
    // Past `FLIGHT` the wheel travels the camera through the scene instead,
    // and what passes behind it is hidden rather than left to CSS, which
    // draws no frustum and would smear it across the screen.
    const deep = page.locator('.nd-lineage[data-nd-view="deep"]').first();
    const field = deep.locator("[data-nd-lineage-field]");
    await field.scrollIntoViewIfNeeded();
    await field.focus();
    await page.waitForTimeout(800);
    const hidden = () =>
      deep.evaluate(
        (root) =>
          [...root.querySelectorAll(".nd-lineage-piece")].filter(
            (piece) => piece.style.visibility === "hidden",
          ).length,
      );
    for (let press = 0; press < 16; press += 1) await page.keyboard.press("+");
    await page.waitForTimeout(300);
    const inside = await hidden();
    expect(inside, "nothing ever passed behind the camera").toBeGreaterThan(0);
    // And the way back is the same road: the flight retraces itself, then
    // magnification resumes, and everything hidden returns.
    for (let press = 0; press < 40; press += 1) await page.keyboard.press("-");
    await page.waitForTimeout(300);
    expect(await hidden(), "the flight back left pieces hidden").toBe(0);
  });

  test("zooming out shrinks the picture, never the drawing", async ({ page }) => {
    await page.goto(url(PAGE.lineage));
    await showParts(page);
    // THE DEFECT THIS NAMES: the zoom was multiplied into the grid's pitches,
    // so zooming out re-laid the columns tighter around cards whose lengths
    // are fixed — narrow cards, full-size type, rows drawn into each other
    // until the figures overlapped. A camera zoom is a magnification of the
    // finished picture: everything shrinks together, in proportion, like
    // stepping back from it.
    // ANY two figures on different rows — the invariant is the component's,
    // and a spec that named two figures from the demo family was hostage to
    // the example's cast.
    const field = at(page).locator("[data-nd-lineage-field]");
    await field.scrollIntoViewIfNeeded();
    const pair = await at(page).evaluate((scene) => {
      const pieces = [...scene.querySelectorAll('.nd-lineage-piece[data-nd-piece="figure"]')];
      const first = pieces[0];
      const off = pieces.find(
        (piece) => piece.style.getPropertyValue("--nd-y") !== first.style.getPropertyValue("--nd-y"),
      );
      return [first?.dataset.ndId, off?.dataset.ndId];
    });
    expect(pair[0], "no figure to measure").toBeTruthy();
    expect(pair[1], "no second row to measure").toBeTruthy();
    const card = at(page).locator(`[data-nd-id="${pair[0]}"] .nd-lineage-card`);
    const below = at(page).locator(`[data-nd-id="${pair[1]}"] .nd-lineage-card`);
    const rest = { card: await card.boundingBox(), below: await below.boundingBox() };
    // Focus is the act that mounts the module; the wait is for the import,
    // the same grace the walking tests give it.
    await field.focus();
    await page.waitForTimeout(800);
    for (let press = 0; press < 4; press += 1) await page.keyboard.press("-");
    await expect(at(page)).toHaveAttribute("data-nd-camera", "three");
    await page.waitForTimeout(200);
    const waned = { card: await card.boundingBox(), below: await below.boundingBox() };
    expect(waned.card.width, "zooming out did not shrink the card").toBeLessThan(
      rest.card.width * 0.8,
    );
    // Two invariants, because the drawing can be distorted two ways: a card
    // squeezed against its own height, and a row pitch squeezed against the
    // cards standing on it — which is the one that drew figures into each
    // other.
    const shape = (box) => box.width / box.height;
    const pitch = (pair) => (pair.below.y - pair.card.y) / pair.card.height;
    expect(shape(waned.card), "the zoom distorted the card").toBeCloseTo(shape(rest.card), 1);
    expect(pitch(waned), "the zoom drew the rows into the cards").toBeCloseTo(pitch(rest), 1);
  });
});

test.describe("the variant grid", () => {
  // KEPT ALONGSIDE `variants.spec.js`, WHICH ASKS THE SAME QUESTION EVERYWHERE.
  // That one fingerprints the whole stage; this one fingerprints the SAMPLE, and
  // for a scene that is the stricter question — `reach` draws two identical
  // samples inside two stages that differ by a pixel of chrome, and only the
  // narrower reading calls that what it is.
  test("no two values of one prop draw the same picture", async ({ page }) => {
    await page.goto(url(PAGE.lineage));
    await showParts(page);
    // THE DEFECT THIS NAMES, in the reader's words: side-by-side examples that
    // should show different results showing exactly the same thing. Four pairs
    // did — `resting` and `raked`, `thin` and `slab`, `bond` and `figure`,
    // `tall` and `full` — because the sample is drawn flat and at a fixed size,
    // so a camera angle, a thickness and a window height had nowhere to show.
    // A grid that draws one picture under two names is worse than no grid.
    const groups = await page.evaluate(() => {
      const fingerprint = (root) =>
        // THE SAMPLE'S OWN BOX IS PART OF THE PICTURE. `height` varies nothing
        // but the window, and `view` varies the air around the drawing: read
        // only the descendants and both groups look like three copies.
        [root, ...root.querySelectorAll("*")]
          .map((el) => {
            const style = getComputedStyle(el);
            const box = el.getBoundingClientRect();
            return [
              el.className,
              style.transform,
              style.display,
              style.borderColor,
              style.backgroundColor,
              style.opacity,
              Math.round(box.width),
              Math.round(box.height),
            ].join("|");
          })
          .join("\n");
      return [...document.querySelectorAll(".nd-example")]
        .filter((example) => example.querySelector(".nd-lineage-sample"))
        .map((example) => ({
          prop: example.querySelector(".nd-example-source code")?.textContent?.trim() ?? "?",
          values: [...example.querySelectorAll(".nd-variant-item")].map((item) => ({
            name: item.querySelector(".nd-lineage")?.getAttribute("aria-labelledby") ?? "?",
            print: fingerprint(item.querySelector(".nd-lineage-sample")),
          })),
        }));
    });

    expect(groups.length, "the variant grid did not render").toBeGreaterThan(5);
    const twins = [];
    for (const group of groups) {
      const seen = new Map();
      for (const value of group.values) {
        const first = seen.get(value.print);
        if (first) twins.push(`${group.prop}: ${first} and ${value.name} are one picture`);
        else seen.set(value.print, value.name);
      }
    }
    expect(twins, "a variant sample lies about its own effect").toEqual([]);
  });
});

test.describe("the camera", () => {
  test("it mounts, and the reading order survives the mount", async ({ page }) => {
    await page.goto(url(PAGE.lineage));
    await showParts(page);
    const scene = at(page);
    await expect(scene).toHaveAttribute("data-nd-camera", "css");
    const before = await scene
      .locator(".nd-lineage-piece")
      .evaluateAll((els) => els.map((el) => el.dataset.ndId));

    const field = scene.locator("[data-nd-lineage-field]");
    await field.scrollIntoViewIfNeeded();
    const box = await field.boundingBox();
    await page.mouse.move(box.x + box.width - 40, box.y + 30);
    await page.mouse.down();
    await page.mouse.move(box.x + box.width - 110, box.y + 60, { steps: 3 });
    await page.mouse.up();
    await expect(scene).toHaveAttribute("data-nd-camera", "three", { timeout: 10000 });

    // THE HAZARD `CSS3DRenderer` BRINGS. It re-parents every element it places
    // into its own camera element, so the renderer is given ONE object — the
    // whole rigid scene — and this is what says so: after the mount every
    // figure is still where the server put it, in the order it put them.
    const after = await scene
      .locator(".nd-lineage-piece")
      .evaluateAll((els) => els.map((el) => el.dataset.ndId));
    expect(after).toEqual(before);
    await expect(scene.locator("[data-nd-lineage-field]")).toHaveAttribute("role", "group");
  });

  test("three.js is fetched on the first interaction and not before", async ({ page }) => {
    const asked = [];
    page.on("request", (request) => {
      if (request.url().includes("three-css3d")) asked.push(request.url());
    });
    await page.goto(url(PAGE.lineage));
    await showParts(page);
    await page.waitForTimeout(500);
    // A page that is only read costs nothing. 235 KB on every page view for a
    // camera nobody moved is the trade this refuses.
    expect(asked, "three.js was fetched on page load").toHaveLength(0);
    // A glance is not an interaction: hovering fetches nothing.
    await at(page).locator("[data-nd-lineage-field]").hover();
    await page.waitForTimeout(600);
    expect(asked, "three.js was fetched on a hover").toHaveLength(0);
    const box = await at(page).locator("[data-nd-lineage-field]").boundingBox();
    await page.mouse.click(box.x + box.width / 2, box.y + box.height / 2);
    await expect.poll(() => asked.length, { timeout: 10000 }).toBe(1);
  });

  test("the camera controls say what they are, in the page's own language", async ({ page }) => {
    await page.goto(url(PAGE.lineage, "pt-BR"));
    await showParts(page);
    const keys = page.locator(".nd-lineage-camera-key");
    await expect(keys.first()).toBeVisible();
    // ONE SCENE'S CONTROLS. The page carries two scenes now — the overview and
    // the told one — so counting every key on the page counted them twice.
    const words = await page
      .locator(".nd-lineage:has(.nd-lineage-camera-key)")
      .first()
      .locator(".nd-lineage-camera-key")
      .evaluateAll((els) => els.map((el) => el.textContent.trim()));
    expect(words.length).toBe(3);

    await page.goto(url(PAGE.lineage, "en"));
    await showParts(page);
    await expect(page.locator(".nd-lineage-camera-key").first()).toBeVisible();
    const english = await page
      .locator(".nd-lineage-camera-key")
      .evaluateAll((els) => els.map((el) => el.textContent.trim()));
    // Every word this component's script says is a prop, so a Portuguese page
    // gets Portuguese controls. A script with a string in it has one language,
    // and this is what would catch that: the two pages would agree.
    expect(words).not.toEqual(english);
  });
});

test.describe("the walk", () => {
  test("the arrows walk the family rather than the DOM", async ({ page }) => {
    await page.goto(url(PAGE.lineage));
    await showParts(page);
    const scene = at(page);
    const field = scene.locator("[data-nd-lineage-field]");
    await field.focus();
    // Focus mounts the module; give the import a beat before the first press.
    await page.waitForTimeout(800);
    await page.keyboard.press("ArrowDown");
    const focused = await page.evaluate(() => document.activeElement?.dataset?.ndId ?? null);
    expect(focused, "a press moved the focus nowhere").not.toBeNull();
  });

  test("the connections stop being forty tab stops", async ({ page }) => {
    await page.goto(url(PAGE.lineage));
    await showParts(page);
    const scene = at(page);
    await scene.locator("[data-nd-lineage-field]").focus();
    await page.waitForTimeout(800);
    // With no script every tie is a real link and a real tab stop, which is
    // correct and is forty presses to cross a family. The script answers the
    // arrows instead and takes them out of the tab order — the roving model
    // `Tree` and `Toolbar` already use.
    const stops = await scene
      .locator(".nd-lineage-tie")
      .evaluateAll((els) => els.filter((el) => el.tabIndex >= 0).length);
    expect(stops).toBe(0);
  });

  test("a picked connection says so in an attribute as well as in a pseudo-class", async ({
    page,
  }) => {
    await page.goto(url(PAGE.lineage));
    await showParts(page);
    const picking = page.locator(`${SCENE}[data-nd-pick="both"]`).first();
    await picking.locator("[data-nd-lineage-field]").focus();
    await page.waitForTimeout(800);
    const bond = picking.locator(".nd-lineage-bond").first();
    await expect(bond).not.toHaveAttribute("data-nd-picked", /.*/u);
    // A STATE NOTHING WRITES IS A STATE THAT DOES NOTHING, and this contract
    // declared one: `:checked` draws the connection heavier with nothing
    // running, but a pseudo-class is not an attribute and a consumer querying
    // this would have found nothing at all.
    await bond.locator(".nd-lineage-pick").dispatchEvent("click");
    await expect(bond).toHaveAttribute("data-nd-picked", "");
  });

  test("a tie names a person, never a union", async ({ page }) => {
    await page.goto(url(PAGE.lineage));
    await showParts(page);
    const names = await at(page)
      .locator(".nd-lineage-tie")
      .evaluateAll((els) => els.map((el) => el.textContent.trim()));
    expect(names.length).toBeGreaterThan(2);
    // THE DEFECT THIS NAMES. A union is a mark, so a figure's only direct
    // connection is to `u0` — and a visually hidden link announcing "u0" is a
    // link that says nothing. The pseudo-locale check caught it on the page
    // before a reader could.
    for (const name of names) expect(name).not.toMatch(/^u\d+$/u);
  });
});

test.describe("what it refuses to do", () => {
  test("no scene casts a shadow", async ({ page }) => {
    await page.goto(url(PAGE.lineage));
    await showParts(page);
    // §9.7, and it binds a component that could so easily have reached for one:
    // separation is a hairline, never elevation. Depth here is perspective, a
    // real slab edge.
    const shadows = await page.evaluate(() =>
      [...document.querySelectorAll(".nd-lineage *")]
        .filter((el) => getComputedStyle(el).boxShadow !== "none")
        .map((el) => el.className),
    );
    expect(shadows).toEqual([]);
  });

  test("an empty scene says so rather than opening onto nothing", async ({ page }) => {
    await page.goto(url(PAGE.lineage));
    await showParts(page);
    const empty = page.locator(".nd-lineage-empty").first();
    await expect(empty).toBeVisible();
    expect((await empty.textContent()).trim().length).toBeGreaterThan(0);
  });

  test("a graph nobody could parse is counted rather than swallowed", async ({ page }) => {
    await page.goto(url(PAGE.lineage));
    await showParts(page);
    const skipped = await page
      .locator(`${SCENE}[data-nd-skipped]`)
      .first()
      .getAttribute("data-nd-skipped");
    expect(Number(skipped)).toBeGreaterThan(0);
  });
});
