//! The shape of `spec/grammars/<name>.toml`: a language, described as data.
//!
//! # Why a grammar is a table and not a program
//!
//! A code block has to be highlighted in **two languages**. The Dioxus crate
//! renders it during SSR and the React package renders it in a browser, and
//! neither may reach for `docs-site/assets/shell.js` — the packages do not ship
//! it, and this system ships no runtime. So the tokeniser exists twice, and two
//! implementations that were each written by hand would disagree within a week.
//!
//! What is written once is the **algorithm** (in `noctua-design-emit`, both
//! languages side by side, the way every custom component body already is). What
//! is written per language is a **table**: the words, the delimiters, the shape
//! of a number. A new language is a file here and nothing else.
//!
//! # Why there are no regular expressions
//!
//! There is no regex crate in this workspace and none may be added
//! (`NOCTUA.md §2.3` — a dependency here is one every consumer inherits). A
//! JavaScript `RegExp` would be free and would only make the problem worse: the
//! two halves have to agree exactly, and two regex dialects do not.
//!
//! So a grammar is delimiters, character classes, word lists and a handful of
//! flags — all of which a hand-written scanner can honour identically in both
//! languages. The ceiling that puts on accuracy is real and documented in
//! `Context`: there is no scope resolution and no type resolution, so `Foo` is a
//! type because it is capitalised.
//!
//! # The invariant that makes the ceiling survivable
//!
//! **Concatenating every token's text reproduces the input, byte for byte.**
//! Every miss degrades to `plain`; nothing can be hidden, reordered or dropped.
//! A highlighter that is imperfect and *provably* never destructive is a
//! different kind of artifact from one that is better and might swallow a line.

use indexmap::IndexMap;
use serde::Deserialize;

use crate::error::{Problem, SpecError};

/// What a run of characters is, as far as a colour is concerned.
///
/// Seventeen, and several share a paint — five hues is the measured ceiling for
/// a categorical set in this colour system (`NOCTUA.md §8.8` rule 6), so a
/// larger role set buys expressiveness in the grammar rather than in the theme.
///
/// This list is **not** spec-driven. It is a property of the scanner, and
/// `spec/syntax.toml` is checked against it rather than the other way round: a
/// theme that failed to paint a role would leave that role looking unstyled.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Role {
    /// Identifiers, whitespace, and anything the grammar could not place.
    Plain,
    /// `+`, `=>`, `::`, and the words some languages spell them with.
    Operator,
    /// `fn`, `if`, `return`, `import`.
    Keyword,
    /// `(){}[],;`.
    Punctuation,
    /// Every comment form, including documentation comments.
    Comment,
    /// A string, a character literal, a template literal.
    String,
    /// An escape sequence inside a string: `\n`, `\u{1f}`.
    Escape,
    /// `42`, `0x1f`, `1.5e3`, `10px`.
    Number,
    /// `true`, `null`, `PI`, an enum variant, a CSS colour.
    Constant,
    /// A type, a class, a namespace.
    Type,
    /// A markup element name.
    Tag,
    /// A call, a definition, a macro, a builtin.
    Function,
    /// An object key, a CSS property, a struct field, a variable sigil.
    Property,
    /// An annotation, a decorator, a preprocessor line, a markup attribute.
    Attribute,
    /// A diff line that adds.
    Inserted,
    /// A diff line that removes.
    Deleted,
    /// A construct the scanner reached the end of the input inside.
    Invalid,
}

impl Role {
    /// Every role, in declaration order.
    pub const ALL: [Self; 17] = [
        Self::Plain,
        Self::Operator,
        Self::Keyword,
        Self::Punctuation,
        Self::Comment,
        Self::String,
        Self::Escape,
        Self::Number,
        Self::Constant,
        Self::Type,
        Self::Tag,
        Self::Function,
        Self::Property,
        Self::Attribute,
        Self::Inserted,
        Self::Deleted,
        Self::Invalid,
    ];

    /// Its name in a spec file and in the emitted `data-nd-tok`.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Plain => "plain",
            Self::Operator => "operator",
            Self::Keyword => "keyword",
            Self::Punctuation => "punctuation",
            Self::Comment => "comment",
            Self::String => "string",
            Self::Escape => "escape",
            Self::Number => "number",
            Self::Constant => "constant",
            Self::Type => "type",
            Self::Tag => "tag",
            Self::Function => "function",
            Self::Property => "property",
            Self::Attribute => "attribute",
            Self::Inserted => "inserted",
            Self::Deleted => "deleted",
            Self::Invalid => "invalid",
        }
    }

    /// Its `PascalCase` spelling, for the generated Rust variant.
    #[must_use]
    pub const fn variant(self) -> &'static str {
        match self {
            Self::Plain => "Plain",
            Self::Operator => "Operator",
            Self::Keyword => "Keyword",
            Self::Punctuation => "Punctuation",
            Self::Comment => "Comment",
            Self::String => "String",
            Self::Escape => "Escape",
            Self::Number => "Number",
            Self::Constant => "Constant",
            Self::Type => "Type",
            Self::Tag => "Tag",
            Self::Function => "Function",
            Self::Property => "Property",
            Self::Attribute => "Attribute",
            Self::Inserted => "Inserted",
            Self::Deleted => "Deleted",
            Self::Invalid => "Invalid",
        }
    }

    /// The role a spec file names, or `None` if it names nothing.
    #[must_use]
    pub fn parse(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|role| role.as_str() == name)
    }
}

impl<'de> serde::Deserialize<'de> for Role {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let name = String::deserialize(deserializer)?;
        Self::parse(&name).ok_or_else(|| {
            serde::de::Error::custom(format!(
                "`{name}` is not a role; the roles are {}",
                Self::ALL
                    .iter()
                    .map(|r| r.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ))
        })
    }
}

/// Which scanner reads this language.
///
/// Five, and the count is a claim: everything C-shaped, everything ML-shaped,
/// every configuration format and every query language is [`Mode::Code`]. The
/// other four exist because they are not sequences of tokens at all — markup is
/// a tree, markdown is a document, a diff is a list of lines, and plain text is
/// one token.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    /// The general scanner: words, comments, strings, numbers, symbols.
    #[default]
    Code,
    /// Angle-bracket markup: tags, attributes, entities, embedded islands.
    Markup,
    /// A document: headings, fences, inline code, links.
    Markdown,
    /// Line-oriented: the first character of a line decides the whole line.
    Diff,
    /// One token. The honest answer for text nobody claimed a grammar for.
    Plain,
}

impl Mode {
    /// Its `PascalCase` spelling, for the generated Rust variant.
    #[must_use]
    pub const fn variant(self) -> &'static str {
        match self {
            Self::Code => "Code",
            Self::Markup => "Markup",
            Self::Markdown => "Markdown",
            Self::Diff => "Diff",
            Self::Plain => "Plain",
        }
    }
}

/// One language.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Grammar {
    /// What it is called and how it is reached.
    pub meta: Meta,

    /// What an identifier is made of.
    #[serde(default)]
    pub identifier: Identifier,

    /// The words that are not identifiers, keyed by the role they take.
    #[serde(default)]
    pub words: IndexMap<Role, Vec<String>>,

    /// Every comment form. Order is irrelevant: the scanner matches longest
    /// first, so `///` beats `//` without anyone having to remember it.
    #[serde(default, rename = "comment")]
    pub comments: Vec<Comment>,

    /// Every string form, matched longest-opener first for the same reason.
    #[serde(default, rename = "string")]
    pub strings: Vec<StringForm>,

    /// What a number looks like.
    #[serde(default)]
    pub number: Number,

    /// The two character sets that are neither identifier nor whitespace.
    #[serde(default)]
    pub symbols: Symbols,

    /// Marks that colour the name after them: `#[`, `@`, `$`.
    #[serde(default, rename = "sigil")]
    pub sigils: Vec<Sigil>,

    /// The handful of decisions that need one token of context.
    #[serde(default)]
    pub context: Context,

    /// What this grammar is claimed to get right, and what it must not get
    /// wrong. Every entry is a trap somebody would otherwise re-discover.
    #[serde(default, rename = "case")]
    pub cases: Vec<Case>,
}

/// What a language is called and how it is reached.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Meta {
    /// The name the generated enum carries, in `kebab-case`.
    pub name: String,
    /// How it is written for a reader: `TypeScript`, not `typescript`.
    pub display: String,
    /// Which scanner reads it.
    #[serde(default)]
    pub mode: Mode,
    /// Other names a caller may pass: `rs`, `sh`, `yml`.
    #[serde(default)]
    pub aliases: Vec<String>,
}

/// What an identifier is made of.
///
/// Letters always count, and digits always count after the first character. A
/// grammar declares only what is unusual about it.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Identifier {
    /// Characters that are identifier characters besides letters and digits.
    #[serde(default = "underscore")]
    pub extra: String,
    /// Whether a character outside ASCII counts as a letter.
    ///
    /// True almost everywhere, and the reason it is a flag is that a language
    /// where it is false wants those characters to fall through to `plain`
    /// rather than glue themselves onto the identifier before them.
    #[serde(default = "yes")]
    pub unicode: bool,
}

fn underscore() -> String {
    "_".to_owned()
}

const fn yes() -> bool {
    true
}

impl Default for Identifier {
    fn default() -> Self {
        Self {
            extra: underscore(),
            unicode: true,
        }
    }
}

/// One comment form.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Comment {
    /// What opens it.
    pub open: String,
    /// What closes it. Empty means the end of the line.
    #[serde(default)]
    pub close: String,
    /// Whether an inner `open` has to be matched by an inner `close`.
    #[serde(default)]
    pub nestable: bool,
}

/// One string form.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StringForm {
    /// What opens it.
    pub open: String,
    /// What closes it. Empty means the end of the line.
    #[serde(default)]
    pub close: String,
    /// The character that escapes the next one. Empty means the form is raw.
    #[serde(default)]
    pub escape: String,
    /// Whether it may cross a newline.
    #[serde(default)]
    pub multiline: bool,
    /// A cap, in characters, after which an unclosed form gives up and the
    /// opener is re-read as something else.
    ///
    /// **This is what stops a Rust lifetime being an unterminated character
    /// literal.** `'a` in `&'a str` opens the `'…'` form, and without a cap the
    /// scanner would eat the rest of the line looking for a close that is not
    /// coming. With `max = 12` it gives up after twelve characters and `'`
    /// falls through to the sigil table, which paints a lifetime as a type.
    #[serde(default)]
    pub max: usize,
    /// What opens an interpolation inside it. Empty means it has none.
    #[serde(default)]
    pub interpolate_open: String,
    /// What closes one.
    #[serde(default)]
    pub interpolate_close: String,
    /// A mark inside the string that, with the name after it, is a hole:
    /// `"$HOME/bin"`.
    ///
    /// A second interpolation form rather than a variant of the first, because
    /// it has no closer — it ends where the name ends. Shell's commonest
    /// interpolation is this one, and a grammar that only knew `${…}` would
    /// paint `"$HOME/bin"` as four unbroken literal characters plus a path.
    #[serde(default)]
    pub interpolate_sigil: String,
}

/// What a number looks like.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Number {
    /// Radix prefixes: `0x`, `0b`, `0o`.
    #[serde(default)]
    pub prefixes: Vec<String>,
    /// A character that may sit between digits and mean nothing: `_`.
    #[serde(default)]
    pub separator: String,
    /// The letters that introduce an exponent: `eE`, or `eEpP` with hex floats.
    #[serde(default)]
    pub exponent: String,
    /// Whether identifier characters immediately after the digits belong to the
    /// number: `10px`, `1_000u64`, `3n`.
    #[serde(default)]
    pub suffix: bool,
    /// Whether `.5` is a number rather than an operator and a number.
    #[serde(default)]
    pub leading_dot: bool,
}

/// The two character sets that are neither identifier nor whitespace.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Symbols {
    /// Characters that combine into operators.
    #[serde(default)]
    pub operator: String,
    /// Characters that stand alone.
    #[serde(default)]
    pub punctuation: String,
}

/// A mark that colours the name after it.
///
/// `#[derive` in Rust, `@Override` in Java, `$name` in shell, `#fff` in CSS.
/// One mechanism, because they are one thing: a sigil and the identifier it
/// binds to are read together and mean something the identifier alone does not.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Sigil {
    /// The mark.
    pub open: String,
    /// What the mark and the name after it become.
    pub role: Role,
    /// Whether it only counts as the first thing on a line — which is what
    /// makes `#include` a directive and `a # b` an operator.
    #[serde(default)]
    pub line_start: bool,
}

/// Where an identifier is coloured by what is next to it.
///
/// Six flags and two tables, and clippy is right that six booleans usually mean
/// a missing type — but not here. Each one is an **independent** claim about a
/// language (`call_paren` and `upper_type` are true together in Rust, apart in
/// shell, and neither in JSON), so there are 2⁶ legitimate combinations and the
/// state machine that replaced them would have sixty-four states.
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Context {
    /// An identifier immediately before `(` is a call.
    #[serde(default)]
    pub call_paren: bool,
    /// An identifier starting with an upper-case letter is a type.
    ///
    /// This is the whole of this system's type resolution, and it is why Python
    /// paints `Foo = 3` as a type. A grammar turns it off where an initial
    /// capital means something else — in shell it means an environment variable.
    #[serde(default)]
    pub upper_type: bool,
    /// An identifier in `SCREAMING_SNAKE_CASE` is a constant.
    #[serde(default)]
    pub screaming_constant: bool,
    /// An identifier immediately after `.` is a member.
    #[serde(default)]
    pub member_dot: bool,
    /// What may become a property by being followed by `:`.
    #[serde(default)]
    pub property_colon: PropertyKey,
    /// Whether an identifier followed by `=` is a property. For `key = value`
    /// formats; never for a language where `=` is assignment.
    #[serde(default)]
    pub property_equals: bool,
    /// Characters that may prefix a string opener and belong to the string:
    /// `r"…"`, `f"…"`, `b"…"`.
    #[serde(default)]
    pub string_prefix: String,
    /// The word before an identifier, and what that makes the identifier.
    ///
    /// `fn` names a function and `struct` names a type, in every language that
    /// has them, and no amount of looking at the identifier itself can tell.
    /// This is the only lookbehind the scanner has and it is one token deep —
    /// enough for a declaration, and far short of a parser.
    #[serde(default)]
    pub name_after: IndexMap<String, Role>,
    /// Whether words are matched without regard to case. SQL, and very little
    /// else — a language where `Select` and `select` are one word is a language
    /// whose keyword table would otherwise need every casing anyone writes.
    #[serde(default)]
    pub case_insensitive: bool,
}

/// What a `:` may promote to a property.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PropertyKey {
    /// Nothing. The default, because in most languages `:` is a type
    /// annotation or a ternary and promoting the name before it is wrong.
    #[default]
    None,
    /// Only a quoted string, which is what JSON has.
    String,
    /// A string or an identifier.
    Any,
}

/// One thing this grammar must get right.
///
/// `expect` is a **subsequence**, not the whole stream: naming every token of a
/// realistic sample would be thousands of lines nobody reviews, and the token
/// stream in full is already guarded by the totality property. What belongs
/// here is the trap — the construct that was got wrong once, or that another
/// implementation gets wrong.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Case {
    /// What it is about, in English.
    pub name: String,
    /// The input.
    pub code: String,
    /// `<role> <text>` pairs that must appear in the token stream, in order,
    /// though not necessarily adjacently.
    #[serde(default)]
    pub expect: Vec<String>,
    /// `<role> <text>` pairs that must not appear at all.
    #[serde(default)]
    pub reject: Vec<String>,
}

impl Case {
    /// `expect` and `reject`, split into the pairs they spell.
    ///
    /// The separator is the **first** space, so the text may contain spaces and
    /// may be a single space itself: `"plain  "` is one space of plain text.
    #[must_use]
    pub fn pairs(entries: &[String]) -> Vec<(Role, String)> {
        entries
            .iter()
            .filter_map(|entry| {
                let (role, text) = entry.split_once(' ')?;
                Some((Role::parse(role)?, text.to_owned()))
            })
            .collect()
    }
}

/// Parses and validates one grammar.
///
/// # Errors
///
/// Returns every syntax and semantic problem found, in one diagnostic.
pub fn parse(path: &str, text: &str) -> Result<Grammar, SpecError> {
    let grammar: Grammar = match toml::from_str(text) {
        Ok(grammar) => grammar,
        Err(error) => {
            let problem = match error.span() {
                Some(span) => Problem::at(span, error.message().to_owned())
                    .labelled("invalid here")
                    .fix("check the key against spec/grammars/rust.toml, which uses every key"),
                None => Problem::whole_file(error.message().to_owned()),
            };
            return Err(SpecError::new(path, text.to_owned(), vec![problem]));
        }
    };

    let problems = check(&grammar);
    if problems.is_empty() {
        Ok(grammar)
    } else {
        Err(SpecError::new(path, text.to_owned(), problems))
    }
}

/// Everything that is well-formed TOML and still wrong.
fn check(grammar: &Grammar) -> Vec<Problem> {
    let mut problems = check_names(grammar);
    problems.extend(check_forms(grammar));
    problems
}

/// The language's own name, its aliases, and every word it lists.
fn check_names(grammar: &Grammar) -> Vec<Problem> {
    let mut problems = Vec::new();
    let name = &grammar.meta.name;

    if !name
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        || name.is_empty()
    {
        problems.push(
            Problem::whole_file(format!("`{name}` is not a language name")).fix(
                "Lower-case ASCII, digits and hyphens. It becomes an enum variant in two \
                 languages and a `data-nd-language` value in the markup.",
            ),
        );
    }

    for alias in &grammar.meta.aliases {
        if alias == name {
            problems.push(
                Problem::whole_file(format!("`{name}` lists its own name as an alias"))
                    .fix("The name always resolves. Remove it from `aliases`."),
            );
        }
    }

    // A WORD THAT CANNOT BE SCANNED CAN NEVER MATCH. The scanner looks a word
    // up only after it has read an identifier, so a "keyword" containing a
    // character the identifier rules exclude is a line in this file that does
    // nothing — which is the failure mode `deny_unknown_fields` exists to stop
    // one level up, and it deserves the same treatment.
    for (role, list) in &grammar.words {
        for word in list {
            if word.is_empty() {
                problems.push(
                    Problem::whole_file(format!(
                        "`{name}` declares an empty {} word",
                        role.as_str()
                    ))
                    .fix("Remove it."),
                );
                continue;
            }
            let bad: String = word
                .chars()
                .filter(|c| !is_identifier_char(*c, &grammar.identifier))
                .collect();
            if !bad.is_empty() && grammar.meta.mode == Mode::Code {
                problems.push(
                    Problem::whole_file(format!(
                        "`{name}` lists `{word}` as a {} word, but `{bad}` is not an identifier \
                         character",
                        role.as_str()
                    ))
                    .fix(
                        "The scanner looks a word up only after reading an identifier, so this \
                         entry can never match. Add the character to `[identifier] extra`, or put \
                         the word in `[symbols] operator` instead.",
                    ),
                );
            }
        }
    }

    let mut seen: Vec<&str> = Vec::new();
    for list in grammar.words.values() {
        for word in list {
            if seen.contains(&word.as_str()) {
                problems.push(
                    Problem::whole_file(format!("`{name}` lists `{word}` under two roles"))
                        .fix("A word has one colour. Keep whichever is more useful to a reader."),
                );
            }
            seen.push(word);
        }
    }

    problems
}

/// The delimiters, the marks, and what the cases claim about them.
fn check_forms(grammar: &Grammar) -> Vec<Problem> {
    let mut problems = Vec::new();
    let name = &grammar.meta.name;

    for comment in &grammar.comments {
        if comment.open.is_empty() {
            problems.push(
                Problem::whole_file(format!("`{name}` has a comment form with no opener"))
                    .fix("`open` is what starts the comment: `//`, `#`, `/*`."),
            );
        }
        if comment.nestable && comment.close.is_empty() {
            problems.push(
                Problem::whole_file(format!(
                    "`{name}` has a nestable comment that ends at the line"
                ))
                .fix("Nesting needs a close. Drop `nestable`, or give it one."),
            );
        }
    }

    for string in &grammar.strings {
        if string.open.is_empty() {
            problems.push(
                Problem::whole_file(format!("`{name}` has a string form with no opener"))
                    .fix("`open` is what starts the string."),
            );
        }
        if string.interpolate_open.is_empty() != string.interpolate_close.is_empty() {
            problems.push(
                Problem::whole_file(format!(
                    "`{name}` has a string form with half an interpolation"
                ))
                .fix("`interpolate_open` and `interpolate_close` come as a pair."),
            );
        }
    }

    for sigil in &grammar.sigils {
        if sigil.open.is_empty() {
            problems.push(
                Problem::whole_file(format!("`{name}` has a sigil with no mark"))
                    .fix("`open` is the mark: `#[`, `@`, `$`."),
            );
        }
    }

    for case in &grammar.cases {
        for entry in case.expect.iter().chain(&case.reject) {
            let spelled = entry
                .split_once(' ')
                .and_then(|(role, _)| Role::parse(role));
            if spelled.is_none() {
                problems.push(
                    Problem::whole_file(format!(
                        "`{name}` case `{}` expects `{entry}`, which names no role",
                        case.name
                    ))
                    .fix(
                        "An entry is `<role> <text>`, split at the first space. The roles are \
                         listed in crates/noctua-design-spec/src/grammar.rs.",
                    ),
                );
            }
        }
    }

    problems
}

/// Whether a character may appear inside an identifier under these rules.
///
/// Shared with the scanner's own definition by being the thing the scanner is
/// generated from — see `noctua-design-emit`'s `scanner` module, which honours
/// `extra` and `unicode` the same way.
#[must_use]
pub fn is_identifier_char(c: char, rules: &Identifier) -> bool {
    c.is_ascii_alphanumeric()
        || rules.extra.contains(c)
        || (rules.unicode && !c.is_ascii() && c.is_alphanumeric())
}

/// Loads every grammar under `spec/grammars/`, sorted by name.
///
/// # Errors
///
/// A missing directory, any grammar that does not validate, or two grammars
/// that answer to the same name.
pub fn load_all(directory: impl AsRef<std::path::Path>) -> Result<Vec<Grammar>, SpecError> {
    let directory = directory.as_ref();
    let display = directory.display().to_string();
    let entries = std::fs::read_dir(directory).map_err(|error| {
        SpecError::new(
            &display,
            String::new(),
            vec![
                Problem::whole_file(format!("could not read {display}: {error}"))
                    .fix("Create spec/grammars/, one <name>.toml per language."),
            ],
        )
    })?;

    let mut paths: Vec<_> = entries
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|e| e == "toml"))
        .collect();
    // SORTED, because the emitted enum's variant order is its `ALL` order and a
    // directory listing is whatever the filesystem felt like. CI builds twice
    // and diffs.
    paths.sort();

    let mut grammars = Vec::new();
    for path in paths {
        let text = std::fs::read_to_string(&path).map_err(|error| {
            SpecError::new(
                &path.display().to_string(),
                String::new(),
                vec![Problem::whole_file(format!("could not read: {error}"))],
            )
        })?;
        grammars.push(parse(&path.display().to_string(), &text)?);
    }

    // ONE NAME, ONE GRAMMAR. `sh` meaning nothing because two files claimed it
    // is a silent failure: the caller gets `plain` and no error anywhere.
    let mut problems = Vec::new();
    let mut claimed: Vec<(&str, &str)> = Vec::new();
    for grammar in &grammars {
        for name in std::iter::once(&grammar.meta.name).chain(&grammar.meta.aliases) {
            if let Some((_, owner)) = claimed.iter().find(|(claim, _)| *claim == name) {
                problems.push(
                    Problem::whole_file(format!(
                        "`{name}` is claimed by both `{owner}` and `{}`",
                        grammar.meta.name
                    ))
                    .fix("A name resolves to one grammar. Remove one of the aliases."),
                );
            }
            claimed.push((name, &grammar.meta.name));
        }
    }
    if problems.is_empty() {
        Ok(grammars)
    } else {
        Err(SpecError::new(&display, String::new(), problems))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RUST: &str = r#"
[meta]
name = "rust"
display = "Rust"
aliases = ["rs"]

[words]
keyword = ["fn", "let"]
type = ["u32"]

[[comment]]
open = "//"

[[string]]
open = '"'
close = '"'
escape = "\\"
"#;

    #[test]
    fn a_grammar_parses_and_keeps_its_order() {
        let grammar = parse("rust.toml", RUST).expect("valid");
        assert_eq!(grammar.meta.name, "rust");
        assert_eq!(grammar.meta.mode, Mode::Code);
        assert_eq!(grammar.words[&Role::Keyword], ["fn", "let"]);
        assert_eq!(grammar.comments[0].close, "");
    }

    /// A word the scanner can never read is a line that does nothing, and a
    /// line that does nothing is the failure this whole format is shaped to
    /// prevent.
    #[test]
    fn a_word_that_is_not_an_identifier_is_rejected() {
        let source = "[meta]\nname = \"x\"\ndisplay = \"X\"\n[words]\noperator = [\"=>\"]\n";
        let error = parse("t.toml", source).expect_err("must fail");
        assert!(
            error.problems().iter().any(|p| p.message().contains("=>")),
            "{:?}",
            error.problems()[0].message()
        );
    }

    #[test]
    fn one_word_may_not_have_two_colours() {
        let source = "[meta]\nname = \"x\"\ndisplay = \"X\"\n\
                      [words]\nkeyword = [\"if\"]\ntype = [\"if\"]\n";
        let error = parse("t.toml", source).expect_err("must fail");
        assert!(error.problems()[0].message().contains("two roles"));
    }

    #[test]
    fn a_nestable_comment_needs_something_to_nest_inside() {
        let source =
            "[meta]\nname = \"x\"\ndisplay = \"X\"\n[[comment]]\nopen = \"#\"\nnestable = true\n";
        let error = parse("t.toml", source).expect_err("must fail");
        assert!(error.problems()[0].message().contains("nestable"));
    }

    #[test]
    fn an_unknown_role_names_the_roles_it_could_have_been() {
        let source = "[meta]\nname = \"x\"\ndisplay = \"X\"\n[words]\nnoun = [\"if\"]\n";
        let error = parse("t.toml", source).expect_err("must fail");
        let message = error.problems()[0].message();
        assert!(message.contains("noun"), "{message}");
        assert!(message.contains("keyword"), "{message}");
    }

    #[test]
    fn a_case_entry_splits_at_the_first_space_only() {
        let pairs = Case::pairs(&[
            "keyword fn".to_owned(),
            "plain  ".to_owned(),
            "string \"a b\"".to_owned(),
        ]);
        assert_eq!(pairs[0], (Role::Keyword, "fn".to_owned()));
        assert_eq!(pairs[1], (Role::Plain, " ".to_owned()));
        assert_eq!(pairs[2], (Role::String, "\"a b\"".to_owned()));
    }

    #[test]
    fn every_role_round_trips_through_its_name() {
        for role in Role::ALL {
            assert_eq!(Role::parse(role.as_str()), Some(role));
        }
    }
}
