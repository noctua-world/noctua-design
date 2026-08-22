//! `Text`, as trees.
//!
//! One entry per example in `examples::TEXT`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Each, Node, Value};
use crate::trees::Tree;

/// Every `Text` example. 5 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r"Every size",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            // Read from the contract, not listed: a fifth step added to
            // `spec/components/text.toml` draws itself here.
            children: &[Node::Each {
                over: Each::Values {
                    component: "Text",
                    prop: "size",
                },
                body: &[Node::Component {
                    name: "Text",
                    props: &[("size", Value::Bound(0))],
                    children: &[Node::Text(Value::Message(Key::TypeSpecimenLine))],
                }],
            }],
        }],
    ),
    (
        r"Every weight",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[Node::Each {
                over: Each::Values {
                    component: "Text",
                    prop: "weight",
                },
                body: &[Node::Component {
                    name: "Text",
                    props: &[("weight", Value::Bound(0))],
                    children: &[Node::Text(Value::Message(Key::TypeSpecimenLine))],
                }],
            }],
        }],
    ),
    (
        r"Every tone",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[Node::Each {
                over: Each::Values {
                    component: "Text",
                    prop: "tone",
                },
                body: &[Node::Component {
                    name: "Text",
                    props: &[("tone", Value::Bound(0))],
                    children: &[Node::Text(Value::Message(Key::TypeSpecimenLine))],
                }],
            }],
        }],
    ),
    (
        r"Text { numeric: true, … }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[
                Node::Component {
                    name: "Text",
                    props: &[
                        ("class", Value::Text("nd-identifier")),
                        ("numeric", Value::Bool(true)),
                    ],
                    children: &[Node::Text(Value::Data("1 111 111"))],
                },
                Node::Component {
                    name: "Text",
                    props: &[
                        ("class", Value::Text("nd-identifier")),
                        ("numeric", Value::Bool(true)),
                    ],
                    children: &[Node::Text(Value::Data("8 888 888"))],
                },
            ],
        }],
    ),
    (
        r"Text { size: Caption, weight: Semibold, tone: Muted }",
        &[Node::Component {
            name: "Text",
            props: &[
                ("size", Value::Text("caption")),
                ("weight", Value::Text("semibold")),
                ("tone", Value::Text("muted")),
            ],
            children: &[Node::Text(Value::Message(Key::TypeSpecimenLine))],
        }],
    ),
];
