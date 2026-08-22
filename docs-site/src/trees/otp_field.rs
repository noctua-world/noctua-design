//! `OtpField`, as trees.
//!
//! One entry per example in `examples::OTP_FIELD`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Each, Node, Value};
use crate::trees::Tree;

/// Every `OtpField` example. 4 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    // SIX BOXES, ONE VALUE. Every box posts under the same name, so the code
    // reaches a server whether or not the script arrived.
    (
        r"OtpField { name, legend, box_label, hint, invalid_label, required, required_label }",
        &[Node::Component {
            name: "OtpField",
            props: &[
                ("name", Value::Text("otp-code")),
                ("legend", Value::Message(Key::ExampleOtpFieldLegend)),
                ("box_label", Value::Message(Key::ExampleOtpFieldBox)),
                ("hint", Value::Message(Key::ExampleOtpFieldHint)),
                ("invalid_label", Value::Message(Key::ExampleOtpFieldInvalid)),
                ("required", Value::Bool(true)),
                (
                    "required_label",
                    Value::Message(Key::ExampleOtpFieldRequired),
                ),
            ],
            children: &[],
        }],
    ),
    (
        r#"OtpField { kind: Alphanumeric, length: "4", value: "AB12" }"#,
        &[Node::Component {
            name: "OtpField",
            props: &[
                ("name", Value::Text("otp-invite")),
                ("legend", Value::Message(Key::ExampleOtpFieldLetters)),
                ("box_label", Value::Message(Key::ExampleOtpFieldLettersBox)),
                ("kind", Value::Text("alphanumeric")),
                ("length", Value::Text("4")),
                ("value", Value::Data("AB12")),
            ],
            children: &[],
        }],
    ),
    (
        r"OtpField { size: Small | Medium | Large }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[Node::Each {
                over: Each::Rows(&[
                    &[Value::Text("small"), Value::Text("small")],
                    &[Value::Text("medium"), Value::Text("medium")],
                    &[Value::Text("large"), Value::Text("large")],
                ]),
                body: &[Node::Component {
                    name: "OtpField",
                    props: &[
                        (
                            "name",
                            Value::Join(&[Value::Text("otp-size-"), Value::Bound(0)]),
                        ),
                        ("legend", Value::Message(Key::ExampleOtpFieldLegend)),
                        ("box_label", Value::Message(Key::ExampleOtpFieldBox)),
                        ("length", Value::Text("4")),
                        ("size", Value::Bound(1)),
                    ],
                    children: &[],
                }],
            }],
        }],
    ),
    (
        r"OtpField { name, legend, box_label, length, value, error }
OtpField { name, legend, box_label, length, autocomplete: false, disabled: true }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[
                Node::Component {
                    name: "OtpField",
                    props: &[
                        ("name", Value::Text("otp-error")),
                        ("legend", Value::Message(Key::ExampleOtpFieldLegend)),
                        ("box_label", Value::Message(Key::ExampleOtpFieldBox)),
                        ("length", Value::Text("4")),
                        ("value", Value::Data("4213")),
                        ("error", Value::Message(Key::ExampleOtpFieldError)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "OtpField",
                    props: &[
                        ("name", Value::Text("otp-off")),
                        ("legend", Value::Message(Key::ExampleOtpFieldLegend)),
                        ("box_label", Value::Message(Key::ExampleOtpFieldBox)),
                        ("length", Value::Text("4")),
                        ("autocomplete", Value::Bool(false)),
                        ("disabled", Value::Bool(true)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
];
