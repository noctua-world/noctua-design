// The shell's structural promises, measured in a real browser.
//
// Every one of these was verified by hand while the shell was built. This file
// is what makes them survive the session: a measurement nobody re-runs is a
// measurement that was true once.

import { test, expect } from "@playwright/test";
import { ANY, LOCALES, PAGE, SCOPED, url } from "./pages.js";

const MOBILE = { width: 390, height: 844 };
const DESKTOP = { width: 1440, height: 900 };

test.describe("the mobile drawer and its partial scrim", () => {
  test.use({ viewport: MOBILE });

  test("the scrim stops exactly where the drawer starts", async ({ page }) => {
    await page.goto(url(ANY));
    await page.locator('[data-nd-action="menu"]').click();
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
    await page.locator('[data-nd-action="menu"]').click();
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
    await page.locator('[data-nd-action="menu"]').click();
    // A rounded corner would cut a notch out of the drawer and show the scrim
    // through it as a dark wedge — the one place "never under the sidebar"
    // would visibly fail.
    const radius = await page.locator(".nd-nav").evaluate((el) => getComputedStyle(el).borderRadius);
    expect(radius).toBe("0px");
  });

  test("growing the viewport releases everything, with no script", async ({ page }) => {
    await page.goto(url(ANY));
    await page.locator('[data-nd-action="menu"]').click();
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
    await page.locator('[data-nd-action="menu"]').click();
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
    await page.goto(url(SCOPED));
    const shadows = await page.evaluate(() =>
      [...document.querySelectorAll("*")]
        .filter((el) => getComputedStyle(el).boxShadow !== "none")
        .map((el) => el.className),
    );
    expect(shadows, "separation is a hairline border, never elevation").toEqual([]);
  });

  test("exactly four kinds of floating surface exist", async ({ page }) => {
    await page.setViewportSize(MOBILE);
    await page.goto(url(SCOPED));
    await page.locator('[data-nd-action="menu"]').click();
    await page.locator(".nd-settings > summary").click();

    // `nd-frame-bar` is listed and no longer floats: it was sticky under the
    // header so that the controls of a frame several screens tall stayed in
    // reach, and it is `relative` now — it stays at the top of its own frame.
    // The entry stays because the list is what this test is for, and an
    // allow-list that quietly loses an entry is one nobody notices growing.
    const allowed = [
      "nd-header",
      "nd-nav",
      "nd-panel",
      "nd-scope",
      "nd-skip",
      "nd-scrim",
      "nd-frame-bar",
      // A SIXTH KIND, NAMED AND JUSTIFIED.
      //
      // `nd-frame-glass`: the material a frame is frosted with, on a layer of
      // its own rather than on the frame. `backdrop-filter` makes an element
      // the containing block for every `position: fixed` descendant, and a
      // frame contains its own theme menu — frosting the frame itself moved
      // that panel inside a box that clips and it opened where nobody could
      // see it. It floats because it has to cover the frame it belongs to; it
      // is `pointer-events: none`, so it is not in anybody's way; and it
      // carries no shadow, like everything else here.
      "nd-frame-glass",
      // `nd-frame-theme-panel` used to be listed here — the theme menu as a
      // band inside the frame. It was 457px tall against a frame that can be
      // 172px, so it covered every example and `overflow: clip` cut off the
      // rows it could not fit. The frame opens `nd-panel` now, already listed,
      // which is the same surface the header opens.
      //
      // --- THE FOUR COMPONENTS THAT ARE FLOATING SURFACES ------------------
      //
      // Everything above this line belongs to the documentation shell. These
      // four are components a consumer renders, and each one is named here
      // because it cannot do its job in the flow of the page.
      //
      // `nd-dialog`: the surface a `<dialog>` draws, which covers both classes
      // that carry it — `nd-dialog-surface` and the wrapper the trigger lives
      // in. A modal one is in the browser's top layer, which is `fixed` by
      // definition and cannot be anything else; a non-modal one is `absolute`
      // against the control that opened it. It carries a hairline and the
      // raised surface colour, and no shadow.
      "nd-dialog",
      // `nd-popover`: a `role="dialog"` anchored to its trigger. It is `fixed`
      // for `anchor.js`'s reason, stated at the top of that file: a panel its
      // own ancestor cannot clip is a panel CSS has nothing to hang it from,
      // and `overflow` does not clip a fixed descendant. Eight surfaces before
      // it float for exactly this and are already listed above.
      "nd-popover",
      // `nd-tooltip`: the hint bubble, at the tooltip layer so it stays legible
      // over a control that is being used. Same anchoring, same reason, plus
      // one of its own: it is `pointer-events: none`, so it is never between a
      // pointer and the control it is about.
      "nd-tooltip",
      // `nd-toast`: a message pinned to a corner of the window, and the ONLY
      // entry on this list that is visible with nothing pressed. That matters
      // here, because this sweep only sees elements whose `offsetParent` is not
      // null — a surface that opens on an interaction is invisible to it and
      // this one is not. It floats because it reports something that happened
      // somewhere else on the page, and it is bounded by `contain: layout` on
      // the documentation site so four of them do not stack in one corner.
      "nd-toast",
    ];
    const floating = await page.evaluate((allowed) => {
      return [...document.querySelectorAll("*")]
        .filter((el) => {
          const s = getComputedStyle(el);
          return (
            ["fixed", "sticky", "absolute"].includes(s.position) &&
            s.backgroundColor !== "rgba(0, 0, 0, 0)" && // allow-literal: the CSS keyword for "no background", not a color this system could take from a token
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
