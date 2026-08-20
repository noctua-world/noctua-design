// The message that floats, and the timer that has to be interruptible.
//
// WHAT IS BEING GUARDED. A message that removes itself on a timer is unreadable
// to a reader who reads slowly, to one using a magnifier who has not reached
// that corner, and to one who looked away. So three things are asserted here:
// the timer is absent unless a caller asked for one, it holds while a pointer or
// the focus is on the message, and the element survives its own dismissal
// because it is a live region.
//
// And the quiet one: `role="status"` rather than `role="alert"`, for
// `MessageBar`'s reason. An alert cuts a screen reader off mid-sentence, and a
// component whose default is the rude one gets the rude one everywhere.

import { expect, test } from "@playwright/test";

import { PAGE, url, showParts } from "./pages.js";

const named = (page, name) => page.locator(`[data-nd-toast="${name}"]`);

async function toasts(page) {
  await page.goto(url(PAGE.toast));
  await showParts(page);
  return page.locator(".nd-toast");
}

test("a message is a status, and its region is armed after the page settles", async ({ page }) => {
  const all = await toasts(page);
  expect(await all.count()).toBeGreaterThan(0);
  const roles = await all.evaluateAll((els) => els.map((el) => el.getAttribute("role")));
  expect([...new Set(roles)]).toEqual(["status"]);

  // QUIET WHILE THE PAGE PAINTS, ARMED AFTERWARDS. A message that shipped with
  // the page has already been read by the time a reader reaches it, so a page
  // of them announcing themselves on load is a page talking about itself.
  // `inputs.js` and `carousel.js` make the same switch for the same reason.
  await page.waitForFunction(() =>
    [...document.querySelectorAll(".nd-toast")].every(
      (el) => el.getAttribute("aria-live") !== "off",
    ),
  );

  // And what comes back is exactly what the caller wrote, including nothing:
  // `role="status"` already implies polite, so an attribute written at that
  // value would be the component overruling the role it just chose.
  const live = await all.evaluateAll((els) => els.map((el) => el.getAttribute("aria-live")));
  expect(live.filter((one) => one === "polite"), "a status was told to be polite").toEqual([]);
  expect(live.some((one) => one === null), "every message overrides its own role").toBe(true);
  expect(live.some((one) => one === "assertive"), "the loud one is not demonstrated").toBe(true);
});

test("with the script blocked the message is whole, closes nothing, and covers nothing", async ({
  browser,
}) => {
  const context = await browser.newContext({ javaScriptEnabled: false });
  const page = await context.newPage();
  await page.goto(url(PAGE.toast));
  await showParts(page);
  // The words are in the markup, so a page that never ran a script still says
  // what happened. The control is not, because it would close nothing.
  await expect(named(page, "toast-saved").locator(".nd-toast-body")).toBeVisible();
  await expect(named(page, "toast-saved").locator(".nd-toast-dismiss")).toBeHidden();

  // AND IT DOES NOT FLOAT, WHICH THIS TEST USED TO ASSERT THE OPPOSITE OF.
  //
  // A dismiss control that ships hidden and a `position: fixed` panel are two
  // correct decisions that together make one defect: the only floating surface
  // in the system that draws a panel over the page with no way out of it.
  // `.nd-dialog` and `.nd-popover` never open at all without a script and a
  // `Menu` does not render, and this had no such rule. In flow it is a message
  // IN the page rather than over it, and nothing is lost — the words are the
  // same words.
  const laid = await named(page, "toast-saved").evaluate((el) => getComputedStyle(el).position);
  expect(laid, "a message with no way out is laid over the page").not.toBe("fixed");

  // Nothing underneath it is covered: the message contributes its own height
  // to the stage it sits in, which an out-of-flow box cannot do. Scrolled to
  // first, because the question is asked in viewport coordinates.
  await named(page, "toast-saved").scrollIntoViewIfNeeded();
  const laidOut = await page.evaluate(() => {
    const toast = document.querySelector('[data-nd-toast="toast-saved"]');
    const box = toast.getBoundingClientRect();
    const at = document.elementFromPoint(box.left + box.width / 2, box.top + box.height / 2);
    const stage = toast.parentElement.getBoundingClientRect();
    return {
      own: toast.contains(at),
      inside: box.height > 0 && stage.height >= box.height,
    };
  });
  expect(laidOut.own, "something is sitting over a message nothing can close").toBe(true);
  expect(laidOut.inside, "the message is not where the layout put it").toBe(true);
  await context.close();
});

test("the script puts it back in its corner", async ({ page }) => {
  // The other half: with a script there IS a way out, so the message may float
  // again. `data-nd-scripting` is stamped by the inline head script before the
  // first paint, so the corner is its first position and not a second one.
  const all = await toasts(page);
  expect(await all.count()).toBeGreaterThan(0);
  const laid = await named(page, "toast-saved").evaluate((el) => getComputedStyle(el).position);
  expect(laid).toBe("fixed");
});

test("the script reveals the control, and pressing it takes the message away", async ({ page }) => {
  await toasts(page);
  const toast = named(page, "toast-saved");
  const control = toast.locator(".nd-toast-dismiss");
  await expect(control).toBeVisible();
  expect(await control.evaluate((el) => el.tagName)).toBe("BUTTON");
  expect(await control.getAttribute("type")).toBe("button");
  // A glyph, so the name is the only thing a screen reader has.
  expect(await control.getAttribute("aria-label")).toMatch(/\S\s\S/);
  const box = await control.boundingBox();
  expect(box.width).toBeGreaterThanOrEqual(24);
  expect(box.height).toBeGreaterThanOrEqual(24);

  await control.click();
  await expect(toast).toBeHidden();
  // THE ELEMENT STAYS. A live region taken out of the document announces
  // nothing the next time a caller writes into it.
  await expect(toast).toBeAttached();
  expect(await toast.getAttribute("data-nd-dismissed")).toBe("");
  // AND FOCUS IS NOT LEFT WHERE THE READER CANNOT SEE IT.
  const landed = await page.evaluate(() => document.activeElement !== document.body);
  expect(landed, "focus fell to the body when a message was dismissed").toBe(true);
});

test("no timer unless a caller asked for one", async ({ page }) => {
  const all = await toasts(page);
  const timers = await all.evaluateAll((els) =>
    els.map((el) => ({ name: el.dataset.ndToast, every: el.dataset.ndDuration ?? null })),
  );
  // The default is a message that stays, which is the component's one opinion.
  expect(timers.some((one) => one.every === null), "no message without a timer").toBe(true);
  expect(timers.some((one) => one.every), "the timer is not demonstrated").toBe(true);

  // And a message with no `duration` is still there after longer than the
  // longest one on the page.
  await page.waitForTimeout(1200);
  await expect(named(page, "toast-saved")).toBeVisible();
});

test("the timer holds under a pointer and under the focus", async ({ page }) => {
  await toasts(page);
  const toast = named(page, "toast-sent");
  await expect(toast).toBeVisible();

  await toast.hover();
  await expect(toast).toHaveAttribute("data-nd-paused", "");
  // A reader who moved onto the message is a reader still reading it, so the
  // wait must not run out underneath them.
  await page.waitForTimeout(400);
  await expect(toast).toBeVisible();

  // The focus holds it too, and counting the two separately is what stops one
  // of them leaving from restarting the wait under the other.
  await toast.locator(".nd-toast-dismiss").focus();
  await page.locator("h1").hover();
  await expect(toast).toHaveAttribute("data-nd-paused", "");

  await page.locator("h1").click();
  await expect(toast).not.toHaveAttribute("data-nd-paused", "");
});

test("the tone paints an edge and never carries the message alone", async ({ page }) => {
  const all = await toasts(page);
  const silent = await all.evaluateAll((els) =>
    els
      .filter((el) => el.dataset.ndTone)
      .filter((el) => !el.querySelector(".nd-toast-body")?.textContent?.trim())
      .map((el) => el.dataset.ndTone),
  );
  // `NOCTUA.md §8.8.6`: the words are the signal and the colour is a second one.
  expect(silent, "a message is saying it in colour alone").toEqual([]);

  const titles = await page.locator(".nd-toast-title").evaluateAll((els) => [
    ...new Set(els.map((el) => el.tagName)),
  ]);
  // A `<strong>` and never a heading: a message arriving after the page has
  // loaded must not rewrite the outline under a reader navigating by it.
  expect(titles).toEqual(["STRONG"]);
});

test("it floats in a corner it was told to, and carries no shadow", async ({ page }) => {
  const all = await toasts(page);
  const shapes = await all.evaluateAll((els) =>
    els.map((el) => ({
      name: el.dataset.ndToast,
      position: getComputedStyle(el).position,
      shadow: getComputedStyle(el).boxShadow,
    })),
  );
  for (const shape of shapes) {
    expect(shape.position, `${shape.name} is not floating`).toBe("fixed");
    expect(shape.shadow, `${shape.name} is drawing an elevated surface`).toBe("none");
  }

  // TOP AND BOTTOM ARE DIFFERENT CORNERS, which is the whole of `placement`.
  const where = await page.evaluate(() => {
    const box = (name) =>
      document.querySelector(`[data-nd-toast="${name}"]`).getBoundingClientRect();
    return { top: box("toast-failed").top, bottom: box("toast-offline").top };
  });
  expect(where.top).toBeLessThan(where.bottom);
});

test("a floating message stays inside the page it is documented on", async ({ page }) => {
  await page.setViewportSize({ width: 320, height: 800 });
  const all = await toasts(page);
  // `.nd-example-toasts` is `contain: layout`, so a `position: fixed` message is
  // pinned to its own example rather than to the window. Without it, four of
  // them stack in one corner over the documentation about them.
  const outside = await all.evaluateAll((els) => {
    const main = document.querySelector(".nd-main").getBoundingClientRect();
    return els
      .filter((el) => {
        const box = el.getBoundingClientRect();
        return box.right > main.right + 1 || box.left < main.left - 1;
      })
      .map((el) => el.dataset.ndToast);
  });
  expect(outside, "a message escaped the column it belongs to").toEqual([]);
});

// --- A message with something to do in it ------------------------------------

const REVIEW = "toast-review";

test("the message keeps its link when the region is armed", async ({ page }) => {
  // WHAT WENT WRONG. `announce()` took the body's `textContent`, emptied the
  // body and wrote the string back through `say`, which assigns `textContent`.
  // That flattens the message: `children`'s contract says it "may hold a
  // `Link` to whatever a reader has to do about it", and that link was deleted
  // on the one path that matters — a message that ARRIVED, after the page had
  // finished loading, which is the only kind a toast is really for.
  //
  // The nodes are moved out and back now, so nothing is ever copied as text.
  await toasts(page);
  await page.waitForLoadState("load");

  const survived = await page.evaluate(async () => {
    const original = document.querySelector('[data-nd-toast="toast-review"]');
    const arriving = original.cloneNode(true);
    arriving.dataset.ndToast = "toast-arrived";
    delete arriving.dataset.ndEnhanced;
    original.parentElement.append(arriving);

    const { enhanceToasts } = await import("/assets/toast.js");
    enhanceToasts(arriving.parentElement);
    await new Promise((settle) => requestAnimationFrame(() => requestAnimationFrame(settle)));

    const body = arriving.querySelector("[data-nd-toast-body]");
    return {
      links: body.querySelectorAll("a[href]").length,
      words: body.textContent.trim().length,
    };
  });
  expect(survived.links, "the message's own link was deleted on arrival").toBe(1);
  expect(survived.words, "the message lost its words as well").toBeGreaterThan(10);
});

test("the wait comes back when the focus leaves a message holding two controls", async ({
  page,
}) => {
  // THE COUNTER COULD NEVER RETURN TO ZERO. `focusin` raised it on every
  // arrival and `focusout` lowered it only when the focus had left the
  // message, so moving from the link to the dismiss control — one Tab, inside
  // one message — raised it twice and lowered it once. The timer then stopped
  // for good on exactly the message `children` exists to allow.
  await toasts(page);
  const toast = named(page, REVIEW);
  await expect(toast).toBeVisible();
  const link = toast.locator("a[href]");
  const control = toast.locator(".nd-toast-dismiss");

  await link.focus();
  await expect(toast).toHaveAttribute("data-nd-paused", "");
  // One Tab inside the message: from the link to the control it ships with.
  await control.focus();
  await expect(toast, "the message let go while the focus was still in it").toHaveAttribute(
    "data-nd-paused",
    "",
  );

  // And out again, which has to release it exactly once.
  await page.locator("h1").click();
  await expect(toast, "the wait never came back after two focusables").not.toHaveAttribute(
    "data-nd-paused",
    "",
  );
});

// --- The time left, and the way to see it again -----------------------------
//
// TWO HALVES THAT BELONG IN DIFFERENT PLACES, and this file measures both.
// `countdown_label` is the component's, because a message with a deadline owes
// its reader the deadline wherever it is used. The control that raises the
// message again is this site's, because raising a message is the job of the
// page that raises it, and `spec/components/toast.toml` writes down why.

const CLOCK = '[data-nd-toast="toast-sent"] [data-nd-toast-countdown]';

/** The number the countdown is showing, or nothing if it shows no number. */
const seconds = async (page) => {
  const words = await page.locator(CLOCK).textContent();
  const found = /\d+/.exec(words ?? "");
  return found ? Number(found[0]) : null;
};

test("a timed message shows what is left of the wait, and holds it under a pointer", async ({
  page,
}) => {
  await page.goto(url(PAGE.toast));
  await showParts(page);
  const clock = page.locator(CLOCK);
  await expect(clock, "the countdown was never revealed").toBeVisible();

  // It counts DOWN, which is the one thing a number on a timer has to do.
  const first = await seconds(page);
  expect(first, "the countdown drew no number").toBeGreaterThan(0);
  await expect
    .poll(async () => await seconds(page), { timeout: 4000 })
    .toBeLessThan(first);

  // AND IT STANDS STILL WHILE THE WAIT IS HELD. A number counting down beside
  // a timer that is not running says something untrue about the message.
  const toast = named(page, "toast-sent");
  await toast.hover();
  await expect(toast).toHaveAttribute("data-nd-paused", "");
  const held = await seconds(page);
  await page.waitForTimeout(1200);
  expect(await seconds(page), "the countdown ran while the wait was held").toBe(held);

  // Leaving restarts the WHOLE wait, which `toast.toml` guarantees, so the
  // number goes back up rather than carrying on from where it stopped.
  await page.locator("h1").hover();
  await expect(toast).not.toHaveAttribute("data-nd-paused", "");
  await expect.poll(async () => await seconds(page), { timeout: 2000 }).toBeGreaterThan(held);
});

test("the example that vanishes can be run again", async ({ page }) => {
  await page.goto(url(PAGE.toast));
  await showParts(page);
  const toast = named(page, "toast-sent");

  // Six seconds, and then the box is empty. That is the state a reader who
  // arrives late has always met, and the control below is the answer to it.
  await expect(toast).toHaveAttribute("data-nd-dismissed", "", { timeout: 12000 });

  await page.locator('[data-nd-action="toast-replay"]').first().click();
  await expect(toast, "the message never came back").not.toHaveAttribute("data-nd-dismissed", "");
  // And the wait is running again rather than merely being visible again: a
  // message put back with a dead timer is a picture of the example.
  await expect(page.locator(CLOCK)).toBeVisible();
  expect(await seconds(page), "the wait was not re-armed").toBeGreaterThan(0);
});

test("with the script blocked no countdown is drawn", async ({ browser }) => {
  const context = await browser.newContext({ javaScriptEnabled: false });
  const page = await context.newPage();
  await page.goto(url(PAGE.toast));
  await showParts(page);
  // The message never leaves without a script, so a number counting down to
  // nothing would be the same lie the dismiss control ships hidden to avoid.
  await expect(page.locator(CLOCK), "a countdown appeared with no timer").toBeHidden();
  await context.close();
});
