//! `DatePicker`, as trees.
//!
//! One entry per example in `examples::DATE_PICKER`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a reader can
//! check: `every_example_has_a_tree` reads it against the table's own.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Every `DatePicker` example. 12 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r"DatePicker { name, legend, month, month_label, monday_label, tuesday_label, wednesday_label, thursday_label, friday_label, saturday_label, sunday_label, today, previous_label, next_label }",
        &[Node::Component {
            name: "DatePicker",
            props: &[
                ("name", Value::Text("visit")),
                ("legend", Value::Message(Key::ExampleDatePickerLegend)),
                ("month", Value::Text("2026-08")),
                ("month_label", Value::Message(Key::ExampleDatePickerAugust)),
                ("today", Value::Text("2026-08-14")),
                (
                    "previous_label",
                    Value::Message(Key::ExampleDatePickerPrevious),
                ),
                ("next_label", Value::Message(Key::ExampleDatePickerNext)),
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
            children: &[],
        }],
    ),
    (
        r#"DatePicker { selection: Range, months: Two, second_month_label, value: "2026-08-18/2026-08-24", range_label, clear_label, cleared_label }"#,
        &[Node::Component {
            name: "DatePicker",
            props: &[
                ("name", Value::Text("stay")),
                ("legend", Value::Message(Key::ExampleDatePickerStayLegend)),
                ("month", Value::Text("2026-08")),
                ("month_label", Value::Message(Key::ExampleDatePickerAugust)),
                (
                    "second_month_label",
                    Value::Message(Key::ExampleDatePickerSeptember),
                ),
                ("months", Value::Text("two")),
                ("selection", Value::Text("range")),
                ("value", Value::Text("2026-08-18/2026-08-24")),
                ("range_label", Value::Message(Key::ExampleDatePickerRange)),
                ("clear_label", Value::Message(Key::ExampleDatePickerClear)),
                (
                    "cleared_label",
                    Value::Message(Key::ExampleDatePickerCleared),
                ),
                ("today", Value::Text("2026-08-14")),
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
            children: &[],
        }],
    ),
    (
        r#"DatePicker { min: "2026-08-10", max: "2026-08-28", min_length: "2", max_length: "9", too_short_label, too_long_label, blocked_label }"#,
        &[Node::Component {
            name: "DatePicker",
            props: &[
                ("name", Value::Text("booking")),
                ("legend", Value::Message(Key::ExampleDatePickerBoundsLegend)),
                ("month", Value::Text("2026-08")),
                ("month_label", Value::Message(Key::ExampleDatePickerAugust)),
                ("selection", Value::Text("range")),
                ("min", Value::Text("2026-08-10")),
                ("max", Value::Text("2026-08-28")),
                ("min_length", Value::Text("2")),
                ("max_length", Value::Text("9")),
                (
                    "too_short_label",
                    Value::Message(Key::ExampleDatePickerTooShort),
                ),
                (
                    "too_long_label",
                    Value::Message(Key::ExampleDatePickerTooLong),
                ),
                (
                    "blocked_label",
                    Value::Message(Key::ExampleDatePickerBlocked),
                ),
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
            children: &[],
        }],
    ),
    (
        r"DatePicker { form: Field, order, format_dmy, format_mdy, format_ymd, open_label, invalid_label, chosen_label, close_label }",
        &[Node::Component {
            name: "DatePicker",
            props: &[
                ("name", Value::Text("arrival")),
                ("legend", Value::Message(Key::ExampleDatePickerFieldLegend)),
                ("month", Value::Text("2026-08")),
                ("month_label", Value::Message(Key::ExampleDatePickerAugust)),
                ("form", Value::Text("field")),
                ("order", Value::Text("auto")),
                (
                    "format_dmy",
                    Value::Message(Key::ExampleDatePickerFormatDmy),
                ),
                (
                    "format_mdy",
                    Value::Message(Key::ExampleDatePickerFormatMdy),
                ),
                (
                    "format_ymd",
                    Value::Message(Key::ExampleDatePickerFormatYmd),
                ),
                ("open_label", Value::Message(Key::ExampleDatePickerOpen)),
                (
                    "invalid_label",
                    Value::Message(Key::ExampleDatePickerInvalid),
                ),
                ("chosen_label", Value::Message(Key::ExampleDatePickerChosen)),
                ("close_label", Value::Message(Key::ExampleDatePickerClose)),
                (
                    "previous_label",
                    Value::Message(Key::ExampleDatePickerPrevious),
                ),
                ("next_label", Value::Message(Key::ExampleDatePickerNext)),
                ("today", Value::Text("2026-08-14")),
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
            children: &[],
        }],
    ),
    (
        r"DatePicker { form: Field, selection: Range, start_label, end_label, range_label }",
        &[Node::Component {
            name: "DatePicker",
            props: &[
                ("name", Value::Text("trip")),
                ("legend", Value::Message(Key::ExampleDatePickerTripLegend)),
                ("month", Value::Text("2026-08")),
                ("month_label", Value::Message(Key::ExampleDatePickerAugust)),
                (
                    "second_month_label",
                    Value::Message(Key::ExampleDatePickerSeptember),
                ),
                ("months", Value::Text("two")),
                ("form", Value::Text("field")),
                ("selection", Value::Text("range")),
                ("start_label", Value::Message(Key::ExampleDatePickerStart)),
                ("end_label", Value::Message(Key::ExampleDatePickerEnd)),
                ("range_label", Value::Message(Key::ExampleDatePickerRange)),
                (
                    "format_dmy",
                    Value::Message(Key::ExampleDatePickerFormatDmy),
                ),
                ("open_label", Value::Message(Key::ExampleDatePickerOpen)),
                ("clear_label", Value::Message(Key::ExampleDatePickerClear)),
                ("close_label", Value::Message(Key::ExampleDatePickerClose)),
                (
                    "previous_label",
                    Value::Message(Key::ExampleDatePickerPrevious),
                ),
                ("next_label", Value::Message(Key::ExampleDatePickerNext)),
                ("today", Value::Text("2026-08-14")),
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
            children: &[],
        }],
    ),
    (
        r"DatePicker { form: Field, selection: Range, pair: Attached, start_label, end_label }",
        &[Node::Component {
            name: "DatePicker",
            props: &[
                ("name", Value::Text("flight")),
                ("legend", Value::Message(Key::ExampleDatePickerFlightLegend)),
                ("month", Value::Text("2026-08")),
                ("month_label", Value::Message(Key::ExampleDatePickerAugust)),
                ("form", Value::Text("field")),
                ("selection", Value::Text("range")),
                (
                    "format_dmy",
                    Value::Message(Key::ExampleDatePickerFormatDmy),
                ),
                ("open_label", Value::Message(Key::ExampleDatePickerOpen)),
                ("range_label", Value::Message(Key::ExampleDatePickerRange)),
                (
                    "previous_label",
                    Value::Message(Key::ExampleDatePickerPrevious),
                ),
                ("next_label", Value::Message(Key::ExampleDatePickerNext)),
                ("today", Value::Text("2026-08-14")),
                ("pair", Value::Text("attached")),
                (
                    "start_label",
                    Value::Message(Key::ExampleDatePickerDeparting),
                ),
                ("end_label", Value::Message(Key::ExampleDatePickerReturning)),
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
            children: &[],
        }],
    ),
    (
        r"DatePicker { form: Field, selection: Range, pair: Stacked, labels: Hidden }",
        &[Node::Component {
            name: "DatePicker",
            props: &[
                ("name", Value::Text("nights")),
                ("legend", Value::Message(Key::ExampleDatePickerNightsLegend)),
                ("month", Value::Text("2026-08")),
                ("month_label", Value::Message(Key::ExampleDatePickerAugust)),
                ("form", Value::Text("field")),
                ("selection", Value::Text("range")),
                ("pair", Value::Text("stacked")),
                (
                    "format_dmy",
                    Value::Message(Key::ExampleDatePickerFormatDmy),
                ),
                ("open_label", Value::Message(Key::ExampleDatePickerOpen)),
                ("range_label", Value::Message(Key::ExampleDatePickerRange)),
                (
                    "previous_label",
                    Value::Message(Key::ExampleDatePickerPrevious),
                ),
                ("next_label", Value::Message(Key::ExampleDatePickerNext)),
                ("today", Value::Text("2026-08-14")),
                ("labels", Value::Text("hidden")),
                ("start_label", Value::Message(Key::ExampleDatePickerCheckIn)),
                ("end_label", Value::Message(Key::ExampleDatePickerCheckOut)),
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
            children: &[],
        }],
    ),
    (
        r"DatePicker { form: Field, selection: Range, pair: Stacked, labels: Inside }",
        &[Node::Component {
            name: "DatePicker",
            props: &[
                ("name", Value::Text("rental")),
                ("legend", Value::Message(Key::ExampleDatePickerRentalLegend)),
                ("month", Value::Text("2026-08")),
                ("month_label", Value::Message(Key::ExampleDatePickerAugust)),
                ("form", Value::Text("field")),
                ("selection", Value::Text("range")),
                ("pair", Value::Text("stacked")),
                (
                    "format_dmy",
                    Value::Message(Key::ExampleDatePickerFormatDmy),
                ),
                ("open_label", Value::Message(Key::ExampleDatePickerOpen)),
                ("range_label", Value::Message(Key::ExampleDatePickerRange)),
                (
                    "previous_label",
                    Value::Message(Key::ExampleDatePickerPrevious),
                ),
                ("next_label", Value::Message(Key::ExampleDatePickerNext)),
                ("today", Value::Text("2026-08-14")),
                ("labels", Value::Text("inside")),
                ("start_label", Value::Message(Key::ExampleDatePickerPickUp)),
                ("end_label", Value::Message(Key::ExampleDatePickerDropOff)),
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
            children: &[],
        }],
    ),
    (
        r"DatePicker { months: Three, third_month_label, week_numbers: true, week_label, first_day: Sunday, selection: Multiple, count_label }",
        &[Node::Component {
            name: "DatePicker",
            props: &[
                ("name", Value::Text("term")),
                ("legend", Value::Message(Key::ExampleDatePickerNamedLegend)),
                ("month", Value::Text("2026-08")),
                ("month_label", Value::Message(Key::ExampleDatePickerAugust)),
                (
                    "second_month_label",
                    Value::Message(Key::ExampleDatePickerSeptember),
                ),
                (
                    "third_month_label",
                    Value::Message(Key::ExampleDatePickerOctober),
                ),
                ("months", Value::Text("three")),
                ("week_numbers", Value::Bool(true)),
                ("week_label", Value::Message(Key::ExampleDatePickerWeek)),
                ("first_day", Value::Text("sunday")),
                ("selection", Value::Text("multiple")),
                ("count_label", Value::Message(Key::ExampleDatePickerCount)),
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
            children: &[],
        }],
    ),
    (
        r"DatePicker { DatePickerDay { datetime, kind, children } }",
        &[Node::Component {
            name: "DatePicker",
            props: &[
                ("name", Value::Text("delivery")),
                ("legend", Value::Message(Key::ExampleDatePickerFormsLegend)),
                ("month", Value::Text("2026-08")),
                ("month_label", Value::Message(Key::ExampleDatePickerAugust)),
                ("hint", Value::Message(Key::ExampleDatePickerHint)),
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
            children: &[
                Node::Component {
                    name: "DatePickerDay",
                    props: &[
                        ("datetime", Value::Text("2026-08-17/2026-08-21")),
                        ("kind", Value::Text("preset")),
                    ],
                    children: &[Node::Text(Value::Message(Key::ExampleDatePickerThisWeek))],
                },
                Node::Component {
                    name: "DatePickerDay",
                    props: &[
                        ("datetime", Value::Text("2026-08-25")),
                        ("kind", Value::Text("refused")),
                    ],
                    children: &[Node::Text(Value::Message(Key::ExampleDatePickerHoliday))],
                },
                Node::Component {
                    name: "DatePickerDay",
                    props: &[
                        ("datetime", Value::Text("2026-08-12")),
                        ("kind", Value::Text("marked")),
                    ],
                    children: &[Node::Text(Value::Message(Key::ExampleDatePickerMarket))],
                },
            ],
        }],
    ),
    (
        r"DatePicker { size: Small }  ·  { size: Large }  ·  { disabled: true }  ·  { required: true, hint, error, error_label }  ·  { posts: false }",
        &[
            Node::Component {
                name: "DatePicker",
                props: &[
                    ("name", Value::Text("compact")),
                    ("legend", Value::Message(Key::ExampleDatePickerSmallLegend)),
                    ("month", Value::Text("2026-08")),
                    ("month_label", Value::Message(Key::ExampleDatePickerAugust)),
                    ("size", Value::Text("small")),
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
                children: &[],
            },
            Node::Component {
                name: "DatePicker",
                props: &[
                    ("name", Value::Text("roomy")),
                    ("legend", Value::Message(Key::ExampleDatePickerLargeLegend)),
                    ("month", Value::Text("2026-08")),
                    ("month_label", Value::Message(Key::ExampleDatePickerAugust)),
                    ("size", Value::Text("large")),
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
                children: &[],
            },
            Node::Component {
                name: "DatePicker",
                props: &[
                    ("name", Value::Text("closed")),
                    ("legend", Value::Message(Key::ExampleDatePickerClosedLegend)),
                    ("month", Value::Text("2026-08")),
                    ("month_label", Value::Message(Key::ExampleDatePickerAugust)),
                    ("disabled", Value::Bool(true)),
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
                children: &[],
            },
            Node::Component {
                name: "DatePicker",
                props: &[
                    ("name", Value::Text("collect")),
                    (
                        "legend",
                        Value::Message(Key::ExampleDatePickerInvalidLegend),
                    ),
                    ("month", Value::Text("2026-08")),
                    ("month_label", Value::Message(Key::ExampleDatePickerAugust)),
                    ("required", Value::Bool(true)),
                    ("error", Value::Message(Key::ExampleDatePickerError)),
                    (
                        "error_label",
                        Value::Message(Key::ExampleDatePickerErrorWord),
                    ),
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
                children: &[],
            },
            Node::Component {
                name: "DatePicker",
                props: &[
                    ("name", Value::Text("nested")),
                    ("legend", Value::Message(Key::ExampleDatePickerNestedLegend)),
                    ("month", Value::Text("2026-08")),
                    ("month_label", Value::Message(Key::ExampleDatePickerAugust)),
                    ("posts", Value::Bool(false)),
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
                children: &[],
            },
        ],
    ),
    (
        r"DatePicker { ai_mark, ai_label, ai_note }",
        &[Node::Component {
            name: "DatePicker",
            props: &[
                ("name", Value::Text("marked")),
                ("legend", Value::Message(Key::ExampleAiSubject)),
                ("month", Value::Text("2026-08")),
                ("month_label", Value::Message(Key::ExampleDatePickerAugust)),
                ("ai_mark", Value::Message(Key::ExampleAiMark)),
                ("ai_label", Value::Message(Key::ExampleAiName)),
                ("ai_note", Value::Message(Key::ExampleAiNote)),
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
            children: &[],
        }],
    ),
];
