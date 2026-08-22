//! `DateInput`, as trees.
//!
//! One entry per example in `examples::DATE_INPUT`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Every `DateInput` example. 4 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r"DateInput { name, legend, hint, day_label, month_label, year_label, bday: true, required: true, required_label, missing_label, number_label, invalid_label, error_label }",
        // `novalidate`, so the sentence a reader hears is the component's own
        // rather than the browser bubble the no-script path already has.
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
                    name: "DateInput",
                    props: &[
                        ("name", Value::Text("birth")),
                        ("legend", Value::Message(Key::ExampleDateInputBirthLegend)),
                        ("hint", Value::Message(Key::ExampleDateInputHint)),
                        ("day_label", Value::Message(Key::ExampleDateInputDay)),
                        ("month_label", Value::Message(Key::ExampleDateInputMonth)),
                        ("year_label", Value::Message(Key::ExampleDateInputYear)),
                        ("bday", Value::Bool(true)),
                        ("required", Value::Bool(true)),
                        (
                            "error_label",
                            Value::Message(Key::ExampleDateInputErrorWord),
                        ),
                        (
                            "required_label",
                            Value::Message(Key::ExampleDateInputRequired),
                        ),
                        (
                            "missing_label",
                            Value::Message(Key::ExampleDateInputMissing),
                        ),
                        ("number_label", Value::Message(Key::ExampleDateInputNumber)),
                        ("invalid_label", Value::Message(Key::ExampleDateInputReal)),
                        (
                            "year_digits_label",
                            Value::Message(Key::ExampleDateInputYearDigits),
                        ),
                    ],
                    children: &[],
                },
                // A form of three boxes and no submit control never submits
                // implicitly — the button is what makes Enter mean send.
                Node::Component {
                    name: "Button",
                    props: &[
                        ("submit", Value::Text("submit")),
                        ("label", Value::Message(Key::ExampleDateInputSend)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r"DateInput { min, max, min_label, max_label, year_digits_label, calendar: true, suggest_label, previous_label, next_label, Time { … } }",
        &[Node::Component {
            name: "DateInput",
            props: &[
                ("name", Value::Text("visit")),
                ("legend", Value::Message(Key::ExampleDateInputVisitLegend)),
                ("day_label", Value::Message(Key::ExampleDateInputDay)),
                ("month_label", Value::Message(Key::ExampleDateInputMonth)),
                ("year_label", Value::Message(Key::ExampleDateInputYear)),
                ("min", Value::Text("2026-01-01")),
                ("max", Value::Text("2027-12-31")),
                (
                    "error_label",
                    Value::Message(Key::ExampleDateInputErrorWord),
                ),
                (
                    "missing_label",
                    Value::Message(Key::ExampleDateInputMissing),
                ),
                ("number_label", Value::Message(Key::ExampleDateInputNumber)),
                ("invalid_label", Value::Message(Key::ExampleDateInputReal)),
                (
                    "year_digits_label",
                    Value::Message(Key::ExampleDateInputYearDigits),
                ),
                ("min_label", Value::Message(Key::ExampleDateInputMin)),
                ("max_label", Value::Message(Key::ExampleDateInputMax)),
                ("calendar", Value::Bool(true)),
                (
                    "suggest_label",
                    Value::Message(Key::ExampleDateInputSuggest),
                ),
                (
                    "previous_label",
                    Value::Message(Key::ExampleDateInputPrevious),
                ),
                ("next_label", Value::Message(Key::ExampleDateInputNext)),
            ],
            children: &[
                Node::Component {
                    name: "Time",
                    props: &[("datetime", Value::Text("2026-12-25"))],
                    children: &[Node::Text(Value::Message(Key::ExampleDateInputChristmas))],
                },
                Node::Component {
                    name: "Time",
                    props: &[("datetime", Value::Text("2027-01-01"))],
                    children: &[Node::Text(Value::Message(Key::ExampleDateInputNewYear))],
                },
            ],
        }],
    ),
    (
        r#"DateInput { value: "2026-08-07", order: Ymd, pad: true, expand: true }"#,
        &[Node::Component {
            name: "DateInput",
            props: &[
                ("name", Value::Text("issued")),
                ("legend", Value::Message(Key::ExampleDateInputIssuedLegend)),
                ("day_label", Value::Message(Key::ExampleDateInputDay)),
                ("month_label", Value::Message(Key::ExampleDateInputMonth)),
                ("year_label", Value::Message(Key::ExampleDateInputYear)),
                ("value", Value::Text("2026-08-07")),
                ("order", Value::Text("ymd")),
                ("pad", Value::Bool(true)),
                ("expand", Value::Bool(true)),
            ],
            children: &[],
        }],
    ),
    (
        r"DateInput { size: Small }  ·  { size: Large, glass: true }  ·  { disabled: true }  ·  { error }",
        &[
            Node::Component {
                name: "DateInput",
                props: &[
                    ("name", Value::Text("compact")),
                    ("legend", Value::Message(Key::ExampleDateInputSmall)),
                    ("day_label", Value::Message(Key::ExampleDateInputDay)),
                    ("month_label", Value::Message(Key::ExampleDateInputMonth)),
                    ("year_label", Value::Message(Key::ExampleDateInputYear)),
                    ("size", Value::Text("small")),
                ],
                children: &[],
            },
            Node::Component {
                name: "DateInput",
                props: &[
                    ("name", Value::Text("roomy")),
                    ("legend", Value::Message(Key::ExampleDateInputLarge)),
                    ("day_label", Value::Message(Key::ExampleDateInputDay)),
                    ("month_label", Value::Message(Key::ExampleDateInputMonth)),
                    ("year_label", Value::Message(Key::ExampleDateInputYear)),
                    ("size", Value::Text("large")),
                    ("glass", Value::Bool(true)),
                ],
                children: &[],
            },
            Node::Component {
                name: "DateInput",
                props: &[
                    ("name", Value::Text("closed")),
                    ("legend", Value::Message(Key::ExampleDateInputClosedLegend)),
                    ("day_label", Value::Message(Key::ExampleDateInputDay)),
                    ("month_label", Value::Message(Key::ExampleDateInputMonth)),
                    ("year_label", Value::Message(Key::ExampleDateInputYear)),
                    ("disabled", Value::Bool(true)),
                ],
                children: &[],
            },
            Node::Component {
                name: "DateInput",
                props: &[
                    ("name", Value::Text("booked")),
                    ("legend", Value::Message(Key::ExampleDateInputVisitLegend)),
                    ("day_label", Value::Message(Key::ExampleDateInputDay)),
                    ("month_label", Value::Message(Key::ExampleDateInputMonth)),
                    ("year_label", Value::Message(Key::ExampleDateInputYear)),
                    ("value", Value::Text("2025-02-14")),
                    ("error", Value::Message(Key::ExampleDateInputErrorMessage)),
                    (
                        "error_label",
                        Value::Message(Key::ExampleDateInputErrorWord),
                    ),
                ],
                children: &[],
            },
        ],
    ),
];
