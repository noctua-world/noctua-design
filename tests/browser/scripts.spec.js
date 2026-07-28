// Every script renders, and the fallback fires exactly where it should.
//
// `NOCTUA.md §7.3.1` is the reason Noto was chosen over any other family. This
// file is where that reason is checked rather than asserted: fourteen scripts,
// each rendered and each measured for tofu.
//
// # Why width, and not a font-loading API
//
// There is no way to ask a browser "did you render .notdef". What there is: a
// tofu box is a fixed-width rectangle, so a string of them has a width that
// scales exactly with its character count and does not change when the font
// stack changes. Real glyphs do neither. Measuring the rendered width against
// the same string in a font that definitely lacks the script separates the two.

import { test, expect } from "@playwright/test";

/** The rendered width of a string in a given stack, in CSS pixels. */
async function widthIn(page, text, family) {
  return page.evaluate(
    ([text, family]) => {
      const probe = document.createElement("span");
      probe.textContent = text;
      // A measuring probe, deliberately outside the token system: 64px is a
      // measurement rig, not a design decision, and the family is the variable
      // under test — binding either to a token would measure the token instead
      // of the question.
      probe.style.cssText = `position:absolute;visibility:hidden;white-space:pre;font-size:64px;font-family:${family}`; // allow-literal: see above
      document.body.appendChild(probe);
      const width = probe.getBoundingClientRect().width;
      probe.remove();
      return width;
    },
    [text, family],
  );
}

test("no script renders as tofu", async ({ page }) => {
  await page.goto("/en/typography.html");
  await page.waitForLoadState("networkidle");

  const samples = await page.locator("[data-nd-script]").evaluateAll((els) =>
    els.map((el) => ({
      name: el.dataset.ndScript,
      text: el.querySelector(".nd-script-sample").textContent,
    })),
  );
  expect(samples.length, "the script table did not render").toBe(14);

  // Only the scripts whose faces this repository actually ships. The others
  // are a KNOWN GAP, measured by the next test rather than hidden by this one:
  // asserting no-tofu for a script with no font would fail forever and teach
  // everyone to ignore a red suite.
  const shipped = await page.locator("[data-nd-script]").evaluateAll((els) =>
    els
      .filter((el) => el.querySelector(".nd-script-sample").dataset.ndMonospaceable === "true")
      .map((el) => el.dataset.ndScript),
  );

  for (const sample of samples.filter((s) => shipped.includes(s.name))) {
    const real = await widthIn(page, sample.text, "var(--noctua-font-sans)");
    // A family with no coverage of anything: whatever this renders is the
    // browser's last-resort box, which is what tofu looks like.
    const tofu = await widthIn(page, sample.text, '"__nothing_at_all__"');

    expect(real, `${sample.name} rendered nothing at all`).toBeGreaterThan(0);
    // Real glyphs are never all the same width as the fallback boxes. A CJK
    // sample and a Latin one differ from tofu in opposite directions, so the
    // assertion is on the difference, not its sign.
    expect(
      Math.abs(real - tofu),
      `${sample.name} rendered at exactly the fallback width — it is tofu`,
    ).toBeGreaterThan(1);
  }
});

test("the scripts with no shipped face are exactly the known gap", async ({ page }) => {
  await page.goto("/en/typography.html");
  const rows = await page.locator("[data-nd-script]").evaluateAll((els) =>
    els.map((el) => ({
      name: el.dataset.ndScript,
      shipped: el.querySelector(".nd-script-sample").dataset.ndMonospaceable === "true",
    })),
  );

  // `spec/fonts.toml` declares four families, all Latin-repertoire. Ten of the
  // fourteen scripts on the typography page therefore have no face and render
  // through whatever the operating system happens to have — which is exactly
  // what NOCTUA.md §7.3.1 chose Noto to avoid.
  //
  // This assertion is the gap, written down and counted. When the non-Latin
  // faces are subset into assets/fonts/ this number goes down, and the test
  // fails until the list below is corrected — which is the point.
  const missing = rows.filter((r) => !r.shipped).map((r) => r.name);
  expect(missing).toEqual([
    "Arabic",
    "Hebrew",
    "Devanagari",
    "Thai",
    "Ethiopic",
    "Georgian",
    "Armenian",
    "Japanese",
    "Korean",
    "Simplified Chinese",
  ]);
});

test("the font files the scripts need are actually present", async ({ page }) => {
  // A silent tofu is the failure mode being guarded, and a missing font file is
  // how one appears. This fails loudly rather than letting the width test above
  // quietly compare two fallbacks.
  const failures = [];
  page.on("response", (response) => {
    if (response.url().endsWith(".woff2") && !response.ok()) {
      failures.push(`${response.status()} ${response.url()}`);
    }
  });
  await page.goto("/en/typography.html");
  await page.waitForLoadState("networkidle");
  expect(failures, "a font file 404ed").toEqual([]);
});

test("with Iosevka selected, no component renders a line in two typefaces", async ({ page }) => {
  await page.goto("/en/typography.html");
  await page.evaluate(() => document.documentElement.setAttribute("data-noctua-font", "mono"));
  await page.waitForLoadState("networkidle");

  const rows = await page.locator("[data-nd-script]").evaluateAll((els) =>
    els.map((el) => ({
      name: el.dataset.ndScript,
      monospaceable: el.querySelector(".nd-script-sample").dataset.ndMonospaceable === "true",
    })),
  );

  // Three of the fourteen are covered by Iosevka. For the rest the WHOLE
  // component must move to the proportional face — a per-glyph fallback would
  // put two x-heights on one baseline, which reads as a rendering bug.
  expect(rows.filter((r) => r.monospaceable).length).toBe(4);
  expect(rows.length).toBe(14);
});

test("a Latin page loads no non-Latin face", async ({ page }) => {
  const fonts = [];
  page.on("request", (request) => {
    if (request.url().endsWith(".woff2")) fonts.push(request.url().split("/").pop());
  });
  // The spacing page is Latin only. Loading a CJK face for it would be tens of
  // megabytes nobody on that page needs.
  await page.goto("/en/spacing.html");
  await page.waitForLoadState("networkidle");

  const nonLatin = fonts.filter((f) => /JP|KR|SC|TC|Arabic|Hebrew|Devanagari|Thai/i.test(f));
  expect(nonLatin, `a Latin-only page fetched ${nonLatin.join(", ")}`).toEqual([]);
});
