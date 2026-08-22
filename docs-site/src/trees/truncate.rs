//! `Truncate`, as trees.
//!
//! One entry per example in `examples::TRUNCATE`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Every `Truncate` example. 5 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"Truncate { title: "…", "…" }"#,
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-narrow"))],
            children: &[Node::Component {
                name: "Truncate",
                props: &[("title", Value::Message(Key::ExampleListLong))],
                children: &[Node::Text(Value::Message(Key::ExampleListLong))],
            }],
        }],
    ),
    (
        r"Truncate { lines: TruncateLines::_2, … }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-narrow"))],
            children: &[Node::Component {
                name: "Truncate",
                props: &[
                    ("lines", Value::Text("2")),
                    ("title", Value::Message(Key::ExampleListLong)),
                ],
                children: &[Node::Text(Value::Message(Key::ExampleListLong))],
            }],
        }],
    ),
    (
        r"Truncate { lines: TruncateLines::_4, … }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-narrow"))],
            children: &[Node::Component {
                name: "Truncate",
                props: &[
                    ("lines", Value::Text("4")),
                    ("title", Value::Message(Key::ExampleListLong)),
                ],
                children: &[Node::Text(Value::Message(Key::ExampleListLong))],
            }],
        }],
    ),
    (
        r"Short enough not to clip",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-narrow"))],
            children: &[Node::Component {
                name: "Truncate",
                props: &[("title", Value::Message(Key::ExampleListOne))],
                children: &[Node::Text(Value::Message(Key::ExampleListOne))],
            }],
        }],
    ),
    (
        r"Truncate { ai_mark, ai_label, ai_note }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-narrow"))],
            children: &[Node::Component {
                name: "Truncate",
                props: &[
                    ("title", Value::Message(Key::ExampleListLong)),
                    ("ai_mark", Value::Message(Key::ExampleAiMark)),
                    ("ai_label", Value::Message(Key::ExampleAiName)),
                    ("ai_note", Value::Message(Key::ExampleAiNote)),
                ],
                children: &[Node::Text(Value::Message(Key::ExampleListLong))],
            }],
        }],
    ),
];
