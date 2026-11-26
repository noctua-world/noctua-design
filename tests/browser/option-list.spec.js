// The list a reader walks with the keyboard.
//
// WHAT THE CASES CANNOT SAY. `spec/components/option_list.cases.toml` drives
// one list from its own focus. Left for here: the control that names a list
// and keeps the focus, the two events, a caller moving the active option,
// columns, and what the active row looks like with marked letters in it.

import { expect, test } from "@playwright/test";

import { PAGE, showParts, url } from "./pages.js";
import { contrast, differing, extremes } from "./pixels.js";

async function open(page, locale = "en") {
  await page.goto(url(PAGE.optionList, locale));
  await showParts(page);
  await expect(page.locator('[data-nd-option-list="walk"][data-nd-enhanced]')).toBeAttached();
}

const list = (page, name) => page.locator(`[data-nd-option-list="${name}"]`);
const box = (page, name) => list(page, name).locator(".nd-option-list-items");
const active = (page, name) => list(page, name).locator('.nd-option-item[data-nd-active="true"]');

/** Collects every event a list says, in order. */
async function listen(page) {
  await page.evaluate(() => {
    window.heard = [];
    for (const type of ["nd-option-active", "nd-option-choose"]) {
      document.addEventListener(type, (event) => {
        window.heard.push([type, event.detail.name, event.detail.value]);
      });
    }
  });
}

test("a control that names the list drives it, and keeps the focus", async ({ page }) => {
  await open(page);
  await listen(page);
  const input = page.locator("#example-option-list-box");
  await input.focus();
  // Nothing is active until the reader asks: the box is for typing in.
  await expect(active(page, "driven")).toHaveCount(0);

  await input.press("ArrowDown");
  await expect(active(page, "driven")).toHaveAttribute("data-nd-value", "settings");
  await input.press("ArrowDown");
  await expect(active(page, "driven")).toHaveAttribute("data-nd-value", "help");
  await expect(input).toBeFocused();

  // BOTH HOLDERS NAME THE SAME OPTION, and the name is a real id.
  const id = await active(page, "driven").getAttribute("id");
  expect(id).toBeTruthy();
  await expect(input).toHaveAttribute("aria-activedescendant", id);
  await expect(box(page, "driven")).toHaveAttribute("aria-activedescendant", id);

  await input.press("Enter");
  expect(await page.evaluate(() => window.heard)).toEqual([
    ["nd-option-active", "driven", "settings"],
    ["nd-option-active", "driven", "help"],
    ["nd-option-choose", "driven", "help"],
  ]);
});

test("in a control, Home, End, Space and letters stay the control's", async ({ page }) => {
  await open(page);
  const input = page.locator("#example-option-list-box");
  await input.focus();
  await input.pressSequentially("s h");
  await input.press("ArrowDown");
  await input.press("Home");
  // The caret moved and the list did not: the first option is still active,
  // and what was typed is all there, space included.
  await expect(active(page, "driven")).toHaveAttribute("data-nd-value", "settings");
  await expect(input).toHaveValue("s h");
  expect(await input.evaluate((el) => el.selectionStart)).toBe(0);

  // With Control the pair is the list's.
  await input.press("Control+End");
  await expect(active(page, "driven")).toHaveAttribute("data-nd-value", "sidebar");
  expect(await input.evaluate((el) => el.selectionStart)).toBe(0);
});

test("a press on a driven list chooses and leaves the focus in the control", async ({ page }) => {
  await open(page);
  await listen(page);
  const input = page.locator("#example-option-list-box");
  await input.focus();
  await list(page, "driven").locator('[data-nd-value="help"]').click();
  await expect(input).toBeFocused();
  expect(await page.evaluate(() => window.heard)).toEqual([
    ["nd-option-active", "driven", "help"],
    ["nd-option-choose", "driven", "help"],
  ]);
});

test("Enter and Space choose from the list itself, and a disabled option is not chosen", async ({
  page,
}) => {
  await open(page);
  await listen(page);
  await box(page, "states").focus();
  // The caller's own active option is where the walk starts from.
  await expect(active(page, "states")).toHaveAttribute("data-nd-value", "settings");
  await page.keyboard.press("Enter");
  await page.keyboard.press("Space");
  await page.keyboard.press("End");
  await expect(active(page, "states")).toHaveAttribute("aria-disabled", "true");
  await page.keyboard.press("Enter");
  expect(await page.evaluate(() => window.heard)).toEqual([
    ["nd-option-choose", "states", "settings"],
    ["nd-option-choose", "states", "settings"],
    ["nd-option-active", "states", "dev"],
  ]);
  // And Space did not scroll the page out from under the list.
  expect(await page.evaluate(() => document.activeElement.className)).toContain("nd-option-list-items");
});

test("an option a caller marks active takes the mark from the one the script moved to", async ({
  page,
}) => {
  await open(page);
  await box(page, "walk").focus();
  await page.keyboard.press("ArrowDown");
  await expect(active(page, "walk")).toHaveAttribute("data-nd-value", "help");

  // What a framework does when the state it renders from changes: one
  // attribute written on another element, and nothing taken off the first.
  await list(page, "walk")
    .locator('[data-nd-value="sidebar"]')
    .evaluate((option) => option.setAttribute("data-nd-active", "true"));
  await expect(active(page, "walk")).toHaveCount(1);
  await expect(active(page, "walk")).toHaveAttribute("data-nd-value", "sidebar");
  const id = await active(page, "walk").getAttribute("id");
  await expect(box(page, "walk")).toHaveAttribute("aria-activedescendant", id);

  // And an active option that leaves the list takes the pointer with it.
  await active(page, "walk").evaluate((option) => option.remove());
  await expect(box(page, "walk")).not.toHaveAttribute("aria-activedescendant");
});

test("an option rendered later is numbered, and never with an id already given", async ({ page }) => {
  await open(page);
  const ids = await page.evaluate(() => {
    const items = document.querySelector('[data-nd-option-list="walk"] .nd-option-list-items');
    items.firstElementChild.remove();
    const added = items.lastElementChild.cloneNode(true);
    added.removeAttribute("id");
    added.dataset.ndValue = "added";
    items.append(added);
    return new Promise((done) =>
      requestAnimationFrame(() => done([...items.children].map((one) => one.id))),
    );
  });
  expect(ids.every(Boolean), `an option has no id: ${ids}`).toBe(true);
  expect(new Set(ids).size, `two options share an id: ${ids}`).toBe(ids.length);
});

test("in columns the arrows across move to the option beside", async ({ page }) => {
  await page.setViewportSize({ width: 1280, height: 900 });
  await open(page);
  const columns = await list(page, "leader").evaluate((root) => {
    const lefts = [...root.querySelectorAll(".nd-option-item")].map(
      (one) => Math.round(one.getBoundingClientRect().left),
    );
    return new Set(lefts).size;
  });
  expect(columns, "the example draws one column at this width").toBeGreaterThan(1);

  await box(page, "leader").focus();
  const first = await active(page, "leader").boundingBox();
  await page.keyboard.press("ArrowRight");
  const beside = await active(page, "leader").boundingBox();
  expect(beside.x, "the Right arrow did not change column").toBeGreaterThan(first.x + 1);
  expect(Math.abs(beside.y - first.y), "the Right arrow changed row").toBeLessThan(first.height / 2);
  await page.keyboard.press("ArrowLeft");
  const back = await active(page, "leader").boundingBox();
  expect(Math.round(back.x)).toBe(Math.round(first.x));
  expect(Math.round(back.y)).toBe(Math.round(first.y));

  // Down fills a column before it starts the next, so it never moves sideways
  // until the column ends.
  await page.keyboard.press("ArrowDown");
  const under = await active(page, "leader").boundingBox();
  expect(Math.round(under.x)).toBe(Math.round(first.x));
  expect(under.y).toBeGreaterThan(first.y);
});

test("a narrow window draws the columns as one, and nothing overflows", async ({ page }) => {
  await page.setViewportSize({ width: 320, height: 700 });
  for (const locale of ["en", "pt-BR", "qps"]) {
    await page.goto(url(PAGE.optionList, locale));
    await showParts(page);
    const facts = await list(page, "leader").evaluate((root) => {
      const stage = root.closest(".nd-example-stage") ?? root.parentElement;
      const edge = stage.getBoundingClientRect().right;
      const items = [...root.querySelectorAll(".nd-option-item")].map((one) => one.getBoundingClientRect());
      return {
        columns: new Set(items.map((one) => Math.round(one.left))).size,
        over: items.filter((one) => one.right > edge + 0.5).length,
        // The floor is the token's, read off the page rather than typed here.
        short: items.filter(
          (one) =>
            one.height <
            Number.parseFloat(getComputedStyle(root).getPropertyValue("--noctua-target-min")),
        ).length,
      };
    });
    expect(facts.over, `${locale}: an option runs past its stage`).toBe(0);
    expect(facts.short, `${locale}: an option is a target under the floor`).toBe(0);
    expect(facts.columns, `${locale}: more than two columns in the narrowest window`).toBeLessThanOrEqual(2);
  }
});

test("every size is a target no smaller than the floor", async ({ page }) => {
  await open(page);
  const floor = await page.evaluate(() =>
    Number.parseFloat(
      getComputedStyle(document.documentElement).getPropertyValue("--noctua-target-min"),
    ),
  );
  expect(floor, "the page declares no smallest target").toBeGreaterThan(0);
  const heights = await page.evaluate(() =>
    [...document.querySelectorAll(".nd-option-item")].map((one) => one.getBoundingClientRect().height),
  );
  expect(heights.length).toBeGreaterThan(10);
  expect(Math.min(...heights)).toBeGreaterThanOrEqual(floor);
});

test("the active row is told apart without colour, and moving it moves no name", async ({ page }) => {
  await open(page);
  const rows = list(page, "walk").locator(".nd-option-item");
  const before = await rows.evaluateAll((all) =>
    all.map((one) => one.querySelector(".nd-option-item-label").getBoundingClientRect().left),
  );
  await box(page, "walk").focus();
  await page.keyboard.press("ArrowDown");
  const after = await rows.evaluateAll((all) =>
    all.map((one) => one.querySelector(".nd-option-item-label").getBoundingClientRect().left),
  );
  // The bar's room is always there, so no name moves when the bar arrives.
  expect(after).toEqual(before);

  // THE BAR IS A SHAPE. With the fill taken away the active row still differs
  // from the row beside it, on its leading edge, where the bar is.
  await page.addStyleTag({ content: `.nd-option-item { background: transparent !important; }` });
  const strip = async (row) => {
    const at = await row.boundingBox();
    return page.screenshot({ clip: { x: at.x, y: at.y + 2, width: 6, height: at.height - 4 } });
  };
  const on = await strip(active(page, "walk"));
  const off = await strip(rows.first());
  expect(await differing(page, on, off), "the active row draws no bar").toBeGreaterThan(0);
});

for (const scheme of ["light", "dark"]) {
  test(`matched letters stay readable on the active row, in ${scheme}`, async ({ browser }) => {
    const context = await browser.newContext({ colorScheme: scheme });
    const page = await context.newPage();
    await open(page);
    // The hero's first row is active and carries the first two tones
    // (`NOCTUA.md §12.4`: the selected item keeps its match colours).
    const row = active(page, "overview-commands");
    await expect(row).toBeVisible();
    for (const word of ["1", "2"]) {
      const mark = row.locator(`mark[data-nd-word="${word}"]`).first();
      const { min, max } = await extremes(page, await mark.screenshot());
      expect(contrast(min, max), `word ${word} on the active row, ${scheme}`).toBeGreaterThanOrEqual(4.5);
    }
    // And the plain letters of the same name, on the row's own fill.
    const label = row.locator(".nd-option-item-label");
    const { min, max } = await extremes(page, await label.screenshot());
    expect(contrast(min, max), `the name on the active row, ${scheme}`).toBeGreaterThanOrEqual(4.5);
    await context.close();
  });
}

test("the selected option draws a tick, and only the selected one", async ({ page }) => {
  await open(page);
  const states = list(page, "states");
  await expect(states.locator('[aria-selected="true"] .nd-option-item-mark svg')).toBeVisible();
  await expect(states.locator(".nd-option-item-mark")).toHaveCount(1);
});

test("an empty list shows its sentence in every locale, and a group with nothing takes no room", async ({
  page,
}) => {
  for (const locale of ["en", "pt-BR"]) {
    await open(page, locale);
    const empty = list(page, "nothing");
    await expect(empty.locator(".nd-option-list-empty")).toBeVisible();
    await expect(empty.locator(".nd-option-list-items")).toBeHidden();
    // Every other list keeps the sentence to itself.
    await expect(list(page, "walk").locator(".nd-option-list-empty")).toHaveCount(0);
  }
  const group = list(page, "grouped").locator(".nd-option-group").first();
  await expect(group).toBeVisible();
  await group.evaluate((one) => {
    for (const option of one.querySelectorAll(".nd-option-item")) option.hidden = true;
  });
  await expect(group).toBeHidden();
  // Hiding every option of the list is the empty list again.
  await list(page, "walk").evaluate((root) => {
    for (const option of root.querySelectorAll(".nd-option-item")) option.hidden = true;
  });
  await expect(box(page, "walk")).toBeHidden();
});

test("a key an input method is using walks nothing and chooses nothing", async ({ page }) => {
  await open(page);
  await listen(page);
  // THE KEYS AS AN INPUT METHOD SENDS THEM: marked as composing, or with the
  // code an engine reports for a key the input method took. WebKit, the
  // engine of the interface's window, sends the Enter that commits a
  // composition after `compositionend`, not composing, with that code.
  const composed = (el) => {
    let prevented = 0;
    for (const init of [
      { key: "Enter", keyCode: 229 },
      { key: "Enter", isComposing: true },
      { key: "ArrowDown", keyCode: 229 },
    ]) {
      const event = new KeyboardEvent("keydown", { ...init, bubbles: true, cancelable: true });
      el.dispatchEvent(event);
      if (event.defaultPrevented) prevented += 1;
    }
    return prevented;
  };

  // FROM A CONTROL THAT NAMES THE LIST, with an option active.
  const input = page.locator("#example-option-list-box");
  await input.focus();
  await input.press("ArrowDown");
  expect(await input.evaluate(composed), "a key the input method had was taken from it").toBe(0);
  await expect(active(page, "driven")).toHaveAttribute("data-nd-value", "settings");

  // AND FROM THE LIST ITSELF.
  await box(page, "walk").focus();
  const from = await active(page, "walk").getAttribute("data-nd-value");
  expect(await box(page, "walk").evaluate(composed), "a key the input method had was taken from it").toBe(0);
  await expect(active(page, "walk")).toHaveAttribute("data-nd-value", from);
  expect(
    (await page.evaluate(() => window.heard)).filter(([type]) => type === "nd-option-choose"),
    "an input method's Enter chose an option",
  ).toEqual([]);
});

/**
 * Where the active option of the list `name` is: how far inside its list's own
 * box and inside the part of the window the site's sticky header leaves, at
 * each end, and how far the page and the box have scrolled. A negative room is
 * that much of the option out of sight.
 */
const seen = (page, name) =>
  box(page, name).evaluate((el) => {
    const option = el.querySelector('[data-nd-active="true"]').getBoundingClientRect();
    const frame = el.getBoundingClientRect();
    const header = document.querySelector(".nd-header")?.getBoundingClientRect().bottom ?? 0;
    return {
      page: window.scrollY,
      scrolled: el.scrollTop,
      box: [option.top - frame.top, frame.bottom - option.bottom],
      window: [option.top - Math.max(header, 0), window.innerHeight - option.bottom],
    };
  });

/** Holds an option inside a box: `room` is `[above, below]`, from `seen`. */
function inside(room, what) {
  expect(room[0], `${what}: the option is cut off above`).toBeGreaterThanOrEqual(-0.5);
  expect(room[1], `${what}: the option is cut off below`).toBeGreaterThanOrEqual(-0.5);
}

test("walking a list moves the box that scrolls it, and the page only as far as the option needs", async ({
  page,
}) => {
  await open(page);
  const items = box(page, "walk");
  // A LIST TALLER THAN ITS BOX, as a command list in a drawer is, and a page
  // scrolled so that only the top of that box is in the window.
  await items.evaluate((el) => {
    el.style.maxBlockSize = "var(--noctua-space-16)";
    el.style.overflowY = "auto";
  });
  await items.focus();
  await page.evaluate(() => {
    const box = document.querySelector('[data-nd-option-list="walk"] .nd-option-list-items');
    window.scrollTo(0, box.getBoundingClientRect().top + window.scrollY - window.innerHeight + 12);
  });
  const before = await page.evaluate(() => window.scrollY);
  await page.keyboard.press("End");
  await expect(active(page, "walk")).toHaveCount(1);
  const end = await seen(page, "walk");
  // The box was scrolled to the option, and the option is in it.
  expect(end.scrolled, "the list's own box did not scroll to the option").toBeGreaterThan(0);
  inside(end.box, "End, in the list's box");
  // AND IN THE WINDOW, which the page moved to by the least way: the option's
  // foot is at the window's.
  inside(end.window, "End, in the window");
  expect(end.page, "the option left the window and the page stayed").toBeGreaterThan(before);
  expect(end.window[1], "the page moved further than the option needed").toBeLessThanOrEqual(1);

  // A MOVE THE WINDOW ALREADY SHOWS moves the box alone.
  await page.keyboard.press("ArrowUp");
  const up = await seen(page, "walk");
  inside(up.box, "the up arrow, in the list's box");
  inside(up.window, "the up arrow, in the window");
  expect(up.page, "a move inside the window scrolled the page").toBe(end.page);
});

test("where only the page scrolls a list, a walk keeps the active option in the window", async ({ page }) => {
  // The interface's smallest window, and a list as long as a caller's.
  await page.setViewportSize({ width: 320, height: 240 });
  await open(page);
  const items = box(page, "walk");
  await items.evaluate((el) => {
    for (let at = 0; at < 9; at += 1) {
      const option = el.lastElementChild.cloneNode(true);
      option.removeAttribute("id");
      option.removeAttribute("data-nd-active");
      option.dataset.ndValue = `more-${at}`;
      el.append(option);
    }
  });
  const count = await list(page, "walk").locator(".nd-option-item").count();
  // The list starts half way down the window, so the walk runs off its foot.
  await items.evaluate((el) => {
    window.scrollTo(0, el.getBoundingClientRect().top + window.scrollY - window.innerHeight / 2);
    el.focus({ preventScroll: true });
  });
  await expect(active(page, "walk")).toHaveCount(1);
  const start = await seen(page, "walk");
  expect(start.scrolled, "the list has a box of its own that scrolls").toBe(0);

  let moved = false;
  for (let step = 1; step < count; step += 1) {
    const from = (await seen(page, "walk")).page;
    await page.keyboard.press("ArrowDown");
    const at = await seen(page, "walk");
    inside(at.window, `down to option ${step + 1} of ${count}`);
    if (at.page !== from) {
      moved = true;
      expect(at.window[1], `down to option ${step + 1}: the page moved further than it needed`).toBeLessThanOrEqual(1);
    }
  }
  expect(moved, "the walk never reached the foot of the window").toBe(true);

  // AND BACK UP, past the header that sticks to the top of the window.
  await page.keyboard.press("Home");
  const home = await seen(page, "walk");
  inside(home.window, "Home, below the header");
  expect(home.window[0], "Home: the page moved further than it needed").toBeLessThanOrEqual(1);
});

test("a released list answers no key, and can be enhanced again", async ({ page }) => {
  await open(page);
  await page.evaluate(async () => {
    const { releaseOptionLists } = await import("../../assets/option-list.js");
    releaseOptionLists(document);
  });
  await box(page, "walk").focus();
  await page.keyboard.press("ArrowDown");
  await expect(active(page, "walk")).toHaveCount(0);
  await page.evaluate(async () => {
    const { enhanceOptionLists } = await import("../../assets/option-list.js");
    enhanceOptionLists(document);
  });
  await page.keyboard.press("ArrowDown");
  await expect(active(page, "walk")).toHaveCount(1);
});

test("with the script blocked it is a named list, and nothing is hidden", async ({ browser }) => {
  const context = await browser.newContext({ javaScriptEnabled: false });
  const page = await context.newPage();
  await page.goto(url(PAGE.optionList));
  // The page's own first example: the folds under it stay shut with no script.
  const first = list(page, "overview-commands");
  await expect(first.locator('ul[role="listbox"][aria-label]')).toBeVisible();
  await expect(first.locator('[role="group"]')).toHaveCount(2);
  for (const option of await first.locator('[role="option"]').all()) await expect(option).toBeVisible();
  // It has options, so the sentence it would say with none stays put away.
  await expect(first.locator(".nd-option-list-empty")).toBeHidden();
  await context.close();
});
