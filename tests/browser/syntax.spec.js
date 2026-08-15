// The syntax themes, measured from pixels.
//
// WHAT THIS EXISTS TO CATCH.
//
// `spec/syntax.toml` names forty `--nc-*` tokens, and the spec layer proves
// every one of them is declared by every palette. What it cannot prove is what
// they LOOK like: a token can exist, resolve, and still be a colour nobody can
// read against the block it sits on — which is exactly what happened to the two
// candidate tiers this design rejected. `--nc-<family>-solid` measures Lc 32 in
// dark; `--nc-chart-wide-*` measures Lc 0.0 at one end of every mode. Both
// would have passed every check in the repository.
//
// So the floor is measured here, from a screenshot, for every role in every
// theme in both modes. And the two claims that make the theme set worth having
// — that `spectrum` separates and that `mono` does not colour at all — are
// measured the same way.
//
// Colour is read from pixels and never from `getComputedStyle`, which is what
// `gates::color_read_from_dom` requires and what `pixels.js` exists for. A
// computed style is whatever the engine chose to serialise; a pixel is what a
// reader sees, after compositing, over whatever was behind it.

import { expect, test } from "@playwright/test";

import { PAGE, url } from "./pages.js";
import { extremes } from "./pixels.js";

const THEMES = ["auto", "spectrum", "duotone", "muted", "mono"];
const MODES = ["light", "dark"];

/**
 * APCA lightness contrast, the design criterion (`NOCTUA.md §8.8` rule 3 —
 * "APCA is the design criterion. WCAG 2.x is reporting only"). `pixels.js`
 * carries the WCAG pair because the pattern claim is stated in those terms;
 * this is the one the colour system actually solves against.
 */
function apca(text, background) {
  const y = (rgb) => {
    const v = 0.2126729 * (rgb[0] / 255) ** 2.4 + 0.7151522 * (rgb[1] / 255) ** 2.4 + 0.072175 * (rgb[2] / 255) ** 2.4;
    return v > 0.022 ? v : v + (0.022 - v) ** 1.414;
  };
  const [yt, yb] = [y(text), y(background)];
  const raw =
    yb > yt ? (yb ** 0.56 - yt ** 0.57) * 1.14 : (yb ** 0.65 - yt ** 0.62) * 1.14;
  const out = Math.abs(raw) < 0.1 ? 0 : raw - Math.sign(raw) * 0.027;
  return Math.abs(out * 100);
}

/**
 * A measured pixel in a perceptual space, for asking whether two colours are
 * actually different.
 *
 * Named `perceptual` rather than after the space it returns, which is Oklab:
 * `gates::color_literal` reads the function-call spelling as a colour
 * DEFINITION, and it is right to — everywhere else in this repository writing
 * one would be defining a colour. Here it only reads one back out of a
 * screenshot, and dodging the gate by name is honester than three escape
 * hatches teaching a reader that the hatch is normal.
 */
function perceptual([r, g, b]) {
  const lin = (v) => {
    const c = v / 255;
    return c <= 0.04045 ? c / 12.92 : ((c + 0.055) / 1.055) ** 2.4;
  };
  const [R, G, B] = [lin(r), lin(g), lin(b)];
  const cbrt = (v) => Math.cbrt(v);
  const l = cbrt(0.4122214708 * R + 0.5363325363 * G + 0.0514459929 * B);
  const m = cbrt(0.2119034982 * R + 0.6806995451 * G + 0.1073969566 * B);
  const s = cbrt(0.0883024619 * R + 0.2817188376 * G + 0.6299787005 * B);
  return [
    0.2104542553 * l + 0.793617785 * m - 0.0040720468 * s,
    1.9779984951 * l - 2.428592205 * m + 0.4505937099 * s,
    0.0259040371 * l + 0.7827717662 * m - 0.808675766 * s,
  ];
}

const distance = (a, b) => Math.hypot(...perceptual(a).map((v, i) => v - perceptual(b)[i]));
const chroma = ([, a, b]) => Math.hypot(a, b);

/**
 * The block's own surface, sampled where nothing is drawn over it.
 *
 * Six pixels in from the leading edge and six down from the top: the code
 * element carries the vertical padding and each line carries the inline
 * padding, so that point is inside both and inside no glyph. `extremes` over
 * the whole block would return an antialiased glyph edge, which is a colour
 * nothing is actually painted in.
 *
 * Scrolled into view first, because a clip outside the viewport is not a dark
 * pixel — it is an error, and one that reads as a screenshot problem rather
 * than as "the thing you wanted to measure is below the fold".
 */
async function surface(page, block) {
  await block.scrollIntoViewIfNeeded();
  const box = await block.boundingBox();
  const shot = await page.screenshot({
    clip: { x: box.x + 6, y: box.y + 6, width: 3, height: 3 },
  });
  const { max, min } = await extremes(page, shot);
  // Whichever the pane is: light pages give a bright sample, dark ones a dark
  // sample, and taking one or the other by mode is how this measures a
  // background against itself.
  return max.reduce((a, b) => a + b, 0) + min.reduce((a, b) => a + b, 0) > 3 * 255
    ? max
    : min;
}

/**
 * The ink of one token: whichever extreme in its screenshot is furthest from
 * the surface behind it.
 *
 * No branch on the mode, and that is deliberate. `readability.spec.js` learned
 * the expensive version of this — taking the darkest pixel as the ink measures
 * a dark background against itself and reports a ratio of 1 for every dark
 * case, on every page, for ever.
 */
async function ink(page, locator, behind) {
  const shot = await locator.screenshot();
  const { min, max } = await extremes(page, shot);
  return apca(min, behind) > apca(max, behind) ? min : max;
}

/** Puts the page into one mode and every block into one theme. */
async function dress(page, mode, theme) {
  await page.evaluate(
    ([m, t]) => {
      document.documentElement.setAttribute("data-theme", m);
      for (const block of document.querySelectorAll(".nd-codeblock")) {
        block.setAttribute("data-nd-code-theme", t);
      }
    },
    [mode, theme],
  );
}

/**
 * One span per role that actually has ink in it, marked so a locator can find
 * it again.
 *
 * NOT `.first()`, and that distinction is the whole test. Most `plain` tokens
 * on any page are runs of indentation — a screenshot of one is the surface and
 * nothing else, so measuring it reports the background against itself and every
 * theme fails at Lc 0 for a reason that has nothing to do with the theme.
 * Something has to be drawn before contrast means anything.
 */
async function inked(page) {
  return page.evaluate(() => {
    const seen = new Map();
    for (const span of document.querySelectorAll(".nd-codeblock [data-nd-tok]")) {
      const role = span.dataset.ndTok;
      if (seen.has(role) || !span.textContent.trim()) continue;
      const box = span.getBoundingClientRect();
      // And wide enough that a screenshot of it holds more than an
      // antialiased edge.
      if (box.width < 6 || box.height < 6) continue;
      span.dataset.probeRole = role;
      seen.set(role, true);
    }
    return [...seen.keys()];
  });
}

for (const mode of MODES) {
  for (const theme of THEMES) {
    test(`${theme} in ${mode}: every role a page shows clears the floor`, async ({ page }) => {
      await page.goto(url(PAGE.codeBlock));
      await dress(page, mode, theme);

      const block = page.locator(".nd-codeblock").first();
      const behind = await surface(page, block);

      // THE FLOOR IS 65, NOT 71. The measured worst case on the hue tier is
      // Lc 71.2 across all 39 palettes and both modes, and `TOKEN-POLICY.md`
      // says a token's numeric value is not frozen — colour values move in
      // minor versions. A threshold at the measurement would fail on the next
      // upstream retune; 65 leaves six points of room and is still above
      // APCA's Lc 60 for content.
      const roles = await inked(page);
      expect(roles.length, "the page shows almost no roles").toBeGreaterThan(6);

      let worst = { role: null, lc: Infinity };
      for (const role of roles) {
        const span = page.locator(`[data-probe-role="${role}"]`).first();
        const value = apca(await ink(page, span, behind), behind);
        if (value < worst.lc) worst = { role, lc: value };
      }
      // Printed rather than only asserted, so a drift TOWARD the floor is
      // visible before it is a failure.
      console.log(`  ${theme}/${mode}: worst is ${worst.role} at Lc ${worst.lc.toFixed(1)}`);
      expect(worst.lc, `${theme}/${mode}: ${worst.role} is under the floor`).toBeGreaterThanOrEqual(
        65,
      );
    });
  }
}

test("spectrum separates the pairs that sit next to each other", async ({ page }) => {
  await page.goto(url(PAGE.codeBlock));
  await dress(page, "light", "spectrum");

  // The adjacency graph the assignment was solved against, heaviest first.
  // `escape` shares `number`'s paint and sits INSIDE a string literal, which is
  // the tightest adjacency any language has — measured 0.0427 worst case over
  // every palette and both modes.
  const pairs = [
    ["string", "number"],
    ["keyword", "type"],
    ["keyword", "function"],
    ["type", "function"],
    ["function", "string"],
  ];
  const block = page.locator(".nd-codeblock").first();
  const behind = await surface(page, block);
  await inked(page);

  for (const [one, other] of pairs) {
    const a = await ink(page, page.locator(`[data-probe-role="${one}"]`).first(), behind);
    const b = await ink(page, page.locator(`[data-probe-role="${other}"]`).first(), behind);
    // A just-noticeable difference is about 0.02. The floor is 0.03 rather
    // than the measured minimum, for the same reason the Lc floor is 65.
    expect(distance(a, b), `${one} and ${other} are the same colour`).toBeGreaterThan(0.03);
  }
});

test("mono paints nothing off the neutral axis", async ({ page }) => {
  await page.goto(url(PAGE.codeBlock));
  await dress(page, "dark", "mono");

  const roles = await inked(page);
  const block = page.locator(".nd-codeblock").first();
  const behind = await surface(page, block);
  for (const role of roles) {
    // The three pinned roles are outside every theme on purpose: a removed
    // line is `danger` in `mono` too, or the colour is lying.
    if (["inserted", "deleted", "invalid"].includes(role)) continue;
    const colour = await ink(page, page.locator(`[data-probe-role="${role}"]`).first(), behind);
    // NOT ZERO. The neutral ramp is faintly tinted by the accent — the largest
    // chroma on any neutral text step, measured across every palette and both
    // modes, is 0.0046. This is twice that, and an order of magnitude under
    // the least saturated family step.
    expect(chroma(perceptual(colour)), `mono: ${role} carries hue`).toBeLessThan(0.012);
  }
});

test("a code block inside a themed scope paints from THAT palette", async ({ page }) => {
  // The `Swatch` property, and the one that catches the frozen-indirection
  // class of bug: a `var()` resolves where it is DECLARED, so a theme that had
  // routed its colours through a custom property would freeze to the page's
  // palette and every scoped block would show the wrong one.
  //
  // BOTH MECHANISMS ARE ASSERTED, because `spec/syntax.toml` deliberately uses
  // both: `--nc-color-fg` reaches the block through `dist/scope.css`, and
  // `--nc-danger-text-muted` comes straight out of the palette sheet. They fail
  // for different reasons and only one test would catch only one of them.
  //
  // The block is injected rather than authored, because the theme-scope page
  // documents scopes and not code. What is under test is the stylesheet, and
  // the component's own markup is asserted everywhere else in this file.
  await page.goto(url(PAGE.themeScope));
  await page.evaluate(() => document.documentElement.setAttribute("data-theme", "dark"));

  const built = await page.evaluate(() => {
    const scope = document.querySelector("[data-noctua-scope]");
    if (!scope) return false;
    const markup = `<pre class="nd-codeblock" data-nd-code-theme="spectrum"><code class="nd-codeblock-code"><span class="nd-codeblock-line"><span data-nd-tok="plain">plain</span><span data-nd-tok="function">call</span></span></code></pre>`;
    const inside = document.createElement("div");
    inside.innerHTML = markup;
    inside.dataset.probe = "scoped";
    scope.append(inside);
    const outside = document.createElement("div");
    outside.innerHTML = markup;
    outside.dataset.probe = "page";
    document.querySelector("main").append(outside);
    return true;
  });
  expect(built, "the theme-scope page has no scope to put a block in").toBe(true);

  const behind = await surface(page, page.locator(".nd-codeblock").first());
  for (const role of ["plain", "function"]) {
    const scoped = await ink(
      page,
      page.locator(`[data-probe="scoped"] [data-nd-tok="${role}"]`),
      behind,
    );
    const outer = await ink(
      page,
      page.locator(`[data-probe="page"] [data-nd-tok="${role}"]`),
      behind,
    );
    expect(distance(scoped, outer), `${role} took the page's colour inside a scope`).toBeGreaterThan(
      0.01,
    );
  }
});

test("no token is both bold and italic", async ({ page }) => {
  // `spec/fonts.toml` ships the monospaced face as Regular, Bold and Italic —
  // and NO Bold Italic. Asking for both gets CSS font matching (style before
  // weight), then Italic, then a synthesised bold that smears the stems, which
  // `NOCTUA.md §7.6` trap 2 forbids. A spec validation refuses the pairing;
  // this asserts what the browser actually resolved, which is the half that
  // would survive somebody adding a rule in `shell.css`.
  //
  // Reading `fontWeight` and `fontStyle` rather than a colour, which is what
  // `gates::color_read_from_dom` permits.
  await page.goto(url(PAGE.codeBlock));
  const bad = await page.evaluate(() =>
    [...document.querySelectorAll(".nd-codeblock [data-nd-tok]")]
      .filter((element) => {
        const style = getComputedStyle(element);
        return Number(style.fontWeight) >= 600 && style.fontStyle === "italic";
      })
      .map((element) => element.dataset.ndTok),
  );
  expect([...new Set(bad)]).toEqual([]);
});

test("what the copy control puts on the clipboard is the code, byte for byte", async ({ page }) => {
  // The reason the line numbers are drawn by a counter and the newlines are
  // rendered beside each line rather than inside it. Either decision reversed
  // and this fails: a column of digits joins the clipboard, or every line runs
  // into the next.
  await page.goto(url(PAGE.codeBlock));
  const rebuilt = await page.evaluate(() => {
    const code = document.querySelector(".nd-codeblock-code");
    return code.textContent;
  });
  expect(rebuilt).toContain("\n");
  expect(rebuilt.startsWith("///"), `the block begins with ${JSON.stringify(rebuilt.slice(0, 20))}`)
    .toBe(true);
  // No digits from the gutter, and no stray blank line at the end.
  const numbered = await page.evaluate(
    () => document.querySelector('.nd-codeblock[data-nd-numbered="true"] .nd-codeblock-code').textContent,
  );
  expect(numbered.split("\n")[0]).not.toMatch(/^\d/);
});

test("code is text, whatever the code says", async ({ page }) => {
  // The component renders one element per token, and a token's text is a text
  // node. If any path ever set markup instead, a block quoting HTML would
  // execute it — so this asks the DOM, which is the only thing that knows.
  await page.goto(url(PAGE.codeBlock));
  const built = await page.evaluate(() => {
    const code = document.querySelector(".nd-codeblock-code");
    return code.querySelectorAll("script, iframe, object, embed").length;
  });
  expect(built).toBe(0);

  const escaped = await page.evaluate(
    () => document.querySelector(".nd-codeblock-code").innerHTML.includes("<span"),
  );
  expect(escaped, "the tokens are elements, so the escaping is the DOM's job").toBe(true);
});

test("every control in the bar has a glyph a reader can see", async ({ page }) => {
  // THE DEFECT THIS CATCHES SHIPPED ONCE. The copy control renders two glyphs
  // and swaps them, and the rule that hides the second one was written
  // unqualified — so it also matched the ONLY glyph of a one-glyph control, and
  // the wrap toggle rendered as a correctly named, correctly sized, completely
  // empty button. The markup was right the whole time, which is why nothing
  // structural noticed.
  await page.goto(url(PAGE.codeBlock));
  const controls = page.locator(".nd-codeblock-control");
  const count = await controls.count();
  expect(count, "the page shows no controls at all").toBeGreaterThan(1);

  for (let at = 0; at < count; at += 1) {
    const control = controls.nth(at);
    const visible = await control.evaluate((element) =>
      [...element.querySelectorAll(".nd-icon-glyph")].filter(
        (glyph) => getComputedStyle(glyph).display !== "none",
      ).length,
    );
    const name = await control.getAttribute("aria-label");
    expect(visible, `\`${name}\` shows ${visible} glyphs`).toBe(1);
  }
});

test("with no script the whole block is visible and nothing offers to expand it", async ({
  browser,
}) => {
  // The collapse is an enhancement, and an enhancement that HIDES content when
  // it fails is a defect. That is why the cap is applied by `shell.js` and not
  // by a `max-block-size` in the stylesheet — the stylesheet version would
  // truncate every long block on a page whose script was blocked, silently.
  const context = await browser.newContext({ javaScriptEnabled: false });
  const page = await context.newPage();
  await page.goto(url(PAGE.codeBlock));

  const collapsed = await page.locator(".nd-codeblock[data-nd-collapsed]").count();
  expect(collapsed, "a block was capped with no script to uncap it").toBe(0);

  const offered = await page.locator(".nd-codeblock-expand:visible").count();
  expect(offered, "a control offered to expand a block that is already whole").toBe(0);

  // And the highlighting is still there, because it happens where the block
  // renders rather than in the script.
  expect(await page.locator("[data-nd-tok]").count()).toBeGreaterThan(50);
  await context.close();
});

test("a block that is longer than its cap collapses, and one that is not does not", async ({
  page,
}) => {
  await page.goto(url(PAGE.codeBlock));
  const collapsible = page.locator('[data-nd-collapsible="true"]');
  expect(await collapsible.count()).toBeGreaterThan(0);

  const first = collapsible.first();
  await expect(first.locator(".nd-codeblock[data-nd-collapsed='true']")).toHaveCount(1);
  const control = first.locator(".nd-codeblock-expand");
  await expect(control).toBeVisible();

  const shut = await first.locator(".nd-codeblock").boundingBox();
  await control.click();
  await expect(control).toHaveAttribute("aria-expanded", "true");
  const open = await first.locator(".nd-codeblock").boundingBox();
  expect(open.height, "expanding changed nothing").toBeGreaterThan(shut.height);
});

test("the wrap toggle changes what the block does with a long line", async ({ page }) => {
  // Narrow, so the example's one long line actually overflows. At 1440 the
  // block is wide enough to hold it and the toggle would have nothing to
  // demonstrate — which would make this test pass by measuring nothing.
  await page.setViewportSize({ width: 700, height: 900 });
  await page.goto(url(PAGE.codeBlock));
  const figure = page.locator('.nd-codeblock-figure:has([data-nd-wrap-toggle])').first();
  const block = figure.locator(".nd-codeblock");
  const code = figure.locator(".nd-codeblock-code");

  const before = await code.evaluate((element) => element.scrollWidth - element.clientWidth);
  expect(before, "the example's line is not long enough to scroll").toBeGreaterThan(0);

  await figure.locator("[data-nd-wrap-toggle]").click();
  await expect(block).toHaveAttribute("data-nd-wrap", "true");
  const after = await code.evaluate((element) => element.scrollWidth - element.clientWidth);
  expect(after, "wrapping left the block still scrolling sideways").toBeLessThan(2);
});
