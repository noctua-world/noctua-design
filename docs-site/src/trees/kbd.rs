//! `Kbd`, as trees.
//!
//! One entry per example in `examples::KBD`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_tree_belongs_to_its_example` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Every `Kbd` example. 4 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"Kbd { "Esc" }"#,
        &[Node::Component {
            name: "Kbd",
            props: &[("class", Value::Text("nd-identifier"))],
            children: &[Node::Text(Value::Data("Esc"))],
        }],
    ),
    (
        r"A chord",
        &[Node::Element {
            tag: "span",
            attributes: &[],
            children: &[
                Node::Component {
                    name: "Kbd",
                    props: &[("class", Value::Text("nd-identifier"))],
                    children: &[Node::Text(Value::Data("Ctrl"))],
                },
                Node::Text(Value::Data(" + ")),
                Node::Component {
                    name: "Kbd",
                    props: &[("class", Value::Text("nd-identifier"))],
                    children: &[Node::Text(Value::Data("K"))],
                },
            ],
        }],
    ),
    (
        r"Kbd { size: KbdSize::Body, … }",
        &[Node::Component {
            name: "Kbd",
            props: &[
                ("class", Value::Text("nd-identifier")),
                ("size", Value::Text("body")),
            ],
            children: &[Node::Text(Value::Data("Enter"))],
        }],
    ),
    (
        r"In a sentence",
        &[Node::Component {
            name: "Paragraph",
            props: &[],
            children: &[
                Node::Text(Value::Join(&[
                    Value::Message(Key::ExampleKbdLead),
                    Value::Text(" "),
                ])),
                Node::Component {
                    name: "Kbd",
                    props: &[("class", Value::Text("nd-identifier"))],
                    children: &[Node::Text(Value::Data("Esc"))],
                },
                Node::Text(Value::Data(".")),
            ],
        }],
    ),
];
