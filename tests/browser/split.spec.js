// Two panes that share a box, and the line between them.
//
// WHAT THE CASES CANNOT SAY. `spec/components/split.cases.toml` holds the
// markup and the keys: what the handle claims, what a script adds, and which
// number each key leaves. A split is about where a line is, and that is asked
// here: that the two panes are in the proportion the number names, that a
// pointer and a finger move the line to where they are, that a narrow box
// stacks the panes or leaves one out, and that it is the split's own box that
// is measured and not the window. And two things a picture of the page cannot
// show: that panes stacked in a window can be scrolled to their end, and that
// a pane the line has taken to nothing holds nothing the keyboard can reach.

import { expect, test } from "@playwright/test";

import { LOCALES, PAGE, showParts, url } from "./pages.js";
import { differing, outside, pixelAt } from "./pixels.js";

async function open(page, locale = "en") {
  await page.goto(url(PAGE.split, locale));
  await showParts(page);
}

const split = (page, name) => page.locator(`[data-nd-split="${name}"]`).first();
const handleOf = (page, name) => split(page, name).locator(":scope > .nd-split-track > .nd-split-handle");
const paneOf = (page, name, side) => page.locator(`#nd-split-${name}-${side}`);

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

const box = (locator) =>
  locator.evaluate((one) => {
    const at = one.getBoundingClientRect();
    return { left: at.left, right: at.right, top: at.top, bottom: at.bottom, width: at.width, height: at.height };
  });

/** Where a split's three parts are, and the share of the box its first pane has. */
async function parts(page, name) {
  const whole = await box(split(page, name));
  const start = await box(paneOf(page, name, "start"));
  const end = await box(paneOf(page, name, "end"));
  const handle = await box(handleOf(page, name));
  return { whole, start, end, handle, share: (start.width / (start.width + end.width)) * 100 };
}

/** What the handle tells a screen reader its value is. */
const said = (page, name) => handleOf(page, name).getAttribute("aria-valuenow");

/** Every `nd-split-resize` the page hears from now on. */
async function listen(page) {
  await page.evaluate(() => {
    window.heard = [];
    document.addEventListener("nd-split-resize", (event) => window.heard.push(event.detail));
  });
  return () => page.evaluate(() => window.heard);
}

for (const width of [768, 1280]) {
  test(`the panes are in the proportion the value names, at ${width}`, async ({ page }) => {
    await page.setViewportSize({ width, height: 900 });
    for (const locale of LOCALES) {
      await open(page, locale);
      const floor = await token(page, "--noctua-target-min");
      for (const [name, value] of [["split-basic", 50], ["split-limits", 30]]) {
        const { whole, start, end, handle, share } = await parts(page, name);
        expect(share, `${locale} ${name}: the first pane is not its share`).toBeCloseTo(value, 0);
        // SIDE BY SIDE, AND THE THREE ARE THE WHOLE BOX with nothing between.
        expect(start.left, `${locale} ${name}`).toBeCloseTo(whole.left, 1);
        expect(handle.left, `${locale} ${name}: a gap before the handle`).toBeCloseTo(start.right, 1);
        expect(end.left, `${locale} ${name}: a gap after the handle`).toBeCloseTo(handle.right, 1);
        expect(end.right, `${locale} ${name}`).toBeCloseTo(whole.right, 1);
        expect(end.top, `${locale} ${name}: the panes are not side by side`).toBeCloseTo(start.top, 1);
        expect(start.height, `${locale} ${name}: the panes are not one height`).toBeCloseTo(end.height, 1);
        expect(handle.height, `${locale} ${name}: the handle is not the split's height`).toBeCloseTo(whole.height, 1);
        // THE HANDLE IS A TARGET OF THE SMALLEST SIZE (WCAG 2.5.8).
        expect(handle.width, `${locale} ${name}: the handle is under the smallest target`).toBeGreaterThanOrEqual(floor - 0.05);
      }
    }
  });
}

test("the handle draws one line down its middle, thinner than itself", async ({ page }) => {
  await open(page);
  const handle = handleOf(page, "split-basic");
  await handle.scrollIntoViewIfNeeded();
  const outer = await box(handle);
  const line = await box(handle.locator(".nd-split-handle-line"));
  expect(line.width, "the handle draws no line").toBeGreaterThanOrEqual(1);
  expect(line.width, "the line is as wide as the target").toBeLessThan(outer.width / 4);
  expect((line.left + line.right) / 2, "the line is off the handle's middle").toBeCloseTo((outer.left + outer.right) / 2, 0);
  expect(line.height).toBeCloseTo(outer.height, 1);
  // AND IT IS DRAWN: the line is not the colour of the ground beside it.
  const shot = await page.screenshot();
  const middle = (outer.top + outer.bottom) / 2;
  const drawn = await pixelAt(page, shot, (line.left + line.right) / 2, middle);
  const ground = await pixelAt(page, shot, outer.left + 2, middle);
  expect(drawn, "the line is the colour of the page").not.toEqual(ground);
});

test("a narrow box stacks the panes, leaves one out, or keeps them, and it is the split's own box", async ({ page }) => {
  // A WIDE WINDOW, so a split that collapses here collapses for its column.
  await page.setViewportSize({ width: 1280, height: 900 });
  await open(page);

  // THE COLUMN EACH OF THESE STANDS IN IS A REAL ONE: narrower than the `sm`
  // breakpoint, and wide enough to read. A box that had collapsed to nothing
  // would pass every comparison below between widths that were all nothing.
  const column = await token(page, "--noctua-space-64");
  for (const name of ["split-stack", "split-hide-start", "split-hide-end", "split-none"]) {
    expect((await box(split(page, name))).width, `${name}: the example's column is not its width`).toBeCloseTo(column, 0);
  }

  // STACK: one above the other, each the whole width, and no handle.
  const stack = await box(split(page, "split-stack"));
  const top = await box(paneOf(page, "split-stack", "start"));
  const under = await box(paneOf(page, "split-stack", "end"));
  expect(under.top, "the panes of a narrow split are side by side").toBeGreaterThanOrEqual(top.bottom - 0.5);
  expect(top.width).toBeCloseTo(stack.width, 1);
  expect(under.width).toBeCloseTo(stack.width, 1);
  await expect(handleOf(page, "split-stack"), "a stacked split keeps a line to move").toBeHidden();
  // No line, and so no tab stop: a handle nobody sees cannot hold the focus.
  const held = await handleOf(page, "split-stack").evaluate((one) => {
    one.focus();
    return document.activeElement === one;
  });
  expect(held, "a hidden handle took the focus").toBe(false);

  // HIDE: one pane is out, and the other is the whole box.
  await expect(paneOf(page, "split-hide-start", "start")).toBeHidden();
  await expect(handleOf(page, "split-hide-start")).toBeHidden();
  expect((await box(paneOf(page, "split-hide-start", "end"))).width).toBeCloseTo((await box(split(page, "split-hide-start"))).width, 1);
  await expect(paneOf(page, "split-hide-end", "end")).toBeHidden();
  await expect(handleOf(page, "split-hide-end")).toBeHidden();
  expect((await box(paneOf(page, "split-hide-end", "start"))).width).toBeCloseTo((await box(split(page, "split-hide-end"))).width, 1);

  // NONE: side by side in the same narrow box, with its handle.
  const kept = await parts(page, "split-none");
  expect(kept.end.top, "`none` stacked its panes").toBeCloseTo(kept.start.top, 1);
  expect(kept.share).toBeCloseTo(50, 0);
  await expect(handleOf(page, "split-none")).toBeVisible();

  // AND THE SAME DECLARATION IN A WIDE BOX IS SIDE BY SIDE. `split-basic`
  // declares `stack` as `split-stack` does, and differs only in its column.
  expect(await split(page, "split-basic").getAttribute("data-nd-collapse")).toBe("stack");
  const wide = await parts(page, "split-basic");
  expect(wide.end.top, "a wide split is stacked").toBeCloseTo(wide.start.top, 1);

  // A NARROW WINDOW MAKES EVERY BOX NARROW, and then it stacks too.
  await page.setViewportSize({ width: 320, height: 900 });
  const small = await parts(page, "split-basic");
  expect(small.end.top, "a split in a narrow window is side by side").toBeGreaterThanOrEqual(small.start.bottom - 0.5);
  const sideways = await page.evaluate(() => document.documentElement.scrollWidth - document.documentElement.clientWidth);
  expect(sideways, "the page scrolls sideways").toBeLessThanOrEqual(0);
});

test("stacked in a window, the panes scroll and nothing is cut off", async ({ page }) => {
  // NARROW, so the two examples that stand in a window stack.
  await page.setViewportSize({ width: 320, height: 900 });
  for (const locale of LOCALES) {
    await open(page, locale);
    for (const name of ["overview-split", "pane-scroll"]) {
      const whole = await box(split(page, name));
      const start = await box(paneOf(page, name, "start"));
      const end = paneOf(page, name, "end");
      expect((await box(end)).top, `${locale} ${name}: the panes are not stacked`).toBeGreaterThanOrEqual(start.bottom - 0.5);
      // Taller than the window together, or the example shows nothing here.
      expect((await box(end)).bottom, `${locale} ${name}: the panes fit, so nothing is asked`).toBeGreaterThan(whole.bottom + 1);
      // Scrolled to its end, the track brings the last of the second pane
      // inside the split. A track that cut it off would leave it where no
      // pointer, finger or key can reach.
      await split(page, name)
        .locator(":scope > .nd-split-track")
        .evaluate((track) => track.scrollTo(0, track.scrollHeight));
      expect((await box(end)).bottom, `${locale} ${name}: the end of the second pane is out of reach`).toBeLessThanOrEqual(whole.bottom + 0.5);
    }
  }
});

test("a pane the line puts away is out of the tab order, and comes back with the line", async ({ page }) => {
  await page.setViewportSize({ width: 1280, height: 900 });
  await open(page);
  const name = "handle-away";
  // Whether a control in a pane can take the focus: one is added to each pane
  // the first time, because what the example holds is text.
  const reach = (side) =>
    paneOf(page, name, side).evaluate((pane) => {
      let probe = pane.querySelector(":scope > [data-probe]");
      if (!probe) {
        probe = document.createElement("button");
        probe.type = "button";
        probe.dataset.probe = "";
        probe.textContent = "probe";
        pane.append(probe);
      }
      probe.focus();
      return document.activeElement === probe;
    });
  expect(await reach("start"), "a pane with a share cannot be reached").toBe(true);
  expect(await reach("end")).toBe(true);

  const handle = handleOf(page, name);
  await handle.focus();
  await page.keyboard.press("Enter");
  expect(await said(page, name)).toBe("0");
  expect(await reach("start"), "a pane put away is still in the tab order").toBe(false);
  expect(await reach("end")).toBe(true);
  await handle.focus();
  await page.keyboard.press("Enter");
  expect(await said(page, name)).toBe("50");
  expect(await reach("start"), "a pane brought back cannot be reached").toBe(true);

  await handle.focus();
  await page.keyboard.press("End");
  expect(await said(page, name)).toBe("100");
  expect(await reach("end"), "the second pane put away is still in the tab order").toBe(false);
  expect(await reach("start")).toBe(true);

  // STACKED, NOTHING IS PUT AWAY: panes one above the other take no share,
  // and both are drawn whatever the number says.
  await page.setViewportSize({ width: 320, height: 900 });
  expect(await reach("end"), "a stacked pane was put away by a share it does not take").toBe(true);
  // And a split that never collapses is side by side in the same narrow box,
  // by its own rules and not the query's, and puts the pane away there too.
  await split(page, name).evaluate((one) => one.setAttribute("data-nd-collapse", "none"));
  expect(await reach("end"), "a split that never collapses kept a pane it put away").toBe(false);
  expect(await reach("start")).toBe(true);
});

test("the keys move the line, the panes follow, and the split says so", async ({ page }) => {
  await open(page);
  const heard = await listen(page);
  const handle = handleOf(page, "split-basic");
  await handle.focus();
  for (let press = 0; press < 5; press += 1) await page.keyboard.press("ArrowRight");
  expect(await said(page, "split-basic")).toBe("55");
  // THE NUMBER A SCREEN READER IS TOLD IS THE NUMBER THE BOX IS DIVIDED BY.
  expect((await parts(page, "split-basic")).share, "the panes did not follow the keys").toBeCloseTo(55, 0);
  expect(await heard()).toEqual(
    [51, 52, 53, 54, 55].map((value) => ({ name: "split-basic", value, dragging: false })),
  );

  await page.keyboard.press("End");
  expect((await parts(page, "split-basic")).share).toBeCloseTo(80, 0);
  await page.keyboard.press("Home");
  expect((await parts(page, "split-basic")).share).toBeCloseTo(20, 0);
  // A key at a limit moves nothing, and says nothing.
  const before = (await heard()).length;
  await page.keyboard.press("ArrowLeft");
  await page.keyboard.press("Home");
  expect((await heard()).length, "a key that moved nothing was announced as a move").toBe(before);
  await expect(handle).toBeFocused();
  // A KEY HELD WITH ANOTHER IS SOMEBODY ELSE'S CHORD, and the line stays.
  await page.keyboard.press("Control+ArrowRight");
  await page.keyboard.press("Alt+End");
  expect(await said(page, "split-basic"), "a chord moved the line").toBe("20");
});

test("a key at a limit does not scroll the page instead", async ({ page }) => {
  await page.setViewportSize({ width: 1280, height: 400 });
  await open(page);
  const handle = handleOf(page, "handle-least");
  await handle.focus();
  const at = await page.evaluate(() => window.scrollY);
  expect(at, "the page did not scroll to the handle, so Home has nothing to undo").toBeGreaterThan(0);
  // Already at its least: Home moves nothing, and the page stays where it is.
  await page.keyboard.press("Home");
  await page.keyboard.press("PageDown");
  expect(await page.evaluate(() => window.scrollY), "a key the handle answers scrolled the page").toBe(at);
  // AT ITS LEAST WITH NOWHERE REMEMBERED, Enter takes the pane to the middle of
  // what it may be, and Enter again puts it away.
  await page.keyboard.press("Enter");
  expect(await said(page, "handle-least")).toBe("50");
  await page.keyboard.press("Enter");
  expect(await said(page, "handle-least")).toBe("20");
  await page.keyboard.press("Enter");
  expect(await said(page, "handle-least")).toBe("50");
});

test("a pointer drags the line to where it is, and no further than the limits", async ({ page }) => {
  await open(page);
  const heard = await listen(page);
  const name = "split-basic";
  const handle = handleOf(page, name);
  await handle.scrollIntoViewIfNeeded();
  const from = await parts(page, name);
  const x = (from.handle.left + from.handle.right) / 2;
  const y = (from.handle.top + from.handle.bottom) / 2;

  await page.mouse.move(x, y);
  await page.mouse.down();
  await page.mouse.move(x + 60, y + 10, { steps: 6 });
  // WHILE IT IS HELD the handle says so, and the line is under the pointer.
  expect(await handle.getAttribute("data-nd-dragging")).toBe("");
  const held = await parts(page, name);
  expect((held.handle.left + held.handle.right) / 2, "the line is not under the pointer").toBeCloseTo(x + 60, 0);
  expect(held.start.width, "the first pane did not grow by what the line moved").toBeCloseTo(from.start.width + 60, 0);
  // And what a screen reader is told is where the line is.
  expect(Number(await said(page, name))).toBeCloseTo(held.share, 1);

  // FAR PAST THE LIMIT, and the line stops at it.
  await page.mouse.move(from.whole.right + 200, y, { steps: 4 });
  expect((await parts(page, name)).share, "the line was dragged past its limit").toBeCloseTo(80, 0);
  await page.mouse.up();
  expect(await handle.getAttribute("data-nd-dragging"), "the handle still says it is dragged").toBeNull();
  // The press gave the handle the focus, so the keys go on from where it is.
  await expect(handle).toBeFocused();
  // Dragged over two panes of text, and none of it was selected.
  expect(await page.evaluate(() => String(window.getSelection())), "the drag selected text").toBe("");

  const all = await heard();
  expect(all.length).toBeGreaterThan(2);
  // Every move while the pointer held it, and the last word with it let go.
  expect(all.slice(0, -1).every((one) => one.dragging === true && one.name === name)).toBe(true);
  expect(all.at(-1)).toEqual({ name, value: 80, dragging: false });

  // A pointer let go moves nothing more.
  await page.mouse.move(x, y, { steps: 3 });
  expect((await parts(page, name)).share).toBeCloseTo(80, 0);
});

test("a finger drags it too, and the page does not scroll under the finger", async ({ browser }) => {
  const context = await browser.newContext({ hasTouch: true, viewport: { width: 1280, height: 500 } });
  const page = await context.newPage();
  await open(page);
  const name = "split-basic";
  await handleOf(page, name).scrollIntoViewIfNeeded();
  const from = await parts(page, name);
  const x = (from.handle.left + from.handle.right) / 2;
  const y = (from.handle.top + from.handle.bottom) / 2;
  const scrolled = await page.evaluate(() => window.scrollY);

  // A finger down on the line and moved across and down the page. Without
  // `touch-action: none` the engine takes that for a scroll, cancels the
  // pointer and moves the page.
  const cdp = await context.newCDPSession(page);
  const touch = (type, tx, ty) =>
    cdp.send("Input.dispatchTouchEvent", {
      type,
      touchPoints: type === "touchEnd" ? [] : [{ x: Math.round(tx), y: Math.round(ty), id: 1 }],
    });
  await touch("touchStart", x, y);
  for (let step = 1; step <= 8; step += 1) await touch("touchMove", x + step * 10, y - step * 8);
  // A touch's moves reach the page with its frames, so the last one can still
  // be on its way when the dispatch returns. Asked again until it lands, and
  // failed if it never does.
  await expect
    .poll(async () => (await parts(page, name)).start.width, { message: "the line did not follow the finger" })
    .toBeCloseTo(from.start.width + 80, 0);
  await touch("touchEnd", 0, 0);

  expect(await page.evaluate(() => window.scrollY), "the page scrolled under the finger").toBe(scrolled);
  expect(await handleOf(page, name).getAttribute("data-nd-dragging")).toBeNull();
  await context.close();
});

test("read from the right, the first pane is on the right and the line moves the way a key points", async ({ page }) => {
  await open(page);
  const name = "handle-mirrored";
  const from = await parts(page, name);
  expect(from.start.right, "the first pane is not at the right").toBeCloseTo(from.whole.right, 1);
  expect(from.end.left).toBeCloseTo(from.whole.left, 1);
  expect(from.share).toBeCloseTo(30, 0);

  const handle = handleOf(page, name);
  await handle.focus();
  await page.keyboard.press("ArrowLeft");
  // The line went left, which here makes the first pane larger.
  expect(await said(page, name)).toBe("31");
  const moved = await parts(page, name);
  expect(moved.handle.left, "Left moved the line to the right").toBeLessThan(from.handle.left);

  // And a pointer likewise: dragged left, the line is under it.
  const x = (moved.handle.left + moved.handle.right) / 2;
  const y = (moved.handle.top + moved.handle.bottom) / 2;
  await page.mouse.move(x, y);
  await page.mouse.down();
  await page.mouse.move(x - 50, y, { steps: 5 });
  const held = await parts(page, name);
  await page.mouse.up();
  expect((held.handle.left + held.handle.right) / 2).toBeCloseTo(x - 50, 0);
  expect(held.start.width).toBeCloseTo(moved.start.width + 50, 0);
});

test("a value the caller writes is drawn and announced", async ({ page }) => {
  await open(page);
  const name = "split-basic";
  await split(page, name).evaluate((one) => one.setAttribute("data-nd-value", "35"));
  await expect(handleOf(page, name)).toHaveAttribute("aria-valuenow", "35");
  expect((await parts(page, name)).share).toBeCloseTo(35, 0);
  // And a limit the caller moves is the limit from then on.
  await split(page, name).evaluate((one) => one.setAttribute("data-nd-max", "30"));
  await expect(handleOf(page, name)).toHaveAttribute("aria-valuemax", "30");
  await expect(handleOf(page, name)).toHaveAttribute("aria-valuenow", "30");
  expect((await parts(page, name)).share).toBeCloseTo(30, 0);
});

test("a pane as tall as the split scrolls what is taller, and the other stays", async ({ page }) => {
  await open(page);
  const name = "pane-scroll";
  await split(page, name).scrollIntoViewIfNeeded();
  const whole = await box(split(page, name));
  const start = await box(paneOf(page, name, "start"));
  const end = paneOf(page, name, "end");
  expect(start.height, "a pane is not as tall as the split").toBeCloseTo(whole.height, 1);
  expect((await box(end)).height).toBeCloseTo(whole.height, 1);
  const scroll = await end.evaluate((one) => ({ all: one.scrollHeight, seen: one.clientHeight }));
  expect(scroll.all, "the example's pane fits, so it shows nothing about scrolling").toBeGreaterThan(scroll.seen + 1);
  await end.evaluate((one) => one.scrollTo(0, 60));
  expect(await end.evaluate((one) => one.scrollTop), "the pane is not what scrolls").toBe(60);
  expect(await paneOf(page, name, "start").evaluate((one) => one.scrollTop), "one pane scrolled the other").toBe(0);
  expect((await box(paneOf(page, name, "start"))).top).toBeCloseTo(start.top, 1);
});

test("a split inside a pane is measured by its own box", async ({ page }) => {
  await page.setViewportSize({ width: 1280, height: 900 });
  await open(page);
  const outer = await parts(page, "pane-outer");
  expect(outer.share).toBeCloseTo(30, 0);
  const inner = split(page, "pane-inner");
  const room = (await box(inner)).width;
  const limit = await token(page, "--noctua-space-16");
  // The `sm` breakpoint is thirty of the root's size, and step 16 is four.
  const sm = (limit / 4) * 30;
  const within = await parts(page, "pane-inner");
  if (room >= sm) {
    expect(within.end.top, "an inner split with room is stacked").toBeCloseTo(within.start.top, 1);
    expect(within.share).toBeCloseTo(40, 0);
  } else {
    expect(within.end.top, "an inner split with no room is side by side").toBeGreaterThanOrEqual(within.start.bottom - 0.5);
  }
  // AND IT TURNS WITH ITS OWN BOX: take the outer pane's room away with the
  // outer line, and the inner split stacks while the window has not changed.
  await split(page, "pane-outer").evaluate((one) => {
    one.setAttribute("data-nd-max", "100");
    one.setAttribute("data-nd-value", "85");
  });
  await expect(handleOf(page, "pane-outer")).toHaveAttribute("aria-valuenow", "85");
  expect((await box(inner)).width, "the inner split still has room").toBeLessThan(sm);
  const squeezed = await parts(page, "pane-inner");
  expect(squeezed.end.top, "the inner split did not stack when its own box narrowed").toBeGreaterThanOrEqual(
    squeezed.start.bottom - 0.5,
  );
});

test("with the script blocked it is two panes at their shares and a line nobody can focus", async ({ browser }) => {
  const context = await browser.newContext({ javaScriptEnabled: false, viewport: { width: 1280, height: 900 } });
  const page = await context.newPage();
  await page.goto(url(PAGE.split));
  await showParts(page);
  const name = "split-limits";
  expect((await parts(page, name)).share, "the stylesheet alone does not draw the value").toBeCloseTo(30, 0);
  const handle = handleOf(page, name);
  await expect(handle).toBeVisible();
  expect(await handle.getAttribute("role")).toBe("separator");
  // NOTHING IT CANNOT KEEP: no stop, no value, and no answer to a pointer.
  expect(await handle.getAttribute("tabindex")).toBeNull();
  expect(await handle.getAttribute("aria-valuenow")).toBeNull();
  const held = await handle.evaluate((one) => {
    one.focus();
    return document.activeElement === one;
  });
  expect(held, "a separator with no script took the focus").toBe(false);
  expect(await handle.evaluate((one) => getComputedStyle(one).cursor), "a line nothing moves offers to be moved").toBe("auto");
  await context.close();
});

test("the keyboard's ring is drawn inside the handle", async ({ page }) => {
  await open(page);
  const handle = handleOf(page, "split-basic");
  await handle.scrollIntoViewIfNeeded();
  const before = await page.screenshot();
  await handle.focus();
  await page.keyboard.press("Shift");
  const after = await page.screenshot();
  const rects = await handle.evaluate((one) =>
    [...one.getClientRects()].map((at) => [at.left, at.top, at.right, at.bottom]),
  );
  const strays = await outside(page, before, after, rects);
  expect(strays, "nothing changed when the handle took the focus").not.toBe(Infinity);
  expect(strays, "the ring is drawn over the panes").toBe(0);
});

test("a second seat's drag is drawn as the pointer's own", async ({ page }) => {
  await open(page);
  const handle = handleOf(page, "split-basic");
  await handle.scrollIntoViewIfNeeded();
  const picture = () => handle.screenshot();
  const rest = await picture();
  await handle.evaluate((one) => one.setAttribute("data-nd-dragging", ""));
  const own = await picture();
  await handle.evaluate((one) => {
    one.removeAttribute("data-nd-dragging");
    one.setAttribute("data-nd-seat-drag", "2");
  });
  const seat = await picture();
  expect(await differing(page, rest, own), "a dragged handle is drawn as one at rest").toBeGreaterThan(0);
  expect(await differing(page, own, seat), "a second seat's drag is not drawn as the pointer's").toBe(0);
});

test("releasing a split takes back the stop and the value, and leaves the line where it is", async ({ page }) => {
  await open(page);
  const name = "split-basic";
  const handle = handleOf(page, name);
  await handle.focus();
  await page.keyboard.press("PageUp");
  expect((await parts(page, name)).share).toBeCloseTo(60, 0);

  await page.evaluate(async () => {
    const { releaseSplits } = await import("../../assets/split.js");
    releaseSplits(document);
  });
  for (const attribute of ["tabindex", "aria-valuenow", "aria-valuemin", "aria-valuemax"]) {
    expect(await handle.getAttribute(attribute), `a released handle keeps ${attribute}`).toBeNull();
  }
  expect(await split(page, name).getAttribute("data-nd-enhanced")).toBeNull();
  expect((await parts(page, name)).share, "releasing moved the line").toBeCloseTo(60, 0);
  // And no pointer moves it any more.
  const at = await parts(page, name);
  const x = (at.handle.left + at.handle.right) / 2;
  const y = (at.handle.top + at.handle.bottom) / 2;
  await page.mouse.move(x, y);
  await page.mouse.down();
  await page.mouse.move(x + 60, y, { steps: 4 });
  await page.mouse.up();
  expect((await parts(page, name)).share, "a released split still answers a pointer").toBeCloseTo(60, 0);

  await page.evaluate(async () => {
    const { enhanceSplits } = await import("../../assets/split.js");
    enhanceSplits(document);
  });
  await expect(handle).toHaveAttribute("aria-valuenow", "60");
  await expect(handle).toHaveAttribute("tabindex", "0");
});
