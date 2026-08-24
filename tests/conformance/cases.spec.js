// One case set, two implementations, one expectation — `NOCTUA.md §9.3` Leg 3.
//
// The cases are `spec/components/<name>.cases.toml`, emitted to
// `dist/component-cases.json`; the page that mounts them twice is
// `docs-site/dist/conformance/index.html`, built by `cargo xtask docs`.
//
// WHY BOTH HALVES ARE CHECKED AGAINST THE FILE RATHER THAN AGAINST EACH OTHER.
//
// Two implementations that agree with each other and disagree with the
// specification is the failure that actually happens, and a diff between them
// cannot see it. So each half answers to the case, and THEN they are diffed —
// the diff catches what nobody thought to write down, which is a different job.
//
// WHY A MISSING REACT BUNDLE IS NOT A SKIP.
//
// It is reported, once, with the number of cases that went unchecked on that
// side. A suite that quietly halves its coverage when an optional artifact is
// absent is a suite that is green for the wrong reason (`NOCTUA.md §19.3`).

import { readFileSync } from "node:fs";

import { expect, test } from "@playwright/test";

const CASES = JSON.parse(readFileSync("dist/component-cases.json", "utf8")).case;

// THE PLATFORM'S BOOLEAN ATTRIBUTES, WHERE PRESENCE IS THE STATE.
//
// Dioxus serialises `disabled` as `disabled="true"` and React as `disabled=""`,
// and neither is wrong: for these attributes HTML says the value is ignored and
// presence alone is the state — `element.disabled`, `:disabled` and
// `[disabled]` agree in both. So this is the one thing the diff below
// canonicalises, and it canonicalises it because comparing the serialisation
// would be comparing something neither implementation promises. Anything else
// that differs is a divergence and fails.
const BOOLEAN = new Set([
  "checked", "disabled", "hidden", "multiple", "open", "readonly", "required", "selected",
]);
const PAGE = "http://127.0.0.1:8731/conformance/";

/** The element a case is about: what the implementation drew, first child. */
const rootOf = (page, id, impl) =>
  page.locator(`[data-nd-case="${id}"] [data-nd-impl="${impl}"] > *`).first();

/**
 * Takes the other half off the page before driving this one.
 *
 * BOTH HALVES DRAW THE SAME IDS, because they are the same component called the
 * same way — so a `Dialog`'s trigger in one half opened the other half's dialog
 * by `getElementById`, and Escape then closed something the case was not
 * looking at. Removing the twin is what makes each half the only answer to its
 * own ids; the shape comparison runs on its own load, with both present.
 */
async function alone(page, id, impl) {
  await page.evaluate(
    ([caseId, keep]) => {
      for (const half of document.querySelectorAll(`[data-nd-case="${caseId}"] [data-nd-impl]`)) {
        if (half.dataset.ndImpl !== keep) half.remove();
      }
    },
    [id, impl],
  );
}

async function run(page, root, steps) {
  // WHERE THE KEY IS PRESSED IS AN ELEMENT, NOT A HOPE.
  //
  // `page.keyboard.press` sends the key wherever the focus happens to be, and
  // under parallel load the focus a step asked for had not always landed — so
  // cases failed at random and said nothing about the component.
  //
  // AND IT IS NOT THE ELEMENT THE LAST STEP NAMED, EITHER. A component may move
  // the focus itself, and several do on purpose: typing in an `OtpField` moves
  // to the next box, which is the whole gesture. So a key goes to whatever
  // holds the focus NOW — waited for, rather than assumed.
  const focused = () => page.locator("*:focus").first();
  for (const step of steps) {
    const [head, rest] = step.includes(":")
      ? [step.slice(0, step.indexOf(":")), step.slice(step.indexOf(":") + 1)]
      : [step, null];
    if (head === "focus" && rest === null) await root.focus();
    else if (head === "focus") await root.locator(rest).first().focus();
    else if (head === "click" && rest === null) await press(root);
    else if (head === "click") await press(root.locator(rest).first());
    else if (head === "key") await focused().press(rest);
    else if (head === "type") await focused().pressSequentially(rest);
    else if (head === "blur") await page.evaluate(() => document.activeElement?.blur());
    else throw new Error(`no step is written \`${step}\``);
  }
}

/**
 * Presses something, including something that refuses.
 *
 * `aria-disabled` is a REFUSAL A READER CAN STILL REACH — that is the whole
 * reason components here write it instead of `disabled` — so a case about
 * pressing one has to be able to press it. Playwright's own actionability
 * check treats it as disabled and waits for ever, which would make the case
 * time out rather than fail, and a timeout says nothing about the component.
 */
async function press(locator) {
  const refused = await locator.evaluate((element) => {
    if (element.getAttribute("aria-disabled") === "true") return true;
    if (element.matches(":disabled")) return true;
    // A LABEL IS AS REFUSED AS WHAT IT LABELS. Pressing the words is the
    // gesture a reader makes on a checkbox, and Playwright resolves a label to
    // its control before deciding whether it can be pressed — so a case about
    // a disabled box pressing its label waits for ever on a control that will
    // never be enabled.
    return element.control?.disabled ?? false;
  });
  await locator.click({ force: refused });
}

async function assert(page, root, expected, where) {
  // EVERY EXPECTATION RETRIES, because some of what a case asserts arrives on
  // an event rather than on the keystroke: the platform closes a `<dialog>` and
  // `dialog.js` takes the state off on `close`, which is a turn later. Reading
  // an attribute once made those cases fail on a machine under load and pass on
  // an idle one, which is the worst kind of test.
  for (const [name, value] of Object.entries(expected.attrs ?? {})) {
    if (BOOLEAN.has(name)) {
      await expect(root, `${where}: ${name} is written`).toHaveAttribute(name);
    } else {
      await expect(root, `${where}: ${name}`).toHaveAttribute(name, value);
    }
  }
  for (const name of expected.absent ?? []) {
    await expect(root, `${where}: ${name} must not be written`).not.toHaveAttribute(name);
  }
  for (const selector of expected.visible ?? []) {
    await expect(root.locator(selector).first(), `${where}: ${selector}`).toBeVisible();
  }
  for (const selector of expected.present ?? []) {
    await expect(root.locator(selector).first(), `${where}: ${selector}`).toBeAttached();
  }
  for (const selector of expected.hidden ?? []) {
    await expect(root.locator(selector).first(), `${where}: ${selector}`).toBeHidden();
  }
  if (expected.value !== undefined) {
    await expect
      .poll(
        () =>
          root.evaluate((element) => {
            const control = element.matches("input, textarea, select")
              ? element
              : element.querySelector("input, textarea, select");
            return control ? control.value : null;
          }),
        { message: `${where}: the control holds` },
      )
      .toBe(expected.value);
  }
  if (expected.text !== undefined) {
    await expect
      .poll(async () => (await root.textContent())?.replace(/\s+/g, " ").trim(), {
        message: `${where}: text`,
      })
      .toBe(expected.text);
  }
  if (expected.focus !== undefined) {
    await expect
      .poll(
        () =>
          root.evaluate((element, selector) => {
            const active = document.activeElement;
            if (selector === "none") return active === document.body || active === null;
            if (selector === "root") return active === element;
            return element.querySelector(selector) === active;
          }, expected.focus),
        { message: `${where}: the focus is on \`${expected.focus}\`` },
      )
      .toBe(true);
  }
}

/** What both halves have to produce: the shape, not the text nodes' whitespace. */
const shapeOf = (impl) =>
  impl.evaluate((element, boolean) => {
    const BOOLEAN = new Set(boolean);
    const walk = (node) => {
      if (node.nodeType === Node.TEXT_NODE) {
        const text = node.textContent.replace(/\s+/g, " ").trim();
        return text ? [text] : [];
      }
      if (node.nodeType !== Node.ELEMENT_NODE) return [];
      // A `style` is its DECLARATIONS, not its text. Dioxus writes
      // `--x:1` and React writes `--x: 1;` — the same declaration, spelt by two
      // serialisers, and comparing the strings would report a divergence
      // neither implementation is making. Anything else is compared verbatim.
      const declarations = (text) =>
        text
          .split(";")
          .map((one) => one.trim().replace(/\s*:\s*/, ":"))
          .filter(Boolean)
          .sort()
          .join(";");
      const attrs = [...node.attributes]
        .map((a) =>
          BOOLEAN.has(a.name)
            ? a.name
            : `${a.name}=${a.name === "style" ? declarations(a.value) : a.value}`,
        )
        .filter((a) => !a.startsWith("id=") && !a.startsWith("aria-controls="))
        .sort()
        .join(" ");
      return [`<${node.tagName.toLowerCase()} ${attrs}>`, ...[...node.childNodes].flatMap(walk)];
    };
    return [...element.childNodes].flatMap(walk);
  }, [...BOOLEAN]);

test.describe("every component answers to its own cases", () => {
  for (const one of CASES) {
    test(`${one.component}: ${one.name}`, async ({ page }) => {
      // One case per load: the behaviours are delegated and several are
      // document-wide, so a page holding all of them is a page where one
      // case's key press reaches another case's handler.
      await page.goto(`${PAGE}?only=${one.id}`);
      await page.waitForFunction(() => document.documentElement.dataset.ndReact !== undefined);
      const react = await page.evaluate(() => document.documentElement.dataset.ndReact);

      const dioxus = rootOf(page, one.id, "dioxus");
      await expect(dioxus, "the Dioxus half is on the page").toBeAttached();
      await alone(page, one.id, "dioxus");
      await run(page, dioxus, one.steps);
      await assert(page, dioxus, one.expect, `${one.id} dioxus`);

      if (react !== "mounted") {
        test.info().annotations.push({
          type: "issue",
          description: "the React bundle is absent, so only the Dioxus half was checked",
        });
        return;
      }

      // A second load, so the React half starts from the same place the Dioxus
      // half did rather than from whatever the steps above left behind.
      await page.goto(`${PAGE}?only=${one.id}`);
      await page.waitForFunction(() => document.documentElement.dataset.ndReact === "mounted");
      const mounted = rootOf(page, one.id, "react");
      await expect(mounted, "the React half is on the page").toBeAttached();
      await alone(page, one.id, "react");
      await run(page, mounted, one.steps);
      await assert(page, mounted, one.expect, `${one.id} react`);
    });
  }
});

test("the two implementations draw the same thing, before anything is done to them", async ({
  page,
}) => {
  // `?raw`: both halves mounted, no behaviour turned on. What is compared is
  // what each implementation DREW — an enhancer rearranges what it finds, and
  // comparing after they run compares two scripts rather than two renderings.
  await page.goto(`${PAGE}?raw`);
  await page.waitForFunction(() => document.documentElement.dataset.ndReact !== undefined);
  const react = await page.evaluate(() => document.documentElement.dataset.ndReact);
  test.skip(react !== "mounted", "the React bundle is absent");

  let compared = 0;
  for (const one of CASES) {
    const left = await shapeOf(page.locator(`[data-nd-case="${one.id}"] [data-nd-impl="dioxus"]`));
    const right = await shapeOf(page.locator(`[data-nd-case="${one.id}"] [data-nd-impl="react"]`));
    expect(right, `${one.component}: ${one.name}`).toEqual(left);
    compared += 1;
  }
  // A comparison that compared nothing passes for ever.
  expect(compared, "no case was compared").toBeGreaterThan(0);
});
