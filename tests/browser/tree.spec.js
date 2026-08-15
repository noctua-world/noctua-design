// The hierarchy, and the thing it is careful not to claim.
//
// `Tree` is the component in this system whose contract is mostly about what
// the markup does NOT say: no `role="tree"`, no `aria-expanded`, no `tabindex`
// and nothing hidden until a script is there to answer for them. A tree that
// claimed the role and answered no keys would be the promise the markup does
// not keep that `Segmented` names and `Table` avoids by refusing `role="grid"`.
//
// So the first block here runs with no script at all, and the rest is the
// keyboard — because a tree is a keyboard.

import { test, expect } from "@playwright/test";
import { LOCALES, PAGE, url, showParts } from "./pages.js";

const T = '[data-nd-tree="pages"]';
const CHOSEN = '[data-nd-tree="chosen"]';

/** The name of whatever has focus. */
const focused = (page) =>
  page.evaluate(() => {
    const active = document.activeElement;
    return active?.querySelector?.(".nd-tree-label")?.textContent ?? active?.tagName ?? null;
  });

/** Puts focus on the tree's first node. */
async function enter(page) {
  await page.locator(`${T} .nd-tree-item`).first().scrollIntoViewIfNeeded();
  await page.locator(`${T} .nd-tree-item`).first().focus();
}

test.describe("with the script blocked", () => {
  test.use({ javaScriptEnabled: false });

  test("it is a nested list of links and claims nothing", async ({ page }) => {
    await page.goto(url(PAGE.tree));
    await showParts(page);
    // No role anywhere. A `role="tree"` with no keyboard is a promise the
    // markup does not keep.
    // Scoped to the trees: the example frame's own toolbar carries a
    // `role="group"` of its own, which is not this component's doing.
    await expect(page.locator('[role="tree"]')).toHaveCount(0);
    await expect(page.locator('[role="treeitem"]')).toHaveCount(0);
    await expect(page.locator('.nd-tree [role="group"]')).toHaveCount(0);
    await expect(page.locator(".nd-tree [aria-expanded]")).toHaveCount(0);

    // What is there is a nested list, and its links work.
    const nested = page.locator(`${T} .nd-tree-item .nd-tree-group .nd-tree-item`);
    expect(await nested.count()).toBeGreaterThan(0);
    expect(await page.locator(`${T} a[href]`).count()).toBeGreaterThan(0);
  });

  test("nothing is folded away behind a control that cannot open it", async ({ page }) => {
    await page.goto(url(PAGE.tree));
    await showParts(page);
    // THE RULE `CodeBlock { max_lines }` STATES: an enhancement that hides
    // content when it fails is a bug. The collapse is keyed on the attribute
    // the inline bootstrap stamps, which never appears without a script — so
    // every node is visible and every toggle stays hidden.
    for (const node of await page.locator(`${T} .nd-tree-item`).all()) {
      await expect(node).toBeVisible();
    }
    await expect(page.locator(`${T} [data-nd-tree-toggle]:not([hidden])`)).toHaveCount(0);
  });
});

test("the script writes every role, and the row keeps its link", async ({ page }) => {
  await page.goto(url(PAGE.tree));
  await showParts(page);
  await expect(page.locator(T)).toHaveAttribute("role", "tree");
  const nodes = page.locator(`${T} .nd-tree-item`);
  expect(await nodes.count()).toBeGreaterThan(3);
  for (const node of await nodes.all()) {
    await expect(node).toHaveAttribute("role", "treeitem");
  }
  // `role="treeitem"` ON AN `<a>` replaces *link* with *tree item* and takes
  // the link's own behaviours with it. So the role is on the `<li>` and the
  // anchor inside is untouched — middle-click, the context menu and *copy link
  // address* all still work.
  const link = page.locator(`${T} .nd-tree-body[href]`).first();
  expect(await link.evaluate((el) => el.tagName)).toBe("A");
  expect(await link.getAttribute("role")).toBeNull();
});

test("a leaf has no expand state and no control", async ({ page }) => {
  await page.goto(url(PAGE.tree));
  await showParts(page);
  // `aria-expanded` on a node with nothing under it announces a control that
  // is not there — the same reason `TableHeaderCell` writes `aria-sort` only
  // on a sortable column.
  for (const leaf of await page.locator(`${T} .nd-tree-item[data-nd-leaf="true"]`).all()) {
    expect(await leaf.getAttribute("aria-expanded")).toBeNull();
    await expect(leaf.locator(":scope > .nd-tree-row > [data-nd-tree-toggle]")).toBeHidden();
  }
});

test("the whole tree is one tab stop", async ({ page }) => {
  await page.goto(url(PAGE.tree));
  await showParts(page);
  // Forty rows would otherwise be forty presses to cross — the trade `Toolbar`
  // already makes, and the reason the arrows do the walking.
  const stops = await page.locator(`${T} .nd-tree-item`).evaluateAll((els) =>
    els.filter((el) => el.tabIndex === 0).length,
  );
  expect(stops).toBe(1);
});

test("the arrows walk what a reader can see, across levels", async ({ page }) => {
  await page.goto(url(PAGE.tree));
  await showParts(page);
  await enter(page);
  const walk = [];
  for (let step = 0; step < 4; step += 1) {
    await page.keyboard.press("ArrowDown");
    walk.push(await focused(page));
  }
  // The next ROW, not the next sibling — an open branch's children come first.
  expect(walk.length).toBe(4);
  expect(new Set(walk).size, `it stopped moving: ${walk.join(" ")}`).toBe(4);
});

test("left closes, then climbs; right opens, then steps in", async ({ page }) => {
  await page.goto(url(PAGE.tree));
  await showParts(page);
  await enter(page);
  const first = page.locator(`${T} .nd-tree-item`).first();
  await expect(first).toHaveAttribute("aria-expanded", "true");

  await page.keyboard.press("ArrowLeft");
  await expect(first).toHaveAttribute("aria-expanded", "false");
  // A closed branch's children are gone from the walk, so Down is now the next
  // sibling rather than the first child.
  await page.keyboard.press("ArrowRight");
  await expect(first).toHaveAttribute("aria-expanded", "true");
  await page.keyboard.press("ArrowRight");
  // Stepped in.
  const parent = await page.evaluate(
    (selector) => document.activeElement.parentElement.closest(selector) !== null,
    `${T} .nd-tree-item`,
  );
  expect(parent).toBe(true);
});

test("a collapse never leaves focus on something it just hid", async ({ page }) => {
  await page.goto(url(PAGE.tree));
  await showParts(page);
  await enter(page);
  await page.keyboard.press("ArrowDown"); // into the branch
  const inside = await focused(page);
  await page.keyboard.press("ArrowLeft"); // no children: climb to the parent
  expect(await focused(page)).not.toBe(inside);
  await page.keyboard.press("ArrowLeft"); // now close it
  // Focus is on the branch, which is still visible. Hiding first would blur to
  // `<body>` and a screen reader would restart at the top of the document.
  const landed = await page.evaluate(() => {
    const active = document.activeElement;
    return active !== document.body && active.offsetParent !== null;
  });
  expect(landed).toBe(true);
});

test("an asterisk opens the level and no deeper", async ({ page }) => {
  await page.goto(url(PAGE.tree));
  await showParts(page);
  await enter(page);
  await page.keyboard.press("ArrowLeft"); // close the first branch
  await page.keyboard.press("*");
  const top = await page.locator(`${T} > .nd-tree-item`).evaluateAll((els) =>
    els.filter((el) => el.dataset.ndLeaf === "false").map((el) => el.getAttribute("aria-expanded")),
  );
  expect(top.every((state) => state === "true")).toBe(true);
});

test("typing jumps, and the slash key does not steal it", async ({ page }) => {
  await page.goto(url(PAGE.tree));
  await showParts(page);
  await enter(page);
  await page.keyboard.press("c");
  expect((await focused(page)).toLowerCase().startsWith("c")).toBe(true);

  // THE DEFECT: `isTyping` knew only about `input`, `textarea`, `select` and
  // `contenteditable`, and the hotkey listener is registered before the tree's
  // — so `/` opened the search palette out from under a reader who was typing
  // a node's name to jump to it. The tree says it reads letters, with
  // `data-nd-typeahead`, and the hotkey steps aside.
  await page.keyboard.press("/");
  await expect(page.locator("#nd-search-docs-dialog")).toBeHidden();
});

test("selection is stored once, in a real checkbox", async ({ page }) => {
  await page.goto(url(PAGE.tree));
  await showParts(page);
  const rows = page.locator(`${CHOSEN} .nd-tree-body`);
  await rows.first().scrollIntoViewIfNeeded();
  await rows.nth(1).click();
  await rows.nth(3).click({ modifiers: ["Shift"] });

  // `Table`'s ruling: selection is a real checkbox, whose state the browser
  // announces without being asked and which survives a form post with no
  // script. `aria-selected` is written FROM it at the one place selection
  // changes, so the two cannot come apart — the "two facts kept in step" trap
  // `TableRow` warns about, avoided by having one fact.
  const both = await page.evaluate((selector) => {
    const tree = document.querySelector(selector);
    const nodes = [...tree.querySelectorAll(".nd-tree-item")];
    return nodes.map((node) => {
      const box = node.querySelector(':scope > .nd-tree-row > input[type="checkbox"]');
      return `${node.getAttribute("aria-selected")}/${box ? box.checked : "none"}`;
    });
  }, CHOSEN);
  expect(both.filter((pair) => pair === "true/true").length).toBe(3);
  expect(both.some((pair) => pair === "true/false" || pair === "false/true")).toBe(false);

  // And a form gets the values, not the labels.
  const arrived = await page.evaluate((selector) => {
    const form = document.querySelector(selector).closest("form");
    return [...new FormData(form)].map(([k, v]) => `${k}=${v}`);
  }, CHOSEN);
  expect(arrived.length).toBe(3);
  expect(arrived.every((entry) => entry.startsWith("page="))).toBe(true);
});

test("select-all belongs to a multiple tree and nobody else", async ({ page }) => {
  await page.goto(url(PAGE.tree));
  await showParts(page);
  // On a tree with no selection this stays the browser's own *select all*.
  await enter(page);
  await page.keyboard.press("Control+a");
  await expect(page.locator(`${T} [aria-selected="true"]`)).toHaveCount(0);

  await page.locator(`${CHOSEN} .nd-tree-item`).first().scrollIntoViewIfNeeded();
  await page.locator(`${CHOSEN} .nd-tree-item`).first().focus();
  await page.keyboard.press("Control+a");
  const chosen = await page.locator(`${CHOSEN} [aria-selected="true"]`).count();
  expect(chosen).toBeGreaterThan(3);
});

test("a refused node keeps its place in the walk", async ({ page }) => {
  await page.goto(url(PAGE.tree));
  await showParts(page);
  const refused = page.locator('.nd-tree-item[aria-disabled="true"]').first();
  await expect(refused).toHaveCount(1);
  // Removing it would leave a gap a reader arrows into and cannot explain.
  await expect(refused).toBeVisible();
  await expect(refused).toHaveAttribute("role", "treeitem");
});

test("no id appears twice on the tree page, in any locale", async ({ page }) => {
  for (const locale of LOCALES) {
    await page.goto(url(PAGE.tree, locale));
    await showParts(page);
    const twice = await page.evaluate(() => {
      const seen = new Set();
      const out = [];
      for (const el of document.querySelectorAll("[id]")) {
        if (seen.has(el.id)) out.push(el.id);
        seen.add(el.id);
      }
      return out;
    });
    expect(twice, locale).toEqual([]);
  }
});
