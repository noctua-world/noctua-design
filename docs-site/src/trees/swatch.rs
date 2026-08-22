//! `Swatch`, as trees.
//!
//! One entry per example in `examples::SWATCH`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Each, Node, Value};
use crate::trees::Tree;

/// Every `Swatch` example. 4 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"Swatch { }  ·  Text { "…" }"#,
        &[Node::Element {
            tag: "span",
            attributes: &[("class", Value::Text("nd-example-byline"))],
            children: &[
                Node::Component {
                    name: "Swatch",
                    props: &[],
                    children: &[],
                },
                Node::Component {
                    name: "Text",
                    props: &[],
                    children: &[Node::Text(Value::Message(Key::ExampleSwatchInForce))],
                },
            ],
        }],
    ),
    (
        r#"Tag { swatch: "rose-vivid", label: "…" }"#,
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-row"))],
            // `Tag` draws a real `Swatch` before the text, which is the
            // shortest correct way to use one: the word is the name and
            // the chip is the colour of it.
            children: &[Node::Each {
                over: Each::Rows(&[
                    &[
                        Value::Text("rose-vivid"),
                        Value::Message(Key::ThemeAccentRose),
                    ],
                    &[
                        Value::Text("jade-balanced"),
                        Value::Message(Key::ThemeAccentJade),
                    ],
                    &[
                        Value::Text("indigo-sober"),
                        Value::Message(Key::ThemeAccentIndigo),
                    ],
                ]),
                body: &[Node::Component {
                    name: "Tag",
                    props: &[
                        ("swatch", Value::Bound(0)),
                        ("variant", Value::Text("outline")),
                        ("label", Value::Bound(1)),
                    ],
                    children: &[],
                }],
            }],
        }],
    ),
    (
        r#"Swatch { size: SwatchSize::…, palette: "amber-vivid" }  ·  Code { "…" }"#,
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-row"))],
            children: &[Node::Each {
                over: Each::Values {
                    component: "Swatch",
                    prop: "size",
                },
                body: &[Node::Element {
                    tag: "span",
                    attributes: &[("class", Value::Text("nd-example-byline"))],
                    children: &[
                        Node::Component {
                            name: "Swatch",
                            props: &[
                                ("size", Value::Bound(0)),
                                ("palette", Value::Text("amber-vivid")),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "Code",
                            props: &[("class", Value::Text("nd-identifier"))],
                            children: &[Node::Text(Value::Bound(0))],
                        },
                    ],
                }],
            }],
        }],
    ),
    (
        r#"Swatch { shape: SwatchShape::…, size: SwatchSize::Large, palette: "teal-balanced" }  ·  Code { "…" }"#,
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-row"))],
            children: &[Node::Each {
                over: Each::Values {
                    component: "Swatch",
                    prop: "shape",
                },
                body: &[Node::Element {
                    tag: "span",
                    attributes: &[("class", Value::Text("nd-example-byline"))],
                    children: &[
                        Node::Component {
                            name: "Swatch",
                            props: &[
                                ("shape", Value::Bound(0)),
                                ("size", Value::Text("large")),
                                ("palette", Value::Text("teal-balanced")),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "Code",
                            props: &[("class", Value::Text("nd-identifier"))],
                            children: &[Node::Text(Value::Bound(0))],
                        },
                    ],
                }],
            }],
        }],
    ),
];
