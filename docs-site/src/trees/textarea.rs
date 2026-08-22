//! `Textarea`, as trees.
//!
//! One entry per example in `examples::TEXTAREA`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Every `Textarea` example. 5 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r"Textarea { name, label, rows, autosize, max_rows, placeholder, hint, required, required_label }",
        &[Node::Element {
            tag: "form",
            attributes: &[
                ("class", Value::Text("nd-example-form")),
                ("method", Value::Text("get")),
                ("action", Value::Text("#")),
                ("novalidate", Value::Bool(true)),
            ],
            children: &[
                Node::Component {
                    name: "Textarea",
                    props: &[
                        ("name", Value::Text("note")),
                        ("label", Value::Message(Key::ExampleInputNoteLabel)),
                        ("hint", Value::Message(Key::ExampleInputNoteHint)),
                        ("rows", Value::Text("3")),
                        ("autosize", Value::Bool(true)),
                        ("max_rows", Value::Text("8")),
                        ("required", Value::Bool(true)),
                        ("required_label", Value::Message(Key::ExampleInputRequired)),
                        ("placeholder", Value::Data("…")),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Button",
                    props: &[
                        ("submit", Value::Text("submit")),
                        ("label", Value::Message(Key::ExampleInputSend)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r"Textarea { name, label, rows, maxlength, counter, counter_label, spellcheck: false }",
        // A LIST OF ADDRESSES, NOT PROSE, which is the one case where
        // turning the underlining off is right. `spellcheck` is **on** by
        // default on a `Textarea`, so `true` here showed nothing.
        &[Node::Component {
            name: "Textarea",
            props: &[
                ("name", Value::Text("recipients")),
                ("label", Value::Message(Key::ExampleTextareaRecipients)),
                ("rows", Value::Text("3")),
                ("maxlength", Value::Text("400")),
                ("counter", Value::Bool(true)),
                ("counter_label", Value::Message(Key::ExampleInputCounter)),
                ("spellcheck", Value::Bool(false)),
            ],
            children: &[],
        }],
    ),
    (
        r"Textarea { name, label, value, rows, resize: None, autosize, size, glass, error }",
        // `resize: None` WANTS `autosize` ON, and the contract says so in
        // writing: a box a reader can neither drag nor have grown for them
        // is a box whose text runs out of sight.
        &[Node::Component {
            name: "Textarea",
            props: &[
                ("name", Value::Text("fixed")),
                ("label", Value::Message(Key::ExampleInputNoteLabel)),
                ("value", Value::Message(Key::ExampleInputFixedNote)),
                ("rows", Value::Text("2")),
                ("resize", Value::Text("none")),
                ("autosize", Value::Bool(true)),
                ("size", Value::Text("small")),
                ("glass", Value::Bool(true)),
                ("error", Value::Message(Key::ExampleInputError)),
            ],
            children: &[],
        }],
    ),
    (
        r"Textarea { readonly } / Textarea { disabled }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[
                Node::Component {
                    name: "Textarea",
                    props: &[
                        ("name", Value::Text("locked")),
                        ("label", Value::Message(Key::ExampleInputNoteLabel)),
                        ("value", Value::Message(Key::ExampleInputLockedNote)),
                        ("readonly", Value::Bool(true)),
                        ("rows", Value::Text("2")),
                        ("size", Value::Text("large")),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Textarea",
                    props: &[
                        ("name", Value::Text("note-off")),
                        ("label", Value::Message(Key::ExampleInputNoteLabel)),
                        ("rows", Value::Text("2")),
                        ("disabled", Value::Bool(true)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r"Textarea { ai_mark, ai_label, ai_note }",
        &[Node::Element {
            tag: "form",
            attributes: &[
                ("class", Value::Text("nd-example-form")),
                ("method", Value::Text("get")),
                ("action", Value::Text("#")),
                ("novalidate", Value::Bool(true)),
            ],
            children: &[
                Node::Component {
                    name: "Textarea",
                    props: &[
                        ("ai_mark", Value::Message(Key::ExampleAiMark)),
                        ("ai_label", Value::Message(Key::ExampleAiName)),
                        ("ai_note", Value::Message(Key::ExampleAiNote)),
                        ("name", Value::Text("note-ai")),
                        ("label", Value::Message(Key::ExampleInputNoteLabel)),
                        ("hint", Value::Message(Key::ExampleInputNoteHint)),
                        ("rows", Value::Text("3")),
                        ("autosize", Value::Bool(true)),
                        ("max_rows", Value::Text("8")),
                        ("required", Value::Bool(true)),
                        ("required_label", Value::Message(Key::ExampleInputRequired)),
                        ("placeholder", Value::Data("…")),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Button",
                    props: &[
                        ("submit", Value::Text("submit")),
                        ("label", Value::Message(Key::ExampleInputSend)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
];
