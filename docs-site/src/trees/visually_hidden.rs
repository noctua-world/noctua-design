//! `VisuallyHidden`, as trees.
//!
//! One entry per example in `examples::VISUALLY_HIDDEN`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Every `VisuallyHidden` example. 4 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"VisuallyHidden { "…" }"#,
        &[Node::Component {
            name: "Paragraph",
            props: &[],
            children: &[
                Node::Component {
                    name: "VisuallyHidden",
                    props: &[],
                    children: &[Node::Text(Value::Message(Key::ExampleHiddenText))],
                },
                Node::Text(Value::Message(Key::ExampleHiddenVisible)),
            ],
        }],
    ),
    // Hand-written markup, deliberately, in the one example that is ABOUT
    // hand-written markup: this is the technique, and `Button { hide_label }`
    // is the technique already done for you. The note says so.
    (
        r"Naming an icon-only control",
        &[Node::Element {
            tag: "button",
            attributes: &[
                ("class", Value::Text("nd-button")),
                ("type", Value::Text("button")),
                ("data-nd-variant", Value::Text("outline")),
                ("data-nd-tone", Value::Text("neutral")),
                ("data-nd-size", Value::Text("medium")),
                ("data-nd-shape", Value::Text("rounded")),
                ("data-nd-hide-label", Value::Text("true")),
            ],
            children: &[
                Node::Component {
                    name: "VisuallyHidden",
                    props: &[],
                    children: &[Node::Text(Value::Message(Key::SearchLabel))],
                },
                Node::Component {
                    name: "Icon",
                    props: &[("icon", Value::Text("search"))],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r"VisuallyHidden { focusable: true, … }",
        &[Node::Component {
            name: "VisuallyHidden",
            props: &[("focusable", Value::Bool(true))],
            children: &[Node::Element {
                tag: "a",
                attributes: &[
                    ("class", Value::Text("nd-a")),
                    ("href", Value::Text("#nd-main")),
                ],
                children: &[Node::Text(Value::Message(Key::AppSkip))],
            }],
        }],
    ),
    (
        r"Extending a link's name",
        &[Node::Component {
            name: "Paragraph",
            props: &[],
            children: &[Node::Component {
                name: "Link",
                props: &[("href", Value::Text("#"))],
                children: &[
                    Node::Text(Value::Message(Key::ExampleLinkText)),
                    Node::Component {
                        name: "VisuallyHidden",
                        props: &[],
                        children: &[Node::Text(Value::Join(&[
                            Value::Text(" — "),
                            Value::Message(Key::ExampleHiddenContextWord),
                        ]))],
                    },
                ],
            }],
        }],
    ),
];
