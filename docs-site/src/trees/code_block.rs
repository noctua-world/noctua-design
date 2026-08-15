//! `CodeBlock`, as trees.
//!
//! One entry per example in `examples::CODE_BLOCK`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_tree_belongs_to_its_example` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Each, Node, Value};
use crate::trees::Tree;

/// Real code in five languages, because a sample nobody would write teaches
/// nothing — and because a grammar's traps only show on code that has them.
pub(crate) const SAMPLE_RUST: &str = r"/// Every attribute a scope stamps, or none.
pub fn scope_attributes(request: &ScopeRequest) -> Option<Attributes> {
    let resolved = resolve(request)?;
    if resolved == inherited {
        return None; // a no-op scope is 1,767 declarations for nothing
    }
    Some(Attributes { palette: resolved.palette, mode: resolved.mode })
}";

pub(crate) const SAMPLE_TS: &str = r"export interface Token {
  readonly role: Role;
  readonly text: string;
}

export function total(tokens: Token[]): number {
  return tokens.reduce((sum, token) => sum + token.text.length, 0);
}";

pub(crate) const SAMPLE_PYTHON: &str = r#"class Palette:
    """A palette, and the mode it was solved in."""

    MAX_STEPS = 12

    def __init__(self, name: str, mode: str = "light") -> None:
        self.name = name
        self.mode = mode

    def step(self, n):
        if n < 1 or n > self.MAX_STEPS:
            raise ValueError(f"{n} is outside 1..{self.MAX_STEPS}")
        return self._steps[n - 1]"#;

pub(crate) const SAMPLE_SHELL: &str = r#"#!/usr/bin/env bash
set -euo pipefail

for palette in "$PALETTES"/*.css; do
  name="$(basename "$palette" .css)"
  echo "checking ${name}" >&2
  grep -c -- '--nc-' "$palette" || exit 1
done"#;

pub(crate) const SAMPLE_DIFF: &str = r#"--- a/spec/tokens.toml
+++ b/spec/tokens.toml
@@ -12,7 +12,7 @@
 [space]
 base = "0.25rem"
-steps = [0, 1, 2, 3, 4, 6, 8]
+steps = [0, 1, 2, 3, 4, 6, 8, 12, 16]
 
 [space.named]"#;

/// One line, longer than any column this site has.
pub(crate) const SAMPLE_LONG: &str =
    "let attributes = scope_attributes(request, inherited).expect(\"a change worth stamping\");";

/// Long enough that the collapse has something to collapse.
pub(crate) const SAMPLE_TALL: &str = r"pub enum Role {
    Plain,
    Operator,
    Keyword,
    Punctuation,
    Comment,
    String,
    Escape,
    Number,
    Constant,
    Type,
    Tag,
    Function,
    Property,
    Attribute,
    Inserted,
    Deleted,
    Invalid,
}";

/// Every `CodeBlock` example. 11 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"CodeBlock { code: "…", language: "rust" }"#,
        &[Node::Component {
            name: "CodeBlock",
            props: &[
                ("code", Value::Data(SAMPLE_RUST)),
                ("language", Value::Text("rust")),
            ],
            children: &[],
        }],
    ),
    (
        r#"CodeBlock { language: "…" }, thirty-four of them"#,
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[
                Node::Component {
                    name: "CodeBlock",
                    props: &[
                        ("code", Value::Data(SAMPLE_TS)),
                        ("language", Value::Text("typescript")),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "CodeBlock",
                    props: &[
                        ("code", Value::Data(SAMPLE_PYTHON)),
                        ("language", Value::Text("python")),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "CodeBlock",
                    props: &[
                        ("code", Value::Data(SAMPLE_SHELL)),
                        ("language", Value::Text("sh")),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r#"CodeBlock { code: "…", language: "rust", theme: CodeBlockTheme::… }"#,
        // NO `filename` HERE, AND THAT IS THE REPAIR. These four carried
        // `spectrum`, `duotone`, `muted` and `mono`, which are theme names
        // and not files. `filename` is also the accessible name of the
        // scrollable region, so a wrong one is a wrong name announced.
        // With it absent the name falls back on the language, which is
        // what the contract promises.
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            // The four that override, and not `auto`, which is the
            // default and would show the prop doing nothing.
            children: &[Node::Each {
                over: Each::Rows(&[
                    &[Value::Text("spectrum")],
                    &[Value::Text("duotone")],
                    &[Value::Text("muted")],
                    &[Value::Text("mono")],
                ]),
                body: &[Node::Component {
                    name: "CodeBlock",
                    props: &[
                        ("code", Value::Data(SAMPLE_RUST)),
                        ("language", Value::Text("rust")),
                        ("theme", Value::Bound(0)),
                    ],
                    children: &[],
                }],
            }],
        }],
    ),
    (
        r#"CodeBlock { numbered: true, start_line: "…", mark_lines: "…", focus: true }"#,
        &[Node::Component {
            name: "CodeBlock",
            props: &[
                ("code", Value::Data(SAMPLE_RUST)),
                ("language", Value::Text("rust")),
                ("numbered", Value::Bool(true)),
                ("start_line", Value::Text("48")),
                ("mark_lines", Value::Text("50-51")),
                ("focus", Value::Bool(true)),
            ],
            children: &[],
        }],
    ),
    (
        r#"CodeBlock { language: "diff", … }"#,
        &[Node::Component {
            name: "CodeBlock",
            props: &[
                ("code", Value::Data(SAMPLE_DIFF)),
                ("language", Value::Text("diff")),
                ("filename", Value::Text("spec/tokens.toml")),
            ],
            children: &[],
        }],
    ),
    (
        r#"CodeBlock { filename: "…", copy: "…", copied: "…", wrap_label: "…", caption: "…" }"#,
        &[Node::Component {
            name: "CodeBlock",
            props: &[
                ("code", Value::Data(SAMPLE_LONG)),
                ("language", Value::Text("rust")),
                ("filename", Value::Text("scope.rs")),
                ("copy", Value::Message(Key::ExampleCopy)),
                ("copied", Value::Message(Key::ExampleCopied)),
                ("wrap_label", Value::Message(Key::ExampleCodeBlockWrapLabel)),
                ("caption", Value::Message(Key::ExampleCodeBlockCaption)),
            ],
            children: &[],
        }],
    ),
    (
        r"CodeBlock { wrap: true, … }",
        &[Node::Component {
            name: "CodeBlock",
            props: &[
                ("code", Value::Data(SAMPLE_LONG)),
                ("language", Value::Text("rust")),
                ("wrap", Value::Bool(true)),
            ],
            children: &[],
        }],
    ),
    (
        r#"CodeBlock { max_lines: "…", expand_label: "…", collapse_label: "…" }"#,
        &[Node::Component {
            name: "CodeBlock",
            props: &[
                ("code", Value::Data(SAMPLE_TALL)),
                ("language", Value::Text("rust")),
                ("numbered", Value::Bool(true)),
                ("max_lines", Value::Text("6")),
                ("expand_label", Value::Message(Key::ExampleCodeBlockExpand)),
                (
                    "collapse_label",
                    Value::Message(Key::ExampleCodeBlockCollapseLabel),
                ),
            ],
            children: &[],
        }],
    ),
    (
        r"A language nothing recognises",
        &[Node::Component {
            name: "CodeBlock",
            props: &[
                ("code", Value::Data(SAMPLE_RUST)),
                ("language", Value::Text("brainfuck")),
            ],
            children: &[],
        }],
    ),
    (
        r"CodeBlock { glass: true, … }",
        &[Node::Component {
            name: "CodeBlock",
            props: &[
                ("code", Value::Data(SAMPLE_RUST)),
                ("language", Value::Text("rust")),
                ("glass", Value::Bool(true)),
            ],
            children: &[],
        }],
    ),
    (
        r"Language::ALL",
        // READ FROM THE GENERATED SET, never listed here. A second copy of
        // `spec/grammars/` is a second copy that stops agreeing, and this
        // is the page a reader comes to to find out what they may pass.
        &[Node::Element {
            tag: "ul",
            attributes: &[("class", Value::Text("nd-language-index"))],
            children: &[Node::Each {
                over: Each::Languages,
                body: &[Node::Element {
                    tag: "li",
                    attributes: &[],
                    children: &[
                        Node::Component {
                            name: "Code",
                            props: &[("class", Value::Text("nd-identifier"))],
                            children: &[Node::Text(Value::Bound(0))],
                        },
                        Node::Text(Value::Text(" ")),
                        // `nd-identifier`, because a language's own name is a
                        // proper noun rather than a sentence: "Rust" is "Rust" in
                        // every locale, and the translation scan skips a subtree
                        // that says so — the same call the token names on the
                        // spacing page already make.
                        Node::Component {
                            name: "Text",
                            props: &[
                                ("class", Value::Text("nd-identifier")),
                                ("size", Value::Text("caption")),
                                ("tone", Value::Text("muted")),
                            ],
                            children: &[Node::Text(Value::Bound(1))],
                        },
                    ],
                }],
            }],
        }],
    ),
];
