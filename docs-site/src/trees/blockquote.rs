//! `Blockquote`, as trees.
//!
//! One entry per example in `examples::BLOCKQUOTE`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Every `Blockquote` example. 7 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r"Blockquote { … }",
        &[Node::Component {
            name: "Blockquote",
            props: &[],
            children: &[Node::Component {
                name: "Paragraph",
                props: &[],
                children: &[Node::Text(Value::Message(Key::ExampleQuote))],
            }],
        }],
    ),
    (
        r"Blockquote { tone: BlockquoteTone::Accent, … }",
        &[Node::Component {
            name: "Blockquote",
            props: &[("tone", Value::Text("accent"))],
            children: &[Node::Component {
                name: "Paragraph",
                props: &[],
                children: &[Node::Text(Value::Message(Key::ExampleQuote))],
            }],
        }],
    ),
    (
        r#"Blockquote { attribution: "…", … }"#,
        &[Node::Component {
            name: "Blockquote",
            props: &[("attribution", Value::Data("Edsger W. Dijkstra"))],
            children: &[Node::Component {
                name: "Paragraph",
                props: &[],
                children: &[Node::Text(Value::Message(Key::ExampleQuote))],
            }],
        }],
    ),
    (
        r#"Blockquote { cite: "…", attribution: "…", … }"#,
        &[Node::Component {
            name: "Blockquote",
            props: &[
                ("tone", Value::Text("accent")),
                ("cite", Value::Text("https://example.invalid/on-simplicity")),
                ("attribution", Value::Data("Edsger W. Dijkstra")),
            ],
            children: &[Node::Component {
                name: "Paragraph",
                props: &[],
                children: &[Node::Text(Value::Message(Key::ExampleQuote))],
            }],
        }],
    ),
    (
        r"Blockquote { … Link { … } }",
        &[Node::Component {
            name: "Blockquote",
            props: &[],
            children: &[Node::Component {
                name: "Paragraph",
                props: &[],
                children: &[
                    Node::Text(Value::Join(&[
                        Value::Message(Key::ExampleQuoteShort),
                        Value::Text(" "),
                    ])),
                    Node::Component {
                        name: "Link",
                        props: &[("href", Value::Text("#"))],
                        children: &[Node::Text(Value::Message(Key::ExampleLinkText))],
                    },
                ],
            }],
        }],
    ),
    (
        r"Blockquote { glass: true, … }",
        &[Node::Component {
            name: "Blockquote",
            props: &[("glass", Value::Bool(true))],
            children: &[Node::Component {
                name: "Paragraph",
                props: &[],
                children: &[Node::Text(Value::Message(Key::ExampleQuote))],
            }],
        }],
    ),
    (
        r"Blockquote { ai_mark, ai_label, ai_note }",
        &[Node::Component {
            name: "Blockquote",
            props: &[
                ("ai_mark", Value::Message(Key::ExampleAiMark)),
                ("ai_label", Value::Message(Key::ExampleAiName)),
                ("ai_note", Value::Message(Key::ExampleAiNote)),
            ],
            children: &[Node::Component {
                name: "Paragraph",
                props: &[],
                children: &[Node::Text(Value::Message(Key::ExampleQuote))],
            }],
        }],
    ),
];
