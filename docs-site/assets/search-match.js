// Ranking one query against one name, and saying which letters matched.
//
// WHY THIS IS A FILE OF ITS OWN.
//
// It touches no DOM, carries no word in any language, and imports nothing. That
// is not tidiness — it is the graduation condition. The day this repository has
// a behaviour package, this file moves into it unchanged and `shell.js` imports
// it the way it already imports `./theme.js`. Until that day it does not live in
// `packages/`, because `packages/tokens/` is generated and committed and a
// hand-written file there would pass `cargo xtask check` only by being a file
// nothing produces.
//
// WHAT IT REPLACES.
//
// `dataset.ndIndex.toLowerCase().includes(needle)` — one contiguous substring,
// no ranking, no reason. `space heading` found nothing, `button-group` matched
// the navigation and not the index because the two were normalised by different
// rules, and a hit could never say *why* it was a hit.
//
// `NOCTUA.md §12.4` is the house doctrine and it is specific: "implement it as a
// subsequence match with the matched positions returned, and color those
// positions. Do not color by prefix only; a fuzzy match that cannot show why it
// matched is confusing."

/** Separators. A run of them starts a word, and none of them is ever matched. */
const SEPARATOR = /[\s\-_./:]/;

/** How far apart the tiers are. Every tie-break below is bounded well under it. */
const TIER = 200_000;

/** exact › prefix › word boundary › substring › subsequence. */
const EXACT = TIER * 5;
const PREFIX = TIER * 4;
const WORD = TIER * 3;
const SUBSTRING = TIER * 2;
const SUBSEQUENCE = TIER;

/**
 * A subsequence whose every letter landed on a word start: `bg` → `ButtonGroup`.
 *
 * Deliberately inside tier 5 rather than lifting it into tier 4. A literal
 * substring beats an acronym, because a reader who typed letters that are
 * actually adjacent meant them adjacent.
 */
const ACRONYM = 150_000;

/**
 * A one-character query is a prefix, not a fuzzy query.
 *
 * `a` as a subsequence matches most of any index, and a list of most of an index
 * is a list of none of it.
 */
const FUZZY_FROM = 2;

/**
 * One code point, cased down and stripped of its accent, **still one code
 * point**.
 *
 * Never `text.toLowerCase()` over a whole string: that is not length-preserving
 * — `İ` lowercases to two code points — and a fold that changes the length
 * changes the indices, which are the only thing the highlight has. The base
 * letter is the first code point of the canonical decomposition, so `é` folds to
 * `e` followed by a combining acute and `[0]` is the letter.
 *
 * `normalize` rather than the Latin-1 table in `site::collate`: that table is a
 * sort key for two languages, and this has to fold whatever a reader types. It
 * is a built-in and not a dependency (`NOCTUA.md §2.3`).
 */
function foldChar(ch) {
  if (ch.length !== 1) return ch;
  return ch.toLowerCase().normalize("NFD")[0] ?? ch;
}

const isUpper = (ch) => ch !== ch.toLowerCase() && ch === ch.toUpperCase();

/**
 * A candidate, prepared once: its folded letters, where its words start, and the
 * same letters with the separators gone.
 *
 * The tight form is what lets `button group` reach `button-group` as an **exact**
 * match rather than as a scattered subsequence, and `tightAt` maps back to the
 * real indices — which is what the highlight needs, because the highlight is
 * drawn on the string the reader can see.
 */
export function prepare(text) {
  const chars = [...text];
  const folded = chars.map(foldChar);
  const starts = chars.map((ch, at) => {
    if (at === 0) return true;
    if (SEPARATOR.test(chars[at - 1])) return true;
    // The capital in `ButtonGroup`, read off the RAW characters: folding has
    // already thrown the case away.
    return isUpper(ch) && !isUpper(chars[at - 1]);
  });
  const tight = [];
  const tightAt = [];
  for (let at = 0; at < chars.length; at += 1) {
    if (SEPARATOR.test(chars[at])) continue;
    tight.push(folded[at]);
    tightAt.push(at);
  }
  return { text, chars, folded, starts, tight, tightAt };
}

/** The query, folded and with its separators dropped. */
export function fold(query) {
  return [...query].map(foldChar).filter((ch) => !SEPARATOR.test(ch));
}

/** Where `needle` occurs in `hay` as a run, or -1. */
function runAt(hay, needle, from) {
  outer: for (let at = from; at + needle.length <= hay.length; at += 1) {
    for (let i = 0; i < needle.length; i += 1) {
      if (hay[at + i] !== needle[i]) continue outer;
    }
    return at;
  }
  return -1;
}

/**
 * Every letter in order, choosing the assignment a reader would call the match.
 *
 * NOT PLAIN GREEDY. Greedy answers `bu` on `b-u… but…` with the leftmost `u`,
 * spanning the whole string and highlighting two letters a paragraph apart.
 * Restarting from each occurrence of the query's first letter — typically one to
 * three — and keeping the best costs almost nothing and is the difference
 * between a highlight that explains itself and one that reads as noise.
 */
function subsequence(hay, query, starts) {
  let best = null;
  for (let from = 0; from + query.length <= hay.length; from += 1) {
    if (hay[from] !== query[0]) continue;
    const found = [from];
    let at = from + 1;
    for (let i = 1; i < query.length; i += 1) {
      while (at < hay.length && hay[at] !== query[i]) at += 1;
      // No assignment exists from here, and none exists from any later start
      // either: a later start can only need the same letters further right.
      if (at === hay.length) return best;
      found.push(at);
      at += 1;
    }
    best = better(best, found, starts);
  }
  return best;
}

/** More letters on word starts wins, then the tighter span, then the earlier. */
function better(a, b, starts) {
  if (!a) return b;
  const on = (list) => list.reduce((n, at) => n + (starts[at] ? 1 : 0), 0);
  if (on(b) !== on(a)) return on(b) > on(a) ? b : a;
  const span = (list) => list[list.length - 1] - list[0];
  if (span(b) !== span(a)) return span(b) < span(a) ? b : a;
  return b[0] < a[0] ? b : a;
}

/**
 * The score, and the positions the highlight will use.
 *
 * EVERY BONUS IS BOUNDED, and their sum is smaller than the gap between two
 * tiers — so no amount of tie-breaking can lift a candidate out of its tier.
 * That is a property worth asserting directly rather than through an example
 * that happens to hold, and `search.spec.js` does.
 */
function score(tier, positions, cand) {
  const first = positions[0];
  const span = positions[positions.length - 1] - first + 1;
  const starts = positions.reduce((n, at) => n + (cand.starts[at] ? 1 : 0), 0);
  const acronym = tier === SUBSEQUENCE && starts === positions.length ? ACRONYM : 0;
  return (
    tier +
    acronym +
    // Earlier is better.
    (2000 - Math.min(2000, first * 20)) +
    // Tighter is better: the excess of the span over the letters it holds.
    (1000 - Math.min(1000, (span - positions.length) * 25)) +
    // More of the letters on a word start is better.
    starts * 100 +
    // And a short name beats a long one that merely contains it.
    Math.max(0, 200 - cand.folded.length)
  );
}

/** A run of `length` positions from `at`. */
const run = (at, length) => Array.from({ length }, (_, i) => at + i);

/**
 * Ranks one already-prepared candidate against an already-folded query.
 *
 * Returns `null`, or `{score, positions}` where the positions are **code point
 * indices into the candidate's own text** — never into the folded or tightened
 * forms, which the reader never sees.
 */
export function matchPrepared(query, cand) {
  if (query.length === 0) return null;

  // Exact, whole or tight. `button group` reaching `button-group` is this line.
  const whole = cand.folded.join("");
  const tight = cand.tight.join("");
  const needle = query.join("");
  if (whole === needle) {
    return { score: score(EXACT, run(0, cand.folded.length), cand), positions: run(0, cand.chars.length) };
  }
  if (tight === needle) {
    return { score: score(EXACT, cand.tightAt, cand), positions: [...cand.tightAt] };
  }

  const at = runAt(cand.folded, query, 0);
  if (at === 0) {
    return { score: score(PREFIX, run(0, query.length), cand), positions: run(0, query.length) };
  }
  if (at > 0) {
    const positions = run(at, query.length);
    const tier = cand.starts[at] ? WORD : SUBSTRING;
    return { score: score(tier, positions, cand), positions };
  }

  // The same run, ignoring the candidate's own separators — `buttongroup`
  // finding `button-group`.
  const tightHit = runAt(cand.tight, query, 0);
  if (tightHit >= 0) {
    const positions = cand.tightAt.slice(tightHit, tightHit + query.length);
    const tier = cand.starts[positions[0]] ? WORD : SUBSTRING;
    return { score: score(tier, positions, cand), positions };
  }

  if (query.length < FUZZY_FROM) return null;
  const found = subsequence(cand.folded, query, cand.starts);
  if (!found) return null;
  return { score: score(SUBSEQUENCE, found, cand), positions: found };
}

/** The same, from raw strings. For a caller with one candidate and no index. */
export function match(query, text) {
  return matchPrepared(fold(query), prepare(text));
}

/**
 * Ranks a prepared list, sorted, capped, with the total kept.
 *
 * `sort` has been stable since ES2019, so equal scores keep the order the index
 * was built in — which is deterministic, and is why nothing here ever compares
 * two strings to break a tie.
 */
export function rank(query, candidates, cap = 0) {
  const folded = fold(query);
  if (folded.length === 0) return { hits: [], total: 0 };
  const hits = [];
  for (const cand of candidates) {
    const found = matchPrepared(folded, cand.match ?? cand);
    if (found) hits.push({ item: cand, ...found });
  }
  hits.sort((a, b) => b.score - a.score);
  return { hits: cap > 0 ? hits.slice(0, cap) : hits, total: hits.length };
}
