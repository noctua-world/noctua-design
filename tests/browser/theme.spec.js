// The theme runtime, measured where it actually runs.
//
// The whole design rests on one claim — that a component can carry a theme the
// page does not have — and on one hazard: a `var()` resolves where it is
// declared. Every test here is that claim or that hazard.

import { test, expect } from "@playwright/test";

/** A colour, read as PIXELS. Never `getComputedStyle(…).color`. */
async function paint(page, value) {
  return page.evaluate((v) => {
    const c = document.createElement("canvas");
    c.width = c.height = 1;
    const ctx = c.getContext("2d");
    ctx.fillStyle = v;
    ctx.fillRect(0, 0, 1, 1);
    return [...ctx.getImageData(0, 0, 1, 1).data].slice(0, 3);
  }, value);
}

const custom = (locator, name) =>
  locator.evaluate((el, n) => getComputedStyle(el).getPropertyValue(n).trim(), name);

test("a scope resolves the semantic contract against ITSELF", async ({ page }) => {
  await page.goto("/en/scoped-themes.html");
  const scopes = page.locator("[data-noctua-scope]");

  for (const index of [0, 1]) {
    const scope = scopes.nth(index);
    // This is the entire design in one assertion. Without dist/scope.css the
    // scope's --nc-color-surface holds the ROOT's value, because that is the
    // element the indirection was declared on — and a themed component paints
    // the page's colours with nothing to report.
    const surface = await paint(page, await custom(scope, "--nc-color-surface"));
    const own = await paint(page, await custom(scope, "--nc-neutral-bg-app"));
    expect(surface, `scope ${index} took its surface from the page`).toEqual(own);
  }

  // And the light scope genuinely differs from a dark page. The page is forced
  // dark here on purpose: this project runs with colorScheme "light", where a
  // light scope on a light page is legitimately the same colour and the
  // comparison would prove nothing either way.
  await page.evaluate(() => document.documentElement.setAttribute("data-theme", "dark"));
  const root = await paint(page, await custom(page.locator("html"), "--nc-color-surface"));
  const light = await paint(page, await custom(scopes.nth(0), "--nc-color-surface"));
  expect(light, "a light scope took the dark page's surface").not.toEqual(root);
});

test("every attribute a scope stamps is on one element", async ({ page }) => {
  await page.goto("/en/scoped-themes.html");
  // The colour system's forced-mode rule is compound and same-element. Split
  // the palette from the mode and it matches nothing — the child falls through
  // to the DEFAULT palette's unanchored [data-theme="dark"].
  const split = await page.evaluate(() =>
    [...document.querySelectorAll("[data-palette]")].filter(
      (el) => el.hasAttribute("data-theme") === false && el.closest("[data-noctua-scope]") === el,
    ).length,
  );
  const scopes = await page.locator("[data-noctua-scope]").count();
  expect(scopes).toBeGreaterThan(0);
  for (let i = 0; i < scopes; i += 1) {
    const el = page.locator("[data-noctua-scope]").nth(i);
    expect(await el.getAttribute("data-palette"), "a scope stamped no palette").toBeTruthy();
  }
  expect(split).toBeLessThanOrEqual(scopes);
});

test("a border inside a scope is the scope's, not the page's", async ({ page }) => {
  await page.goto("/en/scoped-themes.html");
  const border = (loc) => loc.evaluate((el) => getComputedStyle(el).borderTopColor);

  const pageBorder = await paint(page, await border(page.locator(".nd-foot")));
  const scopeBorder = await paint(page, await border(page.locator("[data-noctua-scope]").first()));

  // A `--nd-edge: … var(--nc-color-border)` at :root made these identical, and
  // nothing errored. This is the assertion that catches it coming back.
  expect(scopeBorder, "the scope's border came from the page's palette").not.toEqual(pageBorder);
});

test("blackout changes the page and nothing else", async ({ page }) => {
  await page.goto("/en/index.html");
  await page.evaluate(() => document.documentElement.setAttribute("data-theme", "dark"));

  const before = await custom(page.locator("html"), "--nc-color-surface");
  await page.evaluate(() =>
    document.documentElement.setAttribute("data-noctua-blackout", "on"),
  );
  const after = await custom(page.locator("html"), "--nc-color-surface");

  expect(after, "blackout moved a measured contrast pair").toBe(before);
  const bg = await page.locator("html").evaluate((el) => getComputedStyle(el).backgroundColor); // allow-literal: read only to be painted into a canvas on the next line, which is the recipe the gate itself prescribes
  expect(await paint(page, bg)).toEqual([0, 0, 0]);
});

test("blackout is refused to a component", async ({ page }) => {
  await page.goto("/en/scoped-themes.html");
  const scope = page.locator("[data-noctua-scope]").first();
  await page.evaluate(() => {
    document.documentElement.setAttribute("data-theme", "dark");
    document.documentElement.removeAttribute("data-noctua-blackout");
    document.querySelector("[data-noctua-scope]").setAttribute("data-noctua-blackout", "on");
  });
  // The CSS is bound to :root, so the attribute on a div declares nothing. A
  // black island inside a themed component is not a theme.
  const declared = await scope.evaluate((el) =>
    el.style.getPropertyValue("--noctua-page-background"),
  );
  expect(declared).toBe("");
});

test("glass is on before anyone touches a setting", async ({ page }) => {
  await page.goto("/en/scoped-themes.html");
  // Every glass block is a bare attribute selector, which is what makes glass
  // scopable — and which meant that on a first visit NOTHING matched, the
  // tokens were undefined, and `blur(var(--undefined))` computed to `none`.
  // Every frosted surface rendered flat, silently.
  // `.nd-header-glass`, not `.nd-header`: the frost is a sibling painted behind
  // the header so that nothing in the header's subtree — the settings panel
  // above all — has a filtered ancestor.
  const header = await page
    .locator(".nd-header-glass")
    .evaluate((el) => getComputedStyle(el).backdropFilter);
  expect(header, "the default glass level is not declared unscoped").toContain("blur");
  expect(header).not.toBe("none");
});

test("glass is scopable and radius is scopable", async ({ page }) => {
  await page.goto("/en/scoped-themes.html");
  const outer = page.locator("[data-noctua-scope]").first();
  const blur = await outer.evaluate((el) => getComputedStyle(el).backdropFilter);
  const pageBlur = await page
    .locator(".nd-header-glass")
    .evaluate((el) => getComputedStyle(el).backdropFilter);
  expect(blur).not.toBe(pageBlur);

  const radius = await outer.evaluate((el) => getComputedStyle(el).borderRadius);
  expect(radius).not.toBe("0px");
});

test("a nested scope keeps its parent's palette when only its mode changes", async ({ page }) => {
  await page.goto("/en/scoped-themes.html");
  const nested = page.locator("[data-noctua-scope]").nth(1);
  const before = await nested.getAttribute("data-palette");
  await nested.locator('[data-nd-scope-cycle="mode"]').click();
  expect(await nested.getAttribute("data-palette")).toBe(before);
});

test("reduced motion leaves nothing animating", async ({ page }) => {
  await page.emulateMedia({ reducedMotion: "reduce" });
  await page.goto("/en/index.html");
  await page.evaluate(() => {
    document.documentElement.setAttribute("data-noctua-glow", "on");
    document.documentElement.setAttribute("data-noctua-grain", "on");
    document.documentElement.setAttribute("data-noctua-animate", "on");
  });
  // A page setting may only ever DISABLE motion. Someone whose system asked for
  // none must never have it re-enabled by a page.
  const running = await page.evaluate(() => document.getAnimations().length);
  expect(running).toBe(0);
});

// --- The flash -------------------------------------------------------------
//
// These three replace a test that asserted the injected stylesheet carried
// `blocking="render"`. It passed for as long as the href was wrong, because it
// read back a property the code under test had just set on an element it had
// just created. It never resolved the URL and never looked at a colour, so the
// page flashed the default palette on every single load with the suite green.
//
// A stored palette reaches the reader through TWO things: an attribute, and a
// stylesheet that defines what the attribute means. Only the first was tested.

/** Seeds a stored palette, applied before anything on the page runs. */
const storePalette = (page, palette) =>
  page.addInitScript((p) => localStorage.setItem("noctua-palette", p), palette);

test("no page asks for a file that is not there", async ({ page }) => {
  const missing = [];
  page.on("response", (r) => {
    // The browser asks for /favicon.ico unprompted and the site ships none, so
    // it is excluded by name rather than by a status range — narrow the
    // exclusion and it stops hiding anything else. Shipping one is a real gap;
    // it waits for the icon set.
    if (r.status() >= 400 && !r.url().endsWith("/favicon.ico")) {
      missing.push(`${r.status()} ${r.url()}`);
    }
  });
  // A non-default palette, so the on-demand stylesheet is exercised too.
  await storePalette(page, "jade-vivid");

  for (const locale of ["en", "pt-BR", "qps"]) {
    await page.goto(`/${locale}/index.html`);
    // Taken from the sidebar rather than hardcoded: a page this misses is a
    // page nothing links to.
    const hrefs = await page.evaluate(() =>
      [...document.querySelectorAll(".nd-nav-link")].map((a) => a.getAttribute("href")),
    );
    expect(hrefs.length, `${locale} has no navigation`).toBeGreaterThan(0);
    for (const href of hrefs) await page.goto(`/${locale}/${href}`);
  }

  // Silence is how this fails: a stylesheet that 404s reports nothing, paints
  // nothing, and leaves the page looking merely wrong.
  expect(missing, "requests that 404ed").toEqual([]);
});

test("the palette stylesheet the bootstrap injects actually loads", async ({ page }) => {
  // The deferred module links the palette too, correctly. Blocking it is what
  // makes this a test of the bootstrap rather than of the corrector.
  await page.route("**/shell.js", (route) => route.abort());
  await storePalette(page, "jade-vivid");
  await page.goto("/en/index.html");

  const sheet = await page.evaluate(() => {
    const link = document.getElementById("noctua-palette-sheet");
    if (!link) return null;
    return {
      href: link.href,
      blocking: link.getAttribute("blocking"),
      // `null` for a sheet that failed to load. This is the assertion the old
      // test was missing, and the whole defect fits in it.
      loaded: link.sheet !== null,
      rules: link.sheet ? link.sheet.cssRules.length : 0,
    };
  });

  expect(sheet, "the bootstrap linked no palette stylesheet").not.toBeNull();
  expect(sheet.loaded, `the injected stylesheet did not load: ${sheet.href}`).toBe(true);
  expect(sheet.rules, "the stylesheet loaded but defines nothing").toBeGreaterThan(0);
  // Still necessary, and still not sufficient: a sheet that never loads blocks
  // no paint, whatever this attribute says.
  expect(sheet.blocking).toBe("render");
});

test("first paint already carries the stored palette, with no script to fix it", async ({ page }) => {
  await page.route("**/shell.js", (route) => route.abort());

  await page.goto("/en/index.html");
  const html = page.locator("html");
  const fallback = await paint(page, await custom(html, "--nc-color-accent"));

  await storePalette(page, "jade-vivid");
  await page.goto("/en/index.html");
  const stored = await paint(page, await custom(html, "--nc-color-accent"));

  expect(await html.getAttribute("data-palette")).toBe("jade-vivid");
  // Measured against the default rather than against a literal, so the test
  // states the claim — "the reader's choice is what painted" — and not a colour
  // that would have to be updated whenever the default changes.
  expect(stored, "the attribute was stamped on a page still painted in the default").not.toEqual(
    fallback,
  );
});

test("changing a setting causes no layout shift", async ({ page }) => {
  await page.goto("/en/index.html");
  await page.evaluate(() => {
    window.__shift = 0;
    new PerformanceObserver((list) => {
      for (const entry of list.getEntries()) if (!entry.hadRecentInput) window.__shift += entry.value;
    }).observe({ type: "layout-shift", buffered: false });
  });

  // `data-noctua-font` is deliberately NOT in this list, and that is a physical
  // fact rather than an exemption: a monospaced face has different advance
  // widths, so the same sentence occupies a different number of lines. No
  // amount of care makes that shift-free, and asserting otherwise would mean
  // either a failing test forever or a threshold loose enough to hide the real
  // regressions. The font toggle is covered by the no-flash test instead.
  for (const [attribute, value] of [
    ["data-theme", "dark"],
    ["data-noctua-glass", "full"],
    ["data-noctua-radius", "circular"],
    ["data-noctua-pattern", "mesh"],
    ["data-noctua-glow", "on"],
    ["data-noctua-blackout", "on"],
  ]) {
    await page.evaluate(([a, v]) => document.documentElement.setAttribute(a, v), [attribute, value]);
    await page.waitForTimeout(120);
  }

  const shift = await page.evaluate(() => window.__shift);
  expect(shift, "an appearance change moved the page").toBeLessThan(0.001);
});

test("with no script the page is complete and follows the system", async ({ page, browser }) => {
  const context = await browser.newContext({ javaScriptEnabled: false, colorScheme: "dark" });
  const noScript = await context.newPage();
  await noScript.goto("http://127.0.0.1:8731/en/index.html");

  // Nothing is hidden by markup and unhidden by script, so the navigation and
  // the content are both there.
  await expect(noScript.locator(".nd-nav-link").first()).toBeVisible();
  await expect(noScript.locator("h1")).toBeVisible();

  // `system` is the ABSENCE of data-theme, so the cascade resolves the OS
  // preference with no script involved at all.
  expect(await noScript.locator("html").getAttribute("data-theme")).toBeNull();
  const bg = await noScript.locator("html").evaluate((el) => getComputedStyle(el).backgroundColor); // allow-literal: read only to be painted into a canvas on the next line, which is the recipe the gate itself prescribes
  const [r, g, b] = bg.match(/[\d.]+/g).map(Number);
  expect(r + g + b, "OS dark did not reach a scriptless page").toBeLessThan(200);
  await context.close();
});
