// Every control the window's own components draw is a target a finger can hit.
//
// WHAT IS BEING GUARDED. `NOCTUA.md §9.8`: a hit area meets the minimum target
// size on every component, on every breakpoint, even where the glyph inside it
// is drawn smaller. `--noctua-target-min` is that floor (WCAG 2.5.8), and the
// components the desktop interface is built from (`NOCTUA.md §12`) are the ones
// a finger meets on a touch screen in a small window: the footer, the prompt,
// the rails, the drawer, the split's line, the key capture, the log, the lists
// a keyboard walks, the alert line and the confirmation. They each hold their
// own controls to the floor in their own spec. This asks it of all of them at
// once, on every page they are shown on, so a control that one spec never
// thought to name is measured too.
//
// `audit.spec.js` asks a looser question of every control on the site, at the
// 24-pixel floor written as a number. This one reads the token, so the floor
// moves with it, and it reaches the controls that are not a `<button>`: a
// focusable separator, a focusable region, an option.

import { expect, test } from "@playwright/test";

import { url, walker } from "./pages.js";

// Every walk in this file declares its tests through this line, so that a
// report names this file rather than `pages.js` (`walker` says why).
const walk = walker((title, fn) => test(title, fn));

/** The interface's components, by the root class each draws. */
const SHELL = [
  ".nd-match-text",
  ".nd-option-list",
  ".nd-footer",
  ".nd-prompt",
  ".nd-toolbar[data-nd-density='rail']",
  ".nd-tabs[data-nd-appearance='rail']",
  ".nd-drawer",
  ".nd-split",
  ".nd-key-capture",
  ".nd-log",
  ".nd-message-bar[data-nd-presentation='line']",
  ".nd-message-bar-action",
  ".nd-dialog-actions",
].join(", ");

/** Everything in them a pointer or a finger is asked to hit. */
const TARGETS = [
  "button",
  "a[href]",
  "input:not([type='hidden'])",
  "textarea",
  "select",
  "summary",
  "[role='option']",
  "[role='tab']",
  "[role='separator'][tabindex]",
  "[tabindex]:not([tabindex='-1'])",
].join(", ");

/** The narrowest window and a laptop's: a rail and a footer squeeze at the first. */
const WIDTHS = [320, 1120];

walk(
  "every control the interface's components draw clears the target floor",
  {
    axes: { width: WIDTHS },
    title: ({ width }) => `every control the interface's components draw clears the target floor at ${width}px`,
    // THE FLOOR IS THE WHOLE SITE'S. A page with none of these components
    // measures nothing, and a site where every page measured nothing is a
    // selector that stopped matching.
    tally: (counts) => {
      const measured = counts.reduce((sum, one) => sum + one, 0);
      expect(measured, "no control of the interface's components was measured").toBeGreaterThan(
        40,
      );
    },
  },
  async ({ page, width, name, path }) => {
    await page.setViewportSize({ width, height: 900 });
    await page.goto(url(path));
    const found = await page.evaluate(
      ([shell, targets]) => {
        // The floor, measured on a box rather than parsed off a string.
        const probe = document.createElement("div");
        probe.style.position = "absolute";
        probe.style.blockSize = "var(--noctua-target-min)";
        document.body.append(probe);
        const floor = probe.getBoundingClientRect().height;
        probe.remove();

        const small = [];
        let measured = 0;
        const seen = new Set();
        for (const root of document.querySelectorAll(shell)) {
          // A PICTURE OF A CONTROL IS NOT A CONTROL. Nobody is meant to press a
          // specimen in a grid of variants.
          if (root.closest(".nd-variant-stage")) continue;
          const inside = [...(root.matches(targets) ? [root] : []), ...root.querySelectorAll(targets)];
          for (const el of inside) {
            if (seen.has(el)) continue;
            seen.add(el);
            const style = getComputedStyle(el);
            // Not drawn, so not a target: a shut drawer's controls, a control a
            // script has not revealed, a panel that is shut.
            if (!el.checkVisibility({ visibilityProperty: true, opacityProperty: true })) continue;
            if (style.clipPath !== "none") continue;
            if (el.closest(".nd-visually-hidden")) continue;
            // An inline link inside a sentence is text, not a target: its
            // height is the line's and the line is the reader's own setting.
            if (style.display.startsWith("inline") && el.tagName === "A") continue;
            const box = el.getBoundingClientRect();
            if (box.width === 0 && box.height === 0) continue;
            measured += 1;
            if (box.width < floor - 0.5 || box.height < floor - 0.5) {
              const what = `${el.tagName.toLowerCase()}.${String(el.className).split(" ").filter(Boolean).slice(0, 2).join(".")}`;
              small.push(`${what} (${Math.round(box.width)}x${Math.round(box.height)} under ${floor})`);
            }
          }
        }
        return { small, measured };
      },
      [SHELL, TARGETS],
    );
    expect(found.small.map((one) => `${name}: ${one}`), `too small to hit at ${width}px`).toEqual(
      [],
    );
    return found.measured;
  },
);
