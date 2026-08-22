// Everything a `Table` does once a script is running.
//
// IT SHIPS, for `ColorPicker`'s reason: a component whose consumer cannot use
// it is a component that lies. A data grid that only sorts inside this
// repository's own documentation site is not a data grid.
//
// WHAT IT NEVER DOES, AND THESE ARE THE WHOLE DESIGN:
//
//  1. It never takes the arrow keys from a table nobody can drive. A reading
//     table stays `role="table"` and the screen reader keeps its own table
//     navigation, which is better than anything here. The grid appears only
//     when a column is sortable, filterable or resizable, or a row can be
//     chosen or opened — a derivation, so it cannot be set wrongly or
//     forgotten.
//  2. It never invents a word. Every sentence it announces is the caller's,
//     with named holes; every number it writes is `Intl`'s, in the document's
//     own locale.
//  3. It never removes a row. Filtering hides; clearing a filter is the table
//     exactly as it was, and every total recounts from what is left.
//  4. It never reads the clipboard. `copy` writes, and a failure says nothing
//     rather than claiming a copy that did not happen.
//  5. It never guesses which column a cell is in. The `<colgroup>` declarations
//     say, and a cell may name its own column where a span made the position
//     ambiguous.
//
// FIVE THINGS THAT LOOK LIKE A LAYOUT BUG AND ARE NOT ONE:
//
//  a. NEVER HIDE ONE THING TWO WAYS. A column hidden by `visibility: collapse`
//     on its `<colgroup>` AND by `hidden` on its cells makes automatic table
//     layout take the next column away as well.
//  b. A `<col>` WIDTH MUST NOT SWITCH THE TABLE TO `table-layout: fixed`. Under
//     fixed layout every column with no declared width takes an equal share, so
//     one resize resets all the others and the values print through each other.
//     A width is a preference, not a decree.
//  c. THE NARROW LAYOUT IS A CONTAINER QUERY, NOT A MEDIA QUERY. A props table
//     inside a narrow example frame must stack on a wide window, which is the
//     case a media query cannot see. The corollary: a table left to shrink to
//     its content inside a flex column becomes a box under the breakpoint and
//     stacks itself, taking its heading row and every sort control with it. A
//     table in a stack takes the width.
//  d. THE UN-STACKING QUERY MUST RESTORE ALIGNMENT. The stacked rule is one
//     class and two attributes deep, so it outranks `[data-nd-align]` at every
//     width — invisible, because a heading is a flex column that ends its own
//     items and the words move without the values.
//  e. A ROW DRAWS ITS OWN LEADING CELLS AND THE HEADING ROW DOES NOT. A choice
//     control and an expand control are cells the row added, and the `<col>`
//     declarations know nothing about them, so a table with either wants a
//     leading `TableColumn` declared for it. The runtime inserts what a caller
//     did not, because the failure is silent and looks like a broken width
//     rather than like a missing declaration.

import { fill, plural, say } from "./messages.js";

/** How long after the last keystroke a filter is applied. */
const SETTLE = 160;

/** How much of the table's width one resize step is. */
const STEP = 2;

/** The Shift multiplier on a resize, which both contracts call ten steps. */
// IT WAS 5, UNDER TWO SENTENCES SAYING TEN. Named rather than written into the
// handler, the way `color-picker.js` names the same idea, so the number and the
// sentence have one place to disagree instead of three.
const COARSE = 10;

/** Per-table state, so nothing is recomputed on every keystroke. */
const tables = new WeakMap();

/** One collator per language, because building one per comparison is slow. */
const collators = new Map();

function collatorFor(language) {
  const key = language || "und";
  if (!collators.has(key)) {
    // `numeric` so `item 10` follows `item 9`, and `base` so case and accent do
    // not split a column into two orders.
    collators.set(key, new Intl.Collator(language || undefined, { numeric: true, sensitivity: "base" }));
  }
  return collators.get(key);
}

/** The document's language, which is where every locale decision comes from. */
function language() {
  return document.documentElement.lang || undefined;
}

/** A number, or null — so a threshold nobody declared is not zero. */
function number(text) {
  if (text === null || text === undefined || text === "") return null;
  const value = Number.parseFloat(String(text).replace(/[^\d.,-]/g, "").replace(",", "."));
  return Number.isNaN(value) ? null : value;
}

/** Every attribute read the same way, from the table. */
function reader(table) {
  return (name) => table.getAttribute(`data-nd-${name}`) || null;
}

export function enhanceTables(root = document) {
  for (const element of root.querySelectorAll("[data-nd-table]")) {
    if (element.dataset.ndEnhanced) continue;
    element.dataset.ndEnhanced = "table";
    enhance(element, new AbortController());
  }
}

/**
 * Takes the behaviour back off every table under `root`.
 *
 * A BEHAVIOUR THAT CANNOT BE RELEASED IS A DEFECT FOR EVERY CONSUMER WITH A
 * CLIENT-SIDE ROUTER, and this file has the loudest example in the system.
 * `wireMore` puts an `IntersectionObserver` on the holder at the foot of the
 * table, and that observer FETCHES MORE ROWS when the holder scrolls into view.
 * A table taken out of the document keeps the observer, so a page nobody is
 * looking at can still ask a server for data. That is network traffic no reader
 * asked for, charged to a subtree that no longer exists.
 *
 * The `ResizeObserver` beside it is quieter and just as permanent: an
 * observation keeps its target alive whether or not the target is in the
 * document.
 */
export function releaseTables(root = document) {
  for (const element of root.querySelectorAll("[data-nd-table]")) {
    if (element.dataset.ndEnhanced !== "table") continue;
    tables.get(element)?.binding?.abort();
    tables.delete(element);
    delete element.dataset.ndEnhanced;
  }
}

function enhance(table, binding) {
  const signal = binding.signal;
  const at = reader(table);
  const frame = table.closest(".nd-table-frame");
  const scroll = table.closest(".nd-table-scroll");
  const status = frame?.querySelector("[data-nd-table-status]") ?? null;
  const summary = frame?.querySelector("[data-nd-table-summary]") ?? null;
  const tools = frame?.querySelector("[data-nd-table-tools]") ?? null;
  const template = frame?.querySelector("[data-nd-table-column-template]") ?? null;

  const columns = [...table.querySelectorAll(":scope > .nd-table-column")].map((group, index) => ({
    element: group,
    box: group.querySelector("col"),
    name: group.getAttribute("data-nd-column") ?? String(index),
    index,
    sortable: group.getAttribute("data-nd-sortable") === "true",
    sortAs: group.getAttribute("data-nd-sort-as") ?? "text",
    filterable: group.getAttribute("data-nd-filterable") === "true",
    resizable: group.getAttribute("data-nd-resizable") === "true",
    hideable: group.getAttribute("data-nd-hideable") === "true",
    hidden: group.getAttribute("data-nd-hidden") === "true",
    pinned: group.getAttribute("data-nd-pinned") === "true",
    low: number(group.getAttribute("data-nd-low")),
    high: number(group.getAttribute("data-nd-high")),
    toneBelow: group.getAttribute("data-nd-tone-below"),
    toneWithin: group.getAttribute("data-nd-tone-within"),
    toneAbove: group.getAttribute("data-nd-tone-above"),
    width: null,
  }));

  // A ROW DRAWS ITS OWN LEADING CELLS, AND A `<colgroup>` COUNTS POSITIONS.
  //
  // The choice control and the expand control are cells the component added,
  // and the declarations know nothing about them — so every declared width
  // lands one column to the left, and a `fit` column meant for a code squeezes
  // the one beside it into a letter per line. A caller who declares a leading
  // `TableColumn` for the control gets exactly the right count and nothing
  // happens here; a caller who did not gets the missing groups inserted, which
  // is the difference between a table that is merely undeclared and one that is
  // visibly wrong.
  // COUNTED IN COLUMNS, NOT IN CELLS. A cell with a `colspan` is one child and
  // several columns, and counting children is how a two-level heading — one
  // cell spanning two columns beside one spanning two rows — was read as a row
  // two columns short and given a blank heading it did not need, which pushed
  // every word one column right of the values it named.
  const across = (row) =>
    [...row.children].reduce((total, cell) => total + (cell.colSpan || 1), 0);

  const widest = [...table.querySelectorAll("tbody .nd-table-row")]
    .filter((row) => !row.hasAttribute("data-nd-detail-for"))
    .reduce((most, row) => Math.max(most, across(row)), 0);
  for (let missing = widest - columns.length; missing > 0; missing -= 1) {
    const group = document.createElement("colgroup");
    group.className = "nd-table-column nd-table-column-lead";
    group.append(document.createElement("col"));
    table.prepend(group);
  }

  // AND A HEADING FOR IT, or every value sits one column right of the words
  // that name it. A row draws its own choice control and its own expand
  // control; the heading row draws neither unless the caller remembered, and
  // forgetting is silent — measured on the expanding example, where BUILD sat
  // over the chevrons and RESULT over nothing at all.
  const headRow = table.querySelector("thead .nd-table-row");
  if (headRow) {
    for (let missing = widest - across(headRow); missing > 0; missing -= 1) {
      const blank = document.createElement("th");
      blank.className = "nd-table-header-cell nd-table-lead";
      // No `scope`: it names nothing, and a heading that claims a column it
      // does not describe is worse than a blank one.
      blank.setAttribute("role", "columnheader");
      headRow.prepend(blank);
    }
  }

  const state = {
    table,
    frame,
    scroll,
    status,
    summary,
    columns,
    at,
    sorted: null,
    direction: null,
    // CARRIED ON THE STATE, because every `wire*` function below already takes
    // it. A parallel argument threaded through nine call sites is nine places
    // for one of them to be forgotten.
    binding,
    signal,
  };
  tables.set(table, state);

  // THE GRID IS DERIVED. A caller cannot ask for it and cannot forget it: it is
  // true exactly when something in the table can be driven.
  const chooses = table.querySelector(".nd-table-select") !== null;
  const opens = table.querySelector("[data-nd-expandable='true']") !== null;
  const interactive =
    chooses || opens || columns.some((one) => one.sortable || one.filterable || one.resizable);
  state.interactive = interactive;
  table.toggleAttribute("data-nd-interactive", interactive);
  if (interactive) table.setAttribute("role", "grid");

  // THE ORDER THE DOCUMENT HAD, REMEMBERED BEFORE ANYTHING MOVES. Asking a row
  // for its position after a sort gets the position the sort gave it, which is
  // how "clear the sort" came to mean "leave it as it is".
  for (const [index, row] of [...table.querySelectorAll("tbody .nd-table-row")].entries()) {
    row.dataset.ndOrder = String(index);
  }

  wireColumns(state);
  wireHeadings(state);
  wireDetails(state);
  wireGroups(state);
  wireSelection(state);
  wireSearch(state, tools);
  wireCopy(state, tools);
  wireExport(state, tools);
  wireColumnsMenu(state, tools, template);
  wireMore(state, frame);
  if (interactive) wireGrid(state);

  // The stripe is painted from the start, not only after something moves:
  // a table nobody has touched is still a table with rows in it.
  // The stripe is painted from the start, not only after something moves:
  // a table nobody has touched is still a table with rows in it.
  paintZebra(state);
  paintStatus(state);
  // Once, so the empty row is hidden before a reader has searched for
  // anything — it is *nothing matched*, and there are rows.
  applyFilters(state, false);
  measure(state);
  recount(state);
  // THE DISCONNECT IS WRITTEN BESIDE THE OBSERVE, on the signal the listeners
  // take. A second registry of observers is a second list that stops agreeing
  // with the first.
  const watch = new ResizeObserver(() => measure(state));
  watch.observe(scroll ?? table);
  signal.addEventListener("abort", () => watch.disconnect());
  if (tools && tools.querySelector("details, button")) tools.hidden = false;
}

// --- Columns -----------------------------------------------------------------

/** The cells of one column, in every band, from the declarations. */
function cellsOf(state, column) {
  const wanted = [];
  for (const row of state.table.querySelectorAll(".nd-table-row")) {
    if (row.hasAttribute("data-nd-detail-for")) continue;
    const named = row.querySelector(`[data-nd-column="${CSS.escape(column.name)}"]`);
    // The declaration first, the position second — a cell that named its column
    // is a cell whose position a span may have moved.
    wanted.push(named ?? row.children[dataIndex(state, row, column.index)] ?? null);
  }
  return wanted.filter(Boolean);
}

/**
 * Where a column's cell sits in a row, counting the cells the row drew itself.
 *
 * A choice cell and an expand control are cells the component added, and the
 * declarations know nothing about them — so the offset is measured rather than
 * assumed.
 */
function dataIndex(state, row, index) {
  let offset = 0;
  for (const cell of row.children) {
    if (cell.classList.contains("nd-table-select") || cell.classList.contains("nd-table-expand")) {
      offset += 1;
      continue;
    }
    break;
  }
  return offset + index;
}

function wireColumns(state) {
  for (const column of state.columns) {
    // `fit` IS *AS NARROW AS THE CONTENT ALLOWS*, and a `<colgroup>` cannot say
    // so: `white-space` is not one of the four properties that apply to a
    // column. Measured — a `fit` column squeezed *3 Mar* and *R$ 4.900,00* onto
    // two lines each, which is narrower than the content allows and not what
    // the word means.
    if (column.element.getAttribute("data-nd-width") === "fit") {
      const heading = headingOf(state, column);
      for (const cell of [heading, ...cellsOf(state, column)]) {
        // The generated body always writes `data-nd-wrap`, so its presence
        // says nothing; only a caller asking for `truncate` is a decision this
        // must not overrule.
        if (cell && cell.getAttribute("data-nd-wrap") !== "truncate") {
          cell.setAttribute("data-nd-wrap", "nowrap");
        }
      }
    }
    if (column.hidden) paintColumn(state, column, false);
    if (column.pinned) column.element.setAttribute("data-nd-pinned", "true");
  }
  paintPins(state);
}

/** Shows or hides a column: the declaration, the heading and every cell. */
function paintColumn(state, column, shown) {
  column.hidden = !shown;
  column.element.setAttribute("data-nd-hidden", String(!shown));
  const heading = headingOf(state, column);
  if (heading) heading.hidden = !shown;
  for (const cell of cellsOf(state, column)) cell.hidden = !shown;
  paintPins(state);
}

/**
 * How far in each pinned column starts.
 *
 * Measured rather than declared, because a pinned column's width is whatever
 * its content or a reader's resize made it — and written as a custom property
 * so the stylesheet keeps every other decision about a frozen column.
 */
function paintPins(state) {
  let offset = 0;
  for (const column of state.columns) {
    if (!column.pinned || column.hidden) continue;
    const heading = headingOf(state, column);
    const width = heading?.getBoundingClientRect().width ?? 0;
    const at = `${Math.round(offset)}px`;
    for (const cell of [heading, ...cellsOf(state, column)]) {
      if (!cell) continue;
      cell.classList.add("nd-table-pinned");
      // ONLY WHEN IT MOVED. Writing the same value back is a style change the
      // `ResizeObserver` above sees, which measures, which writes — a loop that
      // costs a core and leaves the page permanently unstable. Rounded for the
      // same reason: a third of a pixel of drift is not a move.
      if (cell.style.getPropertyValue("--nd-table-pin") !== at) {
        cell.style.setProperty("--nd-table-pin", at);
      }
    }
    offset += width;
  }
  const total = `${Math.round(offset)}px`;
  if (state.table.style.getPropertyValue("--nd-table-pins") !== total) {
    state.table.style.setProperty("--nd-table-pins", total);
  }
}

function headingOf(state, column) {
  return (
    state.table.querySelector(
      `thead .nd-table-header-cell[data-nd-column="${CSS.escape(column.name)}"]`,
    ) ?? null
  );
}

// --- The heading, and everything drawn into it -------------------------------

function wireHeadings(state) {
  for (const column of state.columns) {
    const heading = headingOf(state, column);
    if (!heading) continue;
    const words = heading.querySelector("[data-nd-table-heading]");
    // The column's name, read BEFORE anything is added to the heading, so a
    // sentence about the column says what is on the screen.
    column.words = words?.textContent.trim() ?? column.name;
    const holder = heading.querySelector(".nd-table-heading");
    if (!holder) continue;

    if (column.sortable) drawSort(state, column, heading, holder, words);
    if (column.resizable) drawResize(state, column, heading, holder);
    if (heading.getAttribute("aria-sort") !== "none" && heading.getAttribute("aria-sort")) {
      state.sorted = column;
      state.direction = heading.getAttribute("aria-sort");
      state.table.setAttribute("data-nd-sorted", column.name);
    }
  }
}

function drawSort(state, column, heading, holder, words) {
  // THE ATTRIBUTE ARRIVES WITH THE CONTROL. The server writes `aria-sort` only
  // where the caller stated a direction, because a component cannot see its
  // column's declaration. Here the column is known to be sortable, so an
  // unsorted one gains its `none` at the moment it gains something to press.
  if (!heading.hasAttribute("aria-sort")) heading.setAttribute("aria-sort", "none");
  const button = document.createElement("button");
  button.type = "button";
  button.className = "nd-table-sort";
  button.dataset.ndTableSort = column.name;
  // The words move INTO the button: a heading beside a button is two things to
  // read, and the words are what a reader presses.
  if (words) button.append(words);
  const glyph = document.createElement("span");
  glyph.className = "nd-table-sort-glyph";
  glyph.setAttribute("aria-hidden", "true");
  button.append(glyph);
  holder.prepend(button);
  button.addEventListener("click", () => sortBy(state, column), { signal: state.signal });
}

function drawResize(state, column, heading, holder) {
  const handle = document.createElement("button");
  handle.type = "button";
  handle.className = "nd-table-resize";
  handle.dataset.ndTableResize = column.name;
  const name = fill(state.at("resize-label"), { column: column.words });
  if (name) {
    handle.setAttribute("aria-label", name);
    handle.title = name;
  }
  heading.append(handle);

  const width = () =>
    column.width ?? (heading.getBoundingClientRect().width / tableWidth(state)) * 100;

  const resize = (percent) => {
    const next = Math.min(90, Math.max(4, percent));
    // ONE COLUMN'S PREFERENCE, and the rest share what is left. Under
    // automatic table layout that is all a width is, which is why nothing here
    // has to take space from a neighbour and nothing can end up narrower than
    // its own content.
    column.width = next;
    if (column.box) column.box.style.width = `${next.toFixed(3)}%`;
    paintPins(state);
    say(
      state.status,
      fill(state.at("resized-label"), {
        column: column.words,
        width: new Intl.NumberFormat(language(), { style: "percent", maximumFractionDigits: 0 }).format(
          next / 100,
        ),
      }),
    );
  };

  handle.addEventListener("keydown", (event) => {
    const by = { ArrowLeft: -STEP, ArrowRight: STEP }[event.key];
    // ALT, not the bare arrows: the bare arrows belong to the grid, and a
    // handle that swallows them is a handle a reader cannot walk past.
    if (by === undefined || !event.altKey) return;
    event.preventDefault();
    event.stopPropagation();
    resize(width() + by * (event.shiftKey ? COARSE : 1));
  }, { signal: state.signal });

  handle.addEventListener("pointerdown", (event) => {
    event.preventDefault();
    handle.setPointerCapture(event.pointerId);
    const from = event.clientX;
    const start = width();
    const move = (moment) => {
      const moved = ((moment.clientX - from) / tableWidth(state)) * 100;
      resize(start + (document.documentElement.dir === "rtl" ? -moved : moved));
    };
    const done = () => {
      handle.removeEventListener("pointermove", move);
      handle.removeEventListener("pointerup", done);
    };
    handle.addEventListener("pointermove", move, { signal: state.signal });
    handle.addEventListener("pointerup", done, { signal: state.signal });
  });
}

function tableWidth(state) {
  return state.table.getBoundingClientRect().width || 1;
}

// --- Sorting -----------------------------------------------------------------

/** A row and the detail row that belongs to it, so a sort keeps them together. */
function groupsOf(body) {
  const groups = [];
  for (const row of body.children) {
    if (row.hasAttribute("data-nd-detail-for") && groups.length > 0) {
      groups.at(-1).push(row);
      continue;
    }
    groups.push([row]);
  }
  return groups;
}

function valueOf(state, row, column) {
  const named = row.querySelector(`[data-nd-column="${CSS.escape(column.name)}"]`);
  const cell = named ?? row.children[dataIndex(state, row, column.index)];
  if (!cell) return "";
  return cell.getAttribute("data-nd-sort-value") ?? cell.textContent.trim();
}

/** Two values, compared the way the column said to compare them. */
function comparator(column) {
  if (column.sortAs === "number") {
    return (a, b) => {
      const left = Number.parseFloat(String(a).replace(/[^\d.,-]/g, "").replace(",", "."));
      const right = Number.parseFloat(String(b).replace(/[^\d.,-]/g, "").replace(",", "."));
      if (Number.isNaN(left) && Number.isNaN(right)) return 0;
      if (Number.isNaN(left)) return 1;
      if (Number.isNaN(right)) return -1;
      return left - right;
    };
  }
  if (column.sortAs === "date") {
    return (a, b) => {
      const left = Date.parse(a);
      const right = Date.parse(b);
      if (Number.isNaN(left) && Number.isNaN(right)) return 0;
      if (Number.isNaN(left)) return 1;
      if (Number.isNaN(right)) return -1;
      return left - right;
    };
  }
  const collator = collatorFor(language());
  return (a, b) => collator.compare(a, b);
}

function sortBy(state, column) {
  const body = state.table.querySelector("tbody");
  if (!body) return;
  // THREE STATES, NOT TWO. A reader who sorted a long table by accident has no
  // other way back to the order it arrived in.
  const next =
    state.sorted !== column ? "ascending" : state.direction === "ascending" ? "descending" : null;

  // ONLY THE HEADINGS THAT CARRY ONE. `aria-sort` describes a column a reader
  // can sort by, so a heading without it is one nobody can — including every
  // `scope="row"` heading naming a record. Writing `none` over the whole head
  // put the attribute on all of them.
  for (const heading of state.table.querySelectorAll("thead .nd-table-header-cell[aria-sort]")) {
    heading.setAttribute("aria-sort", "none");
  }

  const groups = groupsOf(body);
  if (next === null) {
    // Back to the order the document had, from the position each group was
    // rendered in rather than from a copy of the rows.
    groups.sort((a, b) => order(a[0]) - order(b[0]));
    state.sorted = null;
    state.direction = null;
    state.table.removeAttribute("data-nd-sorted");
    say(state.status, fill(state.at("sort-cleared-label"), { column: column.words }));
  } else {
    const compare = comparator(column);
    const sign = next === "ascending" ? 1 : -1;
    groups.sort(
      (a, b) => sign * compare(valueOf(state, a[0], column), valueOf(state, b[0], column)),
    );
    const heading = headingOf(state, column);
    if (heading) heading.setAttribute("aria-sort", next);
    state.sorted = column;
    state.direction = next;
    state.table.setAttribute("data-nd-sorted", column.name);
    say(
      state.status,
      fill(state.at(next === "ascending" ? "sort-ascending-label" : "sort-descending-label"), {
        column: column.words,
      }),
    );
  }

  // One reattachment, not one per row: a fragment is the difference between a
  // sort a reader sees happen and a sort they see flicker.
  const fragment = document.createDocumentFragment();
  for (const group of groups) for (const row of group) fragment.append(row);
  body.append(fragment);
  paintZebra(state);
}

/** The position a row was rendered in, stamped before anything moved. */
function order(row) {
  return Number(row.dataset.ndOrder ?? 0);
}

// --- Filtering, the empty row, the totals and the count ----------------------

function applyFilters(state, announce) {
  const body = state.table.querySelector("tbody");
  if (!body) return;
  const query = state.query ?? "";
  const searched = state.columns.filter((one) => one.filterable);
  let visible = 0;
  let total = 0;

  for (const group of groupsOf(body)) {
    const row = group[0];
    if (row.getAttribute("data-nd-empty") === "true") continue;
    total += 1;
    // ANY searched column, not every one: a reader typing a client's name
    // means *show me the rows where that appears*, wherever it appears.
    const shown =
      query === "" ||
      searched.some((column) =>
        valueOf(state, row, column).toLocaleLowerCase(language()).includes(query),
      );
    for (const one of group) {
      // The detail row stays closed when its own row comes back — a filter is
      // not a reason to open something the reader had shut.
      if (one !== row && one.hidden) continue;
      one.hidden = !shown;
    }
    if (shown) visible += 1;
  }

  state.table.toggleAttribute("data-nd-filtered", query !== "");
  const empty = body.querySelector("[data-nd-empty='true']");
  if (empty) empty.hidden = visible > 0;
  paintZebra(state);
  paintStatus(state);
  recount(state);
  if (announce) {
    say(state.status, fill(state.at("filtered-label"), { count: visible, total }));
  }
}

/**
 * The tone a value earns, written onto every cell of a banded column.
 *
 * A CELL'S OWN TONE ALWAYS WINS. The server has the last word, and a rule that
 * overrode it would be one a caller could not switch off for a single row — so
 * the script marks what it wrote and only ever revises that.
 */
function paintStatus(state) {
  for (const column of state.columns) {
    if (!column.toneBelow && !column.toneWithin && !column.toneAbove) continue;
    for (const cell of cellsOf(state, column)) {
      if (cell.dataset.ndToned === undefined && cell.hasAttribute("data-nd-tone")) continue;
      const reading = number(cell.getAttribute("data-nd-sort-value") ?? cell.textContent);
      const tone =
        reading === null
          ? null
          : column.low !== null && reading < column.low
            ? column.toneBelow
            : column.high !== null && reading > column.high
              ? column.toneAbove
              : column.toneWithin;
      if (tone) {
        cell.setAttribute("data-nd-tone", tone);
        cell.dataset.ndToned = "";
      } else {
        cell.removeAttribute("data-nd-tone");
        delete cell.dataset.ndToned;
      }
    }
  }
}

/**
 * The stripe, counted over the rows a reader can see.
 *
 * `:nth-child(even)` counts rows the filter hid and the detail rows nobody
 * opened, so the stripe alternates in twos and threes the moment anything is
 * hidden. Counting here is the only way it stays a stripe.
 */
function paintZebra(state) {
  let index = 0;
  for (const row of state.table.querySelectorAll("tbody .nd-table-row")) {
    if (row.hidden || row.hasAttribute("data-nd-detail-for")) continue;
    row.setAttribute("data-nd-stripe", index % 2 === 0 ? "even" : "odd");
    index += 1;
  }
}

/** Every total in the foot, and the line that says how much is showing. */
function recount(state) {
  // A GROUP'S HEADING AND A SUMMARY ROW ARE NOT RECORDS. Counting them is how
  // a group of two rows came to report three.
  const rows = [...state.table.querySelectorAll("tbody .nd-table-row")].filter(
    (row) =>
      !row.hasAttribute("data-nd-detail-for") &&
      !row.classList.contains("nd-table-group") &&
      (row.getAttribute("data-nd-emphasis") ?? "none") === "none" &&
      row.getAttribute("data-nd-empty") !== "true",
  );
  const visible = rows.filter((row) => !row.hidden);

  for (const cell of state.table.querySelectorAll("[data-nd-aggregate]")) {
    const kind = cell.getAttribute("data-nd-aggregate");
    if (!kind || kind === "none") continue;
    const column =
      state.columns.find((one) => one.name === cell.getAttribute("data-nd-column")) ??
      // THROUGH `dataIndex`, like every other lookup in this file. Counting the
      // raw children skips the leading cells a row drew itself, so a foot cell
      // with no declared column resolved one column to the left in any table
      // with a choice control in it.
      state.columns[
        [...cell.parentElement.children].indexOf(cell) -
          (dataIndex(state, cell.parentElement, 0) - 0)
      ];
    if (!column) continue;
    // A SECTION'S OWN AGGREGATE COUNTS ITS OWN GROUP. Several `<tbody>`
    // elements are HTML's own row grouping, and a per-group figure that summed
    // the whole table would be the table's figure written once per group.
    const band =
      cell.getAttribute("data-nd-aggregate-over") === "section"
        ? visible.filter((row) => row.closest("tbody") === cell.closest("tbody"))
        : visible;
    const numbers = band.map((row) => number(valueOf(state, row, column))).filter((one) => one !== null);
    const answer = aggregate(kind, numbers);
    if (answer === null) continue;
    const value = cell.querySelector(".nd-table-value") ?? cell;
    // THE SHAPE THE SERVER RENDERED IS KEPT, AND ONLY THE NUMBER IS REPLACED.
    //
    // A currency column's total written as a bare `6.820` reads as a different
    // kind of thing from the `R$ 4.900,00` above it. The cell already carries
    // the caller's own formatting — a symbol, a suffix, a number of decimals —
    // so the run of digits is found and swapped and everything around it
    // survives. No prop for a currency, and no words invented here.
    const shape = value.dataset.ndShape ?? value.textContent;
    value.dataset.ndShape = shape;
    const found = shape.match(/\d[\d.,\s]*/);
    const digits = found?.[0].trimEnd() ?? "";
    const tail = digits.match(/([.,])(\d+)$/);
    const places = tail ? tail[2].length : 0;
    // AND ITS SEPARATORS, not the document's. A column written `R$ 4.900,00`
    // whose total came back `R$ 6,820.00` is a total in a different notation
    // from the numbers it is under — the shape is the caller's, all of it.
    const written = new Intl.NumberFormat("en-US", {
      minimumFractionDigits: places,
      maximumFractionDigits: places,
    })
      .format(answer)
      .split(",")
      .join("\u0000")
      .split(".")
      .join(tail?.[1] ?? ".")
      .split("\u0000")
      .join(digits.replace(/[\d]/g, "").replace(/[\s]/g, "").replace(tail?.[1] ?? "", "") || ",");
    value.textContent = found ? shape.replace(found[0].trimEnd(), written) : written;
  }

  if (!state.summary) return;
  const sentence = state.at("summary-label");
  const hidden = visible.length === rows.length;
  state.summary.hidden = hidden || !sentence;
  if (!hidden && sentence) {
    state.summary.textContent = fill(sentence, { visible: visible.length, total: rows.length });
  }
}

function aggregate(kind, numbers) {
  if (kind === "count") return numbers.length;
  // A SUM OF NOTHING IS ZERO, and saying so matters more than it sounds: a
  // filter that leaves no rows used to leave the previous total standing under
  // an empty table, which is the exact failure `aggregate` exists to prevent.
  // A minimum of nothing is not a number, so those keep the cell as it was.
  if (kind === "sum") return numbers.reduce((total, one) => total + one, 0);
  if (numbers.length === 0) return null;
  if (kind === "average") return numbers.reduce((total, one) => total + one, 0) / numbers.length;
  if (kind === "min") return Math.min(...numbers);
  if (kind === "max") return Math.max(...numbers);
  return null;
}

// --- The detail row ----------------------------------------------------------

function wireDetails(state) {
  for (const detail of state.table.querySelectorAll("[data-nd-detail-for]")) detail.hidden = true;
  state.table.addEventListener("click", (event) => {
    const button = event.target.closest?.(".nd-table-expand-button");
    if (!button) return;
    const detail = document.getElementById(button.getAttribute("aria-controls") ?? "");
    if (!detail) return;
    const open = button.getAttribute("aria-expanded") !== "true";
    button.setAttribute("aria-expanded", String(open));
    detail.hidden = !open;
    paintZebra(state);
  }, { signal: state.signal });
  for (const button of state.table.querySelectorAll(".nd-table-expand-button")) {
    button.setAttribute("aria-expanded", "false");
  }
}

// --- The search ------------------------------------------------------------

/**
 * One box, behind a magnifying glass, over every searched column.
 *
 * The version before this drew a box into every filterable heading: a heading
 * row twice as tall, and a row of controls a reader had to visit one at a time
 * to narrow a table by two things at once.
 */
function wireSearch(state, tools) {
  const holder = tools?.querySelector("[data-nd-table-search]");
  const open = holder?.querySelector(".nd-table-search-open");
  const field = holder?.querySelector(".nd-table-search-field");
  const box = holder?.querySelector("[data-nd-table-search-input]");
  if (!holder || !open || !field || !box) return;
  // Asked for and refused: a search over no column narrows nothing.
  if (!state.columns.some((column) => column.filterable)) {
    holder.remove();
    return;
  }

  const show = (wanted) => {
    field.hidden = !wanted;
    open.setAttribute("aria-expanded", String(wanted));
    holder.toggleAttribute("data-nd-open", wanted);
    if (wanted) box.focus();
  };

  open.addEventListener("click", () => {
    if (field.hidden) {
      show(true);
      return;
    }
    // Closing clears: a glass that hides a box still narrowing the table is a
    // table missing rows for a reason nothing on the screen gives.
    box.value = "";
    state.query = "";
    applyFilters(state, true);
    show(false);
  }, { signal: state.signal });

  let pending = null;
  box.addEventListener("input", () => {
    clearTimeout(pending);
    // One sentence per pause, not one per keystroke: a live region that speaks
    // through typing is a live region a reader turns off.
    pending = setTimeout(() => {
      state.query = box.value.trim().toLocaleLowerCase(language());
      applyFilters(state, true);
    }, SETTLE);
  }, { signal: state.signal });

  box.addEventListener("keydown", (event) => {
    if (event.key !== "Escape") return;
    event.stopPropagation();
    box.value = "";
    state.query = "";
    applyFilters(state, true);
    show(false);
    open.focus();
  }, { signal: state.signal });

  // The pause the filter waits out, dropped with the box that armed it.
  state.signal.addEventListener("abort", () => clearTimeout(pending));
}

// --- Groups that fold -------------------------------------------------------

/**
 * A `<tbody>` that folds behind its own heading row.
 *
 * SEVERAL AT ONCE, AND NONE OF THEM AN ACCORDION. Closing one is never opening
 * another: a reader comparing two groups should not have to keep re-opening the
 * first.
 */
function wireGroups(state) {
  for (const band of state.table.querySelectorAll("tbody[data-nd-collapsible='true']")) {
    const heading = band.querySelector(".nd-table-group-cell");
    const label = heading?.querySelector(".nd-table-group-label");
    if (!heading || !label) continue;
    const rows = [...band.children].filter((row) => !row.classList.contains("nd-table-group"));

    // The words move INTO a real button — a `<th>` with a handler is not
    // focusable, announces nothing actionable and answers no key.
    // ACROSS THE WHOLE TABLE. A lone cell in a row takes one column, not the
    // width — measured: the group's name sat in the first column with the rest
    // of the row blank beside it.
    const wide = Math.max(
      1,
      ...[...state.table.querySelectorAll("tbody .nd-table-row")]
        .filter((row) => !row.classList.contains("nd-table-group"))
        .map((row) => row.children.length),
    );
    heading.colSpan = wide;

    const button = document.createElement("button");
    button.type = "button";
    button.className = "nd-table-group-button";
    button.setAttribute("aria-expanded", "true");
    if (band.id) button.setAttribute("aria-controls", band.id);
    button.append(label);
    const glyph = document.createElement("span");
    glyph.className = "nd-table-group-glyph";
    glyph.setAttribute("aria-hidden", "true");
    button.append(glyph);
    heading.prepend(button);

    button.addEventListener("click", () => {
      const open = button.getAttribute("aria-expanded") !== "true";
      button.setAttribute("aria-expanded", String(open));
      for (const row of rows) row.hidden = !open;
      paintZebra(state);
      recount(state);
    }, { signal: state.signal });
  }
}

// --- Selection ---------------------------------------------------------------

function boxesOf(table) {
  return [...table.querySelectorAll("tbody .nd-table-select input")];
}

function wireSelection(state) {
  const all = state.table.querySelector(".nd-table-select-all input");
  // READ AT THE MOMENT, NOT AT WIRING. Rows arrive from `more_src` after this
  // runs, and a select-all holding a stale list is a control that stops
  // covering the table it belongs to.
  const boxes = boxesOf(state.table);
  if (boxes.length === 0 && !all) return;
  let anchor = null;

  // A CHANGE, NOT AN ARRIVAL. `paint` ends by saying how many rows are chosen,
  // and it runs once at wiring time to draw what the markup already says — so
  // a table with a pre-selected row announced "1 person chosen." to a reader
  // who had done nothing. The contract says the label is what is announced
  // when the chosen rows CHANGE, and the first pass changes nothing.
  const paint = (announce = true) => {
    for (const box of boxes) {
      const row = box.closest(".nd-table-row");
      if (row) row.setAttribute("data-nd-selected", String(box.checked));
    }
    if (all) {
      const chosen = boxes.filter((box) => box.checked).length;
      // The third state the platform has and nothing hand-built remembers.
      all.indeterminate = chosen > 0 && chosen < boxes.length;
      all.checked = chosen === boxes.length && chosen > 0;
    }
    if (!announce) return;
    say(state.status, plural(state.at("selected-label"), boxes.filter((box) => box.checked).length));
  };
  state.paintSelection = paint;

  all?.addEventListener("change", () => {
    for (const box of boxes) {
      if (box.disabled) continue;
      box.checked = all.checked;
    }
    paint();
  }, { signal: state.signal });

  for (const box of boxes) {
    box.addEventListener("click", (event) => {
      // SHIFT EXTENDS. Every table of records a reader has ever used does this,
      // and a table that does not makes them click forty times.
      if (event.shiftKey && anchor !== null) {
        const from = boxes.indexOf(anchor);
        const to = boxes.indexOf(box);
        for (let index = Math.min(from, to); index <= Math.max(from, to); index += 1) {
          if (!boxes[index].disabled) boxes[index].checked = box.checked;
        }
      }
      anchor = box;
      paint();
    }, { signal: state.signal });
    box.addEventListener("change", () => paint(), { signal: state.signal });
  }

  // The select-all box submits nothing: it is a control over the others, and a
  // form that posted it would post a row that does not exist.
  all?.removeAttribute("name");
  // Draws what the markup already says, and says nothing about it.
  paint(false);
}

// --- Copying -----------------------------------------------------------------

/**
 * What is showing, as rows of strings: one reading, four destinations.
 *
 * The chosen rows when there are any, the visible ones when there are not; the
 * columns a reader left visible; the group heading rows and the detail rows
 * left out, because neither is a record.
 */
function table(state) {
  const wanted = [...state.table.querySelectorAll("tbody .nd-table-row")].filter(
    (row) =>
      !row.hidden &&
      !row.classList.contains("nd-table-group") &&
      !row.hasAttribute("data-nd-detail-for") &&
      row.getAttribute("data-nd-empty") !== "true",
  );
  const chosen = wanted.filter((row) => row.getAttribute("data-nd-selected") === "true");
  const rows = chosen.length > 0 ? chosen : wanted;
  const shown = state.columns.filter((column) => !column.hidden);
  state.keys = shown.map((column) => column.name);
  return {
    columns: shown.map((column) => column.words ?? column.name),
    rows: rows.map((row) => shown.map((column) => valueOf(state, row, column).replace(/\s+/g, " "))),
    count: rows.length,
  };
}

/** The same, tab-separated — which is what a spreadsheet pastes as cells. */
function asText(state) {
  const { columns, rows, count } = table(state);
  return { text: [columns.join("\t"), ...rows.map((row) => row.join("\t"))].join("\n"), count };
}

function wireCopy(state, tools) {
  const button = tools?.querySelector(".nd-table-copy");
  if (!button || state.at("copy") !== "true") return;
  button.hidden = false;
  button.addEventListener("click", () => copy(state), { signal: state.signal });
  state.copy = () => copy(state);
}

function copy(state) {
  const { text, count } = asText(state);
  // WRITTEN, NEVER READ — and a failure says nothing rather than claiming a
  // copy that did not happen. `CodeBlock` set that rule.
  navigator.clipboard
    ?.writeText(text)
    .then(() => say(state.status, plural(state.at("copied-label"), count)))
    .catch(() => {});
}

// --- The columns menu --------------------------------------------------------

/**
 * A file, saved, which is the first download this repository has ever written.
 *
 * The same rule the clipboard keeps: a failure says nothing rather than
 * claiming a file that was never written. The object URL is revoked on the next
 * task, because a page that leaks one per export leaks one per export.
 */
function save(name, kind, text) {
  try {
    const blob = new Blob([text], { type: `${kind};charset=utf-8` });
    const url = URL.createObjectURL(blob);
    const link = document.createElement("a");
    link.href = url;
    link.download = name;
    link.click();
    setTimeout(() => URL.revokeObjectURL(url), 0);
    return true;
  } catch {
    return false;
  }
}

/** A field, quoted the way a spreadsheet expects one. */
function csvField(value) {
  return /["\n,]/.test(value) ? `"${value.split('"').join('""')}"` : value;
}

/** What is showing, as the four shapes a reader can take away. */
function wireExport(state, tools) {
  const menu = tools?.querySelector("[data-nd-table-export]");
  if (!menu) return;
  for (const item of menu.querySelectorAll("[data-nd-table-export-as]")) {
    item.addEventListener("click", () => {
      const as = item.getAttribute("data-nd-table-export-as");
      menu.removeAttribute("open");
      if (as === "print") {
        print();
        return;
      }
      if (as === "clipboard") {
        copy(state);
        return;
      }
      const { rows, columns, count } = table(state);
      const file = `${state.at("table") ?? "table"}.${as}`;
      const written =
        as === "csv"
          ? save(
              file,
              "text/csv",
              [columns.map(csvField).join(","), ...rows.map((row) => row.map(csvField).join(","))].join("\r\n"),
            )
          : save(
              file,
              "application/json",
              JSON.stringify(
                rows.map((row) => Object.fromEntries(row.map((value, at) => [state.keys[at], value]))),
                null,
                2,
              ),
            );
      // Nothing at all when it failed, which is the clipboard's rule.
      if (written) say(state.status, plural(state.at("exported-label"), count));
    }, { signal: state.signal });
  }
}

function wireColumnsMenu(state, tools, template) {
  const menu = tools?.querySelector("[data-nd-table-columns]");
  const list = menu?.querySelector("[data-nd-table-columns-list]");
  if (!menu || !list || !template) return;
  const hideable = state.columns.filter((column) => column.hideable);
  if (hideable.length === 0) {
    // Asked for and refused: a menu of nothing is a control that does nothing.
    menu.remove();
    return;
  }
  for (const column of hideable) {
    const row = template.content.firstElementChild.cloneNode(true);
    const box = row.querySelector("[data-nd-table-column-toggle]");
    const name = row.querySelector("[data-nd-table-column-name]");
    box.checked = !column.hidden;
    box.value = column.name;
    if (name) name.textContent = column.words ?? column.name;
    box.addEventListener("change", () => {
      paintColumn(state, column, box.checked);
      recount(state);
      say(
        state.status,
        fill(state.at(box.checked ? "column-shown-label" : "column-hidden-label"), {
          column: column.words ?? column.name,
        }),
      );
    }, { signal: state.signal });
    list.append(row);
  }
}

// --- More rows -------------------------------------------------------------

/**
 * The next rows, fetched as markup rather than as data.
 *
 * `<tr>` MARKUP, NOT JSON. Every component in this family renders its own row,
 * and a schema invented here would be a second description of a table the
 * server already knows how to draw — one that would go stale the first time a
 * cell gained a `Tag`.
 *
 * The button is real and is always there. The observer presses it when it
 * arrives on screen, which is the convenience; without the button a keyboard
 * never reaches the end of the table and the page's footer runs away.
 */
function wireMore(state, frame) {
  const holder = frame?.querySelector("[data-nd-table-more]");
  const button = holder?.querySelector("button");
  const body = state.table.querySelector("tbody:last-of-type");
  if (!holder || !button || !body) return;
  let next = state.at("more-src");
  if (!next) {
    holder.remove();
    return;
  }
  holder.hidden = false;
  let fetching = false;

  async function more() {
    if (fetching || !next) return;
    fetching = true;
    button.disabled = true;
    try {
      // Against `import.meta.url` rather than the page: `dist/` is where this
      // file lives and a consumer may have put the whole directory anywhere.
      const response = await fetch(new URL(next, document.baseURI));
      if (!response.ok) throw new Error(String(response.status));
      const markup = await response.text();
      // Parsed inside a `<template>`, because a `<tr>` handed to `innerHTML` on
      // anything that is not a table is a start tag the parser throws away.
      // Named `parsed`, not `holder`: shadowing the container this function
      // hides at the end is how a table that ran out of rows kept its button.
      const parsed = document.createElement("template");
      parsed.innerHTML = `<table><tbody>${markup}</tbody></table>`;
      const rows = [...(parsed.content.querySelector("tbody")?.children ?? [])];
      // The response says where the rows after these are; no attribute means
      // there are no more, which is how the button knows to go.
      next = rows[0]?.getAttribute("data-nd-more-src") ?? null;
      for (const row of rows) row.removeAttribute("data-nd-more-src");
      const arriving = document.createDocumentFragment();
      for (const [index, row] of rows.entries()) {
        row.dataset.ndOrder = String(body.children.length + index);
        arriving.append(row);
      }
      const added = rows.length;
      body.append(arriving);
      applyFilters(state, false);
      paintZebra(state);
      paintStatus(state);
      recount(state);
      const total = state.at("total") ?? String(body.children.length);
      say(state.status, fill(state.at("more-status-label"), { count: added, total }));
    } catch {
      // A fetch that failed says nothing rather than claiming rows that never
      // arrived — the same rule the clipboard keeps.
    } finally {
      fetching = false;
      button.disabled = false;
      if (!next) holder.hidden = true;
    }
  }

  button.addEventListener("click", more, { signal: state.signal });
  // AND THIS ONE FETCHES. An observer left running on a detached holder asks a
  // server for rows nobody will ever see, once per intersection, for as long as
  // the tab is open.
  const watch = new IntersectionObserver((entries) => {
    if (entries.some((entry) => entry.isIntersecting)) more();
  });
  watch.observe(holder);
  state.signal.addEventListener("abort", () => watch.disconnect());
}

// --- The grid ----------------------------------------------------------------

/** Every cell a reader can walk, row by row, skipping what is hidden. */
/** The control that chooses the row a cell is in, if the row has one. */
function chooserIn(cell) {
  return cell.closest(".nd-table-row")?.querySelector(".nd-table-select input") ?? null;
}

function gridOf(table) {
  return [...table.querySelectorAll(".nd-table-row")]
    .filter((row) => !row.hidden)
    .map((row) => [...row.children].filter((cell) => !cell.hidden));
}

function wireGrid(state) {
  const { table } = state;
  const place = (cell) => {
    for (const other of table.querySelectorAll("[tabindex='0']")) {
      if (other.closest(".nd-table") === table && other.matches("td, th")) {
        other.setAttribute("tabindex", "-1");
      }
    }
    cell.setAttribute("tabindex", "0");
  };

  const paint = () => {
    const cells = gridOf(table);
    for (const row of cells) for (const cell of row) cell.setAttribute("tabindex", "-1");
    // ONE TAB STOP. A grid of four hundred cells is one press to skip, not
    // four hundred — and the stop is wherever the reader last was.
    const first = cells.find((row) => row.length > 0)?.[0];
    if (first) first.setAttribute("tabindex", "0");
  };
  paint();

  table.addEventListener("focusin", (event) => {
    const cell = event.target.closest?.("td, th");
    if (cell && cell.closest(".nd-table") === table) place(cell);
  }, { signal: state.signal });

  table.addEventListener("keydown", (event) => {
    if (event.key === "c" && (event.metaKey || event.ctrlKey) && state.copy) {
      // Only when nothing is selected in the document: a reader who highlighted
      // one number wants that number, not the whole table.
      if ((getSelection()?.toString() ?? "") === "") {
        event.preventDefault();
        state.copy();
      }
      return;
    }
    if (event.key === "a" && (event.metaKey || event.ctrlKey)) {
      const all = table.querySelector(".nd-table-select-all input");
      if (all) {
        event.preventDefault();
        all.checked = true;
        all.dispatchEvent(new Event("change", { bubbles: true }));
      }
      return;
    }
    const cell = event.target.closest?.("td, th");
    if (!cell || cell.closest(".nd-table") !== table) return;
    if (event.altKey) return;

    // ENTER AS WELL AS SPACE. The contract has always named both, and only
    // Space was read — so a reader who pressed the key the rest of the page
    // answers got nothing, on the one control where choosing is the point.
    if ((event.key === " " || event.key === "Enter") && event.target === cell) {
      const box = chooserIn(cell);
      if (box && !box.disabled) {
        event.preventDefault();
        box.checked = !box.checked;
        box.dispatchEvent(new Event("change", { bubbles: true }));
        return;
      }
    }

    const cells = gridOf(table);
    const row = cells.findIndex((one) => one.includes(cell));
    if (row < 0) return;
    const column = cells[row].indexOf(cell);
    const rows = cells.length;
    const screen = Math.max(1, Math.floor((state.scroll?.clientHeight ?? 400) / (cell.getBoundingClientRect().height || 40)));

    const step = {
      ArrowUp: [-1, 0],
      ArrowDown: [1, 0],
      ArrowLeft: [0, -1],
      ArrowRight: [0, 1],
      PageUp: [-screen, 0],
      PageDown: [screen, 0],
    }[event.key];

    let wanted = null;
    if (step) {
      const nextRow = Math.min(rows - 1, Math.max(0, row + step[0]));
      const nextColumn = Math.min(cells[nextRow].length - 1, Math.max(0, column + step[1]));
      wanted = cells[nextRow][nextColumn];
    } else if (event.key === "Home") {
      wanted = event.ctrlKey || event.metaKey ? cells[0][0] : cells[row][0];
    } else if (event.key === "End") {
      wanted = (event.ctrlKey || event.metaKey ? cells.at(-1) : cells[row]).at(-1);
    }
    if (!wanted) return;
    // SHIFT EXTENDS, AND IT NEVER DID. `shiftKey` was read in the resize handle
    // and in a pointer click and nowhere else, so the one row of the contract
    // about choosing several rows from the keyboard reached no code at all.
    // Only up and down extend: a sideways move stays in the row it is in.
    if (step && step[0] !== 0 && event.shiftKey) {
      for (const box of [chooserIn(cell), chooserIn(wanted)]) {
        // A CHECKBOX ONLY. `select: one` draws a radio, and a range of radios
        // is a contradiction rather than a selection.
        if (!box || box.type !== "checkbox" || box.disabled || box.checked) continue;
        box.checked = true;
        box.dispatchEvent(new Event("change", { bubbles: true }));
      }
    }
    event.preventDefault();
    place(wanted);
    wanted.focus();
  }, { signal: state.signal });
}

// --- Measurement -------------------------------------------------------------

/**
 * Whether the box scrolls, and therefore whether it is a way in for a keyboard.
 *
 * SUBTRACTION ONLY. The tab stop and the region role are rendered, and this
 * takes them away from a table that fits — an enhancement may remove a way in
 * and must never be the thing that adds one.
 */
function measure(state) {
  const { scroll, table } = state;
  if (!scroll) return;
  // The same measurement twice is not a change, and acting on it is what makes
  // an observer that watches what it writes never settle.
  const seen = `${scroll.scrollWidth}x${scroll.clientWidth}`;
  if (state.measured === seen) return;
  state.measured = seen;
  const scrolls = scroll.scrollWidth > scroll.clientWidth + 1;
  table.toggleAttribute("data-nd-scrollable", scrolls);
  if (scrolls) {
    scroll.setAttribute("role", "region");
    scroll.setAttribute("tabindex", "0");
  } else {
    scroll.removeAttribute("role");
    scroll.removeAttribute("tabindex");
  }
  paintPins(state);
}
