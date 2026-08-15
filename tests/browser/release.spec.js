// Applying a behaviour, and taking it back off.
//
// EVERY ENHANCER IN THIS SYSTEM COULD BE APPLIED AND NONE COULD BE RELEASED,
// and that is a defect for every consumer with a client-side router rather than
// a gap in one framework's story. `enhance` registers listeners on the
// document, observes elements for resize and intersection, arms timers and
// holds open panels in module-level registries. None of it stopped when the
// markup left the page, so a route visited ten times carried ten copies.
//
// Six of the enhancers could not even be applied to a subtree: they hardcoded
// `document.querySelectorAll` while every other one took a root.
//
// THE TWO LISTS ARE `enhanceAll` AND `releaseAll` IN `shell.js`, and this file
// reaches them the way anything on the page would — by importing the module the
// document already loaded. A second `import()` of the same URL is the same
// module instance, so what these tests drive is the running script and not a
// copy of it.

import { expect, test } from "@playwright/test";

import { PAGE, url, showParts } from "./pages.js";

/**
 * The running `shell.js`, with `enhanceAll` and `releaseAll` on it.
 *
 * The URL comes off the page's own `<script>` rather than being written here,
 * because the site is served under a base path and a second spelling of it is a
 * second thing that can be wrong.
 */
async function shell(page) {
  return page.evaluateHandle(async () => {
    const tag = document.querySelector('script[type="module"][src*="shell.js"]');
    return import(tag.src);
  });
}

// --- The date input, and the guard four of its neighbours already had --------

test("a date input steps once per keystroke after ten enhancement passes", async ({ page }) => {
  // `enhanceDateInputs` HAD NO GUARD AT ALL. Its four neighbours each check a
  // `WeakMap` before building; this one enhanced whatever it was given, every
  // time. So a second pass bound the arrow keys, the paste and the two blurs a
  // second time, and one press of the up arrow stepped the day twice.
  //
  // It was latent because the function was called exactly once, at load. The
  // moment anything re-enhances a subtree it is live, and a reader watching
  // their date jump two days per press has no way to know why.
  await page.goto(url(PAGE.dateInput));
  await showParts(page);

  const field = page.locator('[data-nd-date-input="visit"]');
  const day = field.locator('[data-nd-part="day"] input');
  await expect(day).toBeVisible();

  const api = await shell(page);
  await page.evaluate(({ mod }) => {
    for (let pass = 0; pass < 10; pass += 1) mod.enhanceAll(document.body);
  }, { mod: api });

  // ONE PRESS, ONE STEP. From empty, the up arrow writes the first day of the
  // month. A handler bound twice writes the second.
  await day.click();
  await day.press("ArrowUp");
  expect((await day.inputValue()).replace(/^0+/, "")).toBe("1");

  await day.press("ArrowUp");
  expect((await day.inputValue()).replace(/^0+/, "")).toBe("2");
});

test("ten enhancement passes draw one suggest control, not ten", async ({ page }) => {
  // THE SAME MISSING GUARD, MEASURED AS GEOMETRY. `enhanceDateInput` DRAWS the
  // suggest button and appends it to the row, so an unguarded second pass does
  // not merely double a listener: it puts another control in the layout. Ten
  // passes made a row ten buttons longer than the boxes it holds.
  await page.goto(url(PAGE.dateInput));
  await showParts(page);

  const field = page.locator('[data-nd-date-input="visit"]');
  const row = field.locator(".nd-date-input-row");
  await expect(row.locator(".nd-date-input-suggest-button")).toHaveCount(1);
  const before = await row.evaluate((el) => Math.round(el.getBoundingClientRect().width));

  const api = await shell(page);
  await page.evaluate(({ mod }) => {
    for (let pass = 0; pass < 10; pass += 1) mod.enhanceAll(document.body);
  }, { mod: api });

  await expect(row.locator(".nd-date-input-suggest-button")).toHaveCount(1);
  // The picture is unchanged, which is the claim a count alone does not make:
  // a control drawn off screen is still a control in the row.
  const after = await row.evaluate((el) => Math.round(el.getBoundingClientRect().width));
  expect(after).toBe(before);
});

// --- collapseBlocks, scoped ---------------------------------------------------

test("collapsing one subtree leaves a block the reader opened elsewhere open", async ({ page }) => {
  // `collapseBlocks` WAS IDEMPOTENT PER ELEMENT AND WRONG PAGE-WIDE. Running it
  // again over the whole document RE-COLLAPSES a block the reader had already
  // expanded, so their answer to the control was undone by an enhancement pass
  // somewhere else entirely. A root parameter is the whole fix: judge the
  // subtree that changed and leave the rest of the page alone.
  //
  // The site ships one collapsible code block, so the second is cloned here.
  // The claim is about scoping, and it needs two blocks in two subtrees.
  await page.goto(url(PAGE.codeBlock));
  await showParts(page);

  const built = await page.evaluate(() => {
    const original = document.querySelector('[data-nd-collapsible="true"]');
    if (!original) return false;
    const twin = original.cloneNode(true);
    twin.setAttribute("data-nd-twin", "");
    const host = document.createElement("div");
    host.setAttribute("data-nd-host", "");
    host.append(twin);
    original.after(host);
    return true;
  });
  expect(built).toBe(true);

  const first = page.locator('[data-nd-collapsible="true"]:not([data-nd-twin]) .nd-codeblock');
  const twin = page.locator("[data-nd-twin] .nd-codeblock");
  const capped = await first.evaluate((el) => Math.round(el.getBoundingClientRect().height));
  expect(await first.getAttribute("data-nd-collapsed")).toBe("true");

  // The reader opens the first one, with the control the page gave them.
  await page
    .locator('[data-nd-collapsible="true"]:not([data-nd-twin]) [data-nd-expand]')
    .first()
    .click();
  const opened = await first.evaluate((el) => Math.round(el.getBoundingClientRect().height));
  expect(opened).toBeGreaterThan(capped);

  // Now a subtree somewhere else on the page is enhanced.
  const api = await shell(page);
  await page.evaluate(({ mod }) => {
    mod.enhanceAll(document.querySelector("[data-nd-host]"));
  }, { mod: api });

  // The clone is capped, because it was never opened.
  expect(await twin.getAttribute("data-nd-collapsed")).toBe("true");
  // And the block the reader opened is still open, measured rather than
  // asked: the attribute is what the script writes and the height is what the
  // reader sees.
  const still = await first.evaluate((el) => Math.round(el.getBoundingClientRect().height));
  expect(still).toBe(opened);
});

// --- Releasing a subtree ------------------------------------------------------

test("releasing a subtree with a menu open leaves nothing behind", async ({ page }) => {
  // A MENU LEFT OPEN BY A RELEASE IS THE WORST OF BOTH. Its listeners are gone,
  // so Escape answers nothing and a press outside answers nothing, and a panel
  // sits over the page with no way out of it. Worse, `menu.js` held the open
  // menu's close function in a module-level `showing` — so the next menu opened
  // ANYWHERE called a closure over a button nobody could see.
  await page.goto(url(PAGE.menu));
  await showParts(page);

  // NAMED BY THEIR PANELS, NEVER BY POSITION. A `:visible` locator is
  // re-evaluated on every use, and releasing the first menu hides its button —
  // so `first()` would then resolve to a different control and the assertion
  // would be about the wrong one.
  //
  // SCOPED TO THE EXAMPLES, which is `menu.spec.js`'s rule and the same reason:
  // the frame's own rail carries a width menu on every page of the site, it
  // comes first in the document, and it sits in a toolbar whose arrow keys walk
  // the rail rather than opening anything.
  const ids = await page.evaluate(() =>
    [...document.querySelectorAll('.nd-example-stage [aria-haspopup="menu"][aria-controls]')]
      .filter((one) => one.offsetParent !== null && !one.closest('[role="toolbar"]'))
      .map((one) => one.getAttribute("aria-controls")),
  );
  expect(ids.length).toBeGreaterThan(1);

  const first = page.locator(`[aria-controls="${ids[0]}"]`);
  const second = page.locator(`[aria-controls="${ids[1]}"]`);
  const firstPanel = page.locator(`#${ids[0]}`);
  const secondPanel = page.locator(`#${ids[1]}`);

  await first.click();
  await expect(firstPanel).toBeVisible();

  // The subtree the first menu lives in, released the way a router would.
  const api = await shell(page);
  await page.evaluate(
    ({ mod, id }) => {
      const button = document.querySelector(`[aria-controls="${id}"]`);
      mod.releaseAll(button.closest("figure, section, body"));
    },
    { mod: api, id: ids[0] },
  );

  // Nothing behind: the surface is shut and the control that opened it is back
  // to what the markup ships, which is hidden.
  await expect(firstPanel).toBeHidden();
  await expect(first).toBeHidden();

  // Opened from the keyboard, because that is what puts focus inside the
  // surface — a menu opened with a press leaves the reader on the button, and
  // Escape there is the button's key rather than the menu's.
  await second.focus();
  await page.keyboard.press("ArrowDown");
  await expect(secondPanel).toBeVisible();
  // EXACTLY ONE. Two surfaces open over one page is the state the released
  // menu's ghost used to produce.
  expect(await page.locator(".nd-menu:visible").count()).toBe(1);

  await page.keyboard.press("Escape");
  await expect(secondPanel).toBeHidden();
});

test("a released subtree takes its document listeners with it", async ({ page }) => {
  // THE LEAK, COUNTED. `menu.js`, `info.js`, `popover.js` and `color-picker.js`
  // each register a `pointerdown` on the DOCUMENT inside a per-element
  // `enhance`, and `carousel.js` registers a `visibilitychange` there. Mount,
  // unmount, mount used to add one per cycle for ever.
  //
  // COUNTED FROM BEFORE THE PAGE'S OWN SCRIPT RAN, which is the only way the
  // number means anything: a counter installed afterwards cannot see the
  // listeners the load registered, so a release that does nothing looks
  // identical to one that works.
  //
  // Only the document's own are counted. A listener on an element inside the
  // subtree goes when the subtree does, and needs no help.
  await page.addInitScript(() => {
    const real = EventTarget.prototype.addEventListener;
    window.__ndLive = 0;
    EventTarget.prototype.addEventListener = function (type, handler, options) {
      if (this === document) {
        window.__ndLive += 1;
        options?.signal?.addEventListener("abort", () => {
          window.__ndLive -= 1;
        });
      }
      return real.call(this, type, handler, options);
    };
  });
  await page.goto(url(PAGE.menu));
  await showParts(page);

  const menus = await page.locator('[aria-haspopup="menu"][aria-controls]').count();
  expect(menus).toBeGreaterThan(0);

  const api = await shell(page);
  const counts = await page.evaluate(({ mod }) => {
    const loaded = window.__ndLive;
    mod.releaseAll(document.body);
    const released = window.__ndLive;
    let peak = 0;
    for (let cycle = 0; cycle < 10; cycle += 1) {
      mod.enhanceAll(document.body);
      peak = Math.max(peak, window.__ndLive);
      if (cycle < 9) mod.releaseAll(document.body);
    }
    return { loaded, released, peak, live: window.__ndLive };
  }, { mod: api });

  // A release drops at least one document listener per menu on the page, so a
  // release that quietly does nothing cannot pass this.
  expect(counts.loaded - counts.released).toBeGreaterThanOrEqual(menus);
  // Ten cycles later the document carries exactly what it carried at load.
  // Growth here is the leak: one more per visit, for ever.
  expect(counts.live).toBe(counts.loaded);
  expect(counts.peak).toBe(counts.loaded);
});

test("a released subtree can be enhanced again", async ({ page }) => {
  // RELEASE IS NOT A ONE-WAY DOOR. A router that leaves a route and comes back
  // enhances the same markup again, so every guard a release drops has to be
  // dropped for real. A `WeakMap` entry nothing removes is a control that can
  // never be rebuilt, which is a quieter failure than a leak and harder to see.
  await page.goto(url(PAGE.menu));
  await showParts(page);

  // In an example, and not in a toolbar: the rail's arrow keys walk the rail.
  const button = page.locator('.nd-example-stage [aria-haspopup="menu"][aria-controls]').first();
  const panel = page.locator(`#${await button.getAttribute("aria-controls")}`);
  await expect(button).toBeVisible();

  const api = await shell(page);
  await page.evaluate(({ mod }) => {
    mod.releaseAll(document.body);
    mod.enhanceAll(document.body);
  }, { mod: api });

  await expect(button).toBeVisible();
  await button.focus();
  await page.keyboard.press("ArrowDown");
  await expect(panel).toBeVisible();
  await page.keyboard.press("Escape");
  await expect(panel).toBeHidden();
});

test("a panel detached with nothing released is dropped by the anchoring pass", async ({
  page,
}) => {
  // `anchor.js`'s `anchored` MAP IS STRONG, and `reanchor` walks it on every
  // scroll frame calling `getBoundingClientRect()` on each entry. A panel whose
  // subtree was taken out of the document stayed in it for ever, because a
  // detached element measures as a zero-sized box at the origin — which passes
  // the off-screen test, so nothing ever evicted it. One leaked panel is quiet.
  // A router leaks one per visit and every one is measured on every frame.
  //
  // THIS IS THE CASE NOTHING CALLS A RELEASE FOR. The subtree is removed with
  // no teardown at all, which is exactly what a consumer who has never heard of
  // `releaseAll` does, so the eviction has to come from the pass itself.
  await page.goto(url(PAGE.menu));
  await showParts(page);

  const button = page.locator('.nd-example-stage [aria-haspopup="menu"][aria-controls]').first();
  const id = await button.getAttribute("aria-controls");
  await button.click();
  await expect(page.locator(`#${id}`)).toBeVisible();

  const seen = await page.evaluate(async (panelId) => {
    const surface = document.getElementById(panelId);
    const scroll = async () => {
      window.scrollBy(0, 120);
      await new Promise((done) => requestAnimationFrame(() => requestAnimationFrame(done)));
    };

    // A SENTINEL, NOT A MEASUREMENT. The value is never drawn and never
    // compared to a token: the claim is only whether the anchoring pass
    // overwrote it, so any value no engine would produce works.
    // While it is attached, the pass owns the panel and writes the offset back.
    surface.style.setProperty("--nd-anchor-block", "1px"); // allow-literal: a sentinel the pass overwrites, never a length anything draws
    await scroll();
    const attached = surface.style.getPropertyValue("--nd-anchor-block");

    // Now the subtree goes, with no teardown of any kind.
    surface.closest(".nd-example-stage, figure, body").remove();
    await scroll();
    surface.style.setProperty("--nd-anchor-block", "1px"); // allow-literal: the same sentinel, re-armed after the subtree is detached
    await scroll();
    return { attached, detached: surface.style.getPropertyValue("--nd-anchor-block") };
  }, id);

  // The pass was running: it overwrote the sentinel while the panel was on the
  // page. Without that, the second half proves nothing.
  expect(seen.attached).not.toBe("1px"); // allow-literal: comparing against the sentinel written above
  // And it has stopped: the detached panel is nobody's work any more.
  expect(seen.detached).toBe("1px"); // allow-literal: comparing against the sentinel written above
});
