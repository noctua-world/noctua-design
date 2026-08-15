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
  await page.goto(url(PAGE.tables));
  await showParts(page);
  const labels = await page
    .locator(".nd-table-label")
    .evaluateAll((all) => all.map((one) => one.getAttribute("aria-hidden")));
  expect(labels.length).toBeGreaterThan(0);
  // A screen reader already has the column from its heading.
  for (const one of labels) expect(one).toBe("true");
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
  for (const width of [1440, 900]) {
    await page.setViewportSize({ width, height: 900 });
    await page.goto(url(PAGE.tables));
    await showParts(page);
    const wrong = await page.evaluate(() =>
      [...document.querySelectorAll('[data-nd-align="numeric"], [data-nd-align="end"]')]
        .filter((cell) => {
          // A stacked table really does flatten them: there is no column left
          // to align to, and the label sits above the value.
          const table = cell.closest(".nd-table");
          return table && getComputedStyle(table).display === "table";
        })
        .filter((cell) => !["end", "right"].includes(getComputedStyle(cell).textAlign))
        .map((cell) => `${cell.closest("[data-nd-table]")?.getAttribute("data-nd-table")}: ${cell.textContent.trim().slice(0, 20)}`),
    );
    expect(wrong).toEqual([]);
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
