// Lines as they arrive, and the end a log follows.
//
// WHAT THE CASES CANNOT SAY. `spec/components/log.cases.toml` has no box with a
// height, so nothing there scrolls. Asked here: that a log in a window is
// bounded by it and opens at its newest line, that a line added is brought into
// view while it follows, that scrolling away stops it and the end resumes it,
// that a selection held as a line arrives stops it and copying gives one line
// of text a line, that a log that does not follow stays where it is put, the
// page with the script blocked, and the release.

import { expect, test } from "@playwright/test";

import { LOCALES, PAGE, showParts, url } from "./pages.js";

async function open(page, locale = "en") {
  await page.goto(url(PAGE.log, locale));
  await showParts(page);
}

const log = (page, name) => page.locator(`[data-nd-log="${name}"]`).first();
const lines = (page, name) => log(page, name).locator(":scope > .nd-log-lines");
const resume = (page, name) => log(page, name).locator(":scope > .nd-log-resume");
const content = (page, name) => lines(page, name).locator(":scope > .nd-log-content");

/** How far the region is from its end, in pixels, and from its start. */
const where = (page, name) =>
  lines(page, name).evaluate((one) => ({
    end: one.scrollHeight - one.clientHeight - one.scrollTop,
    top: one.scrollTop,
    over: one.scrollHeight - one.clientHeight,
    across: one.scrollWidth - one.clientWidth,
  }));

/** Adds a line at the end of a log, as a caller re-rendering it would. */
const append = (page, name, text = "One more line.") =>
  content(page, name).evaluate((one, said) => {
    const line = one.lastElementChild.cloneNode(true);
    line.querySelector(".nd-log-line-text").textContent = said;
    one.append(line);
  }, text);

/** Every `nd-log-follow` the page hears from now on. */
async function listen(page) {
  await page.evaluate(() => {
    window.heard = [];
    document.addEventListener("nd-log-follow", (event) => window.heard.push(event.detail));
  });
  return () => page.evaluate(() => window.heard);
}

/** Scrolls a region to a place, as a reader's wheel or scrollbar does. */
const scrollTo = (page, name, top) =>
  lines(page, name).evaluate((one, at) => {
    one.scrollTop = at === "end" ? one.scrollHeight : at;
  }, top);

for (const width of [320, 768, 1280]) {
  test(`a log in a window is bounded by it, opens at its newest line and never scrolls sideways, at ${width}`, async ({
    page,
  }) => {
    await page.setViewportSize({ width, height: 900 });
    for (const locale of LOCALES) {
      await open(page, locale);
      for (const name of ["example-log-basic", "example-log-narrow", "overview-log"]) {
        const at = `${locale} ${width} ${name}`;
        const window_ = await log(page, name).evaluate((one) => {
          const outer = one.parentElement.getBoundingClientRect();
          const inner = one.getBoundingClientRect();
          return { outer: outer.bottom, inner: inner.bottom };
        });
        expect(window_.inner, `${at}: the log is taller than its window`).toBeLessThanOrEqual(window_.outer + 0.5);
        await expect.poll(async () => (await where(page, name)).over, { message: `${at}: nothing to scroll` }).toBeGreaterThan(0);
        await expect.poll(async () => (await where(page, name)).end, { message: `${at}: not at the newest line` }).toBeLessThanOrEqual(2);
        expect((await where(page, name)).across, `${at}: the lines scroll sideways`).toBeLessThanOrEqual(0);
      }
      // A LOG THAT DOES NOT FOLLOW OPENS AT ITS FIRST LINE.
      expect((await where(page, "example-log-record")).top, `${locale} ${width}: a record opened at its end`).toBe(0);
      // A LOG WITH ROOM FOR EVERY LINE is at its first line and its end at
      // once, and offers nothing to resume.
      expect((await where(page, "example-log-short")).over, `${locale} ${width}: three lines overflow`).toBeLessThanOrEqual(0);
      await expect(log(page, "example-log-short")).not.toHaveAttribute("data-nd-paused");
      await expect(resume(page, "example-log-short")).toBeHidden();
    }
  });
}

test("the region is a named log, and the keyboard reaches it", async ({ page }) => {
  await open(page);
  const region = page.getByRole("log", { name: "Daemon log" }).first();
  await expect(region).toBeAttached();
  await expect(lines(page, "example-log-basic")).toHaveAttribute("tabindex", "0");
  await lines(page, "example-log-basic").focus();
  await expect(lines(page, "example-log-basic")).toBeFocused();
  // THE SOURCE IS A NAME, and the time a `<time>`.
  const first = lines(page, "example-log-basic").locator(".nd-log-line").first();
  await expect(first.locator(":scope > time.nd-log-line-time")).toHaveText("09:41:02.118");
  await expect(first.locator(":scope > .nd-log-line-source")).toHaveClass(/nd-identifier/);
});

test("a line added while the log follows is brought into view", async ({ page }) => {
  await open(page);
  const name = "example-log-basic";
  const heard = await listen(page);
  await expect.poll(async () => (await where(page, name)).end).toBeLessThanOrEqual(2);
  for (let one = 0; one < 3; one += 1) await append(page, name);
  await expect.poll(async () => (await where(page, name)).end, { message: "the new lines are below the view" }).toBeLessThanOrEqual(2);
  await expect(log(page, name)).not.toHaveAttribute("data-nd-paused");
  await expect(resume(page, name)).toBeHidden();
  // EVERY LINE TALLER AND NONE ADDED, as a face that loads late makes them:
  // the end moved, and a log that follows goes with it.
  await content(page, name).evaluate((one) => {
    one.style.fontSize = "150%";
  });
  await expect.poll(async () => (await where(page, name)).end, { message: "taller lines left the end below the view" }).toBeLessThanOrEqual(2);
  expect(await heard(), "following that never stopped was said to change").toEqual([]);
});

test("scrolling away stops following, and the end resumes it", async ({ page }) => {
  await open(page);
  const name = "example-log-basic";
  const heard = await listen(page);
  await expect.poll(async () => (await where(page, name)).end).toBeLessThanOrEqual(2);
  await scrollTo(page, name, 0);
  await expect(log(page, name)).toHaveAttribute("data-nd-paused", "true");
  await expect(resume(page, name)).toBeVisible();
  // A LINE ARRIVING NOW LEAVES THE READER WHERE THEY ARE.
  await append(page, name);
  await page.evaluate(() => new Promise((done) => requestAnimationFrame(() => requestAnimationFrame(done))));
  expect((await where(page, name)).top, "a line arriving moved a reader who had scrolled away").toBe(0);
  await scrollTo(page, name, "end");
  await expect(log(page, name)).not.toHaveAttribute("data-nd-paused");
  await expect(resume(page, name)).toBeHidden();
  await expect.poll(heard).toEqual([
    { name, follow: false },
    { name, follow: true },
  ]);
  // THE ENGINE'S OWN KEYS DO THE SAME: Home leaves the end and End returns.
  await lines(page, name).focus();
  await page.keyboard.press("Home");
  await expect(log(page, name)).toHaveAttribute("data-nd-paused", "true");
  await page.keyboard.press("End");
  await expect(log(page, name)).not.toHaveAttribute("data-nd-paused");
  // AND SO DOES A WHEEL.
  await lines(page, name).hover();
  await page.mouse.wheel(0, -400);
  await expect(log(page, name)).toHaveAttribute("data-nd-paused", "true");
});

test("the resume control goes to the newest line and hands the keyboard to the region", async ({ page }) => {
  await open(page);
  const name = "example-log-basic";
  await expect.poll(async () => (await where(page, name)).end).toBeLessThanOrEqual(2);
  await scrollTo(page, name, 0);
  await expect(resume(page, name)).toBeVisible();
  await expect(resume(page, name)).toHaveAccessibleName("Go to the newest");
  // IT STANDS OVER THE REGION, inside the box, and is a target of the smallest size.
  const box = await resume(page, name).boundingBox();
  const region = await lines(page, name).boundingBox();
  expect(box.x + box.width).toBeLessThanOrEqual(region.x + region.width + 0.5);
  expect(box.y + box.height).toBeLessThanOrEqual(region.y + region.height + 0.5);
  await resume(page, name).focus();
  await page.keyboard.press("Enter");
  await expect.poll(async () => (await where(page, name)).end).toBeLessThanOrEqual(2);
  await expect(log(page, name)).not.toHaveAttribute("data-nd-paused");
  await expect(resume(page, name)).toBeHidden();
  await expect(lines(page, name), "the keyboard was left on a hidden control").toBeFocused();
});

test("a selection held as a line arrives stops following, and stays", async ({ page }) => {
  await open(page);
  const name = "example-log-basic";
  await expect.poll(async () => (await where(page, name)).end).toBeLessThanOrEqual(2);
  const before = await lines(page, name).evaluate((one) => {
    const range = document.createRange();
    range.selectNodeContents(one.querySelector(".nd-log-line:last-child > .nd-log-line-text"));
    const selection = document.getSelection();
    selection.removeAllRanges();
    selection.addRange(range);
    return { text: selection.toString(), top: one.scrollTop };
  });
  for (let one = 0; one < 4; one += 1) await append(page, name);
  await expect(log(page, name)).toHaveAttribute("data-nd-paused", "true");
  const after = await lines(page, name).evaluate((one) => ({
    text: document.getSelection().toString(),
    top: one.scrollTop,
  }));
  expect(after.text, "the selection was lost").toBe(before.text);
  expect(after.top, "the text being selected moved").toBe(before.top);
});

test("copying lines gives one line of text a line, its parts a space apart", async ({ page, context }) => {
  await context.grantPermissions(["clipboard-read", "clipboard-write"]);
  await open(page);
  const region = lines(page, "example-log-line-levels");
  const selected = await region.evaluate((one) => {
    const range = document.createRange();
    range.selectNodeContents(one);
    const selection = document.getSelection();
    selection.removeAllRanges();
    selection.addRange(range);
    return selection.toString();
  });
  const written = [
    "09:41:02.311 debug nc-ipc Read 42 commands from the catalog.",
    "09:41:03.005 info nc-ipc Listening on the local socket.",
    "09:41:07.420 warning nc-ipc A client was slow to answer the heartbeat.",
    "09:41:12.002 error nc-ipc A plugin was refused: its capability is not granted.",
  ];
  expect(selected.trim().split("\n")).toEqual(written);
  await region.focus();
  await page.keyboard.press("Control+KeyC");
  const copied = await page.evaluate(() => navigator.clipboard.readText());
  expect(copied.trim().split("\n"), "the clipboard holds what was drawn rather than what was written").toEqual(written);
});

test("a log that does not follow stays where the reader puts it", async ({ page }) => {
  await open(page);
  const name = "example-log-record";
  const heard = await listen(page);
  expect((await where(page, name)).top).toBe(0);
  await append(page, name);
  await page.evaluate(() => new Promise((done) => requestAnimationFrame(() => requestAnimationFrame(done))));
  expect((await where(page, name)).top, "a record followed a line").toBe(0);
  await scrollTo(page, name, 40);
  await scrollTo(page, name, "end");
  await expect(log(page, name)).not.toHaveAttribute("data-nd-paused");
  await expect(resume(page, name)).toHaveCount(0);
  expect(await heard()).toEqual([]);
});

test("a level is a word in a box with a rule, and a tone never stands alone", async ({ page }) => {
  await open(page);
  const rule = await page.evaluate(() => {
    const probe = document.createElement("div");
    probe.style.position = "absolute";
    probe.style.blockSize = "var(--noctua-border-rule)";
    document.body.append(probe);
    const size = probe.getBoundingClientRect().height;
    probe.remove();
    return size;
  });
  const levels = lines(page, "example-log-line-levels").locator(".nd-log-line-level");
  await expect(levels).toHaveText(["debug", "info", "warning", "error"]);
  for (const one of await levels.all()) {
    const drawn = await one.evaluate((level) => Number.parseFloat(getComputedStyle(level).borderInlineStartWidth));
    expect(drawn, "a level is drawn with no rule").toBeCloseTo(rule, 1);
  }
  // Every line that names a tone names its level in words.
  const bare = await page
    .locator(".nd-log-line[data-nd-tone]")
    .evaluateAll((all) => all.filter((line) => !line.querySelector(":scope > .nd-log-line-level")?.textContent.trim()).length);
  expect(bare, "a line says its level by colour alone").toBe(0);
});

test("a word longer than the line breaks inside itself, and a line break is kept", async ({ page }) => {
  await page.setViewportSize({ width: 320, height: 800 });
  await open(page);
  const text = lines(page, "example-log-line-long").locator(".nd-log-line-text");
  const region = await lines(page, "example-log-line-long").evaluate((one) => one.scrollWidth - one.clientWidth);
  expect(region, "a long word pushed the log sideways").toBeLessThanOrEqual(0);
  const rows = await text.evaluate((one) => one.getClientRects().length);
  expect(rows, "the message is one row").toBeGreaterThan(2);
});

test("with the script blocked it is a region that scrolls, and nothing offers to resume", async ({ browser }) => {
  const context = await browser.newContext({ javaScriptEnabled: false });
  const page = await context.newPage();
  try {
    await page.goto(url(PAGE.log));
    const name = "example-log-basic";
    await expect(lines(page, name)).toHaveAttribute("role", "log");
    await expect(lines(page, name)).toHaveAttribute("tabindex", "0");
    await expect(resume(page, name)).toBeHidden();
    expect((await where(page, name)).top).toBe(0);
  } finally {
    await context.close();
  }
});

test("releasing a log takes back what the script wrote", async ({ page }) => {
  await open(page);
  const name = "example-log-basic";
  await expect.poll(async () => (await where(page, name)).end).toBeLessThanOrEqual(2);
  await scrollTo(page, name, 0);
  await expect(log(page, name)).toHaveAttribute("data-nd-paused", "true");
  await page.evaluate(async () => {
    const { releaseLogs } = await import("../../assets/log.js");
    releaseLogs(document);
  });
  await expect(log(page, name)).not.toHaveAttribute("data-nd-paused");
  await expect(log(page, name)).not.toHaveAttribute("data-nd-enhanced");
  await expect(resume(page, name)).toBeHidden();
  const heard = await listen(page);
  await scrollTo(page, name, "end");
  await append(page, name);
  await page.evaluate(() => new Promise((done) => requestAnimationFrame(() => requestAnimationFrame(done))));
  expect(await heard(), "a released log still says it follows").toEqual([]);

  await page.evaluate(async () => {
    const { enhanceLogs } = await import("../../assets/log.js");
    enhanceLogs(document);
  });
  await expect.poll(async () => (await where(page, name)).end).toBeLessThanOrEqual(2);
  await scrollTo(page, name, 0);
  await expect(log(page, name)).toHaveAttribute("data-nd-paused", "true");
});

test("the resume control takes the page's face, and follows the interface-font setting", async ({ page }) => {
  // A FORM CONTROL DOES NOT TAKE THE PAGE'S FACE ON ITS OWN, and one told the
  // proportional face by name stays proportional when a reader chooses the
  // monospaced one for the interface (`NOCTUA.md §7.7`).
  await open(page);
  await page.evaluate(() => document.documentElement.setAttribute("data-noctua-font", "mono"));
  const faces = await page.locator(".nd-log-resume").first().evaluate((el) => ({
    control: getComputedStyle(el).fontFamily,
    page: getComputedStyle(document.body).fontFamily,
  }));
  expect(faces.control, "the control keeps a face of its own").toBe(faces.page);
});
