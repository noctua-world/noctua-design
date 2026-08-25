// The table, and the claims only a browser can settle.
//
// Its behaviour SHIPS — `dist/table.js` — so what is asserted here is asserted
// about a file a consumer of the packages receives.
//
// THE COMPONENT THIS FILE IS ABOUT WAS DELETED AND REBUILT. The version before
// it stacked by putting `display: block` on the `<table>`, which left the
// `<caption>` a table box with no table to be the caption of; at 390px "The
// spacing scale" wrapped into three lines inside sixty pixels. The last test in
// the first section is that defect, asserted as a measurement.

import { expect, test } from "@playwright/test";

import { LOCALES, PAGE, url, showParts } from "./pages.js";

const named = (page, name) => page.locator(`[data-nd-table="${name}"]`);
const frame = (page, name) => page.locator(`.nd-table-frame:has([data-nd-table="${name}"])`);
const status = (page, name) => frame(page, name).locator("[data-nd-table-status]");

/** The rows a reader can see, by the words in their row heading. */
async function rows(table) {
  return table.locator("tbody .nd-table-row:not([hidden]) > th").allInnerTexts();
}

// --- What ships, and what the script adds ------------------------------------

test.describe("with the script blocked", () => {
  test("it is a real table with real roles and a real caption", async ({ browser }) => {
    const context = await browser.newContext({ javaScriptEnabled: false });
    const page = await context.newPage();
    await page.goto(url(PAGE.tables));
    await showParts(page);

    const table = named(page, "invoices");
    await expect(table).toBeVisible();
    expect(await table.evaluate((el) => el.tagName)).toBe("TABLE");
    // RESTATED IN THE MARKUP, which is the whole reason a stacked table keeps
    // its semantics: engines compute table roles from the layout object.
    expect(await table.getAttribute("role")).toBe("table");
    await expect(table.locator("caption")).toHaveText(/\S/);
    await expect(table.locator("thead .nd-table-header-cell").first()).toHaveAttribute(
      "scope",
      "col",
    );
    await expect(table.locator("tbody .nd-table-header-cell").first()).toHaveAttribute(
      "scope",
      "row",
    );

    // The columns are declared, and they are `<colgroup>` elements — so the
    // resting widths are the browser's own with nothing running.
    await expect(table.locator("> .nd-table-column")).toHaveCount(5);

    // And nothing claims what it cannot keep.
    await expect(page.locator(".nd-table-sort")).toHaveCount(0);
    await expect(page.locator(".nd-table-filter")).toHaveCount(0);
    await expect(page.locator(".nd-table-resize")).toHaveCount(0);
    await expect(page.locator("[data-nd-table-tools]").first()).toBeHidden();
    await context.close();
  });

  test("a choice still submits, because the control is a real one", async ({ browser }) => {
    const context = await browser.newContext({ javaScriptEnabled: false });
    const page = await context.newPage();
    await page.goto(url(PAGE.tables));
    await showParts(page);
    const box = named(page, "team").locator("tbody .nd-table-select input").first();
    expect(await box.getAttribute("type")).toBe("checkbox");
    expect(await box.getAttribute("name")).toBeTruthy();
    expect(await box.getAttribute("value")).toBeTruthy();
    await context.close();
  });
});

test("a reading table is a table, and the arrow keys stay the browser's", async ({ page }) => {
  await page.goto(url(PAGE.tables));
  await showParts(page);
  const table = named(page, "invoices");
  expect(await table.getAttribute("role")).toBe("table");
  expect(await table.getAttribute("data-nd-interactive")).toBe(null);
  // NOT ONE CELL IS A TAB STOP. Taking the arrow keys from a reader who only
  // wants to read is the mistake every data grid on the web has made.
  expect(await table.locator("td[tabindex], th[tabindex]").count()).toBe(0);
});

test("the caption of a stacked table is one line the width of the table", async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 900 });
  await page.goto(url(PAGE.tables));
  await showParts(page);
  const table = named(page, "cell-label");
  await table.scrollIntoViewIfNeeded();

  // THE DEFECT THIS COMPONENT WAS REBUILT FOR. A `display: table-caption` box
  // inside a `display: block` parent has no table to be the caption of and
  // shrinks to its own `min-content`.
  const measured = await table.evaluate((el) => {
    const caption = el.querySelector("caption");
    return {
      display: getComputedStyle(caption).display,
      caption: caption.getBoundingClientRect().width,
      table: el.getBoundingClientRect().width,
      height: caption.getBoundingClientRect().height,
      line: Number.parseFloat(getComputedStyle(caption).lineHeight),
    };
  });
  expect(measured.display).toBe("block");
  expect(measured.caption).toBeGreaterThan(measured.table * 0.9);
  // One line, not three: the height is a line plus the padding under it.
  expect(measured.height).toBeLessThan(measured.line * 2);
});

// THE OTHER HALF OF THAT PAIR, WHICH WAS NEVER ASSERTED AND THEREFORE BROKE.
//
// The test above pins the caption at 390px, where the rows stack. Nothing
// pinned it at the width where they do not, and the rule handing
// `display: table-caption` back lives inside a `@container` query — which adds
// nothing to specificity, so it lost to its own pair outside the query and
// never applied at any width. A `display: block` child of a `display: table`
// parent is wrapped in an anonymous table cell, so what shipped was the title
// and the note in a box the width of the first column, drawn under the heading
// row because a header group is laid first whatever the source order says.
//
// It sweeps every caption on the page rather than one, because the defect was
// not in any one table: it was in a rule every table matches.
test("the caption of a wide table is its heading, at its width and above it", async ({ page }) => {
  await page.setViewportSize({ width: 1280, height: 900 });
  await page.goto(url(PAGE.tables));
  await showParts(page);

  const seen = await page.evaluate(() => {
    // The declared breakpoint the stack is undone at, read rather than typed:
    // the query is on the SCROLLER, so a table in a narrow example frame is
    // legitimately still stacked on a wide screen. That is most of this page.
    const room = 30 * Number.parseFloat(getComputedStyle(document.documentElement).fontSize);
    const visible = [];
    const hidden = [];
    for (const table of document.querySelectorAll(".nd-table")) {
      const caption = table.querySelector(":scope > caption");
      if (!caption) continue;
      const box = caption.getBoundingClientRect();
      const whole = table.getBoundingClientRect();
      const scroll = table.closest(".nd-table-scroll");
      const entry = {
        name: table.getAttribute("data-nd-table"),
        display: getComputedStyle(caption).display,
        caption: box.width,
        table: whole.width,
        // The signature of the anonymous cell: a header group is drawn first,
        // so a caption that became one lands BELOW the top of the table.
        drop: box.top - whole.top,
        // Roomy AND allowed to undo the stack. `layout: "stack"` is the caller
        // fixing the choice, and it is a block caption at every width.
        roomy:
          table.getAttribute("data-nd-layout") === "auto" &&
          !!scroll &&
          scroll.getBoundingClientRect().width >= room,
      };
      if (table.getAttribute("data-nd-caption-hidden") === "true") hidden.push(entry);
      else visible.push(entry);
    }
    return { visible, hidden };
  });

  // A floor on each, because the count of things examined is itself a claim.
  expect(seen.visible.length).toBeGreaterThan(8);
  expect(seen.hidden.length).toBeGreaterThan(4);
  const roomy = seen.visible.filter((one) => one.roomy);
  expect(roomy.length).toBeGreaterThan(8);

  for (const one of seen.visible) {
    // TRUE AT EVERY WIDTH, in both layouts, and it is the defect stated as a
    // measurement: an anonymous cell is neither the table's width nor above it.
    expect(one.caption, one.name).toBeGreaterThan(one.table * 0.98);
    expect(one.drop, one.name).toBeLessThan(1);
    // And where the rows are laid as a table, so is the caption of them.
    expect(one.display, one.name).toBe(one.roomy ? "table-caption" : "block");
  }

  // AND THE HIDDEN ONE STAYS TAKEN OUT. The guard that excluded it from the
  // layout rules is what broke the visible one, so it is gone — and this is
  // what says the door it left open is shut. A clipped box as wide as its own
  // sentence still counts towards the document's scroll width.
  for (const one of seen.hidden) {
    expect(one.caption, one.name).toBeLessThan(2);
  }
});

// A CAPTION IS LAID AT THE TABLE'S WIDTH, AND THAT IS NOT A MEASURE.
//
// The note is prose and the table's width is its columns', so on a
// nine-hundred-pixel table the sentence ran past a hundred and twenty
// characters to the line. It is held to the same token every paragraph on this
// site is held to.
test("the note under a caption is held to a measure", async ({ page }) => {
  await page.setViewportSize({ width: 1280, height: 900 });
  await page.goto(url(PAGE.tables));
  await showParts(page);

  const notes = await page.evaluate(() =>
    [...document.querySelectorAll(".nd-table-note")].map((note) => {
      const table = note.closest(".nd-table");
      return {
        name: table.getAttribute("data-nd-table"),
        note: note.getBoundingClientRect().width,
        table: table.getBoundingClientRect().width,
        letters: note.textContent.trim().length,
      };
    }),
  );

  expect(notes.length).toBeGreaterThan(1);
  for (const one of notes) {
    // The floor. A note is only narrower than its table where the table is
    // wide enough for the cap to bite, so the stage has to be that wide.
    expect(one.table, one.name).toBeGreaterThan(700);
    expect(one.letters, one.name).toBeGreaterThan(40);
    expect(one.note, one.name).toBeLessThan(one.table - 1);
  }
});

// --- Sorting -----------------------------------------------------------------

test("sorting has three states, and the third is the order the table arrived in", async ({
  page,
}) => {
  await page.goto(url(PAGE.tables));
  await showParts(page);
  const table = named(page, "invoices-sorted");
  const arrived = await rows(table);

  const control = table.locator('[data-nd-table-sort="amount"]');
  await control.click();
  const up = await rows(table);
  await control.click();
  const down = await rows(table);
  await control.click();
  const back = await rows(table);

  expect(up).not.toEqual(arrived);
  expect(down).toEqual([...up].reverse());
  // Document order is data — a log's chronology, a server's ranking — and a
  // two-state control destroys it on the first press with no way back.
  expect(back).toEqual(arrived);
});

test("a number column sorts as numbers, because the column said so", async ({ page }) => {
  await page.goto(url(PAGE.tables));
  await showParts(page);
  const table = named(page, "invoices-sorted");
  await table.locator('[data-nd-table-sort="amount"]').click();
  const order = await table
    .locator("tbody .nd-table-row [data-nd-column='amount']")
    .allInnerTexts();
  const numbers = order.map((one) => Number.parseInt(one, 10));
  // The failure this replaces: `10` collating before `9`.
  expect(numbers).toEqual([...numbers].sort((a, b) => a - b));
});

test("only one column claims the sort, and it announces the change", async ({ page }) => {
  await page.goto(url(PAGE.tables));
  await showParts(page);
  const table = named(page, "invoices-sorted");
  await table.locator('[data-nd-table-sort="due"]').click();
  await table.locator('[data-nd-table-sort="amount"]').click();

  const sorted = await table
    .locator("thead .nd-table-header-cell")
    .evaluateAll((all) => all.map((one) => one.getAttribute("aria-sort")));
  expect(sorted.filter((one) => one && one !== "none")).toHaveLength(1);
  await expect(status(page, "invoices-sorted")).toHaveText(/\S/);
});

test("the foot does not sort with the body", async ({ page }) => {
  await page.goto(url(PAGE.tables));
  await showParts(page);
  const table = named(page, "invoice-totals");
  const foot = table.locator("tfoot .nd-table-row").last();
  await expect(foot).toBeVisible();
  // A total that sorted itself into the middle of the data is a number that has
  // stopped being a total.
  expect(await foot.evaluate((el) => el.parentElement.tagName)).toBe("TFOOT");
});

// --- The search, which is one box -------------------------------------------
//
// IT WAS TWO, AND EVERY ONE OF THESE IS A THING THAT SHIPPED. The glass and the
// field were siblings with four pixels between them: two borders, two radii
// that disagreed, two magnifiers once it was open, the focus ring on the bare
// `<input>` inside the second box rather than on the box a reader can see, and
// the engine's own clear control left showing beside ours. Not one of them was
// asserted anywhere, which is why all five shipped together.

/** The search of one named table, and the parts inside it. */
const searchOf = (page, name) => ({
  holder: frame(page, name).locator(".nd-table-search"),
  field: frame(page, name).locator(".nd-table-search-field"),
  glass: frame(page, name).locator(".nd-table-search-open"),
  box: frame(page, name).locator(".nd-table-search-input"),
  clear: frame(page, name).locator(".nd-table-search-clear"),
});

test("the search is one box, and the glass is the control inside it", async ({ page }) => {
  await page.goto(url(PAGE.tables));
  await showParts(page);
  const search = searchOf(page, "searchable");
  await search.holder.scrollIntoViewIfNeeded();

  const shut = await search.holder.evaluate((el) => {
    const field = el.querySelector(".nd-table-search-field");
    const glass = el.querySelector(".nd-table-search-open");
    const box = field.getBoundingClientRect();
    const inside = glass.getBoundingClientRect();
    return {
      // Every drawn magnifier, and there is one. A hidden part draws nothing.
      glyphs: [...el.querySelectorAll("svg")].filter((one) => !one.closest("[hidden]")).length,
      boxes: el.querySelectorAll(".nd-table-search-field").length,
      field: { w: box.width, h: box.height },
      // The glass is INSIDE the box rather than beside it, so its edges are
      // within the field's. Beside it, this was 36 pixels of daylight.
      spare: { start: inside.left - box.left, end: box.right - inside.right },
    };
  });
  expect(shut.glyphs).toBe(1);
  expect(shut.boxes).toBe(1);
  expect(shut.spare.start).toBeGreaterThanOrEqual(0);
  expect(shut.spare.end).toBeGreaterThanOrEqual(0);
  // Shut, it is the size of the buttons it stands in a row with.
  expect(shut.field.w).toBeLessThan(shut.field.h + 6);

  await search.glass.click();
  await expect(search.box).toBeVisible();

  const open = await search.holder.evaluate((el) => ({
    glyphs: [...el.querySelectorAll("svg")].filter((one) => !one.closest("[hidden]")).length,
    boxes: el.querySelectorAll(".nd-table-search-field").length,
    wide: el.querySelector(".nd-table-search-field").getBoundingClientRect().width,
  }));
  // STILL ONE, and this is the duplicate that was reported: the glass drew a
  // magnifier and the field drew a second one beside it.
  expect(open.glyphs).toBe(1);
  expect(open.boxes).toBe(1);
  expect(open.wide).toBeGreaterThan(shut.field.w * 2);
});

// AND ONE ROW IS ONE HEIGHT. Three controls at 32, 34 and 36 pixels is what
// "it does not follow the layout" looks like from the outside, and the search
// was the shortest of the three. Every toolbar on the page rather than one,
// because the mix of controls differs from table to table and the one that
// disagrees is the one nobody put beside the others.
test("every control in a table's toolbar is the same height", async ({ page }) => {
  await page.setViewportSize({ width: 1280, height: 900 });
  await page.goto(url(PAGE.tables));
  await showParts(page);

  const rows = await page.evaluate(() => {
    const out = [];
    for (const tools of document.querySelectorAll(".nd-table-tools")) {
      if (tools.hidden) continue;
      const controls = [];
      for (const kid of tools.children) {
        // The search's box is the control; its wrapper is a layout box.
        const el = kid.querySelector(".nd-table-search-field") ?? kid;
        controls.push({
          what: el.className.split(" ")[0],
          height: Math.round(el.getBoundingClientRect().height),
        });
      }
      out.push({
        table: tools.closest(".nd-table-frame").getAttribute("data-nd-table-frame"),
        controls,
      });
    }
    return out;
  });

  // Floors. A row of one control agrees with itself, and a page with no
  // searched table would pass this without measuring one.
  expect(rows.length).toBeGreaterThan(4);
  const kinds = new Set(rows.flatMap((row) => row.controls.map((one) => one.what)));
  expect(kinds.size).toBeGreaterThan(2);
  expect([...kinds]).toContain("nd-table-search-field");

  const heights = new Set(rows.flatMap((row) => row.controls.map((one) => one.height)));
  expect([...heights], JSON.stringify(rows)).toHaveLength(1);
});

test("the search box is the platform's own Search field, token for token", async ({ page }) => {
  const read = (el) => {
    const s = getComputedStyle(el);
    return {
      radius: s.borderTopLeftRadius,
      border: s.borderTopWidth,
      style: s.borderTopStyle,
      colour: s.borderTopColor,
      background: s.backgroundColor,
    };
  };

  await page.goto(url(PAGE.search));
  await showParts(page);
  const platform = await page
    .locator(".nd-search-field")
    .first()
    .evaluate(read);

  await page.goto(url(PAGE.tables));
  await showParts(page);
  const mine = await searchOf(page, "searchable").field.evaluate(read);

  // A pill beside a rounded rectangle is what was reported, so the radius is
  // the one that matters most — but a border that disagreed would be the same
  // defect with a smaller number, so all five are read.
  expect(mine).toEqual(platform);
});

test("the ring lands on the box a reader can see, never on the bare input", async ({ page }) => {
  await page.goto(url(PAGE.tables));
  await showParts(page);
  const search = searchOf(page, "searchable");
  await search.glass.click();
  await search.box.focus();

  const rings = await search.holder.evaluate((el) => {
    const one = (node) => {
      const s = getComputedStyle(node);
      return { style: s.outlineStyle, width: Number.parseFloat(s.outlineWidth) };
    };
    return {
      field: one(el.querySelector(".nd-table-search-field")),
      box: one(el.querySelector(".nd-table-search-input")),
      glass: one(el.querySelector(".nd-table-search-open")),
    };
  });
  // The blue rectangle drawn over the field in the report.
  expect(rings.box.style).toBe("none");
  expect(rings.glass.style).toBe("none");
  expect(rings.field.style).not.toBe("none");
  expect(rings.field.width).toBeGreaterThan(0);
});

test("the clear control is ours, and it is there only when there is a query", async ({ page }) => {
  await page.goto(url(PAGE.tables));
  await showParts(page);
  const table = named(page, "searchable");
  const search = searchOf(page, "searchable");
  const all = (await rows(table)).length;
  expect(all).toBeGreaterThan(1);

  await search.glass.click();
  // A control that empties an empty box does nothing, and a reader learns to
  // distrust the row it is in.
  await expect(search.clear).toBeHidden();

  await search.box.fill("Aurora");
  await expect(search.clear).toBeVisible();
  expect(await search.clear.getAttribute("aria-label")).toMatch(/\S/);
  // Waited for rather than read once: the filter follows the pause, so a count
  // taken straight after the typing is the count from before it.
  await expect.poll(async () => (await rows(table)).length).toBeLessThan(all);

  await search.clear.click();
  await expect(search.box).toHaveValue("");
  // Emptying leaves the box open and focused: a reader emptying a search is
  // usually about to type another one.
  await expect(search.box).toBeFocused();
  await expect(search.clear).toBeHidden();
  await expect.poll(async () => (await rows(table)).length).toBe(all);

  // AND THE ENGINE'S OWN IS TAKEN AWAY. Blink draws a cross inside the box and
  // Gecko draws none, so leaving it is a second clear control half the readers
  // get, four pixels from ours. Asserted as what it DOES rather than as the
  // declaration that removes it: `getComputedStyle` on that pseudo-element
  // answers for the input, so it says `auto` whether the rule is there or not.
  // A BAND RATHER THAN A POINT, because where the engine puts that control is
  // the engine's business: it sits at the end of the CONTENT box, so a single
  // click five pixels from the edge lands in the padding and misses it. The
  // first version of this test did exactly that and passed with the defect
  // restored.
  // Measured with the query already in the box: our own clear control appears
  // with the first letter and moves the input's trailing edge 32 pixels.
  await search.box.fill("Aurora");
  await expect(search.clear).toBeVisible();
  const band = await search.box.evaluate((el) => {
    const box = el.getBoundingClientRect();
    const at = [];
    for (let back = 2; back <= 26; back += 2) {
      at.push({ x: box.right - back, y: box.top + box.height / 2 });
    }
    return at;
  });
  expect(band.length).toBeGreaterThan(10);
  for (const spot of band) {
    await page.mouse.click(spot.x, spot.y);
    await expect(search.box).toHaveValue("Aurora");
  }
});

test("Escape empties the search, shuts the box and hands focus back to the glass", async ({
  page,
}) => {
  await page.goto(url(PAGE.tables));
  await showParts(page);
  const table = named(page, "searchable");
  const search = searchOf(page, "searchable");
  const all = (await rows(table)).length;

  await search.glass.click();
  await search.box.fill("Aurora");
  await expect.poll(async () => (await rows(table)).length).toBeLessThan(all);

  await search.box.press("Escape");
  await expect(search.box).toBeHidden();
  await expect(search.glass).toBeFocused();
  expect(await search.glass.getAttribute("aria-expanded")).toBe("false");
  // Shutting clears: a box that is out of sight and still narrowing the table
  // is a table missing rows for a reason nothing on the screen gives.
  await expect.poll(async () => (await rows(table)).length).toBe(all);
});

// --- Filtering, the totals and the count -------------------------------------

test("a search hides rows, recounts the total and says how much is showing", async ({ page }) => {
  await page.goto(url(PAGE.tables));
  await showParts(page);
  const table = named(page, "invoice-totals");
  const total = table.locator('tfoot [data-nd-aggregate="sum"]');
  const before = await total.innerText();

  await frame(page, "invoice-totals").locator(".nd-table-search-open").click();
  await frame(page, "invoice-totals").locator(".nd-table-search-input").fill("Bracken");
  await expect(total).not.toHaveText(before);

  const visible = await rows(table);
  expect(visible.length).toBeGreaterThan(0);
  await expect(status(page, "invoice-totals")).toHaveText(/\S/);

  // HIDDEN, NEVER REMOVED. Clearing is the table exactly as it was.
  await frame(page, "invoice-totals").locator(".nd-table-search-input").fill("");
  await expect(total).toHaveText(before);
});

test("a filter that matches nothing shows the caller's own empty row", async ({ page }) => {
  await page.goto(url(PAGE.tables));
  await showParts(page);
  const table = named(page, "searchable");
  await frame(page, "searchable").locator(".nd-table-search-open").click();
  await frame(page, "searchable").locator(".nd-table-search-input").fill("nothing-matches-this");
  await expect(table.locator("[data-nd-empty='true']")).toBeVisible();
  // And a sum of nothing is nothing, not the number that was there before.
  await expect(frame(page, "searchable").locator("[data-nd-table-summary]")).toBeVisible();
});

test("the stripe is counted over the rows a reader can see", async ({ page }) => {
  await page.goto(url(PAGE.tables));
  await showParts(page);
  const table = named(page, "deploys-dense");
  const stripes = await table
    .locator("tbody .nd-table-row:not([hidden])")
    .evaluateAll((all) => all.map((one) => one.getAttribute("data-nd-stripe")));
  // `:nth-child(even)` counts rows the filter hid; this alternates over what is
  // actually on the screen.
  expect(stripes).toEqual(stripes.map((_, index) => (index % 2 === 0 ? "even" : "odd")));
});

// --- Selection ---------------------------------------------------------------

test("the select-all box has the third state, and submits nothing itself", async ({ page }) => {
  await page.goto(url(PAGE.tables));
  await showParts(page);
  const table = named(page, "team");
  const all = table.locator(".nd-table-select-all input");

  // Some, not all — the state the platform has and nothing hand-built
  // remembers, and the one thing on this control a script is needed for.
  expect(await all.evaluate((el) => el.indeterminate)).toBe(true);
  expect(await all.getAttribute("name")).toBe(null);

  await all.dispatchEvent("click");
  await all.evaluate((el) => {
    el.checked = true;
    el.dispatchEvent(new Event("change", { bubbles: true }));
  });
  // EVERY ROW THAT MAY BE CHOSEN, AND NOT THE ONE THAT MAY NOT. A select-all
  // that ticked a refused row would be a form posting a choice the browser
  // itself refuses.
  const boxes = await table
    .locator("tbody .nd-table-select input")
    .evaluateAll((all) => all.map((one) => [one.checked, one.disabled]));
  expect(boxes.filter(([, off]) => !off).every(([on]) => on)).toBe(true);
  expect(boxes.filter(([, off]) => off).every(([on]) => !on)).toBe(true);
  await expect(status(page, "team")).toHaveText(/\S/);
});

test("a single-choice table draws radios, and a refused row is still counted", async ({ page }) => {
  await page.goto(url(PAGE.tables));
  await showParts(page);
  const table = named(page, "row-one");
  const inputs = table.locator("tbody .nd-table-select input");
  expect(await inputs.first().getAttribute("type")).toBe("radio");
  // One answer with several values, which is what the shared name makes it.
  const names = await inputs.evaluateAll((all) => [...new Set(all.map((one) => one.name))]);
  expect(names).toHaveLength(1);
  // A row that cannot be chosen is not a row that has gone away.
  await expect(inputs.nth(2)).toBeDisabled();
  await expect(table.locator("tbody .nd-table-row").nth(2)).toBeVisible();
});

// --- The detail row ----------------------------------------------------------

test("a detail row is a row, and it is closed until its own control opens it", async ({ page }) => {
  await page.goto(url(PAGE.tables));
  await showParts(page);
  const table = named(page, "row-expand");
  const detail = table.locator("[data-nd-detail-for]").first();
  const control = table.locator(".nd-table-expand-button").first();

  await expect(detail).toBeHidden();
  expect(await control.getAttribute("aria-expanded")).toBe("false");
  expect(await detail.evaluate((el) => el.tagName)).toBe("TR");

  await control.click();
  await expect(detail).toBeVisible();
  expect(await control.getAttribute("aria-expanded")).toBe("true");
  // The control points at the row it opens, and the row is really there.
  expect(await control.getAttribute("aria-controls")).toBe(await detail.getAttribute("id"));
});

// --- Columns -----------------------------------------------------------------

test("hiding a column takes its declaration, its heading and its cells together", async ({
  page,
}) => {
  await page.goto(url(PAGE.tables));
  await showParts(page);
  const table = named(page, "invoices-wide");
  // The menu is a real `<details>`, closed until a reader opens it — so the
  // control has to be reached the way a reader reaches it.
  await frame(page, "invoices-wide").locator(".nd-table-columns-summary").click();
  const toggle = frame(page, "invoices-wide")
    .locator("[data-nd-table-column-toggle]")
    .first();
  const name = await toggle.inputValue();

  await toggle.uncheck();
  expect(await table.locator(`> [data-nd-column="${name}"]`).getAttribute("data-nd-hidden")).toBe(
    "true",
  );
  await expect(table.locator(`thead [data-nd-column="${name}"]`)).toBeHidden();
  await expect(table.locator(`tbody [data-nd-column="${name}"]`).first()).toBeHidden();
  await expect(status(page, "invoices-wide")).toHaveText(/\S/);

  await toggle.check();
  await expect(table.locator(`thead [data-nd-column="${name}"]`)).toBeVisible();
});

test("a column with nothing hideable in it has no menu at all", async ({ page }) => {
  await page.goto(url(PAGE.tables));
  await showParts(page);
  // Asked for and refused: a menu of nothing is a control that does nothing.
  await expect(frame(page, "invoices").locator("[data-nd-table-columns]")).toHaveCount(0);
});

test("a column resizes from the keyboard, and says how wide it now is", async ({ page }) => {
  await page.goto(url(PAGE.tables));
  await showParts(page);
  const table = named(page, "invoices-wide");
  const handle = table.locator(".nd-table-resize").first();
  const box = table.locator('[data-nd-column="client"] col');

  await handle.focus();
  // ALT AND THE ARROWS. A four-pixel drag target is a target a reader with a
  // tremor cannot hit and a reader with no pointer cannot reach at all.
  await page.keyboard.press("Alt+ArrowRight");
  const wider = await box.evaluate((el) => el.style.width);
  expect(wider).toMatch(/%$/);
  await page.keyboard.press("Alt+Shift+ArrowLeft");
  const narrower = await box.evaluate((el) => el.style.width);
  expect(Number.parseFloat(narrower)).toBeLessThan(Number.parseFloat(wider));
  await expect(status(page, "invoices-wide")).toHaveText(/\S/);
});

test("several columns pin, and the second starts where the first ends", async ({ page }) => {
  await page.goto(url(PAGE.tables));
  await showParts(page);
  const table = named(page, "column-pin");
  await table.scrollIntoViewIfNeeded();

  const pinned = await table.locator(".nd-table-pinned").evaluateAll((all) =>
    all.slice(0, 2).map((one) => ({
      position: getComputedStyle(one).position,
      start: getComputedStyle(one).insetInlineStart,
    })),
  );
  expect(pinned).toHaveLength(2);
  for (const one of pinned) expect(one.position).toBe("sticky");
  // The second starts where the first ends — a record identified by two columns
  // is left half-named by a switch that freezes one.
  expect(Number.parseFloat(pinned[1].start)).toBeGreaterThan(0);
});

// --- The grid ----------------------------------------------------------------

test("an interactive table is a grid, is one tab stop, and the arrows walk cells", async ({
  page,
}) => {
  await page.goto(url(PAGE.tables));
  await showParts(page);
  const table = named(page, "invoices-sorted");
  expect(await table.getAttribute("role")).toBe("grid");
  expect(await table.getAttribute("data-nd-interactive")).not.toBe(null);

  // ONE STOP for the whole grid, not one per cell.
  expect(await table.locator("td[tabindex='0'], th[tabindex='0']").count()).toBe(1);

  await table.locator("td[tabindex='0'], th[tabindex='0']").first().focus();
  await page.keyboard.press("ArrowRight");
  const across = await page.evaluate(() => document.activeElement.textContent.trim());
  await page.keyboard.press("ArrowDown");
  const down = await page.evaluate(() => document.activeElement.textContent.trim());
  await page.keyboard.press("Control+Home");
  const home = await page.evaluate(() => document.activeElement.textContent.trim());

  expect(across).not.toBe(down);
  expect(home).not.toBe(down);
  expect(await page.evaluate(() => document.activeElement.matches("td, th"))).toBe(true);
});

test("copying writes tab-separated text and never reads the clipboard", async ({ page }) => {
  await page.goto(url(PAGE.tables));
  await showParts(page);
  const written = [];
  await page.exposeFunction("recordCopy", (text) => written.push(text));
  await page.evaluate(() => {
    // The clipboard is replaced rather than granted: what matters is what the
    // component ASKS for, and that it never asks to read.
    navigator.clipboard.writeText = (text) => {
      window.recordCopy(text);
      return Promise.resolve();
    };
    navigator.clipboard.readText = () => {
      window.recordCopy("READ");
      return Promise.resolve("");
    };
  });

  const table = named(page, "team");
  await table.locator("tbody .nd-table-select input").first().check();
  await frame(page, "team").locator(".nd-table-copy").click();
  await expect.poll(() => written.length).toBeGreaterThan(0);

  expect(written.join("")).not.toContain("READ");
  const [text] = written;
  expect(text.split("\n")[0]).toContain("\t");
  // The chosen rows when there are any, which is what a reader who ticked one
  // meant by pressing copy.
  expect(text.split("\n")).toHaveLength(2);
  await expect(status(page, "team")).toHaveText(/\S/);
});

// --- The page ----------------------------------------------------------------

test("the scrolling box is a named region only while it scrolls", async ({ page }) => {
  await page.goto(url(PAGE.tables));
  await showParts(page);
  // The boxes around a real table. The variant grid wraps its specimens in the
  // same class so a `<thead>` handed to the parser has a table to live in, and
  // a specimen is a picture of a component rather than one — nothing enhances
  // it, and nothing should claim it is a region a reader scrolls.
  const boxes = await page.locator(".nd-table-scroll:has([data-nd-table])").evaluateAll((all) =>
    all.map((one) => ({
      scrolls: one.scrollWidth > one.clientWidth + 1,
      role: one.getAttribute("role"),
      stop: one.getAttribute("tabindex"),
    })),
  );
  expect(boxes.length).toBeGreaterThan(4);
  for (const box of boxes) {
    // SUBTRACTION ONLY: the stop is rendered and taken away, never added.
    expect(box.role === "region").toBe(box.scrolls);
    expect(box.stop === "0").toBe(box.scrolls);
  }
});

test("every part of every table carries its role", async ({ page }) => {
  await page.goto(url(PAGE.tables));
  await showParts(page);
  const wrong = await page.evaluate(() => {
    const wanted = {
      TABLE: "table",
      THEAD: "rowgroup",
      TBODY: "rowgroup",
      TFOOT: "rowgroup",
      TR: "row",
      TD: "cell",
    };
    const bad = [];
    for (const element of document.querySelectorAll("[data-nd-table], [data-nd-table] *")) {
      const table = element.closest("[data-nd-table]");
      if (!table) continue;
      const expected = wanted[element.tagName];
      if (element.tagName === "TH") {
        const scope = element.getAttribute("scope");
        const role = element.getAttribute("role");
        if (scope === "row" ? role !== "rowheader" : role !== "columnheader") {
          bad.push(`${element.tagName}[scope=${scope}] is ${role}`);
        }
        continue;
      }
      if (!expected) continue;
      const role = element === table ? element.getAttribute("role") : element.getAttribute("role");
      // A grid is a table that gained a keyboard, not a table that stopped
      // being one.
      if (element === table && role === "grid") continue;
      if (role !== expected) bad.push(`${element.tagName} is ${role}`);
    }
    return bad;
  });
  expect(wrong).toEqual([]);
});

test("the stacked label is drawn and never announced", async ({ page }) => {
  // NARROW ENOUGH FOR THE LAYOUT THIS IS ABOUT. `.nd-table-label` is
  // `display: none` until a table stacks, and `auto` stacks on the CONTAINER
  // rather than the window. At the width this ran at, every label it read was
  // undrawn, so the half of the title about drawing could not fail.
  await page.setViewportSize({ width: 390, height: 900 });
  await page.goto(url(PAGE.tables));
  await showParts(page);

  const drawn = await page.locator(".nd-table-label").evaluateAll((all) =>
    all
      // A label in a column a reader has hidden is a label for nothing, and the
      // page ships one example with a column already off.
      .filter((one) => !one.closest("td, th")?.hidden)
      .map((one) => {
        const box = one.getBoundingClientRect();
        return {
          text: one.textContent.trim(),
          display: getComputedStyle(one).display,
          width: box.width,
          height: box.height,
          hidden: one.getAttribute("aria-hidden"),
        };
      }),
  );
  expect(drawn.length, "the page stacked nothing, so nothing was labelled").toBeGreaterThan(20);
  for (const label of drawn) {
    // DRAWN, AS A BOX A READER CAN SEE. A stacked value with no name over it is
    // a column of figures nobody can read.
    expect(label.display, `${label.text} is not drawn`).not.toBe("none");
    expect(label.width, `${label.text} is ${label.width}px wide`).toBeGreaterThan(0);
    expect(label.height, `${label.text} is ${label.height}px tall`).toBeGreaterThan(0);
    expect(label.hidden, `${label.text} is offered to a screen reader`).toBe("true");
  }

  // AND OUT OF THE TREE, which is what `aria-hidden` only claims. Read from the
  // browser's own accessibility tree rather than from the attribute, because
  // the attribute is the recipe and this is the meal. A screen reader has the
  // column from the row's heading, so a cell announcing both says it twice.
  const cell = page
    .locator(".nd-table-cell:has(.nd-table-label):has(.nd-table-value)")
    .first();
  const named = await cell.locator(".nd-table-label").evaluate((el) => el.textContent.trim());
  const value = await cell.locator(".nd-table-value").evaluate((el) => el.textContent.trim());
  const said = await cell.ariaSnapshot();
  expect(said, "the cell announces nothing at all").toContain(value);
  expect(said, `the cell says ${named} before every value in the column`).not.toContain(named);
});

test("no two examples on the page render the same thing", async ({ page }) => {
  await page.goto(url(PAGE.tables));
  await showParts(page);
  // THE DEFECT THIS PAGE WAS REBUILT FOR, AS A TEST. Five of the thirty
  // examples it replaced were byte-identical apart from which rows they listed:
  // `TableSection { kind: Body }` emitted character-for-character what a bare
  // `TableSection` emitted, and a heading's `tone` was written into the markup
  // with no rule in the stylesheet reading it. Neither is visible in a
  // screenshot anybody skims, and both are visible here.
  //
  // `lineage.spec.js` has the same guard over its variant grid, for the same
  // reason and after the same kind of report.
  const marks = await page.locator(".nd-example-stage").evaluateAll((all) =>
    all.map((stage) => {
      const table = stage.querySelector("[data-nd-table]");
      if (!table) return stage.textContent.replace(/\s+/g, " ").trim();
      // The shape a reader sees: every switch the table carries, every column
      // declaration, and the words in the body.
      const shape = [...table.attributes]
        .filter((one) => one.name.startsWith("data-nd") && one.name !== "data-nd-table")
        .map((one) => `${one.name}=${one.value}`)
        .sort()
        .join(" ");
      const columns = [...table.querySelectorAll(":scope > .nd-table-column")]
        .map((one) =>
          [...one.attributes]
            .filter((two) => two.name.startsWith("data-nd"))
            .map((two) => `${two.name}=${two.value}`)
            .join(","),
        )
        .join("|");
      const body = table.textContent.replace(/\s+/g, " ").trim();
      return `${shape}||${columns}||${body}`;
    }),
  );
  const seen = new Map();
  const twins = [];
  for (const [index, mark] of marks.entries()) {
    if (seen.has(mark)) twins.push(`${seen.get(mark)} and ${index}`);
    seen.set(mark, index);
  }
  expect(marks.length).toBeGreaterThan(20);
  expect(twins).toEqual([]);
});

test("every heading sits over its own column, on every table on the page", async ({ page }) => {
  await page.goto(url(PAGE.tables));
  await showParts(page);
  // THE DEFECT A SCREENSHOT SHOWS AND NOTHING ELSE DOES. A row draws its own
  // choice control and its own expand control; the heading row draws neither
  // unless the caller remembered, and forgetting slides every value one column
  // right of the words that name it. It shipped on the expanding example with
  // BUILD over the chevrons.
  //
  // Three claims, over every table on the page rather than over one: the head
  // has a cell for every column a row draws, each heading starts where its own
  // cell starts, and nothing scrolls sideways with room to spare.
  const wrong = await page.evaluate(() => {
    const bad = [];
    for (const table of document.querySelectorAll("[data-nd-table]")) {
      const name = table.getAttribute("data-nd-table");
      const scroll = table.closest(".nd-table-scroll");
      const head = table.querySelector("thead .nd-table-row");
      const body = [...table.querySelectorAll("tbody .nd-table-row")].filter(
        (row) =>
          !row.hasAttribute("data-nd-detail-for") && !row.classList.contains("nd-table-group"),
      );
      // COUNTED IN COLUMNS, NOT IN CELLS — the same correction the runtime
      // needed. A two-level heading is two cells and three columns, and a test
      // that counts children calls that a table one heading short.
      const across = (row) =>
        [...row.children].reduce((total, cell) => total + (cell.colSpan || 1), 0);
      const widest = Math.max(0, ...body.map(across));
      if (head && across(head) !== widest) {
        bad.push(`${name}: the head covers ${across(head)} columns and a row draws ${widest}`);
      }
      if (head) {
        // A heading's column is where the spans before it have got to, so a
        // heading after a spanning one is compared against the right cell.
        let at = 0;
        for (const heading of [...head.children]) {
          const index = at;
          at += heading.colSpan || 1;
          const column = heading.getAttribute("data-nd-column");
          if (!column || heading.hidden || (heading.colSpan || 1) > 1) continue;
          const cell = body[0]?.children[index];
          if (!cell || cell.hidden) continue;
          const over = heading.getBoundingClientRect();
          const under = cell.getBoundingClientRect();
          if (Math.abs(over.x - under.x) > 2) {
            bad.push(`${name}: ${column} heading at ${Math.round(over.x)}, its cell at ${Math.round(under.x)}`);
          }
        }
      }
      if (scroll && scroll.scrollWidth > scroll.clientWidth + 1) {
        bad.push(`${name}: scrolls sideways at ${scroll.scrollWidth} in ${scroll.clientWidth}`);
      }
    }
    return bad;
  });
  expect(wrong).toEqual([]);
});

test("an aligned cell is really aligned, at every width", async ({ page }) => {
  // THE DEFECT THAT LOOKED LIKE A DEFECT IN SOMETHING ELSE. The stacked layout
  // flattens every cell to `start`, and that rule is one class and two
  // attributes deep — so it outranked `[data-nd-align]` at every width, and
  // `numeric` did nothing on the whole site. It was invisible because a heading
  // is a flex column that ends its own items: the words moved to the end, the
  // values under them did not, and a column of money read as one that had come
  // loose from its name.
  //
  // MEASURED, NOT READ. `text-align` is not what places a heading's words. The
  // heading is a flex column, and a separate `align-items` rule ends its items.
  // A test that read the property was asking a rule with no say over the thing
  // it asserted. What is read below is where the drawn words sit.
  for (const width of [1440, 900]) {
    await page.setViewportSize({ width, height: 900 });
    await page.goto(url(PAGE.tables));
    await showParts(page);
    const cells = await page.evaluate(() => {
      /** Where a cell's drawn content sits, against the cell's own content box. */
      const placed = (cell) => {
        const heading = cell.querySelector(".nd-table-heading");
        const boxes = [];
        if (heading) {
          // The column's items, which is what `align-items` moves.
          for (const item of heading.children) boxes.push(item.getBoundingClientRect());
        } else {
          // The runs of the cell's own text, which is what `text-align` moves.
          // A range over a text node is the glyphs and nothing around them.
          const walk = document.createTreeWalker(cell, NodeFilter.SHOW_TEXT);
          for (let node = walk.nextNode(); node; node = walk.nextNode()) {
            if (!node.textContent.trim()) continue;
            const range = document.createRange();
            range.selectNodeContents(node);
            boxes.push(...range.getClientRects());
          }
        }
        const real = boxes.filter((one) => one.width > 0);
        if (real.length === 0) return null;
        const style = getComputedStyle(cell);
        const box = cell.getBoundingClientRect();
        const from =
          box.left + Number.parseFloat(style.borderLeftWidth) + Number.parseFloat(style.paddingLeft);
        const to =
          box.right -
          Number.parseFloat(style.borderRightWidth) -
          Number.parseFloat(style.paddingRight);
        const left = Math.min(...real.map((one) => one.left));
        const right = Math.max(...real.map((one) => one.right));
        return { start: left - from, end: to - right, slack: to - from - (right - left) };
      };

      const measured = [];
      for (const cell of document.querySelectorAll(
        '[data-nd-align="numeric"], [data-nd-align="end"], [data-nd-align="center"]',
      )) {
        // A stacked table really does flatten them: there is no column left
        // to align to, and the label sits above the value.
        const table = cell.closest(".nd-table");
        if (!table || getComputedStyle(table).display !== "table") continue;
        const where = placed(cell);
        // A cell its own words fill says nothing about where they were put.
        if (!where || where.slack <= 4) continue;
        measured.push({
          align: cell.getAttribute("data-nd-align"),
          where,
          name: `${cell.closest("[data-nd-table]")?.getAttribute("data-nd-table")}: ${cell.textContent.trim().slice(0, 20)}`,
        });
      }
      return measured;
    });

    const ends = cells.filter((cell) => cell.align !== "center");
    const middles = cells.filter((cell) => cell.align === "center");
    // The floor. A sweep that measured nothing would keep every claim under it.
    expect(ends.length, `${width}px: no aligned cell has room to be aligned in`).toBeGreaterThan(20);
    expect(middles.length, `${width}px: nothing on the page declares centre`).toBeGreaterThan(0);
    for (const cell of ends) {
      expect(
        cell.where.end,
        `${width}px: ${cell.name} stops ${Math.round(cell.where.end)}px short of the end`,
      ).toBeLessThanOrEqual(1);
    }
    for (const cell of middles) {
      expect(
        Math.abs(cell.where.start - cell.where.end),
        `${width}px: ${cell.name} sits ${Math.round(cell.where.start)}px in and ${Math.round(cell.where.end)}px out`,
      ).toBeLessThanOrEqual(1);
    }
  }
});

test("hiding a column takes that column and no other", async ({ page }) => {
  await page.goto(url(PAGE.tables));
  await showParts(page);
  const table = named(page, "column-hide");
  const frame = page.locator('.nd-table-frame:has([data-nd-table="column-hide"])');
  await frame.locator(".nd-table-columns-summary").click();

  const before = await table
    .locator("thead th")
    .evaluateAll((all) => all.map((one) => [one.getAttribute("data-nd-column"), one.hidden]));
  const toggle = frame.locator("[data-nd-table-column-toggle]").first();
  const name = await toggle.inputValue();
  await toggle.uncheck();
  const after = await table
    .locator("thead th")
    .evaluateAll((all) => all.map((one) => [one.getAttribute("data-nd-column"), one.hidden]));

  // A hidden column used to be hidden TWICE — `visibility: collapse` on the
  // `<colgroup>` and `hidden` on every cell — and automatic table layout then
  // took the next column away as well.
  const moved = after.filter(([column, gone], at) => gone !== before[at][1]).map(([column]) => column);
  expect(moved).toEqual([name]);
});

test("no id appears twice on the tables page, in any locale", async ({ page }) => {
  for (const locale of LOCALES) {
    await page.goto(url(PAGE.tables, locale));
    await showParts(page);
    const duplicates = await page.evaluate(() => {
      const seen = new Set();
      const twice = [];
      for (const element of document.querySelectorAll("[id]")) {
        if (seen.has(element.id)) twice.push(element.id);
        seen.add(element.id);
      }
      return twice;
    });
    expect(duplicates).toEqual([]);
  }
});

// --- What the head claims, and what the keyboard does ------------------------

test("aria-sort names the columns a reader can sort by, and no others", async ({ page }) => {
  await page.goto(url(PAGE.tables));
  await showParts(page);
  const counted = await page.evaluate(() => {
    const heads = [...document.querySelectorAll(".nd-table-header-cell")];
    return {
      // MEASURED: 403 of these for 13 sortable columns, 214 of them on row
      // headings that name a record and can never be sorted by.
      sorted: heads.filter((cell) => cell.hasAttribute("aria-sort")).length,
      onRows: heads.filter(
        (cell) => cell.hasAttribute("aria-sort") && cell.getAttribute("scope") !== "col",
      ).length,
      // `none` MEANS "SORTABLE, NOT SORTED", so it may only appear where the
      // script drew a control. A stated direction may stand on its own: a
      // table a server sorted says so whether or not a reader can re-sort it.
      idleWithoutControl: heads.filter(
        (cell) =>
          cell.getAttribute("aria-sort") === "none" && !cell.querySelector(".nd-table-sort"),
      ).length,
      controls: document.querySelectorAll(".nd-table-sort").length,
    };
  });
  expect(counted.onRows).toBe(0);
  expect(counted.idleWithoutControl).toBe(0);
  expect(counted.sorted).toBeGreaterThanOrEqual(counted.controls);
  expect(counted.controls).toBeGreaterThan(0);
});

test("with the script blocked only a stated direction claims a sort", async ({ browser }) => {
  const context = await browser.newContext({ javaScriptEnabled: false });
  const page = await context.newPage();
  await page.goto(url(PAGE.tables));
  await showParts(page);
  const said = await page.evaluate(() =>
    [...document.querySelectorAll(".nd-table-header-cell[aria-sort]")].map((cell) =>
      cell.getAttribute("aria-sort"),
    ),
  );
  // The server writes it only where a caller stated a direction, because a
  // component cannot see its column's declaration. So `none` never ships.
  expect(said.includes("none")).toBe(false);
  await context.close();
});

test("Enter chooses a row, and Shift with an arrow extends the choice", async ({ page }) => {
  await page.goto(url(PAGE.tables));
  await showParts(page);
  const table = named(page, "team");
  const boxes = table.locator("tbody .nd-table-select input");
  await expect(boxes.first()).toBeVisible();

  // Focused rather than clicked: a click may land on the `<div>` inside the
  // cell, and the grid's handler answers only a key pressed on the cell.
  const cell = table.locator("tbody .nd-table-row").first().locator("th, td").nth(1);
  await cell.evaluate((el) => el.focus());
  // ENTER, WHICH THE CONTRACT HAS ALWAYS NAMED AND ONLY SPACE ANSWERED. The
  // example arrives with a row already chosen, so what is asserted is the
  // toggle rather than a state.
  const first = await boxes.nth(0).isChecked();
  await page.keyboard.press("Enter");
  expect(await boxes.nth(0).isChecked()).toBe(!first);
  await page.keyboard.press("Enter");
  expect(await boxes.nth(0).isChecked()).toBe(first);

  // AND SHIFT EXTENDS. `shiftKey` used to appear in the resize handle and in a
  // pointer click and nowhere else, so the row of the keyboard table about
  // choosing several rows from the keyboard reached no code at all.
  await boxes.nth(1).uncheck();
  await cell.evaluate((el) => el.focus());
  await page.keyboard.press("Shift+ArrowDown");
  expect(await boxes.nth(1).isChecked()).toBe(true);
});

test("a table with a chosen row says nothing until the choice changes", async ({ page }) => {
  await page.goto(url(PAGE.tables));
  await showParts(page);
  // `wireSelection` painted at wiring time and `paint` ended in `say`, so a
  // table that arrived with a row chosen announced "1 person chosen." having
  // changed nothing.
  const spoken = await page
    .locator("[data-nd-table-status]")
    .evaluateAll((all) => all.map((one) => one.textContent.trim()).filter(Boolean));
  expect(spoken).toEqual([]);
});

test("Shift on a resize moves ten steps, which is what both contracts say", async ({ page }) => {
  await page.goto(url(PAGE.tables));
  await showParts(page);
  const handle = page.locator(".nd-table-resize").first();
  const width = () =>
    handle.evaluate((el) => {
      const column = el.closest(".nd-table-header-cell")?.dataset.ndColumn;
      const box = el
        .closest(".nd-table")
        ?.querySelector(`.nd-table-column[data-nd-column="${column}"] col`);
      return Number.parseFloat(box?.style.width ?? "0");
    });

  await handle.focus();
  await page.keyboard.press("Alt+ArrowRight");
  const one = await width();
  await handle.focus();
  await page.keyboard.press("Shift+Alt+ArrowRight");
  const ten = await width();
  // One step is 2% of the table, so ten of them is twenty. It was five.
  expect(Math.round(ten - one)).toBe(20);
});

test("an emphasised row is not one of the records a total is taken over", async ({ page }) => {
  await page.goto(url(PAGE.tables));
  await showParts(page);
  // `emphasis` IS LOAD-BEARING, and its doc used to say in bold that it is
  // not. A subtotal line must not be summed into the total above it and a
  // group heading must not be counted as a row, so `recount` filters both out
  // — which makes the prop decide what a row IS rather than how it looks.
  const reckoned = await named(page, "row-emphasis").evaluate((table) => {
    const number = (text) =>
      Number.parseFloat(String(text).replace(/[^\d.,-]/g, "").replace(",", ".")) || 0;
    const cell = table.querySelector('[data-nd-aggregate]:not([data-nd-aggregate="none"])');
    if (!cell) return null;
    const column = cell.getAttribute("data-nd-column");
    // The scope the cell declares: its own section, or the whole table.
    const scope =
      cell.getAttribute("data-nd-aggregate-over") === "section"
        ? cell.closest("tbody, tfoot")
        : table;
    const rows = [...scope.querySelectorAll("tbody .nd-table-row")].map((row) => ({
      plain: (row.getAttribute("data-nd-emphasis") ?? "none") === "none",
      value: number(row.querySelector(`[data-nd-column="${column}"] .nd-table-value`)?.textContent),
    }));
    const sum = (only) => rows.filter(only).reduce((total, row) => total + row.value, 0);
    return {
      said: number(cell.textContent),
      plain: sum((row) => row.plain),
      every: sum(() => true),
      emphasised: rows.filter((row) => !row.plain).length,
    };
  });
  expect(reckoned, "no aggregate on the table this is about").not.toBeNull();
  expect(reckoned.emphasised, "no emphasised row to leave out").toBeGreaterThan(0);
  expect(Math.round(reckoned.said)).toBe(Math.round(reckoned.plain));
  expect(Math.round(reckoned.said)).not.toBe(Math.round(reckoned.every));
});

// --- One tab stop, and the way into a cell -----------------------------------
//
// A `role="grid"` promises that Tab skips the whole table in one press and the
// arrows walk it. The row-select box and the expand control are focusable on
// their own, so the promise was broken by the markup the component itself
// writes: a hundred-row table was two hundred and one stops. These three say
// the promise is kept, that a reader can still reach what is in a cell, and
// that releasing puts back what it found.

test("a grid is one tab stop, whatever the cells hold", async ({ page }) => {
  await page.goto(url(PAGE.tables));
  await showParts(page);
  const table = named(page, "team");
  expect(await table.getAttribute("role")).toBe("grid");

  const stops = await table.evaluate((el) =>
    [...el.querySelectorAll("td, th, button, a[href], input, select, textarea")].filter(
      (node) => (node.getAttribute("tabindex") ?? "0") === "0" && !node.disabled,
    ).length,
  );
  expect(stops, "the grid offers more than one tab stop").toBe(1);
});

test("Enter goes into a cell and Escape comes back out", async ({ page }) => {
  await page.goto(url(PAGE.tables));
  await showParts(page);
  const table = named(page, "team");

  // The cell that holds the row's own control, reached the way a reader would.
  const cell = table.locator("tbody .nd-table-select").first();
  await cell.focus();
  await expect(cell).toBeFocused();

  // Enter chooses the row here, because that is what the contract says Enter
  // does in a cell with a chooser. F2 is the way in.
  await page.keyboard.press("F2");
  const box = cell.locator("input");
  await expect(box).toBeFocused();

  // And the arrows belong to what is inside now, not to the table.
  const before = await rows(table);
  await page.keyboard.press("ArrowDown");
  await expect(box).toBeFocused();
  expect(await rows(table)).toEqual(before);

  await page.keyboard.press("Escape");
  await expect(cell).toBeFocused();
});

test("releasing gives the cells' widgets their tab stops back", async ({ page }) => {
  await page.goto(url(PAGE.tables));
  await showParts(page);
  const table = named(page, "team");
  const quieted = await table.locator("[data-nd-grid-tabindex]").count();
  expect(quieted, "nothing was quieted, so this proves nothing").toBeGreaterThan(0);

  // Through the shell the page is already running, rather than a second URL
  // written here: the site is served under a base path, and a second spelling
  // of it is a second thing that can be wrong.
  await page.evaluate(async () => {
    const tag = document.querySelector('script[type="module"][src*="shell.js"]');
    const shell = await import(tag.src);
    shell.releaseAll(document.body);
  });
  await expect(table.locator("[data-nd-grid-tabindex]")).toHaveCount(0);
  const box = table.locator("tbody .nd-table-select input").first();
  expect(await box.getAttribute("tabindex")).toBe(null);
});
