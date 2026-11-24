// A second seat's state is drawn exactly like the first one's.
//
// `NOCTUA.md §9.7`: the system's own pointer and keyboard reach a state through
// `:hover` and `:focus-visible`, and every other seat's state arrives as an
// attribute, "styled exactly like it". `seat_twin` in `xtask/src/gates.rs`
// holds the half a source file can show — every rule on a pseudo-class is also
// a rule on its attribute. This file holds the half only a browser can: that
// the two rules DRAW THE SAME THING, on the built page, for every state rule
// the shipped stylesheet has and every example that reaches one.
//
// THE TARGETS ARE READ OUT OF THE STYLESHEET, NOT LISTED. Each test asks the
// page for the rules of `components.css` that name a state, takes the part of
// each selector up to the state — `.nd-switch` of `.nd-switch:hover >
// .nd-switch-track` — and looks for elements that match it inside an example's
// stage. A hand-written list of "the interactive components" would be a list
// that is one component short the day one is added.
//
// THE COMPARISON IS WHOLE-VIEWPORT AND EXACT. A hover may repaint a child, a
// sibling or the box around the control, so the control's own rectangle is the
// wrong thing to compare; and the two pictures are the same declarations
// reached two ways, so any tolerance at all would be room for a rule that is
// almost the same.

import { writeFileSync } from "node:fs";

import { expect, test } from "@playwright/test";

import { PAGE, showParts, url } from "./pages.js";
import { differing } from "./pixels.js";

// EVERY PICTURE IS RASTERISED WHOLE, AND WITHOUT THAT THIS FILE CANNOT BE
// EXACT. The engine otherwise redraws only the rectangle a state change
// dirtied, over what the tile already held, and the antialiased pixels on that
// rectangle's edge keep a level or two of the old picture — so a picture of a
// control depended on how many times it had changed state and on what else
// changed in the same frame, not only on the state it was in. Measured on the
// first button of `/button/`: eighteen pixels between its first native hover
// and its second. Entering and leaving the state once before photographing it
// was tried and is not enough: the hover sweep still failed on ten pages, by
// a handful of edge pixels each. Drawn from nothing every time, no picture has
// a history.
//
// The option is the fixture's own with one argument added, so the browser this
// file starts is otherwise the one `playwright.config.js` describes.
test.use({
  launchOptions: async ({ launchOptions }, use) =>
    use({ ...launchOptions, args: [...(launchOptions.args ?? []), "--disable-partial-raster"] }),
});

/** How many elements one selector is asked about on one page. */
const PER_SELECTOR = 2;

/**
 * Loads `seats.js` beside the running shell and installs the probe.
 *
 * The module's URL comes off the page's own `<script>`, as `release.spec.js`
 * takes the shell's: the site is served under a base path, and a second
 * spelling of it is a second thing that can be wrong.
 */
async function prepare(page) {
  await showParts(page);
  // EVERY TEXT BOX GETS ITS LAYER BEFORE ANYTHING IS PHOTOGRAPHED. The engine
  // draws a focused text box's caret on a layer of its own, which moves
  // everything painted after the box onto another — so focusing an input
  // redrew the borders of the controls BELOW it a level or two differently,
  // and a second seat's attribute, which puts no caret anywhere, did not.
  // Measured on `/field/`: twenty pixels, none of them on the input, and none
  // at all once the box is a layer in both pictures. `caret-animation: manual`
  // and a transparent caret were tried and change nothing: it is the caret
  // having a layer that matters, not whether it blinks or shows.
  await page.addStyleTag({
    content: 'input, textarea, [contenteditable="true"] { will-change: transform; }',
  });
  await page.evaluate(async () => {
    const tag = document.querySelector('script[type="module"][src*="shell.js"]');
    const seats = await import(new URL("seats.js", tag.src).href);

    // THE PAGE'S OWN HANDLERS ARE SILENCED, because this file is about what a
    // state is drawn as and not about what it sets off. A tooltip opens under
    // a pointer and a carousel pauses under one; a second seat's attribute
    // does neither, by design (`seats.js` says so), so the native half of each
    // comparison would carry a picture the other half never owed. The
    // pseudo-classes are the engine's and no listener can take them away.
    for (const type of [
      "pointerover", "pointerenter", "pointermove", "pointerout", "pointerleave",
      "mouseover", "mouseenter", "mousemove", "mouseout", "mouseleave",
      "focus", "focusin", "blur", "focusout",
    ]) {
      window.addEventListener(type, (event) => event.stopImmediatePropagation(), true);
    }

    /** A selector list, split at the commas that are not inside brackets. */
    const split = (text) => {
      const parts = [];
      let depth = 0;
      let from = 0;
      for (let at = 0; at < text.length; at += 1) {
        if ("([".includes(text[at])) depth += 1;
        if (")]".includes(text[at])) depth -= 1;
        if (text[at] === "," && depth === 0) {
          parts.push(text.slice(from, at));
          from = at + 1;
        }
      }
      parts.push(text.slice(from));
      return parts.map((part) => part.trim());
    };

    /** Every style rule of the shipped component sheet, queries opened. */
    const rules = () => {
      const sheet = [...document.styleSheets].find((s) => s.href?.endsWith("/components.css"));
      const out = [];
      const walk = (list) => {
        for (const rule of list) {
          if (rule.selectorText) out.push(rule.selectorText);
          if (rule.cssRules?.length) walk(rule.cssRules);
        }
      };
      walk(sheet.cssRules);
      return out;
    };

    const STATE = /:(?:hover|focus-visible|focus-within|focus|active)(?![-\w])/;

    /**
     * The selectors that reach a state, as `[state, target selector, whole]`.
     *
     * The twin form is folded back to the bare pseudo-class first, so a rule
     * reads the same here whichever of the two ways the stylesheet wrote it.
     */
    const stated = () => {
      const found = new Map();
      for (const list of rules()) {
        const bare = list
          .replace(/:is\(:focus-within, \[data-nd-seat-focus\], :has\(\[data-nd-seat-focus\]\)\)/g, ":focus-within")
          .replace(/:is\(:(hover|focus-visible|focus|active), \[data-nd-seat-\w+\]\)/g, ":$1");
        for (const selector of split(bare)) {
          const hit = STATE.exec(selector);
          if (!hit) continue;
          const before = selector.slice(0, hit.index);
          // A state inside `:has(` is a state on something BELOW what the
          // selector's own prefix matches, so the prefix is cut in two there.
          let depth = 0;
          let open = -1;
          for (let at = 0; at < before.length; at += 1) {
            if (before[at] === "(") {
              if (depth === 0) open = at;
              depth += 1;
            }
            if (before[at] === ")") depth -= 1;
          }
          let outer = before;
          let inner = null;
          if (depth > 0) {
            if (depth !== 1 || !before.slice(0, open).endsWith(":has")) continue;
            outer = before.slice(0, open - ":has".length);
            inner = `:scope ${before.slice(open + 1)}`;
          }
          if (!outer.trim() || /[>+~\s]$/.test(outer)) outer += "*";
          if (inner && /[>+~\s]$/.test(inner)) inner += "*";
          found.set(`${hit[0]}|${outer}|${inner}`, [hit[0].slice(1), outer, inner, selector]);
        }
      }
      return [...found.values()];
    };

    /** The elements a state selector is about, inside an example's stage. */
    const subjects = (outer, inner) => {
      const out = [];
      for (const element of document.querySelectorAll(outer)) {
        if (!element.closest(".nd-frame-stage")) continue;
        if (inner) out.push(...element.querySelectorAll(inner));
        else out.push(element);
      }
      return out;
    };

    const FOCUSABLE =
      'a[href], button, input, select, textarea, summary, [tabindex], [contenteditable="true"]';

    const chosen = [];
    window.__seatProbe = {
      seats,
      /** Every state selector the sheet has, for the sweep that counts them. */
      stated,
      subjects,
      /**
       * Marks up to `limit` elements per selector for the wanted states and
       * answers how many it marked. `ready` says whether one can be driven.
       */
      choose(states, limit, ready) {
        for (const [state, outer, inner, selector] of stated()) {
          if (!states.includes(state)) continue;
          let taken = 0;
          for (let subject of subjects(outer, inner)) {
            if (taken === limit) break;
            // `:focus-within` is reached from inside: the thing to focus is the
            // first control in the box, or the box itself where it is one.
            if (state === "focus-within" && !subject.matches(FOCUSABLE)) {
              subject = subject.querySelector(FOCUSABLE);
              if (!subject) continue;
            }
            if (chosen.some((entry) => entry.element === subject)) {
              taken += 1;
              continue;
            }
            if (!ready(subject)) continue;
            chosen.push({ element: subject, selector });
            taken += 1;
          }
        }
        return chosen.length;
      },
      at: (index) => chosen[index],
    };
  });
}

/** A point of the viewport with nothing under it that a pointer could wake. */
async function restingPoint(page) {
  const { width, height } = page.viewportSize();
  const point = await page.evaluate(
    ([w, h]) => {
      for (const [x, y] of [[w - 2, h - 2], [2, h - 2], [w - 2, h / 2], [2, h / 2]]) {
        const hit = document.elementFromPoint(x, y);
        if (hit && !hit.closest("a, button, input, label, summary, [role], [tabindex], .nd-frame")) {
          return [x, y];
        }
      }
      return null;
    },
    [width, height],
  );
  expect(point, "nowhere in the viewport to rest the pointer").not.toBeNull();
  return point;
}

/**
 * One picture of the viewport, with every transition finished and every tile
 * drawn.
 *
 * TAKEN UNTIL TWO IN A ROW AGREE. Rasterising whole (see the top of this file)
 * costs a moment the screenshot does not always wait for, and a tile that has
 * not arrived is photographed as blank ground: a stage came back empty on
 * `/label/` exactly so. Whatever is still arriving makes the next picture
 * differ from this one, and nothing else on a page at rest does.
 */
async function picture(page, options = {}) {
  let last = await page.screenshot({ animations: "disabled", ...options });
  for (let attempt = 0; attempt < 10; attempt += 1) {
    const next = await page.screenshot({ animations: "disabled", ...options });
    if (next.equals(last)) return next;
    last = next;
  }
  throw new Error("the page never drew the same picture twice in a row");
}

/**
 * The native state and the seat's, each photographed from the same rest.
 */
async function bothWays(page, { enter, leave, seat, unseat }) {
  await enter();
  const native = await picture(page);
  await leave();
  await seat();
  const seated = await picture(page);
  await unseat();
  return [native, seated];
}

/**
 * Holds two pictures to be one, and keeps both when they are not.
 *
 * A count of pixels says that something differs and nothing about what, so a
 * failure attaches the pair to the report: the next reader opens two images
 * instead of rebuilding the moment.
 */
async function expectSame(page, testInfo, native, seated, what) {
  const count = await differing(page, native, seated);
  if (count !== 0) {
    for (const [file, body] of [["native.png", native], ["seat.png", seated]]) {
      const path = testInfo.outputPath(file);
      writeFileSync(path, body);
      await testInfo.attach(file, { path, contentType: "image/png" });
    }
  }
  expect(count, what).toBe(0);
}

for (const [name, path] of Object.entries(PAGE)) {
  test(`a second seat's hover is drawn like the pointer's on ${name}`, async ({ page }, testInfo) => {
    test.slow();
    // No autoplay and no sweep: a page that moves by itself differs from
    // itself, and no comparison of two moments of it means anything.
    await page.emulateMedia({ reducedMotion: "reduce" });
    await page.goto(url(path));
    await prepare(page);

    const count = await page.evaluate(
      (limit) =>
        window.__seatProbe.choose(["hover"], limit, (element) =>
          element.checkVisibility({ visibilityProperty: true }),
        ),
      PER_SELECTOR,
    );

    let compared = 0;
    for (let index = 0; index < count; index += 1) {
      // Brought to the middle, and asked for a point the pointer can actually
      // reach it at: an element under another is not hovered by going there,
      // and one with no box of its own — a `Lineage` bond is a group of placed
      // parts — is hovered through a part. What is under that point is kept,
      // because that is the element a host would name for a second pointer.
      const point = await page.evaluate((at) => {
        const entry = window.__seatProbe.at(at);
        const { element } = entry;
        element.scrollIntoView({ block: "center", inline: "center", behavior: "instant" });
        for (const part of [element, ...[...element.querySelectorAll("*")].slice(0, 12)]) {
          const box = part.getBoundingClientRect();
          if (box.width === 0 || box.height === 0) continue;
          for (const [fx, fy] of [[0.5, 0.5], [0.25, 0.5], [0.75, 0.5], [0.5, 0.25], [0.5, 0.75]]) {
            const x = box.left + box.width * fx;
            const y = box.top + box.height * fy;
            const hit = document.elementFromPoint(x, y);
            if (hit && element.contains(hit)) {
              entry.under = hit;
              return [x, y];
            }
          }
        }
        return null;
      }, index);
      if (!point) continue;

      const rest = await restingPoint(page);
      await page.mouse.move(...point);
      const hovered = await page.evaluate(
        (at) => window.__seatProbe.at(at).element.matches(":hover"),
        index,
      );
      if (!hovered) continue;
      const label = await page.evaluate((at) => {
        const { element, selector } = window.__seatProbe.at(at);
        return `${selector}  on  ${element.outerHTML.slice(0, 120)}`;
      }, index);
      const [native, seated] = await bothWays(page, {
        enter: () => page.mouse.move(...point),
        leave: () => page.mouse.move(...rest),
        seat: () =>
          page.evaluate((at) => {
            const probe = window.__seatProbe;
            probe.seats.seatHover("second", probe.at(at).under);
          }, index),
        unseat: () => page.evaluate(() => window.__seatProbe.seats.seatHover("second", null)),
      });

      await expectSame(
        page,
        testInfo,
        native,
        seated,
        `pixels that differ between :hover and data-nd-seat-hover — ${label}`,
      );
      compared += 1;
    }

    // Every element chosen was visible, so one that could not be hovered at
    // all is rare; a page where none could is a sweep that looked at nothing.
    if (count > 0) expect(compared, "no chosen element could be hovered").toBeGreaterThan(0);
  });

  test(`a second seat's focus is drawn like the keyboard's on ${name}`, async ({ page }, testInfo) => {
    test.slow();
    await page.emulateMedia({ reducedMotion: "reduce" });
    await page.goto(url(path));
    await prepare(page);
    // A KEY FIRST, AND NO POINTER ANYWHERE IN THIS TEST. Whether a scripted
    // `focus()` matches `:focus-visible` depends on what the reader last used,
    // and after a key press it does — which is checked below, not assumed.
    await page.keyboard.press("Shift");

    const count = await page.evaluate(
      (limit) =>
        window.__seatProbe.choose(["focus-visible", "focus-within", "focus"], limit, (element) => {
          if (!element.checkVisibility()) return false;
          element.focus({ preventScroll: true });
          const taken = document.activeElement === element && element.matches(":focus-visible");
          element.blur();
          return taken;
        }),
      PER_SELECTOR,
    );

    for (let index = 0; index < count; index += 1) {
      const label = await page.evaluate((at) => {
        const { element, selector } = window.__seatProbe.at(at);
        element.scrollIntoView({ block: "center", inline: "center", behavior: "instant" });
        return `${selector}  on  ${element.outerHTML.slice(0, 120)}`;
      }, index);
      const [native, seated] = await bothWays(page, {
        enter: () =>
          page.evaluate(
            (at) => window.__seatProbe.at(at).element.focus({ preventScroll: true }),
            index,
          ),
        leave: () => page.evaluate((at) => window.__seatProbe.at(at).element.blur(), index),
        seat: () =>
          page.evaluate((at) => {
            const probe = window.__seatProbe;
            probe.seats.seatFocus("second", probe.at(at).element);
          }, index),
        unseat: () => page.evaluate(() => window.__seatProbe.seats.seatFocus("second", null)),
      });

      await expectSame(
        page,
        testInfo,
        native,
        seated,
        `pixels that differ between keyboard focus and data-nd-seat-focus — ${label}`,
      );
    }
  });
}

test("the sweeps above have something to compare", async ({ page }) => {
  // THE VACUITY GUARD. Each test above passes on a page with no state rule in
  // reach, which is right for a page of headings and would be just as green if
  // the probe had stopped finding anything anywhere. So the probe is asked,
  // across every page, how many of the sheet's state selectors it can reach —
  // and the answer has to be most of them, and has to include a change a
  // reader can see.
  test.slow();
  const reached = new Set();
  let all = 0;
  for (const path of Object.values(PAGE)) {
    await page.goto(url(path));
    await prepare(page);
    const here = await page.evaluate(() => {
      const probe = window.__seatProbe;
      const stated = probe.stated();
      return {
        all: stated.length,
        reached: stated
          .filter(([, outer, inner]) => probe.subjects(outer, inner).length > 0)
          .map(([state, outer, inner]) => `${state}|${outer}|${inner}`),
      };
    });
    all = here.all;
    for (const key of here.reached) reached.add(key);
  }
  expect(all, "the stylesheet has no state selector the probe can read").toBeGreaterThan(0);
  // Not all: a rule for a combination no example renders — a refused glass
  // button in the danger tone — is reached by nothing, and inventing an
  // example for each would be writing the sweep's answer for it.
  expect(reached.size / all, `${reached.size} of ${all} state selectors reached`).toBeGreaterThan(0.85);

  // And a hover is a change. Two pictures that agree because NEITHER moved
  // would pass every comparison in this file.
  await page.goto(url(PAGE.button));
  await prepare(page);
  const button = page.locator('.nd-frame-stage .nd-button[data-nd-variant="solid"]').first();
  await button.scrollIntoViewIfNeeded();
  const resting = await picture(page);
  await button.evaluate((element) => window.__seatProbe.seats.seatHover("second", element));
  expect(await differing(page, resting, await picture(page))).toBeGreaterThan(0);
});

test("two seats on two controls are both drawn, each as its own pointer would", async ({ page }) => {
  await page.emulateMedia({ reducedMotion: "reduce" });
  await page.goto(url(PAGE.button));
  await prepare(page);

  // Two buttons of one example, so both fit in one viewport.
  const pair = await page.evaluateHandle(() => {
    for (const stage of document.querySelectorAll(".nd-frame-stage")) {
      const found = [
        ...stage.querySelectorAll('.nd-button[data-nd-variant="solid"]:not([disabled], [aria-disabled="true"])'),
      ].filter((element) => element.checkVisibility());
      if (found.length > 1) return found.slice(0, 2);
    }
    return [];
  });
  const first = (await pair.evaluateHandle((found) => found[0])).asElement();
  const second = (await pair.evaluateHandle((found) => found[1])).asElement();
  expect(first && second, "no example with two solid buttons in it").toBeTruthy();
  await first.evaluate((element) => element.scrollIntoView({ block: "center" }));
  const boxes = [await first.boundingBox(), await second.boundingBox()];
  const clip = (box) => picture(page, { clip: box });
  const mark = (element) => element.getAttribute("data-nd-seat-hover");

  // What each looks like under the real pointer, one at a time — the pointer
  // can only ever be on one, which is the whole reason for the attribute.
  const rest = await restingPoint(page);
  await page.mouse.move(...rest);
  const resting = [await clip(boxes[0]), await clip(boxes[1])];
  const native = [];
  for (const box of boxes) {
    await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2);
    native.push(await clip(box));
  }
  await page.mouse.move(...rest);
  expect(await differing(page, resting[0], native[0]), "a hover that changes nothing").toBeGreaterThan(0);

  await first.evaluate((element) => window.__seatProbe.seats.seatHover("b", element));
  await second.evaluate((element) => window.__seatProbe.seats.seatHover("a", element));
  expect(await differing(page, native[0], await clip(boxes[0])), "seat b's control").toBe(0);
  expect(await differing(page, native[1], await clip(boxes[1])), "seat a's control").toBe(0);

  // Each control names its own seat, and what is above both names both, sorted.
  expect(await mark(first)).toBe("b");
  expect(await mark(second)).toBe("a");
  await expect(page.locator("body")).toHaveAttribute("data-nd-seat-hover", "a b");

  // One seat leaving takes its own mark and leaves the other's alone.
  await page.evaluate(() => window.__seatProbe.seats.seatHover("b", null));
  expect(await mark(first)).toBeNull();
  await expect(page.locator("body")).toHaveAttribute("data-nd-seat-hover", "a");
  expect(await differing(page, resting[0], await clip(boxes[0])), "seat b left").toBe(0);
  expect(await differing(page, native[1], await clip(boxes[1])), "seat a stayed").toBe(0);
});

test("a seat's state moves with it, and is stamped where the pseudo-class would match", async ({
  page,
}) => {
  await page.goto(url(PAGE.checkbox));
  await prepare(page);

  const state = await page.evaluate(() => {
    const { seats } = window.__seatProbe;
    const read = (element, name) => element.getAttribute(`data-nd-seat-${name}`);
    const out = {};

    // Hover climbs to the root, and a label under the pointer puts its control
    // under it as well: both are what `:hover` does.
    const label = document.querySelector(".nd-frame-stage label.nd-choice-label");
    const choice = label.parentElement;
    seats.seatHover("2", label);
    out.chain = [label, choice, document.body, document.documentElement].map((el) => read(el, "hover"));
    out.control = read(label.control, "hover");
    // And only the control: the box drawn beside it is not under the pointer.
    out.beside = read(choice.querySelector(".nd-choice-box"), "hover");

    // Moving takes the old chain with it, except what the two share.
    const other = [...document.querySelectorAll(".nd-frame-stage label.nd-choice-label")].at(-1);
    seats.seatHover("2", other);
    out.left = [read(label, "hover"), read(choice, "hover"), read(label.control, "hover")];
    out.shared = read(document.body, "hover");
    out.carriers = document.querySelectorAll("[data-nd-seat-hover]").length;

    // Focus is one element, as `:focus-visible` is: the boxes around it are
    // reached by `:has()`, never by a mark of their own.
    seats.seatFocus("2", label.control);
    out.focus = [read(label.control, "focus"), read(label, "focus"), read(document.body, "focus")];
    seats.seatPress("2", label);
    out.active = [read(label, "active"), read(choice, "active"), read(label.control, "active")];
    seats.seatDrag("2", label);
    out.drag = [read(label, "drag"), read(choice, "drag")];

    // A second seat sorts into the list, and a seat that leaves takes
    // everything of its own and nothing else.
    seats.seatFocus("10", label.control);
    seats.seatFocus("1", label.control);
    out.sorted = read(label.control, "focus");
    seats.clearSeat("2");
    out.cleared = [
      read(label.control, "focus"),
      document.querySelectorAll(
        '[data-nd-seat-hover], [data-nd-seat-active], [data-nd-seat-drag]',
      ).length,
    ];
    seats.clearSeat("1");
    seats.clearSeat("10");
    out.empty = document.querySelectorAll(
      "[data-nd-seat-hover], [data-nd-seat-focus], [data-nd-seat-active], [data-nd-seat-drag]",
    ).length;

    // An id with a space in it would be read back as two seats.
    out.refused = [];
    for (const id of ["a b", "", " "]) {
      try {
        seats.seatHover(id, label);
        out.refused.push("accepted");
      } catch (error) {
        out.refused.push(error.constructor.name);
      }
    }
    return out;
  });

  expect(state.chain).toEqual(["2", "2", "2", "2"]);
  expect(state.control).toBe("2");
  expect(state.beside).toBeNull();
  expect(state.left).toEqual([null, null, null]);
  expect(state.shared).toBe("2");
  expect(state.carriers).toBeGreaterThan(2);
  expect(state.focus).toEqual(["2", null, null]);
  expect(state.active).toEqual(["2", "2", "2"]);
  expect(state.drag).toEqual(["2", null]);
  expect(state.sorted).toBe("1 10 2");
  expect(state.cleared).toEqual(["1 10", 0]);
  expect(state.empty).toBe(0);
  expect(state.refused).toEqual(["TypeError", "TypeError", "TypeError"]);
});

test("a host with no script of its own moves a seat by dispatching an event", async ({ page }) => {
  await page.goto(url(PAGE.button));
  await prepare(page);

  const state = await page.evaluate(() => {
    const { seats } = window.__seatProbe;
    const button = document.querySelector(".nd-frame-stage .nd-button");
    const say = (target, detail) =>
      target.dispatchEvent(new CustomEvent("nd-seat", { bubbles: true, detail }));
    const read = (name) => button.getAttribute(`data-nd-seat-${name}`);
    const out = {};

    // Nothing listens until it is asked to.
    say(button, { seat: "7", state: "hover" });
    out.before = read("hover");

    // Installed twice, on the document and on a subtree, as a host that
    // enhances both does: one event is still one answer.
    seats.enhanceSeats(document);
    seats.enhanceSeats(button.parentElement);
    say(button, { seat: "7", state: "hover" });
    say(button, { seat: "7", state: "focus" });
    say(button, { seat: "7", state: "active" });
    say(button, { seat: "7", state: "drag" });
    out.on = ["hover", "focus", "active", "drag"].map(read);

    say(button, { seat: "7", state: "active", on: false });
    out.released = read("active");
    // On the document there is no element to be on.
    say(document, { seat: "7", state: "hover" });
    out.away = read("hover");
    // And a seat that left takes the rest.
    say(document, { seat: "7", on: false });
    out.gone = ["hover", "focus", "active", "drag"].map(read);

    seats.releaseSeats(document);
    seats.releaseSeats(button.parentElement);
    say(button, { seat: "7", state: "hover" });
    out.after = read("hover");
    return out;
  });

  expect(state.before).toBeNull();
  expect(state.on).toEqual(["7", "7", "7", "7"]);
  expect(state.released).toBeNull();
  expect(state.away).toBeNull();
  expect(state.gone).toEqual([null, null, null, null]);
  expect(state.after).toBeNull();
});
