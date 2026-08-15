// The pill and the count, and the two things neither of them may become.
//
// `Tag` is the only component in this system that renders as three different
// elements depending on what it was asked for, so most of what is worth
// asserting is *which one it became* — a label that is secretly a button is a
// tab stop a reader lands on and cannot use, and a link painted as a `<span>`
// loses the middle mouse button, the context menu and *copy link address*.
//
// `Badge` is the only one whose whole accessible content is a sentence nobody
// can see, so what is worth asserting is that the sentence is there and that
// the digits are not what gets announced.
//
// Neither needs a line of script, and one of these tests runs with none.

import { test, expect } from "@playwright/test";
import { LOCALES, PAGE, url, showParts } from "./pages.js";
import { extremes } from "./pixels.js";

const TAG = ".nd-tag";

test("a tag with nothing asked of it is not a control", async ({ page }) => {
  await page.goto(url(PAGE.tag));
  await showParts(page);
  // The first row is four plain labels.
  const plain = page.locator(`${TAG}:not(:has(a)):not(:has(button))`).first();
  const body = plain.locator(".nd-tag-body");
  expect(await body.evaluate((el) => el.tagName)).toBe("SPAN");
  // No role, no tab stop, and no cursor claiming otherwise.
  expect(await body.getAttribute("role")).toBeNull();
  expect(await body.evaluate((el) => el.tabIndex)).toBe(-1);
  expect(await body.evaluate((el) => getComputedStyle(el).cursor)).toBe("auto");
});

test("a tag that leads somewhere is a real link", async ({ page }) => {
  await page.goto(url(PAGE.tag));
  await showParts(page);
  const link = page.locator(`${TAG}:has(> a.nd-tag-body)`).first().locator("a");
  // A `<span>` with a handler has none of this: no `href` to copy, no entry in
  // a screen reader's list of links, no middle-click.
  expect(await link.evaluate((el) => el.tagName)).toBe("A");
  expect(await link.getAttribute("href")).toBeTruthy();
});

test("a tag that toggles is a real button, and says which way", async ({ page }) => {
  await page.goto(url(PAGE.tag));
  await showParts(page);
  const on = page.locator(`${TAG}:has([aria-pressed="true"])`).first();
  const off = page.locator(`${TAG}:has([aria-pressed="false"])`).first();
  expect(await on.locator(".nd-tag-body").evaluate((el) => el.tagName)).toBe("BUTTON");

  // THE STATE IS VISIBLE, NOT ONLY ANNOUNCED. The twelve colour rules are two
  // attributes' worth of specificity each, so the pressed rule needs its own —
  // and a state that loses to the variant it overrides is a state nothing can
  // see. Read from the pixels: an engine serialises a computed colour in
  // whatever space it likes, and comparing two of those strings proves nothing.
  const painted = async (tag) => {
    // Into view first: a clip outside the viewport is not a screenshot.
    await tag.scrollIntoViewIfNeeded();
    const box = await tag.boundingBox();
    // A strip at the pill's vertical middle, in its leading padding — clear of
    // the word and, on a pill, clear of the rounded corner. Sampling near the
    // top-left corner reads the page through it, which is how both a blue tag
    // and a grey one came back white.
    const clip = { x: box.x + 6, y: box.y + box.height / 2 - 2, width: 4, height: 4 };
    return (await extremes(page, await page.screenshot({ clip }))).min.join(",");
  };
  expect(await painted(on)).not.toBe(await painted(off));
});

test("the remove control is a sibling of the word, not a wrapper round it", async ({ page }) => {
  await page.goto(url(PAGE.tag));
  await showParts(page);
  const tag = page.locator(`${TAG}:has(.nd-tag-remove)`).first();
  // A `<button>` inside an `<a>` is invalid and one inside a `<button>` is
  // unreachable in several engines, so the two are siblings.
  await expect(tag.locator(".nd-tag-body .nd-tag-remove")).toHaveCount(0);
  await expect(tag.locator("> .nd-tag-remove")).toHaveCount(1);

  // And it is named after what it removes, not just "Remove".
  const name = await tag.locator(".nd-tag-remove").getAttribute("aria-label");
  const word = (await tag.locator(".nd-tag-label").textContent()).trim();
  expect(name).toContain(word);
});

test("the cross does not sit on top of the last letter", async ({ page }) => {
  await page.goto(url(PAGE.tag));
  await showParts(page);
  // THE DEFECT: the remove control ate the leading padding as well as the
  // trailing one, which put it eight pixels over the word — measured, and
  // visible in a screenshot as `Draft✕`.
  for (const tag of await page.locator(`${TAG}:has(.nd-tag-remove)`).all()) {
    const gap = await tag.evaluate((el) => {
      const label = el.querySelector(".nd-tag-label").getBoundingClientRect();
      const cross = el.querySelector(".nd-tag-remove").getBoundingClientRect();
      return cross.left - label.right;
    });
    expect(gap, `the cross overlaps the word by ${-gap}px`).toBeGreaterThan(0);
  }
});

test("everything drawn beside the word is hidden from a reader", async ({ page }) => {
  await page.goto(url(PAGE.tag));
  await showParts(page);
  // Each of them is a picture of what the word already says. `Swatch`'s own
  // note is the general rule: a chip that announced its palette would say the
  // same word twice.
  for (const selector of [".nd-tag-glyph", ".nd-tag-image", ".nd-tag .nd-swatch"]) {
    const drawings = await page.locator(selector).all();
    expect(drawings.length, `${selector} is in no example`).toBeGreaterThan(0);
    for (const drawing of drawings) {
      const hidden = await drawing.evaluate(
        (el) => el.getAttribute("aria-hidden") === "true" || el.getAttribute("alt") === "",
      );
      expect(hidden, `${selector} is announced`).toBe(true);
    }
  }
});

test("a picture that is not there keeps its place and draws nothing", async ({ page }) => {
  await page.goto(url(PAGE.tag));
  await showParts(page);
  const tag = page.locator(`${TAG}:has(.nd-tag-image)`).first();
  const before = await tag.boundingBox();

  // THE BREAK IS MADE HERE RATHER THAN SHIPPED. An example pointing at a URL
  // nothing answers would be a permanent 404 on a published page, and
  // `theme.spec.js` refuses those in writing — "an exclusion is a 404 somebody
  // decided to stop looking at". The claim is about the stylesheet anyway:
  // there is no script to catch `onerror` with, so the fallback is CSS or it
  // is nothing.
  await tag.locator(".nd-tag-image").evaluate((el) => {
    el.src = `${el.src}.not-a-file`;
  });
  await expect
    .poll(() => tag.locator(".nd-tag-image").evaluate((el) => el.naturalWidth))
    .toBe(0);

  // No broken-image glyph — that is what the empty `alt` buys — and the box is
  // still the size the stylesheet stated, so the word did not move.
  const after = await tag.boundingBox();
  expect(after.width).toBe(before.width);
  expect(
    await tag.locator(".nd-tag-image").evaluate((el) => el.getBoundingClientRect().width),
  ).toBeGreaterThan(0);
});

// --- Badge ------------------------------------------------------------------

test("the digits are decoration and the sentence is the message", async ({ page }) => {
  await page.goto(url(PAGE.badge));
  await showParts(page);
  const badge = page.locator(".nd-badge").first();
  await expect(badge.locator(".nd-badge-mark")).toHaveAttribute("aria-hidden", "true");
  // A reader hears the control's own name and then the sentence: *Inbox, 3
  // unread messages*. Announcing `3` after a button's name is a page that has
  // said a number and nothing else.
  const said = await badge.locator(".nd-visually-hidden").textContent();
  expect(said.trim().length, "the badge announces nothing but digits").toBeGreaterThan(3);
});

test("a count over the ceiling says so rather than growing", async ({ page }) => {
  await page.goto(url(PAGE.badge));
  await showParts(page);
  const marks = await page.locator(".nd-badge-mark").allTextContents();
  // `count: "128", max: "99"`. A four-digit badge is wider than the control it
  // marks, and the difference between 128 and 4 216 is not what a corner is for.
  expect(marks).toContain("99+");
  expect(marks.join(" ")).not.toContain("128");
});

test("nothing to report is reported by an absence", async ({ page }) => {
  await page.goto(url(PAGE.badge));
  await showParts(page);
  // Two badges with `count: "0"`, one of which asked for `show_zero`.
  const zeros = page.locator('.nd-badge[data-nd-count="0"]');
  await expect(zeros).toHaveCount(2);
  const marked = await zeros.evaluateAll((els) => els.map((el) => el.dataset.ndMarked));
  expect(marked.sort()).toEqual(["false", "true"]);
  // And the one that draws nothing announces nothing either — a hidden
  // sentence about a thing that did not happen is still a thing said.
  await expect(page.locator('.nd-badge[data-nd-marked="false"] .nd-visually-hidden')).toHaveCount(0);
});

test("the badge adds no role to what it marks", async ({ page }) => {
  await page.goto(url(PAGE.badge));
  await showParts(page);
  // Wrapping a control in something with a role of its own is how a button
  // comes to announce as a group containing a button.
  for (const badge of await page.locator(".nd-badge").all()) {
    expect(await badge.getAttribute("role")).toBeNull();
    expect(await badge.getAttribute("aria-label")).toBeNull();
  }
});

test("a badge on a round thing tucks further in than one on a square", async ({ page }) => {
  await page.goto(url(PAGE.badge));
  await showParts(page);
  const inset = (overlap) =>
    page.locator(`.nd-badge[data-nd-overlap="${overlap}"]`).first().evaluate((el) => {
      const box = el.getBoundingClientRect();
      const mark = el.querySelector(".nd-badge-mark").getBoundingClientRect();
      return mark.right - box.right;
    });
  // A circle's corner is further from its bounding box than a square's, so a
  // badge placed for a button hangs off a round thing.
  expect(await inset("circle")).toBeLessThan(await inset("rectangle"));
});

// --- With no script ---------------------------------------------------------

test.describe("with the script blocked", () => {
  test.use({ javaScriptEnabled: false });

  test("removing a tag still submits which one", async ({ page }) => {
    await page.goto(url(PAGE.tag));
    await showParts(page);
    const tag = page.locator(`${TAG}:has(.nd-tag-remove[type="submit"])`).first();
    const cross = tag.locator(".nd-tag-remove");
    // `remove_name` makes it a real submit carrying its own name and value, so
    // the form posts which tag was taken off and the server renders the list
    // again. This is the whole reason that prop exists.
    await expect(cross).toHaveAttribute("type", "submit");
    await expect(cross).toHaveAttribute("name", "remove");
    const value = await cross.getAttribute("value");
    expect(value).toBeTruthy();

    await cross.click();
    await page.waitForLoadState("load");
    expect(new URL(page.url()).searchParams.get("remove")).toBe(value);
  });

  test("both components are complete before anything runs", async ({ page }) => {
    for (const slug of [PAGE.tag, PAGE.badge]) {
      await page.goto(url(slug));
      await showParts(page);
      await expect(page.locator(`${TAG}, .nd-badge`).first()).toBeVisible();
    }
    // And the badge still says its sentence, because it never needed a script
    // to say it.
    await expect(page.locator(".nd-badge .nd-visually-hidden").first()).not.toHaveText("");
  });
});

// --- Across every locale ----------------------------------------------------

test("no id appears twice on the new pages, in any locale", async ({ page }) => {
  for (const locale of LOCALES) {
    for (const slug of [PAGE.tag, PAGE.badge]) {
      await page.goto(url(slug, locale));
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
      expect(twice, `${locale}/${slug}`).toEqual([]);
    }
  }
});
