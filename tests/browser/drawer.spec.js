// The panel along one edge of a window, and the rail down each side of it.
//
// WHAT THE CASES CANNOT SAY. `spec/components/drawer.cases.toml` holds the
// markup: the role, the name, the side, and that Escape turns nothing by
// itself. Everything `NOCTUA.md §12.11` says about a drawer is geometry and
// the keyboard, and that is asked here: how far it opens, that its rails are
// the footer's corners continued, that a collapsed drawer is nothing and out
// of everything, with a script and without one, what Escape says, and where
// the focus is when a drawer shuts around it.
//
// THE CALLER IS THE PAGE. A drawer is turned by whoever renders it. On the
// site that is `docs-site/assets/shell.js`: a control marked `drawer-toggle`
// turns the drawer in its own window, and `nd-drawer-dismiss` shuts it. A test
// that needs the drawer turned presses that control or writes the attribute a
// caller's re-render would write.

import { expect, test } from "@playwright/test";

import { LOCALES, PAGE, showParts, url } from "./pages.js";
import { pixelAt } from "./pixels.js";

// THE ENGINE'S SCROLLBARS ARE ON. The runner hides every scrollbar, and a
// rail's claim to draw none would then be a claim nothing could refuse. A
// window draws its bars, and so does this file.
test.use({
  launchOptions: async ({ launchOptions }, use) =>
    use({ ...launchOptions, ignoreDefaultArgs: ["--hide-scrollbars"] }),
});

async function open(page, locale = "en") {
  await page.goto(url(PAGE.drawer, locale));
  await showParts(page);
}

const drawer = (page, name) => page.locator(`[data-nd-drawer="${name}"]`).first();

/** The window an example's drawer stands in. */
const windowOf = (page, name) => page.locator(`.nd-example-window:has([data-nd-drawer="${name}"])`).first();

/** A token's length in pixels, measured on a box and not parsed off a string. */
async function token(page, name) {
  return page.evaluate((property) => {
    const probe = document.createElement("div");
    probe.style.position = "absolute";
    probe.style.blockSize = `var(${property})`;
    document.body.append(probe);
    const size = probe.getBoundingClientRect().height;
    probe.remove();
    return size;
  }, name);
}

/** The share of a window a drawer may take, as a fraction, measured likewise. */
async function share(page) {
  return page.evaluate(() => {
    const frame = document.createElement("div");
    frame.style.position = "absolute";
    frame.style.blockSize = "1000px"; // allow-literal: the probe's own height, which the share is read against and divided by
    const probe = document.createElement("div");
    probe.style.blockSize = "var(--noctua-shell-drawer-max)";
    frame.append(probe);
    document.body.append(frame);
    const size = probe.getBoundingClientRect().height;
    frame.remove();
    return size / 1000;
  });
}

const box = (locator) =>
  locator.evaluate((one) => {
    const at = one.getBoundingClientRect();
    return { left: at.left, right: at.right, top: at.top, bottom: at.bottom, width: at.width, height: at.height };
  });

/** The box inside a window's own border, which is what its parts divide. */
const inner = (locator) =>
  locator.evaluate((one) => {
    const at = one.getBoundingClientRect();
    // The border's four widths as lengths, which a computed style answers in
    // whole or part pixels. `clientWidth` rounds, and a window is not a whole
    // number of pixels wide at every width.
    const style = getComputedStyle(one);
    const edge = (side) => Number.parseFloat(style.getPropertyValue(`border-${side}-width`)) || 0;
    const left = at.left + edge("left");
    const top = at.top + edge("top");
    const right = at.right - edge("right");
    const bottom = at.bottom - edge("bottom");
    return {
      left,
      top,
      right,
      bottom,
      width: right - left,
      height: bottom - top,
      overflow: one.scrollWidth - one.clientWidth,
    };
  });

/**
 * Turns a drawer and stops it partway, in one task, and says how many
 * transitions it started and where its box is there.
 *
 * A box read off a clock is a different box on a loaded machine, and one read
 * at a named point of a stopped transition is not. The point is a tenth of the
 * way in time: the drawer's easing covers most of its distance early, so by
 * the middle of its time a drawer is all but where it is going.
 *
 * AND IN ONE TASK, because a transition that has finished has nothing left to
 * stop. Turned in one call and stopped in the next, a drawer on a loaded
 * machine had slid all the way shut in between, `getAnimations` was empty, and
 * "partway" read a box of nothing: "the drawer is not partway shut", 0. Asking
 * for the animations right after the write is what starts them, at their
 * first instant.
 */
const turnPartway = (locator, value) =>
  locator.evaluate((one, next) => {
    one.setAttribute("data-nd-collapsed", next);
    const running = one.getAnimations();
    for (const each of running) {
      each.pause();
      each.currentTime = (Number(each.effect.getComputedTiming().duration) || 0) / 10;
    }
    const at = one.getBoundingClientRect();
    return {
      moving: running.length,
      left: at.left,
      right: at.right,
      top: at.top,
      bottom: at.bottom,
      width: at.width,
      height: at.height,
    };
  }, value);

/** Waits until nothing in the drawer is still moving. */
const settled = (locator) =>
  locator.evaluate((one) => Promise.all(one.getAnimations().map((each) => each.finished.catch(() => null))));

for (const width of [320, 768, 1280]) {
  test(`the rails are the footer's corners continued, at ${width}`, async ({ page }) => {
    await page.setViewportSize({ width, height: 900 });
    for (const locale of LOCALES) {
      await open(page, locale);
      const rail = await token(page, "--noctua-shell-rail");
      expect(rail, "the page declares no rail width").toBeGreaterThan(0);

      const frame = windowOf(page, "overview-drawer");
      const edge = await inner(frame);
      expect(edge.overflow, `${locale}: the window scrolls sideways`).toBeLessThanOrEqual(0);

      const whole = await box(drawer(page, "overview-drawer"));
      const footer = await box(frame.locator(".nd-footer"));
      // BELOW THE FOOTER, WITH NOTHING BETWEEN THEM, and as wide as the window.
      expect(whole.top, `${locale}: a gap between the footer and the drawer`).toBeCloseTo(footer.bottom, 1);
      expect(whole.left).toBeCloseTo(edge.left, 1);
      expect(whole.right).toBeCloseTo(edge.right, 1);
      expect(whole.bottom, `${locale}: the drawer is not at the window's bottom edge`).toBeCloseTo(edge.bottom, 1);
      expect(whole.height, `${locale}: an open drawer has no height`).toBeGreaterThan(rail / 2);

      const start = await box(drawer(page, "overview-drawer").locator('.nd-drawer-rail[data-nd-side="start"]'));
      const end = await box(drawer(page, "overview-drawer").locator('.nd-drawer-rail[data-nd-side="end"]'));
      const cornerStart = await box(frame.locator('.nd-footer-corner[data-nd-side="start"]'));
      const cornerEnd = await box(frame.locator('.nd-footer-corner[data-nd-side="end"]'));
      for (const [name, one, corner] of [["start", start, cornerStart], ["end", end, cornerEnd]]) {
        expect(one.width, `${locale}: the ${name} rail is not a rail wide`).toBeCloseTo(rail, 1);
        // EXACTLY UNDER ITS CORNER: the same two edges, one box above the other.
        expect(one.left, `${locale}: the ${name} rail is not under its corner`).toBeCloseTo(corner.left, 1);
        expect(one.right, `${locale}: the ${name} rail is not under its corner`).toBeCloseTo(corner.right, 1);
        // AND AS TALL AS THE DRAWER, whatever is in it.
        expect(one.top, `${locale}: the ${name} rail starts under the drawer's edge`).toBeCloseTo(whole.top, 1);
        expect(one.bottom, `${locale}: the ${name} rail stops short of the drawer`).toBeCloseTo(whole.bottom, 1);
      }
      // The content is what is between them, and it touches both.
      const content = await box(drawer(page, "overview-drawer").locator(".nd-drawer-panel > :not(.nd-drawer-rail)"));
      expect(content.left, `${locale}: the content starts under a rail`).toBeCloseTo(start.right, 1);
      expect(content.right, `${locale}: the content ends under a rail`).toBeCloseTo(end.left, 1);
      expect(content.width, `${locale}: the rails left the content no room`).toBeGreaterThan(0);
    }
  });

  test(`a drawer opens as far as its content and never past its share, at ${width}`, async ({ page }) => {
    await page.setViewportSize({ width, height: 900 });
    for (const locale of LOCALES) {
      await open(page, locale);
      const limit = await share(page);
      expect(limit, "the page declares no share for a drawer").toBeGreaterThan(0);
      expect(limit).toBeLessThan(0.5);

      // A LITTLE IN IT: as tall as that, and under the limit.
      const small = await box(drawer(page, "drawer-basic"));
      const smallEdge = await inner(windowOf(page, "drawer-basic"));
      const smallContent = await box(drawer(page, "drawer-basic").locator(".nd-drawer-panel > *"));
      expect(small.height, `${locale}: a drawer is not as tall as what is in it`).toBeCloseTo(smallContent.height, 1);
      expect(small.height).toBeLessThan(smallEdge.height * limit);

      // MORE THAN ITS SHARE: the share exactly, the content scrolls, and the
      // rail is still the whole height of the drawer.
      const long = drawer(page, "drawer-long");
      const longEdge = await inner(windowOf(page, "drawer-long"));
      const tall = await box(long);
      expect(tall.height, `${locale}: a full drawer is not its share of the window`).toBeCloseTo(longEdge.height * limit, 0);
      expect(tall.bottom).toBeCloseTo(longEdge.bottom, 1);
      const scroll = await long
        .locator(".nd-drawer-panel > :not(.nd-drawer-rail)")
        .evaluate((one) => ({ all: one.scrollHeight, seen: one.clientHeight, overflow: getComputedStyle(one).overflowY }));
      expect(scroll.all, `${locale}: the content of a full drawer does not scroll`).toBeGreaterThan(scroll.seen + 1);
      expect(scroll.overflow).toBe("auto");
      const rail = await box(long.locator(".nd-drawer-rail"));
      expect(rail.height, `${locale}: the rail scrolled away with the content`).toBeCloseTo(tall.height, 1);
      // ALWAYS SHORTER THAN THE WORK ABOVE IT (`NOCTUA.md §12.11`).
      const work = await box(windowOf(page, "drawer-long").locator(".nd-example-work"));
      expect(tall.height, `${locale}: the drawer is taller than the work`).toBeLessThan(work.height);
    }
  });

  test(`a drawer at a side grows across the window and stands at its side, at ${width}`, async ({ page }) => {
    await page.setViewportSize({ width, height: 900 });
    for (const locale of LOCALES) {
      await open(page, locale);
      const limit = await share(page);
      const frame = windowOf(page, "drawer-start");
      const edge = await inner(frame);
      expect(edge.overflow, `${locale}: the window scrolls sideways`).toBeLessThanOrEqual(0);
      const start = await box(drawer(page, "drawer-start"));
      const end = await box(drawer(page, "drawer-end"));
      for (const [name, one] of [["start", start], ["end", end]]) {
        expect(one.width, `${locale}: the ${name} drawer is shut`).toBeGreaterThan(1);
        expect(one.width, `${locale}: the ${name} drawer is past its share`).toBeLessThanOrEqual(edge.width * limit + 0.5);
        expect(one.top).toBeCloseTo(edge.top, 1);
        expect(one.height, `${locale}: the ${name} drawer is not as tall as its side`).toBeCloseTo(edge.height, 1);
      }
      expect(start.left).toBeCloseTo(edge.left, 1);
      expect(end.right).toBeCloseTo(edge.right, 1);
      const work = await box(frame.locator(".nd-example-work"));
      expect(work.left).toBeCloseTo(start.right, 1);
      expect(work.right).toBeCloseTo(end.left, 1);
    }
  });
}

test("a drawer stands at the side it names wherever it is written", async ({ page }) => {
  await open(page);
  const frame = windowOf(page, "drawer-start");
  // Written last and first: the wrong way round for both.
  await frame.evaluate((one) => {
    one.append(one.querySelector('[data-nd-drawer="drawer-start"]'));
    one.prepend(one.querySelector('[data-nd-drawer="drawer-end"]'));
  });
  const edge = await inner(frame);
  expect((await box(drawer(page, "drawer-start"))).left, "a drawer at the start is not at the start").toBeCloseTo(edge.left, 1);
  expect((await box(drawer(page, "drawer-end"))).right, "a drawer at the end is not at the end").toBeCloseTo(edge.right, 1);

  // And a rail likewise, in the drawer it is in.
  const whole = drawer(page, "overview-drawer");
  await whole.locator(".nd-drawer-panel").evaluate((one) => {
    one.append(one.querySelector('.nd-drawer-rail[data-nd-side="start"]'));
  });
  const outer = await box(whole);
  expect((await box(whole.locator('.nd-drawer-rail[data-nd-side="start"]'))).left).toBeCloseTo(outer.left, 1);
  expect((await box(whole.locator('.nd-drawer-rail[data-nd-side="end"]'))).right).toBeCloseTo(outer.right, 1);
});

/** Every element Tab stops on, from the top of the page, until it comes round. */
async function tabStops(page, limit = 400) {
  await page.evaluate(() => document.activeElement?.blur?.());
  const seen = [];
  for (let at = 0; at < limit; at += 1) {
    await page.keyboard.press("Tab");
    const here = await page.evaluate(() => {
      const one = document.activeElement;
      if (!one || one === document.body) return null;
      return { inside: one.closest("[data-nd-drawer]")?.getAttribute("data-nd-drawer") ?? null };
    });
    if (here === null) break;
    seen.push(here);
  }
  return seen;
}

test("a collapsed drawer is nothing along its edge, and nothing in it can be reached", async ({ page }) => {
  await open(page);
  const shut = drawer(page, "drawer-shut");
  const frame = windowOf(page, "drawer-shut");
  expect((await box(shut)).height, "a collapsed drawer takes room").toBeLessThanOrEqual(0.5);
  expect(await shut.evaluate((one) => one.inert), "a collapsed drawer is not inert").toBe(true);
  await expect(shut.locator("button").first()).toBeHidden();
  // The work has the whole window while the drawer is shut.
  const edge = await inner(frame);
  expect((await box(frame.locator(".nd-example-work"))).height).toBeCloseTo(edge.height, 1);

  const stops = await tabStops(page);
  expect(stops.length, "the walk found no control at all").toBeGreaterThan(10);
  expect(stops.filter((one) => one.inside === "drawer-shut"), "Tab reached into a collapsed drawer").toEqual([]);
  // The walk is not blind to drawers: it went through the open ones.
  expect(stops.some((one) => one.inside === "overview-drawer"), "Tab reached no drawer at all").toBe(true);

  // THE CONTROL THAT TURNS IT NAMES IT, AND SAYS WHETHER IT IS OPEN, before
  // anything was pressed: a toggle that says nothing about a shut drawer is
  // a button a screen reader cannot tell from any other.
  const toggle = frame.locator('[data-nd-action="drawer-toggle"]').first();
  await expect(toggle).toHaveAttribute("aria-controls", "drawer-shut");
  await expect(shut).toHaveAttribute("id", "drawer-shut");
  await expect(toggle).toHaveAttribute("aria-expanded", "false");

  // OPENED, EVERYTHING IN IT IS BACK, and the focus did not move to bring it.
  await toggle.focus();
  await page.keyboard.press("Enter");
  await settled(shut);
  await expect(toggle, "the control did not say the drawer opened").toHaveAttribute("aria-expanded", "true");
  expect((await box(shut)).height, "the drawer did not open").toBeGreaterThan(10);
  expect(await shut.evaluate((one) => one.inert)).toBe(false);
  await expect(shut.locator("button").first()).toBeVisible();
  await expect(toggle, "opening a drawer took the keyboard").toBeFocused();
  expect((await tabStops(page)).some((one) => one.inside === "drawer-shut"), "an open drawer cannot be reached").toBe(true);
});

test("with the script blocked a collapsed drawer is still out of the tab order", async ({ browser }) => {
  const context = await browser.newContext({ javaScriptEnabled: false });
  const page = await context.newPage();
  await page.goto(url(PAGE.drawer));
  await showParts(page);
  const shut = drawer(page, "drawer-shut");
  // No script wrote `inert`, and the stylesheet alone has to answer.
  expect(await shut.getAttribute("inert")).toBeNull();
  expect((await box(shut)).height, "a collapsed drawer takes room").toBeLessThanOrEqual(0.5);
  await expect(shut.locator("button").first()).toBeHidden();

  // Asked of the control itself, as the Tab key would ask it.
  const reachable = await page.evaluate(() => {
    const one = document.querySelector('[data-nd-drawer="drawer-shut"] button');
    one.focus();
    return document.activeElement === one;
  });
  expect(reachable, "a control in a collapsed drawer took the focus").toBe(false);
  // And an open one is whole.
  await expect(drawer(page, "drawer-basic")).toBeVisible();
  await context.close();
});

test("Escape says so once, after a surface inside has had it, and the focus goes back", async ({ page }) => {
  await open(page);
  const frame = windowOf(page, "overview-drawer");
  const whole = drawer(page, "overview-drawer");
  await page.evaluate(() => {
    window.said = [];
    document.addEventListener("nd-drawer-dismiss", (event) => window.said.push(event.detail.name));
  });

  // FROM OUTSIDE, IN. The control in the modeline is where the reader was.
  const from = frame.locator('.nd-footer [data-nd-action="drawer-toggle"]');
  await from.focus();
  await whole.locator(".nd-drawer-rail button").nth(1).focus();
  // AND ON, INSIDE. A move from one control of the drawer to the next is not
  // an arrival, and the place to go back to is still the one outside.
  await page.keyboard.press("ArrowLeft");
  const inside = whole.locator(".nd-drawer-rail button").first();
  await expect(inside).toBeFocused();
  // The focus opened that control's hint, which is a surface of its own.
  const bubble = whole.locator(".nd-tooltip:has(button)").first().locator(".nd-tooltip-bubble");
  await expect(bubble).toBeVisible();

  // THE FIRST ESCAPE IS THE HINT'S. Nothing is said and the drawer stays.
  await page.keyboard.press("Escape");
  await expect(bubble).toBeHidden();
  expect(await page.evaluate(() => window.said), "Escape shut the drawer over an open hint").toEqual([]);
  expect(await whole.getAttribute("data-nd-collapsed")).toBe("false");

  // THE SECOND IS THE DRAWER'S, and it says so once, by name.
  await page.keyboard.press("Escape");
  expect(await page.evaluate(() => window.said)).toEqual(["overview-drawer"]);
  // The page is the caller here and shuts the drawer on that word. The focus
  // was inside, so it is given back to where it came in from.
  expect(await whole.getAttribute("data-nd-collapsed")).toBe("true");
  expect(await whole.evaluate((one) => one.inert)).toBe(true);
  await expect(from, "the focus was dropped when the drawer shut around it").toBeFocused();
  await settled(whole);
  expect((await box(whole)).height).toBeLessThanOrEqual(0.5);
});

test("an input method's Escape is not the reader's, and dismisses nothing", async ({ page }) => {
  await open(page);
  const whole = drawer(page, "overview-drawer");
  await page.evaluate(() => {
    window.said = [];
    document.addEventListener("nd-drawer-dismiss", (event) => window.said.push(event.detail.name));
  });
  // FROM THE CONTENT, where a reader types and no hint of a rail's control
  // is open to take the key first.
  const content = whole.locator(".nd-drawer-panel > :not(.nd-drawer-rail)").first();
  const send = (inits) =>
    content.evaluate((el, all) => {
      for (const init of all) {
        el.dispatchEvent(new KeyboardEvent("keydown", { ...init, bubbles: true, cancelable: true }));
      }
    }, inits);
  // AS AN INPUT METHOD SENDS IT: marked as composing, or with the code an
  // engine reports for a key the input method took. WebKit sends the key
  // that cancels a composition after `compositionend`, with that code.
  await send([{ key: "Escape", keyCode: 229 }, { key: "Escape", isComposing: true }]);
  expect(await page.evaluate(() => window.said), "an input method's Escape dismissed the drawer").toEqual([]);
  // The same key from the same place, the reader's own, is heard: the path
  // is live, and only the input method's key was turned away.
  await send([{ key: "Escape" }]);
  expect(await page.evaluate(() => window.said)).toEqual(["overview-drawer"]);
});

test("the modeline's control names the drawer it turns, and says whether it is open", async ({ page }) => {
  await open(page);
  const frame = windowOf(page, "overview-drawer");
  const whole = drawer(page, "overview-drawer");
  const toggle = frame.locator('.nd-footer [data-nd-action="drawer-toggle"]');
  await expect(toggle).toHaveAttribute("aria-controls", "overview-drawer");
  await expect(whole).toHaveAttribute("id", "overview-drawer");
  await expect(toggle).toHaveAttribute("aria-expanded", "true");
  await toggle.click();
  await expect(whole).toHaveAttribute("data-nd-collapsed", "true");
  await expect(toggle).toHaveAttribute("aria-expanded", "false");
  await toggle.click();
  await expect(toggle).toHaveAttribute("aria-expanded", "true");
});

test("a drawer that shuts with the focus elsewhere leaves the focus alone", async ({ page }) => {
  await open(page);
  const frame = windowOf(page, "overview-drawer");
  const whole = drawer(page, "overview-drawer");
  // In, and out again by the reader's own hand: the visit is over.
  await frame.locator('.nd-footer [data-nd-action="drawer-toggle"]').focus();
  await whole.locator(".nd-drawer-rail button").first().focus();
  const elsewhere = windowOf(page, "drawer-shut").locator('[data-nd-action="drawer-toggle"]').first();
  await elsewhere.focus();
  await whole.evaluate((one) => one.setAttribute("data-nd-collapsed", "true"));
  await settled(whole);
  await expect(elsewhere, "a drawer the reader had left took the focus back").toBeFocused();
});

test("it slides shut and open, and under reduced motion it does neither", async ({ page }) => {
  await open(page);
  const whole = drawer(page, "drawer-long");
  const turn = (value) => whole.evaluate((one, next) => one.setAttribute("data-nd-collapsed", next), value);
  /** How many transitions are running on the drawer. */
  const moving = () => whole.evaluate((one) => one.getAnimations().length);
  const resume = () => whole.evaluate((one) => one.getAnimations().forEach((each) => each.play()));
  const full = (await box(whole)).height;
  expect(full).toBeGreaterThan(10);

  const shutting = await turnPartway(whole, "true");
  expect(shutting.moving, "a drawer shuts with no movement").toBeGreaterThan(0);
  // PARTWAY SHUT IT IS PART OF ITS HEIGHT: neither all of it nor none.
  const closing = shutting.height;
  expect(closing, "a drawer shuts in one step").toBeGreaterThan(1);
  expect(closing, "a drawer shuts in one step").toBeLessThan(full - 1);
  // WHILE IT SLIDES IT IS STILL DRAWN, or it would slide shut empty, and
  // nothing in it can be reached, which is what `inert` is written for.
  expect(await whole.evaluate((one) => getComputedStyle(one).visibility)).toBe("visible");
  expect(await whole.evaluate((one) => one.inert)).toBe(true);
  await resume();
  await settled(whole);
  expect((await box(whole)).height).toBeLessThanOrEqual(0.5);
  expect(await whole.evaluate((one) => getComputedStyle(one).visibility)).toBe("hidden");

  const opened = await turnPartway(whole, "false");
  expect(opened.moving, "a drawer opens with no movement").toBeGreaterThan(0);
  const opening = opened.height;
  expect(opening, "a drawer opens in one step").toBeGreaterThan(1);
  expect(opening, "a drawer opens in one step").toBeLessThan(full - 1);
  // Drawn from the first frame of the way open, or it would slide open empty.
  expect(await whole.evaluate((one) => getComputedStyle(one).visibility)).toBe("visible");
  await resume();
  await settled(whole);
  expect((await box(whole)).height).toBeCloseTo(full, 1);

  // A READER WHO ASKED FOR NO MOVEMENT GETS A DRAWER THAT IS OPEN OR SHUT.
  await page.emulateMedia({ reducedMotion: "reduce" });
  await turn("true");
  expect(await moving(), "a drawer moves under reduced motion").toBe(0);
  expect((await box(whole)).height).toBeLessThanOrEqual(0.5);
  await turn("false");
  expect(await moving()).toBe(0);
  expect((await box(whole)).height).toBeCloseTo(full, 1);
});

test("the chosen tab of a rail at the end is marked on the edge that faces the content", async ({ page }) => {
  await open(page);
  const whole = drawer(page, "rail-tabs");
  await whole.scrollIntoViewIfNeeded();
  const chosen = whole.locator('.nd-drawer-rail[data-nd-side="end"] .nd-tab[aria-selected="true"]');
  const rest = whole.locator('.nd-drawer-rail[data-nd-side="end"] .nd-tab[aria-selected="false"]').first();
  const at = await box(chosen);
  const other = await box(rest);
  const shot = await page.screenshot();
  const middle = (one) => (one.top + one.bottom) / 2;
  // One pixel in from each edge of the tab, at half its height, where no
  // glyph is: the rule if there is one, and the tab's own ground if not.
  const facing = await pixelAt(page, shot, at.left + 0.5, middle(at));
  const away = await pixelAt(page, shot, at.right - 1.5, middle(at));
  const ground = await pixelAt(page, shot, at.left + 6, middle(at) - at.height / 2 + 2);
  expect(facing, "the chosen tab has no rule on the edge that faces the content").not.toEqual(ground);
  expect(away, "the chosen tab is ruled on the edge that faces away").toEqual(ground);
  // The rule is the chosen tab's alone.
  const plain = await pixelAt(page, shot, other.left + 0.5, middle(other));
  const plainGround = await pixelAt(page, shot, other.left + 6, middle(other) - other.height / 2 + 2);
  expect(plain, "a tab that is not chosen is ruled").toEqual(plainGround);
  // And the chosen tab stands on the page's surface, not on the rail's shade.
  expect(ground, "the chosen tab is the rail's own shade").not.toEqual(plainGround);
});

test("a rail taller than its drawer scrolls with no scrollbar, and the keyboard brings a control into view", async ({ page }) => {
  await open(page);
  const whole = drawer(page, "rail-tall");
  await whole.scrollIntoViewIfNeeded();
  const rail = whole.locator(".nd-drawer-rail");
  const size = await rail.evaluate((one) => ({
    all: one.scrollHeight,
    seen: one.clientHeight,
    inside: one.clientWidth,
    outside: one.offsetWidth,
    across: one.scrollWidth,
  }));
  expect(size.all, "the example's rail fits, so it shows nothing about scrolling").toBeGreaterThan(size.seen + 1);
  // NO SCROLLBAR TAKES ROOM FROM A RAIL, and nothing in it scrolls sideways.
  expect(size.inside, "a scrollbar took room in the rail").toBe(size.outside);
  expect(size.across).toBeLessThanOrEqual(size.inside);
  // The rail is still the drawer's height and a rail's width.
  const outer = await box(whole);
  const at = await box(rail);
  expect(at.height).toBeCloseTo(outer.height, 1);
  expect(at.width).toBeCloseTo(await token(page, "--noctua-shell-rail"), 1);

  // THE LAST CONTROL IS UNDER THE DRAWER'S EDGE UNTIL THE KEYBOARD GOES THERE.
  const last = rail.locator("button").last();
  expect((await box(last)).top, "the last control already shows").toBeGreaterThanOrEqual(at.bottom - 1);
  await rail.locator("button").first().focus();
  await page.keyboard.press("End");
  await expect(last).toBeFocused();
  // THE RAIL IS WHAT SCROLLED, and the drawer around it stayed where it was.
  expect(await rail.evaluate((one) => one.scrollTop), "the rail did not scroll").toBeGreaterThan(0);
  expect(await whole.locator(".nd-drawer-panel").evaluate((one) => one.scrollTop)).toBe(0);
  const shown = await box(last);
  expect(shown.top, "the focused control is above the rail").toBeGreaterThanOrEqual(at.top - 0.5);
  expect(shown.bottom, "the focused control is under the drawer's edge").toBeLessThanOrEqual(at.bottom + 0.5);
});

test("a drawer at a side is told from the work by its shade, and one at the bottom is the page's own", async ({ page }) => {
  await open(page);
  const frame = windowOf(page, "drawer-start");
  await frame.scrollIntoViewIfNeeded();
  const shot = await page.screenshot();
  // A few pixels in from a far corner of each box, where no letter is.
  const corner = async (locator) => {
    const at = await box(locator);
    return pixelAt(page, shot, at.right - 4, at.bottom - 4);
  };
  const work = await corner(frame.locator(".nd-example-work"));
  const start = await corner(drawer(page, "drawer-start"));
  const end = await corner(drawer(page, "drawer-end"));
  expect(start, "a drawer at the start is the work's own shade").not.toEqual(work);
  expect(end, "a drawer at the end is the work's own shade").not.toEqual(work);
  expect(start).toEqual(end);

  // THE RAILS ARE THE CORNERS' OWN SHADE, which is what makes a rail under a
  // corner one column, and the content between them is not.
  const whole = windowOf(page, "overview-drawer");
  await whole.scrollIntoViewIfNeeded();
  const third = await page.screenshot();
  const low = async (locator) => {
    const here = await box(locator);
    return pixelAt(page, third, here.left + 3, here.bottom - 3);
  };
  const cornerShade = await low(whole.locator('.nd-footer-corner[data-nd-side="start"]'));
  const railShade = await low(whole.locator('.nd-drawer-rail[data-nd-side="start"]'));
  const contentShade = await low(whole.locator(".nd-drawer-panel > :not(.nd-drawer-rail)"));
  expect(railShade, "a rail is not the shade of the corner above it").toEqual(cornerShade);
  expect(contentShade, "the content is the rail's shade").not.toEqual(railShade);

  const below = windowOf(page, "drawer-basic");
  await below.scrollIntoViewIfNeeded();
  const second = await page.screenshot();
  const at = await box(drawer(page, "drawer-basic"));
  const above = await box(below.locator(".nd-example-work"));
  expect(
    await pixelAt(page, second, at.right - 4, at.bottom - 4),
    "a drawer at the bottom is not the page's own colour",
  ).toEqual(await pixelAt(page, second, above.right - 4, above.bottom - 4));
});

test("an empty rail is still a rail wide", async ({ page }) => {
  // A `Toolbar` drawn as a rail is a rail wide by its own rule, so a rail
  // holding one is that wide whatever the rail says. With nothing in it the
  // rail's own width is all there is.
  await open(page);
  const rail = drawer(page, "rail-controls").locator(".nd-drawer-rail");
  await rail.evaluate((one) => one.replaceChildren());
  expect((await box(rail)).width, "an empty rail has no width").toBeCloseTo(await token(page, "--noctua-shell-rail"), 1);
});

test("nothing in a drawer hangs out of it while it slides", async ({ page }) => {
  await open(page);
  // A drawer at the bottom, with a rail and more content than fits, and one at
  // a side. Each is stopped partway shut and asked where its parts are.
  for (const name of ["drawer-long", "drawer-start"]) {
    const frame = windowOf(page, name);
    const one = drawer(page, name);
    const whole = await box(one);
    const part = await turnPartway(one, "true");
    const across = name === "drawer-start";
    const size = across ? "width" : "height";
    expect(part[size], `${name}: the drawer is not partway shut`).toBeLessThan(whole[size] - 4);
    expect(part[size], `${name}: the drawer is not partway shut`).toBeGreaterThan(4);
    // EVERY PART IS INSIDE THE DRAWER'S BOX AS IT IS NOW. A rail or a run of
    // content that kept its size would stand over the work, or push the
    // window's own edge out.
    const parts = await one.locator(".nd-drawer-panel > *").evaluateAll((all) =>
      all.map((each) => {
        const at = each.getBoundingClientRect();
        return { left: at.left, right: at.right, top: at.top, bottom: at.bottom };
      }),
    );
    expect(parts.length).toBeGreaterThan(0);
    for (const [at, each] of parts.entries()) {
      expect(each.top, `${name}: part ${at} hangs out above`).toBeGreaterThanOrEqual(part.top - 0.5);
      expect(each.bottom, `${name}: part ${at} hangs out below`).toBeLessThanOrEqual(part.bottom + 0.5);
      expect(each.left, `${name}: part ${at} hangs out before`).toBeGreaterThanOrEqual(part.left - 0.5);
      expect(each.right, `${name}: part ${at} hangs out after`).toBeLessThanOrEqual(part.right + 0.5);
    }
    const spill = await frame.evaluate((el) => ({
      down: el.scrollHeight - el.clientHeight,
      across: el.scrollWidth - el.clientWidth,
    }));
    expect(spill.down, `${name}: the window grew taller than itself`).toBeLessThanOrEqual(0);
    expect(spill.across, `${name}: the window grew wider than itself`).toBeLessThanOrEqual(0);
  }
});

test("releasing a drawer takes back what the script wrote", async ({ page }) => {
  await open(page);
  const shut = drawer(page, "drawer-shut");
  expect(await shut.evaluate((one) => one.inert)).toBe(true);
  await page.evaluate(async () => {
    const { releaseDrawers } = await import("../../assets/drawer.js");
    releaseDrawers(document);
  });
  // No script reads `collapsed` any more, so none may leave its mark behind.
  expect(await shut.evaluate((one) => one.inert), "a released drawer is still inert").toBe(false);
  expect(await shut.getAttribute("data-nd-enhanced")).toBeNull();
  // The stylesheet still answers for it.
  await expect(shut.locator("button").first()).toBeHidden();
  // And it says nothing on Escape.
  const open_ = drawer(page, "overview-drawer");
  await page.evaluate(() => {
    window.said = [];
    document.addEventListener("nd-drawer-dismiss", (event) => window.said.push(event.detail.name));
  });
  await open_.locator(".nd-tab").first().focus();
  await page.keyboard.press("Escape");
  await page.keyboard.press("Escape");
  expect(await page.evaluate(() => window.said), "a released drawer still answers Escape").toEqual([]);

  await page.evaluate(async () => {
    const { enhanceDrawers } = await import("../../assets/drawer.js");
    enhanceDrawers(document);
  });
  expect(await shut.evaluate((one) => one.inert)).toBe(true);
});
