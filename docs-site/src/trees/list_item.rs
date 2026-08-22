//! `ListItem`, as trees.
//!
//! One entry per example in `examples::LIST_ITEM`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Every `ListItem` example. 5 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"ListItem { "…" }"#,
        &[Node::Component {
            name: "List",
            props: &[("spacing", Value::Text("tight"))],
            children: &[Node::Component {
                name: "ListItem",
                props: &[],
                children: &[Node::Text(Value::Message(Key::ExampleListOne))],
            }],
        }],
    ),
    (
        r#"ListItem { term: "…", "…" }"#,
        &[Node::Component {
            name: "List",
            props: &[("kind", Value::Text("description"))],
            children: &[Node::Component {
                name: "ListItem",
                props: &[("term", Value::Message(Key::ThemeGlassLabel))],
                children: &[Node::Text(Value::Message(Key::ExampleTermGlass))],
            }],
        }],
    ),
    (
        r"An item holding a whole paragraph",
        &[Node::Component {
            name: "List",
            props: &[("spacing", Value::Text("loose"))],
            children: &[Node::Component {
                name: "ListItem",
                props: &[],
                children: &[Node::Component {
                    name: "Paragraph",
                    props: &[],
                    children: &[Node::Text(Value::Message(Key::ExampleListLong))],
                }],
            }],
        }],
    ),
    (
        r"An item holding a link",
        &[Node::Component {
            name: "List",
            props: &[("spacing", Value::Text("tight"))],
            children: &[Node::Component {
                name: "ListItem",
                props: &[],
                children: &[Node::Component {
                    name: "Link",
                    props: &[("href", Value::Text("#"))],
                    children: &[Node::Text(Value::Message(Key::ExampleLinkText))],
                }],
            }],
        }],
    ),
    (
        r"ListItem { ai_mark, ai_label, ai_note }",
        &[Node::Component {
            name: "List",
            props: &[("spacing", Value::Text("tight"))],
            children: &[Node::Component {
                name: "ListItem",
                props: &[
                    ("ai_mark", Value::Message(Key::ExampleAiMark)),
                    ("ai_label", Value::Message(Key::ExampleAiName)),
                    ("ai_note", Value::Message(Key::ExampleAiNote)),
                ],
                children: &[Node::Text(Value::Message(Key::ExampleListOne))],
            }],
        }],
    ),
];
