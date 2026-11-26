// The control that records a shortcut.
//
// WHAT THE CASES CANNOT SAY. `spec/components/key_capture.cases.toml` presses
// one key at a time, so it holds the markup, the labels and the keys alone.
// Asked here: a chord pressed with modifiers held and written in the grammar's
// order whatever order they went down in, a modifier held and let go recording
// nothing, Tab recorded once and the next Tab leaving, Escape kept from what is
// around the control only while it listens, what the two live regions say, the
// labels a layout prints, the page with the script blocked, and the release.

import { expect, test } from "@playwright/test";

import { LOCALES, PAGE, showParts, url } from "./pages.js";

async function open(page, locale = "en") {
  await page.goto(url(PAGE.keyCapture, locale));
  await showParts(page);
}

const capture = (page, name) => page.locator(`[data-nd-key-capture="${name}"]`).first();
const control = (page, name) => capture(page, name).locator(":scope > .nd-key-capture-control");
const held = (page, name) => capture(page, name).locator(':scope > input[type="hidden"]');
const taken = (page, name) => capture(page, name).locator(".nd-key-capture-taken");
const said = (page, name) => capture(page, name).locator(":scope > .nd-key-capture-said");

/** A token's length in pixels, measured on a box and not parsed off a string. */
async function token(page, name) {
  return page.evaluate((property) => {
    const probe = document.createElement("div");
    probe.style.position = "absolute";
    probe.style.blockSize = `var(${property})`;
    document.body.append(probe);
    const size = probe.getBoundingClientRect().height;
    probe.remove();
    return size;
  }, name);
}

/** Every `nd-key-capture-change` the page hears from now on. */
async function listen(page) {
  await page.evaluate(() => {
    window.heard = [];
    document.addEventListener("nd-key-capture-change", (event) => window.heard.push(event.detail));
  });
  return () => page.evaluate(() => window.heard);
}

/** Puts the keyboard on a control and starts it listening, as a reader does. */
async function arm(page, name) {
  await control(page, name).focus();
  await page.keyboard.press("Enter");
  await expect(capture(page, name)).toHaveAttribute("data-nd-listening", "true");
}

const box = (locator) =>
  locator.evaluate((one) => {
    const at = one.getBoundingClientRect();
    return { left: at.left, right: at.right, top: at.top, bottom: at.bottom, width: at.width, height: at.height };
  });

for (const width of [320, 768, 1280]) {
  test(`the label, the box and the remove control share a row that wraps, at ${width}`, async ({ page }) => {
    await page.setViewportSize({ width, height: 900 });
    for (const locale of LOCALES) {
      await open(page, locale);
      const tall = await token(page, "--noctua-space-12");
      const target = await token(page, "--noctua-target-min");
      const names = await page
        .locator("[data-nd-key-capture]")
        .evaluateAll((all) => all.map((one) => one.getAttribute("data-nd-key-capture")));
      expect(names.length, `${locale}: the page draws no capture`).toBeGreaterThan(5);
      for (const name of names) {
        const where = `${locale} ${width} ${name}`;
        const whole = await box(capture(page, name));
        const label = await box(capture(page, name).locator(":scope > .nd-label"));
        const box_ = await box(control(page, name));
        // THE CONTROL IS A MEDIUM BUTTON TALL, and inside the row it stands in.
        expect(box_.height, `${where}: the box is shorter than a medium button`).toBeGreaterThanOrEqual(tall - 0.5);
        expect(box_.right, `${where}: the box spills out of its row`).toBeLessThanOrEqual(whole.right + 0.5);
        expect(label.right, `${where}: the label spills out of its row`).toBeLessThanOrEqual(whole.right + 0.5);
        // THE LABEL AND THE BOX DO NOT OVERLAP, beside each other or one above.
        const across = Math.min(label.right, box_.right) - Math.max(label.left, box_.left);
        const down = Math.min(label.bottom, box_.bottom) - Math.max(label.top, box_.top);
        expect(across > 0.5 && down > 0.5, `${where}: the label and the box overlap`).toBe(false);
        const clear = capture(page, name).locator(":scope > .nd-key-capture-clear");
        if (await clear.isVisible()) {
          const hit = await box(clear);
          expect(hit.width, `${where}: the remove control is under the smallest target`).toBeGreaterThanOrEqual(target - 0.05);
          expect(hit.height, `${where}: the remove control is under the smallest target`).toBeGreaterThanOrEqual(target - 0.05);
          expect(hit.right, `${where}: the remove control spills out of its row`).toBeLessThanOrEqual(whole.right + 0.5);
        }
      }
    }
  });
}

test("the button is named by its label and its keys, and described by a refusal", async ({ page }) => {
  await open(page);
  await expect(control(page, "example-key-capture-basic")).toHaveAccessibleName(/^Leader Alt\s*\+\s*Space$/);
  await expect(control(page, "example-key-capture-unset")).toHaveAccessibleName(/^Open the help None$/);
  const refused = control(page, "example-key-capture-refused");
  await expect(refused).toHaveAccessibleDescription("The production channel already uses this shortcut.");
  await expect(refused).toHaveAttribute("aria-invalid", "true");
  // THE NAME FOLLOWS WHAT IS DRAWN. The caller's keys are hidden once a chord
  // is recorded, and the name is read through their parent, so the hidden run
  // is not read.
  await arm(page, "example-key-capture-basic");
  await page.keyboard.press("Control+KeyB");
  await expect(control(page, "example-key-capture-basic")).toHaveAccessibleName(/^Leader Ctrl\s*\+\s*B$/);
});

test("a chord is written in the grammar's order, whatever order its modifiers went down in", async ({ page }) => {
  await open(page);
  const heard = await listen(page);
  await arm(page, "example-key-capture-basic");
  await page.keyboard.press("Shift+Alt+Control+KeyY");
  const root = capture(page, "example-key-capture-basic");
  await expect(root).toHaveAttribute("data-nd-captured", "true");
  await expect(root).not.toHaveAttribute("data-nd-listening");
  await expect(held(page, "example-key-capture-basic")).toHaveValue("Control+Alt+Shift+KeyY");
  await expect(taken(page, "example-key-capture-basic")).toHaveText("Ctrl+Alt+Shift+Y");
  // THE CALLER'S KEYS GIVE WAY to the recorded ones, and are not rebuilt.
  await expect(root.locator(".nd-key-capture-keys")).toBeHidden();
  await expect(root.locator(".nd-key-capture-keys")).toHaveText("Alt+Space");
  await expect(control(page, "example-key-capture-basic")).toBeFocused();
  expect(await heard()).toEqual([{ name: "example-key-capture-basic", value: "Control+Alt+Shift+KeyY" }]);
});

test("a modifier held is shown and records nothing, and the key after it does", async ({ page }) => {
  await open(page);
  const heard = await listen(page);
  const name = "example-key-capture-basic";
  await arm(page, name);
  await page.keyboard.down("Alt");
  await expect(taken(page, name).locator("kbd[data-nd-code='Alt']")).toHaveCount(1);
  await page.keyboard.down("Shift");
  await expect(taken(page, name).locator("kbd")).toHaveText(["Alt", "Shift"]);
  await page.keyboard.up("Shift");
  await page.keyboard.up("Alt");
  await expect(taken(page, name).locator("kbd")).toHaveCount(0);
  await expect(capture(page, name), "a modifier alone ended listening").toHaveAttribute("data-nd-listening", "true");
  expect(await heard(), "a modifier alone was recorded").toEqual([]);
  await page.keyboard.press("KeyK");
  await expect(held(page, name)).toHaveValue("KeyK");
});

test("it says what to press, then what was recorded, and nothing when it stops", async ({ page }) => {
  await open(page);
  const name = "example-key-capture-basic";
  await expect(said(page, name)).toHaveAttribute("role", "status");
  await expect(said(page, name)).toHaveText("");
  await arm(page, name);
  await expect(said(page, name)).toHaveText("Press the new keys, or Escape");
  // THE WORDS ARE DRAWN IN THE BOX AS WELL, beside the modifiers held so far.
  await expect(taken(page, name).locator(".nd-key-capture-prompt")).toHaveText("Press the new keys, or Escape");
  await page.keyboard.press("Alt+KeyK");
  await expect(said(page, name)).toHaveText("Alt+K");
  await page.keyboard.press("Enter");
  await expect(said(page, name)).toHaveText("Press the new keys, or Escape");
  await page.keyboard.press("Escape");
  await expect(said(page, name), "stopping said something").toHaveText("");
  // THE REFUSAL IS A REGION OF ITS OWN, on the page from the start, so a
  // sentence a caller writes into it later is read.
  const refusal = capture(page, name).locator(":scope > .nd-key-capture-refusal");
  await expect(refusal).toHaveAttribute("role", "status");
  await expect(refusal).toHaveText("");
});

test("Tab is recorded once, and the next Tab leaves the control", async ({ page }) => {
  await open(page);
  const name = "example-key-capture-basic";
  await arm(page, name);
  await page.keyboard.press("Tab");
  await expect(held(page, name)).toHaveValue("Tab");
  await expect(control(page, name), "recording Tab took the focus away").toBeFocused();
  await page.keyboard.press("Tab");
  await expect(control(page, name), "the second Tab was held in the control").not.toBeFocused();
  // And with a modifier, which is a chord a binding may hold.
  await arm(page, name);
  await page.keyboard.press("Shift+Tab");
  await expect(held(page, name)).toHaveValue("Shift+Tab");
  await expect(control(page, name)).toBeFocused();
});

test("Escape stops listening and is kept from what is around only then", async ({ page }) => {
  await open(page);
  const name = "example-key-capture-basic";
  await page.evaluate(() => {
    window.escapes = 0;
    document.addEventListener("keydown", (event) => {
      if (event.key === "Escape") window.escapes += 1;
    });
  });
  await control(page, name).focus();
  await page.keyboard.press("Escape");
  expect(await page.evaluate(() => window.escapes), "an idle control kept Escape").toBe(1);
  await arm(page, name);
  await page.keyboard.press("Escape");
  await expect(capture(page, name)).not.toHaveAttribute("data-nd-listening");
  expect(await page.evaluate(() => window.escapes), "Escape that stopped listening reached the page").toBe(1);
  await expect(held(page, name)).toHaveValue("Alt+Space");
});

test("a pointer starts and stops listening, and so does the focus leaving", async ({ page }) => {
  await open(page);
  const name = "example-key-capture-basic";
  const root = capture(page, name);
  await control(page, name).click();
  await expect(root).toHaveAttribute("data-nd-listening", "true");
  await control(page, name).click();
  await expect(root).not.toHaveAttribute("data-nd-listening");
  await expect(held(page, name)).toHaveValue("Alt+Space");
  await control(page, name).click();
  await expect(root).toHaveAttribute("data-nd-listening", "true");
  await page.locator("h1").first().click();
  await expect(root, "the focus left and the control still listened").not.toHaveAttribute("data-nd-listening");
});

test("the dashed edge says it is listening, and a refusal is a rule and a sentence", async ({ page }) => {
  await open(page);
  const name = "example-key-capture-basic";
  const edge = (locator) => locator.evaluate((one) => getComputedStyle(one).borderTopStyle);
  expect(await edge(control(page, name))).toBe("solid");
  await arm(page, name);
  expect(await edge(control(page, name)), "listening is told by colour alone").toBe("dashed");

  const refused = capture(page, "example-key-capture-refused");
  const rule = await token(page, "--noctua-border-rule");
  const drawn = await refused.evaluate((one) => Number.parseFloat(getComputedStyle(one).borderInlineStartWidth));
  expect(drawn, "a refusal draws no rule down its edge").toBeCloseTo(rule, 1);
  await expect(refused.locator(":scope > .nd-key-capture-refusal")).toBeVisible();
  await expect(refused.locator(":scope > .nd-key-capture-refusal")).toHaveText(
    "The production channel already uses this shortcut.",
  );
});

test("a disabled control holds the focus, says what it holds and starts nothing", async ({ page }) => {
  await open(page);
  const name = "example-key-capture-locked";
  const heard = await listen(page);
  await control(page, name).focus();
  await expect(control(page, name)).toBeFocused();
  await expect(control(page, name)).toHaveAttribute("aria-disabled", "true");
  await page.keyboard.press("Enter");
  await control(page, name).click({ force: true });
  await expect(capture(page, name)).not.toHaveAttribute("data-nd-listening");
  // Nothing to remove from a shortcut that cannot change.
  await expect(capture(page, name).locator(":scope > .nd-key-capture-clear")).toBeHidden();
  expect(await heard()).toEqual([]);
});

test("the remove control empties the shortcut and gives the focus back to the box", async ({ page }) => {
  await open(page);
  const name = "example-key-capture-removable";
  const heard = await listen(page);
  const clear = capture(page, name).locator(":scope > .nd-key-capture-clear");
  await expect(clear).toBeVisible();
  await expect(clear).toHaveAccessibleName("Remove the shortcut");
  await clear.click();
  await expect(held(page, name)).toHaveValue("");
  await expect(taken(page, name)).toHaveText("None");
  await expect(clear, "a remove control with nothing to remove").toBeHidden();
  await expect(control(page, name)).toBeFocused();
  await expect(said(page, name)).toHaveText("None");
  expect(await heard()).toEqual([{ name, value: "" }]);
});

test("a value the caller writes back is what is in force", async ({ page }) => {
  await open(page);
  const name = "example-key-capture-basic";
  await arm(page, name);
  await page.keyboard.press("KeyZ");
  await expect(capture(page, name)).toHaveAttribute("data-nd-captured", "true");
  // THE CALLER TOOK THE CHORD AND WRITES IT BACK, as a view of its own state.
  // A value written back unchanged is no news, and leaves what was recorded.
  await capture(page, name).evaluate((one) => one.setAttribute("data-nd-value", "Alt+Space"));
  await expect(capture(page, name)).toHaveAttribute("data-nd-captured", "true");
  await capture(page, name).evaluate((one) => one.setAttribute("data-nd-value", "KeyZ"));
  await expect(capture(page, name), "a written value left the recorded chord drawn").not.toHaveAttribute(
    "data-nd-captured",
  );
  await expect(capture(page, name).locator(".nd-key-capture-keys")).toBeVisible();
});

test("a layout's own label replaces the US board's where the engine says what it is", async ({ page }) => {
  await page.addInitScript(() => {
    const map = new Map([
      ["KeyS", "ö"],
      ["KeyQ", "a"],
      ["Semicolon", "m"],
      ["KeyZ", "Dead"],
    ]);
    Object.defineProperty(navigator, "keyboard", {
      configurable: true,
      value: { getLayoutMap: async () => map },
    });
  });
  await open(page);
  // THE CALLER'S KEYS ARE RELABELLED IN PLACE, and keep their code.
  const settings = capture(page, "overview-key-capture-settings").locator("kbd[data-nd-code='KeyS']");
  await expect(settings).toHaveText("Ö");
  // A RECORDED KEY IS DRAWN BY THE SAME LABEL, and written by its code.
  const name = "example-key-capture-basic";
  await arm(page, name);
  await page.keyboard.press("Control+KeyQ");
  await expect(held(page, name)).toHaveValue("Control+KeyQ");
  await expect(taken(page, name)).toHaveText("Ctrl+A");
  // A name that is not one character keeps the US board's label.
  await arm(page, name);
  await page.keyboard.press("KeyZ");
  await expect(taken(page, name)).toHaveText("Z");
});

test("where the layout is unknown, a captured key takes the character it typed", async ({ page }) => {
  await page.addInitScript(() => {
    Object.defineProperty(navigator, "keyboard", { configurable: true, value: undefined });
  });
  await open(page);
  const name = "example-key-capture-basic";
  /** A key as a layout other than the runner's would send it. */
  const press = (init) =>
    control(page, name).evaluate((one, at) => {
      one.dispatchEvent(new KeyboardEvent("keydown", { bubbles: true, cancelable: true, ...at }));
      one.dispatchEvent(new KeyboardEvent("keyup", { bubbles: true, cancelable: true, ...at }));
    }, init);
  await arm(page, name);
  await press({ code: "KeyQ", key: "a" });
  await expect(held(page, name)).toHaveValue("KeyQ");
  await expect(taken(page, name)).toHaveText("A");
  // SHIFT CHANGES THE CHARACTER, so a key typed with it is not a label.
  await arm(page, name);
  await press({ code: "KeyW", key: "Z", shiftKey: true });
  await expect(held(page, name)).toHaveValue("Shift+KeyW");
  await expect(taken(page, name)).toHaveText("Shift+W");
});

test("with the script blocked the shortcut is shown and the control is disabled", async ({ browser }) => {
  const context = await browser.newContext({ javaScriptEnabled: false });
  const page = await context.newPage();
  try {
    await page.goto(url(PAGE.keyCapture));
    const name = "example-key-capture-removable";
    await expect(control(page, name)).toBeDisabled();
    await expect(capture(page, name).locator(".nd-key-capture-keys")).toHaveText("S");
    await expect(capture(page, name).locator(":scope > .nd-key-capture-clear")).toBeHidden();
    await expect(held(page, name)).toHaveValue("KeyS");
  } finally {
    await context.close();
  }
});

test("releasing a capture takes back what the script wrote", async ({ page }) => {
  await open(page);
  const name = "example-key-capture-removable";
  await arm(page, name);
  await page.keyboard.press("KeyD");
  await expect(held(page, name)).toHaveValue("KeyD");
  await page.evaluate(async () => {
    const { releaseKeyCaptures } = await import("../../assets/key-capture.js");
    releaseKeyCaptures(document);
  });
  const root = capture(page, name);
  await expect(control(page, name), "a released control still answers").toBeDisabled();
  await expect(root).not.toHaveAttribute("data-nd-captured");
  await expect(root).not.toHaveAttribute("data-nd-enhanced");
  await expect(taken(page, name)).toBeEmpty();
  await expect(held(page, name)).toHaveValue("KeyS");
  await expect(root.locator(":scope > .nd-key-capture-clear")).toBeHidden();

  await page.evaluate(async () => {
    const { enhanceKeyCaptures } = await import("../../assets/key-capture.js");
    enhanceKeyCaptures(document);
  });
  await expect(control(page, name)).toBeEnabled();
  await expect(root.locator(":scope > .nd-key-capture-clear")).toBeVisible();
});

test("the control takes the page's face, and follows the interface-font setting", async ({ page }) => {
  // A FORM CONTROL DOES NOT TAKE THE PAGE'S FACE ON ITS OWN, and one told the
  // proportional face by name stays proportional when a reader chooses the
  // monospaced one for the interface (`NOCTUA.md §7.7`).
  await open(page);
  await page.evaluate(() => document.documentElement.setAttribute("data-noctua-font", "mono"));
  const faces = await page.locator(".nd-key-capture-control").first().evaluate((el) => ({
    control: getComputedStyle(el).fontFamily,
    page: getComputedStyle(document.body).fontFamily,
  }));
  expect(faces.control, "the control keeps a face of its own").toBe(faces.page);
});
