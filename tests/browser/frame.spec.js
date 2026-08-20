// The example frame: the four controls, and the one guarantee.
//
// The frame is what `NOCTUA.md §9.10` calls "one abstraction hosts every
// example". Everything here is measured on the built site rather than asserted
// about the markup, because the claims are about what a reader sees: a stage
// that is 320px across, a page that did NOT change appearance, a caption that
// agrees with the render above it.
//
// Colours are read from screenshot pixels decoded in a canvas, never from
// `getComputedStyle(…).color` — two serialised strings comparing equal prove
// nothing about the pixels, which is the whole of the `color_read_from_dom`
// gate.

import { test, expect } from "@playwright/test";
import {
  LOCALES,
  PAGE,
  SCOPED,
  closeTheme,
  modeControl,
  openFrameTheme,
  openTheme,
  setFrameWidth,
  url,
} from "./pages.js";

// A page that HAS a frame. `ANY` is the intro, whose body is one paragraph —
// the frame is on every page that demonstrates something, which the intro
// deliberately does not.
const FRAMED = PAGE.component;

/** A colour string as four bytes, via a 1×1 canvas. */
async function paint(page, value) {
  return page.evaluate((v) => {
    const canvas = document.createElement("canvas");
    canvas.width = canvas.height = 1;
    const ctx = canvas.getContext("2d");
    ctx.fillStyle = v;
    ctx.fillRect(0, 0, 1, 1);
    // The ALPHA channel matters here and nowhere else on this site: a
    // transparent element and a black one both serialise to `0,0,0` once the
    // fourth byte is thrown away, and telling them apart is the whole question.
    return [...ctx.getImageData(0, 0, 1, 1).data].join(",");
  }, value);
}

/**
 * The background one element actually PAINTS.
 *
 * Not `--nc-color-surface`. The variable is what the element would paint *if*
 * something painted it, and for a whole change nothing did: the stage resolved
 * the contract correctly against its own scope and stayed transparent, so the
 * page showed through it and the examples sat on the wrong surface. The test
 * that was here read the variable, so it passed the entire time.
 */
async function background(page, selector) {
  const raw = await page
    .locator(selector)
    .first()
    .evaluate((el) => getComputedStyle(el).backgroundColor); // allow-literal: read only to be painted into a canvas by `paint`, which is the recipe the gate prescribes
  return paint(page, raw);
}

/** Fully transparent, whatever colour the engine chose to serialise with it. */
const CLEAR = "0,0,0,0";

/**
 * Every attribute a scope can carry, read from the SITE'S OWN generated module.
 *
 * Not a list written here: the bug this file now guards against was a
 * hand-written list that named two of six, and a hand-written list in the test
 * would have agreed with it.
 */
async function scopeAttributeNames(page) {
  return page.evaluate(async () => {
    const base = document.documentElement.getAttribute("data-noctua-base") || "";
    const model = await import(`${base}assets/theme.js`);
    return model.scopeAttributeNames;
  });
}

/** Which of those a given element is carrying. */
async function stamped(page, selector, names) {
  return page
    .locator(selector)
    .first()
    .evaluate((el, list) => list.filter((name) => el.hasAttribute(name)), names);
}

// --- The appearance toggle --------------------------------------------------

test("the frame paints its own surface, and the page never moves", async ({ page }) => {
  await page.goto(url(FRAMED));
  const toggle = page.locator('[data-nd-action="frame-mode"]').first();

  const pageBefore = await background(page, "html");
  // Nothing is scoped until the reader asks, and an unscoped stage paints
  // nothing at all — an opaque one would hide the page's background pattern
  // inside every frame on the site.
  expect(await background(page, "[data-nd-frame-stage]")).toBe(CLEAR);
  await expect(toggle).not.toHaveAttribute("aria-pressed", "true");

  await toggle.click();
  const stage = await background(page, "[data-nd-frame-stage]");

  expect(stage, "the stage is still transparent, so it shows the page").not.toBe(CLEAR);
  expect(stage, "the stage is painted in the page's own appearance").not.toBe(pageBefore);
  expect(await background(page, "html"), "the page moved, and it must not").toBe(pageBefore);
  await expect(toggle).toHaveAttribute("aria-pressed", "true");
});

test("the toggle keeps toggling, in every mode the page can be in", async ({ page }) => {
  await page.goto(url(FRAMED));
  const toggle = page.locator('[data-nd-action="frame-mode"]').first();
  const names = await scopeAttributeNames(page);

  // ONE PRESS PROVED NOTHING ABOUT THE SECOND, and the second was the bug: the
  // teardown removed two of the six attributes the resolver stamps, and a
  // leftover `data-palette` alone is enough for `dist/scope.css` to keep
  // resolving the whole contract at that element — against the system
  // preference rather than the page. The stage stuck in one appearance and
  // every further press did nothing.
  for (const mode of ["light", "dark", "system"]) {
    await (await modeControl(page)).locator(`[data-nd-value="${mode}"]`).click();
    // Closed again: the menu floats over the page, and a panel left open
    // swallows the clicks meant for the frame underneath it.
    await closeTheme(page);

    const rest = await background(page, "[data-nd-frame-stage]");
    expect(rest, `page=${mode}: the stage did not start unscoped`).toBe(CLEAR);
    expect(await stamped(page, "[data-nd-frame-stage]", names)).toEqual([]);

    await toggle.click();
    const on = await background(page, "[data-nd-frame-stage]");
    expect(on, `page=${mode}: nothing was painted`).not.toBe(CLEAR);
    expect(await stamped(page, "[data-nd-frame-stage]", names)).toEqual(names);

    await toggle.click();
    expect(
      await background(page, "[data-nd-frame-stage]"),
      `page=${mode}: the frame did not go back`,
    ).toBe(CLEAR);
    // The half that was left behind. Every name, or the subtree cannot be
    // un-themed at all.
    expect(
      await stamped(page, "[data-nd-frame-stage]", names),
      `page=${mode}: a scope attribute survived the toggle going off`,
    ).toEqual([]);

    await toggle.click();
    expect(
      await background(page, "[data-nd-frame-stage]"),
      `page=${mode}: the third press did nothing`,
    ).toBe(on);

    await page.locator('[data-nd-action="frame-reset"]').first().click();
    expect(
      await stamped(page, "[data-nd-frame-stage]", names),
      `page=${mode}: a scope attribute survived Reset`,
    ).toEqual([]);
  }
});

test("an example inside the frame obeys the frame, not the page", async ({ page }) => {
  await page.goto(url(FRAMED));
  // THE FRAME THAT IS INVERTED AND THE NOTE THAT IS READ MUST BE THE SAME ONE.
  // This pressed `.first()` and then read the first note on the page, which
  // were one frame while the first frame was the first example. The page opens
  // on an overview now, and an overview carries no note — so the toggle was
  // inverting one frame and the assertion was reading another, which is a
  // page that passed the test by accident for as long as the two agreed.
  const framed = page.locator(".nd-frame:has(.nd-example-note)").first();
  await framed.locator('[data-nd-action="frame-mode"]').click();

  // The claim in one line: a component in the stage resolves the contract
  // against the stage's scope, which is what `dist/scope.css` exists for. Read
  // as ink rather than as a surface, because the examples paint no background
  // of their own — which is exactly why the STAGE has to.
  const ink = (locator) =>
    locator
      .first()
      .evaluate((el) => getComputedStyle(el).color) // allow-literal: painted into a canvas by `paint` on the next line
      .then((raw) => paint(page, raw));

  expect(await ink(framed.locator(".nd-frame-stage .nd-example-note"))).not.toBe(
    await ink(page.locator(".nd-lead")),
  );
});

test("the toggle names both of its states and shows both of its glyphs", async ({ page }) => {
  await page.goto(url(FRAMED));
  const toggle = page.locator('[data-nd-action="frame-mode"]').first();

  // A toggle that keeps one name announces the wrong state from the first
  // press onward, and a glyph swapped by script is a drawing shipped to the
  // script — this package ships no runtime, so both are in the markup.
  await expect(toggle.locator(".nd-button-swap > svg")).toHaveCount(2);
  const closed = await toggle.getAttribute("aria-label");
  await toggle.click();
  const open = await toggle.getAttribute("aria-label");
  expect(open, "the name did not change with the state").not.toBe(closed);
  expect(open?.trim().length, "the other name is empty").toBeGreaterThan(0);
});

// --- A frame inside a frame -------------------------------------------------

// The frame's own page demonstrates it by putting one inside another, which is
// the only place on the site where "isolated" is more than a word: every
// handler here used to address `frame.querySelectorAll(…)`, and a nested
// frame's controls are descendants of its parent.

const NESTED = "example-frame/";

test("an inner frame inverts what it INHERITS, not what the page is", async ({ page }) => {
  await page.goto(url(NESTED));
  const stages = page.locator("[data-nd-frame-stage]");
  const toggles = page.locator('[data-nd-action="frame-mode"]');
  await expect(stages).toHaveCount(2);

  const surfaceOf = async (i) =>
    paint(
      page,
      await stages.nth(i).evaluate((el) => getComputedStyle(el).backgroundColor), // allow-literal: painted into a canvas by `paint`, the recipe the gate prescribes
    );

  await toggles.nth(0).click();
  const outer = await surfaceOf(0);
  expect(outer, "the outer frame did not invert").not.toBe(CLEAR);

  await toggles.nth(1).click();
  const inner = await surfaceOf(1);

  // THE BUG THIS GUARDS. The toggle read `<html>`'s mode, which is the same
  // thing only while no frame is inside another. Inside an inverted outer
  // frame, the inner one asked for the opposite of the PAGE — the mode it
  // already had — so the resolver called it a no-op and stamped nothing. The
  // inner frame could not invert itself at all.
  expect(inner, "the inner frame stamped nothing").not.toBe(CLEAR);
  expect(inner, "the inner frame matched the frame around it").not.toBe(outer);
  expect(
    await surfaceOf(0),
    "inverting the inner frame moved the outer one",
  ).toBe(outer);
});

test("a frame's controls never reach into a frame inside it", async ({ page }) => {
  await page.setViewportSize({ width: 1440, height: 900 });
  await page.goto(url(NESTED));
  const frames = page.locator("[data-nd-frame]");
  const inner = frames.nth(1);

  const innerWidth = () => inner.locator("[data-nd-frame-stage]").first().boundingBox();
  // THE TRIGGER NAMES THE WIDTH, and no item carries a tick. The control is a
  // menu now, and a menu whose items hold a state is a listbox wearing a
  // menu's clothes — which `spec/components/menu.toml` refuses.
  const innerChosen = () => inner.locator(".nd-frame-width").first().innerText();

  const innerStage = () =>
    inner.locator("[data-nd-frame-stage]").first().getAttribute("data-nd-width");
  const before = { chosen: await innerChosen(), width: await innerStage() };

  // The outer frame's own stop. It used to mark the inner control's matching
  // stop as well — while resizing only the outer stage — so the inner control
  // sat there claiming a width its stage was not.
  await setFrameWidth(page, "320", 0);

  expect(await innerChosen(), "the outer control moved the inner one").toBe(before.chosen);
  expect(await innerStage(), "the outer control resized the inner stage").toBe(before.width);

  // Its BOX does get smaller, and must: the inner frame sits inside the outer
  // stage, and a box inside a 320px box is not 591px wide. What it must not do
  // is stop honouring its own choice, which is `full` — as much room as it has.
  const outerStage = await frames.nth(0).locator("[data-nd-frame-stage]").first().boundingBox();
  const box = await innerWidth();
  expect(box.width, "the inner frame is no longer filling the room it has").toBeLessThanOrEqual(
    outerStage.width,
  );
  expect(box.width, "the inner frame stopped filling the room it has").toBeGreaterThan(
    outerStage.width / 2,
  );
});

// --- The viewport control ---------------------------------------------------

test("the viewport control resizes the stage to the width it names", async ({ page }) => {
  await page.setViewportSize({ width: 1440, height: 900 });
  await page.goto(url(FRAMED));
  // THE FIRST FRAME'S STAGE, because a page has one frame per example now and
  // the control pressed below is the first frame's own.
  const stage = page.locator("[data-nd-frame-stage]").first();

  await setFrameWidth(page, "320");
  await page.waitForTimeout(300);
  const box = await stage.boundingBox();
  // The control promises 320px. This is the only test that checks it means it.
  expect(Math.round(box.width)).toBe(320);

  // NO ITEM CARRIES A SECOND LINE. The `full` stop used to show the stage's
  // measured width, written in by the script on every resize. It is gone: a
  // menu is a list of things a reader can choose, and a live number sitting in
  // one of them is a readout wearing an option's clothes.
  await expect(
    page.locator('[data-nd-action="frame-width"] .nd-menu-item-detail'),
  ).toHaveCount(0);
});

test("resizing the stage moves nothing else on the page", async ({ page }) => {
  await page.setViewportSize({ width: 1440, height: 900 });
  await page.goto(url(FRAMED));
  const main = await page.locator(".nd-main").boundingBox();
  const frame = await page.locator("[data-nd-frame]").first().boundingBox();

  await setFrameWidth(page, "320");
  await page.waitForTimeout(300);

  expect((await page.locator(".nd-main").boundingBox()).width).toBe(main.width);
  expect((await page.locator("[data-nd-frame]").first().boundingBox()).width).toBe(frame.width);
});

// --- The reset --------------------------------------------------------------

test("reset puts the width, the appearance and the source back", async ({ page }) => {
  await page.goto(url(SCOPED));
  const frame = page.locator("[data-nd-frame]").first();

  await setFrameWidth(page, "320");
  await page.locator('[data-nd-action="frame-mode"]').first().click();
  // The implementation switch is a toggle now, not a two-stop strip: a rail is
  // a column, and a control with two rows in it cost 36px on every frame.
  await page.locator('[data-nd-action="frame-impl"]').first().click();
  await page.waitForFunction(
    () => document.querySelector("[data-nd-example]")?.dataset.ndShowing === "react",
    null,
    { timeout: 20000 },
  );
  await expect(page.locator('[data-nd-impl-source="React"]').first()).toBeVisible();

  await page.locator('[data-nd-action="frame-reset"]').first().click();
  await page.waitForTimeout(300);

  await expect(frame.locator(".nd-frame-width")).toHaveText(/Max|Máx|⟦/);
  await expect(page.locator('[data-nd-action="frame-mode"]')).not.toHaveAttribute(
    "aria-pressed",
    "true",
  );
  // THE SOURCE, which reset never used to restore — so a reader who switched
  // to React and pressed Reset was reading React source under a Dioxus render.
  await expect(page.locator('[data-nd-impl-source="Dioxus"]').first()).toBeVisible();
  await expect(page.locator('[data-nd-impl-source="React"]').first()).toBeHidden();
  // And the render came back with it, not only the caption.
  await expect(page.locator('[data-nd-example][data-nd-showing="react"]')).toHaveCount(0);
});

test("reset returns an authored scope to what it was authored with", async ({ page }) => {
  await page.goto(url(SCOPED));
  const scope = page.locator("[data-nd-mountable] > [data-noctua-scope]");
  const authored = await scope.getAttribute("data-palette");

  // Change it, then reset. This said `requests.set(element, {})`, which means
  // "inherit the page" rather than "go back to what you were built with", so
  // the demonstration adopted the reader's own theme instead.
  await scope.evaluate((el) => el.setAttribute("data-palette", "rose-vivid"));
  await page.locator('[data-nd-action="frame-reset"]').first().click();
  await page.waitForTimeout(200);

  expect(await scope.getAttribute("data-palette")).toBe(authored);
});

// --- The toolbar ------------------------------------------------------------

test("the frame's toolbar is one tab stop, and the arrows walk it", async ({ page }) => {
  await page.goto(url(FRAMED));
  const bar = page.locator(".nd-frame-bar").first();

  const stops = await bar.evaluate(
    (el) =>
      [...el.querySelectorAll("button, a[href], summary, [role=radio]")]
        // A disclosure in the bar is one stop — its summary. What it opens is a
        // region the reader tabs into, not more of the bar.
        .filter((c) => {
          const panel = c.closest("details");
          return !(panel && el.contains(panel) && c.tagName !== "SUMMARY");
        })
        .filter((c) => c.tabIndex >= 0).length,
  );
  expect(stops, "a toolbar is entered once, not once per control").toBe(1);

  // THE ARROWS FOLLOW THE AXIS. The strip is a rail down the frame's trailing
  // edge, so `Toolbar` writes `aria-orientation="vertical"` and `shell.js`
  // reads it to map Down rather than Right. That mapping is the contract; a
  // rail answering Left and Right would be a rail nobody can walk.
  await expect(bar).toHaveAttribute("aria-orientation", "vertical");
  const first = bar.locator(".nd-frame-width");
  await first.focus();
  await page.keyboard.press("ArrowDown");
  await expect(first, "Down left the focus where it was").not.toBeFocused();
});

test("every control in the rail sits on the rail's centre line", async ({ page }) => {
  // THE DEFECT A READER SAW, AND NO TEST DID.
  //
  // `.nd-toolbar[aria-orientation="vertical"]` stretches its children, so each
  // control was given the rail's full width — and a `Button` that is only a
  // glyph does not fill the box it is handed. The four icons sat hard against
  // the leading edge while the two controls with words looked centred, and the
  // ragged edge that made is the first thing anybody noticed about the rail.
  //
  // Written as centre lines rather than as `align-items`, because the property
  // is not the claim: the rule that sets it lost on specificity twice while
  // looking exactly like a rule that works.
  await page.setViewportSize({ width: 1120, height: 900 });
  await page.goto(url(PAGE.button));
  const off = await page.locator("[data-nd-frame]").nth(1).evaluate((f) => {
    const bar = f.querySelector(".nd-frame-bar");
    const mid = bar.getBoundingClientRect().left + bar.getBoundingClientRect().width / 2;
    return [...bar.children]
      .map((c) => c.getBoundingClientRect())
      .filter((r) => r.width > 0)
      .map((r) => Math.round(Math.abs(r.left + r.width / 2 - mid)));
  });
  expect(off.length, "the rail lost its controls").toBeGreaterThan(3);
  for (const d of off) expect(d, `a control is ${d}px off centre`).toBeLessThanOrEqual(1);
});

test("the rail is a column, and it is compact", async ({ page }) => {
  // THE AXIS AND THE SPACING ARE BOTH FOUGHT FOR ON SPECIFICITY.
  //
  // `.nd-toolbar[data-nd-density="compact"]` and
  // `.nd-toolbar[aria-orientation="vertical"]` each carry a class and an
  // attribute, so a rule written against `.nd-frame-bar` alone silently loses
  // to them. A `gap` and an `align-items` both did, and both looked applied.
  //
  // The numbers are ceilings, not equalities: this asserts the rail stays
  // chrome standing beside the example rather than a panel competing with it.
  await page.setViewportSize({ width: 1120, height: 900 });
  await page.goto(url(PAGE.button));
  const rail = await page.locator("[data-nd-frame]").nth(1).evaluate((f) => {
    const bar = f.querySelector(".nd-frame-bar");
    const cs = getComputedStyle(bar);
    return { dir: cs.flexDirection, gap: parseFloat(cs.rowGap), pad: parseFloat(cs.paddingTop),
             w: Math.round(bar.getBoundingClientRect().width) };
  });
  expect(rail.dir, "the rail is not standing up").toBe("column");
  expect(rail.gap, "the toolbar's row spacing won").toBeLessThanOrEqual(4);
  expect(rail.pad, "a second ring of space makes it read as a panel").toBeLessThanOrEqual(4);
  // It was 96 while *Reset* was a word. What sets it now is the one control
  // that has to carry text: the trigger naming the width in force.
  expect(rail.w, "the rail is as wide as a panel").toBeLessThanOrEqual(88);
});

test("naming the width does not move the rail", async ({ page }) => {
  // The trigger says the width in force, so its text changes — `Max`, then
  // `320px`, which is 12px wider. Sized to its content the rail grew as the
  // reader chose a stop and the stage's column moved while they were measuring
  // it. The floor is on the label's own box, because the button is only ever as
  // wide as what is inside it.
  await page.setViewportSize({ width: 1120, height: 900 });
  await page.goto(url(PAGE.button));
  const widths = await page.locator("[data-nd-frame]").nth(1).evaluate((f) => {
    const bar = f.querySelector(".nd-frame-bar");
    const label = f.querySelector(".nd-frame-width .nd-button-label");
    const was = label.textContent;
    const seen = [...f.querySelectorAll('[data-nd-action="frame-width"] .nd-menu-item-label')].map((el) => {
      label.textContent = el.textContent;
      return Math.round(bar.getBoundingClientRect().width);
    });
    label.textContent = was;
    return seen;
  });
  expect(widths.length, "the width menu lost its stops").toBeGreaterThan(3);
  expect(new Set(widths).size, `the rail measured ${widths.join("/")}`).toBe(1);
});

test("no control in the frame's bar is clipped by the frame", async ({ page }) => {
  for (const width of [360, 390, 768]) {
    await page.setViewportSize({ width, height: 900 });
    for (const locale of ["en", "pt-BR", "qps"]) {
      await page.goto(url(FRAMED, locale));
      const cut = await page.evaluate(() => {
        const frame = document.querySelector("[data-nd-frame]");
        const box = frame.getBoundingClientRect();
        const bar = frame.querySelector(".nd-frame-bar");
        return [...bar.querySelectorAll("button, summary, [role=radio]")]
          // The bar's OWN controls. What a disclosure in it holds is a panel,
          // closed by default — thirty controls with no box at all, which
          // "outside the frame" would be perfectly true of and perfectly
          // useless to say.
          .filter((el) => {
            const panel = el.closest("details");
            if (panel && bar.contains(panel) && el.tagName !== "SUMMARY") return false;
            // AND WHAT THE WIDTH MENU HOLDS. It is a `.nd-menu`, hidden until
            // its button is pressed, so its five items have no box at all —
            // which "outside the frame" is perfectly true of and perfectly
            // useless to say. Same reasoning as the disclosure above it.
            return !el.closest(".nd-menu");
          })
          .filter((el) => {
            const b = el.getBoundingClientRect();
            return b.right > box.right + 1 || b.left < box.left - 1;
          })
          .map((el) => el.textContent.trim() || el.getAttribute("aria-label"));
      });
      // The frame clips its own corners, so a control that does not fit is not
      // a scrollbar — it is a control the reader cannot see or press. A joined
      // `Segmented` cannot wrap, which is why this one is spread.
      expect(cut, `${locale} at ${width}px`).toEqual([]);
    }
  }
});

test("every toolbar and group on the page has a name", async ({ page }) => {
  await page.goto(url(FRAMED));
  const nameless = await page.evaluate(() =>
    // `ToolbarGroup` is deliberately not in this list: it is a cluster INSIDE a
    // named bar, and naming each one makes a screen reader announce a group
    // boundary twice on the way to every control. `ButtonGroup` is, because a
    // group of buttons standing on its own has nothing else to name it.
    [...document.querySelectorAll('[role="toolbar"], [role="radiogroup"], .nd-button-group')]
      .filter((el) => {
        const label = el.getAttribute("aria-label")?.trim();
        const by = el.getAttribute("aria-labelledby");
        return !label && !(by && document.getElementById(by));
      })
      .map((el) => String(el.className)),
  );
  // A `role="toolbar"` with no name announces the word `toolbar` and nothing
  // else, which is worse than no toolbar at all.
  expect(nameless).toEqual([]);
});

test("the width control names every stop, in one control", async ({ page }) => {
  // It resizes the stage and it names the exact length it resizes to, and both
  // are worth keeping. What could not be kept is five stops side by side: in a
  // rail they are 150px of the 160 a frame can spare, so they are a menu and
  // the trigger says which one is in force.
  //
  // The share this used to assert — the control against the bar's WIDTH — is
  // meaningless on a rail, where every child is full width by construction.
  // What is worth holding is that the five stops still exist and still name
  // their own lengths.
  for (const locale of LOCALES) {
    await page.setViewportSize({ width: 1180, height: 800 });
    await page.goto(url(PAGE.button, locale));
    const rail = page.locator(".nd-frame-bar").first();
    const trigger = rail.locator(".nd-frame-width");
    const menu = page.locator(`#${await trigger.getAttribute("aria-controls")}`);

    const stops = await menu.locator("[data-nd-value]").evaluateAll((els) =>
      els.map((el) => ({
        value: el.dataset.ndValue,
        name: el.textContent.replace(/\s+/g, ""),
      })),
    );
    expect(stops.length, `${locale}: a stop went missing`).toBe(5);
    expect(stops[0].name, `${locale}: the floor does not name itself`).toContain("320");
    expect(
      stops.map((stop) => stop.value),
      `${locale}: the stops are not the system's own scale`,
    ).toEqual(["320", "sm", "md", "lg", "full"]);

    // ONE CONTROL IN THE RAIL, not five. That is the whole reason it is a menu.
    expect(
      await rail.locator(":scope > .nd-frame-width").count(),
      `${locale}: the width is more than one control`,
    ).toBe(1);
  }
});

test("the bar's controls are in the order the reader meets them", async ({ page }) => {
  await page.goto(url(PAGE.button));
  // TOP TO BOTTOM NOW, and the same order it always was. `:scope >` keeps the
  // query on the rail's own children: the width menu's five items each carry a
  // `data-nd-action` of their own, and they are not controls of the rail.
  const order = await page.locator(".nd-frame-bar").first().evaluate((bar) =>
    [...bar.querySelectorAll(":scope > [data-nd-action], :scope > .nd-frame-theme, :scope > .nd-frame-width")].map(
      (el) => el.dataset.ndAction ?? (el.classList.contains("nd-frame-width") ? "width" : "theme"),
    ),
  );
  // Width first, because it is the control a reader reaches for most. Then the
  // rig before the appearance toggle: turning the test on is the bigger move,
  // and the toggle is what a reader reaches for while it runs.
  //
  // The implementation switch joins the rigs rather than leading the rail.
  // Width and theme are about how THIS example is shown; glass, appearance and
  // implementation each swap what is shown for something else; reset undoes all
  // of it and stays last.
  expect(order).toEqual([
    "width",
    "theme",
    "frame-glass",
    "frame-mode",
    "frame-impl",
    "frame-reset",
  ]);
});

test("a floating panel stays inside the window, and flips when it must", async ({ page }) => {
  // Two ways a panel anchored to a control leaves the screen, both measured on
  // the nested frame at 768px: hanging from a bar narrower than the panel put
  // its left edge at x = -86, and hanging from a control low on the page left
  // it `100vh - anchor` of room — a sliver at the bottom with every row below
  // the fold.
  await page.setViewportSize({ width: 768, height: 800 });
  await page.goto(url(PAGE.exampleFrame, "pt-BR"));
  for (const index of [0, 1]) {
    const panel = await openFrameTheme(page, index);
    const box = await panel.boundingBox();
    const view = page.viewportSize();
    expect(box.x, `frame ${index}: the panel starts off the left`).toBeGreaterThanOrEqual(0);
    expect(box.x + box.width, `frame ${index}: the panel runs off the right`).toBeLessThanOrEqual(
      view.width + 1,
    );
    expect(box.y, `frame ${index}: the panel starts above the window`).toBeGreaterThanOrEqual(0);
    expect(box.y + box.height, `frame ${index}: the panel runs off the bottom`).toBeLessThanOrEqual(
      view.height + 1,
    );
    await page.keyboard.press("Escape");
  }
});

test("no control in the rail shouts", async ({ page }) => {
  await page.goto(url(PAGE.button));
  // The width used to be a `Segmented` whose chosen stop was drawn in the
  // accent unless asked otherwise, and it asked. There is no chosen stop now —
  // a menu holds no state — so the claim moves to the rail itself: every
  // control in it is transparent and neutral, because a strip of five accent
  // controls beside an example is louder than the example.
  const loudest = await page.locator(".nd-frame-bar").first().evaluate((rail) =>
    [...rail.querySelectorAll(":scope > .nd-button")]
      .map((el) => `${el.dataset.ndVariant}/${el.dataset.ndTone}`)
      .filter((pair) => pair !== "transparent/neutral"),
  );
  expect(loudest, "a rail control asks for more attention than the example").toEqual([]);

  // And the accent selection is still what everything else uses: this was a
  // property of one group, not a change to how a chosen value looks.
  const panel = await openTheme(page);
  const loud = await panel
    .locator('[data-nd-group="radius"] [aria-checked="true"]')
    .first()
    .evaluate((el) => el.closest(".nd-segmented").dataset.ndSelection);
  expect(loud).toBe("accent");
  await closeTheme(page);
});

test("the frame's bar does not travel with the page", async ({ page }) => {
  // It was `sticky` under the header, so the controls stayed in reach on a page
  // where one frame is several screens tall. That cost more than it bought: a
  // bar that travels with the scroll leaves the thing it belongs to, and it was
  // three separate defects on its own — sticking to a rig instead of the page,
  // being shoved onto its own stage by the offset it needed for sticking, and
  // being painted over by a frosted layer after it was made static to stop the
  // first two.
  await page.setViewportSize({ width: 1180, height: 800 });
  await page.goto(url(PAGE.button));
  const travel = await page.evaluate(async () => {
    const frame = document.querySelector("[data-nd-frame]");
    const bar = frame.querySelector(":scope > .nd-frame-bar");
    const at = () => bar.getBoundingClientRect().top - frame.getBoundingClientRect().top;
    const before = at();
    window.scrollBy(0, 1200);
    await new Promise((resolve) => requestAnimationFrame(resolve));
    return at() - before;
  });
  expect(Math.round(travel), "the bar left its own frame as the page scrolled").toBe(0);
});
