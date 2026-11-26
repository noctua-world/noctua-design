// The line a reader types on, and the block that is its cursor.
//
// WHAT THE CASES CANNOT SAY. `spec/components/prompt.cases.toml` holds the
// markup and Enter. Everything that makes a custom cursor worth having, or not
// worth having, is asked here (`NOCTUA.md §12.7`): that the block is where the
// engine's own caret is, for every kind of text; that a selection, an input
// method's composition, the clipboard and the accessible name are still the
// engine's; that the prompt grows and then scrolls; and that the block does
// not blink for a reader who asked for less movement.
//
// THE CARET IS READ FROM PIXELS, AND IT IS THE ENGINE'S. No interface says
// where a textarea draws its caret, and the usual answer, laying the text out
// again in a second box and measuring that, is the technique under test and
// would agree with itself. So the block is taken away, the caret the
// stylesheet made transparent is given its colour back and stopped from
// blinking, and two photographs, with the caret and without, differ in exactly
// the caret. Where those pixels are is where the engine put it.

import { expect, test } from "@playwright/test";

import { LOCALES, PAGE, showParts, url } from "./pages.js";
import { contrast, differing, extremes } from "./pixels.js";

// THE ENGINE'S SCROLLBARS ARE LEFT ON. The runner hides every scrollbar, so
// under it a scrolling textarea is as wide inside as one that does not scroll,
// and the rule that keeps the two boxes one width past the prompt's limit
// would decide nothing in any test here. A window draws its bars, and so does
// this file.
test.use({
  launchOptions: async ({ launchOptions }, use) =>
    use({ ...launchOptions, ignoreDefaultArgs: ["--hide-scrollbars"] }),
});

async function open(page, locale = "en") {
  await page.goto(url(PAGE.prompt, locale));
  await showParts(page);
  await expect(page.locator('[data-nd-prompt="example-prompt"][data-nd-enhanced]')).toBeAttached();
}

const prompt = (page, name = "example-prompt") => page.locator(`[data-nd-prompt="${name}"]`);
const input = (page, name) => prompt(page, name).locator(".nd-prompt-input");
const block = (page, name) => prompt(page, name).locator(".nd-prompt-cursor");

/** Two frames: long enough for a `selectionchange` to have been heard. */
const settle = (page) =>
  page.evaluate(() => new Promise((done) => requestAnimationFrame(() => requestAnimationFrame(done))));

/** Puts a text in the prompt with the caret before one UTF-16 unit of it. */
async function put(page, text, caret, name) {
  const box = input(page, name);
  await box.focus();
  await box.evaluate(
    (el, [value, at]) => {
      el.value = value;
      el.setSelectionRange(at, at);
    },
    [text, caret],
  );
  await settle(page);
}

/** What the mirror holds: the text before the block, the block's, and the rest. */
const drawn = (page, name) =>
  prompt(page, name)
    .locator(".nd-prompt-mirror")
    .evaluate((mirror) => {
      const [before, cursor, after] = mirror.childNodes;
      return {
        before: before?.textContent ?? null,
        under: cursor?.textContent ?? null,
        after: after?.textContent ?? null,
        end: cursor?.dataset?.ndAt === "end",
      };
    });

/** The block's box. The first of its boxes, since a cluster never wraps. */
const blockBox = (page, name) =>
  block(page, name).evaluate((span) => {
    const at = span.getClientRects()[0];
    return { left: at.left, right: at.right, top: at.top, bottom: at.bottom, width: at.width };
  });

/**
 * Where the engine drew its caret, in viewport pixels, or `null` for nowhere.
 *
 * See the head of this file. `caret: "initial"` stops the test runner hiding
 * the caret itself, which it does to every screenshot by default.
 */
async function caretBox(page, name) {
  const clip = await prompt(page, name).locator(".nd-prompt-stack").boundingBox();
  const hide = ".nd-prompt-cursor { visibility: hidden !important; }";
  const sheet = await page.addStyleTag({
    content: `${hide} .nd-prompt-input { caret-color: currentColor !important; caret-animation: manual !important; }`,
  });
  // The caret is drawn by the runner's own hiding being left off, and taken
  // away by it being left on: the same page, the same sheet, one difference.
  const without = await page.screenshot({ clip, caret: "hide" });
  const compare = (withCaret) => page.evaluate(
    async ([a, b]) => {
      const read = async (data) => {
        const blob = await (await fetch(`data:image/png;base64,${data}`)).blob();
        const bitmap = await createImageBitmap(blob);
        const canvas = new OffscreenCanvas(bitmap.width, bitmap.height);
        const ctx = canvas.getContext("2d");
        ctx.drawImage(bitmap, 0, 0);
        return { data: ctx.getImageData(0, 0, bitmap.width, bitmap.height).data, width: bitmap.width };
      };
      const one = await read(a);
      const two = await read(b);
      const box = { left: Infinity, top: Infinity, right: -Infinity, bottom: -Infinity };
      for (let i = 0; i < one.data.length; i += 4) {
        if (one.data[i] === two.data[i] && one.data[i + 1] === two.data[i + 1] && one.data[i + 2] === two.data[i + 2]) {
          continue;
        }
        const dot = i / 4;
        const x = dot % one.width;
        const y = (dot - x) / one.width;
        box.left = Math.min(box.left, x);
        box.right = Math.max(box.right, x + 1);
        box.top = Math.min(box.top, y);
        box.bottom = Math.max(box.bottom, y + 1);
      }
      return Number.isFinite(box.left) ? box : null;
    },
    [withCaret.toString("base64"), without.toString("base64")],
  );
  // A CARET THAT WAS HALFWAY THROUGH A BLINK WHEN IT WAS TOLD TO STOP may have
  // stopped away, so the picture is taken until one has it, for longer than a
  // whole turn of any engine's caret.
  let found = null;
  for (let turn = 0; !found && turn < 30; turn += 1) {
    if (turn > 0) await page.waitForTimeout(70);
    found = await compare(await page.screenshot({ clip, caret: "initial" }));
  }
  await sheet.evaluate((node) => node.remove());
  if (!found) return null;
  return {
    left: found.left + clip.x,
    right: found.right + clip.x,
    top: found.top + clip.y,
    bottom: found.bottom + clip.y,
  };
}

/**
 * Holds the block to the engine's caret.
 *
 * `edge` is the side of the block the caret stands at: the left of a cluster
 * written from the left, the right of one written from the right. A caret is
 * a pixel or two wide and the block's edge is a fraction, so the two agree to
 * within two pixels, and the caret's middle is inside the block's own line.
 */
async function agree(page, what, edge = "left", name) {
  const caret = await caretBox(page, name);
  expect(caret, `${what}: the engine drew no caret to compare with`).not.toBeNull();
  const at = await blockBox(page, name);
  const middle = (caret.left + caret.right) / 2;
  expect(Math.abs(middle - at[edge]), `${what}: the block is not where the caret is`).toBeLessThanOrEqual(2);
  const height = (caret.top + caret.bottom) / 2;
  expect(height, `${what}: the block is on another line`).toBeGreaterThan(at.top - 1);
  expect(height, `${what}: the block is on another line`).toBeLessThan(at.bottom + 1);
  return { caret, at };
}

// Written as escapes, so what each text holds can be read and no editor
// reorders it: a combining acute, an emoji of four people held by three
// zero-width joiners, and two scripts written from the right.
const ACUTE = "cafe\u0301 noir";
const FAMILY = "a\u{1F469}\u200D\u{1F469}\u200D\u{1F467}\u200D\u{1F466}b";
const HEBREW = "\u05E9\u05DC\u05D5\u05DD \u05E2\u05D5\u05DC\u05DD";
const ARABIC = "\u0633\u0644\u0627\u0645 \u0639\u0644\u064A\u0643\u0645";

test("the block is where the caret is, in plain text", async ({ page }) => {
  await open(page);
  for (const caret of [0, 1, 5, 6, 10]) {
    await put(page, "hello world", caret);
    expect((await drawn(page)).under, `caret ${caret}`).toBe("hello world"[caret]);
    await agree(page, `caret ${caret}`);
  }
  // At the end the caret stands before nothing, and the block is still there.
  await put(page, "hello world", 11);
  const end = await drawn(page);
  expect(end.end).toBe(true);
  expect(end.before).toBe("hello world");
  const { at } = await agree(page, "the end of the text");
  expect(at.width, "the block at the end of a line has no width").toBeGreaterThan(3);
});

test("the engine's own caret is not drawn beside the block", async ({ page }) => {
  await open(page);
  await put(page, "hello world", 5);
  // Two carets on one prompt is the defect a custom cursor is most likely to
  // ship with. For longer than a whole turn of the engine's blink, a picture
  // that would show its caret is the picture that hides it.
  const stack = prompt(page).locator(".nd-prompt-stack");
  await page.emulateMedia({ reducedMotion: "reduce" });
  const hidden = await stack.screenshot({ caret: "hide" });
  for (let turn = 0; turn < 16; turn += 1) {
    await page.waitForTimeout(70);
    expect(
      await differing(page, hidden, await stack.screenshot({ caret: "initial" })),
      "the engine's caret is drawn as well as the block",
    ).toBe(0);
  }
});

test("a block at the end of a full line takes no room, so the line does not turn", async ({ page }) => {
  await open(page);
  const box = input(page);
  await box.focus();
  // The narrowest letter there is, until the line turns, and then one taken
  // back: a line with less room left on it than the block is wide.
  //
  // THE LINE IS FILLED WITH THE CARET AT ITS START, where the block stands on
  // a letter and adds nothing. Filled with the caret at its end, a block that
  // took room would turn the line early and the search would stop one letter
  // short of the case it is looking for.
  const lines = () => box.evaluate((el) => el.scrollHeight);
  const one = await lines();
  let text = "i".repeat(40);
  await put(page, text, 0);
  while ((await lines()) === one) {
    text += "iiii";
    expect(text.length, "the line never turned").toBeLessThan(4000);
    await put(page, text, 0);
  }
  while ((await lines()) > one) {
    text = text.slice(0, -1);
    await put(page, text, 0);
  }
  await put(page, text, text.length);
  const heights = await prompt(page).evaluate((root) => ({
    text: root.querySelector(".nd-prompt-input").getBoundingClientRect().height,
    mirror: root.querySelector(".nd-prompt-mirror").scrollHeight,
    cell: root.querySelector(".nd-prompt-stack").getBoundingClientRect().height,
  }));
  expect(heights.cell, "the block turned a line the textarea did not").toBeCloseTo(heights.text, 1);
  expect(heights.mirror).toBeCloseTo(heights.text, 0);
  expect(heights.text).toBeCloseTo(one, 0);
  await agree(page, "at the end of a full line");
});

test("the block is a whole cluster: a letter with its mark, an emoji with its joiners", async ({ page }) => {
  await open(page);
  // Before the `e` that carries the acute: the block is both.
  await put(page, ACUTE, 3);
  expect((await drawn(page)).under).toBe("e\u0301");
  await agree(page, "before a letter with a combining mark");
  // BETWEEN THE LETTER AND ITS MARK, where only a script can put a caret. The
  // block is still the whole cluster, and it starts where the cluster does.
  await put(page, ACUTE, 4);
  const inside = await drawn(page);
  expect(inside.under).toBe("e\u0301");
  expect(inside.before).toBe("caf");
  await put(page, ACUTE, 5);
  expect((await drawn(page)).under).toBe(" ");
  await agree(page, "after a letter with a combining mark");

  await put(page, FAMILY, 1);
  expect((await drawn(page)).under, "the block cut a joined emoji").toBe(FAMILY.slice(1, -1));
  await agree(page, "before a joined emoji");
  await put(page, FAMILY, FAMILY.length - 1);
  expect((await drawn(page)).under).toBe("b");
  await agree(page, "after a joined emoji");
});

test("an engine with no segmenter still keeps a cluster whole", async ({ page }) => {
  // Before any script of the page's own: the module asks once, when it loads.
  await page.addInitScript(() => {
    delete Intl.Segmenter;
  });
  await open(page);
  expect(await page.evaluate(() => typeof Intl.Segmenter)).toBe("undefined");
  await put(page, ACUTE, 3);
  expect((await drawn(page)).under).toBe("e\u0301");
  await put(page, ACUTE, 4);
  expect(await drawn(page)).toMatchObject({ before: "caf", under: "e\u0301" });
  await put(page, FAMILY, 1);
  expect((await drawn(page)).under).toBe(FAMILY.slice(1, -1));
  // A caret a script put inside the joined emoji is still the whole of it.
  await put(page, FAMILY, 4);
  expect(await drawn(page)).toMatchObject({ before: "a", under: FAMILY.slice(1, -1), after: "b\u200B" });
  await put(page, FAMILY, FAMILY.length - 1);
  expect((await drawn(page)).under).toBe("b");
  // A letter outside the first plane is two units and one block.
  await put(page, "a\u{1D4B3}b", 1);
  expect((await drawn(page)).under).toBe("\u{1D4B3}");
  await agree(page, "before a letter of two units");
});

test("the block is where the caret is, in text written from the right", async ({ page }) => {
  await open(page);
  // A RUN FROM THE RIGHT INSIDE A LINE FROM THE LEFT. Inside the run the
  // caret stands at the right of the letter it is before.
  const mixed = `abc ${HEBREW} def`;
  for (const caret of [5, 6, 10]) {
    await put(page, mixed, caret);
    expect((await drawn(page)).under).toBe(mixed[caret]);
    await agree(page, `caret ${caret} of a mixed line`, "right");
  }
  // And outside it, on either side, the left again.
  for (const caret of [1, mixed.length - 2]) {
    await put(page, mixed, caret);
    await agree(page, `caret ${caret} of a mixed line`);
  }

  // A WHOLE LINE FROM THE RIGHT. Both boxes take their direction from the
  // text, so the line starts at the right in both.
  for (const [name, text] of [["Hebrew", HEBREW], ["Arabic", ARABIC]]) {
    for (const caret of [0, 2, 6]) {
      await put(page, text, caret);
      await agree(page, `caret ${caret} of a ${name} line`, "right");
    }
    const first = await blockBox(page);
    const stack = await prompt(page).locator(".nd-prompt-stack").boundingBox();
    expect(first.left, `a ${name} line is laid out from the left`).toBeGreaterThan(stack.x + stack.width / 2);
  }
});

test("the block is where the caret is on every line of a wrapped text", async ({ page }) => {
  await page.setViewportSize({ width: 320, height: 800 });
  await open(page);
  const text = "the quick brown fox jumps over the lazy dog and keeps going until the line has turned twice";
  const tops = new Set();
  for (const caret of [4, 30, 31, 44, 60, 75, text.length - 1, text.length]) {
    await put(page, text, caret);
    const { at } = await agree(page, `caret ${caret} of a wrapped text`);
    tops.add(Math.round(at.top));
  }
  expect(tops.size, "the text did not wrap, so no line but the first was asked about").toBeGreaterThanOrEqual(3);

  // A LINE BROKEN BY THE READER. Before the break the caret is at the end of
  // its line, and the block there takes no room: the text after it stays put.
  await put(page, "ab\ncd", 2);
  const atBreak = await drawn(page);
  expect(atBreak.end).toBe(true);
  expect(atBreak.after.startsWith("\ncd")).toBe(true);
  const upper = await agree(page, "before a line break");
  await put(page, "ab\ncd", 3);
  const lower = await agree(page, "after a line break");
  expect(lower.at.top, "the block did not go down with the caret").toBeGreaterThan(upper.at.top + 5);
  // AND A LAST LINE WITH NOTHING ON IT is still a line the block can stand on.
  await put(page, "ab\n", 3);
  const last = await agree(page, "on an empty last line");
  expect(last.at.top).toBeGreaterThan(upper.at.top + 5);
  // It is a line with the block elsewhere, too: the mirror sizes the prompt,
  // and a mirror one line short would leave the textarea to scroll.
  const tall = () => prompt(page).locator(".nd-prompt-stack").evaluate((cell) => cell.getBoundingClientRect().height);
  await put(page, "ab\nc", 0);
  const two = await tall();
  await put(page, "ab\n", 0);
  expect(await tall(), "an empty last line is not a line of the prompt").toBeCloseTo(two, 1);
  expect(await input(page).evaluate((el) => el.scrollHeight - el.clientHeight)).toBeLessThanOrEqual(1);
});

test("the two boxes are one box, at every width and in every locale", async ({ page }) => {
  for (const width of [320, 768, 1280]) {
    await page.setViewportSize({ width, height: 800 });
    for (const locale of LOCALES) {
      await open(page, locale);
      const boxes = await page.evaluate(() =>
        [...document.querySelectorAll(".nd-example-stage .nd-prompt")].map((root) => {
          const text = root.querySelector(".nd-prompt-input");
          const mirror = root.querySelector(".nd-prompt-mirror");
          const one = text.getBoundingClientRect();
          const two = mirror.getBoundingClientRect();
          const stage = root.closest(".nd-example-stage").getBoundingClientRect();
          return {
            name: root.dataset.ndPrompt,
            apart: Math.max(
              Math.abs(one.left - two.left),
              Math.abs(one.right - two.right),
              Math.abs(one.top - two.top),
              Math.abs(one.bottom - two.bottom),
            ),
            // The width the TEXT is given, which is what decides where a line
            // turns: the box less its padding and less the bar's own room.
            inner: text.clientWidth - mirror.clientWidth,
            over: root.getBoundingClientRect().right - stage.right,
          };
        }),
      );
      expect(boxes.length).toBeGreaterThan(4);
      for (const one of boxes) {
        const where = `${one.name} at ${width} in ${locale}`;
        expect(one.apart, `${where}: the mirror is not over the textarea`).toBeLessThan(0.5);
        expect(one.inner, `${where}: the two boxes give their text different widths`).toBe(0);
        expect(one.over, `${where}: the prompt runs past its stage`).toBeLessThanOrEqual(0.5);
      }
    }
  }
});

test("a selection is the engine's, and the block gives way to it", async ({ page }) => {
  await open(page);
  const box = input(page);
  await box.focus();
  await page.keyboard.type("hello world");
  await expect(block(page)).toBeVisible();

  await page.keyboard.press("Shift+ArrowLeft");
  await page.keyboard.press("Shift+ArrowLeft");
  await settle(page);
  expect(await box.evaluate((el) => [el.selectionStart, el.selectionEnd])).toEqual([9, 11]);
  await expect(prompt(page)).toHaveAttribute("data-nd-selecting", "");
  await expect(block(page)).toBeHidden();
  // The engine painted its highlight: the two letters differ from themselves
  // unselected, with the block out of both pictures.
  const stack = prompt(page).locator(".nd-prompt-stack");
  const selected = await stack.screenshot();

  await page.keyboard.press("ControlOrMeta+a");
  await settle(page);
  expect(await box.evaluate((el) => [el.selectionStart, el.selectionEnd])).toEqual([0, 11]);
  await expect(block(page)).toBeHidden();
  expect(await differing(page, selected, await stack.screenshot()), "the highlight did not grow").toBeGreaterThan(0);

  // Collapsed again, the block is back, on the character the caret is before.
  await page.keyboard.press("ArrowLeft");
  await settle(page);
  await expect(prompt(page)).not.toHaveAttribute("data-nd-selecting");
  await expect(block(page)).toBeVisible();
  expect((await drawn(page)).under).toBe("h");

  // A selection made by a pointer is the same selection.
  await box.dblclick({ position: { x: 20, y: 12 } });
  await settle(page);
  expect(await box.evaluate((el) => el.selectionEnd - el.selectionStart)).toBeGreaterThan(0);
  await expect(block(page)).toBeHidden();
});

test("an input method composes in the textarea, and Enter that confirms it sends nothing", async ({ page }) => {
  await open(page);
  await page.evaluate(() => {
    window.sent = [];
    document.addEventListener("nd-prompt-send", (event) => window.sent.push(event.detail));
  });
  const box = input(page);
  await box.focus();
  await page.keyboard.type("ab");
  const session = await page.context().newCDPSession(page);

  // The input method's own protocol: a composition under way, two characters
  // of it written, the caret after them.
  await session.send("Input.imeSetComposition", { text: "\u306B\u307B", selectionStart: 2, selectionEnd: 2 });
  await settle(page);
  await expect(prompt(page)).toHaveAttribute("data-nd-composing", "");
  await expect(box).toHaveValue("ab\u306B\u307B");
  await expect(block(page)).toBeHidden();
  // THE ENGINE'S CARET IS BACK, so the candidate window has something to
  // stand against. Taken away, a pixel changes; with the block it would not.
  const stack = prompt(page).locator(".nd-prompt-stack");
  const composing = await stack.screenshot({ caret: "initial" });
  const sheet = await page.addStyleTag({ content: ".nd-prompt-input { caret-color: transparent !important; }" });
  const noCaret = await stack.screenshot({ caret: "initial" });
  await sheet.evaluate((node) => node.remove());
  let seen = await differing(page, composing, noCaret);
  // The engine's caret blinks, so a picture can land on the half it is away.
  for (let turn = 0; seen === 0 && turn < 20; turn += 1) {
    await page.waitForTimeout(120);
    seen = await differing(page, await stack.screenshot({ caret: "initial" }), noCaret);
  }
  expect(seen, "no caret is drawn while an input method composes").toBeGreaterThan(0);

  // ENTER, AS AN INPUT METHOD SENDS IT: marked as composing, and as the key
  // code an engine reports for a key the input method took.
  await box.evaluate((el) => {
    el.dispatchEvent(new KeyboardEvent("keydown", { key: "Enter", bubbles: true, cancelable: true, isComposing: true }));
    el.dispatchEvent(new KeyboardEvent("keydown", { key: "Enter", bubbles: true, cancelable: true, keyCode: 229 }));
  });
  expect(await page.evaluate(() => window.sent), "Enter sent a composition that was not finished").toEqual([]);

  // The input method commits. The composition is the textarea's text, the
  // state is gone, and the block is back after it.
  await session.send("Input.insertText", { text: "\u65E5\u672C" });
  await settle(page);
  await expect(prompt(page)).not.toHaveAttribute("data-nd-composing");
  const value = await box.inputValue();
  expect(value.startsWith("ab")).toBe(true);
  expect(value.endsWith("\u65E5\u672C")).toBe(true);
  await expect(block(page)).toBeVisible();
  expect((await drawn(page)).before).toBe(value);

  await page.keyboard.press("Enter");
  expect(await page.evaluate(() => window.sent)).toEqual([{ name: "example-prompt", value }]);
});

test("the clipboard is the engine's: copy, cut and paste go through the textarea", async ({ browser }) => {
  const context = await browser.newContext({ permissions: ["clipboard-read", "clipboard-write"] });
  const page = await context.newPage();
  await open(page);
  const box = input(page);
  await box.focus();
  await page.keyboard.type("hello");
  await page.keyboard.press("ControlOrMeta+a");
  await page.keyboard.press("ControlOrMeta+c");
  expect(await page.evaluate(() => navigator.clipboard.readText())).toBe("hello");

  await page.keyboard.press("ArrowRight");
  await page.keyboard.type(" ");
  await page.keyboard.press("ControlOrMeta+v");
  await expect(box).toHaveValue("hello hello");
  await settle(page);
  // What was pasted is drawn like what was typed: the block is after it.
  expect(await drawn(page)).toMatchObject({ before: "hello hello", end: true });

  await page.keyboard.press("ControlOrMeta+a");
  await page.keyboard.press("ControlOrMeta+x");
  await expect(box).toHaveValue("");
  expect(await page.evaluate(() => navigator.clipboard.readText())).toBe("hello hello");
  await settle(page);
  expect(await drawn(page)).toMatchObject({ before: "", end: true });
  await context.close();
});

test("a screen reader is given one text box with one name and one text", async ({ page }) => {
  await open(page);
  await put(page, "close the page", 5);
  const tree = await prompt(page).ariaSnapshot();
  // THE WHOLE TREE, AND IT IS TWO THINGS. The text is read once: the mirror
  // holds the same words and is out of the tree, the block with it. The glyph
  // a prompt starts with is decoration and is not read either.
  expect(tree.trim().split("\n")).toEqual(['- textbox "Prompt": close the page', '- button "Send"']);
  await expect(page.getByRole("textbox", { name: "Prompt" }).first()).toBeVisible();
  // And the control that sends is a button with a name, though it shows none.
  await expect(prompt(page).getByRole("button", { name: "Send" })).toBeVisible();
});

test("the prompt grows a line at a time to its limit, and then scrolls", async ({ page }) => {
  await open(page);
  const box = input(page);
  await box.focus();
  const facts = () =>
    prompt(page).evaluate((root) => {
      const text = root.querySelector(".nd-prompt-input");
      const mirror = root.querySelector(".nd-prompt-mirror");
      return {
        height: text.getBoundingClientRect().height,
        mirror: mirror.getBoundingClientRect().height,
        hidden: text.scrollHeight - text.clientHeight,
        scrolled: text.scrollTop,
        followed: mirror.scrollTop,
        limit: Number.parseFloat(getComputedStyle(root).getPropertyValue("--noctua-shell-prompt-max")),
      };
    });
  const one = await facts();
  expect(one.limit, "the page declares no limit").toBeGreaterThan(1);
  const line = await page.evaluate(() => {
    const probe = document.createElement("div");
    probe.style.position = "absolute";
    probe.style.blockSize = "var(--noctua-shell-line)";
    document.body.append(probe);
    const size = probe.getBoundingClientRect().height;
    probe.remove();
    return size;
  });
  expect(one.height, "a prompt of one line is not one line tall").toBeCloseTo(line, 1);

  const heights = [one.height];
  for (let lines = 2; lines <= one.limit; lines += 1) {
    await page.keyboard.type("x");
    await page.keyboard.press("Shift+Enter");
    const now = await facts();
    expect(now.height, `at ${lines} lines the textarea is not the mirror's height`).toBeCloseTo(now.mirror, 1);
    expect(now.height, `the prompt did not grow at ${lines} lines`).toBeGreaterThan(heights.at(-1) + 5);
    expect(now.hidden, `the prompt scrolls at ${lines} lines, under its limit`).toBeLessThanOrEqual(1);
    heights.push(now.height);
  }
  // Every line added the same height: the text's own line.
  const steps = heights.slice(1).map((height, at) => height - heights[at]);
  expect(Math.max(...steps) - Math.min(...steps)).toBeLessThan(0.5);

  // PAST THE LIMIT IT STOPS GROWING AND SCROLLS, and the mirror goes with it:
  // the block is still on the caret, inside the box.
  for (let extra = 0; extra < 4; extra += 1) {
    await page.keyboard.type("y");
    await page.keyboard.press("Shift+Enter");
  }
  await settle(page);
  const full = await facts();
  expect(full.height, "the prompt grew past its limit").toBeCloseTo(heights.at(-1), 1);
  expect(full.hidden, "the prompt does not scroll past its limit").toBeGreaterThan(line / 2);
  expect(full.scrolled).toBeGreaterThan(0);
  expect(full.followed, "the mirror did not scroll with the textarea").toBe(full.scrolled);
  const { at } = await agree(page, "at the foot of a scrolled prompt");
  const stack = await prompt(page).locator(".nd-prompt-stack").boundingBox();
  expect(at.bottom, "the block is drawn under the prompt").toBeLessThanOrEqual(stack.y + stack.height + 0.5);

  // Scrolled back by the reader, the block leaves with its line.
  await box.evaluate((el) => {
    el.scrollTop = 0;
  });
  await settle(page);
  expect((await facts()).followed).toBe(0);
});

/** The block's own pixels, with the textarea's letter under it. */
const picture = async (page, name) => {
  const at = await blockBox(page, name);
  return page.screenshot({
    clip: { x: Math.floor(at.left) - 1, y: Math.floor(at.top) - 1, width: Math.ceil(at.width) + 2, height: Math.ceil(at.bottom - at.top) + 2 },
    caret: "initial",
  });
};

test("the block blinks, and for a reader who asked for less movement it stays", async ({ page }) => {
  await open(page);
  await put(page, "hello", 2);
  // IT BLINKS: over more than one turn, the block is seen in two states.
  const first = await picture(page);
  let changed = 0;
  for (let turn = 0; changed === 0 && turn < 30; turn += 1) {
    await page.waitForTimeout(90);
    changed = await differing(page, first, await picture(page));
  }
  expect(changed, "the block never blinks").toBeGreaterThan(0);

  await page.emulateMedia({ reducedMotion: "reduce" });
  await put(page, "hello", 3);
  const still = await picture(page);
  // Longer than a whole turn, sampled finer than half of one.
  for (let turn = 0; turn < 16; turn += 1) {
    await page.waitForTimeout(90);
    expect(await differing(page, still, await picture(page)), "the block blinks under reduced motion").toBe(0);
  }
  // And what stays is the block, not its absence.
  const sheet = await page.addStyleTag({ content: ".nd-prompt-cursor { visibility: hidden !important; }" });
  const none = await picture(page);
  await sheet.evaluate((node) => node.remove());
  expect(await differing(page, still, none), "under reduced motion there is no block at all").toBeGreaterThan(0);
});

test("the block is filled with the focus, an outline without it, and gone when disabled", async ({ page }) => {
  await page.emulateMedia({ reducedMotion: "reduce" });
  await open(page);
  const gone = async (name) => {
    const sheet = await page.addStyleTag({ content: ".nd-prompt-cursor { visibility: hidden !important; }" });
    const shot = await picture(page, name);
    await sheet.evaluate((node) => node.remove());
    return shot;
  };

  await put(page, "hello", 2);
  const filled = await picture(page);
  await input(page).blur();
  const outline = await picture(page);
  const nothing = await gone();
  expect(await differing(page, filled, outline), "the focus does not change the block").toBeGreaterThan(0);
  expect(await differing(page, outline, nothing), "a prompt without the focus shows no caret").toBeGreaterThan(0);
  // AN OUTLINE IS LESS THAN A FILL: the letter under it is the textarea's own
  // and shows through, so fewer pixels differ from the bare letter.
  expect(await differing(page, outline, nothing)).toBeLessThan(await differing(page, filled, nothing));

  // READ-ONLY KEEPS THE OUTLINE WITH THE FOCUS IN IT.
  const name = "example-prompt-readonly";
  await prompt(page, name).scrollIntoViewIfNeeded();
  const resting = await picture(page, name);
  await input(page, name).focus();
  await settle(page);
  // The ring is the textarea's own and is not in the block's box, so the
  // block's pixels are the same pixels.
  expect(await differing(page, resting, await picture(page, name)), "a read-only prompt fills its block").toBe(0);
  expect(await differing(page, resting, await gone(name)), "a read-only prompt shows no caret").toBeGreaterThan(0);

  // DISABLED DRAWS NONE.
  await expect(block(page, "example-prompt-disabled")).toBeHidden();
});

test("Enter and the send control say so, and nothing else does", async ({ page }) => {
  await open(page);
  await page.evaluate(() => {
    window.sent = [];
    document.addEventListener("nd-prompt-send", (event) => window.sent.push(event.detail));
  });
  const box = input(page);
  await box.focus();
  await page.keyboard.type("one");
  await page.keyboard.press("Shift+Enter");
  await page.keyboard.type("two");
  await expect(box).toHaveValue("one\ntwo");
  expect(await page.evaluate(() => window.sent), "Shift with Enter sent").toEqual([]);

  await page.keyboard.press("Enter");
  await prompt(page).locator(".nd-prompt-send").click();
  expect(await page.evaluate(() => window.sent)).toEqual([
    { name: "example-prompt", value: "one\ntwo" },
    { name: "example-prompt", value: "one\ntwo" },
  ]);
  // IT CLEARS NOTHING, and a caller that does is drawn at once: the property
  // fires no event, and the block is at the start of an empty prompt.
  await expect(box).toHaveValue("one\ntwo");
  await box.evaluate((el) => {
    el.value = "";
  });
  expect(await drawn(page)).toMatchObject({ before: "", end: true, after: "\u200B" });

  // A prompt nobody can type in sends nothing, by the key or by the control.
  await input(page, "example-prompt-readonly").focus();
  await page.keyboard.press("Enter");
  await prompt(page, "example-prompt-readonly").locator(".nd-prompt-send").click({ force: true });
  expect(await page.evaluate(() => window.sent.length)).toBe(2);
});

test("a prompt that names a list walks it and keeps the focus, and Enter is still the prompt's", async ({
  page,
}) => {
  await open(page);
  await page.evaluate(() => {
    window.heard = [];
    for (const type of ["nd-prompt-send", "nd-option-active", "nd-option-choose"]) {
      document.addEventListener(type, (event) => window.heard.push([type, event.detail.value]));
    }
  });
  const name = "example-prompt-driven";
  const box = input(page, name);
  await box.focus();
  await page.keyboard.type("se");
  await page.keyboard.press("ArrowDown");
  await page.keyboard.press("ArrowDown");
  const active = page.locator('[data-nd-option-list="prompt-commands"] [data-nd-active="true"]');
  await expect(active).toHaveAttribute("data-nd-value", "help");
  await expect(box).toBeFocused();
  await expect(box).toHaveAttribute("aria-activedescendant", await active.getAttribute("id"));
  // The arrows moved the list and not the caret, and the text is as typed.
  await expect(box).toHaveValue("se");
  expect((await drawn(page, name)).before).toBe("se");

  await page.keyboard.press("Enter");
  // ONE EVENT FOR ONE KEY. The prompt says it was sent, and the caller knows
  // which option is active. The list does not also say one was chosen.
  expect(await page.evaluate(() => window.heard)).toEqual([
    ["nd-option-active", "settings"],
    ["nd-option-active", "help"],
    ["nd-prompt-send", "se"],
  ]);

  // A KEY WITH A MODIFIER IS THE TEXT'S, list or no list. Shift with Enter
  // breaks the line, Shift with an arrow selects, and Control with Enter is
  // somebody's shortcut: none of them chooses an option or moves the mark.
  await page.evaluate(() => {
    window.heard = [];
  });
  await page.keyboard.press("Shift+Enter");
  await expect(box, "Shift+Enter did not break the line").toHaveValue("se\n");
  await page.keyboard.press("Shift+ArrowUp");
  expect(await box.evaluate((el) => el.selectionEnd - el.selectionStart), "Shift+Up selected nothing").toBeGreaterThan(0);
  await page.keyboard.press("Control+Enter");
  expect(await page.evaluate(() => window.heard), "a key with a modifier walked or chose").toEqual([]);
  await expect(active).toHaveAttribute("data-nd-value", "help");
});

test("a released prompt is a textarea again, and can be enhanced again", async ({ page }) => {
  await open(page);
  await put(page, "hello", 2);
  await page.evaluate(async () => {
    const { releasePrompts } = await import("../../assets/prompt.js");
    releasePrompts(document);
  });
  const state = await prompt(page).evaluate((root) => {
    const text = root.querySelector(".nd-prompt-input");
    return {
      enhanced: root.hasAttribute("data-nd-enhanced"),
      mirror: root.querySelector(".nd-prompt-mirror").childNodes.length,
      // The property is the prototype's again, with nothing left on the element.
      own: Object.hasOwn(text, "value"),
      value: text.value,
    };
  });
  expect(state).toEqual({ enhanced: false, mirror: 0, own: false, value: "hello" });
  // THE ENGINE'S CARET IS BACK with the attribute that hid it.
  expect(await caretBox(page), "a released prompt has no caret of any kind").not.toBeNull();

  await page.evaluate(async () => {
    const { enhancePrompts } = await import("../../assets/prompt.js");
    enhancePrompts(document);
  });
  await page.keyboard.type("!");
  await settle(page);
  expect((await drawn(page)).before).toBe("he!");
});

test("with the script blocked it is a textarea with the engine's caret", async ({ browser }) => {
  const context = await browser.newContext({ javaScriptEnabled: false });
  const page = await context.newPage();
  await page.goto(url(PAGE.prompt));
  // The page's own first example: the folds under it stay shut with no script.
  const root = page.locator('[data-nd-prompt="overview-prompt"]');
  await expect(root).not.toHaveAttribute("data-nd-enhanced");
  const box = root.locator(".nd-prompt-input");
  await box.focus();
  await page.keyboard.type("hello");
  await expect(box).toHaveValue("hello");
  // THE CARET IS DRAWN. The runner hides every caret in a picture unless it
  // is told not to, so one picture of each kind differ in the caret alone. No
  // script runs in this page, so they are compared here, as bytes, and taken
  // again while the caret is on the half of its blink it is away for.
  const stack = root.locator(".nd-prompt-stack");
  const without = await stack.screenshot({ caret: "hide" });
  let seen = false;
  for (let turn = 0; !seen && turn < 30; turn += 1) {
    if (turn > 0) await page.waitForTimeout(70);
    seen = !(await stack.screenshot({ caret: "initial" })).equals(without);
  }
  expect(seen, "with the script blocked the prompt has no caret").toBe(true);
  await context.close();
});

test("past its limit the prompt shows a scrollbar, and both boxes keep its room", async ({ page }) => {
  await open(page);
  const name = "example-prompt-long";
  const widths = (which) =>
    prompt(page, which).evaluate((root) => {
      const text = root.querySelector(".nd-prompt-input");
      const mirror = root.querySelector(".nd-prompt-mirror");
      return {
        bar: text.offsetWidth - text.clientWidth,
        text: text.clientWidth,
        mirror: mirror.clientWidth,
        hidden: text.scrollHeight - text.clientHeight,
      };
    });
  const long = await widths(name);
  expect(long.hidden, "the example does not scroll, so no bar was asked about").toBeGreaterThan(10);
  expect(long.bar, "a prompt past its limit keeps no room for a scrollbar").toBeGreaterThan(0);
  // AND THE BAR IS DRAWN IN THAT ROOM: something in the strip stands out from
  // its ground, where a prompt of one line has the strip and nothing in it.
  const strip = async (which) => {
    const at = await input(page, which).boundingBox();
    const shot = await page.screenshot({
      clip: { x: at.x + at.width - long.bar, y: at.y, width: long.bar, height: at.height },
    });
    const { min, max } = await extremes(page, shot);
    return contrast(min, max);
  };
  await prompt(page, name).scrollIntoViewIfNeeded();
  expect(await strip(name), "a prompt past its limit shows no scrollbar").toBeGreaterThan(1.1);
  await prompt(page).scrollIntoViewIfNeeded();
  expect(await strip("example-prompt"), "a prompt of one line shows a scrollbar").toBeLessThan(1.02);
  expect(long.mirror, "the bar narrowed the textarea and not the mirror").toBe(long.text);
  // AND A PROMPT OF ONE LINE KEEPS THE SAME ROOM, so the ninth line does not
  // move where the first eight turn.
  const short = await widths("example-prompt");
  expect(short.hidden).toBeLessThanOrEqual(1);
  expect(short.mirror).toBe(short.text);
  expect(short.bar, "the bar's room arrives with the bar").toBe(long.bar);

  // The block is on the caret with the bar there, on a line that turned.
  const text = `${"window.close and window.hide and page.close ".repeat(30)}end`;
  await put(page, text, text.length - 3, name);
  // A caret a script placed is not scrolled to. One a key moved is.
  await page.keyboard.press("ArrowLeft");
  await page.keyboard.press("ArrowRight");
  await settle(page);
  expect((await drawn(page, name)).under).toBe("e");
  const { at } = await agree(page, "in a prompt with a scrollbar", "left", name);
  const cell = await prompt(page, name).locator(".nd-prompt-stack").boundingBox();
  expect(at.top, "the block is drawn outside the prompt").toBeGreaterThanOrEqual(cell.y - 0.5);
  expect(at.bottom, "the block is drawn outside the prompt").toBeLessThanOrEqual(cell.y + cell.height + 0.5);
});
