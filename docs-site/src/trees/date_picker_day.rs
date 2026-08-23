//! `DatePickerDay`, as trees.
//!
//! One entry per example in `examples::DATE_PICKER_DAY`, in the same order.
//! Each draws the day inside the picker that reads it, because the element
//! renders hidden and a picture of it on its own is a picture of nothing.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Every `DatePickerDay` example. 4 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"DatePickerDay { datetime: "2026-08-24/2026-08-28", kind: Preset }"#,
        &[Node::Component {
            name: "DatePicker",
            props: &[
                ("name", Value::Text("picks")),
                ("legend", Value::Message(Key::ExampleDatePickerNamedLegend)),
                ("month", Value::Text("2026-08")),
                ("month_label", Value::Message(Key::ExampleDatePickerAugust)),
                ("monday_label", Value::Message(Key::ExampleDatePickerMonday)),
                (
                    "tuesday_label",
                    Value::Message(Key::ExampleDatePickerTuesday),
                ),
                (
                    "wednesday_label",
                    Value::Message(Key::ExampleDatePickerWednesday),
                ),
                (
                    "thursday_label",
                    Value::Message(Key::ExampleDatePickerThursday),
                ),
                ("friday_label", Value::Message(Key::ExampleDatePickerFriday)),
                (
                    "saturday_label",
                    Value::Message(Key::ExampleDatePickerSaturday),
                ),
                ("sunday_label", Value::Message(Key::ExampleDatePickerSunday)),
            ],
            children: &[Node::Component {
                name: "DatePickerDay",
                props: &[
                    ("datetime", Value::Text("2026-08-24/2026-08-28")),
                    ("kind", Value::Text("preset")),
                ],
                children: &[Node::Text(Value::Message(Key::ExampleDatePickerNextWeek))],
            }],
        }],
    ),
    (
        r#"DatePickerDay { datetime: "2026-08-25", kind: Refused }"#,
        &[Node::Component {
            name: "DatePicker",
            props: &[
                ("name", Value::Text("closedays")),
                ("legend", Value::Message(Key::ExampleDatePickerNamedLegend)),
                ("month", Value::Text("2026-08")),
                ("month_label", Value::Message(Key::ExampleDatePickerAugust)),
                ("monday_label", Value::Message(Key::ExampleDatePickerMonday)),
                (
                    "tuesday_label",
                    Value::Message(Key::ExampleDatePickerTuesday),
                ),
                (
                    "wednesday_label",
                    Value::Message(Key::ExampleDatePickerWednesday),
                ),
                (
                    "thursday_label",
                    Value::Message(Key::ExampleDatePickerThursday),
                ),
                ("friday_label", Value::Message(Key::ExampleDatePickerFriday)),
                (
                    "saturday_label",
                    Value::Message(Key::ExampleDatePickerSaturday),
                ),
                ("sunday_label", Value::Message(Key::ExampleDatePickerSunday)),
            ],
            children: &[Node::Component {
                name: "DatePickerDay",
                props: &[
                    ("datetime", Value::Text("2026-08-25")),
                    ("kind", Value::Text("refused")),
                ],
                children: &[Node::Text(Value::Message(Key::ExampleDatePickerHoliday))],
            }],
        }],
    ),
    (
        r#"DatePickerDay { datetime: "2026-08-12", kind: Marked }"#,
        &[Node::Component {
            name: "DatePicker",
            props: &[
                ("name", Value::Text("marks")),
                ("legend", Value::Message(Key::ExampleDatePickerNamedLegend)),
                ("month", Value::Text("2026-08")),
                ("month_label", Value::Message(Key::ExampleDatePickerAugust)),
                ("monday_label", Value::Message(Key::ExampleDatePickerMonday)),
                (
                    "tuesday_label",
                    Value::Message(Key::ExampleDatePickerTuesday),
                ),
                (
                    "wednesday_label",
                    Value::Message(Key::ExampleDatePickerWednesday),
                ),
                (
                    "thursday_label",
                    Value::Message(Key::ExampleDatePickerThursday),
                ),
                ("friday_label", Value::Message(Key::ExampleDatePickerFriday)),
                (
                    "saturday_label",
                    Value::Message(Key::ExampleDatePickerSaturday),
                ),
                ("sunday_label", Value::Message(Key::ExampleDatePickerSunday)),
            ],
            children: &[Node::Component {
                name: "DatePickerDay",
                props: &[
                    ("datetime", Value::Text("2026-08-12")),
                    ("kind", Value::Text("marked")),
                ],
                children: &[Node::Text(Value::Message(Key::ExampleDatePickerMarket))],
            }],
        }],
    ),
    (
        r#"DatePickerDay { datetime: "2026-08-03/2026-08-07" }"#,
        &[Node::Component {
            name: "DatePicker",
            props: &[
                ("name", Value::Text("spans")),
                ("legend", Value::Message(Key::ExampleDatePickerNamedLegend)),
                ("month", Value::Text("2026-08")),
                ("month_label", Value::Message(Key::ExampleDatePickerAugust)),
                ("monday_label", Value::Message(Key::ExampleDatePickerMonday)),
                (
                    "tuesday_label",
                    Value::Message(Key::ExampleDatePickerTuesday),
                ),
                (
                    "wednesday_label",
                    Value::Message(Key::ExampleDatePickerWednesday),
                ),
                (
                    "thursday_label",
                    Value::Message(Key::ExampleDatePickerThursday),
                ),
                ("friday_label", Value::Message(Key::ExampleDatePickerFriday)),
                (
                    "saturday_label",
                    Value::Message(Key::ExampleDatePickerSaturday),
                ),
                ("sunday_label", Value::Message(Key::ExampleDatePickerSunday)),
            ],
            children: &[Node::Component {
                name: "DatePickerDay",
                props: &[("datetime", Value::Text("2026-08-03/2026-08-07"))],
                children: &[Node::Text(Value::Message(Key::ExampleDatePickerClosed))],
            }],
        }],
    ),
];
