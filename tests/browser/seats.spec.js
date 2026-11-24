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
// WHAT IS SHUT IS OPENED FOR IT, BY THE SWEEP AND NOT BY THE COMPONENT. A menu's
// items, a dialog's close control and a colour picker's sliders are in the page
// and drawn nowhere until something opens the panel around them, and a sweep
// that only photographed what was already showing never looked at a quarter of
// the state rules. So an element that is not rendered has every `hidden` above
// it taken off and every `<dialog>` and `<details>` above it opened, for the
// two pictures and no longer. The component's own script is not asked to: what
// is compared here is how a state is DRAWN, and a panel standing where the
// script did not place it is drawn by the same rules in both pictures.
//
// AND WHAT WAS COMPARED IS COUNTED BY NAME. The test after the sweeps drives
// every page the way they do and holds the state selectors nothing reached to
// a list written out below, `UNSEEN`, one reason each. A selector the sweeps
// stop reaching fails there, and so does a listed one they start to reach.
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
          // The name the selector goes by in `UNSEEN` and in a failure: the
          // part before the state, then the state.
          const name = inner
            ? `${outer}:has(${inner.slice(":scope ".length)}${hit[0]})`
            : `${outer}${hit[0]}`;
          found.set(name, [hit[0].slice(1), outer, inner, selector, name]);
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

    // What a stylesheet keeps out of the page until its component has a reason
    // to show it: a control that clears a box is absent while the box is empty.
    // The part, the box it answers to, and the attribute that says there is
    // something to clear.
    const SHOWN_BY = [
      [".nd-search-clear", ".nd-search", "data-nd-has-query", ""],
      [".nd-select-clear", ".nd-select-field", "data-nd-filled", "true"],
    ];

    const rendered = (element) => element.checkVisibility({ visibilityProperty: true });

    /**
     * Opens what is shut around an element, and answers how to shut it again.
     *
     * Attributes only, and each put back as it was: nothing is focused, no
     * `close` event fires, and the page after the two pictures is the page
     * before them.
     */
    const reveal = (element) => {
      const undo = [];
      if (rendered(element)) return () => {};
      const set = (at, name, value) => {
        const was = at.getAttribute(name);
        if (was === value) return;
        if (value === null) at.removeAttribute(name);
        else at.setAttribute(name, value);
        undo.push(() => (was === null ? at.removeAttribute(name) : at.setAttribute(name, was)));
      };
      for (const [part, box, name, value] of SHOWN_BY) {
        const holder = element.matches(part) ? element.closest(box) : null;
        if (holder) set(holder, name, value);
      }
      for (let at = element; at && !at.matches(".nd-frame-stage"); at = at.parentElement) {
        if (at.hasAttribute("hidden")) set(at, "hidden", null);
        if (["dialog", "details"].includes(at.localName) && !at.hasAttribute("open")) {
          set(at, "open", "");
        }
      }
      return () => {
        while (undo.length > 0) undo.pop()();
      };
    };

    /**
     * Scrolls until an element is in the viewport, and answers whether it is.
     *
     * A state on an element outside the picture is a state nobody
     * photographed. And a floating panel is `position: fixed`, which no
     * scrolling moves: one opened here rather than by its component was never
     * placed against its control and stands wherever the flow left it, often
     * outside the viewport. Such a panel is pinned to the viewport's corner
     * instead, and `undo` is handed the way back.
     */
    const bring = (element, undo = []) => {
      const inView = () => {
        const box = element.getBoundingClientRect();
        return (
          box.width > 0 && box.height > 0 &&
          box.bottom > 0 && box.right > 0 && box.top < innerHeight && box.left < innerWidth
        );
      };
      element.scrollIntoView({ block: "center", inline: "center", behavior: "instant" });
      if (inView()) return true;
      for (let at = element; at && !at.matches(".nd-frame-stage"); at = at.parentElement) {
        if (getComputedStyle(at).position !== "fixed") continue;
        const was = at.getAttribute("style");
        at.style.inset = "0 auto auto 0";
        undo.push(() => (was === null ? at.removeAttribute("style") : at.setAttribute("style", was)));
        break;
      }
      return inView();
    };

    const chosen = [];
    window.__seatProbe = {
      seats,
      /** Every state selector the sheet has, for the sweep that counts them. */
      stated,
      subjects,
      /**
       * Marks up to `limit` elements per selector for the wanted states and
       * answers how many it marked. `ready` says whether one can be driven,
       * and is asked with whatever is shut around the element opened.
       */
      choose(states, limit, ready) {
        chosen.length = 0;
        for (const [state, outer, inner, selector, name] of stated()) {
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
            // One element stands for every selector it is a subject of: the
            // picture is of the whole viewport, so every rule on it is in it.
            const already = chosen.find((entry) => entry.element === subject);
            if (already) {
              already.names.add(name);
              taken += 1;
              continue;
            }
            const conceal = reveal(subject);
            const fit = ready(subject);
            conceal();
            if (!fit) continue;
            chosen.push({ element: subject, selector, names: new Set([name]) });
            taken += 1;
          }
        }
        return chosen.length;
      },
      at: (index) => chosen[index],
      /**
       * Opens what is shut around one of the chosen and brings it into the
       * viewport, until `hide`. `entry.seen` says whether it got there.
       */
      show(index) {
        const entry = chosen[index];
        entry.opened = !rendered(entry.element);
        const conceal = reveal(entry.element);
        const placed = [];
        entry.seen = bring(entry.element, placed);
        entry.conceal = () => {
          while (placed.length > 0) placed.pop()();
          conceal();
        };
        return entry;
      },
      hide(index) {
        chosen[index].conceal?.();
        chosen[index].conceal = null;
      },
      /**
       * Resolves once what `show` opened has finished arriving.
       *
       * OPENING IS NOT OVER WHEN THE ATTRIBUTE IS SET. A `<details>` fires its
       * `toggle` from a task of its own, and what answers it may ask for a
       * stylesheet: the frame's theme panel fetches one palette per accent
       * swatch the first time it opens. A sheet landing between the two
       * pictures repaints a swatch in one of them — measured on
       * `/example-frame/` under a loaded machine, one swatch and 120 pixels —
       * and two screenshots agreeing in a row says nothing about a request
       * still in flight.
       */
      async settle(index) {
        if (!chosen[index].opened) return;
        await new Promise((done) => {
          requestAnimationFrame(() => requestAnimationFrame(() => setTimeout(done)));
        });
        const loading = [...document.querySelectorAll('link[rel~="stylesheet"]')].filter(
          (link) => !link.sheet,
        );
        await Promise.all(
          loading.map(
            (link) =>
              new Promise((done) => {
                link.addEventListener("load", done, { once: true });
                link.addEventListener("error", done, { once: true });
              }),
          ),
        );
      },
      /** The selectors one of the chosen stands for. */
      names: (index) => [...chosen[index].names],
      rendered,
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

/**
 * Drives every hover the page's examples can show, and answers which state
 * selectors that reached.
 *
 * `compare` is handed each element with the pointer already on it and takes
 * the two pictures. The test after the sweeps passes none: it wants to know
 * what the sweep reaches, by the sweep's own steps, without the photographs.
 */
async function hoverSweep(page, compare = async () => {}) {
  const count = await page.evaluate(
    (limit) => window.__seatProbe.choose(["hover"], limit, window.__seatProbe.rendered),
    PER_SELECTOR,
  );

  const reached = new Set();
  for (let index = 0; index < count; index += 1) {
    // Brought to the middle, and asked for a point the pointer can actually
    // reach it at: an element under another is not hovered by going there,
    // and one with no box of its own — a `Lineage` bond is a group of placed
    // parts — is hovered through a part. What is under that point is kept,
    // because that is the element a host would name for a second pointer.
    const point = await page.evaluate(async (at) => {
      const entry = window.__seatProbe.show(at);
      await window.__seatProbe.settle(at);
      const { element } = entry;
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

    if (point) {
      const rest = await restingPoint(page);
      await page.mouse.move(...point);
      const hovered = await page.evaluate(
        (at) => window.__seatProbe.at(at).element.matches(":hover"),
        index,
      );
      if (hovered) {
        const label = await page.evaluate((at) => {
          const { element, selector } = window.__seatProbe.at(at);
          return `${selector}  on  ${element.outerHTML.slice(0, 120)}`;
        }, index);
        await compare({ index, point, rest, label });
        for (const name of await page.evaluate((at) => window.__seatProbe.names(at), index)) {
          reached.add(name);
        }
      }
      await page.mouse.move(...rest);
    }
    await page.evaluate((at) => window.__seatProbe.hide(at), index);
  }
  return { count, reached };
}

/**
 * The same for the keyboard: every focus the page's examples can show.
 *
 * A KEY FIRST, AND NO POINTER PRESSED ANYWHERE. Whether a scripted `focus()`
 * matches `:focus-visible` depends on what the reader last used, and after a
 * key press it does. That is not assumed: an element is chosen only if it took
 * the focus and matched, and the test after the sweeps fails if that stops
 * being true of a plain button.
 */
async function focusSweep(page, compare = async () => {}) {
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

  const reached = new Set();
  for (let index = 0; index < count; index += 1) {
    // In the picture, or not compared: two photographs of a viewport the
    // element is outside of agree whatever its focus is drawn as.
    const label = await page.evaluate(async (at) => {
      const { element, selector, seen } = window.__seatProbe.show(at);
      await window.__seatProbe.settle(at);
      if (!seen) return null;
      return `${selector}  on  ${element.outerHTML.slice(0, 120)}`;
    }, index);
    if (label !== null) {
      await compare({ index, label });
      for (const name of await page.evaluate((at) => window.__seatProbe.names(at), index)) {
        reached.add(name);
      }
    }
    await page.evaluate((at) => window.__seatProbe.hide(at), index);
  }
  return { count, reached };
}

for (const [name, path] of Object.entries(PAGE)) {
  test(`a second seat's hover is drawn like the pointer's on ${name}`, async ({ page }, testInfo) => {
    test.slow();
    // No autoplay and no sweep: a page that moves by itself differs from
    // itself, and no comparison of two moments of it means anything.
    await page.emulateMedia({ reducedMotion: "reduce" });
    await page.goto(url(path));
    await prepare(page);

    const { count, reached } = await hoverSweep(page, async ({ index, point, rest, label }) => {
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
    });

    // Every element chosen was rendered, so one that could not be hovered at
    // all is rare; a page where none could is a sweep that looked at nothing.
    if (count > 0) expect(reached.size, "no chosen element could be hovered").toBeGreaterThan(0);
  });

  test(`a second seat's focus is drawn like the keyboard's on ${name}`, async ({ page }, testInfo) => {
    test.slow();
    await page.emulateMedia({ reducedMotion: "reduce" });
    await page.goto(url(path));
    await prepare(page);

    const { count, reached } = await focusSweep(page, async ({ index, label }) => {
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
    });

    // Every element chosen took the focus, so one that could not be brought
    // into the picture is rare; a page where none could compared nothing.
    if (count > 0) expect(reached.size, "no chosen element was compared").toBeGreaterThan(0);
  });
}

/**
 * The state selectors of the shipped sheet that no sweep above compares.
 *
 * Written out, with the reason, so that "every state" is a claim with its
 * exceptions beside it. The test below holds the sweeps to exactly this list.
 *
 * ONE REASON COVERS ALL OF THEM TODAY: no example on any page in `pages.js`
 * renders an element the selector is about, so there is nothing to put a
 * pointer or a keyboard on. `seat_twin` still holds each of these rules to its
 * twin in the source; what is missing is the photograph. An entry leaves this
 * list when an example that shows it arrives, and the test says so.
 */
const UNSEEN = [
  // A combination of variant and tone no example asks for.
  '.nd-button[data-nd-variant="outline"][data-nd-tone="danger"]:hover',
  '.nd-button[data-nd-variant="outline"][data-nd-tone="success"]:hover',
  '.nd-button[data-nd-variant="subtle"][data-nd-tone="neutral"]:hover',
  '.nd-button[data-nd-variant="subtle"][data-nd-tone="success"]:hover',
  '.nd-button[data-nd-variant="transparent"][data-nd-tone="danger"]:hover',
  '.nd-button[data-nd-variant="transparent"][data-nd-tone="success"]:hover',
  // No example refuses a button with `aria-disabled`, or attaches a group.
  '.nd-button[aria-disabled="true"]:hover',
  '.nd-button-group[data-nd-attached="true"] > .nd-button:hover',
  '.nd-button-group[data-nd-attached="true"] > .nd-button:focus-visible',
  // A recent search exists once a reader has searched, and no page starts so.
  ".nd-search-recent-item:hover",
  // No example holds a skip link, or a `Lineage` card that is a link.
  '.nd-visually-hidden[data-nd-focusable="true"]:focus-within',
  "a.nd-lineage-card:hover",
  "a.nd-lineage-card:focus-visible",
];

test("the sweeps above compare every state selector but the ones listed", async ({ page }) => {
  // THE VACUITY GUARD. Each test above passes on a page with no state rule in
  // reach, which is right for a page of headings and would be just as green if
  // the probe had stopped finding anything anywhere, or found elements it then
  // could not drive. So every page is driven again here by the sweeps' own
  // steps, the photographs left out, and what those steps reached is held
  // against what the stylesheet has. An element being in the page is not
  // counted: only one the pointer was on, or the keyboard.
  test.setTimeout(600_000);
  await page.emulateMedia({ reducedMotion: "reduce" });
  const reached = new Set();
  let all = [];
  for (const path of Object.values(PAGE)) {
    await page.goto(url(path));
    await prepare(page);
    all = await page.evaluate(() => window.__seatProbe.stated().map((entry) => entry[4]));
    for (const sweep of [hoverSweep, focusSweep]) {
      for (const name of (await sweep(page)).reached) reached.add(name);
    }
  }
  expect(all.length, "the stylesheet has no state selector the probe can read").toBeGreaterThan(0);
  expect(reached.size, "the sweeps reached no state selector at all").toBeGreaterThan(0);
  const unseen = all.filter((name) => !reached.has(name)).sort();
  expect(unseen, "the state selectors no sweep compared").toEqual([...UNSEEN].sort());

  // And a hover is a change. Two pictures that agree because NEITHER moved
  // would pass every comparison in this file.
  await page.goto(url(PAGE.button));
  await prepare(page);
  const button = page.locator('.nd-frame-stage .nd-button[data-nd-variant="solid"]').first();
  await button.scrollIntoViewIfNeeded();
  const resting = await picture(page);
  await button.evaluate((element) => window.__seatProbe.seats.seatHover("second", element));
  expect(await differing(page, resting, await picture(page)), "a seat's hover draws nothing").toBeGreaterThan(0);
  await button.evaluate(() => window.__seatProbe.seats.seatHover("second", null));
  expect(await differing(page, resting, await picture(page)), "the seat left").toBe(0);

  // So is a focus, and the keyboard's is one the sweep can produce: after a
  // key, a scripted focus on a plain button is `:focus-visible`. If it were
  // not, the focus sweep would choose nothing and compare nothing.
  await page.keyboard.press("Shift");
  const ringed = await button.evaluate((element) => {
    element.focus({ preventScroll: true });
    return document.activeElement === element && element.matches(":focus-visible");
  });
  expect(ringed, "a scripted focus after a key press is not :focus-visible").toBe(true);
  const native = await picture(page);
  expect(await differing(page, resting, native), "a keyboard focus draws nothing").toBeGreaterThan(0);
  await button.evaluate((element) => {
    element.blur();
    window.__seatProbe.seats.seatFocus("second", element);
  });
  expect(await differing(page, native, await picture(page)), "a seat's focus is not the keyboard's").toBe(0);
});

test("two pictures of different shapes are never the same picture", async ({ page }) => {
  // `differing` is what every comparison above rests on. The same number of
  // pixels in another shape is another picture, whatever the bytes say.
  await page.goto(url(PAGE.button));
  const wide = await page.screenshot({ clip: { x: 0, y: 0, width: 40, height: 20 } });
  const tall = await page.screenshot({ clip: { x: 0, y: 0, width: 20, height: 40 } });
  expect(await differing(page, wide, tall)).toBe(Infinity);
  expect(await differing(page, wide, wide)).toBe(0);
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

    // A number is an id as it stands.
    seats.seatHover(3, label);
    out.number = read(label, "hover");
    seats.clearSeat(3);

    // An id with a space in it would be read back as two seats, and a seat
    // nobody named is not a seat called "undefined".
    out.refused = [];
    for (const id of ["a b", "", " ", undefined, null, Number.NaN, true, {}]) {
      try {
        seats.seatHover(id, label);
        out.refused.push("accepted");
      } catch (error) {
        out.refused.push(error.constructor.name);
      }
    }
    try {
      seats.clearSeat(undefined);
      out.refused.push("accepted");
    } catch (error) {
      out.refused.push(error.constructor.name);
    }
    out.stamped = document.querySelectorAll("[data-nd-seat-hover]").length;
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
  expect(state.number).toBe("3");
  expect(state.refused).toEqual(Array(9).fill("TypeError"));
  expect(state.stamped).toBe(0);
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
    // enhances both does.
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

    // A detail that names no seat, or no state there is, is refused out loud:
    // a listener's throw reaches the window as an `error` event. And one event
    // is one answer however many roots heard it — two roots are installed
    // here, and each bad event is still reported once.
    const errors = [];
    const note = (event) => {
      errors.push(event.error?.constructor.name ?? "not an error");
      event.preventDefault();
    };
    window.addEventListener("error", note);
    for (const detail of [
      { state: "hover" },
      { seat: null, state: "hover" },
      { on: false },
      { seat: "7" },
      { seat: "7", state: "hovering" },
      null,
    ]) {
      say(button, detail);
    }
    window.removeEventListener("error", note);
    out.errors = errors;
    out.guessed = document.querySelectorAll(
      "[data-nd-seat-hover], [data-nd-seat-focus], [data-nd-seat-active], [data-nd-seat-drag]",
    ).length;

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
  expect(state.errors).toEqual(Array(6).fill("TypeError"));
  expect(state.guessed).toBe(0);
  expect(state.after).toBeNull();
});
