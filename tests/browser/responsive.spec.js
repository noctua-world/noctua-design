// Nothing on this site scrolls sideways, in any language, at any width.
//
// WHY THIS IS A TEST AND NOT A LOOK.
//
// A horizontal scrollbar is the one layout failure that is invisible to the
// person who wrote the page and unmissable to everyone else: it appears at a
// width the author does not use, or in a language the author does not read, and
// it makes every line on the page require two gestures instead of one.
//
// `docs-site/README.md` says to open `/qps/` at 320px "as well: anything that
// overflows only fitted because English is short". This is that sentence as a
// check that runs. It found the compound button's second line, which
// inherited `white-space: nowrap` from the button around it and was 410px of
// pseudo-locale text inside a 199px control.
//
// 320px is the floor `spec/tokens.toml` names — "the narrowest screen still in
// real use", and not a breakpoint because it is the default. The other three
// are the breakpoints either side of the sidebar docking.

import { test, expect } from "@playwright/test";
import { LOCALES, showParts, url, walker } from "./pages.js";

// Every walk in this file declares its tests through this line, so that a
// report names this file rather than `pages.js` (`walker` says why).
const walk = walker((title, fn) => test(title, fn));

// 1024 IS NOT DECORATION. It is the first width where the sidebar is docked
// AND the column beside it is tight, and skipping it is how three pages came to
// scroll sideways on every laptop without anybody noticing.
const WIDTHS = [320, 390, 768, 1024, 1440];

walk(
  "no page scrolls sideways",
  {
    axes: { locale: LOCALES, width: WIDTHS },
    title: ({ locale, width }) => `${locale} does not scroll sideways at ${width}px`,
  },
  async ({ page, locale, width, path }) => {
    await page.setViewportSize({ width, height: 900 });
    await page.goto(url(path, locale));

    const overflow = await page.evaluate(() => {
      const root = document.documentElement;
      if (root.scrollWidth <= root.clientWidth + 1) return null;
      // Name the element that did it, not just the fact. A failure that says
      // "the page is 69px too wide" costs an afternoon; one that says
      // `.nd-button-detail` costs a minute.
      const worst = [...document.querySelectorAll(".nd-main *")]
        .map((el) => {
          const box = el.getBoundingClientRect();
          const parent = el.parentElement.getBoundingClientRect();
          return { name: String(el.className), by: Math.round(box.width - parent.width) };
        })
        .filter((row) => row.by > 2)
        .sort((a, b) => b.by - a.by)[0];
      return { page: root.scrollWidth - root.clientWidth, worst };
    });

    expect(overflow, "something is wider than the page").toBeNull();
  },
);

// --- And in a wider face ------------------------------------------------------
//
// EVERY WIDTH ABOVE IS MEASURED IN NOTO SANS, because every page preloads it.
// A reader whose preload loses the race to the first paint is drawn in what
// their machine falls back to, which on Linux is often DejaVu Sans, a good deal
// wider. A long name that fitted in Noto then pushed what stood beside it past
// the edge: `ErrorSummaryItem · 4 examples` scrolled `/error-summary/` sideways
// by twelve pixels at 320px, the chevron of its fold past the window. Inside an
// example the frame cuts such an overflow off instead of scrolling, so the
// walk asks two things: that the page does not scroll sideways, and that no
// row in the main column pushes what it holds past its own end. A meter's
// reading, cut off in Portuguese, and the assistant's mark after it, past the
// line in English even in Noto, were the second kind.
//
// THE FACE IS ASKED FOR AND THEN CHECKED. `fonts.css` is served with DejaVu
// Sans at the head of the interface's stack, and the walk fails rather than
// measures where the page's lead is drawn in anything else: a machine
// without the face would otherwise measure Noto again and pass.

const WIDE = "DejaVu Sans"; // allow-literal: the wide face this walk measures in, which it asks Chrome was drawn
const PRELOADED = "Noto Sans"; // allow-literal: the face `fonts.css` opens the interface's stack with, which this walk takes out

/** Rows that push past their own end by design, each with its reason. */
const PAST_THEIR_END = [
  // An avatar overlaps the next one by its `overlap`, which is the stack.
  ".nd-avatar-group > li",
  // A footer line is one line, and what does not fit is cut at its end.
  ".nd-example-run",
];

walk(
  "nothing is pushed past its row in a wide face",
  {
    axes: { locale: LOCALES },
    title: ({ locale }) => `${locale} pushes nothing past its row at ${WIDTHS[0]}px in ${WIDE}`,
  },
  async ({ page, locale, path }) => {
    await page.route("**/fonts.css", async (route) => {
      const response = await route.fetch();
      const sheet = await response.text();
      const wide = sheet.replace(
        `--noctua-font-sans: "${PRELOADED}",`,
        `--noctua-font-sans: "${WIDE}",`,
      );
      expect(wide, `fonts.css no longer opens the interface's stack with ${PRELOADED}`).not.toBe(sheet);
      await route.fulfill({ response, body: wide });
    });
    await page.setViewportSize({ width: WIDTHS[0], height: 900 });
    await page.goto(url(path, locale));
    await showParts(page);
    await page.evaluate(() => document.fonts.ready);

    const cdp = await page.context().newCDPSession(page);
    await cdp.send("DOM.enable");
    await cdp.send("CSS.enable");
    const { root } = await cdp.send("DOM.getDocument", { depth: -1 });
    const { nodeId } = await cdp.send("DOM.querySelector", {
      nodeId: root.nodeId,
      selector: ".nd-doc-head .nd-lead",
    });
    const { fonts } = await cdp.send("CSS.getPlatformFontsForNode", { nodeId });
    expect(
      fonts.map((font) => font.familyName),
      `the page was not drawn in ${WIDE}, so this walk measures nothing: install it`,
    ).toContain(WIDE);

    const found = await page.evaluate((exempt) => {
      const root = document.documentElement;
      const scrolls = root.scrollWidth - root.clientWidth;
      const pushed = [];
      for (const row of document.querySelectorAll(".nd-main *")) {
        const style = getComputedStyle(row);
        if (!/flex/.test(style.display) || style.flexWrap !== "nowrap") continue;
        if (style.flexDirection.startsWith("column") || style.overflowX !== "visible") continue;
        const box = row.getBoundingClientRect();
        if (box.width === 0 || box.height === 0) continue;
        if (row.closest("[hidden], [aria-hidden='true']")) continue;
        // Off the screen on purpose: a label kept for a screen reader, a
        // stacked table's heading row.
        let drawn = true;
        for (let up = row; up && drawn; up = up.parentElement) {
          const s = getComputedStyle(up);
          if (s.display === "contents") continue;
          const b = up.getBoundingClientRect();
          drawn = s.clipPath !== "inset(50%)" && b.width > 1 && b.height > 1;
        }
        if (!drawn || exempt.some((selector) => row.matches(selector))) continue;
        const end = box.right - Number.parseFloat(style.borderRightWidth);
        for (const child of row.children) {
          const own = child.getBoundingClientRect();
          if (own.width === 0 || /absolute|fixed/.test(getComputedStyle(child).position)) continue;
          // Two pixels: a glyph turned forty-five degrees reports its
          // diagonal, which is past its own box by that much and draws nothing
          // there.
          const past = own.right - end;
          if (past > 2) {
            const name = (el) => `${el.tagName.toLowerCase()}.${[...el.classList].join(".")}`;
            pushed.push(`${name(row)} > ${name(child)}, ${Math.round(past)}px past its end`);
          }
        }
      }
      return { scrolls, pushed };
    }, PAST_THEIR_END);

    expect(found.scrolls, "the page scrolls sideways").toBeLessThanOrEqual(1);
    expect(found.pushed, "a row pushed what it holds past its end").toEqual([]);
  },
);
