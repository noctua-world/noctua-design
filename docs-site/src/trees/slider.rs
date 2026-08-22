//! `Slider`, as trees.
//!
//! One entry per example in `examples::SLIDER`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Every `Slider` example. 5 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r"Slider { name, label, value, min, max, step, output_label, hint, required, required_label }",
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
                    name: "Slider",
                    props: &[
                        ("name", Value::Text("volume")),
                        ("label", Value::Message(Key::ExampleInputVolumeLabel)),
                        ("value", Value::Text("35")),
                        ("min", Value::Text("0")),
                        ("max", Value::Text("100")),
                        ("step", Value::Text("5")),
                        (
                            "output_label",
                            Value::Message(Key::ExampleInputPercentValue),
                        ),
                        ("hint", Value::Message(Key::ExampleInputHint)),
                        ("required", Value::Bool(true)),
                        ("required_label", Value::Message(Key::ExampleInputRequired)),
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
        r"Slider { marks, mark_labels }",
        &[Node::Component {
            name: "Slider",
            props: &[
                ("name", Value::Text("marked")),
                ("label", Value::Message(Key::ExampleInputVolumeLabel)),
                ("value", Value::Text("50")),
                ("marks", Value::Text("0 50 100")),
                ("mark_labels", Value::Message(Key::ExampleInputVolumeMarks)),
                (
                    "output_label",
                    Value::Message(Key::ExampleInputPercentValue),
                ),
            ],
            children: &[],
        }],
    ),
    (
        r"Slider { name, label, range, value, end, min, max, step, output_label, tone, size }",
        &[Node::Component {
            name: "Slider",
            props: &[
                ("name", Value::Text("budget")),
                ("label", Value::Message(Key::ExampleInputBudgetLabel)),
                ("range", Value::Bool(true)),
                ("value", Value::Text("200")),
                ("end", Value::Text("800")),
                ("min", Value::Text("0")),
                ("max", Value::Text("1000")),
                ("step", Value::Text("50")),
                (
                    "output_label",
                    Value::Message(Key::ExampleInputCurrencyValue),
                ),
                ("tone", Value::Text("success")),
                ("size", Value::Text("large")),
            ],
            children: &[],
        }],
    ),
    (
        r"Slider { name, label, value, output: false, tone, size, glass, error }
Slider { name, label, value, tone, disabled }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[
                Node::Component {
                    name: "Slider",
                    props: &[
                        ("name", Value::Text("quiet")),
                        ("label", Value::Message(Key::ExampleInputVolumeLabel)),
                        ("value", Value::Text("60")),
                        ("output", Value::Bool(false)),
                        ("tone", Value::Text("neutral")),
                        ("size", Value::Text("small")),
                        ("glass", Value::Bool(true)),
                        ("error", Value::Message(Key::ExampleInputError)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Slider",
                    props: &[
                        ("name", Value::Text("refused-slider")),
                        ("label", Value::Message(Key::ExampleInputVolumeLabel)),
                        ("value", Value::Text("20")),
                        ("tone", Value::Text("danger")),
                        ("disabled", Value::Bool(true)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r"Slider { ai_mark, ai_label, ai_note }",
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
                    name: "Slider",
                    props: &[
                        ("ai_mark", Value::Message(Key::ExampleAiMark)),
                        ("ai_label", Value::Message(Key::ExampleAiName)),
                        ("ai_note", Value::Message(Key::ExampleAiNote)),
                        ("name", Value::Text("volume-ai")),
                        ("label", Value::Message(Key::ExampleInputVolumeLabel)),
                        ("value", Value::Text("35")),
                        ("min", Value::Text("0")),
                        ("max", Value::Text("100")),
                        ("step", Value::Text("5")),
                        (
                            "output_label",
                            Value::Message(Key::ExampleInputPercentValue),
                        ),
                        ("hint", Value::Message(Key::ExampleInputHint)),
                        ("required", Value::Bool(true)),
                        ("required_label", Value::Message(Key::ExampleInputRequired)),
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
