// The carousel, and the claims only a browser can settle.
//
// Its behaviour SHIPS — `dist/carousel.js` — so what is asserted here is
// asserted about a file a consumer of the packages receives.
//
// THE ONE COMPONENT ON THIS SITE THAT MOVES ON ITS OWN, which is why half of
// this file is about how it stops.

import { expect, test } from "@playwright/test";

import { LOCALES, PAGE, url, showParts } from "./pages.js";

const GALLERY = '[data-nd-carousel="gallery"]';
const SHELF = '[data-nd-carousel="shelf"]';
const COUNTED = '[data-nd-carousel="counted"]';
const ROTATING = '[data-nd-carousel="rotating"]';
const LOOPING = '[data-nd-carousel="looping"]';

// --- What ships, and what the script adds ------------------------------------

test("with the script blocked it is a real list in a real scroll container", async ({ browser }) => {
  const context = await browser.newContext({ javaScriptEnabled: false });
  const page = await context.newPage();
  await page.goto(url(PAGE.carousel));
  await showParts(page);

  // EVERY ONE OF THESE IS THE PLATFORM'S. Dragging, flicking, the trackpad,
  // the scrollbar and Home/End all come from the track being a scroll box —
  // and every hand-built slideshow this avoids gets one of them wrong.
  const track = page.locator(`${GALLERY} [data-nd-carousel-track]`);
  await expect(track).toBeVisible();
  expect(await track.evaluate((el) => el.tagName)).toBe("UL");
  expect(await track.evaluate((el) => getComputedStyle(el).overflowX)).toBe("auto");
  expect(await track.evaluate((el) => el.scrollWidth > el.clientWidth)).toBe(true);
  await expect(page.locator(`${GALLERY} [data-nd-carousel-slide]`)).toHaveCount(5);

  // And nothing claims what it cannot keep: every control the script owns is
  // rendered hidden.
  await expect(page.locator(`${GALLERY} [data-nd-carousel-step]`).first()).toBeHidden();
  await expect(page.locator(`${COUNTED} [data-nd-carousel-dots]`)).toBeHidden();
  await expect(page.locator(`${COUNTED} [data-nd-carousel-counter]`)).toBeHidden();
  await context.close();
});

test("it announces itself as a carousel, and says what is in it", async ({ page }) => {
  await page.goto(url(PAGE.carousel));
  await showParts(page);
  const root = page.locator(GALLERY);
  expect(await root.getAttribute("role")).toBe("group");
  // *Carousel* alone gives a reader the shape and none of the contents, which
  // is the half they need to decide whether to walk it.
  expect(await root.getAttribute("aria-roledescription")).toBe("carousel");
  expect(await root.getAttribute("aria-label")).toBeTruthy();
});

test("every slide is named by its position, counted from the slides that are there", async ({
  page,
}) => {
  await page.goto(url(PAGE.carousel));
  await showParts(page);
  const names = await page
    .locator(`${GALLERY} [data-nd-carousel-slide]`)
    .evaluateAll((all) => all.map((one) => one.getAttribute("aria-label")));
  // A count a caller wrote goes wrong the first time a slide is added, so the
  // runtime writes it — and a `label` of their own is joined to it rather than
  // replacing it, because losing the position to gain a description is a bad
  // trade.
  expect(names[0]).toContain("1");
  expect(names[0]).toContain("5");
  expect(names[0]).toContain("kettle");
  expect(new Set(names).size).toBe(5);
  for (const slide of await page.locator(`${GALLERY} [data-nd-carousel-slide]`).all()) {
    expect(await slide.getAttribute("aria-roledescription")).toBe("slide");
  }
});

// --- Moving ------------------------------------------------------------------

test("the controls scroll the track, and are disabled at the ends", async ({ page }) => {
  await page.goto(url(PAGE.carousel));
  await showParts(page);
  const track = page.locator(`${GALLERY} [data-nd-carousel-track]`);
  const previous = page.locator(`${GALLERY} [data-nd-carousel-step="previous"]`);
  const next = page.locator(`${GALLERY} [data-nd-carousel-step="next"]`);
  await expect(next).toBeVisible();
  // An end a reader can feel is how they learn how long the row is.
  await expect(previous).toBeDisabled();

  await next.click();
  await expect.poll(() => track.evaluate((el) => el.scrollLeft)).toBeGreaterThan(0);
  await expect(previous).toBeEnabled();
});

test("it scrolls rather than transforming, so the platform keeps working", async ({ page }) => {
  await page.goto(url(PAGE.carousel));
  await showParts(page);
  const track = page.locator(`${GALLERY} [data-nd-carousel-track]`);
  await page.locator(`${GALLERY} [data-nd-carousel-step="next"]`).click();
  await expect.poll(() => track.evaluate((el) => el.scrollLeft)).toBeGreaterThan(0);
  // NOT A TRANSFORM AND NOT A `left`. The moment either of those is how a
  // slide moves, dragging, flicking, the trackpad and the scrollbar stop
  // agreeing with the controls.
  const moved = await track.evaluate((el) => {
    const style = getComputedStyle(el);
    return { transform: style.transform, left: style.left };
  });
  expect(moved.transform === "none" || moved.transform === "matrix(1, 0, 0, 1, 0, 0)").toBe(true);
});

test("a slide arriving is announced, and nothing takes the focus", async ({ page }) => {
  await page.goto(url(PAGE.carousel));
  await showParts(page);
  const next = page.locator(`${GALLERY} [data-nd-carousel-step="next"]`);
  await next.click();
  await expect(page.locator(`${GALLERY} [data-nd-carousel-status]`)).toContainText("2");
  // A carousel that focuses the slide it just brought in tears a reader out of
  // wherever they were.
  expect(await next.evaluate((el) => el === document.activeElement)).toBe(true);
});

test("wrap turns the ends into a loop, and without it they stay ends", async ({ page }) => {
  await page.goto(url(PAGE.carousel));
  await showParts(page);
  // A control that wraps is never at an end, so it is never disabled.
  await expect(page.locator(`${LOOPING} [data-nd-carousel-step="previous"]`)).toBeEnabled();
  await expect(page.locator(`${GALLERY} [data-nd-carousel-step="previous"]`)).toBeDisabled();
});

// --- Saying where it is ------------------------------------------------------

test("the dots are real buttons, named, and one carries the position", async ({ page }) => {
  await page.goto(url(PAGE.carousel));
  await showParts(page);
  const dots = page.locator(`${COUNTED} [data-nd-carousel-dot]`);
  await expect(dots).toHaveCount(5);
  const names = await dots.evaluateAll((all) => all.map((one) => one.getAttribute("aria-label")));
  // A row of unnamed circles is a row a screen reader reads as *button,
  // button, button*.
  expect(names.every(Boolean)).toBe(true);
  expect(new Set(names).size).toBe(5);
  await expect(dots.first()).toHaveAttribute("aria-current", "true");

  await dots.nth(2).click();
  await expect(dots.nth(2)).toHaveAttribute("aria-current", "true");
  await expect(dots.first()).toHaveAttribute("aria-current", "false");
});

test("the one in force is not only a colour", async ({ page }) => {
  await page.goto(url(PAGE.carousel));
  await showParts(page);
  const dots = page.locator(`${COUNTED} [data-nd-carousel-dot]`);
  // `NOCTUA.md §8.8.6`: never by colour alone. The dot in force is wider, and
  // it is the only one carrying `aria-current`.
  const widths = await dots.evaluateAll((all) =>
    all.map((one) => getComputedStyle(one, "::before").inlineSize),
  );
  expect(new Set(widths).size).toBeGreaterThan(1);
});

test("the counter says how many, which a dot cannot", async ({ page }) => {
  await page.goto(url(PAGE.carousel));
  await showParts(page);
  const counter = page.locator(`${COUNTED} [data-nd-carousel-counter]`);
  await expect(counter).toBeVisible();
  await expect(counter).toContainText("5");
});

// --- Autoplay, and the four rules it carries ---------------------------------

test("it refuses to rotate without a named pause control", async ({ page }) => {
  await page.goto(url(PAGE.carousel));
  await showParts(page);
  // WCAG 2.2.2 wants a way to stop anything that moves for more than five
  // seconds. Making that a condition of STARTING is the only version of the
  // rule that cannot be forgotten — so a carousel with an interval and no
  // `pause_label` is a carousel that never moves on its own.
  const refused = await page.evaluate(() => {
    const rotating = [...document.querySelectorAll("[data-nd-autoplay]")];
    return rotating
      .filter((one) => !one.getAttribute("data-nd-pause-label"))
      .every((one) => one.querySelector("[data-nd-carousel-rotation]") === null);
  });
  expect(refused).toBe(true);
});

test("the one that does rotate moves on its own, and stops when told", async ({ page }) => {
  await page.goto(url(PAGE.carousel));
  await showParts(page);
  const track = page.locator(`${ROTATING} [data-nd-carousel-track]`);
  const rotation = page.locator(`${ROTATING} [data-nd-carousel-rotation]`);
  await expect(rotation).toBeVisible();
  // The region goes quiet while it rotates: something that speaks every four
  // seconds unasked makes the rest of the page unusable.
  await expect(track).toHaveAttribute("aria-live", "off");

  await expect.poll(() => track.evaluate((el) => el.scrollLeft), { timeout: 12000 }).toBeGreaterThan(0);

  const before = await rotation.getAttribute("aria-label");
  await rotation.click();
  await expect(page.locator(ROTATING)).toHaveAttribute("data-nd-paused", "");
  await expect(rotation).toHaveAttribute("aria-pressed", "true");
  // A control that says *stop* while it is stopped has lied about a state.
  expect(await rotation.getAttribute("aria-label")).not.toBe(before);
  await expect(track).toHaveAttribute("aria-live", "polite");
});

test("touching any control stops it for good", async ({ page }) => {
  await page.goto(url(PAGE.carousel));
  await showParts(page);
  const track = page.locator(`${ROTATING} [data-nd-carousel-track]`);
  // Something that resumes under a reader who has taken over is something they
  // have to fight.
  await page.locator(`${ROTATING} [data-nd-carousel-step="next"]`).click();
  await expect(page.locator(ROTATING)).toHaveAttribute("data-nd-paused", "");
  // The press itself scrolls, and smoothly — so the reading has to wait for
  // that to finish before it can mean "and then nothing moved it".
  let settled = -1;
  await expect
    .poll(async () => {
      const now = await track.evaluate((el) => el.scrollLeft);
      const same = now === settled;
      settled = now;
      return same;
    })
    .toBe(true);
  await page.waitForTimeout(5000);
  expect(await track.evaluate((el) => el.scrollLeft)).toBe(settled);
});

test("it never starts under reduced motion", async ({ browser }) => {
  const context = await browser.newContext({ reducedMotion: "reduce" });
  const page = await context.newPage();
  await page.goto(url(PAGE.carousel));
  await showParts(page);
  const track = page.locator(`${ROTATING} [data-nd-carousel-track]`);
  // A reader who has asked for less movement has asked in as many words, and
  // the control that would toggle a rotation which never happens is removed
  // rather than left as a button that does nothing.
  await expect(page.locator(`${ROTATING} [data-nd-carousel-rotation]`)).toHaveCount(0);
  await page.waitForTimeout(5000);
  expect(await track.evaluate((el) => el.scrollLeft)).toBe(0);
  await context.close();
});

// --- The shell it sits in ----------------------------------------------------

test("the track is a tab stop only when nothing inside it is", async ({ page }) => {
  await page.goto(url(PAGE.carousel));
  await showParts(page);
  // A scroll container with no focusable content is a region a keyboard cannot
  // scroll; one that is focusable AND full of links is an extra press before
  // every slide.
  expect(
    await page.locator(`${GALLERY} [data-nd-carousel-track]`).evaluate((el) => el.tabIndex),
  ).toBe(0);
  expect(
    await page
      .locator('[data-nd-carousel="with-links"] [data-nd-carousel-track]')
      .evaluate((el) => el.tabIndex),
  ).toBe(-1);
});

test("the shelf lets each card keep its own width", async ({ page }) => {
  await page.goto(url(PAGE.carousel));
  await showParts(page);
  // `per_view: auto` is the shape that survives a telephone — a three-across
  // grid squeezed onto a narrow screen is three unreadable cards.
  const widths = await page
    .locator(`${SHELF} [data-nd-carousel-slide]`)
    .evaluateAll((all) => all.map((one) => Math.round(one.getBoundingClientRect().width)));
  expect(new Set(widths).size).toBeGreaterThan(1);
});

test("no id appears twice on the page, in any locale", async ({ page }) => {
  for (const locale of LOCALES) {
    await page.goto(url(PAGE.carousel, locale));
    await showParts(page);
    const twice = await page.evaluate(() => {
      const seen = new Map();
      for (const element of document.querySelectorAll("[id]")) {
        seen.set(element.id, (seen.get(element.id) ?? 0) + 1);
      }
      return [...seen].filter(([, count]) => count > 1).map(([id]) => id);
    });
    expect(twice, locale).toEqual([]);
  }
});
