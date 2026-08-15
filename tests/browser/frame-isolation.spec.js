// A FRAME OWNS WHAT IS INSIDE IT.
//
// This file exists because the rule it asserts has been got wrong more than
// once, in more than one way, and each time the fix was patched into the script
// while the doctrine and two tests put it straight back. So the rule is written
// here, in its own file, named after itself:
//
//   A setting a frame STATES owns its whole subtree — including a nested frame,
//   its bar, its border and its own menu panel. A setting nobody stated comes
//   from OUTSIDE, which on an ordinary page is the page and inside another frame
//   is that frame. Reset CLEARS the request; it does not state the page.
//
// The reported symptom, every time, is the same: page on light with glass;
// outer frame set to dark with glass off; the inner frame stays light with a
// frosted header. That is test A.
//
// TWO TRAPS THIS SPEC HAS ALREADY FALLEN INTO, both recorded so the next
// version does not repeat them:
//
// 1. **Never `stage.getAttribute(…)`.** A frame that has stated nothing carries
//    NO attributes at all — carrying nothing is exactly how a subtree inherits
//    everything. An earlier version asserted "the inner stage has no scope
//    attributes" and passed with the bug present and with it absent.
// 2. **Ask the pixels, or ask what is in force.** In force means the nearest
//    ancestor carrying the attribute, falling back to the model's default.

import { readFileSync } from "node:fs";
import { test, expect } from "@playwright/test";
import { PAGE, closeTheme, openTheme, url } from "./pages.js";

/** The generated model — `dist/theme.json` exists for readers with no toolchain. */
const THEME = JSON.parse(readFileSync(new URL("../../dist/theme.json", import.meta.url), "utf8"));

/** Every page on the site that renders at least one frame. */
const FRAMED = [PAGE.exampleFrame, PAGE.button, PAGE.themeScope, PAGE.component];

/** One frame's own parts, never those of a frame nested inside it. */
function frameParts(page, index) {
  const frame = page.locator("[data-nd-frame]").nth(index);
  return {
    frame,
    stage: frame.locator("[data-nd-frame-stage]").first(),
    // `:scope >` all the way down: a descendant selector inside the outer frame
    // also matches the inner frame's copy of everything, which is the trap the
    // whole nested case is about.
    own: (selector) => frame.locator(`:scope > .nd-frame-bar ${selector}`),
  };
}

/** A colour string as four bytes, through a 1×1 canvas. */
const paint = (page, value) =>
  page.evaluate((v) => {
    const canvas = document.createElement("canvas");
    canvas.width = canvas.height = 1;
    const ctx = canvas.getContext("2d");
    ctx.fillStyle = v;
    ctx.fillRect(0, 0, 1, 1);
    return [...ctx.getImageData(0, 0, 1, 1).data].join(",");
  }, value);

/** What surface an element resolves — the honest question about a theme. */
async function surface(page, locator) {
  const raw = await locator.evaluate(
    (el) => getComputedStyle(el).getPropertyValue("--nc-color-surface"), // allow-literal: fed straight into the canvas, which is the recipe the gate prescribes
  );
  return paint(page, raw.trim());
}

/** The value the cascade would use at this element. */
const inForce = async (locator, name, attribute) =>
  (await locator.evaluate((el, a) => el.closest(`[${a}]`)?.getAttribute(a) ?? null, attribute)) ??
  THEME.settings[name].default;

/** Sets the page's own appearance, explicitly, through the header menu. */
async function setPage(page, values) {
  const panel = await openTheme(page);
  for (const [group, value] of values) {
    await panel.locator(`[data-nd-group="${group}"] [data-nd-value="${value}"]`).click();
  }
  await closeTheme(page);
}

/** Sets one frame's appearance through its own menu. */
async function setFrame(parts, values) {
  await parts.own(".nd-frame-theme > summary").click();
  for (const [group, value] of values) {
    // TWO CONTROL SHAPES, AND A SWITCH IS NOT A ROW OF VALUES. `grain` is a
    // `Toggle`: one `role="switch"` carrying both values, which is why a helper
    // that only knew `[data-nd-value]` waited thirty seconds for a node that was
    // never going to exist.
    const flip = parts.own(`[role="switch"][data-nd-group="${group}"]`);
    if ((await flip.count()) > 0) {
      const on = (await flip.getAttribute("aria-checked")) === "true";
      if (on !== (value === (await flip.getAttribute("data-nd-on")))) await flip.click();
      continue;
    }
    await parts.own(`[data-nd-group="${group}"] [data-nd-value="${value}"]`).click();
  }
  await parts.own(".nd-frame-theme > summary").click();
}

// --- A. The report, exactly -------------------------------------------------

test("a frame's theme reaches the frame inside it", async ({ page }) => {
  await page.goto(url(PAGE.exampleFrame));
  const outer = frameParts(page, 0);
  const inner = frameParts(page, 1);
  expect(
    await outer.frame.locator("[data-nd-frame]").count(),
    "this spec needs the page where one frame sits inside another",
  ).toBeGreaterThan(0);

  // The page states its own appearance, so that "the frame differs from the
  // page" is a plain comparison rather than one against an implicit `system`.
  await setPage(page, [
    ["mode", "light"],
    ["glass", "subtle"],
  ]);
  await setFrame(outer, [
    ["mode", "dark"],
    ["glass", "off"],
  ]);

  const [innerSurface, outerSurface, pageSurface] = await Promise.all([
    surface(page, inner.stage),
    surface(page, outer.stage),
    surface(page, page.locator("html")),
  ]);
  expect(outerSurface, "the outer frame did not actually change").not.toBe(pageSurface);
  expect(innerSurface, "the page reached past the frame around it").toBe(outerSurface);

  // AND THE FROST, which is the other half of what the reader reported. The
  // layer paints the whole interior of a frame that has stated nothing, so a
  // dark frame with a page-coloured layer is dark text on white — worse than
  // the bug it replaced.
  const frost = inner.frame.locator(".nd-frame-glass");
  expect(await frost.getAttribute("data-nd-glass"), "the inner header is still frosted").toBeNull();
  expect(
    await frost.evaluate((el) => getComputedStyle(el).backdropFilter),
    "the inner header still samples what is behind it",
  ).toMatch(/^(none|blur\(0px\).*)$/);
  expect(
    await paint(page, await frost.evaluate((el) => getComputedStyle(el).backgroundColor)), // allow-literal: painted into a canvas, which is the recipe the gate prescribes
    "the inner frame is dark ink on the page's white",
  ).toBe(outerSurface);
});

// --- B. Reset means stop overriding -----------------------------------------

test("reset returns a frame to following, not to the page", async ({ page }) => {
  await page.goto(url(PAGE.exampleFrame));
  const outer = frameParts(page, 0);
  const inner = frameParts(page, 1);

  await setPage(page, [["mode", "light"]]);
  await setFrame(outer, [["mode", "dark"]]);
  await setFrame(inner, [["mode", "light"]]);

  const outerSurface = await surface(page, outer.stage);
  expect(await surface(page, inner.stage), "the inner frame did not take its own light").not.toBe(
    outerSurface,
  );

  await inner.own('[data-nd-action="frame-reset"]').click();

  // RESET IS NOT "STATE THE PAGE". Reset must equal reload: on a fresh load the
  // inner frame inside a dark outer frame IS dark, so a reset that produced
  // light would leave a state no reload could reach. It also re-introduces the
  // seal by another name — a concrete page request on an element that stated
  // nothing.
  expect(
    await surface(page, inner.stage),
    "reset stated the page instead of clearing the request",
  ).toBe(outerSurface);
});

// --- E. The generic one -----------------------------------------------------

test("a frame that has stated nothing stamps nothing", async ({ page }) => {
  // THE TEST THAT MAKES A SEAL FAIL EVERYWHERE AT ONCE. It cannot be satisfied
  // by a special case for one page, and any future "default the frame to the
  // page" turns it red on four pages in one run.
  //
  // It is also the guard for the opposite fear, the one the seal's author had:
  // `.nd-frame-stage[data-noctua-scope]` paints an opaque surface, so a frame
  // that stamped for no reason would grow a panel and hide the page's pattern.
  for (const slug of FRAMED) {
    await page.goto(url(slug));
    await expect(
      page.locator("[data-nd-frame][data-noctua-scope], [data-nd-frame-stage][data-noctua-scope]"),
      `${slug}: a frame stamped a scope nobody asked for`,
    ).toHaveCount(0);

    // Through the canvas, and only the ALPHA byte — so this names no colour and
    // asks the one question it means: did anything get painted at all.
    const painted = await page.locator("[data-nd-frame-stage]").evaluateAll((stages) => {
      const canvas = document.createElement("canvas");
      canvas.width = canvas.height = 1;
      const ctx = canvas.getContext("2d");
      return stages.filter((el) => {
        ctx.clearRect(0, 0, 1, 1);
        ctx.fillStyle = getComputedStyle(el).backgroundColor; // allow-literal: painted into the canvas on the next line and read as an alpha byte, which is the recipe the gate prescribes
        ctx.fillRect(0, 0, 1, 1);
        return ctx.getImageData(0, 0, 1, 1).data[3] !== 0;
      }).length;
    });
    expect(painted, `${slug}: a stage grew a surface of its own`).toBe(0);
  }
});

// --- G. Order must not matter -----------------------------------------------

test("theming the inner frame first does not cost it the outer frame's theme", async ({ page }) => {
  await page.goto(url(PAGE.exampleFrame));
  const outer = frameParts(page, 0);
  const inner = frameParts(page, 1);

  await setPage(page, [["mode", "light"]]);
  // THE INNER ONE FIRST. `inheritedAt` finds its parent by the STAMPED
  // attribute, so which ancestors happen to be stamped in the current pass
  // decides what a scope inherits. Resolve the inner before the outer has ever
  // been stamped and it inherits the page — and then stamps the page's values
  // absolutely, which no later pass can undo.
  await setFrame(inner, [["radius", "square"]]);
  await setFrame(outer, [
    ["mode", "dark"],
    ["font", "mono"],
  ]);

  expect(await inForce(inner.stage, "radius", "data-noctua-radius")).toBe("square");
  expect(
    await inForce(inner.stage, "mode", "data-theme"),
    "the inner frame lost the outer frame's mode because it was themed first",
  ).toBe("dark");
  expect(await inForce(inner.stage, "font", "data-noctua-font")).toBe("mono");
});

// --- C. A reset stops at the frame that owns it -----------------------------

test("a frame's reset does not reach into the frame inside it", async ({ page }) => {
  await page.goto(url(PAGE.exampleFrame));
  const outer = frameParts(page, 0);
  const inner = frameParts(page, 1);

  await setPage(page, [["mode", "light"]]);
  await setFrame(outer, [["mode", "dark"]]);
  await setFrame(inner, [["font", "mono"]]);

  // THE OUTER ONE'S RESET, WHICH IS THE OUTER ONE'S. It clears what the outer
  // frame stated; the inner frame stated `mono` for itself and nobody asked
  // for that to go. The handler used to select every `[data-noctua-scope]`
  // under the frame, which is the inner stage as well.
  await outer.own('[data-nd-action="frame-reset"]').click();

  expect(
    await inForce(inner.stage, "font", "data-noctua-font"),
    "the outer frame's reset cleared what the inner frame had stated",
  ).toBe("mono");
  // And the outer one did clear, so this is not passing on a reset that did
  // nothing at all.
  expect(await inForce(outer.stage, "mode", "data-theme")).toBe("light");
});

// --- D. `system` means the machine, at any depth ----------------------------

test("system in a nested frame means the machine, not the frame around it", async ({ page }) => {
  await page.goto(url(PAGE.exampleFrame));
  const outer = frameParts(page, 0);
  const inner = frameParts(page, 1);

  await setPage(page, [["mode", "light"]]);
  await setFrame(outer, [["mode", "dark"]]);
  // A VALUE SPELLED AS THE ABSENCE OF ITS ATTRIBUTE, which also happens to be
  // how a subtree says "whatever is above me". The same spelling for two
  // different facts, and this is the case that tells them apart: a reader who
  // picks `system` inside a dark frame asked for the MACHINE's answer, and
  // this machine is on light.
  await setFrame(inner, [["mode", "system"]]);

  expect(
    await inner.stage.evaluate((el) => el.closest("[data-theme]")?.getAttribute("data-theme")),
    "system inside a dark frame resolved to the frame rather than to the machine",
  ).toBe("light");
  expect(await surface(page, inner.stage)).toBe(await surface(page, page.locator("html")));
});

// --- F. One axis at a time --------------------------------------------------

test("an accent chosen in a nested frame keeps the enclosing saturation", async ({ page }) => {
  await page.goto(url(PAGE.exampleFrame));
  const outer = frameParts(page, 0);
  const inner = frameParts(page, 1);

  await setFrame(outer, [["saturation", "vivid"]]);
  // `accent` is not an attribute of its own — the PALETTE is — so choosing one
  // means moving along the accent axis while HOLDING the saturation already in
  // force. Read from the page while every request was absolute, this took the
  // page's `balanced` and moved two axes for one click.
  await setFrame(inner, [["accent", "teal"]]);

  expect(await inForce(inner.stage, "palette", "data-palette")).toBe("teal-vivid");
});

// --- H. A control reports what is in force -----------------------------------

test("a nested frame's controls report what is in force, not what the page is", async ({ page }) => {
  await page.goto(url(PAGE.exampleFrame));
  const outer = frameParts(page, 0);
  const inner = frameParts(page, 1);

  await setPage(page, [
    ["mode", "light"],
    ["radius", "round"],
  ]);
  await setFrame(outer, [
    ["mode", "dark"],
    ["radius", "square"],
  ]);

  // The inner frame has stated nothing, so its own menu has nothing of its own
  // to show — and what it must show is what the cascade would use HERE. A panel
  // that reported the page's values would tell a reader the frame is on light
  // while they are looking at a dark one.
  await inner.own(".nd-frame-theme > summary").click();
  for (const [group, value] of [
    ["mode", "dark"],
    ["radius", "square"],
  ]) {
    await expect(
      inner.own(`[data-nd-group="${group}"] [data-nd-value="${value}"]`),
      `the inner frame's ${group} row reports the page instead of the frame around it`,
    ).toHaveAttribute("aria-checked", "true");
  }
});

// --- I. The ambient settings reach in too ------------------------------------

test("a frame's backdrop and grain reach the frame inside it", async ({ page }) => {
  await page.goto(url(PAGE.exampleFrame));
  const outer = frameParts(page, 0);
  const inner = frameParts(page, 1);

  // THE FOUR THAT WERE PAGE-ONLY. Each used to paint from a descendant the
  // docs site rendered once, at page level, so a frame could neither have its
  // own nor cancel the page's. They declare a custom property on the element
  // carrying the attribute now, which is what makes this assertable at all.
  await setPage(page, [["pattern", "checks"]]);
  await setFrame(outer, [
    ["backdrop", "cool"],
    ["grain", "on"],
    ["pattern", "none"],
  ]);

  const painted = (locator) =>
    locator.evaluate((el) => {
      const style = getComputedStyle(el);
      // The alpha byte only — so this names no colour and asks the one thing
      // it means: is there a surface here at all. A scope that re-resolves the
      // contract and paints nothing is a light island on a dark page, which is
      // the defect `.nd-scope` and `.nd-frame-stage` each learned separately.
      const canvas = document.createElement("canvas");
      canvas.width = canvas.height = 1;
      const ctx = canvas.getContext("2d");
      ctx.fillStyle = style.backgroundColor; // allow-literal: painted into the canvas on the next line and read as an alpha byte, which is the recipe the gate prescribes
      ctx.fillRect(0, 0, 1, 1);
      return {
        opaque: ctx.getImageData(0, 0, 1, 1).data[3] === 255,
        image: style.backgroundImage,
        // The layers, in the order the paint rule lists them: grain, then
        // pattern. Counting them is how "the pattern was cancelled" is asked
        // without naming a colour.
        layers: style.backgroundImage.split(/,(?![^(]*\))/).length,
      };
    });

  const [outerPaint, innerPaint, pagePaint] = await Promise.all([
    painted(outer.stage),
    painted(inner.stage),
    painted(page.locator("html")),
  ]);
  expect(pagePaint.image, "the page did not take the pattern").toContain("conic-gradient");
  // The grain is a data URI rather than a gradient, which is also what makes
  // the last assertion below able to tell the two layers apart.
  expect(outerPaint.image, "the frame did not take the grain").toContain("url(");
  expect(outerPaint.opaque, "a frame that stated a backdrop painted no surface").toBe(true);
  // The inner frame stated nothing, so it carries no attributes — the grain
  // reaches it by INHERITANCE of the declared property, which is the whole
  // mechanism. Its surface is the outer's, and so is what is painted on it.
  expect(
    await surface(page, inner.stage),
    "the page reached past the frame around it",
  ).toBe(await surface(page, outer.stage));
  expect(innerPaint.image, "the page's pattern showed through a frame that cancelled it").not.toContain(
    "conic-gradient",
  );
});
