//! `Link`, as trees.
//!
//! One entry per example in `examples::LINK`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Every `Link` example. 5 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"Link { href: "…", "…" }"#,
        &[Node::Component {
            name: "Paragraph",
            props: &[],
            children: &[Node::Component {
                name: "Link",
                props: &[("href", Value::Text("#"))],
                children: &[Node::Text(Value::Message(Key::ExampleLinkText))],
            }],
        }],
    ),
    (
        r#"Link { external: true, rel: "noreferrer", … }"#,
        &[Node::Component {
            name: "Paragraph",
            props: &[],
            children: &[Node::Component {
                name: "Link",
                props: &[
                    (
                        "href",
                        Value::Text("https://noctua-world.github.io/noctua-colors/"),
                    ),
                    ("external", Value::Bool(true)),
                    ("rel", Value::Text("noreferrer")),
                ],
                children: &[Node::Text(Value::Message(Key::ColorsSystemLink))],
            }],
        }],
    ),
    (
        r"Link { tone: LinkTone::Inherit, … }",
        &[Node::Component {
            name: "Paragraph",
            props: &[("tone", Value::Text("muted"))],
            children: &[
                Node::Text(Value::Join(&[
                    Value::Message(Key::ExampleLinkLead),
                    Value::Text(" "),
                ])),
                Node::Component {
                    name: "Link",
                    props: &[("href", Value::Text("#")), ("tone", Value::Text("inherit"))],
                    children: &[Node::Text(Value::Message(Key::ExampleLinkText))],
                },
                Node::Text(Value::Text(".")),
            ],
        }],
    ),
    (
        r"In a sentence",
        &[Node::Component {
            name: "Paragraph",
            props: &[],
            children: &[
                Node::Text(Value::Join(&[
                    Value::Message(Key::ExampleLinkLead),
                    Value::Text(" "),
                ])),
                Node::Component {
                    name: "Link",
                    props: &[("href", Value::Text("#"))],
                    children: &[Node::Text(Value::Message(Key::ExampleLinkText))],
                },
                Node::Text(Value::Text(".")),
            ],
        }],
    ),
    (
        r"Link { ai_mark, ai_label }",
        &[Node::Component {
            name: "Paragraph",
            props: &[],
            children: &[Node::Component {
                name: "Link",
                props: &[
                    ("href", Value::Text("#")),
                    ("ai_mark", Value::Message(Key::ExampleAiMark)),
                    ("ai_label", Value::Message(Key::ExampleAiName)),
                ],
                children: &[Node::Text(Value::Message(Key::ExampleLinkText))],
            }],
        }],
    ),
];
