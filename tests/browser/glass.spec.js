// Does the glass actually sample?
//
// This is the test neither P0 had, and its absence is why both shipped. The
// 49 specs before it asserted that `backdrop-filter` was *declared* and that
// its computed value contained `blur` — which was true of the settings panel
// the whole time it was rendering a flat fill.
//
// A declared `backdrop-filter` is not a sampling one. It stops sampling if any
// ANCESTOR has `filter`, `opacity` below 1, `mask` or a `backdrop-filter` of
// its own, and the computed value on the element itself is unchanged when that
// happens. The only way to know is to look at the pixels.

import { test, expect } from "@playwright/test";
import { ANY, PAGE, SCOPED, closeTheme, openTheme, url } from "./pages.js";
import { contrast, extremes, pixelAt } from "./pixels.js";

/** Variance of the luminance across a screenshot's pixels. */
async function variance(page, buffer) {
  return page.evaluate(async (data) => {
    const blob = await (await fetch(`data:image/png;base64,${data}`)).blob();
    const bitmap = await createImageBitmap(blob);
    const canvas = new OffscreenCanvas(bitmap.width, bitmap.height);
    const ctx = canvas.getContext("2d");
    ctx.drawImage(bitmap, 0, 0);
    const { data: px } = ctx.getImageData(0, 0, bitmap.width, bitmap.height);

    const values = [];
    for (let i = 0; i < px.length; i += 4) values.push((px[i] + px[i + 1] + px[i + 2]) / 3);
    const mean = values.reduce((a, b) => a + b, 0) / values.length;
    return values.reduce((a, v) => a + (v - mean) ** 2, 0) / values.length;
  }, buffer.toString("base64"));
}

/** No ancestor may carry anything that stops the backdrop sampling. */
async function filteredAncestors(page, selector) {
  return page.evaluate((sel) => {
    const el = document.querySelector(sel);
    const found = [];
    for (let node = el?.parentElement; node; node = node.parentElement) {
      const s = getComputedStyle(node);
      if (
        (s.backdropFilter && s.backdropFilter !== "none") ||
        (s.filter && s.filter !== "none") ||
        Number(s.opacity) < 1 ||
        (s.maskImage && s.maskImage !== "none")
      ) {
        found.push(`${node.className || node.tagName}: ${s.backdropFilter}/${s.filter}/${s.opacity}`);
      }
    }
    return found;
  }, selector);
}

test("the settings panel's glass has no filtered ancestor", async ({ page }) => {
  await page.goto(url(ANY));
  await page.locator(".nd-settings > summary").click();
  // The header used to be exactly this: a `backdrop-filter` ancestor of the
  // panel, which silently turned the panel's own into a flat fill.
  expect(await filteredAncestors(page, ".nd-panel")).toEqual([]);
});

test("every glass surface has no filtered ancestor", async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto(url(SCOPED));
  await page.locator('[data-nd-action="menu"]').click();
  await page.locator(".nd-settings > summary").click();

  for (const selector of [".nd-header-glass", ".nd-nav", ".nd-panel", "[data-noctua-scope]"]) {
    expect(await filteredAncestors(page, selector), selector).toEqual([]);
  }
});

test("a frosted surface sees what is behind it", async ({ page }) => {
  await page.goto(url(ANY));
  // A hard-edged pattern behind the page. Through real glass it survives as a
  // blurred gradient with measurable variance; through a flat fill it does not
  // survive at all.
  await page.evaluate(() => {
    document.documentElement.setAttribute("data-noctua-glass", "full");
    const stripes = document.createElement("div");
    stripes.id = "probe";
    stripes.style.cssText =
      "position:fixed;inset:0;z-index:0;background:repeating-linear-gradient(90deg,#000 0 12px,#fff 12px 24px)"; // allow-literal: a test probe deliberately outside the system — maximum-contrast stripes at a fixed pitch are what make "did the blur sample this" measurable, and a palette color would weaken the very signal being read
    document.body.prepend(stripes);
  });
  await page.locator(".nd-settings > summary").click();

  const panel = await page.locator(".nd-settings .nd-panel").screenshot();
  const spread = await variance(page, panel);

  // A flat fill over the stripes gives variance close to zero. Anything that
  // actually samples carries the pattern through, softened.
  expect(spread, "the panel rendered a flat fill — its backdrop is not sampling").toBeGreaterThan(20);
});

// THE THIRTY-NINE-PALETTE GRID IS GONE, AND SO IS THE SPEC THAT MEASURED IT.
//
// The page that showed every palette at once was deleted. What that spec was
// really guarding — that a `data-palette` scope resolves against a stylesheet
// this page actually linked, rather than falling back to the reader's own —
// is now `settings.spec.js`'s accent row: thirteen swatches, thirteen distinct
// painted colors, on a page that fetches their stylesheets lazily. Same
// mechanism, fewer bytes, and still read from pixels.

// --- The frame's glass test ------------------------------------------------
//
// The material is only ever what is behind it, and until this control existed
// every glass example on the site sat on a flat page and read as a tint. These
// assert the three things the test has to arrange at once, and the one thing it
// must not do.

/** The frame's glass control, and its stage. */
function rig(page, index = 0) {
  const frame = page.locator("[data-nd-frame]").nth(index);
  return {
    frame,
    press: () => frame.locator(':scope > .nd-frame-bar [data-nd-action="frame-glass"]').click(),
    stage: frame.locator("[data-nd-frame-stage]").first(),
  };
}

/**
 * The frame with the most controls that can be frosted, and its rig.
 *
 * A page used to have ONE frame holding every example on it, so "the frame"
 * meant something. It has one per example now, and the two tests below are
 * about a control that frosts *everything in a stage* — which needs a stage
 * with enough in it to be worth the claim. Found rather than hardcoded: the
 * page decides which of its examples has the most, and an example added next
 * month can take the job over without an edit here.
 */
async function busiest(page) {
  const counts = await page
    .locator("[data-nd-frame]")
    .evaluateAll((frames) => frames.map((frame) => frame.querySelectorAll("[data-nd-glass]").length));
  const at = counts.indexOf(Math.max(...counts));
  return rig(page, at);
}

test("the glass test frosts every control that can be frosted, and no list says which", async ({
  page,
}) => {
  await page.goto(url(PAGE.button));
  const { press, stage } = await busiest(page);

  // A `bool` prop always writes its attribute, so `[data-nd-glass]` IS the set
  // of controls that declare the prop. A component that gains it next month is
  // picked up with no edit to the script or to this test.
  const capable = await stage.locator("[data-nd-glass]").count();
  expect(capable, "no control in the stage declares the glass prop").toBeGreaterThan(2);
  // Some examples are AUTHORED with `glass: true` — that is the prop's own
  // documentation. Counted over the PAGE now rather than over one stage: an
  // example is a frame of its own, and the frame with the most frostable
  // controls in it is not necessarily the one whose author asked for frost.
  expect(await page.locator('[data-nd-glass="true"]').count()).toBeGreaterThan(0);
  const authored = await stage.locator('[data-nd-glass="true"]').count();
  expect(authored, "nothing here is left for the control to do").toBeLessThan(capable);

  await press();
  expect(await stage.locator('[data-nd-glass="true"]').count()).toBe(capable);

  // And it is the real material, not a look-alike: the same computed filter the
  // shell's own glass has. A second recipe is a source-gate finding, and this
  // is the browser half of that rule.
  const [frosted, shell] = await Promise.all(
    [stage.locator('[data-nd-glass="true"]').first(), page.locator(".noctua-glass").first()].map(
      (locator) => locator.evaluate((el) => getComputedStyle(el).backdropFilter),
    ),
  );
  expect(frosted).not.toBe("none");
  expect(frosted, "the control's glass is a different material from the shell's").toBe(shell);

  await press();
  expect(
    await stage.locator('[data-nd-glass="true"]').count(),
    "leaving the test did not put every control back the way it was authored",
  ).toBe(authored);
});

test("the colour behind the examples does not move when they do", async ({ page }) => {
  await page.goto(url(PAGE.button));
  const { press, stage } = rig(page);
  await press();

  const behind = await stage.evaluate((el) => {
    const scroll = el.querySelector(".nd-glass-scroll");
    const field = el.querySelector(".nd-glass-field");
    const before = field.getBoundingClientRect().top;
    const scrolls = scroll.scrollHeight > scroll.clientHeight;
    scroll.scrollTop = scroll.scrollHeight;
    const box = el.getBoundingClientRect();
    const paint = field.getBoundingClientRect();
    return {
      scrolls,
      inside: scroll.contains(field),
      block: paint.top - box.top,
      inline: paint.left - box.left,
      shapes: field.querySelectorAll(".nd-glass-shape").length,
      moved: field.getBoundingClientRect().top - before,
    };
  });
  // The scrollbar, and the blank space that makes one.
  expect(behind.scrolls, "the stage does not scroll, so nothing can cross the colour").toBe(true);
  // The whole question about glass is what happens while something moves behind
  // it, so the colour belongs to the STAGE and not to the thing that scrolls.
  // `moved` alone cannot fail — a sibling of the scroller never moves whatever
  // it is — so what is asserted is the structure that makes it true.
  expect(behind.inside, "the colour is inside the scroller, so it travels with the examples").toBe(
    false,
  );
  // INSET, and by more on the block axis. Glued to the frame's edges the colour
  // had no beginning and no end, and an example crossing it never left it.
  expect(behind.inline, "the colour runs to the frame's side edges").toBeGreaterThan(0);
  expect(behind.block, "the colour runs to the frame's top edge").toBeGreaterThan(behind.inline);
  // Hard edges at several sizes: blur is only visible where there is an edge to
  // lose, and a 192px block loses the same edge at `subtle` as at `full`.
  expect(behind.shapes, "too few shapes to judge a blur against").toBeGreaterThan(16);
  expect(behind.moved, "the colour scrolled with the examples").toBe(0);
});

test("the glass test stops at the frame it was pressed in", async ({ page }) => {
  await page.goto(url(PAGE.exampleFrame));
  const outer = rig(page, 0);
  const inner = rig(page, 1);
  await outer.press();

  expect(await outer.stage.getAttribute("data-nd-glass-test")).not.toBeNull();
  expect(
    await inner.stage.getAttribute("data-nd-glass-test"),
    "the test reached into the frame nested inside",
  ).toBeNull();
  expect(
    await page.locator("body [data-nd-glass='true']:not([data-nd-frame] *)").count(),
    "the page outside the frames was frosted",
  ).toBe(0);
});

test("no page still says where a component comes from", async ({ page }) => {
  // Four file paths under every component page, and the reader who wants them
  // is reading the repository, not the site. Asserted across the locales
  // because the heading was a message and messages are per-locale.
  for (const locale of ["en", "pt-BR", "qps"]) {
    await page.goto(url(PAGE.button, locale));
    const headings = await page.evaluate(() =>
      [...document.querySelectorAll("h2, h3")].map((el) => el.textContent.trim()),
    );
    expect(
      headings.some((text) => /comes from|onde ele vem|coomees/i.test(text)),
      `${locale}: ${headings.join(" / ")}`,
    ).toBe(false);
  }
});

test("the test starts with the examples still on the screen", async ({ page }) => {
  // The blank is the point — an example has to be able to travel the whole
  // height of the colour and off both ends of it — but a blank first screen is
  // a different thing from a big blank. It was a fixed step twice: too much and
  // a short frame's examples were pushed off the bottom, too little and a frame
  // holding ONE example did not overflow and had no scrollbar at all.
  //
  // A whole stage each side, and the rig scrolls to the content when it starts.
  await page.goto(url(PAGE.exampleFrame));
  const { press } = rig(page);
  await press();
  await page
    .locator("[data-nd-frame]")
    .nth(1)
    .locator(':scope > .nd-frame-bar [data-nd-action="frame-glass"]')
    .click();

  // BOTH frames, because the one that broke was the short one.
  for (const index of [0, 1]) {
    const seen = await page
      .locator("[data-nd-frame]")
      .nth(index)
      .locator("[data-nd-frame-stage]")
      .first()
      .evaluate((el) => {
        const scroll = el.querySelector(".nd-glass-scroll");
        const first = scroll.querySelector(":scope > *");
        const box = el.getBoundingClientRect();
        return {
          lead: first.getBoundingClientRect().top - box.top,
          blank: parseFloat(getComputedStyle(scroll, "::before").blockSize),
          height: box.height,
          visible: first.getBoundingClientRect().top < box.bottom,
          overflows: scroll.scrollHeight - scroll.clientHeight,
          gutter: getComputedStyle(scroll).scrollbarGutter,
        };
      });
    expect(seen.visible, `frame ${index}: the examples start below the stage`).toBe(true);
    // A whole stage of blank each side, so any content overflows it.
    expect(seen.blank, `frame ${index}: the blank is smaller than the stage`).toBeGreaterThanOrEqual(
      seen.height - 1,
    );
    expect(seen.overflows, `frame ${index}: no scrollbar — nothing to drag`).toBeGreaterThan(0);
    // And the rig landed on the content rather than on the blank.
    expect(seen.lead, `frame ${index}: the rig opened on an empty field`).toBeLessThan(
      seen.height / 3,
    );
    expect(seen.gutter, `frame ${index}: the scrollbar is not reserved`).toContain("stable");
  }
});

test("a frame inside the rig is frosted by a layer, and still opens its own menu", async ({
  page,
}) => {
  // Three rules written for a frame sitting on a PAGE, and the rig is the one
  // place a frame is not sitting on one.
  await page.goto(url(PAGE.exampleFrame));
  const { press } = rig(page);
  await press();

  const nested = await page.evaluate(() => {
    const scroll = document.querySelector("[data-nd-frame-stage] .nd-glass-scroll");
    const frame = document.querySelectorAll("[data-nd-frame]")[1];
    const bar = frame.querySelector(":scope > .nd-frame-bar");
    const at = () => bar.getBoundingClientRect().top - frame.getBoundingClientRect().top;
    const before = at();
    scroll.scrollTop += 140;
    const drift = at() - before;
    scroll.scrollTop = 0;

    // Its own theme menu, opened with the rig running.
    const menu = bar.querySelector(".nd-frame-theme");
    menu.querySelector("summary").click();
    const panel = menu.querySelector(".nd-panel");
    const box = panel.getBoundingClientRect();
    const answer = {
      drift,
      layer: getComputedStyle(frame.querySelector(":scope > .nd-frame-glass")).backdropFilter,
      frame: getComputedStyle(frame).backdropFilter,
      bar: getComputedStyle(bar).backdropFilter,
      menuHost: panel.offsetParent ? panel.offsetParent.tagName : null,
      menuOnScreen: box.width > 0 && box.top >= 0 && box.bottom <= innerHeight + 1,
      inside: frame.querySelectorAll(':scope > [data-nd-frame-stage] [data-nd-glass="true"]').length,
    };
    menu.removeAttribute("open");
    return answer;
  });

  // Its bar is `position: sticky` at the header's height, which sticks to the
  // nearest scroll container — inside the rig that is the rig, so the bar left
  // its own frame and floated a third of the way down the field.
  expect(nested.drift, "the bar left its own frame and floated in the rig").toBe(0);

  // A COMPONENT ON THE COLOUR, NOT A HOLE IN IT — frosted rather than merely
  // opaque, because a frame being dragged across the field is the thing under
  // test. The material goes on a LAYER: `backdrop-filter` on an element makes
  // it the containing block for every `position: fixed` descendant, and the
  // frame's own theme menu is one. Frosting the frame moved that panel inside a
  // box that clips and it opened 1400px below the fold.
  expect(nested.layer, "the frame's glass layer is not frosted").not.toBe("none");
  expect(nested.frame, "the filter is on the frame, which will capture its panel").toBe("none");
  expect(nested.bar, "the filter is on the bar, which will capture its panel").toBe("none");
  expect(nested.menuHost, "the frame's menu is anchored to something inside the frame").toBeNull();
  expect(nested.menuOnScreen, "the frame's own theme menu opened off screen").toBe(true);

  // But not the controls inside it: they belong to the inner frame, which has a
  // control of its own for them, and frosting them inside an already-frosted
  // pane is glass sampling glass.
  expect(nested.inside, "the rig frosted the nested frame's own controls").toBe(0);
});

// --- What the glass does to a control, in pixels ---------------------------
//
// The first version of this tinted the PANE with the control's tone and left
// the ink alone, which is the opposite of what the prop's own contract says. It
// shipped because nothing measured a control: `readability.spec.js` has always
// been about the background pattern under prose.

test("a frosted control stays readable over the colour", async ({ page }) => {
  await page.goto(url(PAGE.button));
  const { press, stage } = await busiest(page);
  await press();

  // Every frosted control that has a label, over the field, one screenshot
  // each. The extremes of a button's own box are its glyph and its pane — the
  // same reasoning `readability.spec.js` uses about a glyph on a pattern.
  // Not the disabled ones. A refused control is dimmed to half opacity on
  // purpose, and WCAG 1.4.3 exempts an inactive component by name — measuring
  // it would force the dimming out rather than fix any contrast.
  const controls = stage.locator(
    '.nd-button[data-nd-glass="true"]:not([disabled]):not([aria-disabled="true"])',
  );
  const count = await controls.count();
  expect(count, "no frosted button to measure").toBeGreaterThan(2);

  for (let i = 0; i < count; i++) {
    const control = controls.nth(i);
    const label = (await control.textContent()).trim().slice(0, 20);
    // The element's own screenshot rather than a clip: it scrolls the control
    // into view first, and a control below the fold is exactly the one a clip
    // cannot capture.
    const { min, max } = await extremes(page, await control.screenshot());
    expect(
      contrast(min, max),
      `${label}: ink and pane at ${contrast(min, max).toFixed(2)}:1`,
    ).toBeGreaterThanOrEqual(4.5);
  }
});

test("frosting a control does not repaint it a different colour", async ({ page }) => {
  // `Cancel` is `outline/neutral` — no fill at all, a strong border, `fg` ink.
  // The rule that matched `[data-nd-variant][data-nd-tone="neutral"]` for EVERY
  // variant painted it `--nc-color-subtle`, the solid grey a neutral button is
  // FILLED with, and a white control came back grey.
  //
  // THE PANE, AT A NAMED POINT. The extremes of the box cannot answer this: the
  // lightest pixel in a frosted control is as often an antialiased edge or a
  // pale shape behind it as it is the pane. A few pixels in from the leading
  // edge, at half the height, is the pane and nothing else.
  //
  // AND WITH NOTHING BEHIND IT, which is the difference between this test and
  // the one it used to be. The claim is about the control's OWN paint — a rule
  // that filled an outline button because it matched every variant of the
  // neutral tone. What shows THROUGH a frosted pane is a different question,
  // and the rig's whole purpose is that the answer to it changes as an example
  // travels across the colour. So this used to pass on alignment: the control
  // happened to sit in the blank, and the day an example above it grew by a
  // few pixels — the call's header did it — the pane came back blue and this
  // read as a colour defect. Hiding the field leaves the frost on and the
  // colour out, so the only thing that can move the pane now is the paint.
  await page.goto(url(PAGE.exampleFrame));
  const outline = page
    .locator('[data-nd-frame] .nd-button[data-nd-variant="outline"][data-nd-tone="neutral"]')
    .first();
  await outline.scrollIntoViewIfNeeded();
  const box = await outline.boundingBox();
  const pane = async () => pixelAt(page, await outline.screenshot(), 6, box.height / 2);

  const before = await pane();
  const inner = page.locator("[data-nd-frame]").nth(1);
  await inner.locator(':scope > .nd-frame-bar [data-nd-action="frame-glass"]').click();
  await inner.locator(".nd-glass-field").evaluate((field) => {
    field.style.visibility = "hidden";
  });
  await outline.scrollIntoViewIfNeeded();
  const after = await pane();

  const drift = before.reduce((sum, channel, i) => sum + Math.abs(channel - after[i]), 0);
  expect(
    drift,
    `the pane went from ${before} to ${after}: a frosted white control must stay white`,
  ).toBeLessThan(60);
  // And the label still reads against that pane.
  const { min } = await extremes(page, await outline.screenshot());
  expect(contrast(min, after), "the label stopped reading against its own pane").toBeGreaterThan(
    4.5,
  );
});

test("a frosted primary button is still a primary button", async ({ page }) => {
  // Frosting everything the same made `Save` and `Cancel` the same object: a
  // primary button that loses its colour has lost the one thing it was for.
  //
  // The first tint tried was the tone's `-bg` step, which measured 0.975
  // lightness at ZERO chroma in this palette — a tint nobody can see, and this
  // assertion is what says so. The pane is mixed from the tone's solid step
  // against the surface instead.
  await page.goto(url(PAGE.exampleFrame));
  const inner = page.locator("[data-nd-frame]").nth(1);
  await inner.locator(':scope > .nd-frame-bar [data-nd-action="frame-glass"]').click();

  const paneOf = async (selector) => {
    const control = page.locator(`[data-nd-frame-stage] .nd-example .nd-button${selector}`).first();
    await control.scrollIntoViewIfNeeded();
    const box = await control.boundingBox();
    const shot = await control.screenshot();
    return {
      pane: await pixelAt(page, shot, 6, Math.round(box.height / 2)),
      ink: (await extremes(page, shot)).min,
    };
  };

  const filled = await paneOf('[data-nd-variant="solid"][data-nd-tone="accent"]');
  const plain = await paneOf('[data-nd-variant="outline"][data-nd-tone="neutral"]');

  const distance = filled.pane.reduce((sum, channel, i) => sum + Math.abs(channel - plain.pane[i]), 0);
  expect(
    distance,
    `a frosted primary at ${filled.pane} against a frosted secondary at ${plain.pane}`,
  ).toBeGreaterThan(40);
  // And it did not buy that colour with legibility, which is the trade the
  // first version of this made.
  expect(contrast(filled.ink, filled.pane), "the primary stopped reading").toBeGreaterThanOrEqual(
    4.5,
  );
});

// --- The rig owns the material it tests ------------------------------------

test("the rig turns the glass on, refuses Off, and puts it back", async ({ page }) => {
  // A control that says "look at the glass" over a stage whose glass is `off`
  // is a control that does nothing, and the reader could turn it off underneath
  // it at any moment.
  await page.goto(url(PAGE.button));
  const frame = page.locator("[data-nd-frame]").first();

  // What the material actually resolves to, rather than what attribute happens
  // to be stamped: a stage that agrees with the page carries nothing, and that
  // is `subtle`, not `off`.
  const state = () =>
    frame.evaluate((el) => {
      const stage = el.querySelector("[data-nd-frame-stage]");
      const off = el.querySelector('[data-nd-group="glass"] [data-nd-value="off"]');
      return {
        opacity: getComputedStyle(stage).getPropertyValue("--noctua-glass-opacity").trim(),
        refused: off.getAttribute("aria-disabled"),
        chosen: off.getAttribute("aria-checked"),
        note: getComputedStyle(el.querySelector(".nd-glass-held")).display,
      };
    });

  await frame.evaluate((el) => {
    el.querySelector(".nd-frame-theme > summary").click();
    el.querySelector('[data-nd-group="glass"] [data-nd-value="off"]').click();
  });
  const chosenOff = await state();
  expect(chosenOff.chosen, "the reader's own choice did not take").toBe("true");
  expect(chosenOff.note, "the note shows with no rig running").toBe("none");

  await frame.evaluate((el) => el.querySelector('[data-nd-action="frame-glass"]').click());
  const running = await state();
  expect(running.opacity, "the rig ran with no material to look at").not.toBe(chosenOff.opacity);
  expect(running.refused, "`Off` is still offered while the rig needs the material").toBe("true");
  expect(running.note, "nothing says why `Off` is refused").toBe("block");

  // Refused, not removed: `SegmentedItem.disabled` exists so a reader "can find
  // out it exists and is refused instead of meeting a gap where an option used
  // to be" — so it is still there, still reachable, and pressing it does
  // nothing.
  await frame.evaluate((el) =>
    el.querySelector('[data-nd-group="glass"] [data-nd-value="off"]').click(),
  );
  expect(await state()).toEqual(running);

  await frame.evaluate((el) => el.querySelector('[data-nd-action="frame-glass"]').click());
  const after = await state();
  expect(after, "leaving the rig did not give the reader their `off` back").toEqual(chosenOff);
});

test("the note is a frame's own, and never the page's", async ({ page }) => {
  await page.goto(url(PAGE.exampleFrame));
  // The header's panel has no rig behind it, so it must not carry the sentence.
  const header = await openTheme(page);
  expect(await header.locator(".nd-glass-held").count(), "the page's panel explains a rig").toBe(0);
  await closeTheme(page);

  const { press } = rig(page);
  await press();
  const shown = await page.evaluate(() =>
    [...document.querySelectorAll("[data-nd-frame]")].map(
      (frame) => getComputedStyle(frame.querySelector(".nd-glass-held")).display,
    ),
  );
  // The outer frame is testing; the one nested inside it is not, and an
  // explanation for somebody else's rig is worse than none.
  expect(shown).toEqual(["block", "none"]);
});

test("the nested frame keeps its header under the rig", async ({ page }) => {
  // `.nd-frame-glass` is `position: absolute` and the bar inside a rig was
  // `static`, so the frost painted over the bar and the nested frame lost its
  // header entirely. Then `relative` alone shoved it 65px down onto its own
  // stage, because `inset-block-start: var(--nd-header-h)` — meant for the
  // sticky case — applies to `relative` as an offset.
  //
  // IN PIXELS, because the layer is `pointer-events: none` and a hit test walks
  // straight through it: with the bar covered the box is a smooth frosted band,
  // and with the bar there it has text in it.
  await page.goto(url(PAGE.exampleFrame));
  const { press } = rig(page);
  await press();

  const frame = page.locator("[data-nd-frame]").nth(1);
  await frame.scrollIntoViewIfNeeded();
  const bar = frame.locator(":scope > .nd-frame-bar");
  const { min, max } = await extremes(page, await bar.screenshot());
  expect(
    contrast(min, max),
    "the nested frame's bar is a blank band — the frost is painting over it",
  ).toBeGreaterThan(4);

  // And it sits BESIDE its own stage rather than over it. The strip was a bar
  // across the top and is a rail down the trailing edge, so the claim moves
  // axis with it: the stage ends where the rail begins, and neither overlaps.
  const [barBox, stageBox] = await Promise.all([
    bar.boundingBox(),
    frame.locator(":scope > [data-nd-frame-stage]").boundingBox(),
  ]);
  expect(
    Math.round(barBox.x - (stageBox.x + stageBox.width)),
    "the rail overlaps its stage",
  ).toBeGreaterThanOrEqual(0);
});

test("the page frosts the frames on it, and stops when told to", async ({ page }) => {
  await page.goto(url(PAGE.button));
  const layer = page.locator(".nd-frame-glass").first();
  // `subtle` is the default, so a frame is a frosted pane out of the box —
  // what the outer rig does to a nested frame, the page does to this one.
  expect(await layer.evaluate((el) => getComputedStyle(el).backdropFilter)).not.toBe("none");

  const panel = await openTheme(page);
  await panel.locator('[data-nd-group="glass"] [data-nd-value="off"]').click();
  expect(
    await layer.evaluate((el) => getComputedStyle(el).backdropFilter),
    "the page's glass is off and its frames are still frosted",
  ).toBe("none");
});

test("the note names the value it is refusing", async ({ page }) => {
  await page.goto(url(PAGE.button));
  const held = page.locator(".nd-glass-held").first();
  // A `Message`, so the value is a code chip rather than a word with backticks
  // around it — which is what plain interpolation rendered.
  await expect(held.locator("code")).toHaveCount(1);
  expect(await held.textContent()).not.toContain("`");
});

test("a frame has a surface of its own, and it is the page's", async ({ page }) => {
  // The frame had no background: it was a hole with a border, and what showed
  // inside it was whatever the page happened to be — a pattern ran straight
  // through the examples. Its surface is on the LAYER rather than on the frame,
  // because a `backdrop-filter` samples what is painted behind the element that
  // carries it, and a fill on the frame is behind the layer.
  await page.goto(url(PAGE.button));
  const panel = await openTheme(page);
  await panel.locator('[data-nd-group="glass"] [data-nd-value="off"]').click();
  await closeTheme(page);

  const [frame, root] = await Promise.all(
    [page.locator(".nd-frame-glass").first(), page.locator("html")].map((locator) =>
      locator.evaluate((el) => {
        const canvas = document.createElement("canvas");
        canvas.width = canvas.height = 1;
        const ctx = canvas.getContext("2d");
        ctx.fillStyle = getComputedStyle(el).backgroundColor; // allow-literal: read back through the canvas below, the recipe the gate prescribes
        ctx.fillRect(0, 0, 1, 1);
        return [...ctx.getImageData(0, 0, 1, 1).data];
      }),
    ),
  );
  expect(frame[3], "the frame is transparent with the glass off").toBe(255);
  expect(frame.slice(0, 3), "the frame's surface is not the page's").toEqual(root.slice(0, 3));
});
