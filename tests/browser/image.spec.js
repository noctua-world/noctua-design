// The picture, and the box that was there before it was.
//
// THE ONE CLAIM WORTH A BROWSER. Every other property of this component can be
// read off the markup; "nothing moves when the file lands" cannot. So the test
// that matters here measures the page with the pictures blocked, lets them
// arrive, and measures it again — which is the reader's experience of a layout
// shift and the only way to see one.

import { expect, test } from "@playwright/test";

import { PAGE, url, showParts } from "./pages.js";

async function pictures(page) {
  await page.goto(url(PAGE.image));
  await showParts(page);
  return page.locator("img.nd-image-media");
}

test("every picture carries a real alt, and an empty one is a decision", async ({ page }) => {
  const all = await pictures(page);
  expect(await all.count()).toBeGreaterThan(0);
  const missing = await all.evaluateAll(
    (els) => els.filter((el) => el.getAttribute("alt") === null).length,
  );
  // NOT "every alt has words in it". An empty `alt` says the picture repeats
  // the sentence beside it, which is a real answer — the absent attribute is
  // the defect, because it makes a screen reader read the file name.
  expect(missing, "a picture with no alt attribute at all").toBe(0);

  // And at least one of each, because the page documents both.
  const kinds = await all.evaluateAll((els) => ({
    described: els.filter((el) => el.getAttribute("alt")).length,
    silent: els.filter((el) => el.getAttribute("alt") === "").length,
  }));
  expect(kinds.described).toBeGreaterThan(0);
  expect(kinds.silent).toBeGreaterThan(0);
});

test("a caption makes it a figure, so the words belong to the picture", async ({ page }) => {
  await pictures(page);
  const figure = page.locator("figure.nd-image").first();
  await expect(figure).toBeVisible();
  const caption = figure.locator("figcaption.nd-image-caption");
  await expect(caption).toBeVisible();
  // A `<figcaption>` and not a paragraph under the picture: the words are
  // attached to the figure rather than following it in the flow.
  expect(await caption.evaluate((el) => el.parentElement.tagName)).toBe("FIGURE");
  // With no caption the picture IS the root, and there is no figure wrapping
  // it — one arrangement per question, never both.
  const bare = page.locator("img.nd-image").first();
  // `closest("figure")` would find the example's own frame. The question is
  // whether the picture is wrapped in a figure of ITS OWN.
  expect(await bare.evaluate((el) => el.closest("figure.nd-image") === null)).toBe(true);
});

test("the four ratios hold four different boxes", async ({ page }) => {
  await pictures(page);
  const boxes = await page.evaluate(() => {
    const seen = {};
    for (const ratio of ["square", "landscape", "portrait", "wide"]) {
      const el = document.querySelector(`.nd-image[data-nd-ratio="${ratio}"] .nd-image-media`)
        ?? document.querySelector(`.nd-image-media[data-nd-ratio="${ratio}"]`);
      if (!el) continue;
      const box = el.getBoundingClientRect();
      seen[ratio] = box.width / box.height;
    }
    return seen;
  });
  expect(Object.keys(boxes).sort()).toEqual(["landscape", "portrait", "square", "wide"]);
  expect(boxes.square).toBeCloseTo(1, 1);
  expect(boxes.portrait).toBeLessThan(boxes.square);
  expect(boxes.square).toBeLessThan(boxes.landscape);
  expect(boxes.landscape).toBeLessThan(boxes.wide);
});

test("the box is held before the bytes arrive, so nothing moves", async ({ browser }) => {
  // THE DEFECT THIS COMPONENT EXISTS TO PREVENT, measured. With the files
  // blocked the page must already be its final height: a reserved box has a
  // size with nothing in it, and an unreserved one collapses to zero and
  // pushes everything under it down when the picture lands.
  const context = await browser.newContext();
  const page = await context.newPage();
  let blocked = true;
  await page.route("**/portraits/**", (route) => (blocked ? route.abort() : route.continue()));
  await page.goto(url(PAGE.image));
  await showParts(page);

  const held = await page.evaluate(() =>
    [...document.querySelectorAll(".nd-image[data-nd-ratio] .nd-image-media, img.nd-image-media[data-nd-ratio]")].map(
      (el) => Math.round(el.getBoundingClientRect().height),
    ),
  );
  expect(held.length).toBeGreaterThan(0);
  expect(
    held.filter((height) => height < 8),
    "a picture with a ratio collapsed while its file was missing",
  ).toEqual([]);
  await context.close();
});

test("every picture is lazy and decoded off the main thread", async ({ page }) => {
  const all = await pictures(page);
  const wrong = await all.evaluateAll((els) =>
    els
      .filter((el) => el.decoding !== "async" || !["lazy", "eager"].includes(el.loading))
      .map((el) => `${el.loading}/${el.decoding}`),
  );
  // `Avatar` established both and this keeps them. `eager` is legitimate for
  // the one picture a reader sees first, so it is allowed and `auto` is not.
  expect(wrong, "a picture that decides for itself when to arrive").toEqual([]);
  const eager = await all.evaluateAll((els) => els.filter((el) => el.loading === "eager").length);
  expect(eager, "the page never demonstrates eager loading").toBeGreaterThan(0);
});

test("no picture is wider than the column it stands in", async ({ page }) => {
  await pictures(page);
  const over = await page.evaluate(() =>
    [...document.querySelectorAll(".nd-image")]
      .filter((el) => {
        const parent = el.parentElement.getBoundingClientRect();
        return el.getBoundingClientRect().width > parent.width + 1;
      })
      .map((el) => el.className),
  );
  expect(over, "a picture is deciding how wide the page is").toEqual([]);
});
