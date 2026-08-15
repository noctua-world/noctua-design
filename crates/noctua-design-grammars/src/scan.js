// The scanner. Its twin is `scan.rs`, beside it, and the two are one file.
//
// A .js file inside a Rust crate's `src/` is deliberate. These two are read
// together or not at all: a code block is highlighted where it renders — in
// Dioxus during SSR and in React in a browser — and neither can reach
// `docs-site/assets/shell.js`, because the packages do not ship it and this
// system ships no runtime. So the same algorithm exists twice, and the only
// way two implementations stay honest is to keep them in one directory and
// change them in one commit.
//
// This file is not generated from `scan.rs`. It is a transcription, function
// for function and branch for branch, and `dist/grammars.cases.json` is the
// blessed expectation both are checked against — never against each other,
// because two halves that agree with one another and disagree with the spec is
// the failure that actually happens.
//
// THE THREE INVARIANTS
//
//   1. Totality. Concatenating every token's `text` reproduces the input
//      exactly. Every miss degrades to `plain`; nothing is hidden, reordered
//      or dropped.
//   2. Termination. Every loop advances by at least one code point, and an
//      unclosed construct that reaches the end emits `invalid`.
//   3. Purity. The output is a function of (grammar, source) and nothing else.
//
// WHY INDICES NEVER HAVE TO AGREE
//
// `scan.rs` indexes bytes and this file indexes UTF-16 units. They never have
// to agree, because no index crosses the boundary — only substrings do, and
// both advance by whole code points.
//
// THE ONE REGULAR EXPRESSION POLICY
//
// A grammar carries no regular expressions: two regex dialects would not agree,
// and there is no regex crate in the Rust workspace. The `\p{…}` tests below
// are a different thing — they ask the Unicode character database the same
// question `char::is_alphabetic` asks it, which is the only way to give the two
// halves the same answer for a character neither file lists.

const ALPHABETIC = /\p{Alphabetic}/u;
const ALPHANUMERIC = /[\p{Alphabetic}\p{Nd}\p{Nl}\p{No}]/u;
const UPPERCASE = /\p{Uppercase}/u;
const LOWERCASE = /\p{Lowercase}/u;

/** The language a name or an alias resolves to, or `null`. */
export function language(name) {
  if (!name) return null;
  return GRAMMARS[name] ?? GRAMMARS[ALIASES[name]] ?? null;
}

/**
 * Splits `source` into coloured runs.
 *
 * Concatenating every returned `text` reproduces `source` exactly.
 */
export function scan(grammar, source) {
  const out = [];
  const to = source.length;
  if (!grammar) {
    if (source) out.push({ role: "plain", text: source });
    return out;
  }
  switch (grammar.mode) {
    case "plain":
      if (source) out.push({ role: "plain", text: source });
      break;
    case "diff":
      scanDiff(out, source, 0, to);
      break;
    case "markup":
      scanMarkup(out, source, 0, to);
      break;
    case "markdown":
      scanMarkdown(out, source, 0, to);
      break;
    default:
      scanCode(out, grammar, source, 0, to);
  }
  return out;
}

/**
 * The same tokens, cut into lines.
 *
 * A token may contain a newline — a multiline string, a block comment, a whole
 * diff line — so a component that wants one element per line cannot simply
 * group the token stream. It has to cut, and cutting is the kind of thing two
 * implementations get subtly different, so it lives here beside the scanner
 * rather than in each of them.
 *
 * The result has one more entry than the input has newlines, so joining every
 * group's text with a newline reproduces the input exactly — totality, one
 * level up.
 */
export function lines(tokens) {
  const out = [[]];
  for (const token of tokens) {
    let rest = token.text;
    for (;;) {
      const at = rest.indexOf("\n");
      if (at === -1) {
        if (rest) out[out.length - 1].push({ role: token.role, text: rest });
        break;
      }
      if (at > 0) out[out.length - 1].push({ role: token.role, text: rest.slice(0, at) });
      out.push([]);
      rest = rest.slice(at + 1);
    }
  }
  return out;
}

// --- The general scanner ----------------------------------------------------

// The order of the eight branches is the whole precedence story, and it is the
// only place precedence is expressed: a comment beats a string, a string beats
// a sigil (so `'a'` is a character literal and only a lifetime falls through to
// `'`), and everything beats the unclassified run at the bottom, which is what
// makes the pass total.
function scanCode(out, g, src, from, to) {
  let i = from;
  let run = from;

  while (i < to) {
    const c = ch(src, i, to);
    if (c === null) break;

    const comment = longestComment(g, src, i, to);
    if (comment) {
      const end = commentEnd(comment, src, i, to);
      flush(out, src, run, i);
      push(out, "comment", src, i, end);
      i = end;
      run = i;
      continue;
    }

    const found = stringAt(g, src, i, to);
    if (found) {
      const [open, form] = found;
      const extent = stringEnd(form, src, open, to);
      if (extent) {
        const [end, terminated] = extent;
        flush(out, src, run, i);
        const before = out.length;
        emitString(out, form, src, i, open, end, terminated);
        // A KEY IS A STRING WITH A COLON AFTER IT, which is the whole of JSON's
        // structure. Only promoted when the form produced exactly one token: a
        // key containing an escape would otherwise be part property and part
        // string, which is worse than being neither.
        if (
          terminated &&
          g.context.propertyColon !== "none" &&
          out.length === before + 1 &&
          followedBy(src, end, to, ":")
        ) {
          out[before].role = "property";
        }
        i = end;
        run = i;
        continue;
      }
    }

    const sigil = longestSigil(g, src, i, from, to);
    if (sigil) {
      flush(out, src, run, i);
      const end = identEnd(g, src, i + sigil.open.length, to);
      push(out, sigil.role, src, i, end);
      i = end;
      run = i;
      continue;
    }

    if (numberAt(g, src, i, to)) {
      const end = numberEnd(g, src, i, to);
      flush(out, src, run, i);
      push(out, "number", src, i, end);
      i = end;
      run = i;
      continue;
    }

    if (isIdentStart(g, c)) {
      const end = identEnd(g, src, i, to);
      flush(out, src, run, i);
      const role = classify(g, out, src, i, end, to);
      push(out, role, src, i, end);
      i = end;
      run = i;
      continue;
    }

    if (g.symbols.operator.includes(c)) {
      const end = runOf(g.symbols.operator, src, i, to);
      flush(out, src, run, i);
      push(out, "operator", src, i, end);
      i = end;
      run = i;
      continue;
    }

    if (g.symbols.punctuation.includes(c)) {
      flush(out, src, run, i);
      const end = i + c.length;
      push(out, "punctuation", src, i, end);
      i = end;
      run = i;
      continue;
    }

    // Whitespace, and anything no rule claimed. It joins the run, which is what
    // keeps the pass total without a rule for every character.
    i += c.length;
  }

  flush(out, src, run, to);
}

function classify(g, out, src, start, end, to) {
  const word = src.slice(start, end);

  const known = lookup(g, word);
  if (known) return known;

  // A DECLARATION NAMES ITS OWN KIND, and it is the only thing that can:
  // `fn longest<'a>(…)` has no paren after the name and `struct point` has no
  // capital, so both would otherwise be plain. One token of lookbehind, and no
  // more — a second would be the beginning of a parser.
  const previous = previousWord(out);
  if (previous !== null) {
    for (const [word_, role] of g.context.nameAfter) {
      if (word_ === previous) return role;
    }
  }

  // A CALL BEATS A CAPITAL, and the order matters in both directions.
  // `Console.WriteLine(` is a method in a language that capitalises them, and
  // `Vec<Point>` is a type in one that does not — the paren is what tells them
  // apart, and nothing else here can.
  if (g.context.callParen && ch(src, end, to) === "(") return "function";
  if (g.context.screamingConstant && isScreaming(word)) return "constant";
  if (g.context.upperType && UPPERCASE.test(word[0] ?? "")) return "type";
  const last = out[out.length - 1];
  if (g.context.memberDot && last && last.role === "punctuation" && last.text === ".") {
    return "property";
  }
  if (g.context.propertyColon === "any" && followedBy(src, end, to, ":")) return "property";
  if (g.context.propertyEquals && followedBy(src, end, to, "=")) return "property";
  return "plain";
}

// Whether the next thing after `at`, ignoring spaces and tabs, is `mark` and
// not a doubling of it. The doubling check is what keeps `Vec::new` from being
// a property and `a == b` from being an assignment.
function followedBy(src, at, to, mark) {
  const j = skipBlank(src, at, to);
  return ch(src, j, to) === mark && ch(src, j + mark.length, to) !== mark;
}

// The last token that was not whitespace. Whitespace only: a comment between
// `fn` and its name is not something to see through, because at that point
// nobody knows what they are reading either.
function previousWord(out) {
  for (let i = out.length - 1; i >= 0; i -= 1) {
    const token = out[i];
    if (token.role === "plain" && token.text.trim() === "") continue;
    return token.text;
  }
  return null;
}

function lookup(g, word) {
  for (const [candidate, role] of g.words) {
    if (candidate.length !== word.length) continue;
    if (g.context.caseInsensitive ? equalAsciiCase(candidate, word) : candidate === word) {
      return role;
    }
  }
  return null;
}

// ASCII-only on purpose: `String.prototype.toLowerCase` is locale- and
// Unicode-aware and `str::eq_ignore_ascii_case` is not, and a keyword table is
// ASCII in every language here.
function equalAsciiCase(a, b) {
  for (let i = 0; i < a.length; i += 1) {
    let x = a.charCodeAt(i);
    let y = b.charCodeAt(i);
    if (x >= 65 && x <= 90) x += 32;
    if (y >= 65 && y <= 90) y += 32;
    if (x !== y) return false;
  }
  return true;
}

function isScreaming(word) {
  let letters = false;
  for (const c of word) {
    if (ALPHABETIC.test(c)) {
      if (LOWERCASE.test(c)) return false;
      letters = true;
    }
  }
  // One capital letter is a generic parameter, not a constant.
  return letters && [...word].length > 1;
}

// --- Comments ---------------------------------------------------------------

// Longest rather than first, so `///` beats `//` and `<!--` beats `<!` whatever
// order a spec file happens to list them in.
function longestComment(g, src, i, to) {
  let best = null;
  for (const form of g.comments) {
    if (startsWith(src, i, to, form.open) && (!best || best.open.length < form.open.length)) {
      best = form;
    }
  }
  return best;
}

// An unterminated block comment runs to the end of the input and stays a
// comment. That is what the language would do with it, and calling it invalid
// would paint the rest of a truncated file red for a reason nobody can act on.
function commentEnd(form, src, i, to) {
  let j = i + form.open.length;
  if (!form.close) {
    for (;;) {
      const c = ch(src, j, to);
      if (c === null) return to;
      if (c === "\n") return j;
      j += c.length;
    }
  }
  let depth = 1;
  while (j < to) {
    if (form.nestable && startsWith(src, j, to, form.open)) {
      depth += 1;
      j += form.open.length;
      continue;
    }
    if (startsWith(src, j, to, form.close)) {
      j += form.close.length;
      depth -= 1;
      if (depth === 0) return j;
      continue;
    }
    const c = ch(src, j, to);
    if (c === null) break;
    j += c.length;
  }
  return to;
}

// --- Strings ----------------------------------------------------------------

// `r"…"`, `b"…"`, `f"…"` and `br"…"` are one token each. The prefix is only
// taken when a string opener follows it immediately, so an ordinary `r` or `b`
// is still an identifier.
function stringAt(g, src, i, to) {
  const direct = longestString(g, src, i, to);
  if (direct) return [i, direct];
  if (!g.context.stringPrefix) return null;
  let j = i;
  for (;;) {
    const c = ch(src, j, to);
    if (c === null || !g.context.stringPrefix.includes(c)) break;
    j += c.length;
  }
  if (j > i) {
    const prefixed = longestString(g, src, j, to);
    if (prefixed) return [j, prefixed];
  }
  return null;
}

function longestString(g, src, i, to) {
  let best = null;
  for (const form of g.strings) {
    if (startsWith(src, i, to, form.open) && (!best || best.open.length < form.open.length)) {
      best = form;
    }
  }
  return best;
}

// `null` means "this is not that form after all" — the capped case, where a
// Rust lifetime declines to be a character literal and the caller falls through
// to the sigil table.
function stringEnd(form, src, open, to) {
  let j = open + form.open.length;
  if (j > to) return null;
  let seen = 0;
  while (j < to) {
    if (form.max > 0 && seen > form.max) return null;
    const c = ch(src, j, to);
    if (c === null) break;
    if (form.max > 0 && (c === " " || c === "\t" || c === "\n")) return null;
    if (form.escape && startsWith(src, j, to, form.escape)) {
      j += form.escape.length;
      const next = ch(src, j, to);
      if (next !== null) j += next.length;
      seen += 2;
      continue;
    }
    if (form.close && startsWith(src, j, to, form.close)) {
      return [j + form.close.length, true];
    }
    if (c === "\n") {
      if (!form.close) return [j, true];
      if (!form.multiline) return [j, false];
    }
    j += c.length;
    seen += 1;
  }
  return [to, !form.close];
}

// An interpolation is emitted whole, as an `escape`. Scanning its inside as
// code would be a second entry point that both halves would have to reproduce
// exactly, for a construct whose useful signal — this part is not literal text
// — is already carried by giving it a different colour from the string.
function emitString(out, form, src, token, open, end, terminated) {
  if (!terminated) {
    push(out, "invalid", src, token, end);
    return;
  }

  const bodyEnd = end - form.close.length;
  let seg = token;
  let j = open + form.open.length;

  while (j < bodyEnd) {
    if (form.escape && startsWith(src, j, bodyEnd, form.escape)) {
      let k = j + form.escape.length;
      const next = ch(src, k, bodyEnd);
      if (next !== null) k += next.length;
      push(out, "string", src, seg, j);
      push(out, "escape", src, j, k);
      seg = k;
      j = k;
      continue;
    }
    if (form.interpolateOpen && startsWith(src, j, bodyEnd, form.interpolateOpen)) {
      const k = interpolationEnd(form, src, j, bodyEnd);
      if (k !== null) {
        push(out, "string", src, seg, j);
        push(out, "escape", src, j, k);
        seg = k;
        j = k;
        continue;
      }
    }
    if (form.interpolateSigil && startsWith(src, j, bodyEnd, form.interpolateSigil)) {
      const k = plainName(src, j + form.interpolateSigil.length, bodyEnd);
      if (k > j + form.interpolateSigil.length) {
        push(out, "string", src, seg, j);
        push(out, "escape", src, j, k);
        seg = k;
        j = k;
        continue;
      }
    }
    const c = ch(src, j, bodyEnd);
    if (c === null) break;
    j += c.length;
  }

  push(out, "string", src, seg, end);
}

// The name after a sigil inside a string. ASCII letters, digits and `_` rather
// than the grammar's own identifier rules: what follows `$` inside a shell
// string is a variable name, and a grammar whose identifiers allow `-` would
// otherwise swallow `$HOME-backup` whole.
function plainName(src, i, to) {
  for (;;) {
    const c = ch(src, i, to);
    if (c === null || !(isAsciiAlnum(c) || c === "_")) break;
    i += c.length;
  }
  return i;
}

// Both the opener and its last character count as a nesting level, so
// `${ {a: 1} }` closes where it should. An unbalanced one returns null and the
// text stays string, which is the safe direction.
function interpolationEnd(form, src, at, to) {
  const nest = [...form.interpolateOpen].pop() ?? null;
  let depth = 0;
  let k = at;
  while (k < to) {
    if (startsWith(src, k, to, form.interpolateOpen)) {
      depth += 1;
      k += form.interpolateOpen.length;
      continue;
    }
    if (startsWith(src, k, to, form.interpolateClose)) {
      depth -= 1;
      k += form.interpolateClose.length;
      if (depth === 0) return k;
      continue;
    }
    const c = ch(src, k, to);
    if (c === null) return null;
    if (c === nest) depth += 1;
    k += c.length;
  }
  return null;
}

// --- Sigils, numbers, identifiers -------------------------------------------

function longestSigil(g, src, i, from, to) {
  let best = null;
  for (const sigil of g.sigils) {
    if (
      startsWith(src, i, to, sigil.open) &&
      (!sigil.lineStart || atLineStart(src, i, from)) &&
      (!best || best.open.length < sigil.open.length)
    ) {
      best = sigil;
    }
  }
  return best;
}

// Stepped back one unit at a time on purpose: the only characters this asks
// about are a space, a tab and a newline, all of which are one byte in UTF-8
// and one unit in UTF-16, so `scan.rs` is the same loop over bytes.
function atLineStart(src, i, from) {
  let j = i;
  while (j > from) {
    const code = src.charCodeAt(j - 1);
    if (code === 32 || code === 9) {
      j -= 1;
      continue;
    }
    return code === 10;
  }
  return true;
}

function numberAt(g, src, i, to) {
  const c = ch(src, i, to);
  if (c === null) return false;
  if (isDigit(c)) return true;
  if (g.number.leadingDot && c === ".") {
    const next = ch(src, i + 1, to);
    return next !== null && isDigit(next);
  }
  return false;
}

function numberEnd(g, src, i, to) {
  const n = g.number;
  let j = i;

  if (ch(src, j, to) === ".") {
    j = digits(n, src, j + 1, to);
    return numberTail(g, src, j, to);
  }

  for (const prefix of n.prefixes) {
    const after = ch(src, j + prefix.length, to);
    if (startsWith(src, j, to, prefix) && after !== null && isAsciiAlnum(after)) {
      j += prefix.length;
      // Alphanumeric rather than "digits of that radix": `0xFF_FFu32` is one
      // number, and separating the digits from the suffix would need a radix
      // table that buys nothing a reader can see.
      for (;;) {
        const c = ch(src, j, to);
        if (c === null) break;
        // EXCEPT AN EXPONENT WITH A SIGN, which is the one thing a radix run
        // must not swallow: `0x1p-3` is a hex float, and reading it as `0x1p`
        // and a subtraction is a number that ends in the middle of itself. The
        // sign is what makes this safe — `0xFFe_FF` has an `e` too, and no
        // sign after it.
        const sign = ch(src, j + c.length, to);
        const after = ch(src, j + c.length + 1, to);
        if (
          n.exponent.includes(c) &&
          (sign === "+" || sign === "-") &&
          after !== null &&
          isDigit(after)
        ) {
          break;
        }
        if (isAsciiAlnum(c) || n.separator.includes(c)) j += c.length;
        else break;
      }
      return numberTail(g, src, j, to);
    }
  }

  j = digits(n, src, j, to);
  const dot = ch(src, j, to);
  const afterDot = ch(src, j + 1, to);
  if (dot === "." && afterDot !== null && isDigit(afterDot)) {
    j = digits(n, src, j + 1, to);
  }
  return numberTail(g, src, j, to);
}

function numberTail(g, src, j, to) {
  const n = g.number;
  const c = ch(src, j, to);
  if (c !== null && n.exponent && n.exponent.includes(c)) {
    let k = j + c.length;
    const sign = ch(src, k, to);
    if (sign === "+" || sign === "-") k += 1;
    const digit = ch(src, k, to);
    if (digit !== null && isDigit(digit)) j = digits(n, src, k, to);
  }
  if (n.suffix) {
    for (;;) {
      const next = ch(src, j, to);
      if (next === null || !isIdentChar(g, next)) break;
      j += next.length;
    }
  }
  return j;
}

function digits(n, src, j, to) {
  for (;;) {
    const c = ch(src, j, to);
    if (c === null) break;
    if (isDigit(c) || (n.separator && n.separator.includes(c))) j += c.length;
    else break;
  }
  return j;
}

function isDigit(c) {
  return c >= "0" && c <= "9";
}

function isAsciiAlnum(c) {
  return (c >= "0" && c <= "9") || (c >= "a" && c <= "z") || (c >= "A" && c <= "Z");
}

function isIdentChar(g, c) {
  return (
    isAsciiAlnum(c) ||
    g.identifier.extra.includes(c) ||
    (g.identifier.unicode && c.charCodeAt(0) > 127 && ALPHANUMERIC.test(c))
  );
}

function isIdentStart(g, c) {
  return !isDigit(c) && isIdentChar(g, c);
}

function identEnd(g, src, i, to) {
  for (;;) {
    const c = ch(src, i, to);
    if (c === null || !isIdentChar(g, c)) break;
    i += c.length;
  }
  return i;
}

function runOf(set, src, i, to) {
  for (;;) {
    const c = ch(src, i, to);
    if (c === null || !set.includes(c)) break;
    i += c.length;
  }
  return i;
}

// --- Diff -------------------------------------------------------------------

// A diff is not a sequence of tokens; it is a list of lines whose first
// characters decide the whole line. The newline belongs to the line, so a stack
// of added lines is a stack of blocks rather than a ladder.
function scanDiff(out, src, from, to) {
  let i = from;
  while (i < to) {
    const end = lineEnd(src, i, to);
    const next = end < to ? end + 1 : end;
    const line = src.slice(i, next);
    let role = "plain";
    if (
      line.startsWith("+++") ||
      line.startsWith("---") ||
      line.startsWith("diff ") ||
      line.startsWith("index ")
    ) {
      role = "comment";
    } else if (line.startsWith("@@")) {
      role = "attribute";
    } else if (line.startsWith("+")) {
      role = "inserted";
    } else if (line.startsWith("-")) {
      role = "deleted";
    }
    out.push({ role, text: line });
    i = next;
  }
}

// --- Markup -----------------------------------------------------------------

function scanMarkup(out, src, from, to) {
  let i = from;
  let run = from;

  while (i < to) {
    if (startsWith(src, i, to, "<!--")) {
      flush(out, src, run, i);
      const at = find(src, i + 4, to, "-->");
      const end = at === null ? to : at + 3;
      push(out, "comment", src, i, end);
      i = end;
      run = i;
      continue;
    }
    if (startsWith(src, i, to, "<!") || startsWith(src, i, to, "<?")) {
      flush(out, src, run, i);
      const at = find(src, i + 2, to, ">");
      const end = at === null ? to : at + 1;
      push(out, "attribute", src, i, end);
      i = end;
      run = i;
      continue;
    }
    const next = ch(src, i + 1, to);
    if (startsWith(src, i, to, "<") && next !== null && (ALPHABETIC.test(next) || next === "/")) {
      flush(out, src, run, i);
      i = scanTag(out, src, i, to);
      run = i;
      continue;
    }
    if (ch(src, i, to) === "&") {
      const end = entityEnd(src, i, to);
      if (end !== null) {
        flush(out, src, run, i);
        push(out, "escape", src, i, end);
        i = end;
        run = i;
        continue;
      }
    }
    const c = ch(src, i, to);
    if (c === null) break;
    i += c.length;
  }

  flush(out, src, run, to);
}

function scanTag(out, src, i, to) {
  let j = i + 1;
  if (ch(src, j, to) === "/") j += 1;
  const closing = j > i + 1;
  push(out, "punctuation", src, i, j);

  const nameStart = j;
  const nameEnd = nameRun(src, j, to);
  push(out, "tag", src, nameStart, nameEnd);
  const name = src.slice(nameStart, nameEnd);

  let k = nameEnd;
  let run = k;
  while (k < to) {
    const c = ch(src, k, to);
    if (c === null) break;
    if (c === ">") {
      flush(out, src, run, k);
      push(out, "punctuation", src, k, k + 1);
      k += 1;
      run = k;
      break;
    }
    if (startsWith(src, k, to, "/>")) {
      flush(out, src, run, k);
      push(out, "punctuation", src, k, k + 2);
      k += 2;
      run = k;
      break;
    }
    if (c === "=") {
      flush(out, src, run, k);
      push(out, "operator", src, k, k + 1);
      k += 1;
      run = k;
      continue;
    }
    if (c === '"' || c === "'") {
      flush(out, src, run, k);
      const at = findChar(src, k + 1, to, c);
      const end = at === null ? to : at + 1;
      push(out, "string", src, k, end);
      k = end;
      run = k;
      continue;
    }
    if (isNameChar(c)) {
      flush(out, src, run, k);
      const end = nameRun(src, k, to);
      push(out, "attribute", src, k, end);
      k = end;
      run = k;
      continue;
    }
    k += c.length;
  }
  flush(out, src, run, k);

  // AN ISLAND IS SOMEBODY ELSE'S LANGUAGE. `<script>` and `<style>` hold raw
  // text that the markup scanner would otherwise mangle into tags at the first
  // `<` in a comparison.
  const island = islandLanguage(name);
  if (!closing && island) {
    const close = island === "javascript" ? "</script" : "</style";
    const at = findIgnoringCase(src, k, to, close);
    if (at !== null) {
      if (at > k) {
        const inner = language(island);
        if (inner) scanCode(out, inner, src, k, at);
        else push(out, "plain", src, k, at);
      }
      return at;
    }
  }
  return k;
}

function islandLanguage(name) {
  const lower = name.toLowerCase();
  if (lower === "script") return "javascript";
  if (lower === "style") return "css";
  return null;
}

function isNameChar(c) {
  return (
    ALPHANUMERIC.test(c) ||
    c === "-" ||
    c === "_" ||
    c === ":" ||
    c === "." ||
    c === "@" ||
    c === "#"
  );
}

function nameRun(src, i, to) {
  for (;;) {
    const c = ch(src, i, to);
    if (c === null || !isNameChar(c)) break;
    i += c.length;
  }
  return i;
}

// `&amp;` and friends. Capped, so a bare `&` in prose is not the start of a
// search that runs to the end of the document.
function entityEnd(src, i, to) {
  let j = i + 1;
  let seen = 0;
  while (j < to && seen < 12) {
    const c = ch(src, j, to);
    if (c === null) return null;
    if (c === ";") return seen === 0 ? null : j + 1;
    if (!isAsciiAlnum(c) && c !== "#") return null;
    j += c.length;
    seen += 1;
  }
  return null;
}

// --- Markdown ---------------------------------------------------------------

function scanMarkdown(out, src, from, to) {
  let i = from;
  while (i < to) {
    const end = lineEnd(src, i, to);
    const next = end < to ? end + 1 : end;
    const line = src.slice(i, next);
    const body = line.trimStart();
    const indent = line.length - body.length;

    if (body.startsWith("```") || body.startsWith("~~~")) {
      const marker = body.startsWith("```") ? "```" : "~~~";
      push(out, "attribute", src, i, next);
      const info = body.slice(3).trim();
      const named = info ? language(firstWord(info)) : null;
      const [bodyEnd, fenceEnd] = fenceExtent(src, next, to, marker);
      if (bodyEnd > next) {
        if (named) scanCode(out, named, src, next, bodyEnd);
        else push(out, "plain", src, next, bodyEnd);
      }
      if (fenceEnd > bodyEnd) push(out, "attribute", src, bodyEnd, fenceEnd);
      i = fenceEnd;
      continue;
    }

    if (body.startsWith("#")) {
      push(out, "keyword", src, i, next);
      i = next;
      continue;
    }
    if (body.startsWith(">")) {
      push(out, "comment", src, i, next);
      i = next;
      continue;
    }
    if (isRule(body)) {
      push(out, "punctuation", src, i, next);
      i = next;
      continue;
    }
    scanMarkdownInline(out, src, i + indent, next, i);
    i = next;
  }
}

// One line of prose: inline code, links, and the marker a list item starts
// with. Emphasis is deliberately not marked — `*` is a bullet, a multiplication
// sign and two kinds of emphasis, and guessing wrong colours a whole paragraph.
function scanMarkdownInline(out, src, start, to, lineStart) {
  let run = lineStart;
  let i = start;

  const marker = listMarker(src, start, to);
  if (marker > start) {
    flush(out, src, run, start);
    push(out, "punctuation", src, start, marker);
    i = marker;
    run = i;
  }

  while (i < to) {
    const c = ch(src, i, to);
    if (c === null) break;
    if (c === "`") {
      const ticks = runOf("`", src, i, to);
      const at = find(src, ticks, to, src.slice(i, ticks));
      if (at !== null) {
        flush(out, src, run, i);
        const end = at + (ticks - i);
        push(out, "string", src, i, end);
        i = end;
        run = i;
        continue;
      }
    }
    if (c === "(" && i > start && ch(src, i - 1, to) === "]") {
      const at = findChar(src, i + 1, to, ")");
      if (at !== null) {
        flush(out, src, run, i);
        push(out, "string", src, i, at + 1);
        i = at + 1;
        run = i;
        continue;
      }
    }
    i += c.length;
  }
  flush(out, src, run, to);
}

function listMarker(src, start, to) {
  const c = ch(src, start, to);
  if ((c === "-" || c === "*" || c === "+") && ch(src, start + 1, to) === " ") return start + 2;
  if (c !== null && isDigit(c)) {
    let j = start;
    for (;;) {
      const d = ch(src, j, to);
      if (d === null || !isDigit(d)) break;
      j += 1;
    }
    const mark = ch(src, j, to);
    if ((mark === "." || mark === ")") && ch(src, j + 1, to) === " ") return j + 2;
  }
  return start;
}

function isRule(body) {
  const trimmed = body.trimEnd();
  if (trimmed.length < 3) return false;
  return /^-+$/.test(trimmed) || /^\*+$/.test(trimmed) || /^_+$/.test(trimmed);
}

function fenceExtent(src, from, to, marker) {
  let i = from;
  while (i < to) {
    const end = lineEnd(src, i, to);
    const next = end < to ? end + 1 : end;
    if (src.slice(i, end).trimStart().startsWith(marker)) return [i, next];
    i = next;
  }
  return [to, to];
}

function firstWord(text) {
  const at = text.search(/\s/);
  return at === -1 ? text : text.slice(0, at);
}

// --- The primitives everything above is built from --------------------------

/** The code point at `i`, or null at or past `to`. */
function ch(src, i, to) {
  if (i >= to || i >= src.length) return null;
  const point = src.codePointAt(i);
  return point > 0xffff ? src.slice(i, i + 2) : src[i];
}

function startsWith(src, i, to, pattern) {
  return Boolean(pattern) && i + pattern.length <= to && src.startsWith(pattern, i);
}

function find(src, from, to, pattern) {
  if (from > to) return null;
  const at = src.slice(from, to).indexOf(pattern);
  return at === -1 ? null : from + at;
}

function findChar(src, from, to, needle) {
  return find(src, from, to, needle);
}

function findIgnoringCase(src, from, to, pattern) {
  let i = from;
  while (i < to) {
    if (
      i + pattern.length <= to &&
      equalAsciiCase(src.slice(i, i + pattern.length), pattern)
    ) {
      return i;
    }
    const c = ch(src, i, to);
    if (c === null) return null;
    i += c.length;
  }
  return null;
}

function lineEnd(src, from, to) {
  const at = find(src, from, to, "\n");
  return at === null ? to : at;
}

function skipBlank(src, i, to) {
  for (;;) {
    const c = ch(src, i, to);
    if (c !== " " && c !== "\t") break;
    i += 1;
  }
  return i;
}

// Pushes a token, unless it would be empty. An empty token would break nothing
// and would appear in every parity fixture as noise.
function push(out, role, src, from, to) {
  if (from < to) out.push({ role, text: src.slice(from, to) });
}

// Ends the unclassified run at `upto`, which is what makes the pass total.
function flush(out, src, run, upto) {
  if (run < upto) out.push({ role: "plain", text: src.slice(run, upto) });
}
