// The radio group's keyboard contract, which is what the role promises, and
// the geometry a row of stops owes a reader.
//
// The settings panel had twelve of these written as raw markup, and they were
// forty-two tab stops: every option was focusable, so a reader crossing the
// panel with Tab pressed it forty-two times. A radio group is ONE tab stop and
// the arrows move within it — not a nicety, but what the markup claims the
// moment it says `role="radiogroup"`.
//
// THE SECOND HALF OF THIS FILE MEASURES BOXES AND READS PIXELS, and it is here
// because five layout defects shipped at once under a suite this size. A
// horizontal group drawn as a vertical stack, a vertical group with one rounded
// corner at each end diagonally opposite, an `equal` group whose first stop was
// a pixel narrower than the rest, a bevelled group notched apart at every
// shared edge, and a joined group that ran off a 320px viewport. Every one of
// them was visible in a screenshot and invisible to a test that asked the
// stylesheet what it said.

import { test, expect } from "@playwright/test";
import { ANY, PAGE, modeControl, showParts, url } from "./pages.js";
import { pixelAt } from "./pixels.js";

const openSettings = (page) =>
  page.evaluate(() => {
    document.querySelector(".nd-settings").open = true;
  });

test("a group is one tab stop, not one per option", async ({ page }) => {
  await page.goto(url(ANY));
  await openSettings(page);

  const stops = await page.evaluate(() =>
    [...document.querySelectorAll('[role="radiogroup"]')].map((group) => ({
      name: group.dataset.ndGroup,
      focusable: [...group.querySelectorAll('[role="radio"]')].filter(
        (option) => option.tabIndex >= 0,
      ).length,
      options: group.querySelectorAll('[role="radio"]').length,
    })),
  );

  expect(stops.length, "there are no radiogroups on the page").toBeGreaterThan(5);
  for (const group of stops) {
    expect(group.options, `${group.name} has no options`).toBeGreaterThan(1);
    expect(group.focusable, `${group.name} is ${group.focusable} tab stops`).toBe(1);
  }
});

test("every option announces a state, chosen or not", async ({ page }) => {
  await page.goto(url(ANY));
  await openSettings(page);
  // Unlike `aria-pressed` on a button, absence here is not "not pressed" — it
  // is "state unknown", and a group of those announces nothing chosen.
  const missing = await page.evaluate(() =>
    [...document.querySelectorAll('[role="radio"]')]
      .filter((option) => !option.hasAttribute("aria-checked"))
      .map((option) => option.dataset.ndValue ?? "(unnamed)"),
  );
  expect(missing, "a radio with no aria-checked has no state").toEqual([]);

  const groups = await page.evaluate(() =>
    [...document.querySelectorAll('[role="radiogroup"]')].map((group) => ({
      name: group.dataset.ndGroup,
      chosen: [...group.querySelectorAll('[role="radio"]')].filter(
        (option) => option.getAttribute("aria-checked") === "true",
      ).length,
    })),
  );
  for (const group of groups) {
    expect(group.chosen, `${group.name} announces ${group.chosen} chosen`).toBe(1);
  }
});

test("arrows move within a group and choose as they go", async ({ page }) => {
  await page.goto(url(ANY));
  const group = await modeControl(page);
  const chosen = () =>
    group.evaluate(
      (el) => el.querySelector('[aria-checked="true"]')?.getAttribute("data-nd-value"),
    );

  await group.locator('[data-nd-value="light"]').click();
  expect(await chosen()).toBe("light");

  // A radio group moves AND selects — it does not merely move focus, which is
  // what a toolbar does.
  await page.keyboard.press("ArrowRight");
  expect(await chosen()).toBe("dark");
  await page.keyboard.press("ArrowRight");
  expect(await chosen()).toBe("system");
  await page.keyboard.press("ArrowLeft");
  expect(await chosen()).toBe("dark");
});

test("a group has a name, visible or not", async ({ page }) => {
  await page.goto(url(ANY));
  await openSettings(page);
  const nameless = await page.evaluate(() =>
    [...document.querySelectorAll('[role="radiogroup"]')]
      .filter((group) => {
        const label = group.getAttribute("aria-label")?.trim();
        const by = group.getAttribute("aria-labelledby");
        return !label && !(by && document.getElementById(by));
      })
      .map((group) => group.dataset.ndGroup ?? "(unnamed)"),
  );
  // A radiogroup with neither announces its role and nothing else.
  expect(nameless, "a radiogroup with no accessible name").toEqual([]);
});

// --- One stop, answered or not -----------------------------------------------

test("a group with nothing chosen is still one tab stop, with no script", async ({ browser }) => {
  // THE GUARANTEE, MEASURED WHERE IT USED TO FAIL. The stop was rendered on
  // the chosen item — `0` when `selected` and `-1` otherwise — which is one
  // stop for a group that has an answer and ZERO for one that does not. A
  // `Segmented` nothing had chosen was unreachable by keyboard, and the first
  // `SegmentedItem` example on `/segmented/` was exactly that shape. An item
  // cannot fix it, because an item cannot see its siblings, so the group holds
  // the stop.
  const context = await browser.newContext({ javaScriptEnabled: false });
  const page = await context.newPage();

  let unanswered = 0;
  // THE SECOND PAGE IS WHERE THE CASE LIVES. `/theme-scope/`'s scope controls
  // are three groups a reader has not answered — the state a scope is in comes
  // from above it until they choose — so it is the page that would have shown
  // zero tab stops.
  for (const target of [PAGE.segmented, PAGE.themeScope]) {
    await page.goto(url(target));
    await showParts(page);
    const groups = await page.evaluate(() =>
      // NOT THE VARIANT GRID. A specimen is raw markup drawn to be looked at
      // rather than a component, and its buttons are focusable because a
      // `<button>` is — which is a question about specimens, not about this.
      [...document.querySelectorAll('[role="radiogroup"]')]
        .filter((group) => !group.closest(".nd-variant-stage"))
        .map((group) => ({
          name: group.dataset.ndGroup,
          chosen: group.querySelectorAll('[role="radio"][aria-checked="true"]').length,
          // Every element in or at the group that a keyboard can land on.
          stops: [group, ...group.querySelectorAll("*")].filter((el) => el.tabIndex === 0).length,
        })),
    );
    expect(groups.length, `no radiogroup on ${target}`).toBeGreaterThan(0);
    for (const group of groups) {
      expect(group.stops, `${group.name} is ${group.stops} tab stops`).toBe(1);
    }
    unanswered += groups.filter((group) => group.chosen === 0).length;
  }
  // And the pages really do carry the case this is about, or the count
  // above proves only that a group with an answer has one stop.
  expect(unanswered, "no group on either page has nothing chosen").toBeGreaterThan(0);
  await context.close();
});

test("the script moves the stop onto a radio and leaves the group alone", async ({ page }) => {
  await page.goto(url(PAGE.segmented));
  await showParts(page);
  const groups = await page.evaluate(() =>
    // A GROUP INSIDE A TOOLBAR DOES NOT OWN ITS STOPS: the bar's list already
    // holds these radios, so the bar places one stop across the whole of it
    // and a group within it may legitimately hold none.
    [...document.querySelectorAll('[role="radiogroup"]')]
      .filter((group) => !group.closest('[role="toolbar"], .nd-variant-stage'))
      .map((group) => ({
        name: group.dataset.ndGroup,
        // The markup's fallback stop is the script's to take away: a group
        // that kept it would be a second stop, and inside a toolbar it would
        // join the bar's own walk through `[tabindex]`.
        onGroup: group.hasAttribute("tabindex"),
        onRadios: [...group.querySelectorAll('[role="radio"]')].filter((el) => el.tabIndex === 0)
          .length,
      })),
  );
  for (const group of groups) {
    expect(group.onGroup, `${group.name} kept the markup's own stop`).toBe(false);
    expect(group.onRadios, `${group.name} has ${group.onRadios} stops`).toBe(1);
  }
});

// --- The geometry, measured ---------------------------------------------------

/** Every stop's box, relative to the group's own, so scrolling cannot move it. */
const stops = (group) =>
  group.evaluate((el) => {
    const g = el.getBoundingClientRect();
    const round = (n) => Math.round(n * 100) / 100;
    return [...el.querySelectorAll(":scope > .nd-segmented-item")].map((item) => {
      const b = item.getBoundingClientRect();
      return {
        value: item.dataset.ndValue,
        x: round(b.x - g.x),
        y: round(b.y - g.y),
        width: round(b.width),
        height: round(b.height),
      };
    });
  });

test("choosing a stop moves nothing, including the stop chosen", async ({ page }) => {
  // THE ONE INVARIANT A SEGMENTED CONTROL CANNOT TRADE AWAY. A chosen stop that
  // gains a border, a weight or a pixel of padding pushes every stop after it
  // along, so the row twitches under the pointer that just chose. Measured from
  // the boxes rather than from the rule that paints them, because the paint is
  // allowed to change and the geometry is not.
  await page.goto(url(PAGE.segmented));
  await showParts(page);

  const group = page.locator('.nd-segmented[data-nd-group="example-mode"]');
  await expect(group).toBeVisible();
  const before = await stops(group);
  expect(before.length, "the mode example has no stops").toBeGreaterThan(1);

  for (const stop of before) {
    const item = group.locator(`[data-nd-value="${stop.value}"]`);
    await item.click();
    await expect(item).toHaveAttribute("aria-checked", "true");
    expect(await stops(group), `choosing ${stop.value} moved the row`).toEqual(before);
  }
});

test("equal and stretch give every stop one box on one line", async ({ page }) => {
  // `repeat(auto-fit, minmax(…, 1fr))` counts tracks by how many FIT, and a
  // group sized by its own content has no definite space to fit them in: the
  // overview at the head of this page drew three stops as three rows, and the
  // `fill` example drew two on one row and one on the next. One track per item
  // is the claim, and a row of equal boxes is what it looks like.
  await page.goto(url(PAGE.segmented));
  await showParts(page);

  const measured = await page.evaluate(() => {
    const round = (n) => Math.round(n * 100) / 100;
    return [...document.querySelectorAll('.nd-segmented[data-nd-fill="equal"], .nd-segmented[data-nd-fill="stretch"]')]
      .filter((g) => g.getBoundingClientRect().width > 0)
      .map((g) => ({
        name: g.dataset.ndGroup,
        fill: g.dataset.ndFill,
        vertical: g.getAttribute("aria-orientation") === "vertical",
        stops: [...g.querySelectorAll(":scope > .nd-segmented-item")].map((item) => {
          const b = item.getBoundingClientRect();
          return { width: round(b.width), height: round(b.height), top: Math.round(b.top), left: Math.round(b.left) };
        }),
      }));
  });

  expect(measured.length, "no filled group on the page").toBeGreaterThan(1);
  for (const group of measured) {
    const sizes = new Set(group.stops.map((s) => (group.vertical ? s.height : s.width)));
    expect([...sizes], `${group.name} (${group.fill}) has stops of different sizes`).toHaveLength(1);
    // One line, and the axis says which. Three stops sharing a top edge are a
    // row; three sharing a left edge are a column.
    const lines = new Set(group.stops.map((s) => (group.vertical ? s.left : s.top)));
    expect([...lines], `${group.name} (${group.fill}) is on ${lines.size} lines`).toHaveLength(1);
  }
});

test("a joined group takes the container rather than the page, at 320", async ({ browser }) => {
  // AT 320px IN pt-BR THE FIRST EXAMPLE RAN OFF THE SIDE. A joined group may
  // not wrap into two rows and may not be squeezed by a toolbar, and the two
  // rules together said it may not fit either — so "Acompanhar o sistema" sat
  // half on screen. A label takes a second line inside its own box now, which
  // is a different question from whether the row of boxes wraps.
  for (const locale of ["en", "pt-BR"]) {
    const context = await browser.newContext({ viewport: { width: 320, height: 900 } });
    const page = await context.newPage();
    await page.goto(url(PAGE.segmented, locale));
    await showParts(page);
    // EVERY GROUP A READER CAN REACH, WHICH INCLUDES THE SETTINGS PANEL'S. A
    // shut `<details>` is `content-visibility: hidden` and its contents lay out
    // against a containing block of zero, so measuring one shut reports a
    // number no reader will ever see. Opening it is what makes the twelve
    // groups in there part of this claim rather than noise in it.
    await openSettings(page);

    const measured = await page.evaluate(() => {
      // A CONTAINER OF ZERO IS A CONTAINER THAT WAS NEVER LAID OUT. The theme
      // menu is rendered twice — the header's panel and the drawer's — and the
      // one still shut is inside `content-visibility: hidden`, where every box
      // answers zero. Both copies are the same markup, so the count below is
      // what keeps this from quietly measuring nothing.
      const groups = [...document.querySelectorAll('.nd-segmented[data-nd-fill="hug"]')].filter(
        (group) => group.parentElement.getBoundingClientRect().width > 0,
      );
      return {
        seen: groups.length,
        outside: groups
          .filter((group) => {
            const box = group.getBoundingClientRect();
            const held = group.parentElement.getBoundingClientRect();
            return box.right > held.right + 1 || box.left < held.left - 1;
          })
          .map((group) => {
            const box = group.getBoundingClientRect();
            return `${group.dataset.ndGroup ?? "(specimen)"} ends ${Math.round(
              box.right - group.parentElement.getBoundingClientRect().right,
            )} past its container`;
          }),
      };
    });
    expect(measured.seen, `almost no group was measured in ${locale}`).toBeGreaterThan(20);
    expect(measured.outside, `a group wider than what holds it in ${locale}`).toEqual([]);

    // AND THE PAGE ITSELF NEVER SCROLLS SIDEWAYS, which is the reader-facing
    // half: a control that overflows into a scroller is reachable, and one
    // that pushes the document is a page that slides under the thumb.
    const document_ = await page.evaluate(() => ({
      scroll: document.documentElement.scrollWidth,
      client: document.documentElement.clientWidth,
    }));
    expect(document_.scroll, `the page scrolls sideways in ${locale}`).toBeLessThanOrEqual(
      document_.client + 1,
    );
    await context.close();
  }
});

/**
 * One pixel of a shot taken around an element, in the shot's own coordinates.
 *
 * The margin is what makes the corners readable: an element screenshot is the
 * element and nothing else, so there is no backdrop in it to compare a cut
 * corner against.
 */
async function around(page, locator, margin = 6) {
  const box = await locator.boundingBox();
  const shot = await page.screenshot({
    clip: {
      x: box.x - margin,
      y: box.y - margin,
      width: box.width + margin * 2,
      height: box.height + margin * 2,
    },
  });
  return { shot, box, margin };
}

const same = (a, b) => a.every((channel, i) => Math.abs(channel - b[i]) <= 4);

/**
 * Whether a hairline is drawn at a point on the group's outer edge.
 *
 * A border is one pixel and the box it belongs to lands on a fraction, so the
 * ink can sit on either of two rows at partial alpha. Three rows across the
 * edge and the strongest of them is the answer, which is what makes this a
 * measurement rather than a coin toss.
 */
async function drawn(page, shot, x, y, vertical, backdrop) {
  for (const step of [0, 1, -1]) {
    const pixel = vertical
      ? await pixelAt(page, shot, x + step, y)
      : await pixelAt(page, shot, x, y + step);
    if (!same(pixel, backdrop)) return true;
  }
  return false;
}

test("a joined group is cut at its four corners and nowhere between", async ({ page }) => {
  // THE DEFECT THIS IS NAMED AFTER. The corner rules for the horizontal axis
  // were not restricted to it, so a vertical group ran both sets and kept one
  // rounded corner at the top left and one at the bottom right, diagonally
  // opposite, with fourteen square ones between them. `ButtonGroup` has the
  // same shape of defect on record.
  //
  // Read from pixels: a corner that is cut shows the page through it, and a
  // join between two stops does not. Asking for `borderStartEndRadius` would
  // have reported the rule that lost on specificity as though it had won.
  await page.goto(url(PAGE.segmented));
  await showParts(page);

  const groups = page.locator(
    '.nd-segmented[data-nd-shape="rounded"]:not([data-nd-appearance="spread"])',
  );
  const count = await groups.count();
  expect(count, "no rounded joined group on the page").toBeGreaterThan(2);

  let vertical = 0;
  for (let index = 0; index < count; index += 1) {
    const group = groups.nth(index);
    if (!(await group.isVisible())) continue;
    const items = group.locator(":scope > .nd-segmented-item");
    if ((await items.count()) < 2) continue;
    await group.scrollIntoViewIfNeeded();

    const { shot, box, margin } = await around(page, group);
    const name = (await group.getAttribute("data-nd-group")) ?? "(specimen)";
    const isVertical = (await group.getAttribute("aria-orientation")) === "vertical";
    if (isVertical) vertical += 1;

    const page_ = await pixelAt(page, shot, 1, 1);
    const corners = [
      ["top left", margin, margin],
      ["top right", margin + box.width - 1, margin],
      ["bottom right", margin + box.width - 1, margin + box.height - 1],
      ["bottom left", margin, margin + box.height - 1],
    ];
    for (const [where, x, y] of corners) {
      const pixel = await pixelAt(page, shot, x, y);
      expect(same(pixel, page_), `${name}: the ${where} corner is not cut`).toBe(true);
    }

    // And the outer edge runs unbroken PAST the join between two stops, which
    // is what "the ones between take no radius" looks like. A rounded inner
    // corner cuts the edge away either side of the join and the group reads as
    // loose boxes rather than one control.
    const first = await items.nth(0).boundingBox();
    const join = isVertical
      ? first.y + first.height - box.y
      : first.x + first.width - box.x;
    for (const step of [-3, 3]) {
      const [x, y] = isVertical
        ? [margin, margin + join + step]
        : [margin + join + step, margin];
      expect(
        await drawn(page, shot, x, y, isVertical, page_),
        `${name}: the edge is cut ${Math.abs(step)}px from a join`,
      ).toBe(true);
    }
  }
  expect(vertical, "no vertical group was measured").toBeGreaterThan(0);
});

test("a square group is not cut at its corners", async ({ page }) => {
  // The other side of the same claim, and the reason the corners come from one
  // named radius: `square` and `bevel` set it to nothing, and a test that only
  // ever asserts "cut" passes on a control that lost its shape prop entirely.
  await page.goto(url(PAGE.segmented));
  await showParts(page);

  const group = page
    .locator('.nd-segmented[data-nd-shape="square"]:not([data-nd-appearance="spread"])')
    .first();
  await group.scrollIntoViewIfNeeded();
  const { shot, box, margin } = await around(page, group);
  const page_ = await pixelAt(page, shot, 1, 1);
  for (const [where, x, y] of [
    ["top left", margin, margin],
    ["bottom right", margin + box.width - 1, margin + box.height - 1],
  ]) {
    const pixel = await pixelAt(page, shot, x, y);
    expect(same(pixel, page_), `a square group's ${where} corner is cut`).toBe(false);
  }
});
