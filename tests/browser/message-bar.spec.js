// The bar that says what happened, and the two ways it can say it.
//
// WHAT IS BEING GUARDED. `role="alert"` cuts a screen reader off mid-sentence.
// That is correct about a refused payment and rude about a saved record, so the
// default is the quiet one and the loud one is a prop — and a default is
// exactly the kind of thing that drifts without a test naming it.
//
// The dismissal is here for `Info`'s reason: the control ships hidden and
// `dist/message-bar.js` reveals it, so a page whose script never ran carries no
// button that closes nothing. Both halves of that are asserted.

import { expect, test } from "@playwright/test";

import { PAGE, url, showParts } from "./pages.js";

async function bars(page) {
  await page.goto(url(PAGE.messageBar));
  await showParts(page);
  return page.locator(".nd-message-bar");
}

test("a bar is a status until a caller asks for the rude one", async ({ page }) => {
  const all = await bars(page);
  expect(await all.count()).toBeGreaterThan(0);

  // THE ROLE IS THE WHOLE OF WHAT `alert` DECIDES, and it is what assistive
  // technology acts on. This used to read the prop back out of a
  // `data-nd-alert` attribute that both bodies wrote and nothing else read —
  // so the assertion compared the component's output with the component's
  // output, and would have passed with the role wired to nothing.
  const roles = await all.evaluateAll((els) => els.map((el) => el.getAttribute("role")));
  for (const role of roles) {
    expect(role, "a bar with neither role announces nothing").toMatch(/^(status|alert)$/);
  }
  // Both are demonstrated, or the claim is untested on this page.
  expect(roles.includes("status")).toBe(true);
  expect(roles.includes("alert")).toBe(true);
});

test("aria-live is absent unless a caller wrote it", async ({ page }) => {
  const all = await bars(page);
  const live = await all.evaluateAll((els) =>
    els.map((el) => ({ role: el.getAttribute("role"), live: el.getAttribute("aria-live") })),
  );
  // `status` already implies polite and `alert` already implies assertive, so
  // an attribute written at a default would be the component overruling the
  // role it just chose. `role="alert" aria-live="polite"` is an alert that
  // never interrupts, and it is silent about being one.
  expect(
    live.filter((one) => one.role === "alert" && one.live === "polite"),
    "an alert was quietly downgraded to polite",
  ).toEqual([]);
  expect(live.some((one) => one.live === null), "every bar overrides its own role").toBe(true);
});

test("the icon repeats the words rather than replacing them", async ({ page }) => {
  const all = await bars(page);
  const icons = page.locator(".nd-message-bar-icon");
  expect(await icons.count()).toBeGreaterThan(0);
  // `aria-hidden`, because it says what the sentence says. A glyph a screen
  // reader announces beside the sentence it draws is the sentence twice.
  const heard = await icons.evaluateAll(
    (els) => els.filter((el) => el.getAttribute("aria-hidden") !== "true").length,
  );
  expect(heard).toBe(0);

  // AND THE TONE IS NEVER THE ONLY SIGNAL (`NOCTUA.md §8.8.6`). Every toned bar
  // has words in it, and the ones that carry a tone also carry a drawing.
  const silent = await all.evaluateAll((els) =>
    els
      .filter((el) => el.dataset.ndTone)
      .filter((el) => !el.querySelector(".nd-message-bar-body")?.textContent?.trim())
      .map((el) => el.dataset.ndTone),
  );
  expect(silent, "a bar is saying it in colour alone").toEqual([]);
});

test("the title is a strong and never a heading", async ({ page }) => {
  await bars(page);
  const titles = page.locator(".nd-message-bar-title");
  expect(await titles.count()).toBeGreaterThan(0);
  // A message that arrives after the page has loaded must not rewrite the
  // outline under a reader who is navigating by it.
  const shapes = await titles.evaluateAll((els) => [...new Set(els.map((el) => el.tagName))]);
  expect(shapes).toEqual(["STRONG"]);
});

test("with the script blocked there is no control that closes nothing", async ({ browser }) => {
  const context = await browser.newContext({ javaScriptEnabled: false });
  const page = await context.newPage();
  await page.goto(url(PAGE.messageBar));
  await showParts(page);
  const control = page.locator(".nd-message-bar-dismiss").first();
  await expect(control).toBeAttached();
  await expect(control).toBeHidden();
  await context.close();
});

test("the script reveals the control, and pressing it takes the bar away", async ({ page }) => {
  await bars(page);
  const bar = page.locator(".nd-message-bar[data-nd-dismissible='true']").first();
  const control = bar.locator(".nd-message-bar-dismiss");
  await expect(control).toBeVisible();
  expect(await control.evaluate((el) => el.tagName)).toBe("BUTTON");
  expect(await control.getAttribute("type")).toBe("button");
  // A glyph, so the name is the only thing a screen reader has.
  expect(await control.getAttribute("aria-label")).toMatch(/\S\s\S/);
  // 24px is the AA floor for a target.
  const box = await control.boundingBox();
  expect(box.width).toBeGreaterThanOrEqual(24);
  expect(box.height).toBeGreaterThanOrEqual(24);

  await control.click();
  await expect(bar).toBeHidden();
  // THE ELEMENT STAYS. A live region taken out of the document is a region
  // that announces nothing the next time a caller writes into it.
  await expect(bar).toBeAttached();
  expect(await bar.getAttribute("data-nd-dismissed")).toBe("");
  // AND FOCUS IS NOT LEFT WHERE THE READER CANNOT SEE IT. The control they
  // pressed is inside what they just dismissed, so focus lands on the box the
  // bar was in rather than on `<body>`, where the next Tab would restart at
  // the top of the document.
  const landed = await page.evaluate(() => {
    const active = document.activeElement;
    if (!active || active === document.body) return "body";
    if (active.classList.contains("nd-message-bar-dismiss")) return "the control";
    return "somewhere visible";
  });
  expect(landed).toBe("somewhere visible");
});

test("it is a bordered block and never a card", async ({ page }) => {
  const all = await bars(page);
  const wrong = await all.evaluateAll((els) =>
    els
      .map((el) => getComputedStyle(el))
      .filter((style) => style.boxShadow !== "none" || style.borderInlineStartWidth === "0px")
      .map((style) => style.boxShadow),
  );
  // `NOCTUA.md §9.7`: separation is a hairline plus a surface colour, and the
  // rule down the leading edge is what tells one bar from the page around it.
  expect(wrong, "a message bar is drawing an elevated surface").toEqual([]);
});
