//! The scanner. Its twin is `scan.js`, beside it, and the two are one file.
//!
//! # Read them together or not at all
//!
//! A code block is highlighted where it renders: in Dioxus during SSR, and in
//! React in a browser. Neither can reach `docs-site/assets/shell.js`, because
//! the packages do not ship it and this system ships no runtime. So the same
//! algorithm has to exist twice, and the only way two implementations stay
//! honest is to keep them in one directory and change them in one commit.
//!
//! `scan.js` is not generated from this file. It is a transcription, function
//! for function and branch for branch, and `dist/grammars.cases.json` is the
//! blessed expectation both are checked against — never against each other,
//! because two halves that agree with one another and disagree with the spec
//! is the failure that actually happens.
//!
//! # The three invariants
//!
//! 1. **Totality.** Concatenating every token's `text` reproduces the input,
//!    byte for byte. Every miss degrades to [`Role::Plain`]; nothing is hidden,
//!    reordered or dropped. This is what makes an imperfect highlighter safe.
//! 2. **Termination.** Every loop advances by at least one scalar value, and an
//!    unclosed construct that reaches the end of the input emits
//!    [`Role::Invalid`] rather than looping or panicking.
//! 3. **Purity.** The output is a function of `(grammar, source)` and nothing
//!    else — no locale, no clock, no environment.
//!
//! # Why byte indices are safe here
//!
//! Every advance is by a whole scalar value (`char::len_utf8`), so a slice
//! boundary is always a character boundary. `scan.js` advances by whole code
//! points over UTF-16 units, and the two never have to agree about an index
//! because **no index crosses the boundary** — only substrings do.

extern crate alloc;

use alloc::vec::Vec;

use crate::{Language, Role};

/// A run of the input, and what it is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token<'a> {
    /// What to paint it.
    pub role: Role,
    /// The characters themselves, borrowed from the input.
    pub text: &'a str,
}

/// Which scanner reads a language. See `spec/grammars/`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// Words, comments, strings, numbers, symbols.
    Code,
    /// Angle-bracket markup.
    Markup,
    /// A document: headings, fences, inline code, links.
    Markdown,
    /// Line-oriented: the first characters of a line decide the line.
    Diff,
    /// One token.
    Plain,
}

/// What an identifier is made of.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Identifier {
    /// Identifier characters besides letters and digits.
    pub extra: &'static str,
    /// Whether a letter outside ASCII counts.
    pub unicode: bool,
}

/// One comment form.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Comment {
    /// What opens it.
    pub open: &'static str,
    /// What closes it. Empty is the end of the line.
    pub close: &'static str,
    /// Whether an inner opener needs an inner closer.
    pub nestable: bool,
}

/// One string form.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringForm {
    /// What opens it.
    pub open: &'static str,
    /// What closes it. Empty is the end of the line.
    pub close: &'static str,
    /// What escapes the next character. Empty means the form is raw.
    pub escape: &'static str,
    /// Whether it may cross a newline.
    pub multiline: bool,
    /// A cap in characters. **A capped form also refuses a space, a tab and a
    /// newline**, because the construct the cap exists for is a character
    /// literal and a character literal contains none of them. Without that
    /// rule `'a` in `fn f<'a>(x: &'a str)` finds the next `'` seven characters
    /// later and paints half the signature as a literal.
    pub max: usize,
    /// What opens an interpolation. Empty means it has none.
    pub interpolate_open: &'static str,
    /// What closes one.
    pub interpolate_close: &'static str,
    /// A mark that, with the name after it, is a hole: `"$HOME/bin"`. It has
    /// no closer — it ends where the name ends.
    pub interpolate_sigil: &'static str,
}

/// What a number looks like.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Number {
    /// Radix prefixes.
    pub prefixes: &'static [&'static str],
    /// A character between digits that means nothing.
    pub separator: &'static str,
    /// Letters that introduce an exponent.
    pub exponent: &'static str,
    /// Whether identifier characters after the digits belong to the number.
    pub suffix: bool,
    /// Whether `.5` is a number.
    pub leading_dot: bool,
}

/// The characters that are neither identifier nor whitespace.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Symbols {
    /// Characters that combine into operators.
    pub operator: &'static str,
    /// Characters that stand alone.
    pub punctuation: &'static str,
}

/// A mark that colours the name after it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sigil {
    /// The mark.
    pub open: &'static str,
    /// What the mark and the name become.
    pub role: Role,
    /// Whether it only counts first on a line.
    pub line_start: bool,
}

/// What a `:` may promote to a property.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropertyKey {
    /// Nothing.
    None,
    /// A quoted string only.
    String,
    /// A string or an identifier.
    Any,
}

/// Where an identifier is coloured by what is next to it.
///
/// Six flags and two tables. Clippy is right that six booleans usually mean a
/// missing type — but not here: each is an **independent** claim about a
/// language (`call_paren` and `upper_type` are true together in Rust, apart in
/// shell, and neither in JSON), so there are sixty-four legitimate combinations
/// and the state machine that replaced them would have sixty-four states.
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Context {
    /// An identifier immediately before `(` is a call.
    pub call_paren: bool,
    /// An identifier starting upper-case is a type.
    pub upper_type: bool,
    /// `SCREAMING_SNAKE_CASE` is a constant.
    pub screaming_constant: bool,
    /// An identifier right after `.` is a member.
    pub member_dot: bool,
    /// What a following `:` promotes.
    pub property_colon: PropertyKey,
    /// Whether a following `=` promotes an identifier.
    pub property_equals: bool,
    /// Characters that may prefix a string opener and belong to it.
    pub string_prefix: &'static str,
    /// The word before an identifier, and what that makes the identifier.
    pub name_after: &'static [(&'static str, Role)],
    /// Whether words are matched without regard to case.
    pub case_insensitive: bool,
}

/// One language, as data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Grammar {
    /// Its name, in `kebab-case`.
    pub name: &'static str,
    /// How it is written for a reader.
    pub display: &'static str,
    /// Other names it answers to.
    pub aliases: &'static [&'static str],
    /// Which scanner reads it.
    pub mode: Mode,
    /// What an identifier is made of.
    pub identifier: Identifier,
    /// The words that are not identifiers, and what they become.
    pub words: &'static [(&'static str, Role)],
    /// Every comment form.
    pub comments: &'static [Comment],
    /// Every string form.
    pub strings: &'static [StringForm],
    /// What a number looks like.
    pub number: Number,
    /// The symbol sets.
    pub symbols: Symbols,
    /// The marks that colour a name.
    pub sigils: &'static [Sigil],
    /// The context rules.
    pub context: Context,
}

/// Splits `source` into coloured runs.
///
/// Concatenating every returned `text` reproduces `source` exactly.
#[must_use]
pub fn scan<'a>(grammar: &Grammar, source: &'a str) -> Vec<Token<'a>> {
    let mut out = Vec::new();
    let to = source.len();
    match grammar.mode {
        Mode::Plain => {
            if !source.is_empty() {
                out.push(Token {
                    role: Role::Plain,
                    text: source,
                });
            }
        }
        Mode::Diff => scan_diff(&mut out, source, 0, to),
        Mode::Markup => scan_markup(&mut out, source, 0, to),
        Mode::Markdown => scan_markdown(&mut out, source, 0, to),
        Mode::Code => scan_code(&mut out, grammar, source, 0, to),
    }
    out
}

/// The same, for a language named at runtime. Unknown names are plain text,
/// which is the honest answer: a wrong grammar is confidently wrong.
#[must_use]
pub fn scan_language(language: Option<Language>, source: &str) -> Vec<Token<'_>> {
    match language {
        Some(language) => scan(language.grammar(), source),
        None => scan(Language::Plain.grammar(), source),
    }
}

/// The same tokens, cut into lines.
///
/// A token may contain a newline — a multiline string, a block comment, a whole
/// diff line — so a component that wants one element per line cannot simply
/// group the token stream. It has to cut, and cutting is the kind of thing two
/// implementations get subtly different, so it lives here beside the scanner
/// rather than in each of them.
///
/// The result has one more entry than the input has newlines, so joining every
/// group's text with `\n` reproduces the input exactly — totality, one level up.
/// A trailing newline therefore produces a final empty group, which a caller
/// drops or renders as a blank line depending on what it is drawing.
#[must_use]
pub fn lines<'a>(tokens: &[Token<'a>]) -> Vec<Vec<Token<'a>>> {
    let mut out: Vec<Vec<Token<'a>>> = Vec::new();
    out.push(Vec::new());
    for token in tokens {
        let mut rest = token.text;
        while let Some(at) = rest.find('\n') {
            if at > 0 {
                let current = out.len() - 1;
                out[current].push(Token {
                    role: token.role,
                    text: &rest[..at],
                });
            }
            out.push(Vec::new());
            rest = &rest[at + 1..];
        }
        if !rest.is_empty() {
            let current = out.len() - 1;
            out[current].push(Token {
                role: token.role,
                text: rest,
            });
        }
    }
    out
}

// --- The general scanner ----------------------------------------------------

/// Words, comments, strings, numbers, symbols — everything C-shaped, ML-shaped,
/// every configuration format and every query language.
///
/// The order of the eight branches is the whole precedence story, and it is the
/// only place precedence is expressed: a comment beats a string (so `//` inside
/// no string is a comment), a string beats a sigil (so `'a'` is a character
/// literal and only a lifetime falls through to `'`), and everything beats the
/// unclassified run at the bottom, which is what makes the pass total.
fn scan_code<'a>(out: &mut Vec<Token<'a>>, g: &Grammar, src: &'a str, from: usize, to: usize) {
    let mut i = from;
    let mut run = from;

    while i < to {
        let Some(c) = ch(src, i, to) else { break };

        if let Some(form) = longest_comment(g, src, i, to) {
            let end = comment_end(form, src, i, to);
            flush(out, src, &mut run, i);
            push(out, Role::Comment, src, i, end);
            i = end;
            run = i;
            continue;
        }

        if let Some((open, form)) = string_at(g, src, i, to)
            && let Some((end, terminated)) = string_end(form, src, open, to)
        {
            flush(out, src, &mut run, i);
            let before = out.len();
            emit_string(out, form, src, i, open, end, terminated);
            // A KEY IS A STRING WITH A COLON AFTER IT, which is the whole of
            // JSON's structure. Only promoted when the form produced exactly
            // one token: a key containing an escape would otherwise be part
            // property and part string, which is worse than being neither.
            if terminated
                && g.context.property_colon != PropertyKey::None
                && out.len() == before + 1
                && followed_by(src, end, to, ':')
            {
                out[before].role = Role::Property;
            }
            i = end;
            run = i;
            continue;
        }

        if let Some(sigil) = longest_sigil(g, src, i, from, to) {
            flush(out, src, &mut run, i);
            let end = ident_end(g, src, i + sigil.open.len(), to);
            push(out, sigil.role, src, i, end);
            i = end;
            run = i;
            continue;
        }

        if number_at(g, src, i, to) {
            let end = number_end(g, src, i, to);
            flush(out, src, &mut run, i);
            push(out, Role::Number, src, i, end);
            i = end;
            run = i;
            continue;
        }

        if is_ident_start(g, c) {
            let end = ident_end(g, src, i, to);
            flush(out, src, &mut run, i);
            let role = classify(g, out, src, i, end, to);
            push(out, role, src, i, end);
            i = end;
            run = i;
            continue;
        }

        if g.symbols.operator.contains(c) {
            let end = run_of(g.symbols.operator, src, i, to);
            flush(out, src, &mut run, i);
            push(out, Role::Operator, src, i, end);
            i = end;
            run = i;
            continue;
        }

        if g.symbols.punctuation.contains(c) {
            flush(out, src, &mut run, i);
            let end = i + c.len_utf8();
            push(out, Role::Punctuation, src, i, end);
            i = end;
            run = i;
            continue;
        }

        // Whitespace, and anything no rule claimed. It joins the run, which is
        // what keeps the pass total without a rule for every character.
        i += c.len_utf8();
    }

    flush(out, src, &mut run, to);
}

/// What an identifier is, once the words and the context have had their say.
fn classify(
    g: &Grammar,
    out: &[Token<'_>],
    src: &str,
    start: usize,
    end: usize,
    to: usize,
) -> Role {
    let word = &src[start..end];

    if let Some(role) = lookup(g, word) {
        return role;
    }

    // A DECLARATION NAMES ITS OWN KIND, and it is the only thing that can:
    // `fn longest<'a>(…)` has no paren after the name and `struct point` has no
    // capital, so both would otherwise be plain. One token of lookbehind, and
    // no more — a second would be the beginning of a parser.
    if let Some(previous) = previous_word(out) {
        for (word, role) in g.context.name_after {
            if *word == previous {
                return *role;
            }
        }
    }

    // A CALL BEATS A CAPITAL, and the order matters in both directions.
    // `Console.WriteLine(` is a method in a language that capitalises them, and
    // `Vec<Point>` is a type in one that does not — the paren is what tells
    // them apart, and nothing else here can.
    if g.context.call_paren && ch(src, end, to) == Some('(') {
        return Role::Function;
    }
    if g.context.screaming_constant && is_screaming(word) {
        return Role::Constant;
    }
    if g.context.upper_type && word.chars().next().is_some_and(char::is_uppercase) {
        return Role::Type;
    }
    if g.context.member_dot
        && out
            .last()
            .is_some_and(|t| t.role == Role::Punctuation && t.text == ".")
    {
        return Role::Property;
    }
    if g.context.property_colon == PropertyKey::Any && followed_by(src, end, to, ':') {
        return Role::Property;
    }
    if g.context.property_equals && followed_by(src, end, to, '=') {
        return Role::Property;
    }
    Role::Plain
}

/// Whether the next thing after `at`, ignoring spaces and tabs, is `mark` and
/// not a doubling of it.
///
/// The doubling check is what keeps `Vec::new` from being a property and `a ==
/// b` from being an assignment: `::` is a path and `==` is a comparison, and
/// both would otherwise read as the single character in front of them.
fn followed_by(src: &str, at: usize, to: usize, mark: char) -> bool {
    let j = skip_blank(src, at, to);
    ch(src, j, to) == Some(mark) && ch(src, j + mark.len_utf8(), to) != Some(mark)
}

/// The last token that was not whitespace.
///
/// Whitespace only: a comment between `fn` and its name is not something to
/// see through, because at that point nobody knows what they are reading either.
fn previous_word<'a>(out: &[Token<'a>]) -> Option<&'a str> {
    for token in out.iter().rev() {
        if token.role == Role::Plain && token.text.trim().is_empty() {
            continue;
        }
        return Some(token.text);
    }
    None
}

fn lookup(g: &Grammar, word: &str) -> Option<Role> {
    for (candidate, role) in g.words {
        if candidate.len() == word.len()
            && (if g.context.case_insensitive {
                candidate.eq_ignore_ascii_case(word)
            } else {
                *candidate == word
            })
        {
            return Some(*role);
        }
    }
    None
}

fn is_screaming(word: &str) -> bool {
    let mut letters = false;
    for c in word.chars() {
        if c.is_alphabetic() {
            if c.is_lowercase() {
                return false;
            }
            letters = true;
        }
    }
    // One capital letter is a generic parameter, not a constant.
    letters && word.chars().nth(1).is_some()
}

// --- Comments ---------------------------------------------------------------

/// The longest opener that matches here.
///
/// Longest rather than first, so `///` beats `//` and `<!--` beats `<!`
/// whatever order a spec file happens to list them in. Nobody should have to
/// remember an ordering rule that a comparison can enforce.
fn longest_comment<'g>(g: &'g Grammar, src: &str, i: usize, to: usize) -> Option<&'g Comment> {
    let mut best: Option<&Comment> = None;
    for form in g.comments {
        if starts_with(src, i, to, form.open)
            && best.is_none_or(|found| found.open.len() < form.open.len())
        {
            best = Some(form);
        }
    }
    best
}

/// An unterminated block comment runs to the end of the input and stays a
/// comment. That is what the language would do with it, and calling it invalid
/// would paint the rest of a truncated file red for a reason nobody can act on.
fn comment_end(form: &Comment, src: &str, i: usize, to: usize) -> usize {
    let mut j = i + form.open.len();
    if form.close.is_empty() {
        while let Some(c) = ch(src, j, to) {
            if c == '\n' {
                return j;
            }
            j += c.len_utf8();
        }
        return to;
    }
    let mut depth = 1usize;
    while j < to {
        if form.nestable && starts_with(src, j, to, form.open) {
            depth += 1;
            j += form.open.len();
            continue;
        }
        if starts_with(src, j, to, form.close) {
            j += form.close.len();
            depth -= 1;
            if depth == 0 {
                return j;
            }
            continue;
        }
        let Some(c) = ch(src, j, to) else { break };
        j += c.len_utf8();
    }
    to
}

// --- Strings ----------------------------------------------------------------

/// Where a string starts here, allowing for a prefix that belongs to it.
///
/// `r"…"`, `b"…"`, `f"…"` and `br"…"` are one token each. The prefix is only
/// taken when a string opener follows it immediately, so an ordinary `r` or `b`
/// is still an identifier.
fn string_at<'g>(
    g: &'g Grammar,
    src: &str,
    i: usize,
    to: usize,
) -> Option<(usize, &'g StringForm)> {
    if let Some(form) = longest_string(g, src, i, to) {
        return Some((i, form));
    }
    if g.context.string_prefix.is_empty() {
        return None;
    }
    let mut j = i;
    while let Some(c) = ch(src, j, to) {
        if !g.context.string_prefix.contains(c) {
            break;
        }
        j += c.len_utf8();
    }
    if j > i
        && let Some(form) = longest_string(g, src, j, to)
    {
        return Some((j, form));
    }
    None
}

fn longest_string<'g>(g: &'g Grammar, src: &str, i: usize, to: usize) -> Option<&'g StringForm> {
    let mut best: Option<&StringForm> = None;
    for form in g.strings {
        if starts_with(src, i, to, form.open)
            && best.is_none_or(|found| found.open.len() < form.open.len())
        {
            best = Some(form);
        }
    }
    best
}

/// Where the form ends, and whether it got there honestly.
///
/// `None` means "this is not that form after all" — the capped case, where a
/// Rust lifetime declines to be a character literal and the caller falls
/// through to the sigil table.
fn string_end(form: &StringForm, src: &str, open: usize, to: usize) -> Option<(usize, bool)> {
    let mut j = open + form.open.len();
    if j > to {
        return None;
    }
    let mut seen = 0usize;
    while j < to {
        if form.max > 0 && seen > form.max {
            return None;
        }
        let c = ch(src, j, to)?;
        if form.max > 0 && (c == ' ' || c == '\t' || c == '\n') {
            return None;
        }
        if !form.escape.is_empty() && starts_with(src, j, to, form.escape) {
            j += form.escape.len();
            if let Some(next) = ch(src, j, to) {
                j += next.len_utf8();
            }
            seen += 2;
            continue;
        }
        if !form.close.is_empty() && starts_with(src, j, to, form.close) {
            return Some((j + form.close.len(), true));
        }
        if c == '\n' {
            if form.close.is_empty() {
                return Some((j, true));
            }
            if !form.multiline {
                return Some((j, false));
            }
        }
        j += c.len_utf8();
        seen += 1;
    }
    Some((to, form.close.is_empty()))
}

/// One string, as its segments: text, escapes and interpolations.
///
/// An interpolation is emitted whole, as an [`Role::Escape`]. Scanning its
/// inside as code would be a second entry point into this function that the
/// JavaScript half would have to reproduce exactly, for a construct where the
/// useful signal — *this part is not literal text* — is already carried by
/// giving it a different colour from the string around it.
fn emit_string<'a>(
    out: &mut Vec<Token<'a>>,
    form: &StringForm,
    src: &'a str,
    token: usize,
    open: usize,
    end: usize,
    terminated: bool,
) {
    if !terminated {
        push(out, Role::Invalid, src, token, end);
        return;
    }

    let body_end = end - form.close.len();
    let mut seg = token;
    let mut j = open + form.open.len();

    while j < body_end {
        if !form.escape.is_empty() && starts_with(src, j, body_end, form.escape) {
            let mut k = j + form.escape.len();
            if let Some(next) = ch(src, k, body_end) {
                k += next.len_utf8();
            }
            push(out, Role::String, src, seg, j);
            push(out, Role::Escape, src, j, k);
            seg = k;
            j = k;
            continue;
        }
        if !form.interpolate_open.is_empty()
            && starts_with(src, j, body_end, form.interpolate_open)
            && let Some(k) = interpolation_end(form, src, j, body_end)
        {
            push(out, Role::String, src, seg, j);
            push(out, Role::Escape, src, j, k);
            seg = k;
            j = k;
            continue;
        }
        if !form.interpolate_sigil.is_empty()
            && starts_with(src, j, body_end, form.interpolate_sigil)
        {
            let k = plain_name(src, j + form.interpolate_sigil.len(), body_end);
            if k > j + form.interpolate_sigil.len() {
                push(out, Role::String, src, seg, j);
                push(out, Role::Escape, src, j, k);
                seg = k;
                j = k;
                continue;
            }
        }
        let Some(c) = ch(src, j, body_end) else { break };
        j += c.len_utf8();
    }

    push(out, Role::String, src, seg, end);
}

/// The name after a sigil inside a string.
///
/// ASCII letters, digits and `_` rather than the grammar's own identifier
/// rules: what follows `$` inside a shell string is a variable name, and a
/// grammar whose identifiers allow `-` would otherwise swallow `$HOME-backup`
/// whole.
fn plain_name(src: &str, mut i: usize, to: usize) -> usize {
    while let Some(c) = ch(src, i, to) {
        if !c.is_ascii_alphanumeric() && c != '_' {
            break;
        }
        i += c.len_utf8();
    }
    i
}

/// Where `${…}` closes, counting nesting.
///
/// Both the opener and its last character count as a nesting level, so
/// `${ {a: 1} }` closes where it should. An unbalanced one returns `None` and
/// the text stays string, which is the safe direction.
fn interpolation_end(form: &StringForm, src: &str, at: usize, to: usize) -> Option<usize> {
    let nest = form.interpolate_open.chars().next_back();
    let mut depth = 0usize;
    let mut k = at;
    while k < to {
        if starts_with(src, k, to, form.interpolate_open) {
            depth += 1;
            k += form.interpolate_open.len();
            continue;
        }
        if starts_with(src, k, to, form.interpolate_close) {
            depth -= 1;
            k += form.interpolate_close.len();
            if depth == 0 {
                return Some(k);
            }
            continue;
        }
        let c = ch(src, k, to)?;
        if Some(c) == nest {
            depth += 1;
        }
        k += c.len_utf8();
    }
    None
}

// --- Sigils, numbers, identifiers -------------------------------------------

fn longest_sigil<'g>(
    g: &'g Grammar,
    src: &str,
    i: usize,
    from: usize,
    to: usize,
) -> Option<&'g Sigil> {
    let mut best: Option<&Sigil> = None;
    for sigil in g.sigils {
        if starts_with(src, i, to, sigil.open)
            && (!sigil.line_start || at_line_start(src, i, from))
            && best.is_none_or(|found| found.open.len() < sigil.open.len())
        {
            best = Some(sigil);
        }
    }
    best
}

/// Whether everything between the previous newline and here is blank.
///
/// Stepped back a **byte** at a time on purpose: the only characters this asks
/// about are a space, a tab and a newline, all of which are one byte in UTF-8
/// and one unit in UTF-16, so the JavaScript half is the same loop over
/// `charCodeAt`.
fn at_line_start(src: &str, i: usize, from: usize) -> bool {
    let bytes = src.as_bytes();
    let mut j = i;
    while j > from {
        let byte = bytes[j - 1];
        if byte == b' ' || byte == b'\t' {
            j -= 1;
            continue;
        }
        return byte == b'\n';
    }
    true
}

fn number_at(g: &Grammar, src: &str, i: usize, to: usize) -> bool {
    match ch(src, i, to) {
        Some(c) if c.is_ascii_digit() => true,
        Some('.') if g.number.leading_dot => ch(src, i + 1, to).is_some_and(|d| d.is_ascii_digit()),
        _ => false,
    }
}

fn number_end(g: &Grammar, src: &str, i: usize, to: usize) -> usize {
    let n = &g.number;
    let mut j = i;

    if ch(src, j, to) == Some('.') {
        j = digits(n, src, j + 1, to);
        return number_tail(g, src, j, to);
    }

    for prefix in n.prefixes {
        if starts_with(src, j, to, prefix)
            && ch(src, j + prefix.len(), to).is_some_and(|c| c.is_ascii_alphanumeric())
        {
            j += prefix.len();
            // Alphanumeric rather than "digits of that radix": `0xFF_FFu32` is
            // one number, and separating the digits from the suffix would need
            // a radix table that buys nothing a reader can see.
            while let Some(c) = ch(src, j, to) {
                // EXCEPT AN EXPONENT WITH A SIGN, which is the one thing a
                // radix run must not swallow: `0x1p-3` is a hex float, and
                // reading it as `0x1p` and a subtraction is a number that ends
                // in the middle of itself. The sign is what makes this safe —
                // `0xFFe_FF` has an `e` too, and no sign after it.
                if n.exponent.contains(c)
                    && matches!(ch(src, j + c.len_utf8(), to), Some('+' | '-'))
                    && ch(src, j + c.len_utf8() + 1, to).is_some_and(|d| d.is_ascii_digit())
                {
                    break;
                }
                if c.is_ascii_alphanumeric() || n.separator.contains(c) {
                    j += c.len_utf8();
                } else {
                    break;
                }
            }
            return number_tail(g, src, j, to);
        }
    }

    j = digits(n, src, j, to);
    if ch(src, j, to) == Some('.') && ch(src, j + 1, to).is_some_and(|c| c.is_ascii_digit()) {
        j = digits(n, src, j + 1, to);
    }
    number_tail(g, src, j, to)
}

/// The exponent and the suffix, which every branch above ends with.
fn number_tail(g: &Grammar, src: &str, mut j: usize, to: usize) -> usize {
    let n = &g.number;
    if let Some(c) = ch(src, j, to)
        && !n.exponent.is_empty()
        && n.exponent.contains(c)
    {
        let mut k = j + c.len_utf8();
        if matches!(ch(src, k, to), Some('+' | '-')) {
            k += 1;
        }
        if ch(src, k, to).is_some_and(|d| d.is_ascii_digit()) {
            j = digits(n, src, k, to);
        }
    }
    if n.suffix {
        while let Some(c) = ch(src, j, to) {
            if is_ident_char(g, c) {
                j += c.len_utf8();
            } else {
                break;
            }
        }
    }
    j
}

fn digits(n: &Number, src: &str, mut j: usize, to: usize) -> usize {
    while let Some(c) = ch(src, j, to) {
        if c.is_ascii_digit() || (!n.separator.is_empty() && n.separator.contains(c)) {
            j += c.len_utf8();
        } else {
            break;
        }
    }
    j
}

fn is_ident_char(g: &Grammar, c: char) -> bool {
    c.is_ascii_alphanumeric()
        || g.identifier.extra.contains(c)
        || (g.identifier.unicode && !c.is_ascii() && c.is_alphanumeric())
}

fn is_ident_start(g: &Grammar, c: char) -> bool {
    !c.is_ascii_digit() && is_ident_char(g, c)
}

fn ident_end(g: &Grammar, src: &str, mut i: usize, to: usize) -> usize {
    while let Some(c) = ch(src, i, to) {
        if !is_ident_char(g, c) {
            break;
        }
        i += c.len_utf8();
    }
    i
}

fn run_of(set: &str, src: &str, mut i: usize, to: usize) -> usize {
    while let Some(c) = ch(src, i, to) {
        if !set.contains(c) {
            break;
        }
        i += c.len_utf8();
    }
    i
}

// --- Diff -------------------------------------------------------------------

/// A diff is not a sequence of tokens; it is a list of lines whose first
/// characters decide the whole line. The newline belongs to the line, so a
/// stack of added lines is a stack of blocks rather than a ladder.
fn scan_diff<'a>(out: &mut Vec<Token<'a>>, src: &'a str, from: usize, to: usize) {
    let mut i = from;
    while i < to {
        let end = line_end(src, i, to);
        let next = if end < to { end + 1 } else { end };
        let line = &src[i..next];
        let role = if line.starts_with("+++")
            || line.starts_with("---")
            || line.starts_with("diff ")
            || line.starts_with("index ")
        {
            Role::Comment
        } else if line.starts_with("@@") {
            Role::Attribute
        } else if line.starts_with('+') {
            Role::Inserted
        } else if line.starts_with('-') {
            Role::Deleted
        } else {
            Role::Plain
        };
        out.push(Token { role, text: line });
        i = next;
    }
}

// --- Markup -----------------------------------------------------------------

/// Angle-bracket markup: a tree, scanned as the flat sequence of its marks.
fn scan_markup<'a>(out: &mut Vec<Token<'a>>, src: &'a str, from: usize, to: usize) {
    let mut i = from;
    let mut run = from;

    while i < to {
        if starts_with(src, i, to, "<!--") {
            flush(out, src, &mut run, i);
            let end = find(src, i + 4, to, "-->").map_or(to, |at| at + 3);
            push(out, Role::Comment, src, i, end);
            i = end;
            run = i;
            continue;
        }
        if starts_with(src, i, to, "<!") || starts_with(src, i, to, "<?") {
            flush(out, src, &mut run, i);
            let end = find(src, i + 2, to, ">").map_or(to, |at| at + 1);
            push(out, Role::Attribute, src, i, end);
            i = end;
            run = i;
            continue;
        }
        if starts_with(src, i, to, "<")
            && ch(src, i + 1, to).is_some_and(|c| c.is_alphabetic() || c == '/')
        {
            flush(out, src, &mut run, i);
            i = scan_tag(out, src, i, to);
            run = i;
            continue;
        }
        if ch(src, i, to) == Some('&')
            && let Some(end) = entity_end(src, i, to)
        {
            flush(out, src, &mut run, i);
            push(out, Role::Escape, src, i, end);
            i = end;
            run = i;
            continue;
        }
        let Some(c) = ch(src, i, to) else { break };
        i += c.len_utf8();
    }

    flush(out, src, &mut run, to);
}

/// One tag, from `<` to `>`, and the island it may open.
fn scan_tag<'a>(out: &mut Vec<Token<'a>>, src: &'a str, i: usize, to: usize) -> usize {
    let mut j = i + 1;
    if ch(src, j, to) == Some('/') {
        j += 1;
    }
    let closing = j > i + 1;
    push(out, Role::Punctuation, src, i, j);

    let name_start = j;
    let name_end = name_run(src, j, to);
    push(out, Role::Tag, src, name_start, name_end);
    let name = &src[name_start..name_end];

    let mut k = name_end;
    let mut run = k;
    while k < to {
        let Some(c) = ch(src, k, to) else { break };
        if c == '>' {
            flush(out, src, &mut run, k);
            push(out, Role::Punctuation, src, k, k + 1);
            k += 1;
            run = k;
            break;
        }
        if starts_with(src, k, to, "/>") {
            flush(out, src, &mut run, k);
            push(out, Role::Punctuation, src, k, k + 2);
            k += 2;
            run = k;
            break;
        }
        if c == '=' {
            flush(out, src, &mut run, k);
            push(out, Role::Operator, src, k, k + 1);
            k += 1;
            run = k;
            continue;
        }
        if c == '"' || c == '\'' {
            flush(out, src, &mut run, k);
            let end = find_char(src, k + 1, to, c).map_or(to, |at| at + 1);
            push(out, Role::String, src, k, end);
            k = end;
            run = k;
            continue;
        }
        if is_name_char(c) {
            flush(out, src, &mut run, k);
            let end = name_run(src, k, to);
            push(out, Role::Attribute, src, k, end);
            k = end;
            run = k;
            continue;
        }
        k += c.len_utf8();
    }
    flush(out, src, &mut run, k);

    // AN ISLAND IS SOMEBODY ELSE'S LANGUAGE. `<script>` and `<style>` hold raw
    // text that the markup scanner would otherwise mangle into tags at the
    // first `<` in a comparison.
    if !closing && let Some(island) = island_language(name) {
        let close = concat_close(name);
        if let Some(at) = find_ignoring_case(src, k, to, close) {
            if at > k {
                match Language::parse(island) {
                    Some(language) => scan_code(out, language.grammar(), src, k, at),
                    None => push(out, Role::Plain, src, k, at),
                }
            }
            return at;
        }
    }
    k
}

const fn island_language(name: &str) -> Option<&'static str> {
    // `const fn` over bytes rather than a match on `&str`, so the comparison is
    // the same one the JavaScript half makes.
    match name.as_bytes() {
        b"script" | b"SCRIPT" => Some("javascript"),
        b"style" | b"STYLE" => Some("css"),
        _ => None,
    }
}

const fn concat_close(name: &str) -> &'static str {
    match name.as_bytes() {
        b"script" | b"SCRIPT" => "</script",
        _ => "</style",
    }
}

fn is_name_char(c: char) -> bool {
    c.is_alphanumeric() || c == '-' || c == '_' || c == ':' || c == '.' || c == '@' || c == '#'
}

fn name_run(src: &str, mut i: usize, to: usize) -> usize {
    while let Some(c) = ch(src, i, to) {
        if !is_name_char(c) {
            break;
        }
        i += c.len_utf8();
    }
    i
}

/// `&amp;` and friends. Capped, so a bare `&` in prose is not the start of a
/// search that runs to the end of the document.
fn entity_end(src: &str, i: usize, to: usize) -> Option<usize> {
    let mut j = i + 1;
    let mut seen = 0;
    while j < to && seen < 12 {
        let c = ch(src, j, to)?;
        if c == ';' {
            return if seen == 0 { None } else { Some(j + 1) };
        }
        if !c.is_ascii_alphanumeric() && c != '#' {
            return None;
        }
        j += c.len_utf8();
        seen += 1;
    }
    None
}

// --- Markdown ---------------------------------------------------------------

/// A document, line by line, with fenced code handed to its own grammar.
fn scan_markdown<'a>(out: &mut Vec<Token<'a>>, src: &'a str, from: usize, to: usize) {
    let mut i = from;
    while i < to {
        let end = line_end(src, i, to);
        let next = if end < to { end + 1 } else { end };
        let line = &src[i..next];
        let body = line.trim_start();
        let indent = line.len() - line.trim_start().len();

        if body.starts_with("```") || body.starts_with("~~~") {
            let marker = if body.starts_with("```") {
                "```"
            } else {
                "~~~"
            };
            push(out, Role::Attribute, src, i, next);
            let info = body[3..].trim();
            let language = if info.is_empty() {
                None
            } else {
                Language::parse(first_word(info))
            };
            let (body_end, fence_end) = fence_extent(src, next, to, marker);
            if body_end > next {
                match language {
                    Some(language) => scan_code(out, language.grammar(), src, next, body_end),
                    None => push(out, Role::Plain, src, next, body_end),
                }
            }
            if fence_end > body_end {
                push(out, Role::Attribute, src, body_end, fence_end);
            }
            i = fence_end;
            continue;
        }

        if body.starts_with('#') {
            push(out, Role::Keyword, src, i, next);
            i = next;
            continue;
        }
        if body.starts_with('>') {
            push(out, Role::Comment, src, i, next);
            i = next;
            continue;
        }
        if is_rule(body) {
            push(out, Role::Punctuation, src, i, next);
            i = next;
            continue;
        }
        scan_markdown_inline(out, src, i + indent, next, i);
        i = next;
    }
}

/// One line of prose: inline code, links, and the marker a list item starts
/// with. Emphasis is deliberately not marked — `*` is a bullet, a multiplication
/// sign and two kinds of emphasis, and guessing wrong colours a whole paragraph.
fn scan_markdown_inline<'a>(
    out: &mut Vec<Token<'a>>,
    src: &'a str,
    start: usize,
    to: usize,
    line_start: usize,
) {
    let mut run = line_start;
    let mut i = start;

    // The list marker, if this line opens an item.
    let marker = list_marker(src, start, to);
    if marker > start {
        flush(out, src, &mut run, start);
        push(out, Role::Punctuation, src, start, marker);
        i = marker;
        run = i;
    }

    while i < to {
        let Some(c) = ch(src, i, to) else { break };
        if c == '`' {
            let ticks = run_of("`", src, i, to);
            if let Some(at) = find(src, ticks, to, &src[i..ticks]) {
                flush(out, src, &mut run, i);
                let end = at + (ticks - i);
                push(out, Role::String, src, i, end);
                i = end;
                run = i;
                continue;
            }
        }
        if c == '(' && i > start && ch(src, i - 1, to) == Some(']') {
            if let Some(at) = find_char(src, i + 1, to, ')') {
                flush(out, src, &mut run, i);
                push(out, Role::String, src, i, at + 1);
                i = at + 1;
                run = i;
                continue;
            }
        }
        i += c.len_utf8();
    }
    flush(out, src, &mut run, to);
}

fn list_marker(src: &str, start: usize, to: usize) -> usize {
    match ch(src, start, to) {
        Some('-' | '*' | '+') if ch(src, start + 1, to) == Some(' ') => start + 2,
        Some(c) if c.is_ascii_digit() => {
            let mut j = start;
            while ch(src, j, to).is_some_and(|d| d.is_ascii_digit()) {
                j += 1;
            }
            if matches!(ch(src, j, to), Some('.' | ')')) && ch(src, j + 1, to) == Some(' ') {
                j + 2
            } else {
                start
            }
        }
        _ => start,
    }
}

fn is_rule(body: &str) -> bool {
    let trimmed = body.trim_end();
    trimmed.len() >= 3
        && (trimmed.bytes().all(|b| b == b'-')
            || trimmed.bytes().all(|b| b == b'*')
            || trimmed.bytes().all(|b| b == b'_'))
}

/// Where a fenced block's body ends, and where its closing fence line ends.
fn fence_extent(src: &str, from: usize, to: usize, marker: &str) -> (usize, usize) {
    let mut i = from;
    while i < to {
        let end = line_end(src, i, to);
        let next = if end < to { end + 1 } else { end };
        if src[i..end].trim_start().starts_with(marker) {
            return (i, next);
        }
        i = next;
    }
    (to, to)
}

fn first_word(text: &str) -> &str {
    match text.find(|c: char| c.is_whitespace()) {
        Some(at) => &text[..at],
        None => text,
    }
}

// --- The primitives everything above is built from --------------------------

/// The character at `i`, or `None` at or past `to`.
fn ch(src: &str, i: usize, to: usize) -> Option<char> {
    if i >= to || i >= src.len() {
        return None;
    }
    src[i..].chars().next()
}

fn starts_with(src: &str, i: usize, to: usize, pattern: &str) -> bool {
    !pattern.is_empty() && i + pattern.len() <= to && src[i..].starts_with(pattern)
}

fn find(src: &str, from: usize, to: usize, pattern: &str) -> Option<usize> {
    if from > to {
        return None;
    }
    src[from..to].find(pattern).map(|at| from + at)
}

fn find_char(src: &str, from: usize, to: usize, needle: char) -> Option<usize> {
    if from > to {
        return None;
    }
    src[from..to].find(needle).map(|at| from + at)
}

fn find_ignoring_case(src: &str, from: usize, to: usize, pattern: &str) -> Option<usize> {
    let mut i = from;
    while i < to {
        if i + pattern.len() <= to && src[i..i + pattern.len()].eq_ignore_ascii_case(pattern) {
            return Some(i);
        }
        let c = ch(src, i, to)?;
        i += c.len_utf8();
    }
    None
}

fn line_end(src: &str, from: usize, to: usize) -> usize {
    find_char(src, from, to, '\n').unwrap_or(to)
}

fn skip_blank(src: &str, mut i: usize, to: usize) -> usize {
    while matches!(ch(src, i, to), Some(' ' | '\t')) {
        i += 1;
    }
    i
}

/// Pushes a token, unless it would be empty. An empty token would break nothing
/// and would appear in every parity fixture as noise.
fn push<'a>(out: &mut Vec<Token<'a>>, role: Role, src: &'a str, from: usize, to: usize) {
    if from < to {
        out.push(Token {
            role,
            text: &src[from..to],
        });
    }
}

/// Ends the unclassified run at `upto`, which is what makes the pass total.
fn flush<'a>(out: &mut Vec<Token<'a>>, src: &'a str, run: &mut usize, upto: usize) {
    if *run < upto {
        out.push(Token {
            role: Role::Plain,
            text: &src[*run..upto],
        });
    }
    *run = upto;
}
