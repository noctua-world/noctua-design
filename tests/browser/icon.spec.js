// The component that draws the drawings.
//
// WHAT THIS FILE IS ACTUALLY ASKING, and why it is not `icons.spec.js`. That
// one is about the GALLERY: which drawings exist, that each one is vendored,
// that none of them names a colour. This is about the COMPONENT: the box it
// renders, the two states its `label` chooses between, and the three sizes.
//
// The promotion these tests arrived with is what makes them possible. `Icon`
// used to be generated outside the catalogue, so it had no page, no props table
// and no variant grid — there was nothing here to measure.

import { expect, test } from "@playwright/test";

import { PAGE, url, showParts } from "./pages.js";

/** The component's own page, with the folded sections open. */
async function iconPage(page) {
  await page.goto(url(PAGE.icon));
  await showParts(page);
}

test("an unlabelled icon is out of the accessibility tree entirely", async ({ page }) => {
  await iconPage(page);
  // EVERY DRAWING WITH NO NAME, not only the ones already saying they are
  // hidden. The old sweep asked for `aria-hidden="true"` first. That filter
  // threw away the one icon this test exists to find. An unnamed drawing left
  // in the tree is announced as *graphic* in the middle of a sentence.
  //
  // The three attributes go together. Out of the tree, no role to claim from
  // inside it, and out of the tab order in the engines that put SVG in it.
  const wrong = await page.evaluate(() =>
    [...document.querySelectorAll("svg:not([aria-label])")]
      .filter(
        (el) =>
          el.getAttribute("aria-hidden") !== "true" ||
          el.getAttribute("role") !== null ||
          el.getAttribute("focusable") !== "false",
      )
      .map((el) => el.getAttribute("data-nd-icon") ?? el.getAttribute("class")),
  );
  expect(wrong, "an unnamed icon a screen reader can still reach").toEqual([]);
  // A FLOOR, because an empty selection satisfies every filter above. The
  // header alone draws more drawings than this.
  const counted = await page.locator("svg:not([aria-label])").count();
  expect(counted, "no unlabelled icon was measured at all").toBeGreaterThan(5);
});

test("a labelled icon is a role=img with that name and no aria-hidden", async ({ page }) => {
  await iconPage(page);
  const named = page.locator("svg.nd-icon-glyph[aria-label]").first();
  await expect(named).toBeAttached();
  expect(await named.getAttribute("role")).toBe("img");
  expect(await named.getAttribute("aria-hidden")).toBeNull();
  expect((await named.getAttribute("aria-label"))?.trim()).toBeTruthy();
});

test("the three sizes are three different boxes", async ({ page }) => {
  await iconPage(page);
  // The variant grid draws one per value, and `size` is the only enum prop
  // this component has — so this is also the check that the grid is not three
  // copies of one picture.
  const drawn = await page.evaluate(() => {
    const seen = {};
    for (const size of ["small", "medium", "large"]) {
      const el = document.querySelector(
        `.nd-variant-stage[data-nd-slug="icon"] svg.nd-icon-glyph[data-nd-size="${size}"]`,
      );
      if (el) seen[size] = el.getBoundingClientRect().height;
    }
    return seen;
  });
  expect(Object.keys(drawn).sort()).toEqual(["large", "medium", "small"]);
  expect(drawn.small).toBeLessThan(drawn.medium);
  expect(drawn.medium).toBeLessThan(drawn.large);
});

test("the drawing takes its size from the text around it", async ({ page }) => {
  await iconPage(page);
  // `1em` is the whole reason one class covers every context, and it is the
  // claim `size`'s documentation makes. An icon in a heading and an icon in a
  // caption are the same component with no `size` between them.
  const measured = await page.evaluate(() => {
    const icons = [...document.querySelectorAll(".nd-example-stage svg.nd-icon-glyph")];
    return icons.map((el) => {
      const style = getComputedStyle(el);
      return {
        font: Number.parseFloat(style.fontSize),
        box: el.getBoundingClientRect().height,
      };
    });
  });
  expect(measured.length).toBeGreaterThan(0);
  // Within a pixel: an `em` box is the font size, whatever the font size is.
  for (const one of measured) {
    expect(Math.abs(one.box - one.font)).toBeLessThan(1.5);
  }
});

test("every icon on the page is a vendored drawing and not a coloured one", async ({ page }) => {
  await iconPage(page);
  // A drawing that names its own colour cannot follow a theme, which is the
  // frozen-colour trap wearing a different hat. The wrapper paints; the
  // geometry inside it does not.
  const coloured = await page.evaluate(() =>
    [...document.querySelectorAll("svg.nd-icon-glyph")]
      .filter((el) =>
        [...el.children].some(
          (child) => child.hasAttribute("fill") || child.hasAttribute("stroke"),
        ),
      )
      .map((el) => el.getAttribute("data-nd-icon")),
  );
  expect(coloured, "a drawing that paints itself").toEqual([]);
});
