//! Enough of a JavaScript reader to prove the JavaScript this crate emits is
//! JavaScript.
//!
//! # Why this exists at all
//!
//! Two generated components — `table.js` and `lineage.js` — did not parse, and
//! had not for an unknown length of time, with `cargo xtask check` green
//! throughout. Nothing in this repository had ever read an emitted `.js` as
//! code: the browser suite fetches a pre-built bundle, and a bundler that never
//! ran reported nothing. `table.js` bound a parameter named `export`, which is
//! a keyword; `lineage.js` declared `const filter` beside a parameter already
//! called `filter`. Both are *early errors* — the module fails before a single
//! statement runs, so every consumer importing the package got a
//! `SyntaxError` rather than a component.
//!
//! # Why it is written here rather than shelled out to `node`
//!
//! `NOCTUA.md §9.11` promises the Rust half builds and tests with no Node
//! installed, and `cargo xtask check` verifies that promise structurally. A
//! guard that ran `node --check` would either break the promise or skip itself
//! on the machines that need it most — and a guard that skips is the state this
//! module exists to end.
//!
//! # What it does and does not claim
//!
//! It is **not** a JavaScript engine and it does not type-check. It lexes the
//! real grammar — comments, strings, template literals with their
//! substitutions, regular expressions — and then asserts four things a
//! generated file must satisfy:
//!
//! | [`check`] asserts | The defect it would have caught |
//! |---|---|
//! | Every literal and comment is closed | a template literal missing its backtick |
//! | Brackets balance and match | a body emitted with one `)` too few |
//! | No reserved word is bound | `function Table({ export })` |
//! | No name is declared twice in one scope | `const filter` beside the `filter` parameter |
//!
//! and [`undeclared`] answers a fifth: every name a body *reads* is a name it
//! declared, took as a parameter, or imported. That one is a scope check by
//! approximation — it collects declarations without regard to which block they
//! are in — so it can miss a defect and cannot invent one. Missing a defect is
//! the acceptable direction for a guard nobody will be able to argue with.

use std::collections::BTreeSet;

/// What a token is. The text is carried separately, as a slice of the source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    /// An identifier, a keyword, or a reserved word — the lexer does not care
    /// which, because the difference is a question about position.
    Word,
    /// Anything punctuation: an operator or a bracket.
    Punct,
    /// A string, a template chunk, a number or a regular expression. Nothing
    /// reads a name out of one, so the contents are dropped — except a
    /// template's `${…}`, which is lexed as the code it is.
    Literal,
}

/// One token, pointing into the source it came from.
#[derive(Debug, Clone, Copy)]
struct Tok<'a> {
    kind: Kind,
    text: &'a str,
}

impl Tok<'_> {
    fn word(&self, name: &str) -> bool {
        self.kind == Kind::Word && self.text == name
    }

    fn punct(&self, symbol: &str) -> bool {
        self.kind == Kind::Punct && self.text == symbol
    }
}

/// Every word JavaScript refuses as a binding name in a module.
///
/// Modules are strict-mode code, so this is the keyword list plus the words
/// strict mode reserves for the future, plus `arguments` and `eval`, which
/// strict mode refuses as binding targets specifically. It is stated here as
/// well as in [`crate::props`] on purpose: one is the escape and one is the
/// check, and a check that imported its answer from the thing it checks would
/// agree with a broken escape.
const RESERVED: [&str; 48] = [
    "arguments",
    "await",
    "break",
    "case",
    "catch",
    "class",
    "const",
    "continue",
    "debugger",
    "default",
    "delete",
    "do",
    "else",
    "enum",
    "eval",
    "export",
    "extends",
    "false",
    "finally",
    "for",
    "function",
    "if",
    "implements",
    "import",
    "in",
    "instanceof",
    "interface",
    "let",
    "new",
    "null",
    "package",
    "private",
    "protected",
    "public",
    "return",
    "static",
    "super",
    "switch",
    "this",
    "throw",
    "true",
    "try",
    "typeof",
    "var",
    "void",
    "while",
    "with",
    "yield",
];

/// Words that are grammar rather than names, so reading one is not reading a
/// variable. A superset of [`RESERVED`]: `of`, `from`, `as`, `get` and `set`
/// are contextual, legal as bindings, and never a free variable here.
const GRAMMAR: [&str; 12] = [
    "of",
    "from",
    "as",
    "get",
    "set",
    "async",
    "undefined",
    "null",
    "true",
    "false",
    "NaN",
    "Infinity",
];

/// Names the language itself provides. A body naming one of these has not
/// forgotten to declare anything.
///
/// The list is short because the emitted bodies are: they render markup and do
/// arithmetic. Adding a global here is a deliberate widening of what a
/// generated file is allowed to reach for, which is why it is a list and not a
/// heuristic about capital letters.
const GLOBALS: [&str; 17] = [
    "Array",
    "Boolean",
    "Date",
    "Error",
    "JSON",
    "Map",
    "Math",
    "Number",
    "Object",
    "Promise",
    "RegExp",
    "Set",
    "String",
    "Symbol",
    "console",
    "globalThis",
    "parseInt",
];

/// Whether JavaScript refuses this word as a binding name.
#[must_use]
pub fn is_reserved(name: &str) -> bool {
    RESERVED.contains(&name)
}

/// Reads a module's structure, or says what is wrong with it.
///
/// # Errors
///
/// One sentence naming the defect and, where there is one, the line it is on.
pub fn check(source: &str) -> Result<(), String> {
    let tokens = lex(source)?;
    reserved_bindings(&tokens)?;
    redeclarations(&tokens)
}

/// Every name the module reads and never declares, imports or takes.
///
/// Empty is the answer for a module that holds together. Anything in it is a
/// `ReferenceError` waiting for the first render that reaches that line.
///
/// # Errors
///
/// The same lexing failure [`check`] reports, because a file that does not lex
/// has no identifiers worth listing.
pub fn undeclared(source: &str) -> Result<Vec<String>, String> {
    let tokens = lex(source)?;
    let declared = declarations(&tokens);
    let mut missing: BTreeSet<&str> = BTreeSet::new();
    for name in reads(&tokens) {
        if !declared.contains(name)
            && !GLOBALS.contains(&name)
            && !GRAMMAR.contains(&name)
            && !RESERVED.contains(&name)
        {
            missing.insert(name);
        }
    }
    Ok(missing.into_iter().map(str::to_owned).collect())
}

// --- The lexer ---------------------------------------------------------------

/// Where the reader is: which bracket it is inside, and whether a `}` closes a
/// block or resumes a template literal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Ctx {
    /// A `(`, a `[` or a `{`, carried as the character that opened it.
    Bracket(char),
    /// Inside the text of a template literal, between the backticks.
    Template,
    /// Inside a template's `${…}`, where ordinary code resumes.
    Substitution,
}

/// Splits a module into tokens, or says where it stops making sense.
#[allow(clippy::too_many_lines)] // One grammar, one function; splitting it
// would hide the fact that every branch here is exclusive with every other.
fn lex(source: &str) -> Result<Vec<Tok<'_>>, String> {
    let bytes = source.as_bytes();
    let mut at = 0usize;
    let mut out: Vec<Tok<'_>> = Vec::new();
    let mut stack: Vec<Ctx> = Vec::new();

    while at < bytes.len() {
        // A TEMPLATE'S TEXT IS NOT CODE. Scanning it as code is how a `${`
        // inside one gets read as a block and every bracket after it is off by
        // one — which is why this is a mode rather than a token kind.
        if stack.last() == Some(&Ctx::Template) {
            let start = at;
            loop {
                if at >= bytes.len() {
                    return Err(format!(
                        "line {}: a template literal is never closed",
                        line_of(source, start)
                    ));
                }
                match bytes[at] {
                    b'\\' => at += 2,
                    b'`' => {
                        at += 1;
                        stack.pop();
                        break;
                    }
                    b'$' if bytes.get(at + 1) == Some(&b'{') => {
                        at += 2;
                        stack.push(Ctx::Substitution);
                        break;
                    }
                    _ => at += 1,
                }
            }
            out.push(Tok {
                kind: Kind::Literal,
                text: "",
            });
            continue;
        }

        let byte = bytes[at];
        if byte.is_ascii_whitespace() {
            at += 1;
            continue;
        }

        // Comments, before the `/` below can mistake one for a division or a
        // regular expression.
        if byte == b'/' && bytes.get(at + 1) == Some(&b'/') {
            while at < bytes.len() && bytes[at] != b'\n' {
                at += 1;
            }
            continue;
        }
        if byte == b'/' && bytes.get(at + 1) == Some(&b'*') {
            let start = at;
            at += 2;
            loop {
                if at + 1 >= bytes.len() {
                    return Err(format!(
                        "line {}: a block comment is never closed",
                        line_of(source, start)
                    ));
                }
                if bytes[at] == b'*' && bytes[at + 1] == b'/' {
                    at += 2;
                    break;
                }
                at += 1;
            }
            continue;
        }

        if byte == b'"' || byte == b'\'' {
            let start = at;
            at += 1;
            loop {
                if at >= bytes.len() || bytes[at] == b'\n' {
                    return Err(format!(
                        "line {}: a string is never closed",
                        line_of(source, start)
                    ));
                }
                if bytes[at] == b'\\' {
                    at += 2;
                    continue;
                }
                if bytes[at] == byte {
                    at += 1;
                    break;
                }
                at += 1;
            }
            out.push(Tok {
                kind: Kind::Literal,
                text: "",
            });
            continue;
        }

        if byte == b'`' {
            at += 1;
            stack.push(Ctx::Template);
            continue;
        }

        if byte == b'/' && regex_here(out.last()) {
            let start = at;
            at += 1;
            let mut class = false;
            loop {
                if at >= bytes.len() || bytes[at] == b'\n' {
                    return Err(format!(
                        "line {}: a regular expression is never closed",
                        line_of(source, start)
                    ));
                }
                match bytes[at] {
                    b'\\' => at += 2,
                    b'[' => {
                        class = true;
                        at += 1;
                    }
                    b']' => {
                        class = false;
                        at += 1;
                    }
                    b'/' if !class => {
                        at += 1;
                        break;
                    }
                    _ => at += 1,
                }
            }
            while at < bytes.len() && (bytes[at] as char).is_ascii_alphabetic() {
                at += 1;
            }
            out.push(Tok {
                kind: Kind::Literal,
                text: "",
            });
            continue;
        }

        if byte.is_ascii_digit() {
            while at < bytes.len()
                && (bytes[at].is_ascii_alphanumeric() || bytes[at] == b'.' || bytes[at] == b'_')
            {
                at += 1;
            }
            out.push(Tok {
                kind: Kind::Literal,
                text: "",
            });
            continue;
        }

        if byte == b'_' || byte == b'$' || (byte as char).is_ascii_alphabetic() {
            let start = at;
            while at < bytes.len() {
                let here = bytes[at];
                if here == b'_' || here == b'$' || here.is_ascii_alphanumeric() {
                    at += 1;
                } else {
                    break;
                }
            }
            out.push(Tok {
                kind: Kind::Word,
                text: &source[start..at],
            });
            continue;
        }

        match byte {
            b'(' | b'[' | b'{' => {
                stack.push(Ctx::Bracket(byte as char));
                out.push(Tok {
                    kind: Kind::Punct,
                    text: &source[at..=at],
                });
                at += 1;
            }
            b')' | b']' | b'}' => {
                let wanted = match byte {
                    b')' => '(',
                    b']' => '[',
                    _ => '{',
                };
                match stack.pop() {
                    // A `}` closing a `${` puts the reader back inside the
                    // template it interrupted, which is still under it on the
                    // stack.
                    Some(Ctx::Substitution) if byte == b'}' => {
                        out.push(Tok {
                            kind: Kind::Literal,
                            text: "",
                        });
                        at += 1;
                        continue;
                    }
                    Some(Ctx::Bracket(open)) if open == wanted => {}
                    _ => {
                        return Err(format!(
                            "line {}: `{}` closes nothing",
                            line_of(source, at),
                            byte as char
                        ));
                    }
                }
                out.push(Tok {
                    kind: Kind::Punct,
                    text: &source[at..=at],
                });
                at += 1;
            }
            _ => {
                // The longest operator that starts here. `??` and `?.` have to
                // be one token each, or the `?` of a nullish coalesce would be
                // read as the start of a conditional and every `:` after it
                // would be counted against a ternary that is not there.
                let rest = &source[at..];
                let symbol = [
                    "...", "===", "!==", "=>", "??", "?.", "==", "!=", "<=", ">=",
                ]
                .into_iter()
                .find(|candidate| rest.starts_with(candidate))
                .unwrap_or(&source[at..=at]);
                out.push(Tok {
                    kind: Kind::Punct,
                    text: symbol,
                });
                at += symbol.len();
            }
        }
    }

    if let Some(open) = stack.first() {
        return Err(match open {
            Ctx::Bracket(char) => format!("the file ends with `{char}` still open"),
            _ => "the file ends inside a template literal".to_owned(),
        });
    }
    Ok(out)
}

/// Whether a `/` here opens a regular expression rather than dividing.
///
/// The standard reading: a regular expression may start where a value may
/// start, and a division may only follow one. `)` is division — `(a + b) / c` —
/// and so is a name, a literal and a `]`.
fn regex_here(previous: Option<&Tok<'_>>) -> bool {
    match previous {
        None => true,
        Some(token) => match token.kind {
            Kind::Literal => false,
            // `return /x/` is a regular expression; `count / 2` is not.
            Kind::Word => RESERVED.contains(&token.text),
            Kind::Punct => !matches!(token.text, ")" | "]" | "}"),
        },
    }
}

/// The one-based line a byte offset falls on.
fn line_of(source: &str, at: usize) -> usize {
    source[..at.min(source.len())]
        .bytes()
        .filter(|b| *b == b'\n')
        .count()
        + 1
}

// --- The four structural rules ----------------------------------------------

/// No reserved word may be bound.
fn reserved_bindings(tokens: &[Tok<'_>]) -> Result<(), String> {
    for (at, token) in tokens.iter().enumerate() {
        // `const export = …`, `function export(…)`, `class export`.
        if token.kind == Kind::Word
            && matches!(token.text, "const" | "let" | "var" | "function" | "class")
            && let Some(next) = tokens.get(at + 1)
            && next.kind == Kind::Word
            && RESERVED.contains(&next.text)
        {
            return Err(format!(
                "`{}` is a reserved word and cannot be bound by `{}`",
                next.text, token.text
            ));
        }
    }

    // And every parameter list, where `{ export }` is a binding while
    // `{ export: _export }` is a property name, which may be a keyword.
    for (at, token) in tokens.iter().enumerate() {
        if !token.word("function") {
            continue;
        }
        let Some((from, to)) = parameter_list(tokens, at) else {
            continue;
        };
        for name in bindings(&tokens[from..to]) {
            if RESERVED.contains(&name) {
                return Err(format!(
                    "`{name}` is a reserved word and cannot be a parameter name; \
                     destructure it as `{name}: _{name}`"
                ));
            }
        }
    }
    Ok(())
}

/// No name may be declared twice in one scope, parameters included.
fn redeclarations(tokens: &[Tok<'_>]) -> Result<(), String> {
    // One set per brace, seeded with the parameters of the function the brace
    // belongs to. A `const` at the top of a body colliding with a parameter is
    // the same early error as two `const`s of one name.
    let mut scopes: Vec<BTreeSet<&str>> = vec![BTreeSet::new()];
    let mut pending: Vec<&str> = Vec::new();
    let mut at = 0usize;

    while at < tokens.len() {
        let token = tokens[at];
        if token.punct("{") {
            let mut scope = BTreeSet::new();
            for name in pending.drain(..) {
                scope.insert(name);
            }
            scopes.push(scope);
            at += 1;
            continue;
        }
        if token.punct("}") {
            scopes.pop();
            if scopes.is_empty() {
                scopes.push(BTreeSet::new());
            }
            at += 1;
            continue;
        }
        // A function's parameters belong to the scope its BODY opens, and the
        // body opens after them — so the braces of a destructuring parameter
        // list are stepped over rather than treated as a scope of their own.
        // Seeding them into the pattern's own braces is what let
        // `Lineage({ filter })` sit beside `const filter` unreported.
        if token.word("function")
            && let Some((from, to)) = parameter_list(tokens, at)
        {
            pending = bindings(&tokens[from..to]);
            at = to + 1;
            continue;
        }
        // A `for` head declares into the LOOP, not into the scope around it, so
        // two sibling `for (let index …)` loops are not a redeclaration.
        if token.word("for")
            && let Some((from, to)) = parameter_list(tokens, at)
        {
            pending = if tokens
                .get(from)
                .is_some_and(|first| matches!(first.text, "const" | "let" | "var"))
            {
                let end = statement_end(tokens, from + 1).min(to);
                bindings(&tokens[from + 1..end.max(from + 1)])
            } else {
                Vec::new()
            };
            at = to + 1;
            continue;
        }
        if token.kind == Kind::Word && matches!(token.text, "const" | "let") {
            let end = statement_end(tokens, at + 1);
            for name in bindings(&tokens[at + 1..end.max(at + 1)]) {
                let scope = scopes.last_mut().expect("a scope is always open");
                if !scope.insert(name) {
                    return Err(format!(
                        "`{name}` is declared twice in one scope — a parameter and \
                         a `{}`, or two declarations",
                        token.text
                    ));
                }
            }
            at = end.max(at + 1);
            continue;
        }
        at += 1;
    }
    Ok(())
}

/// The names a pattern binds, reserved words included.
///
/// A name followed by `:` is a property key, which may be anything — `export`
/// is a perfectly ordinary key, and only the binding beside it is refused. The
/// reserved ones are LEFT IN because refusing them is the caller's job:
/// filtering here is what let `export` slip past the check that exists for it.
fn bindings<'a>(tokens: &[Tok<'a>]) -> Vec<&'a str> {
    let mut out = Vec::new();
    let mut at = 0usize;
    while at < tokens.len() {
        let token = tokens[at];
        if token.punct("=") {
            // A default value is an expression, not a pattern. Skip to the
            // next comma at this depth.
            at = next_comma(tokens, at, tokens.len());
            continue;
        }
        if token.kind == Kind::Word
            && !tokens.get(at + 1).is_some_and(|next| next.punct(":"))
            && !at
                .checked_sub(1)
                .and_then(|before| tokens.get(before))
                .is_some_and(|before| before.punct("."))
        {
            out.push(token.text);
        }
        at += 1;
    }
    out
}

/// The `(…)` after a `function` or a `catch`, as a range into `tokens`.
fn parameter_list(tokens: &[Tok<'_>], from: usize) -> Option<(usize, usize)> {
    let open = tokens
        .iter()
        .enumerate()
        .skip(from)
        .find(|(_, token)| token.punct("("))
        .map(|(at, _)| at)?;
    let close = matching(tokens, open)?;
    Some((open + 1, close))
}

/// Where a `const`/`let` statement's pattern stops: its `=`, or its `;`.
fn statement_end(tokens: &[Tok<'_>], from: usize) -> usize {
    let mut depth = 0i32;
    for (offset, token) in tokens.iter().enumerate().skip(from) {
        // `for (const entry of sets)` has no `=` and no `;`: the pattern ends
        // at `of`. Checked before the punctuation below, because `of` is a
        // word — reading it as one is how `for (const part of markLines…)`
        // came to look like a second declaration of `markLines`.
        if token.kind == Kind::Word && depth == 0 && matches!(token.text, "of" | "in") {
            return offset;
        }
        if token.kind != Kind::Punct {
            continue;
        }
        match token.text {
            "(" | "[" | "{" => depth += 1,
            ")" | "]" | "}" => depth -= 1,
            "=" | ";" | "," if depth == 0 => return offset,
            _ => {}
        }
        if depth < 0 {
            return offset;
        }
    }
    tokens.len()
}

/// The next comma at the current depth, or the end of the range.
fn next_comma(tokens: &[Tok<'_>], from: usize, to: usize) -> usize {
    let mut depth = 0i32;
    for (offset, token) in tokens.iter().enumerate().take(to).skip(from) {
        if token.kind != Kind::Punct {
            continue;
        }
        match token.text {
            "(" | "[" | "{" => depth += 1,
            ")" | "]" | "}" => depth -= 1,
            "," if depth == 0 => return offset,
            _ => {}
        }
    }
    to
}

/// The index of the bracket closing the one at `open`.
fn matching(tokens: &[Tok<'_>], open: usize) -> Option<usize> {
    let mut depth = 0i32;
    for (offset, token) in tokens.iter().enumerate().skip(open) {
        if token.kind != Kind::Punct {
            continue;
        }
        match token.text {
            "(" | "[" | "{" => depth += 1,
            ")" | "]" | "}" => {
                depth -= 1;
                if depth == 0 {
                    return Some(offset);
                }
            }
            _ => {}
        }
    }
    None
}

// --- Names read and names declared ------------------------------------------

/// Every name the module declares, takes as a parameter, or imports.
///
/// Flat rather than scoped: a name declared in one block and read in another
/// passes, which is a defect this cannot find and never a defect it invents.
fn declarations<'a>(tokens: &[Tok<'a>]) -> BTreeSet<&'a str> {
    let mut out: BTreeSet<&str> = BTreeSet::new();
    for (at, token) in tokens.iter().enumerate() {
        if token.word("import") {
            // `import { a as b, c } from "…"`, `import x from "…"`, and
            // `import * as x from "…"` — the binding is what is left of `from`.
            let end = tokens
                .iter()
                .enumerate()
                .skip(at)
                .find(|(_, candidate)| candidate.word("from"))
                .map_or(tokens.len(), |(offset, _)| offset);
            let mut renamed = false;
            for offset in at + 1..end {
                let here = tokens[offset];
                if here.word("as") {
                    renamed = true;
                    continue;
                }
                if here.kind != Kind::Word || here.word("type") {
                    continue;
                }
                // `a as b` binds `b`; without a rename every name binds itself.
                if renamed || !tokens.get(offset + 1).is_some_and(|next| next.word("as")) {
                    out.insert(here.text);
                }
                renamed = false;
            }
            continue;
        }
        if token.word("function") || token.word("catch") {
            if let Some(name) = tokens.get(at + 1).filter(|next| next.kind == Kind::Word) {
                out.insert(name.text);
            }
            if let Some((from, to)) = parameter_list(tokens, at) {
                for name in bindings(&tokens[from..to]) {
                    out.insert(name);
                }
            }
            continue;
        }
        if token.kind == Kind::Word && matches!(token.text, "const" | "let" | "var") {
            let end = statement_end(tokens, at + 1).max(at + 1);
            for name in bindings(&tokens[at + 1..end]) {
                out.insert(name);
            }
            continue;
        }
        // An arrow function's parameters: `(a, b) => …` or `a => …`.
        if token.punct("=>") {
            let Some(before) = at.checked_sub(1) else {
                continue;
            };
            if tokens[before].punct(")") {
                if let Some(open) = opening(tokens, before) {
                    for here in tokens.iter().take(before).skip(open + 1) {
                        if here.kind == Kind::Word && !RESERVED.contains(&here.text) {
                            out.insert(here.text);
                        }
                    }
                }
            } else if tokens[before].kind == Kind::Word {
                out.insert(tokens[before].text);
            }
        }
    }
    out
}

/// The index of the bracket the one at `close` closes.
fn opening(tokens: &[Tok<'_>], close: usize) -> Option<usize> {
    let mut depth = 0i32;
    for offset in (0..=close).rev() {
        let token = tokens[offset];
        if token.kind != Kind::Punct {
            continue;
        }
        match token.text {
            ")" | "]" | "}" => depth += 1,
            "(" | "[" | "{" => {
                depth -= 1;
                if depth == 0 {
                    return Some(offset);
                }
            }
            _ => {}
        }
    }
    None
}

/// Every name the module reads as a value.
///
/// Not a property key — `{ className: root }` reads `root` and names
/// `className` — and not a member, because `piece.label` reads `piece` and
/// nothing else. Telling a key's `:` from a conditional's needs the `?` counted
/// per bracket, which is what the stack below is for: `{ role: a ? b : c }` has
/// one key colon and one conditional colon, and reading the second as a key
/// would lose `b` from the check.
fn reads<'a>(tokens: &[Tok<'a>]) -> Vec<&'a str> {
    let mut out = Vec::new();
    // (the bracket that opened this context, how many `?` are waiting for a `:`)
    let mut stack: Vec<(&str, usize)> = vec![("", 0)];
    // An import statement reads nothing: `createElement` in
    // `import { createElement as h }` is a name in somebody else's module.
    // The brackets in it still have to be counted, so this is a flag rather
    // than a jump.
    let mut importing = false;

    for (at, token) in tokens.iter().enumerate() {
        if token.word("import") {
            importing = true;
        }
        match token.kind {
            Kind::Punct => match token.text {
                ";" => importing = false,
                "(" | "[" | "{" => stack.push((token.text, 0)),
                ")" | "]" | "}" => {
                    if stack.len() > 1 {
                        stack.pop();
                    }
                }
                "?" => {
                    if let Some(top) = stack.last_mut() {
                        top.1 += 1;
                    }
                }
                ":" => {
                    if let Some(top) = stack.last_mut()
                        && top.1 > 0
                    {
                        top.1 -= 1;
                    }
                }
                _ => {}
            },
            Kind::Word => {
                if importing || RESERVED.contains(&token.text) || GRAMMAR.contains(&token.text) {
                    continue;
                }
                // A member, not a name: `piece.label`.
                if at
                    .checked_sub(1)
                    .and_then(|before| tokens.get(before))
                    .is_some_and(|before| before.punct(".") || before.punct("?."))
                {
                    continue;
                }
                // A property key, in an object literal or an object pattern.
                let key = tokens.get(at + 1).is_some_and(|next| next.punct(":"))
                    && stack
                        .last()
                        .is_some_and(|(open, ternary)| *open == "{" && *ternary == 0);
                if !key {
                    out.push(token.text);
                }
            }
            Kind::Literal => {}
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_module_that_holds_together_passes() {
        let source = r#"
import { createElement as h } from "react";
import { Icon } from "./icon.js";

export function Thing({ name, wrap, export: _export, className, ...rest }) {
  const wrap_ = wrap ?? "nowrap";
  const root = ["nd-thing", className].filter(Boolean).join(" ");
  const total = name.length / 2;
  const words = name.split(/\s+/u).map((word) => `${word}-${wrap_}`);
  return h("div", { className: root, "data-nd-wrap": wrap_, key: _export ? "a" : "b", ...rest }, words, total, h(Icon, { icon: "star" }));
}
"#;
        check(source).expect("this module is fine");
        assert_eq!(undeclared(source).expect("lexes"), Vec::<String>::new());
    }

    /// The `table.js` defect, in miniature.
    #[test]
    fn a_reserved_word_cannot_be_a_parameter() {
        let source = "export function Table({ name, export, className }) {\n  return name;\n}\n";
        let problem = check(source).expect_err("`export` is a keyword");
        assert!(problem.contains("export"), "{problem}");
    }

    /// The `lineage.js` defect, in miniature.
    #[test]
    fn a_parameter_cannot_be_declared_again() {
        let source = "export function Lineage({ filter }) {\n  const filter = filter_ ?? \"\";\n  return filter;\n}\n";
        let problem = check(source).expect_err("`filter` is already the parameter");
        assert!(problem.contains("filter"), "{problem}");
    }

    /// The other half of the `lineage.js` defect.
    #[test]
    fn a_name_from_nowhere_is_reported() {
        let source = "export function Figure({ at }) {\n  return [root, at, shape_];\n}\n";
        let missing = undeclared(source).expect("lexes");
        assert_eq!(missing, vec!["root".to_owned(), "shape_".to_owned()]);
    }

    #[test]
    fn a_template_substitution_is_code_and_its_text_is_not() {
        // `name` and `piece` are read; the `const x` in the template's TEXT is
        // a run of characters and declares nothing.
        let source = "const stem = `nd-${name}-${piece.id} const x`;\n";
        assert_eq!(
            undeclared(source).expect("lexes"),
            vec!["name".to_owned(), "piece".to_owned()]
        );
    }

    #[test]
    fn a_conditional_colon_is_not_a_property_name() {
        // `chosen` sits where a key sits and is a value, because of the `?`.
        let source = "const a = { role: flag ? chosen : \"none\" };\n";
        let missing = undeclared(source).expect("lexes");
        assert_eq!(missing, vec!["chosen".to_owned(), "flag".to_owned()]);
    }

    #[test]
    fn a_regular_expression_is_not_a_division_and_the_reverse() {
        check("const a = b.split(/[/]+/u);\nconst c = d / e / f;\n").expect("both read");
        assert_eq!(
            undeclared("const c = d / e;\n").expect("lexes"),
            vec!["d".to_owned(), "e".to_owned()]
        );
    }

    #[test]
    fn an_unclosed_literal_is_reported() {
        assert!(check("const a = \"open;\n").is_err());
        assert!(check("const a = `open;\n").is_err());
        assert!(check("const a = 1; /* open\n").is_err());
    }

    #[test]
    fn brackets_have_to_match() {
        assert!(check("function a(] {}\n").is_err());
        assert!(check("function a() {\n").is_err());
        assert!(check("const a = 1);\n").is_err());
    }
}
