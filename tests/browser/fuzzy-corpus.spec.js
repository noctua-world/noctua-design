// The matcher held to the fuzzy core's corpus, row by row.
//
// `assets/vendor/nc-fuzzy/fuzzy-corpus.tsv` is the rule written as data: what
// each character folds to, how a query splits into words, how a query matches
// an item and in what order a list comes back. `noctua-central`'s `nc-fuzzy`
// crate replays the same rows in its own suite, so the two implementations are
// each held to one case file and never to each other: comparing them would
// pass whenever both were wrong the same way.
//
// NO BROWSER IS OPENED. The matcher touches no DOM, so this file imports the
// module that ships, `dist/search-match.js`, and calls it. It lives in this
// directory because this is where the repository's JavaScript is run, by
// `cargo xtask docs --verify`.
//
// THE READER IS STRICT ON PURPOSE. A row it cannot read fails naming its line
// rather than being skipped: a corpus whose malformed rows vanish is a corpus
// that shrinks while every test stays green. And every failure names the line
// of the row that differs, because the row is the claim about the rule.
//
// Each test was confirmed by restoring a defect: a rule difference planted in
// the matcher (a name no longer ranked before a tag) and a row of the corpus
// edited, each of which fails here naming the rows it moved.

import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

import { expect, test } from "@playwright/test";

import {
  FIELD_CHARS_MAX,
  Item,
  Matcher,
  QUERY_CHARS_MAX,
  Query,
  QueryTooLong,
  Role,
  Score,
  WORDS_MAX,
  fold,
} from "../../dist/search-match.js";

const CORPUS = readFileSync(
  fileURLToPath(new URL("../../assets/vendor/nc-fuzzy/fuzzy-corpus.tsv", import.meta.url)),
  "utf8",
);

/** How many columns follow the kind, for each kind of row the corpus has. */
const COLUMNS = { fold: 2, words: 2, match: 3, order: 3 };

/**
 * A column with every escape written out as its character.
 *
 * A backslash is read only as a code point in braces; any other is a row the
 * reader cannot read, and throws rather than standing for itself.
 */
function unescaped(column, line) {
  let out = "";
  let rest = column;
  for (let at = rest.indexOf("\\"); at >= 0; at = rest.indexOf("\\")) {
    out += rest.slice(0, at);
    const escape = /^\\u\{([^}]*)\}/.exec(rest.slice(at));
    if (!escape) {
      throw new Error(`line ${line}: a backslash opens nothing but a code point in braces`);
    }
    const digits = escape[1];
    const point = /^[0-9A-Fa-f]{1,6}$/.test(digits) ? Number.parseInt(digits, 16) : -1;
    if (point < 0 || point > 0x10ffff || (point >= 0xd800 && point <= 0xdfff)) {
      throw new Error(`line ${line}: \`${digits}\` is not a character's code point`);
    }
    out += String.fromCodePoint(point);
    rest = rest.slice(at + escape[0].length);
  }
  return out + rest;
}

/** Every row of `corpus`: its line, its kind and its columns after the kind, unescaped. */
function rows(corpus = CORPUS) {
  const out = [];
  // What Rust's `lines()` yields: split at a line feed, a carriage return
  // before it dropped, and no last line for a text that ends in one.
  const lines = corpus.split("\n");
  if (lines[lines.length - 1] === "") lines.pop();
  lines.forEach((raw, index) => {
    const line = index + 1;
    const text = raw.endsWith("\r") ? raw.slice(0, -1) : raw;
    if (text.startsWith("#") || text.trim() === "") return;
    const [kind, ...columns] = text.split("\t").map((column) => unescaped(column, line));
    const wanted = COLUMNS[kind];
    if (!Object.hasOwn(COLUMNS, kind)) {
      throw new Error(`line ${line}: \`${kind}\` is not a kind of row this corpus has`);
    }
    if (columns.length !== wanted) {
      throw new Error(`line ${line}: a \`${kind}\` row holds ${wanted} columns after its kind`);
    }
    out.push({ line, kind, columns });
  });
  return out;
}

/** The rows of one kind. */
const ofKind = (kind) => rows().filter((row) => row.kind === kind);

/** An item from its column: fields joined by `|`, each a name unless it opens with `tag:`. */
function item(column) {
  return new Item(
    column
      .split("|")
      .map((field) =>
        field.startsWith("tag:") ? [Role.Tag, field.slice("tag:".length)] : [Role.Name, field],
      ),
  );
}

/** A position or a field index, which is digits and nothing else. */
function number(text, line, what) {
  if (!/^[0-9]+$/.test(text)) throw new Error(`line ${line}: \`${text}\` is not ${what}`);
  return Number(text);
}

/** The positions a hit column writes: indices and inclusive ranges joined by commas. */
function positions(written, line) {
  const out = [];
  for (const part of written.split(",")) {
    const dash = part.indexOf("-");
    if (dash < 0) {
      out.push(number(part, line, "a position"));
      continue;
    }
    const from = number(part.slice(0, dash), line, "a position");
    const to = number(part.slice(dash + 1), line, "a position");
    for (let at = from; at <= to; at += 1) out.push(at);
  }
  return out;
}

/** What a match row expects: each word's field and positions, or `null` for `-`. */
function expectedHits(column, line) {
  if (column === "-") return null;
  return column.split("|").map((hit) => {
    const colon = hit.indexOf(":");
    if (colon < 0) return [0, positions(hit, line)];
    return [number(hit.slice(0, colon), line, "a field"), positions(hit.slice(colon + 1), line)];
  });
}

/** A query from its column, which every row writes within the bound. */
function query(column, line) {
  try {
    return Query.parse(column);
  } catch (error) {
    if (error instanceof QueryTooLong) throw new Error(`line ${line}: the query is past its bound`);
    throw error;
  }
}

/** One value as the text a failure shows, so two that differ read differently. */
const shown = (value) => JSON.stringify(value);

test("the corpus reaches this file at all", () => {
  // Floors, not the numbers: a reader that stopped early, or a kind whose rows
  // all went, fails here rather than passing every test below.
  const all = rows();
  for (const [kind, floor] of [["fold", 20], ["words", 10], ["match", 50], ["order", 20]]) {
    const count = all.filter((row) => row.kind === kind).length;
    expect(count, `\`${kind}\` rows, and the corpus holds at least ${floor}`).toBeGreaterThanOrEqual(
      floor,
    );
  }
  // And the rows the acceptance of the rule names: accents both ways, a
  // Portuguese synonym found through a tag, an English name, a word that
  // matches nothing removing the item, and two words sharing characters.
  const has = (kind, asked, expected) =>
    all.some(
      (row) =>
        row.kind === kind && row.columns[0] === asked && row.columns.at(-1) === expected,
    );
  expect(has("match", "configuracoes", "0-12")).toBe(true);
  expect(has("match", "verificar historico", "0-8|12-20")).toBe(true);
  expect(has("match", "procurar comando", "1:0-7|13-19")).toBe(true);
  expect(has("order", "verify", "1")).toBe(true);
  expect(has("match", "verificar zzz", "-")).toBe(true);
  expect(has("match", "ver verificar", "0-2|0-8")).toBe(true);
  // AND THE FIVE ROWS THIS FILE ONCE ASSERTED ON ITS OWN, whole. Each is the
  // smallest input on which the matcher and one planted difference answer
  // apart, and each is replayed by the tests below now, so a corpus that lost
  // one would lose the only thing holding that part of the rule here.
  const holds = (...written) =>
    all.some((row) => shown([row.kind, ...row.columns]) === shown(written));
  // Earliness counts the characters before a match as written, separators included.
  expect(holds("order", "x", "-x;x", "1;0")).toBe(true);
  // A run opening a word wins over an earlier run inside a word.
  expect(holds("match", "AA", "baAA", "2-3")).toBe(true);
  // Of two runs inside a word, the one holding a word start, not the first found.
  expect(holds("match", "AA", "BAaA", "2-3")).toBe(true);
  // The words' tiers are summed, not the largest taken.
  expect(holds("order", "xb b", "xaabx;axb", "1;0")).toBe(true);
  // A title-case letter is no capital, so after a small letter it opens no word.
  expect(holds("order", "\u{1C5}", "x\u{1C5}; \u{1C5}a", "1;0")).toBe(true);
});

test("a row the reader cannot read fails naming its line", () => {
  // The strictness is the guard, so it is asked for directly: each of these
  // would otherwise be a row that silently stopped being replayed.
  expect(() => rows("# a comment\n\nfold\ta\ta\n")).not.toThrow();
  expect(rows("# a comment\n\nfold\ta\ta\n")).toHaveLength(1);
  expect(() => rows("fold\ta\ta\nfolds\ta\ta\n")).toThrow(/^line 2: `folds` is not a kind/);
  expect(() => rows("\nmatch\ta\ta\n")).toThrow(/^line 2: a `match` row holds 3 columns/);
  expect(() => rows("fold\ta\\n\ta\n")).toThrow(/^line 1: a backslash opens nothing but/);
  expect(() => rows("fold\t\\u{D800}\ta\n")).toThrow(/^line 1: `D800` is not a character/);
  expect(() => rows("fold\t\\u{zz}\ta\n")).toThrow(/^line 1: `zz` is not a character/);
  expect(rows("fold\t\\u{9}\\u{5C}\t\\u{7C}\n")[0].columns).toEqual(["\t\\", "|"]);
  // A carriage return before the line feed is no part of the last column.
  expect(rows("fold\ta\tb\r\n")[0].columns).toEqual(["a", "b"]);
});

test("every fold row folds as it says", () => {
  const differing = [];
  for (const row of ofKind("fold")) {
    const text = [...row.columns[0]];
    const folded = [...row.columns[1]];
    if (text.length !== folded.length) {
      differing.push(`line ${row.line}: a fold keeps the length of what it folds`);
      continue;
    }
    text.forEach((from, at) => {
      if (fold(from) === folded[at]) return;
      const point = from.codePointAt(0).toString(16).toUpperCase().padStart(4, "0");
      differing.push(
        `line ${row.line}: ${shown(from)} (U+${point}) folds to ${shown(folded[at])}, ` +
          `and the matcher folds it to ${shown(fold(from))}`,
      );
    });
  }
  expect(differing).toEqual([]);
});

test("every words row splits as it says", () => {
  const differing = [];
  for (const row of ofKind("words")) {
    const found = query(row.columns[0], row.line)
      .words()
      .map((word) => word.join(""));
    const expected = row.columns[1] === "-" ? [] : row.columns[1].split("|");
    if (shown(found) !== shown(expected)) {
      differing.push(`line ${row.line}: expected ${shown(expected)}, found ${shown(found)}`);
    }
  }
  expect(differing).toEqual([]);
});

test("every match row matches as it says", () => {
  const matcher = new Matcher();
  const differing = [];
  for (const row of ofKind("match")) {
    const match = matcher.matched(query(row.columns[0], row.line), item(row.columns[1]));
    const found = match === null ? null : match.hits.map((hit) => [hit.field, hit.positions]);
    const expected = expectedHits(row.columns[2], row.line);
    if (shown(found) !== shown(expected)) {
      differing.push(
        `line ${row.line}: \`${row.columns[0]}\` on \`${row.columns[1]}\` — ` +
          `expected ${shown(expected)}, found ${shown(found)}`,
      );
    }
  }
  expect(differing).toEqual([]);
});

test("every order row ranks as it says", () => {
  const matcher = new Matcher();
  const differing = [];
  for (const row of ofKind("order")) {
    const items = row.columns[1].split(";").map(item);
    const found = matcher
      .ranked(query(row.columns[0], row.line), items)
      .map((ranked) => ranked.index);
    const expected =
      row.columns[2] === "-"
        ? []
        : row.columns[2].split(";").map((index) => number(index, row.line, "an index"));
    if (shown(found) !== shown(expected)) {
      differing.push(
        `line ${row.line}: \`${row.columns[0]}\` — expected ${shown(expected)}, found ${shown(found)}`,
      );
    }
  }
  expect(differing).toEqual([]);
});

// --- What the corpus has no column for ----------------------------------------
//
// The bounds, the packed score and the two shapes a surface draws from. The
// crate holds the first two in its own unit tests; the last two exist only
// here, because only a surface in this language asks for them.

test("a query longer than its bound is refused, counted in characters", () => {
  expect(() => Query.parse("a".repeat(QUERY_CHARS_MAX))).not.toThrow();
  expect(() => Query.parse("a".repeat(QUERY_CHARS_MAX + 1))).toThrow(QueryTooLong);
  // In characters, not UTF-16 units: the bound of letters outside the basic
  // plane is two units each, and is accepted.
  expect(() => Query.parse("\u{10400}".repeat(QUERY_CHARS_MAX))).not.toThrow();
  // And separators included.
  expect(() => Query.parse(`${"a".repeat(QUERY_CHARS_MAX)} `)).toThrow(QueryTooLong);
  expect(new QueryTooLong().message).toBe("a query holds at most 64 characters; shorten it");
  // The densest query of the bound holds the most words the score can sum.
  const densest = Array.from({ length: WORDS_MAX }, () => "a").join(" ");
  expect(Query.parse(densest).words()).toHaveLength(WORDS_MAX);
  expect(WORDS_MAX).toBe(32);
});

test("a field is read to its bound and no further", () => {
  const matcher = new Matcher();
  const z = Query.parse("z");
  const read = `${"a".repeat(FIELD_CHARS_MAX - 1)}z`;
  const past = `${"a".repeat(FIELD_CHARS_MAX)}z`;
  expect(matcher.matched(z, new Item([[Role.Name, read]])).hits[0].positions).toEqual([
    FIELD_CHARS_MAX - 1,
  ]);
  expect(matcher.matched(z, new Item([[Role.Name, past]]))).toBeNull();
  // A field cut at its bound is a run, never the whole: it was not read whole.
  const word = "a".repeat(QUERY_CHARS_MAX);
  const longer = word + " ".repeat(FIELD_CHARS_MAX - QUERY_CHARS_MAX);
  const tiers = (text) =>
    matcher.matched(Query.parse(word), new Item([[Role.Name, text]])).score.tiers;
  expect(tiers(word)).toBe(3);
  expect(tiers(longer)).toBe(3);
  expect(tiers(`${longer}b`)).toBe(2);
});

test("the packed score orders as its components do, and is a safe integer", () => {
  // The largest sum each component can reach, in the order they are compared.
  const widest = [3 * WORDS_MAX, WORDS_MAX, QUERY_CHARS_MAX, QUERY_CHARS_MAX - 1];
  widest.push(WORDS_MAX * (FIELD_CHARS_MAX - 1), WORDS_MAX * (FIELD_CHARS_MAX - 1));
  for (let at = 0; at < widest.length; at += 1) {
    // Every component after this one at its largest is still below one unit of it.
    const below = new Score(...widest.map((largest, index) => (index > at ? largest : 0)));
    const above = new Score(...widest.map((_, index) => (index === at ? 1 : 0)));
    expect(below.compare(above), `component ${at}`).toBeLessThan(0);
    expect(below.packed(), `component ${at}, packed`).toBeLessThan(above.packed());
  }
  const most = new Score(...widest).packed();
  expect(Number.isSafeInteger(most)).toBe(true);
  expect(most).toBeLessThan(2 ** 50);
  expect(Score.ZERO.packed()).toBe(0);
  // What the crate's own suite counts for `bgr` on the tag `Bug Group`.
  const found = new Matcher().matched(Query.parse("bgr"), new Item([[Role.Tag, "Bug Group"]]));
  expect({ ...found.score }).toEqual({
    tiers: 0,
    names: 0,
    starts: 2,
    consecutive: 1,
    earliness: 127,
    shortness: 120,
  });
});

test("each word says where it matched, as runs", () => {
  const matcher = new Matcher();
  const verify = new Item([[Role.Name, "journal.verify"], [Role.Tag, "history"]]);
  const found = matcher.matched(Query.parse("journalverify hist"), verify);
  expect(found.hits.map((hit) => [hit.field, hit.ranges()])).toEqual([
    // A run that crosses the separator is two runs of the text a reader sees.
    [0, [{ start: 0, end: 7 }, { start: 8, end: 14 }]],
    [1, [{ start: 0, end: 4 }]],
  ]);
  // Indices count characters, never UTF-16 units: the letter after one outside
  // the basic plane is at 1.
  const astral = matcher.matched(Query.parse("b"), new Item([[Role.Name, "\u{10400}b"]]));
  expect(astral.hits[0].ranges()).toEqual([{ start: 1, end: 2 }]);
});

test("a character two words claim is the first word's", () => {
  const matcher = new Matcher();
  const item_ = new Item([[Role.Name, "Verificar o histórico"], [Role.Tag, "audit"]]);
  // `ver` and `verificar` share three characters; the first typed keeps them.
  const shared = matcher.matched(Query.parse("ver verificar hist aud"), item_);
  expect(shared.claims(0)).toEqual([
    { start: 0, end: 3, word: 0 },
    { start: 3, end: 9, word: 1 },
    { start: 12, end: 16, word: 2 },
  ]);
  expect(shared.claims(1)).toEqual([{ start: 0, end: 3, word: 3 }]);
  // Typed the other way round, the longer word is first and keeps them all.
  const turned = matcher.matched(Query.parse("verificar ver"), item_);
  expect(turned.claims(0)).toEqual([{ start: 0, end: 9, word: 0 }]);
  // The empty query claims nothing, and neither does a field no word matched.
  expect(matcher.matched(Query.parse(""), item_).claims(0)).toEqual([]);
  expect(turned.claims(1)).toEqual([]);
});
