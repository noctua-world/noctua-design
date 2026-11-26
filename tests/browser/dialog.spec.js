// The platform's `<dialog>`, and the two halves of it this system writes.
//
// WHAT IS BEING GUARDED. Almost everything a modal dialog does is the browser's
// — the focus trap, the inert background, the focus restoration, Escape and the
// backdrop — and the value of using `<dialog>` at all is that none of it is
// reimplemented here. So these tests assert the platform's behaviour is reached
// rather than replaced, and then assert the two things that ARE written: the
// trigger the script reveals, and the Escape a non-modal surface is owed.
//
// The `[open]` scoping is asserted too, because it is the bug this system has
// already shipped once: a closed `<dialog>` is `display: none` by the
// user-agent stylesheet, and one layout property on the class un-hides every
// closed dialog in the document.

import { expect, test } from "@playwright/test";

import { PAGE, url, showParts } from "./pages.js";
import { contrast, extremes } from "./pixels.js";

const named = (page, name) => page.locator(`[data-nd-dialog="${name}"]`);

async function dialogs(page) {
  await page.goto(url(PAGE.dialog));
  await showParts(page);
}

test("with the script blocked there is no control that opens nothing", async ({ browser }) => {
  const context = await browser.newContext({ javaScriptEnabled: false });
  const page = await context.newPage();
  await page.goto(url(PAGE.dialog));
  await showParts(page);
  const trigger = named(page, "dialog-remove").locator(".nd-dialog-open");
  await expect(trigger).toBeAttached();
  await expect(trigger).toBeHidden();
  await context.close();
});

test("every dialog on the page ships shut", async ({ page }) => {
  await dialogs(page);
  // THE BUG THIS EXISTS FOR. `display: flex` on the class rather than on
  // `[open]` un-hides every closed dialog in the document, and each one then
  // draws its box over the page with nothing else anywhere saying so.
  const surfaces = await page.locator(".nd-dialog-surface").evaluateAll((els) => ({
    count: els.length,
    open: els.filter((el) => el.open || el.checkVisibility()).length,
  }));
  // THE FLOOR, AND IT MATTERS MOST HERE. This is the only guard for the bug
  // above, and a count of nothing has nothing open in it. A renamed surface
  // class read as every dialog being shut. The page ships five of them, and
  // three are named by the tests below.
  expect(surfaces.count, "the page drew no dialog to check").toBeGreaterThan(2);
  expect(surfaces.open, "a closed dialog is being drawn over the page").toBe(0);
});

test("the script reveals the trigger, and its box was there before it", async ({ page }) => {
  await dialogs(page);
  const trigger = named(page, "dialog-remove").locator(".nd-dialog-open");
  await expect(trigger).toBeVisible();
  expect(await trigger.evaluate((el) => el.tagName)).toBe("BUTTON");
  expect(await trigger.getAttribute("type")).toBe("button");
  expect(await trigger.getAttribute("aria-haspopup")).toBe("dialog");

  // 24px is the AA floor for a target.
  const box = await trigger.boundingBox();
  expect(box.height).toBeGreaterThanOrEqual(24);

  // AND THE SPACE WAS RESERVED BEFORE THE SCRIPT RAN. The container holds a
  // minimum height under `[data-nd-scripting]`, which the inline head script
  // stamps before the first paint — so unhiding the button moves nothing.
  const held = await named(page, "dialog-remove").evaluate(
    (el) => Number.parseFloat(getComputedStyle(el).minBlockSize) > 0,
  );
  expect(held, "the trigger arrives into a box nothing was holding").toBe(true);
});

test("a modal one takes the focus, marks the page inert and gives the focus back", async ({
  page,
}) => {
  await dialogs(page);
  const root = named(page, "dialog-remove");
  const surface = root.locator(".nd-dialog-surface");

  await root.locator(".nd-dialog-open").click();
  await expect(surface).toBeVisible();
  expect(await root.getAttribute("data-nd-open")).toBe("");

  // The platform's own doing, and the reason none of it is written by hand:
  // the surface is in the top layer and the document behind it is inert.
  const inside = await page.evaluate(() =>
    document
      .querySelector('[data-nd-dialog="dialog-remove"] .nd-dialog-surface')
      .contains(document.activeElement),
  );
  expect(inside, "focus is outside a modal surface").toBe(true);

  await page.keyboard.press("Escape");
  await expect(surface).toBeHidden();
  // POLLED, NOT READ ONCE. The browser hides the surface and dispatches
  // `close` on a later task, and the state is taken off in that listener — so
  // a single read between the two saw `data-nd-open=""` on a dialog that had
  // shut. It failed about one run in three, which is the worst rate there is.
  await expect(root).not.toHaveAttribute("data-nd-open");
  // Restored by `close`, which is the other half of what `showModal` buys.
  await expect
    .poll(() => page.evaluate(() => document.activeElement.className))
    .toContain("nd-dialog-open");
});

test("a non-modal one leaves the page reachable and still answers Escape", async ({ page }) => {
  await dialogs(page);
  const root = named(page, "dialog-notes");
  const surface = root.locator(".nd-dialog-surface");
  await root.locator(".nd-dialog-open").click();
  await expect(surface).toBeVisible();

  // `show()` puts nothing in the top layer, so the page behind it is ordinary.
  const modal = await surface.evaluate((el) => el.matches(":modal"));
  expect(modal, "a `modal: false` dialog reached the top layer").toBe(false);

  // AND THE PLATFORM ANSWERS NOTHING HERE. `cancel` fires for a modal surface
  // only, so this key is `dist/dialog.js` doing the half that was left out.
  await surface.locator(".nd-dialog-close").focus();
  await page.keyboard.press("Escape");
  await expect(surface).toBeHidden();
  // And focus is not left on `<body>`, where the next Tab restarts the document.
  const landed = await page.evaluate(() => document.activeElement !== document.body);
  expect(landed, "focus fell to the body when a non-modal dialog closed").toBe(true);
});

test("a non-modal one takes room in the flow, so nothing above it can cut it off", async ({
  page,
}) => {
  // THE DEFECT THIS IS NAMED AFTER. The surface was `position: absolute`
  // against the control that opened it, which is the one position that could
  // not work: out of flow, so no ancestor grows to hold it, and not `fixed`,
  // so every ancestor's `overflow` still clips it. Inside an example frame at
  // 1120px that was a 168px panel in a stage 171px tall, cut off 95px short —
  // its own heading, its close control and half a line of body, with the rest
  // unreachable and 53px of blank frame below the cut.
  //
  // MEASURED, NOT READ. `getComputedStyle(…).position` would say `relative`
  // for a panel that is still being cut in half by something above it, and it
  // would say `absolute` for one that happens to fit. The claim is about where
  // the edges land, so the edges are what this reads.
  await dialogs(page);
  const root = named(page, "dialog-notes");
  const surface = root.locator(".nd-dialog-surface");
  await root.locator(".nd-dialog-open").click();
  await expect(surface).toBeVisible();

  const seen = await root.evaluate((el) => {
    const box = (node) => {
      const r = node.getBoundingClientRect();
      return { top: r.top, bottom: r.bottom, height: r.height };
    };
    const stage = el.closest("[data-nd-frame-stage]");
    return {
      panel: box(el.querySelector(".nd-dialog-surface")),
      trigger: box(el.querySelector(".nd-dialog-open")),
      root: box(el),
      stage: box(stage),
      frame: box(el.closest(".nd-frame")),
      // What the stage is holding below its own bottom edge. `overflow-x: auto`
      // computes `overflow-y` to `auto` as well, so a container that cannot
      // grow hides the rest behind a scrollbar rather than clipping it — and a
      // screenshot of the two looks the same.
      hidden: stage.scrollHeight - stage.clientHeight,
    };
  });

  // THE FLOOR. A panel that drew nothing would satisfy every bound below it.
  expect(seen.panel.height, "the non-modal surface drew almost nothing").toBeGreaterThan(60);
  // It is under the control that opened it, and clear of it.
  expect(seen.panel.top).toBeGreaterThanOrEqual(seen.trigger.bottom);
  // THE COMPONENT GREW AROUND IT, which is what being in the flow means and
  // what the old arrangement could not do at any size.
  expect(seen.root.bottom, "the component did not grow to hold its own panel").toBeGreaterThanOrEqual(
    seen.panel.bottom - 1,
  );
  // And so did everything above it.
  expect(seen.panel.bottom, "the stage cut the panel off").toBeLessThanOrEqual(seen.stage.bottom + 1);
  expect(seen.panel.bottom, "the frame cut the panel off").toBeLessThanOrEqual(seen.frame.bottom + 1);
  expect(seen.hidden, "the stage is holding something below its own bottom edge").toBeLessThanOrEqual(
    1,
  );
});

test("with no heading the close control is still in the corner, and costs no line", async ({
  page,
}) => {
  // THE DEFECT. `Dialog` draws its head row whenever there is a heading OR a
  // close control, and that row is a flex line at `space-between` — where ONE
  // item lands at the START. So a dialog named by `label` put the `×` at the
  // top left with the whole width of the panel empty beside it, and spent a
  // 48px line on it in a 111px panel holding a single sentence.
  //
  // BOTH SHAPES ARE MEASURED IN ONE TEST, because the claim is that they agree:
  // the control a reader reaches for is in the same corner whether or not there
  // is a title beside it.
  await dialogs(page);

  const read = async (name) => {
    const root = named(page, name);
    await root.locator(".nd-dialog-open").click();
    await expect(root.locator(".nd-dialog-surface")).toBeVisible();
    const seen = await root.evaluate((el) => {
      const box = (node) => {
        const r = node.getBoundingClientRect();
        return { top: r.top, right: r.right, left: r.left, bottom: r.bottom };
      };
      const surface = el.querySelector(".nd-dialog-surface");
      const close = surface.querySelector(".nd-dialog-close");
      const body = surface.querySelector(".nd-dialog-body");
      const first = body.firstElementChild;
      return {
        surface: box(surface),
        close: box(close),
        body: box(body),
        first: first ? box(first) : null,
        name: close.getAttribute("aria-label"),
        named: surface.getAttribute("aria-label") || surface.getAttribute("aria-labelledby"),
        titled: Boolean(surface.querySelector(".nd-dialog-title")),
      };
    });
    await page.keyboard.press("Escape");
    await expect(root.locator(".nd-dialog-surface")).toBeHidden();
    return seen;
  };

  const titled = await read("dialog-remove");
  const bare = await read("dialog-rename");
  expect(titled.titled, "the titled dialog drew no title").toBe(true);
  expect(bare.titled, "the headless dialog drew a title").toBe(false);

  // ONE CORNER, TWO SHAPES. Measured from the panel's own edges, so the two are
  // comparable across dialogs of different widths and sizes.
  const fromEnd = (one) => Math.round(one.surface.right - one.close.right);
  const fromTop = (one) => Math.round(one.close.top - one.surface.top);
  expect(fromEnd(bare), "the close control is not at the panel's trailing edge").toBe(
    fromEnd(titled),
  );
  expect(fromTop(bare)).toBe(fromTop(titled));
  // AND THE TWO AGREEING IS NOT ENOUGH ON ITS OWN: both at the leading edge
  // would satisfy it. Every locale built here is left to right, so the far side
  // is the right one, and the control is nearer to it than to the near side.
  expect(bare.close.left - bare.surface.left).toBeGreaterThan(bare.surface.right - bare.close.right);

  // AND IT COSTS NO LINE. With a title the row is real content and the body
  // begins under it; with none there is no row, so the body begins at the top
  // of the panel and the sentence is the first thing in it.
  const bodyStart = (one) => Math.round(one.body.top - one.surface.top);
  expect(bodyStart(titled), "the titled dialog has no head row to compare against").toBeGreaterThan(
    24,
  );
  expect(bodyStart(bare), "a headless dialog still reserves a line for its close control").toBeLessThan(
    8,
  );

  // AND THE CONTENT DOES NOT RUN UNDER THE GLYPH. The control is out of flow,
  // so nothing else knows it is there unless the gutter says so.
  expect(bare.first, "the headless dialog drew no content").not.toBeNull();
  expect(bare.first.right, "the first line runs under the close control").toBeLessThanOrEqual(
    bare.close.left,
  );

  // THE NAMES SURVIVE BOTH SHAPES: the surface is named by its title in one and
  // by `label` in the other, and the control is named in each.
  expect(titled.named).toMatch(/\S/);
  expect(bare.named).toMatch(/\S\s\S/);
  expect(bare.name).toMatch(/\S\s\S/);
});

test("the close control has a name, and a caller may refuse one", async ({ page }) => {
  await dialogs(page);
  const close = named(page, "dialog-remove").locator(".nd-dialog-close");
  await named(page, "dialog-remove").locator(".nd-dialog-open").click();
  await expect(close).toBeVisible();
  expect(await close.getAttribute("aria-label")).toMatch(/\S\s\S/);
  const box = await close.boundingBox();
  expect(box.width).toBeGreaterThanOrEqual(24);
  expect(box.height).toBeGreaterThanOrEqual(24);
  await page.keyboard.press("Escape");

  // `dismissible: false` draws none. Escape and the backdrop are still the way
  // out, and both of those are the platform's.
  await expect(named(page, "dialog-terms").locator(".nd-dialog-close")).toHaveCount(0);
});

test("it is named, and the visible title is what names it", async ({ page }) => {
  await dialogs(page);
  const shapes = await page.locator(".nd-dialog-surface").evaluateAll((els) =>
    els.map((el) => ({
      labelledby: el.getAttribute("aria-labelledby"),
      label: el.getAttribute("aria-label"),
      titled: Boolean(el.querySelector(".nd-dialog-title")),
      resolves: el.getAttribute("aria-labelledby")
        ? Boolean(document.getElementById(el.getAttribute("aria-labelledby")))
        : true,
    })),
  );
  expect(shapes.length).toBeGreaterThan(0);
  for (const shape of shapes) {
    expect(shape.labelledby || shape.label, "a dialog with no accessible name").toBeTruthy();
    // Never both: a name written twice can drift from itself.
    expect(shape.labelledby && shape.label).toBeFalsy();
    expect(shape.resolves, "aria-labelledby points at nothing").toBe(true);
    if (shape.titled) expect(shape.labelledby).toBeTruthy();
  }
});

test("the surface clips and the body scrolls, and only one of them is round", async ({ page }) => {
  await dialogs(page);
  await page.evaluate(() => document.documentElement.setAttribute("data-noctua-radius", "circular"));
  await named(page, "dialog-terms").locator(".nd-dialog-open").click();

  const shape = await named(page, "dialog-terms").evaluate((el) => {
    const surface = getComputedStyle(el.querySelector(".nd-dialog-surface"));
    const body = getComputedStyle(el.querySelector(".nd-dialog-body"));
    return {
      clips: surface.overflow,
      radius: surface.borderRadius,
      scrolls: body.overflowY,
      bodyRadius: body.borderRadius,
      shadow: surface.boxShadow,
    };
  });
  // A scrollable element never carries a radius, and a radius element never
  // scrolls. `.nd-panel` is two elements for this reason and so is this.
  expect(shape.clips).toBe("hidden");
  expect(shape.scrolls).toBe("auto");
  expect(shape.bodyRadius).toBe("0px");
  // And separation is a hairline plus a surface colour, never elevation.
  expect(shape.shadow, "a dialog is drawing an elevated surface").toBe("none");
});

test("a non-modal one still answers Escape after Tab has walked out of it", async ({ page }) => {
  // THE HOLE THIS CLOSES. `keydown` was bound to the SURFACE, and `dialog.toml`
  // says in as many words that Tab may walk out of a non-modal dialog and leave
  // it open. The first place Tab lands on the way out is the trigger — outside
  // the surface, inside the root — and from there Escape reached nothing at
  // all, so the one key the contract offers a keyboard reader did nothing.
  // `popover.js` binds on the root and has never had this hole.
  await dialogs(page);
  const root = named(page, "dialog-notes");
  const surface = root.locator(".nd-dialog-surface");
  await root.locator(".nd-dialog-open").click();
  await expect(surface).toBeVisible();

  // Focus moved deliberately out of the surface and onto the trigger, which is
  // exactly where the reader's own Tab takes them.
  await root.locator(".nd-dialog-open").focus();
  const outside = await page.evaluate(() => {
    const dialog = document.querySelector('[data-nd-dialog="dialog-notes"] .nd-dialog-surface');
    return !dialog.contains(document.activeElement);
  });
  expect(outside, "the focus never left the surface, so this proves nothing").toBe(true);

  await page.keyboard.press("Escape");
  await expect(surface, "Escape did nothing once the focus had left the surface").toBeHidden();
});

test("Escape on a shut dialog is left for whatever else wants it", async ({ page }) => {
  // The other half: the key is answered only while the surface is open, or a
  // closed dialog would swallow the Escape meant for something around it.
  await dialogs(page);
  const root = named(page, "dialog-notes");
  await expect(root.locator(".nd-dialog-surface")).toBeHidden();
  const swallowed = await root.evaluate((el) => {
    let reached = false;
    const listen = () => {
      reached = true;
    };
    document.addEventListener("keydown", listen);
    el.querySelector(".nd-dialog-open").dispatchEvent(
      new KeyboardEvent("keydown", { key: "Escape", bubbles: true }),
    );
    document.removeEventListener("keydown", listen);
    return !reached;
  });
  expect(swallowed, "a shut dialog stopped an Escape that was not its own").toBe(false);
});

// --- What the layer behind it is for ----------------------------------------

/**
 * How readable the page is, from the pixels of a strip beside the dialog.
 *
 * The extremes of a strip of the page are its ink and its ground, so the
 * contrast between them is how legible whatever is written there still is. The
 * strip is taken clear of the surface itself, because the question is about
 * what the veil did to the page and not about the dialog drawn over it.
 */
async function legibility(page, strip) {
  const shot = await page.screenshot({ clip: strip });
  const { min, max } = await extremes(page, shot);
  return contrast(min, max);
}

for (const scheme of ["light", "dark"]) {
  test(`the page behind a modal one stops being readable in ${scheme}`, async ({ page }) => {
    // THE VEIL WAS A MARK WEARING A SCRIM'S HAT. `Dialog` painted its backdrop
    // from `--nd-veil-checked` — the fourteen percent that marks which value
    // is in force in a group — over the INK, whose colour inverts with the
    // mode. In light that was a wash nobody had to read around; in dark it was
    // a white haze at the same fourteen percent, which is fainter still.
    //
    // Measured from the pixels of the page beside the dialog, before and
    // after, and in both appearances — because a veil that works in one is
    // routinely invisible in the other.
    await page.emulateMedia({ colorScheme: scheme });
    await dialogs(page);
    const surface = named(page, "dialog-remove").locator(".nd-dialog-surface");
    await named(page, "dialog-remove").locator(".nd-dialog-open").click();
    await expect(surface).toBeVisible();

    // ONE STRIP, MEASURED TWICE. It is named while the surface is open, so it
    // is the same pixels either way — the page beside the dialog, clear of the
    // dialog itself, at the height the dialog covers.
    const box = await surface.boundingBox();
    const strip = {
      x: 8,
      y: box.y,
      width: Math.max(Math.round(box.x) - 24, 24),
      height: Math.round(box.height),
    };
    const veiled = await legibility(page, strip);
    await page.keyboard.press("Escape");
    await expect(surface).toBeHidden();
    const plain = await legibility(page, strip);

    // The page was readable a moment ago, so the strip has something in it.
    expect(plain, "the page beside the dialog carries no text to hide").toBeGreaterThan(4);
    // And behind an open dialog it is not. 3:1 is the floor for large text, so
    // a page under this is a page nothing on it can be read from.
    expect(veiled, `the page behind the dialog is still readable in ${scheme}`).toBeLessThan(3);
  });
}

// --- The confirmation ---------------------------------------------------------
//
// `alert`, the requester, the mark and the two answers: the surface the
// interface asks a person to confirm a command that cannot be undone on. The
// answers are submit controls of a `<form method="dialog">`, so closing on an
// answer is the platform's, and `dist/dialog.js` reports it as
// `nd-dialog-close` with the answer.

/** Every `nd-dialog-close` the page hears from now on, as `name:answer`. */
async function answers(page) {
  await page.evaluate(() => {
    window.__answers = [];
    document.addEventListener("nd-dialog-close", (event) => {
      window.__answers.push(`${event.detail.name}:${event.detail.answer}`);
    });
  });
  return () => page.evaluate(() => window.__answers.slice());
}

test("an alert dialog takes the focus on its decline, and a press beside it is no answer", async ({
  page,
}) => {
  await dialogs(page);
  const heard = await answers(page);
  const root = named(page, "dialog-delete");
  const surface = root.locator(".nd-dialog-surface");
  await root.locator(".nd-dialog-open").click();
  await expect(surface).toBeVisible();
  expect(await surface.getAttribute("role")).toBe("alertdialog");

  // THE LEAST DESTRUCTIVE ANSWER HAS THE FOCUS, so a stray Enter declines.
  await expect(root.locator(".nd-dialog-decline")).toBeFocused();

  // DESCRIBED BY WHAT IS ASKED AND BY THE MARK, and every id resolves.
  const described = await surface.evaluate((el) =>
    (el.getAttribute("aria-describedby") ?? "")
      .split(/\s+/)
      .filter(Boolean)
      .map((id) => document.getElementById(id)?.className ?? null),
  );
  expect(described).toEqual(["nd-dialog-note", "nd-dialog-mark"]);

  // A PRESS ON THE BACKDROP, which closes an ordinary modal dialog.
  await page.mouse.click(4, 4);
  await expect(surface, "a press beside an alert dialog answered it").toBeVisible();

  // ESCAPE LEAVES IT, as a decline.
  await page.keyboard.press("Escape");
  await expect(surface).toBeHidden();
  await expect.poll(heard).toEqual(["dialog-delete:decline"]);
});

test("each answer closes the surface and says which it was", async ({ page }) => {
  await dialogs(page);
  const heard = await answers(page);
  const root = named(page, "dialog-delete");
  const surface = root.locator(".nd-dialog-surface");

  await root.locator(".nd-dialog-open").click();
  await root.locator(".nd-dialog-confirm").click();
  await expect(surface).toBeHidden();
  await expect.poll(heard).toEqual(["dialog-delete:confirm"]);

  // An answer is per opening: the next one starts with none.
  await root.locator(".nd-dialog-open").click();
  await expect(surface).toBeVisible();
  expect(await surface.evaluate((el) => el.returnValue)).toBe("");
  await page.keyboard.press("Enter");
  await expect(surface).toBeHidden();
  await expect.poll(heard).toEqual(["dialog-delete:confirm", "dialog-delete:decline"]);
});

test("the mark is words beside a glyph, and the confirm control is painted from it", async ({
  page,
}) => {
  await dialogs(page);
  const root = named(page, "dialog-delete");
  await root.locator(".nd-dialog-open").click();
  const mark = root.locator(".nd-dialog-mark");
  await expect(mark).toBeVisible();
  expect((await mark.textContent()).trim()).toMatch(/\S\s\S/);
  expect(await mark.locator(".nd-dialog-mark-icon").getAttribute("aria-hidden")).toBe("true");
  // THE TONE IS THE SECOND SIGNAL, the words the first (`NOCTUA.md §8.8.6`).
  expect(await root.locator(".nd-dialog-confirm").getAttribute("data-nd-tone")).toBe("danger");
  // Both answers clear the target floor.
  for (const answer of [".nd-dialog-decline", ".nd-dialog-confirm"]) {
    const box = await root.locator(answer).boundingBox();
    expect(box.height).toBeGreaterThanOrEqual(24);
    expect(box.width).toBeGreaterThanOrEqual(24);
  }
});

test("a dialog its caller shows opens and closes when the caller writes it", async ({ page }) => {
  await dialogs(page);
  const heard = await answers(page);
  const root = named(page, "dialog-confirm");
  const surface = root.locator(".nd-dialog-surface");

  // NO TRIGGER, AND NO LINE KEPT FOR ONE.
  await expect(root.locator(".nd-dialog-open")).toHaveCount(0);
  const held = await root.evaluate((el) => Number.parseFloat(getComputedStyle(el).minBlockSize) || 0);
  expect(held, "a dialog with no trigger holds a line for one").toBe(0);

  // THE PAGE PLAYS THE CALLER: its control writes `shown`.
  await page.locator('[data-nd-action="dialog-show"]').click();
  await expect(surface).toBeVisible();
  await expect(root.locator(".nd-dialog-decline")).toBeFocused();
  await root.locator(".nd-dialog-confirm").click();
  await expect(surface).toBeHidden();
  await expect.poll(heard).toEqual(["dialog-confirm:confirm"]);
  // And writes the answer it heard under it, then `shown` back to false.
  await expect(page.locator(".nd-example-answer")).toContainText("confirm");
  await expect(root).toHaveAttribute("data-nd-shown", "false");

  // A CALLER THAT CLOSES ITS OWN DIALOG is told, with no answer.
  await root.evaluate((el) => el.setAttribute("data-nd-shown", "true"));
  await expect(surface).toBeVisible();
  await root.evaluate((el) => el.setAttribute("data-nd-shown", "false"));
  await expect(surface).toBeHidden();
  await expect.poll(heard).toEqual(["dialog-confirm:confirm", "dialog-confirm:"]);
});

for (const locale of ["en", "pt-BR", "qps"]) {
  test(`the confirmation fits a 320 by 240 window in ${locale}, and nothing in it is cut off`, async ({
    page,
  }) => {
    // THE INTERFACE'S SMALLEST WINDOW (`NOCTUA.md §12.19`). A head, a
    // description and two answers that wrap onto two rows are taller than it,
    // so below the `sm` step the surface scrolls as one: a part that does not
    // scroll is a part cut off.
    await page.setViewportSize({ width: 320, height: 240 });
    await page.goto(url(PAGE.dialog, locale));
    const root = named(page, "dialog-confirm");
    await root.evaluate((el) => el.setAttribute("data-nd-shown", "true"));
    const surface = root.locator(".nd-dialog-surface");
    await expect(surface).toBeVisible();

    const seen = await surface.evaluate((el) => {
      const box = el.getBoundingClientRect();
      const style = getComputedStyle(el);
      const cut = [];
      for (const part of el.querySelectorAll("*")) {
        const s = getComputedStyle(part);
        if (/^(hidden|clip)$/.test(s.overflowX) && part.scrollWidth > part.clientWidth + 1) {
          cut.push(`${part.className} across`);
        }
        if (/^(hidden|clip)$/.test(s.overflowY) && part.scrollHeight > part.clientHeight + 1) {
          cut.push(`${part.className} down`);
        }
      }
      return {
        top: box.top,
        bottom: box.bottom,
        left: box.left,
        right: box.right,
        across: el.scrollWidth - el.clientWidth,
        // What the surface holds below its own edge, and whether a reader can
        // scroll to it.
        below: el.scrollHeight - el.clientHeight,
        scrolls: style.overflowY === "auto" || style.overflowY === "scroll",
        cut,
      };
    });
    expect(seen.top).toBeGreaterThanOrEqual(0);
    expect(seen.bottom).toBeLessThanOrEqual(240);
    expect(seen.left).toBeGreaterThanOrEqual(0);
    expect(seen.right).toBeLessThanOrEqual(320);
    expect(seen.across, "the surface holds something wider than itself").toBeLessThanOrEqual(1);
    if (seen.below > 1) expect(seen.scrolls, "the surface cuts off what does not fit").toBe(true);
    expect(seen.cut, "a part of the surface is cut off by a box that does not scroll").toEqual([]);

    // AND THE FOCUS IS WHERE A READER CAN SEE IT.
    const decline = root.locator(".nd-dialog-decline");
    await expect(decline).toBeFocused();
    const box = await decline.boundingBox();
    expect(box.y).toBeGreaterThanOrEqual(seen.top - 1);
    expect(box.y + box.height).toBeLessThanOrEqual(seen.bottom + 1);
  });
}
