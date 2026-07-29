// The shell's structural promises, measured in a real browser.
//
// Every one of these was verified by hand while the shell was built. This file
// is what makes them survive the session: a measurement nobody re-runs is a
// measurement that was true once.

import { test, expect } from "@playwright/test";
import { ANY, LOCALES, PAGE, url } from "./pages.js";

const MOBILE = { width: 390, height: 844 };
const DESKTOP = { width: 1440, height: 900 };

test.describe("the mobile drawer and its partial scrim", () => {
  test.use({ viewport: MOBILE });

  test("the scrim stops exactly where the drawer starts", async ({ page }) => {
    await page.goto(url(ANY));
    await page.locator("[data-nd-menu]").click();
    // Both transitions are 180ms; disabling them makes the geometry final
    // rather than sampled somewhere along an animation.
    await page.addStyleTag({ content: ".nd-nav, .nd-scrim { transition: none !important }" });

    const nav = await page.locator(".nd-nav").boundingBox();
    const scrim = await page.locator(".nd-scrim").boundingBox();

    // The seam is solved by the browser from one shared custom property, so it
    // is exactly zero rather than nearly zero. A calc(100vw - …) width would
    // round independently and leave a one-pixel dim line under the drawer's
    // edge — the artefact that ruins glass.
    expect(scrim.x - (nav.x + nav.width)).toBe(0);

    const overlapX = Math.max(0, Math.min(nav.x + nav.width, scrim.x + scrim.width) - Math.max(nav.x, scrim.x));
    const overlapY = Math.max(0, Math.min(nav.y + nav.height, scrim.y + scrim.height) - Math.max(nav.y, scrim.y));
    expect(overlapX * overlapY, "the scrim covers part of the drawer").toBe(0);
  });

  test("a point inside the drawer never hits the scrim", async ({ page }) => {
    await page.goto(url(ANY));
    await page.locator("[data-nd-menu]").click();
    // Without this the drawer is still sliding: 180ms of transition, and a
    // point measured mid-flight lands where the drawer is not yet. The first
    // run of this test failed for exactly that reason and the code was fine.
    await page.addStyleTag({ content: ".nd-nav, .nd-scrim { transition: none !important }" });
    const nav = await page.locator(".nd-nav").boundingBox();

    const hit = await page.evaluate(
      ([x, y]) => document.elementFromPoint(x, y)?.closest(".nd-scrim") !== null,
      [nav.x + nav.width / 2, nav.y + 40],
    );
    expect(hit, "the scrim is on top of the drawer").toBe(false);
  });

  test("the drawer ignores the reader's corner preset", async ({ page }) => {
    await page.goto(url(ANY));
    // Circular is the most extreme preset; if any radius survives, it is this.
    await page.evaluate(() =>
      document.documentElement.setAttribute("data-noctua-radius", "circular"),
    );
    await page.locator("[data-nd-menu]").click();
    // A rounded corner would cut a notch out of the drawer and show the scrim
    // through it as a dark wedge — the one place "never under the sidebar"
    // would visibly fail.
    const radius = await page.locator(".nd-nav").evaluate((el) => getComputedStyle(el).borderRadius);
    expect(radius).toBe("0px");
  });

  test("growing the viewport releases everything, with no script", async ({ page }) => {
    await page.goto(url(ANY));
    await page.locator("[data-nd-menu]").click();
    expect(await page.locator("html").getAttribute("data-nd-nav")).toBe("open");

    await page.setViewportSize(DESKTOP);

    // The open attribute is still there — nothing unwound it — and yet the
    // scrim is gone, the sidebar is docked and the scroll is released, because
    // all three are CSS. There is no listener here to miss.
    expect(await page.locator("html").getAttribute("data-nd-nav")).toBe("open");
    await expect(page.locator(".nd-scrim")).toBeHidden();
    expect(await page.locator("html").evaluate((el) => getComputedStyle(el).overflow)).toBe("visible");
    expect(await page.locator("[inert]").count(), "something was made inert").toBe(0);
  });

  test("nothing in the shell is modal", async ({ page }) => {
    await page.goto(url(ANY));
    await page.locator("[data-nd-menu]").click();
    expect(await page.locator("[aria-modal]").count()).toBe(0);
    expect(await page.locator("[inert]").count()).toBe(0);
  });
});

test.describe("the header", () => {
  for (const width of [320, 390, 768, 1440]) {
    test(`fits on one line at ${width}px`, async ({ page }) => {
      await page.setViewportSize({ width, height: 800 });
      await page.goto(url(ANY));

      for (const open of [false, true]) {
        if (open) await page.locator(".nd-settings > summary").click();
        const header = await page.locator(".nd-header").boundingBox();
        const tops = await page.locator(".nd-header > *").evaluateAll((els) =>
          els.filter((el) => el.offsetParent !== null).map((el) => el.getBoundingClientRect().top),
        );
        // One line means every visible child shares a row. Two rows would show
        // up as tops differing by more than a few pixels of baseline shift.
        const spread = Math.max(...tops) - Math.min(...tops);
        expect(spread, `panel ${open ? "open" : "closed"} at ${width}px`).toBeLessThan(24);
        expect(header.height, `header grew at ${width}px`).toBeLessThanOrEqual(80);
      }
    });
  }
});

test.describe("§9.7, enforced", () => {
  test("no element in the shell casts a shadow", async ({ page }) => {
    await page.goto("/en/scoped-themes/");
    const shadows = await page.evaluate(() =>
      [...document.querySelectorAll("*")]
        .filter((el) => getComputedStyle(el).boxShadow !== "none")
        .map((el) => el.className),
    );
    expect(shadows, "separation is a hairline border, never elevation").toEqual([]);
  });

  test("exactly four kinds of floating surface exist", async ({ page }) => {
    await page.setViewportSize(MOBILE);
    await page.goto("/en/scoped-themes/");
    await page.locator("[data-nd-menu]").click();
    await page.locator(".nd-settings > summary").click();

    const allowed = ["nd-header", "nd-nav", "nd-panel", "nd-scope", "nd-skip", "nd-scrim"];
    const floating = await page.evaluate((allowed) => {
      return [...document.querySelectorAll("*")]
        .filter((el) => {
          const s = getComputedStyle(el);
          return (
            ["fixed", "sticky", "absolute"].includes(s.position) &&
            s.backgroundColor !== "rgba(0, 0, 0, 0)" && // allow-literal: the CSS keyword for "no background", not a colour this system could take from a token
            el.offsetParent !== null
          );
        })
        .map((el) => el.className)
        .filter((name) => !allowed.some((ok) => String(name).includes(ok)));
    }, allowed);

    // A fifth floating surface has to be named and justified here before it can
    // ship. That is the whole mechanism.
    expect(floating, "an unlisted floating surface appeared").toEqual([]);
  });

  test("the measure holds however wide the page gets", async ({ page }) => {
    await page.setViewportSize({ width: 2560, height: 900 });
    await page.goto(url(ANY));
    const widths = await page.locator(".nd-prose").evaluateAll((els) =>
      els.map((el) => el.getBoundingClientRect().width),
    );
    // The page is as wide as the screen; the LINE is not.
    for (const width of widths) expect(width).toBeLessThan(900);
    const main = await page.locator(".nd-main").boundingBox();
    expect(main.width, "the measure was applied to main instead of the prose").toBeGreaterThan(1200);
  });

  test("a sidebar row never grows a background or reflows", async ({ page }) => {
    await page.goto(url(ANY));
    const link = page.locator(".nd-nav-link").first();
    const before = await link.evaluate((el) => ({
      bg: getComputedStyle(el).backgroundColor, // allow-literal: compared for EQUALITY against another serialisation of the same computation, never for its value
      weight: getComputedStyle(el.querySelector(".nd-nav-text")).fontWeight,
      width: el.querySelector(".nd-nav-text").getBoundingClientRect().width,
    }));
    await link.hover();
    const after = await link.evaluate((el) => ({
      bg: getComputedStyle(el).backgroundColor, // allow-literal: compared for EQUALITY against another serialisation of the same computation, never for its value
      weight: getComputedStyle(el.querySelector(".nd-nav-text")).fontWeight,
      width: el.querySelector(".nd-nav-text").getBoundingClientRect().width,
    }));
    expect(after.bg).toBe(before.bg);
    // font-weight on hover reflows the line under the pointer, which makes the
    // whole list jitter as the cursor crosses it.
    expect(after.weight).toBe(before.weight);
    expect(after.width).toBeCloseTo(before.width, 1);
  });
});
