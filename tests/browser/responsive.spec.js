// Nothing on this site scrolls sideways, in any language, at any width.
//
// WHY THIS IS A TEST AND NOT A LOOK.
//
// A horizontal scrollbar is the one layout failure that is invisible to the
// person who wrote the page and unmissable to everyone else: it appears at a
// width the author does not use, or in a language the author does not read, and
// it makes every line on the page require two gestures instead of one.
//
// `docs-site/PROVISIONAL.md` says to open `/qps/` at 320px "as well: anything
// that overflows only fitted because English is short". This is that sentence
// as a check that runs. It found the compound button's second line, which
// inherited `white-space: nowrap` from the button around it and was 410px of
// pseudo-locale text inside a 199px control.
//
// 320px is the floor `spec/tokens.toml` names — "the narrowest screen still in
// real use", and not a breakpoint because it is the default. The other three
// are the breakpoints either side of the sidebar docking.

import { test, expect } from "@playwright/test";
import { LOCALES, PAGE, url } from "./pages.js";

// 1024 IS NOT DECORATION. It is the first width where the sidebar is docked
// AND the column beside it is tight, and skipping it is how three pages came to
// scroll sideways on every laptop without anybody noticing.
const WIDTHS = [320, 390, 768, 1024, 1440];

for (const locale of LOCALES) {
  for (const [name, page_] of Object.entries(PAGE)) {
    for (const width of WIDTHS) {
      test(`${locale} ${name} does not scroll sideways at ${width}px`, async ({ page }) => {
        await page.setViewportSize({ width, height: 900 });
        await page.goto(url(page_, locale));

        const overflow = await page.evaluate(() => {
          const root = document.documentElement;
          if (root.scrollWidth <= root.clientWidth + 1) return null;
          // Name the element that did it, not just the fact. A failure that
          // says "the page is 69px too wide" costs an afternoon; one that says
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
      });
    }
  }
}
