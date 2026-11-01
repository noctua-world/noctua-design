// The search, which is the one component whose whole subject is behaviour.
//
// Everything else in `spec/components/` can be checked by rendering it. This
// one cannot: the markup it ships is a labelled box and a list of links, and
// every claim worth making about it — that the box and the list are one thing
// in the accessibility tree, that the ranking puts the name above the name that
// merely contains it, that the letters marked are the letters that matched, that
// the count is announced once rather than once per keystroke — is a claim about
// what happens after a key is pressed.
//
// EVERY TEST HERE IS PHRASED AS THE DEFECT IT CATCHES, and each was confirmed by
// putting that defect back. The component this replaces would have failed nine
// of them, and the tenth — the duplicate-id sweep at the bottom — would have
// caught its shipped `id="nd-search-"` the day it landed.

import { test, expect } from "@playwright/test";
import { ANY, LOCALES, OFFERED, PAGE, url, showParts } from "./pages.js";

/** The site's own palette, which is the `dialog` presentation with an index. */
const PALETTE = {
  root: '[data-nd-search="docs"]',
  trigger: '[data-nd-search="docs"] [data-nd-search-open]',
  dialog: "#nd-search-docs-dialog",
  box: "#nd-search-docs",
  list: "#nd-search-docs-results",
  status: "#nd-search-docs-status",
  rows: "#nd-search-docs-results .nd-search-result:not([hidden])",
};

/** The children form on the component's own page: real links, in the document. */
const CHILDREN = {
  root: '[data-nd-search="children"]',
  box: "#nd-search-children",
  status: "#nd-search-children-status",
  rows: '[data-nd-search="children"] .nd-search-result:not([hidden])',
};

/** Opens the palette and waits for the index, which is fetched on first open. */
async function openPalette(page, query = "") {
  await page.locator(PALETTE.trigger).click();
  await expect(page.locator(PALETTE.dialog)).toBeVisible();
  if (query) {
    await page.locator(PALETTE.box).fill(query);
    await expect(page.locator(PALETTE.rows).first()).toBeVisible();
  }
}

/** The visible text of every result row, in the order they are drawn. */
const labels = (page, of) =>
  page.locator(`${of} .nd-search-result-label`).allTextContents();

// --- With no script ---------------------------------------------------------

test.describe("with the script blocked", () => {
  test.use({ javaScriptEnabled: false });

  test("the children form is still a list of links", async ({ page }) => {
    await page.goto(url(PAGE.search));
    await showParts(page);
    const rows = page.locator(CHILDREN.rows);
    expect(await rows.count()).toBeGreaterThan(3);
    // A real `<a href>`, reachable and followable. `role="option"` is written
    // by the script, so with no script there is no listbox to claim.
    const first = rows.first().locator("a");
    await expect(first).toBeVisible();
    expect(await first.getAttribute("href")).toBeTruthy();
    expect(await rows.first().getAttribute("role")).toBeNull();
  });

  test("nothing floats over the page", async ({ page }) => {
    await page.goto(url(PAGE.search));
    await showParts(page);
    // THE DEFECT: `position: fixed` on the popover's panel was unconditional,
    // and `--nd-anchor-block` is written by the script — so with the script
    // blocked the panel took its `100%` fallback against the VIEWPORT and
    // parked a full list of results over the top-left of the page, across the
    // navigation. A popover with nothing to open it is a list in flow.
    for (const panel of await page.locator(".nd-search-panel").all()) {
      expect(await panel.evaluate((el) => getComputedStyle(el).position)).not.toBe("fixed");
    }
  });

  test("the entries form offers nothing rather than a dead box", async ({ page }) => {
    await page.goto(url(ANY));
    await showParts(page);
    // The trigger is hidden until the script reveals it: a control that opens a
    // modal nothing can fill is a button that does nothing.
    await expect(page.locator(PALETTE.trigger)).toBeHidden();
    await expect(page.locator(PALETTE.dialog)).toBeHidden();
  });

  test("no dialog on any page is visible before it is opened", async ({ page }) => {
    // THE DEFECT: `display: flex` on `.nd-search-dialog` overrode the user-agent
    // `dialog:not([open]) { display: none }`, which is the ONLY thing keeping a
    // closed dialog off the page. Every closed palette drew its box over the
    // document, permanently, and no other rule anywhere would have said so.
    for (const page_ of [PAGE.search, ANY]) {
      await page.goto(url(page_));
      await showParts(page);
      for (const dialog of await page.locator("dialog").all()) {
        expect(await dialog.evaluate((el) => el.getBoundingClientRect().height)).toBe(0);
      }
    }
  });
});

// --- The combobox contract --------------------------------------------------

test("the box is a combobox and the list is its listbox", async ({ page }) => {
  await page.goto(url(PAGE.search));
  await showParts(page);
  const box = page.locator(CHILDREN.box);
  await expect(box).toHaveAttribute("role", "combobox");
  await expect(box).toHaveAttribute("aria-autocomplete", "list");

  // `aria-controls` RESOLVES. A box claiming to control a listbox that is not
  // there is the promise the markup does not keep, which is the whole reason
  // every one of these attributes is written by the script and none by the
  // component.
  const controls = await box.getAttribute("aria-controls");
  const list = page.locator(`#${controls}`);
  await expect(list).toHaveAttribute("role", "listbox");

  await expect(box).toHaveAttribute("aria-expanded", "true");
  await box.fill("zzzzqqqq");
  await expect(box).toHaveAttribute("aria-expanded", "false");
});

test("aria-activedescendant names a row that exists", async ({ page }) => {
  await page.goto(url(PAGE.search));
  await showParts(page);
  const box = page.locator(CHILDREN.box);
  // A letter every row in the catalogue carries, so the walk below has
  // somewhere to go on all four presses.
  await box.fill("o");
  await box.press("ArrowDown");

  // THE DEFECT this guards: rebuilding the `<ul>` on every keystroke, which is
  // the commonest way this pattern breaks and is invisible without a screen
  // reader — the attribute keeps naming a node that was thrown away. It is why
  // the entries form draws into a POOL of rows with fixed ids.
  for (let press = 0; press < 4; press += 1) {
    const named = await box.getAttribute("aria-activedescendant");
    expect(named, "nothing is active after an arrow press").toBeTruthy();
    const row = page.locator(`#${named}`);
    await expect(row).toHaveAttribute("role", "option");
    await expect(row).toHaveAttribute("aria-selected", "true");
    await box.press("ArrowDown");
  }

  // Focus never left the box, so a fourth typed letter still reaches the query.
  await box.press("p");
  await expect(box).toBeFocused();
  await expect(box).toHaveValue("op");
});

test("only the active row claims to be selected", async ({ page }) => {
  await page.goto(url(PAGE.search));
  await showParts(page);
  const box = page.locator(CHILDREN.box);
  await box.press("ArrowDown");
  await box.press("ArrowDown");
  // Three hundred `aria-selected="false"` is noise, so absence carries it.
  const selected = page.locator(`${CHILDREN.root} [aria-selected="true"]`);
  await expect(selected).toHaveCount(1);
});

// --- The ranking ------------------------------------------------------------

test("the name outranks the name that merely contains it", async ({ page }) => {
  await page.goto(url(ANY));
  await showParts(page);
  await openPalette(page, "list");
  const found = await labels(page, PALETTE.list);
  expect(found.length).toBeGreaterThan(1);
  // `List` before `ListItem`: an exact match is a tier above a prefix, and no
  // tie-break may lift a candidate out of its tier.
  expect(found[0]).toBe("List");
});

test("a query the old search could not answer is answered", async ({ page }) => {
  await page.goto(url(ANY));
  await showParts(page);
  // `space heading` found NOTHING before: the match was one contiguous
  // substring, and the separator in the query is not in the name.
  await openPalette(page, "space heading");
  const found = await labels(page, PALETTE.list);
  expect(found[0]).toBe("--noctua-space-heading");
});

test("an acronym reaches the name it abbreviates", async ({ page }) => {
  await page.goto(url(ANY));
  await showParts(page);
  await openPalette(page, "bg");
  const found = await labels(page, PALETTE.list);
  expect(found).toContain("ButtonGroup");
});

test("keywords reach a page whose name does not contain them", async ({ page }) => {
  await page.goto(url(PAGE.search));
  await showParts(page);
  const box = page.locator('[data-nd-search="result-keywords"] .nd-search-input');
  await box.fill("slic");
  // Neither the name nor the line under it carries an `l` after an `s`:
  // "Cherokee Purple" has no `s` at all, and `slicing` is in the keywords.
  // That is the only way this row is here.
  const heirloom = page.locator('[data-nd-search="result-keywords"] li:has(a[href*="cherokee"])');
  await expect(heirloom).toBeVisible();

  await box.fill("zzzz");
  await expect(heirloom).toBeHidden();
});

test("a row matched only on its keywords is not highlighted", async ({ page }) => {
  await page.goto(url(PAGE.search));
  await showParts(page);
  const box = page.locator('[data-nd-search="result-keywords"] .nd-search-input');
  await box.fill("slic");
  // The positions index a string that is not on the screen, and a highlight
  // over the wrong letters is worse than no highlight.
  const heirloom = page.locator('[data-nd-search="result-keywords"] li:has(a[href*="cherokee"])');
  await expect(heirloom.locator("[data-nd-hit]")).toHaveCount(0);
  // And a row matched on its NAME is marked, which is what makes the line
  // above a difference between two paths rather than highlighting being off.
  await box.fill("sun");
  const named_ = page.locator('[data-nd-search="result-keywords"] li:has(a[href*="sungold"])');
  // Waited for rather than read straight away: the query is answered a beat
  // after the keystroke, and a marked row read too early is an empty one.
  await expect(named_.locator("[data-nd-hit]").first()).toBeVisible();
  expect((await named_.locator("[data-nd-hit]").allTextContents()).join("")).toBe("Sun");
});

// --- The highlight ----------------------------------------------------------

test("the letters that matched are the letters that are marked", async ({ page }) => {
  await page.goto(url(ANY));
  await showParts(page);
  await openPalette(page, "bg");
  const row = page.locator(PALETTE.rows).first();
  await expect(row.locator(".nd-search-result-label")).toHaveText("ButtonGroup");
  // `NOCTUA.md §12.4`: the matched positions are coloured, never a prefix alone,
  // because a match that cannot show why it matched is confusing. Colouring the
  // prefix would mark `B`
  // alone, and the concatenation would be `B` rather than `BG`.
  const marked = (await row.locator("[data-nd-hit]").allTextContents()).join("");
  expect(marked).toBe("BG");
});

test("a highlight never destroys the label it came from", async ({ page }) => {
  await page.goto(url(PAGE.search));
  await showParts(page);
  const box = page.locator(CHILDREN.box);
  const label = page.locator(`${CHILDREN.rows} .nd-search-result-label`).first();
  const before = await label.innerHTML();

  await box.fill("san");
  await expect(page.locator(`${CHILDREN.root} [data-nd-hit]`).first()).toBeVisible();
  await box.fill("");
  await expect(page.locator(`${CHILDREN.root} [data-nd-hit]`)).toHaveCount(0);

  // Byte-identical. The marks are undone with `normalize()`, which merges the
  // split text nodes back and leaves every element, class and listener alone —
  // and `[data-nd-hit]` rather than `mark` is what makes a caller's own `Mark`
  // inside a label survive the teardown.
  expect(await label.innerHTML()).toBe(before);
});

// --- The keyboard -----------------------------------------------------------

test("the chord opens the palette from anywhere, including a text field", async ({ page }) => {
  await page.goto(url(PAGE.search));
  await showParts(page);
  // From inside another box on purpose: that is the convention a palette is
  // judged against, and the reason this listener does not bail on `isTyping`.
  await page.locator(CHILDREN.box).fill("anything");
  await page.keyboard.press("Control+k");
  await expect(page.locator(PALETTE.dialog)).toBeVisible();
  await expect(page.locator(PALETTE.box)).toBeFocused();
  // And it toggles, so the same keystroke puts it away.
  await page.keyboard.press("Control+k");
  await expect(page.locator(PALETTE.dialog)).toBeHidden();
});

test("slash opens it, and never while something is being typed into", async ({ page }) => {
  await page.goto(url(PAGE.search));
  await showParts(page);
  await page.locator(CHILDREN.box).fill("x");
  await page.locator(CHILDREN.box).press("/");
  await expect(page.locator(PALETTE.dialog)).toBeHidden();
  // A slash typed into a search box is a slash.
  await expect(page.locator(CHILDREN.box)).toHaveValue("x/");

  await page.locator("h1").click();
  await page.keyboard.press("/");
  await expect(page.locator(PALETTE.dialog)).toBeVisible();
});

test("the first Escape clears and the second closes", async ({ page }) => {
  await page.goto(url(ANY));
  await showParts(page);
  await openPalette(page, "button");
  await page.keyboard.press("Escape");
  // The query is what a reader means to abandon first; closing on the first
  // press throws it away along with the panel. Implemented WITH the platform:
  // the `cancel` event's default IS the close, so preventing it is the feature.
  await expect(page.locator(PALETTE.box)).toHaveValue("");
  await expect(page.locator(PALETTE.dialog)).toBeVisible();

  await page.keyboard.press("Escape");
  await expect(page.locator(PALETTE.dialog)).toBeHidden();
});

test("the dialog takes focus and gives it back", async ({ page }) => {
  await page.goto(url(ANY));
  await showParts(page);
  await page.locator(PALETTE.trigger).focus();
  await page.keyboard.press("Enter");
  await expect(page.locator(PALETTE.box)).toBeFocused();
  await page.keyboard.press("Escape");
  // The platform's, not this component's: `showModal()` restores focus.
  await expect(page.locator(PALETTE.trigger)).toBeFocused();
});

test("the page behind the dialog cannot be reached by Tab", async ({ page }) => {
  await page.goto(url(ANY));
  await showParts(page);
  await openPalette(page);
  // Which proves nobody hand-rolled a focus trap fighting the browser's own.
  // `shell.rs` says "nothing here is modal" about the DRAWER; this is the one
  // documented exception, and it is the platform's modality rather than ours.
  //
  // "Outside the dialog", not "inside it": the cycle passes through `<body>` on
  // its way round, which is the browser handing focus to its own chrome and
  // back. What must never happen is focus landing on a link or a control in
  // the page behind — and that is what this asks.
  for (let press = 0; press < 8; press += 1) {
    await page.keyboard.press("Tab");
    const landed = await page.evaluate((selector) => {
      const active = document.activeElement;
      if (!active || active === document.body || active === document.documentElement) return null;
      if (document.querySelector(selector).contains(active)) return null;
      return `${active.tagName}.${active.className}`;
    }, PALETTE.dialog);
    expect(landed, `focus reached the page behind after ${press + 1} tabs`).toBeNull();
  }
  await expect(page.locator(PALETTE.dialog)).toBeVisible();
});

// --- What it says -----------------------------------------------------------

test("the count is announced once, not once per letter", async ({ page }) => {
  await page.goto(url(ANY));
  await showParts(page);
  await openPalette(page);
  await page.evaluate((selector) => {
    window.__writes = 0;
    new MutationObserver(() => {
      window.__writes += 1;
    }).observe(document.querySelector(selector), { childList: true, characterData: true, subtree: true });
  }, PALETTE.status);

  // Eight keystrokes. A live region written on every `input` event queues an
  // utterance per character, five of which are stale before they are spoken.
  for (const key of "buttongr") await page.locator(PALETTE.box).press(key);
  await page.waitForTimeout(900);
  const writes = await page.evaluate(() => window.__writes);
  expect(writes, `${writes} writes over eight keystrokes`).toBeLessThanOrEqual(3);
  await expect(page.locator(PALETTE.status)).not.toHaveText("");
});

test("the sentence agrees with the number in it", async ({ page }) => {
  // THE DEFECT: `{count} results` printed `1 results`, and `1 resultados` in
  // Portuguese — the exact failure the `status` prop's own contract names.
  await page.goto(url(ANY));
  await showParts(page);
  await openPalette(page, "searchable");
  await expect(page.locator(PALETTE.status)).toHaveText("1 result");

  await page.goto(url(ANY, "pt-BR"));
  await showParts(page);
  await openPalette(page, "searchable");
  await expect(page.locator(PALETTE.status)).toHaveText("1 resultado");
});

test("a capped list says how many it is not showing", async ({ page }) => {
  await page.goto(url(PAGE.search));
  await showParts(page);
  const box = page.locator('[data-nd-search="popover"] .nd-search-input');
  // A letter every variety in the catalogue carries, so there are six answers
  // and the cap has something to leave out.
  await box.fill("o");
  // `cap: "3"`. Without the sentence a capped search silently claims there were
  // three answers when there were five, which is the quietest way a search lies.
  const rows = page.locator('[data-nd-search="popover"] .nd-search-result:not([hidden])');
  await expect(rows).toHaveCount(3);
  await expect(page.locator("#nd-search-popover-status")).toContainText("Showing 3 of 6");
});

test("nothing matched is an answer, not silence", async ({ page }) => {
  await page.goto(url(PAGE.search));
  await showParts(page);
  await page.locator(CHILDREN.box).fill("zzzzqqqq");
  await expect(page.locator(CHILDREN.status)).toHaveText("Nothing matches that.");
});

test("a list still coming says so instead of showing nothing", async ({ page }) => {
  await page.goto(url(PAGE.search));
  await showParts(page);
  // THE DEFECT: `data-nd-loading` was stamped by the component, documented in
  // the contract and read by nothing — so `loading: true` rendered a box
  // indistinguishable from one whose results had all arrived.
  await expect(page.locator("#nd-search-loading-status")).toHaveText("Still loading.");
  const field = page.locator('[data-nd-search="loading"] .nd-search-field');
  expect(await field.evaluate((el) => getComputedStyle(el).cursor)).toBe("progress");
});

test("a refused box looks refused", async ({ page }) => {
  await page.goto(url(PAGE.search));
  await showParts(page);
  const box = page.locator("#nd-search-refused");
  // The real attribute rather than `aria-disabled`, because only the real one
  // stops a form submitting — and it was invisible until the field was dimmed.
  await expect(box).toBeDisabled();
  const field = page.locator('[data-nd-search="refused"] .nd-search-field');
  expect(Number(await field.evaluate((el) => getComputedStyle(el).opacity))).toBeLessThan(1);
});

// --- Filtering, and what it must not touch ----------------------------------

test("filtering reaches the page around the box and stops there", async ({ page }) => {
  await page.goto(url(PAGE.search));
  await showParts(page);
  const rows = page.locator(".nd-example-filtered li");
  await page.locator('[data-nd-search="filter"] .nd-search-input').fill("san");
  await expect(rows.filter({ visible: true })).toHaveCount(1);
  await expect(rows.filter({ visible: true }).first()).toHaveText("San Marzano");

  // And the navigation did not move. Filtering the navigation removes the rows
  // a reader is navigating BY, which is why the site's search became a palette.
  const links = page.locator(".nd-nav-link");
  expect(await links.count()).toBe(await links.filter({ visible: true }).count());
});

test("typing in the palette does not move the navigation", async ({ page }) => {
  await page.goto(url(ANY));
  await showParts(page);
  const before = await page.locator(".nd-nav-link").count();
  await openPalette(page, "button");
  expect(await page.locator(".nd-nav-link").count()).toBe(before);
});

test("the trigger's space is reserved before the trigger arrives", async ({ page }) => {
  // THE DEFECT: the trigger ships `hidden` and the script unhides it, so on
  // every page load the navigation dropped by the height of a control — 0.0042
  // of layout shift, which `NOCTUA.md §9.6` forbids and `theme.spec.js` caught
  // only when the sequence it ran happened to be slow enough.
  //
  // MEASURED, NOT TIMED, and that is the point. A `layout-shift` observer sees
  // this only when the script lands after the first paint, so on a fast machine
  // serving local files the same defect reads as green. What is claimed is that
  // the box exists before the button does — so the box is measured without the
  // button, and the button is measured, and they have to be the same size.
  //
  // `shell.js` is blocked rather than all scripting: the inline head script has
  // to run, because `data-nd-scripting` is what says a trigger is coming. That
  // is exactly the state a real page is in between paint and enhancement.
  await page.route("**/assets/shell.js", (route) => route.abort());
  await page.goto(url(ANY));
  await showParts(page);
  await expect(page.locator(PALETTE.trigger)).toBeHidden();
  const reserved = (await page.locator(PALETTE.root).boundingBox()).height;

  await page.unroute("**/assets/shell.js");
  await page.goto(url(ANY));
  await showParts(page);
  await expect(page.locator(PALETTE.trigger)).toBeVisible();
  const arrived = (await page.locator(PALETTE.trigger).boundingBox()).height;

  expect(arrived, "the trigger has no height to reserve").toBeGreaterThan(0);
  expect(reserved, "the slot is not the size of what fills it").toBe(arrived);
});

// --- Recents ----------------------------------------------------------------

test("a hostile stored value does not break the box", async ({ page }) => {
  await page.goto(url(ANY));
  await showParts(page);
  for (const hostile of ['"not an array"', '{"a":1}', "not json at all", JSON.stringify(
    Array.from({ length: 10_000 }, (_, i) => `query ${i}`),
  )]) {
    await page.evaluate((value) => localStorage.setItem("noctua.docs.recent", value), hostile);
    await page.reload();
    await openPalette(page);
    // Whatever was in there, the box still searches.
    await page.locator(PALETTE.box).fill("button");
    await expect(page.locator(PALETTE.rows).first()).toBeVisible();
    await page.keyboard.press("Escape");
    await page.keyboard.press("Escape");
  }
});

test("what it remembers can be forgotten, in the place it is shown", async ({ page }) => {
  await page.goto(url(ANY));
  await showParts(page);
  await openPalette(page, "button");
  await page.locator(PALETTE.rows).first().locator("a").click();
  await page.waitForLoadState("load");

  await openPalette(page);
  const recent = page.locator("#nd-search-docs-recent");
  await expect(recent).toBeVisible();
  await expect(recent).toContainText("button");

  await page.locator("[data-nd-search-forget]").click();
  await expect(recent).toBeHidden();
});

// --- Across every page and locale -------------------------------------------

test("no id appears twice on any page, in any locale", async ({ page }) => {
  // TEN LINES THAT CATCH THIS ACROSS EVERY COMPONENT, and they would have
  // caught the shipped defect the day it landed: the search this replaces built
  // its id from an OPTIONAL name, so the four examples on its own page all
  // carried `id="nd-search-"` and four `<label for>` pointed at whichever the
  // browser resolved first. `name` is required now, which is the fix; this is
  // the guard that says so.
  for (const locale of LOCALES) {
    for (const slug of Object.values(PAGE)) {
      await page.goto(url(slug, locale));
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
      expect(duplicates, `${locale}/${slug}`).toEqual([]);
    }
  }
});

test("every offered locale searches in its own words", async ({ page }) => {
  for (const locale of OFFERED) {
    await page.goto(url(ANY, locale));
    await showParts(page);
    await openPalette(page, "button");
    await expect(page.locator(PALETTE.rows).first()).toBeVisible();
    // The index is per locale, so a Portuguese reader gets Portuguese titles.
    const status = await page.locator(PALETTE.status).textContent();
    expect(status.trim(), `${locale} said nothing`).not.toBe("");
  }
});

// --- An example is not the page it sits on ----------------------------------
//
// EVERY ROW THESE EXAMPLES DREW WAS A REAL PAGE OF THIS SITE: `Button`,
// `Checkbox`, *Spacing*, *Typography*, and `/search/` itself marked `current`.
// A reader could not tell the demonstration from the application, which is the
// one thing an example has to make obvious. They are a seed catalogue now,
// invented, and this is what keeps them that way.

test("the examples search an invented catalogue rather than this site", async ({ page }) => {
  await page.goto(url(PAGE.search));
  await showParts(page);

  // What is drawn on the page, which is every example except the palette: its
  // index is fetched, and `src` is the one prop whose subject is fetching.
  const labels = await page
    .locator(".nd-search-result-label")
    .evaluateAll((all) => all.map((one) => one.textContent.trim()));
  expect(labels.length, "the page drew no results at all").toBeGreaterThan(10);

  // The site's own names, which are what these rows used to be. `Table` and
  // `Switch` are ordinary words, so each is compared whole rather than by
  // substring, and the two that are not — a page title in the reader's
  // language — are checked from the sidebar instead.
  const OURS = ["Button", "ButtonGroup", "Checkbox", "Segmented", "Select", "Switch", "Table"];
  for (const label of labels) {
    expect(OURS, `an example still lists this site's own components: ${label}`).not.toContain(label);
  }

  const titles = await page
    .locator(".nd-nav a")
    .evaluateAll((all) => all.map((one) => one.textContent.trim()).filter(Boolean));
  for (const label of labels) {
    expect(titles, `an example still lists this site's own pages: ${label}`).not.toContain(label);
  }

  // AND THE CATALOGUE IS ACTUALLY THERE, because every check above holds for a
  // page that drew nothing at all.
  expect(labels, "the invented catalogue is missing").toContain("Cherokee Purple");
});

test("the invented rows are the same rows in every locale, and the sentences are not", async ({
  page,
}) => {
  const seen = [];
  for (const locale of OFFERED) {
    await page.goto(url(PAGE.search, locale));
    await showParts(page);
    // A VARIETY IS A PROPER NAME AND CROSSES UNCHANGED, which is what lets a
    // query written into an example match in every language. The line under it
    // is a sentence and goes through the catalogue, so the two must differ.
    const row = page.locator('[data-nd-search="children"] .nd-search-result').first();
    expect(await row.locator(".nd-search-result-label").textContent()).toBe("Cherokee Purple");
    seen.push((await row.locator(".nd-search-result-detail").textContent()).trim());
  }
  expect(new Set(seen).size, "the detail line is the same in both languages").toBe(seen.length);
});
