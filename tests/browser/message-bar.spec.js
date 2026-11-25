// The bar that says what happened, and the two ways it can say it.
//
// WHAT IS BEING GUARDED. `role="alert"` cuts a screen reader off mid-sentence.
// That is correct about a refused payment and rude about a saved record, so the
// default is the quiet one and the loud one is a prop — and a default is
// exactly the kind of thing that drifts without a test naming it.
//
// The dismissal is here for `Info`'s reason: the control ships hidden and
// `dist/message-bar.js` reveals it, so a page whose script never ran carries no
// button that closes nothing. Both halves of that are asserted.

import { expect, test } from "@playwright/test";

import { PAGE, url, showParts } from "./pages.js";
import { contrast, extremes } from "./pixels.js";

async function bars(page) {
  await page.goto(url(PAGE.messageBar));
  await showParts(page);
  return page.locator(".nd-message-bar");
}

test("a bar is a status until a caller asks for the rude one", async ({ page }) => {
  const all = await bars(page);
  expect(await all.count()).toBeGreaterThan(0);

  // THE ROLE IS THE WHOLE OF WHAT `alert` DECIDES, and it is what assistive
  // technology acts on. This used to read the prop back out of a
  // `data-nd-alert` attribute that both bodies wrote and nothing else read —
  // so the assertion compared the component's output with the component's
  // output, and would have passed with the role wired to nothing.
  const roles = await all.evaluateAll((els) => els.map((el) => el.getAttribute("role")));
  for (const role of roles) {
    expect(role, "a bar with neither role announces nothing").toMatch(/^(status|alert)$/);
  }
  // Both are demonstrated, or the claim is untested on this page.
  expect(roles.includes("status")).toBe(true);
  expect(roles.includes("alert")).toBe(true);
});

test("aria-live is absent unless a caller wrote it", async ({ page }) => {
  const all = await bars(page);
  const live = await all.evaluateAll((els) =>
    els.map((el) => ({ role: el.getAttribute("role"), live: el.getAttribute("aria-live") })),
  );
  // `status` already implies polite and `alert` already implies assertive, so
  // an attribute written at a default would be the component overruling the
  // role it just chose. `role="alert" aria-live="polite"` is an alert that
  // never interrupts, and it is silent about being one.
  expect(
    live.filter((one) => one.role === "alert" && one.live === "polite"),
    "an alert was quietly downgraded to polite",
  ).toEqual([]);
  expect(live.some((one) => one.live === null), "every bar overrides its own role").toBe(true);
});

test("the icon repeats the words rather than replacing them", async ({ page }) => {
  const all = await bars(page);
  const icons = page.locator(".nd-message-bar-icon");
  expect(await icons.count()).toBeGreaterThan(0);
  // `aria-hidden`, because it says what the sentence says. A glyph a screen
  // reader announces beside the sentence it draws is the sentence twice.
  const heard = await icons.evaluateAll(
    (els) => els.filter((el) => el.getAttribute("aria-hidden") !== "true").length,
  );
  expect(heard).toBe(0);

  // AND THE TONE IS NEVER THE ONLY SIGNAL (`NOCTUA.md §8.8.6`). Every toned bar
  // has words in it, and the ones that carry a tone also carry a drawing.
  const silent = await all.evaluateAll((els) =>
    els
      .filter((el) => el.dataset.ndTone)
      .filter((el) => !el.querySelector(".nd-message-bar-body")?.textContent?.trim())
      .map((el) => el.dataset.ndTone),
  );
  expect(silent, "a bar is saying it in colour alone").toEqual([]);
});

test("the title is a strong and never a heading", async ({ page }) => {
  await bars(page);
  const titles = page.locator(".nd-message-bar-title");
  expect(await titles.count()).toBeGreaterThan(0);
  // A message that arrives after the page has loaded must not rewrite the
  // outline under a reader who is navigating by it.
  const shapes = await titles.evaluateAll((els) => [...new Set(els.map((el) => el.tagName))]);
  expect(shapes).toEqual(["STRONG"]);
});

test("with the script blocked there is no control that closes nothing", async ({ browser }) => {
  const context = await browser.newContext({ javaScriptEnabled: false });
  const page = await context.newPage();
  await page.goto(url(PAGE.messageBar));
  await showParts(page);
  const control = page.locator(".nd-message-bar-dismiss").first();
  await expect(control).toBeAttached();
  await expect(control).toBeHidden();
  await context.close();
});

test("the script reveals the control, and pressing it takes the bar away", async ({ page }) => {
  await bars(page);
  const bar = page.locator(".nd-message-bar[data-nd-dismissible='true']").first();
  const control = bar.locator(".nd-message-bar-dismiss");
  await expect(control).toBeVisible();
  expect(await control.evaluate((el) => el.tagName)).toBe("BUTTON");
  expect(await control.getAttribute("type")).toBe("button");
  // A glyph, so the name is the only thing a screen reader has.
  expect(await control.getAttribute("aria-label")).toMatch(/\S\s\S/);
  // 24px is the AA floor for a target.
  const box = await control.boundingBox();
  expect(box.width).toBeGreaterThanOrEqual(24);
  expect(box.height).toBeGreaterThanOrEqual(24);

  await control.click();
  await expect(bar).toBeHidden();
  // THE ELEMENT STAYS. A live region taken out of the document is a region
  // that announces nothing the next time a caller writes into it.
  await expect(bar).toBeAttached();
  expect(await bar.getAttribute("data-nd-dismissed")).toBe("");
  // AND FOCUS IS NOT LEFT WHERE THE READER CANNOT SEE IT. The control they
  // pressed is inside what they just dismissed, so focus lands on the box the
  // bar was in rather than on `<body>`, where the next Tab would restart at
  // the top of the document.
  const landed = await page.evaluate(() => {
    const active = document.activeElement;
    if (!active || active === document.body) return "body";
    if (active.classList.contains("nd-message-bar-dismiss")) return "the control";
    return "somewhere visible";
  });
  expect(landed).toBe("somewhere visible");
});

test("it is a bordered block and never a card", async ({ page }) => {
  const all = await bars(page);
  // A LINE IS NOT A BLOCK. It is a row filled with its tone along a window's
  // edge, so it has no rule down its leading edge to tell it from the page; it
  // is still held to no shadow.
  const wrong = await all.evaluateAll((els) =>
    els
      .map((el) => ({ style: getComputedStyle(el), line: el.dataset.ndPresentation === "line" }))
      .filter(
        ({ style, line }) =>
          style.boxShadow !== "none" || (!line && style.borderInlineStartWidth === "0px"),
      )
      .map(({ style }) => style.boxShadow),
  );
  // `NOCTUA.md §9.7`: separation is a hairline plus a surface colour, and the
  // rule down the leading edge is what tells one bar from the page around it.
  expect(wrong, "a message bar is drawing an elevated surface").toEqual([]);
});

test("the icon sits on the centre of the first line, not on the top of the block", async ({
  page,
}) => {
  // THE BAR IS `align-items: flex-start` AND THAT IS RIGHT — a glyph centred
  // against a paragraph floats in the middle of nothing. What was wrong is
  // what flush-with-the-top means: the glyph's box is shorter than the line
  // beside it, so its centre sat six pixels above the title's on every bar on
  // the page. `Button` has no such gap because its glyph takes its box from
  // the type scale and its row is one line tall.
  //
  // The line is measured with a Range rather than from the element, because an
  // element that wraps is several lines and only the first one is the question.
  const all = await bars(page);
  const offsets = await all.evaluateAll((els) =>
    els
      .map((el) => {
        const icon = el.querySelector(".nd-message-bar-icon");
        const first = el.querySelector(".nd-message-bar-text")?.firstElementChild;
        if (!icon || !first) return null;
        const range = document.createRange();
        range.selectNodeContents(first);
        const line = range.getClientRects()[0];
        if (!line) return null;
        const box = icon.getBoundingClientRect();
        return box.top + box.height / 2 - (line.top + line.bottom) / 2;
      })
      .filter((one) => one !== null),
  );
  expect(offsets.length, "no bar on the page has an icon and a sentence").toBeGreaterThan(2);
  for (const off of offsets) {
    // Within a pixel of the line's own centre. It was -6 on every bar.
    expect(Math.abs(off), "the icon is off the first line's centre").toBeLessThanOrEqual(1.5);
  }
});

// --- The line, and the time left ---------------------------------------------
//
// `presentation: line` is the interface's alert line (`NOCTUA.md §12.17`), and
// `deadline` the instant its countdown draws to. The deadline is the caller's,
// and on this page the caller is `shell.js`, which writes one ahead of now when
// the page loads. These tests write their own, on a clock they hold still: the
// countdown reads the time only through `Date.now()` and the page's timers, so
// a clock installed before the page loads is the whole of the freezing.

const LINE = ".nd-example-window .nd-message-bar[data-nd-presentation='line']";
const COUNTDOWN = "[data-nd-message-bar-countdown]";
const SAID = "[data-nd-message-bar-said]";

/** A token's length in pixels, measured on a box and not parsed off a string. */
async function token(page, name) {
  return page.evaluate((property) => {
    const probe = document.createElement("div");
    probe.style.position = "absolute";
    probe.style.blockSize = `var(${property})`;
    document.body.append(probe);
    const size = probe.getBoundingClientRect().height;
    probe.remove();
    return size;
  }, name);
}

/**
 * The page in a locale, on a clock held still, with the line's deadline
 * `seconds` ahead and a record of everything the polite region is given.
 */
async function frozen(page, locale = "en", seconds = 10) {
  await page.clock.install({ time: new Date("2026-11-25T12:00:00Z") });
  await page.goto(url(PAGE.messageBar, locale));
  await page.clock.pauseAt(new Date("2026-11-25T12:05:00Z"));
  const line = page.locator(LINE).first();
  await line.evaluate((el, ahead) => {
    // Every sentence written into the polite region, in order.
    window.__said = [];
    const region = el.querySelector("[data-nd-message-bar-said]");
    new MutationObserver(() => window.__said.push(region.textContent)).observe(region, {
      childList: true,
      characterData: true,
      subtree: true,
    });
    el.setAttribute("data-nd-deadline", String(Date.now() + ahead * 1000));
  }, seconds);
  // A frame, for the region the script writes a frame after the number.
  await page.clock.runFor(50);
  return line;
}

const said = (page) => page.evaluate(() => window.__said.slice());

test("a line is the last row of its window, as tall as a footer line and filled with its tone", async ({
  page,
}) => {
  await page.goto(url(PAGE.messageBar));
  const line = page.locator(LINE).first();
  await expect(line).toBeVisible();
  const seen = await line.evaluate((el) => {
    const window_ = el.parentElement;
    const box = el.getBoundingClientRect();
    const frame = window_.getBoundingClientRect();
    const border = Number.parseFloat(getComputedStyle(window_).borderBottomWidth);
    return {
      last: window_.lastElementChild === el,
      gap: frame.bottom - border - box.bottom,
      edges: [box.left - frame.left, frame.right - box.right],
      height: box.height,
      above: el.previousElementSibling?.getBoundingClientRect().bottom ?? 0,
      top: box.top,
      position: getComputedStyle(el).position,
    };
  });
  // IN THE FLOW AND LAST. A line over the window's bottom edge would cover the
  // footer's last line (WCAG 2.4.11), so it takes a row of its own.
  expect(seen.last, "the line is not the last thing in its window").toBe(true);
  expect(seen.position, "the line was taken out of the flow").toBe("static");
  expect(Math.abs(seen.gap), "the line does not reach the window's bottom edge").toBeLessThanOrEqual(1);
  expect(seen.top, "the line covers what is above it").toBeGreaterThanOrEqual(seen.above - 1);
  // EDGE TO EDGE: nothing of the window shows beside it.
  for (const edge of seen.edges) expect(Math.abs(edge - 1)).toBeLessThanOrEqual(1);
  // AT LEAST A FOOTER LINE, which is the height the two meet at.
  const floor = await token(page, "--noctua-shell-line");
  expect(seen.height).toBeGreaterThanOrEqual(floor - 0.5);

  // THE WORDS ARE READABLE ON THE FILL. The darkest and lightest pixels of the
  // sentence are its ink and the tone under it.
  await line.scrollIntoViewIfNeeded();
  const body = await line.locator(".nd-message-bar-body").boundingBox();
  const shot = await page.screenshot({ clip: body });
  const { min, max } = await extremes(page, shot);
  expect(contrast(min, max), "the line's words are unreadable on its fill").toBeGreaterThanOrEqual(4.5);
});

test("every control in a line clears the target floor, and its ring is drawn inside it", async ({
  page,
}) => {
  await page.goto(url(PAGE.messageBar));
  const floor = await token(page, "--noctua-target-min");
  const action = page.locator(`${LINE} .nd-message-bar-action`).first();
  await expect(action).toBeVisible();
  expect(await action.evaluate((el) => el.tagName)).toBe("BUTTON");
  expect(await action.getAttribute("data-nd-action")).toBe("restore");
  const box = await action.boundingBox();
  expect(box.width).toBeGreaterThanOrEqual(floor);
  expect(box.height).toBeGreaterThanOrEqual(floor);

  // A RING HANGING OUT OF THE LINE would be drawn over the footer above it.
  await action.focus();
  const ring = await action.evaluate((el) => ({
    offset: Number.parseFloat(getComputedStyle(el).outlineOffset),
    width: Number.parseFloat(getComputedStyle(el).outlineWidth),
  }));
  expect(ring.width, "the action shows no ring").toBeGreaterThan(0);
  expect(ring.offset + ring.width, "the action's ring is drawn outside it").toBeLessThanOrEqual(0);
});

test("the countdown draws the whole seconds left, and says them only now and then", async ({
  page,
}) => {
  const line = await frozen(page);
  const clock = line.locator(COUNTDOWN);
  await expect(clock).toBeVisible();
  await expect(clock).toHaveText("Restoring it in 10 seconds");
  // ITS OWN REGION IS SILENT, written out, inside an alert that is atomic.
  expect(await clock.getAttribute("role")).toBe("timer");
  expect(await clock.getAttribute("aria-live")).toBe("off");
  expect(await line.locator(SAID).getAttribute("aria-live")).toBe("polite");

  // One second: the number moves, and nothing is said.
  await page.clock.runFor(1000);
  await expect(clock).toHaveText("Restoring it in 9 seconds");
  // Five: said again, the last count a reader can still act on.
  await page.clock.runFor(4000);
  await expect(clock).toHaveText("Restoring it in 5 seconds");
  // To the deadline and past it: the number stops at nothing left.
  await page.clock.runFor(7000);
  await expect(clock).toHaveText("Restoring it in 0 seconds");
  await page.clock.runFor(50);

  // NOT EVERY SECOND. Twelve seconds of a ten-second count said two things:
  // the count starting and five seconds left.
  expect(await said(page)).toEqual(["Restoring it in 10 seconds", "Restoring it in 5 seconds"]);
});

test("a longer count is said every ten seconds, and the singular is the caller's", async ({
  page,
}) => {
  const line = await frozen(page, "en", 21);
  const clock = line.locator(COUNTDOWN);
  await page.clock.runFor(20000);
  await expect(clock).toHaveText("Restoring it in 1 second");
  await page.clock.runFor(50);
  expect(await said(page)).toEqual([
    "Restoring it in 21 seconds",
    "Restoring it in 20 seconds",
    "Restoring it in 10 seconds",
    "Restoring it in 5 seconds",
  ]);
});

test("a deadline the caller writes again is counted again", async ({ page }) => {
  const line = await frozen(page);
  const clock = line.locator(COUNTDOWN);
  await page.clock.runFor(3000);
  await expect(clock).toHaveText("Restoring it in 7 seconds");
  // THE CALLER KEEPS THE CLOCK, and this draws whatever instant it writes.
  await line.evaluate((el) => el.setAttribute("data-nd-deadline", String(Date.now() + 30_000)));
  await expect(clock).toHaveText("Restoring it in 30 seconds");
  // And a value that is not an instant draws nothing at all.
  await line.evaluate((el) => el.setAttribute("data-nd-deadline", "soon"));
  await expect(clock).toBeHidden();
});

test("with no sentence of the caller's the seconds are the platform's, in the page's language", async ({
  page,
}) => {
  for (const [locale, words] of [
    ["en", /^10 seconds$/],
    ["pt-BR", /^10 segundos$/],
  ]) {
    const context = await page.context().browser().newContext();
    const tab = await context.newPage();
    const line = await frozen(tab, locale);
    await line.evaluate((el) => el.removeAttribute("data-nd-countdown-label"));
    await expect(line.locator(COUNTDOWN)).toHaveText(words);
    await context.close();
  }
});

test("the countdown is drawn in every locale, the pseudo-locale included", async ({ page }) => {
  // THE DEFECT THIS NAMES. The pseudo-locale stretched a plural message whole,
  // so its forms were called `oonee` and `ootheer`, `plural()` found neither,
  // and the count was drawn as an empty run, which no bracket sweep can see.
  const line = await frozen(page, "qps");
  const clock = line.locator(COUNTDOWN);
  await expect(clock).toBeVisible();
  const text = await clock.textContent();
  expect(text, "the countdown drew nothing in qps").toMatch(/10/);
  expect(text, "the countdown is not the pseudo-locale's sentence").toMatch(/^⟦.*⟧$/);
});

test("a bar a reader takes away stops counting", async ({ page }) => {
  await page.goto(url(PAGE.messageBar));
  const bar = page.locator(".nd-message-bar[data-nd-deadline]:not([data-nd-presentation='line'])").first();
  await expect(bar.locator(COUNTDOWN)).toBeVisible();
  // The bar is not dismissible, so the caller takes it away, as the script
  // would.
  await bar.evaluate((el) => el.setAttribute("data-nd-dismissed", ""));
  await expect(bar.locator(COUNTDOWN)).toBeHidden();
  expect(await bar.locator(COUNTDOWN).textContent()).toBe("");
});

test("with the script blocked nothing counts, and the action is still there", async ({ browser }) => {
  const context = await browser.newContext({ javaScriptEnabled: false });
  const page = await context.newPage();
  await page.goto(url(PAGE.messageBar));
  const line = page.locator(LINE).first();
  await expect(line.locator(COUNTDOWN)).toBeAttached();
  await expect(line.locator(COUNTDOWN)).toBeHidden();
  expect(await line.locator(SAID).textContent()).toBe("");
  // THE PRESS IS THE CALLER'S CODE, so nothing here hides the control.
  await expect(line.locator(".nd-message-bar-action")).toBeVisible();
  await context.close();
});
