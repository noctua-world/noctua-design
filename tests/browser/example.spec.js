// The example frame, and the claim only a browser can settle.
//
// `NOCTUA.md §9.3` says the Dioxus and React implementations must be
// behaviourally identical. Generating both from one table makes their prop
// surfaces identical by construction — but "the same props" is not "the same
// behaviour", and the only way to show the second is to mount both and compare
// what they put in the DOM.

import { test, expect } from "@playwright/test";
import { ANY, LOCALES, PAGE, SCOPED, url, showParts, setFrameWidth } from "./pages.js";

// The frame is on every component page now, and `ThemeScope`'s is the one it
// can mount in both implementations — so that is where these run.
//
// There is no code panel to open any more: each example carries its call as its
// caption, always visible, because a reader who has to open a disclosure to see
// the line they came to copy is a reader who copies the wrong thing.


const requests = (page) =>
  page.evaluate(() =>
    performance.getEntriesByType("resource").filter((r) => r.name.includes("example-react")).length,
  );

test("the React bundle is not fetched on a normal page view", async ({ page }) => {
  await page.goto(url(SCOPED));
  await showParts(page);
  await page.waitForLoadState("networkidle");
  // 190 KB that most readers never need. A lazy path that loads eagerly is not
  // a lazy path; it is a lazy path with a bug nobody notices.
  expect(await requests(page)).toBe(0);
});

test("switching to React fetches it once and draws what Dioxus drew", async ({ page }) => {
  // THE WHOLE MECHANISM, ON A REAL COMPONENT PAGE.
  //
  // This used to be one hand-written example on the ThemeScope page, and one
  // example is what let six modules of the React package ship broken. Every
  // example is data now, so this asserts the thing that matters: the two
  // implementations put the SAME elements, classes and attributes in the DOM.
  await page.goto(url(PAGE.button));

  const shape = (root) =>
    [...root.querySelectorAll("*")].map((el) =>
      [
        el.tagName,
        [...el.classList].sort().join("."),
        [...el.attributes]
          .filter((a) => /^(data-nd|aria|role)/.test(a.name))
          .map((a) => `${a.name}=${a.value}`)
          .sort()
          .join(","),
      ].join("|"),
    );

  const stage = '[data-nd-example="button-1"]';
  const before = await page.evaluate(
    ([sel, fn]) => eval(`(${fn})`)(document.querySelector(sel)),
    [stage, shape.toString()],
  );

  await page.locator('[data-nd-action="impl"]').click();
  // A frame is drawn when it comes into reach, so it has to be brought into
  // reach. That laziness is the whole reason a page with forty-seven examples
  // stays answerable, and it is asserted separately.
  await page.locator(stage).scrollIntoViewIfNeeded();
  await page.waitForFunction(
    (sel) => document.querySelector(sel)?.dataset.ndShowing === "react",
    stage,
    { timeout: 20000 },
  );

  expect(await requests(page)).toBe(1);

  const after = await page.evaluate(
    ([sel, fn]) => eval(`(${fn})`)(document.querySelector(`${sel} [data-nd-react-slot]`)),
    [stage, shape.toString()],
  );
  expect(after, "React drew something else").toEqual(before);
  expect(before.length, "the comparison compared nothing").toBeGreaterThan(4);

  // And switching twice more does not fetch it again.
  await page.locator('[data-nd-action="impl"]').click();
  await page.locator('[data-nd-action="impl"]').click();
  expect(await requests(page)).toBe(1);
});

test("the switch draws the half it would move to, and keeps both glyphs", async ({ page }) => {
  // THE GLYPH IS THE DESTINATION. An atom while Dioxus is showing, because
  // pressing it goes to React; a helix while React is showing, to come back.
  // `Button` ships both and the state picks one, so this works with no script.
  await page.goto(url(PAGE.button));
  const glyphs = () =>
    page.evaluate(() =>
      [...document.querySelector('[data-nd-action="frame-impl"]').querySelectorAll(".nd-icon-glyph")]
        .map((g) => `${g.dataset.ndIcon}:${g.getBoundingClientRect().width > 2 ? "shown" : "hidden"}`),
    );

  expect(await glyphs()).toEqual(["atom:shown", "dna:hidden"]);

  await page.locator('[data-nd-action="impl"]').click();
  await page.waitForFunction(
    () => document.querySelectorAll('[data-nd-showing="react"]').length > 0,
    null,
    { timeout: 20000 },
  );

  // BOTH ARE STILL THERE. The loading message used to be written into the
  // control's `textContent`, which was harmless on a stop with a word in it and
  // destroyed both SVGs on an icon — a pressed button with nothing inside it.
  // It is `aria-busy` and the accessible name now, neither of which touches the
  // tree.
  expect(await glyphs()).toEqual(["atom:hidden", "dna:shown"]);
  await expect(page.locator('[data-nd-action="frame-impl"]').first()).not.toHaveAttribute(
    "aria-busy",
    "true",
  );
});

test("switching back unmounts React rather than hiding it", async ({ page }) => {
  await page.goto(url(PAGE.button));
  const stage = '[data-nd-example="button-1"]';

  await page.locator('[data-nd-action="impl"]').click();
  await page.locator(stage).scrollIntoViewIfNeeded();
  await page.waitForFunction(
    (sel) => document.querySelector(sel)?.dataset.ndShowing === "react",
    stage,
    { timeout: 20000 },
  );

  await page.locator('[data-nd-action="impl"]').click();
  // A live React root behind a hidden div keeps its state and its effects, and
  // the next switch would show a component that had been running all along.
  await expect(page.locator(`${stage} [data-nd-react-slot] > *`)).toHaveCount(0);
  await expect(page.locator(`${stage} > *:not([data-nd-react-slot])`).first()).toBeVisible();
});

test("the code panel shows the implementation that is mounted", async ({ page }) => {
  // On a component page rather than `/theme-scope/`. That frame was the one
  // that could mount React, and it was the one because every other example was
  // a closure. They are all trees now, and `ThemeScope` is the one component
  // left out — it comes from `spec/theme.toml`, so the generated interpreter
  // has no arm for it. That is recorded as work owed.
  await page.goto(url(PAGE.button));
  const frame = '[data-nd-example="button-1"]';
  await expect(page.locator('[data-nd-impl-source="Dioxus"]').first()).toBeVisible();

  await page.locator('[data-nd-action="impl"]').click();
  await page.locator(frame).scrollIntoViewIfNeeded();
  await page.waitForFunction(
    (sel) => document.querySelector(sel)?.dataset.ndShowing === "react",
    frame,
    { timeout: 20000 },
  );

  const band = page.locator(`${frame}`).locator("xpath=../figcaption");
  await expect(band.locator('[data-nd-impl-source="React"]')).toBeVisible();
  await expect(band.locator('[data-nd-impl-source="Dioxus"]')).toBeHidden();
});

test("the width control resizes the stage and nothing else", async ({ page }) => {
  await page.goto(url(SCOPED));
  await showParts(page);
  const stage = page.locator("[data-nd-frame-stage]");
  const full = (await stage.boundingBox()).width;

  await setFrameWidth(page, "320");
  const narrow = (await stage.boundingBox()).width;
  expect(narrow).toBeLessThan(full);

  // The page around it did not move.
  const main = await page.locator(".nd-main").boundingBox();
  expect(main.width).toBeGreaterThan(narrow);

  await page.locator('[data-nd-action="frame-reset"]').click();
  await page.waitForTimeout(250);
  expect((await stage.boundingBox()).width).toBeCloseTo(full, 0);
});

// The test that was here — "search finds the system's own names, and only when
// asked" — lives in `search.spec.js` now, rewritten around the palette. It was
// written against 313 hidden anchors and a `.includes()`; both are gone, and
// the three queries it made survive there as the ranking tests.

test("every locale ships the same page and switches between them", async ({ page }) => {
  await page.goto(url(SCOPED));
  await showParts(page);
  // Read the English sidebar before leaving it: the component block is compared
  // against it below, and a second navigation to fetch it would compare two
  // page loads instead of one page in two languages.
  const english = await page.locator(".nd-nav-text").allTextContents();
  await page.locator(".nd-locale > summary").click();
  await page.locator("[data-nd-locale='pt-BR']").click();
  await expect(page).toHaveURL(new RegExp(`/pt-BR/${SCOPED}$`));
  expect(await page.locator("html").getAttribute("lang")).toBe("pt-BR");

  // The introduction alone, then the foundations translated and in the
  // reader's alphabet, then every component alphabetically with nothing
  // between them.
  const order = await page.locator(".nd-nav-text").allTextContents();
  expect(order[0]).toBe("Introdução");
  expect(order).toContain("Tipografia");
  // THE CLAIM IS THE COLLATION, NOT THE MEMBERSHIP.
  //
  // `Ícones` sorts by its base letter rather than after `z`, and `Espaçamento`
  // before both — which is the reader's alphabet and not Rust's `str` ordering.
  // This used to assert the exact list, so adding the spacing page failed a
  // test about sorting for a reason that had nothing to do with sorting.
  const foundations = order.slice(1, order.indexOf("Blockquote"));
  expect(foundations).toContain("Ícones");
  expect(foundations).toContain("Tipografia");
  expect(foundations.indexOf("Espaçamento")).toBeLessThan(foundations.indexOf("Ícones"));
  expect(foundations.indexOf("Ícones")).toBeLessThan(foundations.indexOf("Tipografia"));

  // And the components are a block of their own, in order, after them.
  const components = order.slice(order.indexOf("Blockquote"));
  expect(components[0]).toBe("Blockquote");
  expect(components.at(-1)).toBe("VisuallyHidden");
  // `ThemeScope` among them, sorted by its title like the rest — it is a
  // component, and the block has no exceptions in it. So is the one page in it
  // whose title is a label rather than an identifier: the ten inline marks are
  // documented together under `Inline text`, which is not translated either.
  expect(components).toContain("ThemeScope");
  expect(components).toContain("Inline text");

  // THE BLOCK IS THE SAME WORD FOR WORD IN BOTH LANGUAGES.
  //
  // It was not. `/tree/` was `Árvore` here, `/search/` was `Busca` and
  // `/avatars/` was `Avatares` — three rows naming components that exist under
  // no such name in any file, any import or any search box. `site.rs` asserts
  // the rule at its source; this asserts what a reader actually sees, in the
  // one place both languages are on screen in one run.
  expect(components).toEqual(english.slice(english.indexOf("Blockquote")));

  // COLLATED, like the sidebar itself. A plain `.sort()` is codepoint order,
  // which agrees with this block while every title in it is ASCII — so the
  // claim it settles today is the foundations' (`Espaçamento` before `Ícones`
  // before `Tipografia`, above). It stays here because the day a component
  // block title stops being ASCII is the day it starts settling this one too.
  const collated = [...components].sort((a, b) => a.localeCompare(b, "pt-BR"));
  expect(collated).toEqual(components);
});

test("the language menu offers automatic, and remembers what was chosen", async ({
  page,
}) => {
  await page.goto(url(SCOPED));
  await showParts(page);

  // Three entries: the two locales, and the answer that is not a locale.
  const options = page.locator("[data-nd-locale]");
  await expect(options).toHaveCount(3);
  expect(await options.evaluateAll((all) => all.map((a) => a.dataset.ndLocale))).toEqual([
    "auto",
    "en",
    "pt-BR",
  ]);

  // Automatic is server-rendered as the one in force, so nothing announces a
  // wrong answer before the script runs.
  await expect(page.locator("[data-nd-locale='auto']")).toHaveAttribute(
    "aria-current",
    "true",
  );

  await page.locator(".nd-locale > summary").click();
  await page.locator("[data-nd-locale='pt-BR']").click();
  await expect(page).toHaveURL(new RegExp(`/pt-BR/${SCOPED}$`));
  expect(await page.evaluate(() => localStorage.getItem("noctua-locale"))).toBe("pt-BR");
  await expect(page.locator("[data-nd-locale='pt-BR']")).toHaveAttribute(
    "aria-current",
    "true",
  );
});

test("an explicitly requested locale is never redirected away from", async ({ page }) => {
  // Automatic, and a browser that asks for something this page is not in. A
  // link somebody shared has to open in the language it was shared in, so a
  // cold entry — no referrer inside the site — must not move.
  await page.goto(url(ANY));
  await showParts(page);
  await page.evaluate(() => localStorage.setItem("noctua-locale", "auto"));

  await page.goto(url(PAGE.typography, "pt-BR"));
  await showParts(page);
  await expect(page).toHaveURL(new RegExp(`/pt-BR/${PAGE.typography}$`));

  // Following a link inside the site is the other case, and there the reader's
  // browser wins: Playwright's Chromium asks for `en-US`.
  await page.locator(`.nd-nav-link[href="../${PAGE.icons}"]`).click();
  await expect(page).toHaveURL(new RegExp(`/en/${PAGE.icons}$`));
});

test("the ten inline marks are one page, and their own are gone", async ({ page }) => {
  // `Em` was a page. `Strong` was a page. A reader asking which mark a
  // keystroke takes opened three of them.
  const MARKS = ["abbr", "code", "del", "em", "ins", "kbd", "mark", "strong", "text", "time"];

  await page.goto(url("inline-text/"));
  await showParts(page);
  const sections = await page.evaluate(
    (marks) =>
      marks.map((slug) => {
        const section = document.getElementById(slug);
        return {
          slug,
          there: !!section,
          // A merge that quietly dropped a component's props would look exactly
          // like a merge that worked.
          props: !!section?.querySelector("table.nd-table"),
          heading: section?.querySelector("h2")?.textContent.trim() ?? null,
        };
      }),
    MARKS,
  );
  for (const section of sections) {
    expect(section.there, `${section.slug} is not on the page`).toBe(true);
    expect(section.props, `${section.slug} lost its props table`).toBe(true);
    expect(section.heading, `${section.slug} has no name of its own`).toBeTruthy();
  }

  // Every jump in the index lands on something.
  const jumps = await page.evaluate(() =>
    [...document.querySelectorAll(".nd-doc-index-list a")].map((a) => ({
      href: a.getAttribute("href"),
      lands: !!document.querySelector(a.getAttribute("href")),
    })),
  );
  expect(jumps.length).toBe(MARKS.length);
  expect(jumps.filter((jump) => !jump.lands)).toEqual([]);

  // AND IT LANDS BELOW THE HEADER. The header is sticky, so a jump that only
  // scrolls the section to y=0 puts the header on the section's own name.
  await page.goto(url("inline-text/#kbd"));
  await showParts(page);
  const landed = await page.evaluate(() => ({
    heading: document.querySelector("#kbd h2").getBoundingClientRect().top,
    header: document.querySelector(".nd-header").getBoundingClientRect().bottom,
  }));
  expect(landed.heading, "the jump landed under the header").toBeGreaterThan(landed.header);

  // And the pages they had are not written any more, in any locale.
  for (const locale of LOCALES) {
    for (const slug of MARKS) {
      const response = await page.request.get(url(`${slug}/`, locale));
      expect(response.status(), `/${locale}/${slug}/ is still being written`).toBe(404);
    }
  }
});

// --- The call's own header --------------------------------------------------
//
// The call and the example used to sit in one flex column, on one surface,
// twelve pixels apart, and the reader's complaint was that the two ran
// together. The caption is a band now: flush with the stage's edges, on the
// same `surface-subtle` the rail carries, with a hairline under it.
//
// Everything below is written as GEOMETRY rather than as the properties that
// produce it. The rail's two rounds taught that lesson twice: a declaration
// that loses on specificity is indistinguishable from one that works, and
// asserting `align-items` passes against a rule that never ran.

/** One frame's caption, its stage and the boxes around them. */
const band = (page, index = 0) =>
  page.locator("[data-nd-frame]").nth(index).evaluate((frame) => {
    const cap = frame.querySelector(".nd-example-source");
    const stage = frame.querySelector(".nd-frame-stage");
    const shown = frame.querySelector(".nd-example-stage");
    const cs = getComputedStyle(cap);
    const box = (el) => {
      const r = el.getBoundingClientRect();
      return { left: r.left, right: r.right, top: r.top, bottom: r.bottom, w: r.width, h: r.height };
    };
    // The stage's CONTENT box, which is what the band has to reach: it is the
    // padding moving off this element and onto the example's parts that lets
    // the band be flush at all.
    const ss = getComputedStyle(stage);
    const s = box(stage);
    return {
      cap: box(cap),
      stage: { ...s, padLeft: parseFloat(ss.paddingLeft), padRight: parseFloat(ss.paddingRight) },
      shown: box(shown),
      style: {
        border: parseFloat(cs.borderBlockEndWidth),
        bg: cs.backgroundColor,
        stageBg: ss.backgroundColor,
        shadow: cs.boxShadow,
      },
      frame: box(frame),
      rail: box(frame.querySelector(".nd-frame-bar")),
    };
  });

test("the call's band reaches both edges of its stage", async ({ page }) => {
  await page.setViewportSize({ width: 1120, height: 900 });
  for (const path of [PAGE.button, PAGE.spacing]) {
    await page.goto(url(path));
    const m = await band(page);
    // Flush, to the pixel. A band inset by the stage's old padding is a chip,
    // not a header — and that padding is exactly what moved to make room.
    expect(Math.round(m.cap.left - m.stage.left - m.stage.padLeft), path).toBe(0);
    expect(Math.round(m.stage.right - m.stage.padRight - m.cap.right), path).toBe(0);
  }
});

test("every line of the call starts on the same column", async ({ page }) => {
  // THE DEFECT: the first character of the first line looked like a space.
  //
  // `.nd-code-inline` is a pill — a tint with `0 var(--noctua-space-1)` around
  // it — which is right for a token name inside a sentence. On the band the
  // `subtle` tone already drops the tint, so all that survived was the padding,
  // and under `pre-wrap` only the FIRST line is inside it. Every call on the
  // site opened four pixels further in than its own second line.
  //
  // Measured on the rendered text rather than on the box, because the box was
  // never wrong: the run began where it should and the glyphs did not.
  await page.setViewportSize({ width: 1120, height: 1000 });
  await page.goto(url(PAGE.exampleFrame));
  const starts = await page.locator(".nd-example-source").first().evaluate((cap) => {
    const code = cap.querySelector(".nd-code-inline");
    const text = code.firstChild;
    const range = document.createRange();
    const seen = new Map();
    for (let at = 0; at < text.length; at += 1) {
      range.setStart(text, at);
      range.setEnd(text, at + 1);
      const box = range.getBoundingClientRect();
      if (!box.width || !text.data[at].trim()) continue;
      // The first painted glyph on each line, by the line's own top edge.
      const line = Math.round(box.top);
      if (!seen.has(line)) seen.set(line, Math.round(box.left - cap.getBoundingClientRect().left));
    }
    return [...seen.values()];
  });
  // `ExampleFrame {` and its closing `}` are the two unindented lines. They
  // have to begin on the same column, and that column is the band's padding.
  expect(starts.length, "the call is one line, so it cannot show this").toBeGreaterThan(2);
  expect(Math.min(...starts), `lines began at ${starts.join("/")}`).toBe(starts.at(-1));
});

test("the band is separated by a line, and never by elevation", async ({ page }) => {
  await page.goto(url(PAGE.button));
  const m = await band(page);
  expect(m.style.border, "nothing separates the band from the example").toBeGreaterThan(0);
  // A surface, so the band and the rail read as one layer of chrome. Stated
  // against the stage's own background rather than against a colour: naming one
  // here would be the test asserting the stylesheet against itself, and the
  // gate refuses a colour literal in source for exactly that reason. What is
  // claimed is that a reader can tell the header from what it heads.
  expect(m.style.bg).not.toBe(m.style.stageBg);
  // §9.7. `shell.spec.js` sweeps this globally; it is repeated here because
  // this is the newest painted surface on the site and the sweep is the kind of
  // test that only fails once somebody has already shipped the mistake.
  expect(m.style.shadow).toBe("none");
});

test("the example is under the band rather than inside it", async ({ page }) => {
  // THE DEVELOPER'S ACTUAL COMPLAINT, stated as a measurement: *isso acaba se
  // misturando ao conteúdo*.
  await page.setViewportSize({ width: 1120, height: 900 });
  await page.goto(url(PAGE.button));
  const m = await band(page);
  expect(Math.round(m.shown.top - m.cap.bottom), "the example overlaps its header").toBeGreaterThan(0);
});

test("the band costs the frame no height, and the rail still starts at the top", async ({ page }) => {
  await page.setViewportSize({ width: 1120, height: 900 });
  await page.goto(url(PAGE.button));
  const m = await band(page, 1);
  // The header spans the STAGE's column, not the frame's whole width. Spanning
  // both would put the rail in a second grid row and add about 36px to every
  // one of the site's frames — which is most of two rounds of compaction.
  expect(Math.round(m.rail.top - m.frame.top), "the rail was pushed down a row").toBeLessThanOrEqual(2);
  const heights = await page.evaluate(() =>
    [...document.querySelectorAll("[data-nd-frame]")]
      .map((f) => Math.round(f.getBoundingClientRect().height))
      .sort((a, b) => a - b),
  );
  // 189px before the band, 225px now, and the 36px between them is one more
  // control in the rail: the switch between the two implementations, which
  // every frame carries because every example can be drawn by both.
  //
  // It was a two-stop `Segmented` first, and that measured 260 — the rail is a
  // COLUMN, so a control with two rows in it costs twice what an icon does, and
  // the rail exists precisely to spend the width no frame was using rather than
  // the height every frame is short of. An icon toggle costs one row like the
  // four under it.
  //
  // The ceiling is what stops the next control being free. The assertion above
  // is the one that catches the real regression, a rail pushed into a second
  // grid row, which costs 36px on every frame at once.
  expect(heights[Math.floor(heights.length / 2)]).toBeLessThanOrEqual(230);
});

test("the band survives the narrowest screen", async ({ page }) => {
  await page.setViewportSize({ width: 320, height: 900 });
  for (const locale of LOCALES) {
    await page.goto(url(PAGE.button, locale));
    const m = await band(page);
    expect(Math.round(m.cap.left - m.stage.left - m.stage.padLeft), locale).toBe(0);
    expect(m.cap.h, `the band collapsed in ${locale}`).toBeGreaterThan(0);
    const sideways = await page.evaluate(
      () => document.documentElement.scrollWidth - document.documentElement.clientWidth,
    );
    expect(sideways, locale).toBe(0);
  }
});
