// One material, and the two structural rules that keep it one.
//
// Glass was written by hand four times — a utility, the header's backdrop, the
// settings panel and the drawer — each repeating the same blur over the same
// mix. They drifted: the drawer switched its glass off above `lg`, so on every
// desktop viewport the sidebar and the settings panel were visibly different
// materials on the same page. That is the first thing a reader notices and the
// last thing a compiler can.

import { test, expect } from "@playwright/test";
import { ANY, PAGE, SCOPED, url } from "./pages.js";

/** Opens the settings panel without depending on where its summary sits. */
const openSettings = (page) =>
  page.evaluate(() => {
    document.querySelector(".nd-settings").open = true;
  });

test("every frosted surface is the same material", async ({ page }) => {
  await page.goto(url(ANY));
  await openSettings(page);

  const filters = await page.evaluate(() =>
    [...document.querySelectorAll(".noctua-glass")].map((el) => ({
      what: el.className,
      filter: getComputedStyle(el).backdropFilter,
    })),
  );

  expect(filters.length, "nothing on the page is glass").toBeGreaterThan(2);
  const distinct = new Set(filters.map((f) => f.filter));
  expect(
    [...distinct],
    `surfaces disagree about the material: ${JSON.stringify(filters)}`,
  ).toHaveLength(1);
  expect([...distinct][0]).toContain("blur");
});

test("no glass surface is inside another", async ({ page }) => {
  await page.goto(url(SCOPED));
  await openSettings(page);
  // A `backdrop-filter` whose ancestor has one samples nothing at all. It is
  // inert, it reports nothing, and the surface simply renders as a flat fill —
  // which is how the settings panel shipped opaque for a whole stage.
  const nested = await page.evaluate(() =>
    [...document.querySelectorAll(".noctua-glass")]
      .filter((el) => el.parentElement?.closest(".noctua-glass"))
      .map((el) => el.className),
  );
  expect(nested, "a nested backdrop-filter is inert").toEqual([]); // allow-literal: the word is the assertion, not a second recipe — this test exists to prove the rule the gate enforces
});

test("a scrollable element never carries a radius", async ({ page }) => {
  await page.goto(url(ANY));
  await page.evaluate(() => {
    // The worst case, and the one the complaint was about: the roundest corners
    // the system offers.
    document.documentElement.setAttribute("data-noctua-radius", "circular");
    document.querySelector(".nd-settings").open = true;
  });

  const offenders = await page.evaluate(() =>
    [...document.querySelectorAll("*")]
      .filter((el) => {
        const style = getComputedStyle(el);
        const scrolls = ["auto", "scroll"].some(
          (v) => style.overflowX === v || style.overflowY === v,
        );
        if (!scrolls) return false;
        // A radius on a scroller has nothing to clip the bar against, so the
        // scrollbar paints square straight through the rounded corner.
        return style.borderRadius !== "0px";
      })
      .map((el) => `${el.className || el.tagName}: ${getComputedStyle(el).borderRadius}`),
  );
  expect(offenders, "a scrollbar will paint over these corners").toEqual([]);
});

test.describe("with the script blocked", () => {
  test.use({ javaScriptEnabled: false });

  test("a scrollable element never carries a radius there either", async ({ page }) => {
    // THE SWEEP THAT FOUND THIS RAN WITHOUT A SCRIPT AND THIS SUITE DID NOT.
    //
    // `TagPicker`'s native `<select multiple>` is a four-row scrolling listbox
    // and it is `hidden` the moment the script enhances the control — so every
    // test asking this question saw nothing, and the one state a reader could
    // actually meet it in was the one nobody looked at. Half of what a
    // no-script page shows is a control some enhanced page hides.
    for (const slug of Object.values(PAGE)) {
      await page.goto(url(slug));
      await page.evaluate(() =>
        document.documentElement.setAttribute("data-noctua-radius", "circular"),
      );
      const offenders = await page.evaluate(() =>
        [...document.querySelectorAll("*")]
          .filter((el) => {
            const style = getComputedStyle(el);
            const scrolls =
              (el.scrollHeight > el.clientHeight + 1 &&
                ["auto", "scroll"].includes(style.overflowY)) ||
              (el.scrollWidth > el.clientWidth + 1 && ["auto", "scroll"].includes(style.overflowX));
            return scrolls && style.borderRadius !== "0px";
          })
          .map((el) => `${el.className || el.tagName}: ${getComputedStyle(el).borderRadius}`),
      );
      expect(offenders, `${slug}: a scrollbar will paint over these corners`).toEqual([]);
    }
  });
});

test("the settings panel clips its own scroller", async ({ page }) => {
  await page.goto(url(ANY));
  await page.evaluate(() => {
    document.documentElement.setAttribute("data-noctua-radius", "circular");
    document.querySelector(".nd-settings").open = true;
  });

  const shape = await page.evaluate(() => {
    const panel = document.querySelector(".nd-settings .nd-panel");
    const body = document.querySelector(".nd-settings .nd-panel-body");
    return {
      panelClips: getComputedStyle(panel).overflow,
      panelRadius: getComputedStyle(panel).borderRadius,
      bodyScrolls: getComputedStyle(body).overflowY,
      bodyRadius: getComputedStyle(body).borderRadius,
      bodyOverflows: body.scrollHeight > body.clientHeight,
      thin: getComputedStyle(body).scrollbarWidth,
    };
  });

  expect(shape.panelClips).toBe("hidden");
  expect(shape.panelRadius).not.toBe("0px");
  expect(shape.bodyScrolls).toBe("auto");
  expect(shape.bodyRadius).toBe("0px");
  expect(shape.bodyOverflows, "the panel does not scroll, so this proves nothing").toBe(true);
  // `scrollbar-width` is not inherited. Set on `:root` alone it styles the
  // page's own bar and leaves every panel with the browser's default — which
  // is the bar a reader is actually looking at.
  expect(shape.thin).toBe("thin");
});

test("the spacing ruler never paints over its label", async ({ page }) => {
  for (const width of [320, 390, 1280]) {
    await page.setViewportSize({ width, height: 900 });
    await page.goto(url(PAGE.icons));
    const overlaps = await page.evaluate(() =>
      [...document.querySelectorAll(".nd-scale-row")]
        .map((row) => {
          const name = row.querySelector(".nd-scale-name")?.getBoundingClientRect();
          const bar = row.querySelector(".nd-bar")?.getBoundingClientRect();
          if (!name || !bar) return null;
          return name.right > bar.left ? row.textContent.trim().slice(0, 40) : null;
        })
        .filter(Boolean),
    );
    expect(overlaps, `at ${width}px the label runs under the bar`).toEqual([]);
  }
});

test("the background pattern survives across the whole viewport", async ({ page }) => {
  await page.goto(url(ANY));
  await page.evaluate(() => {
    document.documentElement.setAttribute("data-noctua-pattern", "mesh");
  });
  // A mask used to erase 18%–62% of the width — 44% of the feature, over
  // exactly the part of the page a reader looks at. The ink ceiling in
  // spec/tokens.toml is the readability guarantee; the mask was a second one
  // that removed the feature instead.
  //
  // ASKED OF THE ROOT, because there is no `.noctua-fx` any more: the three
  // effects are background layers on whatever element carries the attribute,
  // which is what made them scopable. This test used to name that element and
  // would have gone on passing against `null` had it been left alone.
  const root = await page.evaluate(() => {
    const style = getComputedStyle(document.documentElement);
    return { mask: style.maskImage, image: style.backgroundImage };
  });
  expect(root.mask).toBe("none");
  // And it is genuinely painting — a mask of `none` over nothing is not a
  // guarantee about a pattern.
  expect(root.image, "the page paints no pattern at all").toContain("gradient");
});
