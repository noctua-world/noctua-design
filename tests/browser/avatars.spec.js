// A person, and several of them.
//
// The whole of `Avatar` is a fallback that has to work with no script — this
// package ships no runtime, so there is no `onerror` to listen to and the
// picture, the letters and the icon are three layers rather than three
// branches. Most of what follows is that claim, measured.
//
// `AvatarGroup` is the one component here that cannot count its own children,
// so the tests are about what it does instead: a real list with a real name,
// and a counter that is one of the items rather than a decoration beside them.

import { test, expect } from "@playwright/test";
import { LOCALES, PAGE, url, showParts } from "./pages.js";
import { extremes } from "./pixels.js";

test("the picture is laid over the letters, not instead of them", async ({ page }) => {
  await page.goto(url(PAGE.avatars));
  await showParts(page);
  // NOT DECORATIVE. The page opens on an overview whose first avatar stands
  // beside the name it belongs to, so it carries no label of its own — which
  // is the contract, not a defect, and the wrong subject for this test.
  const avatar = page
    .locator(".nd-avatar[aria-label]:has(.nd-avatar-image)")
    .first();
  await avatar.scrollIntoViewIfNeeded();

  // All three layers are in the markup at once. That is what makes the
  // fallback a stacking order: nothing here chose between them.
  await expect(avatar.locator(".nd-avatar-icon")).toHaveCount(1);
  await expect(avatar.locator(".nd-avatar-initials")).toHaveCount(1);
  await expect(avatar.locator(".nd-avatar-image")).toHaveCount(1);

  const shot = async () => {
    const box = await avatar.boundingBox();
    return (await extremes(page, await page.screenshot({ clip: box }))).min.join(",");
  };
  const withPicture = await shot();

  // Break it the way the network does. Read from the pixels, because the
  // question is what a reader sees and not what the DOM says.
  await avatar.locator(".nd-avatar-image").evaluate((el) => {
    el.src = `${el.src}.not-a-file`;
  });
  await expect
    .poll(() => avatar.locator(".nd-avatar-image").evaluate((el) => el.naturalWidth))
    .toBe(0);
  expect(await shot(), "the letters did not show through").not.toBe(withPicture);

  // And the box did not move, because it was never the picture's size.
  const box = await avatar.boundingBox();
  expect(box.width).toBe(box.height);
});

test("one person has one name, and it is not on the picture", async ({ page }) => {
  await page.goto(url(PAGE.avatars));
  await showParts(page);
  // NAMED, AND SHOWING A PICTURE — both halves stated rather than one of them
  // assumed. The page opens on an overview whose avatar is `decorative`: a
  // picture standing beside the name it belongs to, which by contract carries
  // no name of its own. That is the rule, not a defect, and the wrong subject
  // for a test about what a named avatar announces.
  const avatar = page.locator(".nd-avatar[aria-label]:has(.nd-avatar-image)").first();
  // An `<img>` with its own alternative text beside a wrapper with a label is
  // two names for one person, and a reader hears both. The empty alt is also
  // what stops a broken image drawing a glyph and its own words.
  await expect(avatar.locator(".nd-avatar-image")).toHaveAttribute("alt", "");
  expect(await avatar.getAttribute("aria-label")).toBeTruthy();
  await expect(avatar).toHaveAttribute("role", "img");
  // The letters are a picture of the name. A reader who heard *Ana Lima* does
  // not need *AL* after it.
  await expect(avatar.locator(".nd-avatar-initials")).toHaveAttribute("aria-hidden", "true");
});

test("a decorative avatar is hidden and is not a control", async ({ page }) => {
  await page.goto(url(PAGE.avatars));
  await showParts(page);
  const decorative = page.locator('.nd-avatar[aria-hidden="true"]').first();
  await expect(decorative).toHaveCount(1);
  // No name, no role — it is a picture of a word that is on the screen beside
  // it, and announcing it says the same thing twice.
  expect(await decorative.getAttribute("aria-label")).toBeNull();
  expect(await decorative.getAttribute("role")).toBeNull();
  // And it is not an `<a>`: a focusable element inside `aria-hidden` is what
  // axe calls `aria-hidden-focus`, a control a keyboard reaches and a screen
  // reader cannot describe.
  expect(await decorative.evaluate((el) => el.tagName)).toBe("SPAN");
});

test("the group is a named list and the counter is one of its items", async ({ page }) => {
  await page.goto(url(PAGE.avatars));
  await showParts(page);
  const group = page.locator(".nd-avatar-group:has(.nd-avatar-more)").first();
  expect(await group.evaluate((el) => el.tagName)).toBe("UL");
  expect(await group.getAttribute("aria-label")).toBeTruthy();

  // A list of four faces and a chip beside it would announce *list, 4 items*
  // while standing for nine people. The chip is the fifth item, so the number
  // the browser says is true.
  const items = await group.locator("> li").count();
  const faces = await group.locator("> li .nd-avatar").count();
  expect(items).toBe(faces + 1);
  // And the counter says how many people it stands for, in words.
  const said = group.locator(".nd-avatar-more .nd-visually-hidden, .nd-avatar-more a");
  expect((await said.first().getAttribute("aria-label")) ?? (await said.first().textContent()))
    .toBeTruthy();
});

test("a counter with nowhere to go is not focusable", async ({ page }) => {
  await page.goto(url(PAGE.avatars));
  await showParts(page);
  // A chip a reader can tab to and press, which does nothing, is the control
  // that lies — the defect this repository keeps recording.
  for (const chip of await page.locator(".nd-avatar-more-chip").all()) {
    const tag = await chip.evaluate((el) => el.tagName);
    if (tag === "A") expect(await chip.getAttribute("href")).toBeTruthy();
    else expect(await chip.evaluate((el) => el.tabIndex)).toBe(-1);
  }
});

test("the group's size wins over an avatar's own", async ({ page }) => {
  await page.goto(url(PAGE.avatars));
  await showParts(page);
  // Siblings share their box. A row of faces that disagreed about theirs would
  // read as a mistake rather than as a hierarchy.
  for (const group of await page.locator(".nd-avatar-group:has(.nd-avatar)").all()) {
    const widths = await group.locator(".nd-avatar").evaluateAll((els) =>
      els.map((el) => Math.round(el.getBoundingClientRect().width)),
    );
    expect(new Set(widths).size, `a group draws ${widths.join("/")}`).toBe(1);
  }
});

test("every face in a group keeps a strip a pointer can reach", async ({ page }) => {
  await page.goto(url(PAGE.avatars));
  await showParts(page);
  const group = page.locator('.nd-avatar-group[data-nd-overlap="tight"]').first();
  const strips = await group.evaluate((el) => {
    const boxes = [...el.querySelectorAll(".nd-avatar")].map((a) => a.getBoundingClientRect());
    // What each face leaves uncovered by the one after it.
    return boxes.map((box, at) => (at + 1 < boxes.length ? boxes[at + 1].left - box.left : box.width));
  });
  // TWENTY-FOUR PIXELS, WHICH IS THE NUMBER THIS COMMENT ALWAYS NAMED. The
  // assertion under it read `> 0`, so a crescent one pixel wide passed a test
  // about a target a pointer can hit. WCAG 2.5.8 asks for 24, and the tightest
  // overlap this component offers still clears it at every size on the page.
  expect(strips.length, "the group this is about lost its faces").toBeGreaterThan(2);
  for (const strip of strips) {
    expect(strip, `a face leaves ${Math.round(strip)}px to aim at`).toBeGreaterThanOrEqual(24);
  }
});

test.describe("with the script blocked", () => {
  test.use({ javaScriptEnabled: false });

  test("nothing here needed a script in the first place", async ({ page }) => {
    await page.goto(url(PAGE.avatars));
    await showParts(page);
    await expect(page.locator(".nd-avatar").first()).toBeVisible();
    await expect(page.locator(".nd-avatar-group").first()).toBeVisible();
    // The fallback included: it is a stacking order, and CSS is all of it.
    await expect(page.locator(".nd-avatar-initials").first()).toBeVisible();
  });
});

test("no id appears twice on the avatars page, in any locale", async ({ page }) => {
  for (const locale of LOCALES) {
    await page.goto(url(PAGE.avatars, locale));
    await showParts(page);
    const twice = await page.evaluate(() => {
      const seen = new Set();
      const out = [];
      for (const el of document.querySelectorAll("[id]")) {
        if (seen.has(el.id)) out.push(el.id);
        seen.add(el.id);
      }
      return out;
    });
    expect(twice, locale).toEqual([]);
  }
});
