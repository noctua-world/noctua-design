//! `TimeInput`, as trees.
//!
//! One entry per example in `examples::TIME_INPUT`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Every `TimeInput` example. 5 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r"TimeInput { name, legend, hour_label, minute_label, value, hint, size }",
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
                    name: "TimeInput",
                    props: &[
                        ("name", Value::Text("starts")),
                        ("legend", Value::Message(Key::ExampleInputTimeLegend)),
                        ("hour_label", Value::Message(Key::ExampleInputHour)),
                        ("minute_label", Value::Message(Key::ExampleInputMinute)),
                        ("value", Value::Text("09:30")),
                        ("hint", Value::Message(Key::ExampleInputTimeHint)),
                        ("size", Value::Text("medium")),
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
        r"TimeInput { name, legend, hour_label, minute_label, clock: _12, meridiem_label, am_label, pm_label, value }",
        &[Node::Component {
            name: "TimeInput",
            props: &[
                ("name", Value::Text("meeting")),
                ("legend", Value::Message(Key::ExampleInputTimeLegend)),
                ("hour_label", Value::Message(Key::ExampleInputHour)),
                ("minute_label", Value::Message(Key::ExampleInputMinute)),
                // `_12`, AND IT IS NOT THE DEFAULT ANY MORE. Twenty-four is
                // what a field is read on unless it is asked otherwise, so
                // this is the one example that shows the other clock and the
                // meridiem control that goes with it.
                ("clock", Value::Text("12")),
                ("meridiem_label", Value::Message(Key::ExampleInputMeridiem)),
                ("am_label", Value::Message(Key::ExampleInputAm)),
                ("pm_label", Value::Message(Key::ExampleInputPm)),
                ("value", Value::Text("14:05")),
            ],
            children: &[],
        }],
    ),
    (
        r"TimeInput { min, max, step, pad: false, required, required_label, missing_label, invalid_label, min_label, max_label }",
        &[Node::Component {
            name: "TimeInput",
            props: &[
                ("name", Value::Text("booking")),
                ("legend", Value::Message(Key::ExampleInputTimeLegend)),
                ("hour_label", Value::Message(Key::ExampleInputHour)),
                ("minute_label", Value::Message(Key::ExampleInputMinute)),
                ("min", Value::Text("08:00")),
                ("max", Value::Text("18:00")),
                ("step", Value::Text("15")),
                // `pad` is ON by default, so `false` is the only value that
                // shows it doing anything: leave a box holding `9` and it
                // stays `9` rather than becoming `09`.
                ("pad", Value::Bool(false)),
                ("required", Value::Bool(true)),
                (
                    "required_label",
                    Value::Message(Key::ExampleInputTimeNeeded),
                ),
                (
                    "missing_label",
                    Value::Message(Key::ExampleInputTimeMissing),
                ),
                (
                    "invalid_label",
                    Value::Message(Key::ExampleInputTimeInvalid),
                ),
                ("min_label", Value::Message(Key::ExampleInputTimeEarly)),
                ("max_label", Value::Message(Key::ExampleInputTimeLate)),
            ],
            children: &[],
        }],
    ),
    (
        r"TimeInput { seconds, second_label, glass, error } / TimeInput { disabled }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[
                Node::Component {
                    name: "TimeInput",
                    props: &[
                        ("name", Value::Text("logged")),
                        ("legend", Value::Message(Key::ExampleInputTimeLegend)),
                        ("hour_label", Value::Message(Key::ExampleInputHour)),
                        ("minute_label", Value::Message(Key::ExampleInputMinute)),
                        ("second_label", Value::Message(Key::ExampleInputSecond)),
                        ("seconds", Value::Bool(true)),
                        ("clock", Value::Text("24")),
                        ("value", Value::Text("23:59:30")),
                        ("size", Value::Text("small")),
                        ("glass", Value::Bool(true)),
                        ("error", Value::Message(Key::ExampleInputError)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "TimeInput",
                    props: &[
                        ("name", Value::Text("refused-time")),
                        ("legend", Value::Message(Key::ExampleInputTimeLegend)),
                        ("hour_label", Value::Message(Key::ExampleInputHour)),
                        ("minute_label", Value::Message(Key::ExampleInputMinute)),
                        ("clock", Value::Text("12")),
                        // A NAME AND BOTH WORDS. The meridiem control is
                        // drawn only when all three are given, because a
                        // radiogroup with no name announces its role and
                        // nothing else.
                        ("meridiem_label", Value::Message(Key::ExampleInputMeridiem)),
                        ("am_label", Value::Message(Key::ExampleInputAm)),
                        ("pm_label", Value::Message(Key::ExampleInputPm)),
                        ("disabled", Value::Bool(true)),
                        ("size", Value::Text("large")),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r"TimeInput { ai_mark, ai_label, ai_note }",
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
                    name: "TimeInput",
                    props: &[
                        ("ai_mark", Value::Message(Key::ExampleAiMark)),
                        ("ai_label", Value::Message(Key::ExampleAiName)),
                        ("ai_note", Value::Message(Key::ExampleAiNote)),
                        ("name", Value::Text("starts-ai")),
                        ("legend", Value::Message(Key::ExampleInputTimeLegend)),
                        ("hour_label", Value::Message(Key::ExampleInputHour)),
                        ("minute_label", Value::Message(Key::ExampleInputMinute)),
                        ("value", Value::Text("09:30")),
                        ("hint", Value::Message(Key::ExampleInputTimeHint)),
                        ("size", Value::Text("medium")),
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
