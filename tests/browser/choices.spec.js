// The choice controls, measured rather than described.
//
// Two claims here are the whole reason `Checkbox` and `Radio` exist at all, and
// both are the kind a stylesheet can silently stop honouring: the drawn box is
// 40, 32 or 24 pixels, and the thing a pointer has to hit is 44 at every one of
// them. Asserting those numbers from the
// stylesheet would only prove the stylesheet says what it says, so everything
// below comes off the rendered page.
//
// The other half is progressive enhancement. The exclusive option and the
// conditional reveal are behaviour, so they live in `shell.js` — and a
// behaviour that has to run for the page to make sense is a page that breaks
// when it does not. Every one of them is checked with the script blocked too.
//
// WHICH PAGE EACH TEST OPENS IS PART OF THE TEST. These six controls were on
// one `/forms` page and are now on six, so a test that measures a fieldset
// opens `/fieldset/` and one that measures a box opens `/checkbox/`. Landing on
// a page that happens not to render the thing under test is how a suite goes
// green by measuring nothing.

import { test, expect } from "@playwright/test";
import { PAGE, showParts, url } from "./pages.js";
import { contrast, extremes } from "./pixels.js";

/** The controls the page renders, as against the specimens in the variant grid. */
const REAL = ".nd-choice:has(.nd-choice-input)";

/** The box, the target, and the offset between their centres. */
async function geometry(page, size) {
  return page.evaluate((wanted) => {
    const row = [...document.querySelectorAll(`.nd-choice[data-nd-size="${wanted}"]`)].find(
      (choice) => choice.querySelector(".nd-choice-input"),
    );
    if (!row) return null;
    const box = row.querySelector(".nd-choice-box").getBoundingClientRect();
    const input = row.querySelector(".nd-choice-input").getBoundingClientRect();
    return {
      box: [box.width, box.height],
      target: [input.width, input.height],
      offset: [
        box.left + box.width / 2 - (input.left + input.width / 2),
        box.top + box.height / 2 - (input.top + input.height / 2),
      ],
    };
  }, size);
}

test("the box is 40, 32 or 24 and the target is 44 at all three", async ({ page }) => {
  await page.goto(url(PAGE.checkbox));

  // THE NUMBERS, NOT THE TOKENS THAT COMPOSE THEM.
  //
  // `calc(var(--noctua-space-8) + var(--noctua-space-2))` is 40px because the
  // scale says so, and a scale that moved would move this with it — silently,
  // and away from the size the research was done at. This is the guard on that.
  for (const [size, side] of [
    ["large", 40],
    ["medium", 32],
    ["small", 24],
  ]) {
    const measured = await geometry(page, size);
    expect(measured, `no rendered ${size} choice`).not.toBeNull();
    expect(measured.box, `the ${size} box`).toEqual([side, side]);
    // WCAG 2.5.8 AAA, and it does not shrink with the drawing: a smaller box is
    // a smaller picture and never a smaller thing to press.
    expect(measured.target, `the ${size} target`).toEqual([44, 44]);
    // And it is centred on the box rather than hanging off one corner, which is
    // where the 2px of overhang would be useless.
    expect(Math.abs(measured.offset[0]), `the ${size} target is off-centre`).toBeLessThan(0.5);
    expect(Math.abs(measured.offset[1]), `the ${size} target is off-centre`).toBeLessThan(0.5);
  }
});

test("the label is part of the target and the box is not in the way", async ({ page }) => {
  await page.goto(url(PAGE.checkbox));
  const input = page.locator("#nd-apply-study");
  await expect(input).not.toBeChecked();

  // Clicking the WORDS checks it, which is what `<label for>` buys and what a
  // hand-built row of a span beside an input does not.
  await page.locator('label[for="nd-apply-study"]').click();
  await expect(input).toBeChecked();

  // And a point in the middle of the drawn box reaches the input rather than
  // the drawing. Two things make that true and the invariant is what is
  // asserted, not either of them: the box takes no pointer events, and the
  // input is positioned so it paints above a box that is not. Removing the
  // first alone does not break this — the second still holds — which is why
  // the check is written against what a pointer does rather than against a
  // property that turns out to be the belt beside a brace.
  const hit = await page.evaluate(() => {
    const row = document.querySelector("#nd-apply-study").closest(".nd-choice");
    const box = row.querySelector(".nd-choice-box").getBoundingClientRect();
    const at = document.elementFromPoint(box.left + box.width / 2, box.top + box.height / 2);
    return at?.className ?? null;
  });
  expect(hit, "the drawing swallowed the click").toBe("nd-choice-input");
});

test("the state a reader sees comes off the real input", async ({ page }) => {
  await page.goto(url(PAGE.checkbox));

  // The mark is drawn from `:checked` on the input, so there is one fact and
  // not two kept in step. Toggling the input from script — never touching a
  // class — has to move the drawing.
  const mark = page.locator("#nd-apply-study").locator("xpath=../span/*");
  expect(await mark.evaluate((el) => getComputedStyle(el).opacity)).toBe("0");
  await page.locator("#nd-apply-study").check();
  expect(await mark.evaluate((el) => getComputedStyle(el).opacity)).toBe("1");
});

/**
 * The mark inside a checked box, against the box it sits in.
 *
 * Read from PIXELS. `getComputedStyle().color` returns whatever the engine
 * serialised, and the mark here is an antialiased glyph over a filled box — the
 * only honest question is what the two ended up looking like.
 */
async function mark(page, id) {
  const row = page.locator(id).locator("xpath=..");
  // `clip` is against the VIEWPORT image, so a control further down the page
  // than the window is tall clips to nothing at all — which fails with
  // "clipped area is outside the resulting image" rather than with anything
  // about contrast.
  await row.scrollIntoViewIfNeeded();
  const box = await row.locator(".nd-choice-box").boundingBox();
  // INSIDE the border, not across it. The border is the accent on a checked
  // control either way, so a clip that included it would measure accent
  // against accent and pass for the wrong reason. Four pixels clears a
  // two-pixel border and its antialiasing.
  const shot = await page.screenshot({
    clip: { x: box.x + 4, y: box.y + 4, width: box.width - 8, height: box.height - 8 },
  });
  // The extremes rather than a named point, because a point at the centre of a
  // tick lands in the gap between its two strokes.
  const { min, max } = await extremes(page, shot);
  return contrast(min, max);
}

test("the tick is legible on the box it sits in", async ({ page }) => {
  await page.goto(url(PAGE.checkbox));
  await page.locator("#nd-apply-study").check();
  expect(await mark(page, "#nd-apply-study"), "the checkbox tick against its box").toBeGreaterThan(
    3,
  );
});

test("the dot is legible on the box it sits in", async ({ page }) => {
  // Two tests where there was one. They made the same claim about two
  // components that were on one page and are now on two, and a single test
  // would have had to visit both — which is a test that fails for whichever
  // page it happens to be looking at.
  await page.goto(url(PAGE.radio));
  expect(await mark(page, "#nd-contact-email"), "the radio dot against its box").toBeGreaterThan(3);
});

test("the exclusive option clears the others, and they clear it", async ({ page }) => {
  await page.goto(url(PAGE.checkbox));
  const study = page.locator("#nd-apply-none-study");
  const work = page.locator("#nd-apply-none-work");
  const none = page.locator("#nd-apply-none-none");

  await study.check();
  await work.check();
  await none.check();
  await expect(study, "the exclusive option left another checked").not.toBeChecked();
  await expect(work, "the exclusive option left another checked").not.toBeChecked();
  await expect(none).toBeChecked();

  await study.check();
  await expect(none, "another option left the exclusive one checked").not.toBeChecked();
  await expect(study).toBeChecked();
});

test("a group is the fieldset it is asked in, never the whole page", async ({ page }) => {
  await page.goto(url(PAGE.checkbox));

  // THE PAGE CANNOT ASK THIS QUESTION BY ITSELF, so the test builds it.
  //
  // Every group here has a `name` of its own, deliberately — radios that share
  // one are a single group as far as the browser is concerned — so nothing on
  // the page exercises what happens when two fieldsets DO. A consumer's form
  // will: the same question asked once per row of a table is the ordinary case.
  // A second copy of the exclusive group, with the same names and fresh ids,
  // is that situation exactly.
  await page.evaluate(() => {
    const original = document.querySelector('.nd-fieldset[data-nd-group="apply-none"]');
    const copy = original.cloneNode(true);
    copy.dataset.ndGroup = "apply-none-copy";
    for (const element of copy.querySelectorAll("[id]")) element.id = `copy-${element.id}`;
    for (const label of copy.querySelectorAll("label[for]")) label.htmlFor = `copy-${label.htmlFor}`;
    original.after(copy);
  });

  await page.locator("#copy-nd-apply-none-study").check();
  await page.locator("#nd-apply-none-none").check();
  await expect(
    page.locator("#copy-nd-apply-none-study"),
    "the exclusive option cleared a choice in another fieldset",
  ).toBeChecked();
  // And the other direction: a choice over there must not clear the exclusive
  // option over here.
  await page.locator("#copy-nd-apply-none-work").check();
  await expect(
    page.locator("#nd-apply-none-none"),
    "a choice in another fieldset cleared the exclusive option",
  ).toBeChecked();

  // Radios are the sharper version of it: two groups sharing a `name` are ONE
  // group as far as the browser is concerned, whatever anything here does.
  const names = await page.evaluate(() =>
    [...document.querySelectorAll('.nd-choice-input[type="radio"]')].map((input) => [
      input.name,
      input.closest("fieldset")?.dataset.ndGroup ?? null,
    ]),
  );
  const groups = new Map();
  for (const [name, fieldset] of names) {
    if (!groups.has(name)) groups.set(name, new Set());
    groups.get(name).add(fieldset);
  }
  for (const [name, fieldsets] of groups) {
    expect([...fieldsets].length, `radios called "${name}" are in ${fieldsets.size} fieldsets`).toBe(
      1,
    );
  }
});

test("the reveal follows its control, and says so", async ({ page }) => {
  await page.goto(url(PAGE.checkbox));
  const control = page.locator("#nd-apply-other-other");
  const block = page.locator("#nd-apply-other-other-reveal");

  await expect(block).toBeHidden();
  // `aria-expanded` is written by the script and only by the script — see the
  // scriptless case below for why that is the right way round.
  expect(await control.getAttribute("aria-expanded")).toBe("false");
  expect(await control.getAttribute("aria-controls")).toBe("nd-apply-other-other-reveal");

  await control.check();
  await expect(block).toBeVisible();
  expect(await control.getAttribute("aria-expanded")).toBe("true");
});

test("choosing another answer closes the block the old one revealed", async ({ page }) => {
  await page.goto(url(PAGE.radio));
  const other = page.locator("#nd-contact-other-other");
  const block = page.locator("#nd-contact-other-other-reveal");

  await other.check();
  await expect(block).toBeVisible();

  // No event fires on the radio that was unchecked, so a handler that only
  // looked at the control that changed would leave this open under an answer
  // nobody gave any more.
  await page.locator("#nd-contact-other-email").check();
  await expect(block, "the block stayed open under an answer nobody gave").toBeHidden();
  expect(await other.getAttribute("aria-expanded")).toBe("false");
});

test("a disabled fieldset refuses every control inside it", async ({ page }) => {
  await page.goto(url(PAGE.fieldset));
  const inside = await page.locator('.nd-fieldset[disabled] .nd-choice-input').all();
  expect(inside.length, "no disabled fieldset on the page").toBeGreaterThan(1);
  for (const input of inside) await expect(input).toBeDisabled();
});

test("the error and the rule are one fact", async ({ page }) => {
  await page.goto(url(PAGE.fieldset));

  // `data-nd-invalid` is derived from `error` being present, so a red edge with
  // no sentence explaining it is not a state this can be in — in either
  // direction.
  const groups = await page.evaluate(() =>
    [...document.querySelectorAll(".nd-fieldset")].map((fieldset) => ({
      name: fieldset.dataset.ndGroup,
      flagged: fieldset.dataset.ndInvalid === "true",
      sentence: fieldset.querySelector(".nd-fieldset-error") !== null,
    })),
  );
  // A floor rather than a census: the point is that the loop below has
  // something to loop over. It said "more than five" when six components
  // shared one page, which was a fact about that page and not about this
  // claim.
  expect(groups.length, "the page renders no groups").toBeGreaterThan(2);
  for (const group of groups) {
    expect(group.flagged, `${group.name} is flagged without a sentence`).toBe(group.sentence);
  }

  const error = page.locator(".nd-fieldset-error").first();
  await expect(error).toBeVisible();
  // It appears after a submission, and a message that appears silently is one a
  // reader never learns about.
  expect(await error.getAttribute("role")).toBe("alert");
  // The group points at it, so it is read with the question rather than after
  // the last control.
  const described = await page
    .locator('.nd-fieldset[data-nd-invalid="true"]')
    .first()
    .getAttribute("aria-describedby");
  expect(described).toContain(await error.getAttribute("id"));
});

test("the hint is pointed at rather than merely placed beside", async ({ page }) => {
  await page.goto(url(PAGE.checkbox));
  const pairs = await page.evaluate(() =>
    [...document.querySelectorAll(".nd-choice-hint")].map((hint) => ({
      id: hint.id,
      named: hint.closest(".nd-choice").querySelector(".nd-choice-input")?.getAttribute("aria-describedby"),
    })),
  );
  expect(pairs.length, "no hints on the page").toBeGreaterThan(2);
  for (const pair of pairs) {
    expect(pair.named, `the hint ${pair.id} is not announced`).toBe(pair.id);
  }

  // And nothing points at an id nothing carries, which is read as silence by
  // some engines and as the whole document by others.
  const dangling = await page.evaluate(() =>
    [...document.querySelectorAll("[aria-describedby]")]
      .flatMap((el) => el.getAttribute("aria-describedby").split(/\s+/))
      .filter((id) => id && !document.getElementById(id)),
  );
  expect(dangling, "aria-describedby names an id nothing carries").toEqual([]);
});

test("every label points at a control, and every control has one", async ({ page }) => {
  await page.goto(url(PAGE.checkbox));
  const problems = await page.evaluate(() => {
    const found = [];
    for (const label of document.querySelectorAll(".nd-choice-label")) {
      const target = document.getElementById(label.htmlFor);
      if (!target) found.push(`the label "${label.textContent}" points at nothing`);
    }
    // And no id is claimed twice, which is the silent version of the same
    // failure: the second control is nameless and nothing says so.
    const ids = [...document.querySelectorAll(".nd-choice-input")].map((input) => input.id);
    const seen = new Set();
    for (const id of ids) {
      if (seen.has(id)) found.push(`two controls are called ${id}`);
      seen.add(id);
    }
    return found;
  });
  expect(problems).toEqual([]);
});

test("with the script blocked a checkbox group is still a form", async ({ browser }) => {
  const context = await browser.newContext({ javaScriptEnabled: false });
  const noScript = await context.newPage();
  await noScript.goto(`http://127.0.0.1:8731${url(PAGE.checkbox)}`);

  // Real controls, with the value that would be submitted on them.
  const input = noScript.locator("#nd-apply-work");
  expect(await input.getAttribute("type")).toBe("checkbox");
  expect(await input.getAttribute("value")).toBe("work");
  await expect(input).toBeChecked();

  // THE CONDITIONAL BLOCK IS VISIBLE, and that is the point of the direction
  // this degrades in. Hidden by CSS and revealed by script would be a follow-up
  // question a reader without script never sees at all.
  await expect(noScript.locator("#nd-apply-other-other-reveal")).toBeVisible();

  // And nothing claims a state it is not in: `aria-expanded` is written by the
  // script, so with no script there is nothing announcing `collapsed` over a
  // block the reader can see.
  expect(await noScript.locator("#nd-apply-other-other").getAttribute("aria-expanded")).toBeNull();

  // Checking still works, because it is a checkbox.
  await noScript.locator("#nd-apply-study").check();
  await expect(noScript.locator("#nd-apply-study")).toBeChecked();
  await context.close();
});

test("with the script blocked a radiogroup is still a form", async ({ browser }) => {
  const context = await browser.newContext({ javaScriptEnabled: false });
  const noScript = await context.newPage();
  await noScript.goto(`http://127.0.0.1:8731${url(PAGE.radio)}`);
  await expect(noScript.locator("#nd-contact-email")).toBeChecked();
  const chosen = noScript.locator("#nd-contact-phone");
  await chosen.check();
  await expect(chosen).toBeChecked();
  await expect(noScript.locator("#nd-contact-email")).not.toBeChecked();
  await context.close();
});

test("a radiogroup is one tab stop, walked with the arrows", async ({ page }) => {
  await page.goto(url(PAGE.radio));

  // A radiogroup is entered once and walked with the arrows, which is the
  // browser's own behaviour on `<input type=radio>` and the reason this is one
  // rather than three buttons wearing `role="radio"`.
  await page.locator("#nd-contact-email").focus();
  await page.keyboard.press("ArrowDown");
  await expect(page.locator("#nd-contact-phone"), "the arrow did not move").toBeFocused();
  await expect(page.locator("#nd-contact-phone"), "the arrow moved without choosing").toBeChecked();

  // Tab leaves the group entirely rather than stepping to the next answer.
  await page.keyboard.press("Tab");
  const left = await page.evaluate(() => document.activeElement?.name !== "contact");
  expect(left, "Tab stepped within the group instead of out of it").toBe(true);
});

test("Space is the checkbox's, unchanged", async ({ page }) => {
  await page.goto(url(PAGE.checkbox));
  await page.locator("#nd-apply-study").focus();
  await page.keyboard.press("Space");
  await expect(page.locator("#nd-apply-study")).toBeChecked();
});

test("focus is visible on the drawing, since the control is not", async ({ page }) => {
  await page.goto(url(PAGE.checkbox));
  await page.locator("#nd-apply-study").focus();
  // Keyboard focus, not a click: `:focus-visible` is the whole point.
  await page.keyboard.press("Tab");
  await page.keyboard.press("Shift+Tab");

  const ring = await page.evaluate(() => {
    const box = document.querySelector("#nd-apply-study").parentElement.querySelector(".nd-choice-box");
    const style = getComputedStyle(box);
    return { width: style.outlineWidth, style: style.outlineStyle };
  });
  // An outline on an `opacity: 0` input is invisible with it, so the ring has
  // to be on the box the reader can actually see.
  expect(ring.style, "the focused choice draws no ring").not.toBe("none");
  expect(parseFloat(ring.width)).toBeGreaterThan(0);
});

test("no choice on the page is a specimen pretending to be a control", async ({ page }) => {
  await page.goto(url(PAGE.checkbox));
  // The variant grid ships raw markup — a box and nothing else — and it is
  // deliberate: `size` is the box. What it must not become is a row that looks
  // like a control and answers nothing, so the two are told apart here and
  // every assertion above reads only the real ones.
  const real = await page.locator(REAL).count();
  const boxes = await page.locator(".nd-choice").count();
  expect(real, "there are no real controls on the page").toBeGreaterThan(10);
  expect(boxes - real, "the variant grid lost its specimens").toBeGreaterThan(2);
});

test("the tick is drawn heavily enough to read as a mark", async ({ page }) => {
  // LEGIBLE AND THIN ARE TWO DIFFERENT CLAIMS, and only the first was being
  // made. The tick was the icon set's own 1.75 stroke on a 24px glyph — white
  // on the accent fill, so the contrast test above passed comfortably while
  // the mark itself was a hairline in a forty-pixel box. What separates a
  // hairline from a mark is how much of the box it covers, so that is what is
  // measured: the fraction of the drawing that is ink.
  await page.goto(url(PAGE.checkbox));
  await page.locator("#nd-apply-study").check();
  const glyph = page
    .locator(".nd-choice:has(#nd-apply-study) .nd-choice-box .nd-icon-glyph")
    .first();
  const shot = await glyph.screenshot();
  // `ink` takes the lightest pixel as the ground, which here is the tick
  // itself: it is white on a filled square. The complement is what is wanted,
  // so the box is read the other way round — the darkest pixel is the fill and
  // everything near the light end is the stroke.
  const covered = await page.evaluate(async (data) => {
    const blob = await (await fetch(`data:image/png;base64,${data}`)).blob();
    const bitmap = await createImageBitmap(blob);
    const canvas = new OffscreenCanvas(bitmap.width, bitmap.height);
    const ctx = canvas.getContext("2d");
    ctx.drawImage(bitmap, 0, 0);
    const { data: px } = ctx.getImageData(0, 0, bitmap.width, bitmap.height);
    let ground = Infinity;
    for (let i = 0; i < px.length; i += 4) {
      ground = Math.min(ground, px[i] + px[i + 1] + px[i + 2]);
    }
    let drawn = 0;
    for (let i = 0; i < px.length; i += 4) {
      if (px[i] + px[i + 1] + px[i + 2] - ground > 60) drawn += 1;
    }
    return drawn / (px.length / 4);
  }, shot.toString("base64"));
  // The glyph's box is 24px square and the tick crosses about two thirds of
  // it, so a one-unit change in the stroke is a few percent of the box.
  // Measured: 0.070 at the 1.75 this shipped with, 0.113 at the weight of the
  // box's own edge and a hairline more. The floor sits between them.
  expect(covered, "the tick is a hairline rather than a mark").toBeGreaterThan(0.09);
});

test("the rule down a refused group covers the whole of its first line", async ({ page }) => {
  // A `<legend>` STRADDLES ITS FIELDSET'S BLOCK-START BORDER. Every engine
  // paints a fieldset's borders from the legend's own middle downwards, so the
  // mark on a refused group began halfway down the question and left the top
  // half of it unmarked — measured on this page as red starting 14px into a
  // 117px box whose legend is 26px tall. Nothing anywhere reported it: the
  // declaration is correct, and it is the element that moves.
  //
  // Read from the pixels rather than from the declaration, because the
  // declaration was never the thing that was wrong.
  await page.goto(url(PAGE.fieldset));
  await showParts(page);
  const group = page.locator('.nd-fieldset[data-nd-invalid="true"]').first();
  await expect(group).toBeVisible();
  const shot = await group.screenshot();
  const rule = await page.evaluate(async (data) => {
    const blob = await (await fetch(`data:image/png;base64,${data}`)).blob();
    const bitmap = await createImageBitmap(blob);
    const canvas = new OffscreenCanvas(bitmap.width, bitmap.height);
    const ctx = canvas.getContext("2d");
    ctx.drawImage(bitmap, 0, 0);
    const { data: px } = ctx.getImageData(0, 0, bitmap.width, bitmap.height);
    // One pixel in from the leading edge, down the whole box: the mark is the
    // only red thing that column can hold.
    const rows = [];
    for (let y = 0; y < bitmap.height; y += 1) {
      const i = (y * bitmap.width + 1) * 4;
      if (px[i] > 140 && px[i + 1] < 110 && px[i + 2] < 110) rows.push(y);
    }
    return { height: bitmap.height, first: rows[0], last: rows[rows.length - 1], drawn: rows.length };
  }, shot.toString("base64"));

  expect(rule.drawn, "no mark is drawn beside the refused group").toBeGreaterThan(0);
  // It starts at the top of the question rather than in the middle of it. One
  // pixel of slack for the antialiasing of a rounded corner above it.
  expect(rule.first, "the mark starts below the first line").toBeLessThanOrEqual(1);
  expect(rule.last, "the mark stops before the group does").toBeGreaterThanOrEqual(
    rule.height - 2,
  );
});
