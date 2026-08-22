//! `Paragraph`, as trees.
//!
//! One entry per example in `examples::PARAGRAPH`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Every `Paragraph` example. 5 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r"Paragraph { … }",
        &[Node::Component {
            name: "Paragraph",
            props: &[],
            children: &[Node::Text(Value::Message(Key::ExampleListLong))],
        }],
    ),
    (
        r"Paragraph { size: ParagraphSize::Lead, … }",
        &[Node::Component {
            name: "Paragraph",
            props: &[("size", Value::Text("lead"))],
            children: &[Node::Text(Value::Message(Key::ExampleListLong))],
        }],
    ),
    (
        r"Paragraph { size: ParagraphSize::Small, … }",
        &[Node::Component {
            name: "Paragraph",
            props: &[("size", Value::Text("small"))],
            children: &[Node::Text(Value::Message(Key::ExampleListLong))],
        }],
    ),
    (
        r"Paragraph { tone: ParagraphTone::Muted, … }",
        &[Node::Component {
            name: "Paragraph",
            props: &[("tone", Value::Text("muted"))],
            children: &[Node::Text(Value::Message(Key::ExampleListLong))],
        }],
    ),
    (
        r"Two, one after the other",
        &[Node::Element {
            tag: "div",
            attributes: &[],
            children: &[
                Node::Component {
                    name: "Paragraph",
                    props: &[],
                    children: &[Node::Text(Value::Message(Key::ExampleListLong))],
                },
                Node::Component {
                    name: "Paragraph",
                    props: &[("tone", Value::Text("muted"))],
                    children: &[Node::Text(Value::Message(Key::ExampleListLong))],
                },
            ],
        }],
    ),
];
