// The JavaScript half of the syntax scanner, against the same expectation the
// Rust half is held to.
//
// THE HOLE THIS FILLS, AND THE TEST IT REFUSES TO BE.
//
// There are two scanners: `crates/noctua-design-grammars/src/scan.rs`, which
// runs during SSR, and its twin `scan.js`, which runs in a browser for the
// React package. They must agree, and the obvious way to check that — render
// the same code with both and compare — **passes whenever they are both wrong
// in the same way**. That is the failure that actually happens: one shared
// misreading of a grammar file, transcribed twice.
//
// So neither is compared to the other. Both are compared to
// `dist/grammars.cases.json`, which is emitted from `spec/grammars/` alongside
// the Rust `CASES` table by one function in one emitter. A disagreement names
// which half moved.
//
// This spec runs in Node rather than in a page, which is why it takes no
// `page` fixture: `packages/tokens/grammars.js` is a plain ES module with no
// DOM in it, and loading a page to test a pure function would only add ways for
// the test to fail for reasons that are not about the scanner.

import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

import { expect, test } from "@playwright/test";

import { GRAMMARS, ROLES, scan } from "../../packages/tokens/grammars.js";

const root = fileURLToPath(new URL("../..", import.meta.url));
const FIXTURE = JSON.parse(readFileSync(`${root}dist/grammars.cases.json`, "utf8"));

/** `<role> <text>`, split at the FIRST space so the text may be a space. */
function pair(entry) {
  const at = entry.indexOf(" ");
  return [entry.slice(0, at), entry.slice(at + 1)];
}

/** The tokens `language` makes of `code`, rebuilt into the input. */
function rebuild(language, code) {
  return scan(GRAMMARS[language], code)
    .map((token) => token.text)
    .join("");
}

test("the fixture reaches this file at all", () => {
  // A fixture that failed to load would make every test below vacuously true,
  // which is the one way a parity suite can be worse than no parity suite.
  expect(FIXTURE.cases.length).toBeGreaterThan(10);
  expect(Object.keys(GRAMMARS).length).toBeGreaterThan(2);
});

test("concatenating the tokens reproduces the input", () => {
  // THE INVARIANT EVERYTHING ELSE RESTS ON, and the reason an imperfect
  // highlighter is safe: every miss degrades to `plain` and nothing can be
  // hidden, reordered or dropped.
  for (const { language, name, code } of FIXTURE.cases) {
    expect(rebuild(language, code), `${language} lost bytes in \`${name}\``).toBe(code);
  }
});

test("every grammar survives every other language", () => {
  // What a reader does the first time they mislabel a block. It must not
  // panic, loop or drop anything.
  for (const { code, name } of FIXTURE.cases) {
    for (const language of Object.keys(GRAMMARS)) {
      expect(rebuild(language, code), `${language} lost bytes reading \`${name}\``).toBe(code);
    }
  }
});

test("a truncated input still terminates and keeps every byte", () => {
  // Every prefix of every case: the deterministic form of "what happens inside
  // an unterminated construct". No seed, no flake, and it covers the truncated
  // string, comment and tag without anybody having to think of them.
  for (const { language, name, code } of FIXTURE.cases) {
    for (let at = 0; at <= code.length; at += 1) {
      // Never cut a surrogate pair in half: the Rust half skips indices that
      // are not character boundaries, and cutting one here would compare two
      // different inputs.
      const cut = code.charCodeAt(at - 1);
      if (cut >= 0xd800 && cut <= 0xdbff) continue;
      const piece = code.slice(0, at);
      expect(rebuild(language, piece), `${language} lost bytes in \`${name}\`[..${at}]`).toBe(
        piece,
      );
    }
  }
});

test("every case gets what it expects and nothing it refuses", () => {
  for (const { language, name, code, expect: wanted, reject } of FIXTURE.cases) {
    const tokens = scan(GRAMMARS[language], code);

    let at = 0;
    for (const entry of wanted) {
      const [role, text] = pair(entry);
      const found = tokens.slice(at).findIndex((t) => t.role === role && t.text === text);
      expect(
        found,
        `${language}: \`${name}\` expected \`${entry}\`; the tokens were ` +
          JSON.stringify(tokens),
      ).toBeGreaterThan(-1);
      at += found + 1;
    }

    for (const entry of reject) {
      const [role, text] = pair(entry);
      expect(
        tokens.some((t) => t.role === role && t.text === text),
        `${language}: \`${name}\` produced \`${entry}\`, which it refuses`,
      ).toBe(false);
    }
  }
});

test("no token carries a role the stylesheet does not know about", () => {
  // The other half of the theme's completeness check. `spec/syntax.toml` is
  // validated against the role list at build time; this asserts the scanner
  // never invents one outside it, which is what would leave a run of code
  // wearing the colour of whatever contains it.
  for (const { language, code } of FIXTURE.cases) {
    for (const language_ of Object.keys(GRAMMARS)) {
      for (const token of scan(GRAMMARS[language_], code)) {
        expect(ROLES, `${language_} on ${language}'s code`).toContain(token.role);
      }
    }
  }
});
