// The theme menu: one component, two placements, two things it themes.
//
// It used to be two surfaces that did not know about each other — a mode
// control in the header and a panel that also had a mode row. There is one
// menu now, and the example frame opens the same one to theme its own stage.
// What decides where a value lands is where the control SITS, so most of what
// is worth asserting here is about place rather than about markup.

import { readFileSync } from "node:fs";
import { test, expect } from "@playwright/test";
import { ANY, LOCALES, PAGE, closeTheme, openFrameTheme, openTheme, url } from "./pages.js";

/** The generated model — `dist/theme.json` exists for readers with no toolchain. */
const THEME = JSON.parse(readFileSync(new URL("../../dist/theme.json", import.meta.url), "utf8"));

const FRAMED = PAGE.component;

/** A colour string as four bytes, via a 1×1 canvas. */
async function paint(page, value) {
  return page.evaluate((v) => {
    const canvas = document.createElement("canvas");
    canvas.width = canvas.height = 1;
    const ctx = canvas.getContext("2d");
    ctx.fillStyle = v;
    ctx.fillRect(0, 0, 1, 1);
    return [...ctx.getImageData(0, 0, 1, 1).data].join(",");
  }, value);
}

/** What an element actually paints. */
async function background(page, selector) {
  const raw = await page
    .locator(selector)
    .first()
    .evaluate((el) => getComputedStyle(el).backgroundColor); // allow-literal: painted into a canvas by `paint`, the recipe the gate prescribes
  return paint(page, raw);
}

// --- The header ---------------------------------------------------------

test("the header carries no mode control of its own", async ({ page }) => {
  await page.goto(url(ANY));

  // Two ways to change one setting, and the header spent a third of its 320px
  // budget on the duplicate. There is one, and it is in the menu.
  const loose = await page.evaluate(
    () =>
      [...document.querySelectorAll(".nd-header-controls > *")].filter((el) =>
        el.matches('[data-nd-group="mode"]'),
      ).length,
  );
  expect(loose, "a mode control is loose in the header again").toBe(0);
  await expect(page.locator('.nd-panel [data-nd-group="mode"]')).toHaveCount(1);
});

test("the menu themes the page", async ({ page }) => {
  await page.goto(url(ANY));
  const before = await background(page, "html");

  const panel = await openTheme(page);
  await panel.locator('[data-nd-group="mode"] [data-nd-value="dark"]').click();
  const dark = await background(page, "html");
  expect(dark, "choosing dark did not repaint the page").not.toBe(before);

  await panel.locator('[data-nd-group="mode"] [data-nd-value="light"]').click();
  expect(await background(page, "html"), "light and dark paint the same").not.toBe(dark);
});

// --- The same menu, inside a frame --------------------------------------

test("the frame's menu themes the stage and leaves the page alone", async ({ page }) => {
  await page.goto(url(FRAMED));
  await page.locator(".nd-frame-theme > summary").first().click();

  const pageBefore = await background(page, "html");
  const menu = page.locator("[data-nd-frame] .nd-theme-menu").first();

  await menu.locator('[data-nd-group="radius"] [data-nd-value="circular"]').click();
  await menu.locator('[data-nd-group="accent"] [data-nd-value="rose"]').click();

  const stage = page.locator("[data-nd-frame-stage]").first();
  expect(await stage.getAttribute("data-noctua-radius")).toBe("circular");
  expect(await stage.getAttribute("data-palette")).toContain("rose");

  // The page: untouched, in the attribute and in the pixels.
  expect(await page.locator("html").getAttribute("data-noctua-radius")).toBeNull();
  expect(await page.locator("html").getAttribute("data-palette")).not.toContain("rose");
  expect(await background(page, "html"), "the page moved").toBe(pageBefore);
});

test("the frame's menu and its quick toggle show one state", async ({ page }) => {
  await page.goto(url(FRAMED));
  const toggle = page.locator('[data-nd-action="frame-mode"]').first();

  // Invert with the toggle, then ask the menu what the stage is in. They drive
  // the same request, so the menu is not allowed to disagree with the button.
  await toggle.click();
  const forced = await page.locator("[data-nd-frame-stage]").first().getAttribute("data-theme");
  await page.locator(".nd-frame-theme > summary").first().click();
  await expect(
    page.locator(`[data-nd-frame] [data-nd-group="mode"] [data-nd-value="${forced}"]`).first(),
  ).toHaveAttribute("aria-checked", "true");

  // And the other way: choose in the menu, and the toggle knows.
  await page.locator('[data-nd-frame] [data-nd-group="mode"] [data-nd-value="system"]').first().click();
  await expect(toggle).not.toHaveAttribute("aria-pressed", "true");
});

test("the frame's menu offers every setting a scope can carry, and no other", async ({ page }) => {
  await page.goto(url(FRAMED));
  await page.locator(".nd-frame-theme > summary").first().click();

  // BOTH HALVES, FROM THE MODEL, because each one alone has already shipped a
  // defect. This test used to assert only the first half and it named
  // `"blackout"` — a setting renamed to `backdrop` long before — so one of the
  // four it claimed to guard was a selector matching nothing, passing whatever
  // the menu did.
  //
  // A control offered where it does nothing is a control that lies. So is a
  // setting a scope can carry and no control offers.
  const groups = await page.locator("[data-nd-frame] [data-nd-group]").evaluateAll((els) =>
    els.map((el) => el.dataset.ndGroup),
  );
  for (const [name, setting] of Object.entries(THEME.settings)) {
    // A DERIVED SETTING HAS NO CONTROL OF ITS OWN, and `derivedFrom` is in the
    // model so this exception is read rather than remembered: `palette` is
    // computed from `accent` × `saturation`, and those two are the rows.
    if (setting.derivedFrom.length > 0) continue;
    if (setting.scope) {
      expect(groups, `${name} is scopable and the frame's menu does not offer it`).toContain(name);
    } else {
      expect(groups, `${name} is offered on a frame and cannot work there`).not.toContain(name);
    }
  }
});

// --- A frame is a boundary, not a layer -----------------------------------
//
// The ExampleFrame page puts one frame inside another, which is the only place
// these can be asked. Both of the following were true and both were wrong.

/** One frame's own parts, never the parts of a frame nested inside it. */
function frameParts(page, index) {
  const frame = page.locator("[data-nd-frame]").nth(index);
  return {
    frame,
    menu: frame.locator(".nd-frame-theme").nth(index === 0 ? 0 : 0),
    stage: frame.locator("[data-nd-frame-stage]").first(),
    // `>` all the way down: a descendant selector inside the outer frame also
    // matches the inner frame's copy of everything, which is the trap this
    // whole spec is about.
    own: (selector) => frame.locator(`:scope > .nd-frame-bar ${selector}`),
  };
}

test("a nested frame's menu themes its own stage and not its parent's", async ({ page }) => {
  await page.goto(url(PAGE.exampleFrame));
  // The outer frame is the one with a frame inside it — see below.
  const at = await page
    .locator("[data-nd-frame]")
    .evaluateAll((frames) => frames.findIndex((frame) => frame.querySelector("[data-nd-frame]")));
  expect(at, "this spec needs the page where one frame sits inside another").toBeGreaterThan(-1);
  const outer = frameParts(page, at);
  const inner = frameParts(page, at + 1);

  // `themedBy` asked for the nearest SCOPE and only then for the nearest
  // frame. With the outer stage themed, that scope is nearer to the inner
  // frame's toolbar than the inner frame is — so the nested example drove its
  // own parent.
  await outer.own(".nd-frame-theme > summary").click();
  await outer.own('[data-nd-group="radius"] [data-nd-value="circular"]').click();
  await outer.own(".nd-frame-theme > summary").click();

  await inner.own(".nd-frame-theme > summary").click();
  await inner.own('[data-nd-group="radius"] [data-nd-value="square"]').click();

  expect(await inner.stage.getAttribute("data-noctua-radius")).toBe("square");
  expect(
    await outer.stage.getAttribute("data-noctua-radius"),
    "the inner frame reached out and changed its parent",
  ).toBe("circular");
});

test("a frame reaches the frame inside it, and the page cannot reach past it", async ({ page }) => {
  await page.goto(url(PAGE.exampleFrame));
  // FOUND, NOT COUNTED. This page used to open with the nested pair, so the
  // outer frame was the first and the inner one the second. A page has a frame
  // per example now, so the pair is wherever its example is — and the outer
  // frame is the one with a frame inside it, which is a description rather than
  // a position.
  const at = await page
    .locator("[data-nd-frame]")
    .evaluateAll((frames) => frames.findIndex((frame) => frame.querySelector("[data-nd-frame]")));
  expect(at, "this spec needs the page where one frame sits inside another").toBeGreaterThan(-1);
  const outer = frameParts(page, at);
  const inner = frameParts(page, at + 1);

  // EVERY scopable setting, from the generated model rather than from a list
  // written here — so a setting added to `spec/theme.toml` is covered the day
  // it exists. The rule is generic: a setting a frame STATES owns its subtree,
  // and one nobody stated comes from outside. Neither half names a setting.
  const scopable = Object.entries(THEME.settings)
    .filter(([, setting]) => setting.scope && setting.attribute)
    .map(([name, setting]) => [name, setting.attribute]);
  expect(scopable.length, "the model publishes no scopable settings").toBeGreaterThan(3);

  // The page states its own appearance first. `system` is the ABSENCE of
  // `data-theme`, so comparing against a page in `system` compares "light" with
  // nothing and proves neither. With the page explicit, "the inner frame
  // matches the outer stage rather than the page" is a plain equality.
  const header = await openTheme(page);
  await header.locator('[data-nd-group="mode"] [data-nd-value="light"]').click();
  await header.locator('[data-nd-group="accent"] [data-nd-value="teal"]').click();
  await closeTheme(page);

  await outer.own(".nd-frame-theme > summary").click();
  for (const [group, value] of [
    ["mode", "dark"],
    ["radius", "circular"],
    ["glass", "full"],
    ["font", "mono"],
    ["accent", "rose"],
  ]) {
    await outer.own(`[data-nd-group="${group}"] [data-nd-value="${value}"]`).click();
  }
  await outer.own(".nd-frame-theme > summary").click();

  // The outer stage really did change — otherwise this proves nothing.
  expect(await outer.stage.getAttribute("data-theme")).toBe("dark");

  // WHAT IT PAINTS, NOT WHAT IT CARRIES. An earlier version of this test
  // asserted the inner stage had no scope attributes and passed either way —
  // because carrying nothing is exactly how a subtree inherits everything. The
  // pixels are the only honest question.
  const [innerPaint, outerPaint, pagePaint] = await Promise.all(
    [inner.stage, outer.stage, page.locator("body")].map((locator) =>
      locator.evaluate((el) => {
        const canvas = document.createElement("canvas");
        canvas.width = canvas.height = 1;
        const ctx = canvas.getContext("2d");
        ctx.fillStyle = getComputedStyle(el).getPropertyValue("--nc-color-surface"); // allow-literal: fed straight into the canvas below, which is the recipe the gate prescribes
        ctx.fillRect(0, 0, 1, 1);
        return [...ctx.getImageData(0, 0, 1, 1).data].join(",");
      }),
    ),
  );
  expect(outerPaint, "the outer frame did not actually change").not.toBe(pagePaint);
  expect(innerPaint, "the page reached past the frame around it").toBe(outerPaint);

  // And every scopable setting, from the generated model rather than from a
  // list written here — so a setting added to the spec is covered without
  // touching this test.
  //
  // "In force" is what the cascade would use: the nearest ancestor carrying the
  // attribute, and the model's default when none does. A frame that has stated
  // nothing carries nothing, so a raw `getAttribute` would compare a value with
  // `null` and prove neither. Both spellings mean the same value and this is
  // what compares them.
  const inForce = async (locator, name, attribute) =>
    (await locator.evaluate((el, a) => el.closest(`[${a}]`)?.getAttribute(a) ?? null, attribute)) ??
    THEME.settings[name].default;

  // THE FRAME AND ITS STAGE, because inside the outer stage BOTH are content.
  // "A control that changes appearance along with the thing it controls takes
  // away the reader's anchor" was written for a frame sitting on the real page,
  // and there it still holds: the page's own header keeps the page's theme
  // whatever a frame on it is showing. One level in, the inner frame's bar and
  // border are things the outer stage is displaying, and they follow it.
  for (const part of [inner.frame, inner.stage]) {
    for (const [name, attribute] of scopable) {
      expect(
        await inForce(part, name, attribute),
        `${name} did not reach the frame inside`,
      ).toBe(await inForce(outer.stage, name, attribute));
    }
  }
});

test("a frame reaches inside even when the page is following the machine", async ({ page }) => {
  // `system` is stamped as the ABSENCE of `data-theme`, and absence is also how
  // a subtree says "whatever is above me" — the same markup meaning two things.
  // This is the only test that reaches that branch: everywhere else the page's
  // mode is set explicitly.
  await page.emulateMedia({ colorScheme: "light" });
  await page.goto(url(PAGE.exampleFrame));
  const outer = frameParts(page, 0);
  const inner = frameParts(page, 1);
  expect(await page.locator("html").getAttribute("data-theme"), "the page is not on `system`").toBeNull();

  await outer.own('[data-nd-action="frame-mode"]').click();
  expect(await outer.stage.getAttribute("data-theme")).toBe("dark");
  // The inner stage states NOTHING — which is how it inherits — and what
  // reaches it is the outer stage's dark. Asserting the raw attribute alone
  // would compare `null` with a value and prove neither, so both halves are
  // asked: what it carries, and what is in force.
  expect(
    await inner.stage.getAttribute("data-theme"),
    "the nested frame stamped a mode nobody asked it for",
  ).toBeNull();
  expect(
    await inner.stage.evaluate((el) => el.closest("[data-theme]")?.getAttribute("data-theme")),
    "the nested frame did not follow the frame around it into the dark",
  ).toBe("dark");
});

// --- Geometry, which is where three shipped defects lived ----------------
//
// Every one of these was visible at a glance and none of them was caught,
// because nothing in this suite ever asked where a control ENDED UP. The three
// below are the general form of each: they would fail on any panel, any group
// and any locale that made the same mistake, not just on the one that did.

test("no control overlaps the one beside it", async ({ page }) => {
  // The thirteen accent swatches each overhung their grid track by 7px and
  // landed 2.93px inside the next chip's border, because the track's minimum
  // was 24px under an item that `hide_label` makes a fixed 32px.
  for (const locale of LOCALES) {
    await page.goto(url(PAGE.exampleFrame, locale));
    const panel = await openTheme(page);
    const worst = await panel.evaluate((root) => {
      let overlap = 0;
      let group = null;
      for (const set of root.querySelectorAll(".nd-segmented")) {
        // A JOINED group shares its edges on purpose — that is what makes it
        // read as one control — so it is the one appearance exempted, by name
        // rather than by tolerance.
        if (set.dataset.ndAppearance === "joined") continue;
        const boxes = [...set.children].map((el) => el.getBoundingClientRect());
        for (let i = 1; i < boxes.length; i++) {
          if (Math.abs(boxes[i].top - boxes[i - 1].top) > 2) continue; // a wrapped line
          const over = boxes[i - 1].right - boxes[i].left;
          if (over > overlap) {
            overlap = over;
            group = set.dataset.ndGroup;
          }
        }
      }
      return { overlap, group };
    });
    expect(worst.overlap, `${locale}: ${worst.group} overlaps itself`).toBeLessThanOrEqual(0.5);
    await closeTheme(page);
  }
});

test("no row's name runs over the control beside it", async ({ page }) => {
  // The name column is a fixed `space-24`, and `⟦Saatuuraatiioon⟧` is 138px of
  // it with nowhere to wrap — so in the pseudo-locale the name ran straight
  // across the control. `qps` exists to find exactly this, and it only finds it
  // if something asks; nothing did.
  for (const locale of LOCALES) {
    await page.goto(url(ANY, locale));
    const panel = await openTheme(page);
    const spilling = await panel.evaluate((root) =>
      [...root.querySelectorAll(".nd-label, .nd-note-line, .nd-switch-label, .nd-theme-title")]
        .filter((el) => el.scrollWidth > el.clientWidth + 1)
        .map((el) => el.textContent.trim()),
    );
    expect(spilling, `${locale}: text wider than the box holding it`).toEqual([]);
    await closeTheme(page);
  }
});

test("every row of a frame's menu can be reached", async ({ page }) => {
  // The menu was a band inside the frame, and `.nd-frame` is `overflow: clip`:
  // it was 457px tall, so on a 172px frame all eight of its rows were cut off.
  // Hit-testing catches that and being covered with one question — is the thing
  // the reader would click actually the thing that is there?
  //
  // 768 AND `pt-BR` ON PURPOSE. This passed at a wide viewport in English and
  // failed here, because a longer toolbar wraps the bar onto two lines and
  // moves the panel down over the frame nested inside — whose own bar is
  // `sticky` at the same layer and later in the document, so it painted over
  // the menu's first row. A reachability test that only runs at one width and
  // one locale is a test that finds the easy half.
  await page.setViewportSize({ width: 768, height: 800 });
  await page.goto(url(PAGE.exampleFrame, "pt-BR"));
  for (const index of [0, 1]) {
    const panel = await openFrameTheme(page, index);
    const unreachable = await panel.evaluate((root) => {
      const missed = [];
      const body = root.querySelector(".nd-panel-body");
      for (const field of root.querySelectorAll(".nd-theme-field")) {
        // THE PANEL'S OWN SCROLL, not `scrollIntoView`. That one scrolls every
        // ancestor, including the page — and a panel anchored to a summary that
        // has scrolled out of view closes itself, which is the behaviour, so
        // the test would be measuring a menu it had just dismissed.
        body.scrollTop = field.offsetTop - body.clientHeight / 2;
        const box = field.getBoundingClientRect();
        const hit = document.elementFromPoint(box.left + 4, box.top + box.height / 2);
        if (!hit || !root.contains(hit)) missed.push(field.querySelector(".nd-label")?.textContent);
      }
      return missed;
    });
    expect(unreachable, `frame ${index}: rows nobody can click`).toEqual([]);

    // And the panel itself is on the screen, which is what "floating" bought.
    const box = await panel.boundingBox();
    const view = page.viewportSize();
    expect(box.y).toBeGreaterThanOrEqual(0);
    expect(box.y + box.height).toBeLessThanOrEqual(view.height + 1);
    // ESCAPE, not a second click on the summary. An open panel covers the
    // control that opened it, so the click lands on the panel and the menu
    // stays open — and the next frame's menu then opens underneath a panel
    // that is still there, which is a test failing on its own tidying up.
    await page.keyboard.press("Escape");
  }
});

test("a setting is further from the next one than from its own control", async ({ page }) => {
  // Rows sat 12px apart while a row's name sat 16px from its own control, so
  // `Corners` and `Glass` read as one control with seven buttons. The rule is
  // the general one, and it is the only spacing assertion worth writing:
  // inside a control < within a row < between rows < between sections.
  await page.goto(url(ANY));
  const panel = await openTheme(page);
  const gaps = await panel.evaluate((root) => {
    const px = (el, prop) => parseFloat(getComputedStyle(el).getPropertyValue(prop));
    const sections = [...root.querySelectorAll(".nd-theme-section")];
    return {
      inControl: px(root.querySelector('[data-nd-group="radius"]'), "column-gap"),
      inRow: px(root.querySelector(".nd-theme-field"), "column-gap"),
      betweenRows: px(root.querySelector(".nd-theme-rows"), "row-gap"),
      betweenSections:
        sections[1].getBoundingClientRect().top - sections[0].getBoundingClientRect().bottom,
    };
  });
  expect(gaps.inControl, "a group's parts are not the tightest thing").toBeLessThan(gaps.inRow);
  expect(gaps.inRow, "a row's halves are further apart than two rows").toBeLessThan(
    gaps.betweenRows,
  );
  expect(gaps.betweenRows, "two rows are as far apart as two sections").toBeLessThanOrEqual(
    gaps.betweenSections,
  );

  // AND NOTHING ABOVE THE TITLE BUT THE PANEL'S OWN PADDING. `.nd-heading`
  // carries the gap between a heading and the text above it — prose rhythm,
  // meaningless where there is nothing above — and it beat the panel's
  // `margin: 0` on source order alone, so 32px of nothing sat over the title of
  // a floating menu.
  const above = await panel.evaluate((root) => {
    const title = root.querySelector(".nd-theme-title").getBoundingClientRect();
    return title.top - root.getBoundingClientRect().top;
  });
  const padding = await panel.evaluate((root) =>
    parseFloat(getComputedStyle(root.querySelector(".nd-panel-body")).paddingBlockStart),
  );
  expect(above, "something is padding the top of the menu").toBeLessThanOrEqual(padding + 4);
  await closeTheme(page);
});

test("the frame's menu starts closed", async ({ page }) => {
  await page.goto(url(FRAMED));
  const menu = page.locator(".nd-frame-theme").first();
  await expect(menu).not.toHaveAttribute("open", "");
  await expect(page.locator("[data-nd-frame] .nd-theme-menu").first()).toBeHidden();

  await menu.locator("summary").click();
  await expect(page.locator("[data-nd-frame] .nd-theme-menu").first()).toBeVisible();
});

// --- Switch ---------------------------------------------------------------

test("every switch announces its state, and flipping one changes the page", async ({ page }) => {
  await page.goto(url(ANY));
  await openTheme(page);

  const stateless = await page.evaluate(() =>
    [...document.querySelectorAll('[role="switch"]')]
      .filter((el) => !el.hasAttribute("aria-checked"))
      .map((el) => el.textContent.trim()),
  );
  // There is no such thing as a switch whose state is unknown.
  expect(stateless).toEqual([]);

  const grain = page.locator('.nd-switch[data-nd-group="grain"]');
  await expect(grain).toHaveAttribute("aria-checked", "false");
  await grain.click();
  await expect(grain).toHaveAttribute("aria-checked", "true");
  // The attribute is not the point — the setting is.
  await expect(page.locator("html")).toHaveAttribute("data-noctua-grain", "on");

  await grain.click();
  await expect(grain).toHaveAttribute("aria-checked", "false");
  await closeTheme(page);
});
