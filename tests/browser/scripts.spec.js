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
  await page.goto("/en/typography/");
  await page.waitForLoadState("networkidle");

  const samples = await page.locator("[data-nd-script]").evaluateAll((els) =>
    els.map((el) => ({
      name: el.dataset.ndScript,
      text: el.querySelector(".nd-script-sample").textContent,
    })),
  );
  expect(samples.length, "the script table did not render").toBe(14);

  // Every one of the fourteen, with no exemptions. Until the eleven script
  // faces were subset into assets/fonts/ this loop ran over four of them and
  // the other ten were a documented gap; that gap is closed, so the loop is
  // the whole table again.
  for (const sample of samples) {
    const real = await widthIn(page, sample.text, "var(--noctua-font-sans)");
    expect(real, `${sample.name} rendered nothing at all`).toBeGreaterThan(0);

    // `document.fonts.check` is the question actually being asked: can a loaded
    // face render this text. Width alone cannot answer it for CJK, because an
    // ideograph is exactly 1em wide and so is the browser's tofu box — the two
    // measure identical, and an earlier version of this test called a correctly
    // rendered Chinese line tofu on that basis.
    const covered = await page.evaluate(
      ([text]) => {
        const stack = getComputedStyle(document.body).fontFamily;
        return stack
          .split(",")
          .map((f) => f.trim())
          .some((family) => {
            try {
              return document.fonts.check(`64px ${family}`, text); // allow-literal: a measuring size for the coverage API, not a design decision
            } catch {
              return false;
            }
          });
      },
      [sample.text],
    );
    expect(covered, `${sample.name} has no loaded face that can render it`).toBe(true);

    // Width still guards the proportional scripts, where tofu and real glyphs
    // genuinely differ. CJK is exempt because 1em is the correct width for
    // both, not because it is allowed to fail.
    if (!/Japanese|Korean|Chinese/.test(sample.name)) {
      const tofu = await widthIn(page, sample.text, '"__nothing_at_all__"');
      expect(
        Math.abs(real - tofu),
        `${sample.name} rendered at exactly the fallback width — it is tofu`,
      ).toBeGreaterThan(1);
    }
  }
});

test("every script has a face of its own", async ({ page }) => {
  await page.goto("/en/typography/");
  await page.waitForLoadState("networkidle");

  // The assertion that used to list ten missing families. It lists none now,
  // and it fails the moment a script is added to the table without a face —
  // which is the failure mode this replaces, not a weaker version of it.
  const unfaced = await page.evaluate(() => {
    const stack = getComputedStyle(document.body).fontFamily;
    const needed = {
      Arabic: "Noto Sans Arabic", // allow-literal: the family this script must resolve to — the assertion itself
      Hebrew: "Noto Sans Hebrew", // allow-literal: the family this script must resolve to — the assertion itself
      Devanagari: "Noto Sans Devanagari", // allow-literal: the family this script must resolve to — the assertion itself
      Thai: "Noto Sans Thai", // allow-literal: the family this script must resolve to — the assertion itself
      Ethiopic: "Noto Sans Ethiopic", // allow-literal: the family this script must resolve to — the assertion itself
      Georgian: "Noto Sans Georgian", // allow-literal: the family this script must resolve to — the assertion itself
      Armenian: "Noto Sans Armenian", // allow-literal: the family this script must resolve to — the assertion itself
      Japanese: "Noto Sans JP", // allow-literal: the family this script must resolve to — the assertion itself
      Korean: "Noto Sans KR", // allow-literal: the family this script must resolve to — the assertion itself
      "Simplified Chinese": "Noto Sans SC", // allow-literal: the family this script must resolve to — the assertion itself
    };
    return Object.entries(needed)
      .filter(([, family]) => !stack.includes(family))
      .map(([script]) => script);
  });

  expect(unfaced, "a script on the page has no family in the stack").toEqual([]);
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
  await page.goto("/en/typography/");
  await page.waitForLoadState("networkidle");
  expect(failures, "a font file 404ed").toEqual([]);
});

test("with Iosevka selected, no component renders a line in two typefaces", async ({ page }) => {
  await page.goto("/en/typography/");
  await page.evaluate(() => document.documentElement.setAttribute("data-noctua-font", "mono"));
  await page.waitForLoadState("networkidle");

  const rows = await page.locator("[data-nd-script]").evaluateAll((els) =>
    els.map((el) => ({
      name: el.dataset.ndScript,
      monospaceable: el.querySelector(".nd-script-sample").dataset.ndMonospaceable === "true",
    })),
  );

  // Four of the fourteen are covered by Iosevka. For the rest the WHOLE
  // component must move to the proportional face — a per-glyph fallback would
  // put two x-heights on one baseline, which reads as a rendering bug.
  expect(rows.filter((r) => r.monospaceable).length).toBe(4);
  expect(rows.length).toBe(14);
});

test("a CJK page fetches a handful of slices, not the whole family", async ({ page }) => {
  const fetched = [];
  page.on("response", async (response) => {
    const name = response.url().split("/").pop();
    if (!name?.endsWith(".woff2")) return;
    const length = Number(response.headers()["content-length"] ?? 0);
    fetched.push({ name, bytes: length });
  });

  await page.goto("/en/typography/");
  await page.waitForLoadState("networkidle");

  const cjk = fetched.filter((f) => /NotoSans(JP|KR|SC)-/.test(f.name));
  const bytes = cjk.reduce((sum, f) => sum + f.bytes, 0);

  expect(cjk.length, "no CJK slice was fetched at all").toBeGreaterThan(0);

  // BYTES, not slice count. Three lines of CJK draw characters scattered right
  // across the ideograph block, so they legitimately touch dozens of the 134
  // slices — counting files makes that look like a failure when it is the
  // mechanism working. What matters is the weight: whole, the three families
  // were 13.6 MB every reader of this page waited for.
  expect(
    bytes,
    `fetched ${(bytes / 1_000_000).toFixed(1)} MB across ${cjk.length} slices, against 13.6 MB whole`,
  ).toBeLessThan(4_000_000);

  // And no monolithic file survived the change.
  const whole = fetched.filter((f) => /^NotoSans(JP|KR|SC)\.woff2$/.test(f.name));
  expect(whole, "an unsliced CJK family is still being served").toEqual([]);
});

test("a Latin page loads no non-Latin face", async ({ page }) => {
  const fonts = [];
  page.on("request", (request) => {
    if (request.url().endsWith(".woff2")) fonts.push(request.url().split("/").pop());
  });
  // The spacing page is Latin only. Loading a CJK face for it would be tens of
  // megabytes nobody on that page needs.
  await page.goto("/en/spacing/");
  await page.waitForLoadState("networkidle");

  // Eleven script faces are declared and 14.5 MB sit in assets/fonts/. A Latin
  // page must fetch none of them — which is the whole point of pairing
  // `unicode-range` with `preload = false`, and is now actually testable
  // because the files exist to be wrongly fetched.
  // Including every one of the 134 slices: a slice with a wrong range would
  // be fetched here, and this is what catches it.
  const nonLatin = fonts.filter((f) =>
    /JP|KR|SC|TC|Arabic|Hebrew|Devanagari|Thai|Ethiopic|Georgian|Armenian|Mongolian/i.test(f),
  );
  expect(nonLatin, `a Latin-only page fetched ${nonLatin.join(", ")}`).toEqual([]);
});
