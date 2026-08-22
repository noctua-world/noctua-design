//! `Meter`, as trees.
//!
//! One entry per example in `examples::METER`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Each, Node, Value};
use crate::trees::Tree;

/// Every `Meter` example. 5 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r"Meter { name, label, value, max, unit, value_label }",
        &[Node::Component {
            name: "Meter",
            props: &[
                ("name", Value::Text("storage")),
                ("label", Value::Message(Key::ExampleMeterStorage)),
                ("value", Value::Text("41.2")),
                ("max", Value::Text("50")),
                ("unit", Value::Text("GB")),
                ("value_label", Value::Message(Key::ExampleMeterStorageValue)),
            ],
            children: &[],
        }],
    ),
    (
        r"Meter { name, label, value, max, low, high, optimum, value_label }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            // ONE READING PER BAND, which is the only way three thresholds
            // can be seen to do anything: under `low`, between the two, and
            // over `high`.
            //
            // AND ONE LABEL PER BAR. Three bars under one name is three
            // bars nobody can use, because a reader hearing the same
            // sentence three times has no way to tell them apart.
            children: &[Node::Each {
                over: Each::Rows(&[
                    &[
                        Value::Text("tokens-low"),
                        Value::Text("48"),
                        Value::Message(Key::ExampleMeterTokensAtlas),
                    ],
                    &[
                        Value::Text("tokens-warn"),
                        Value::Text("121"),
                        Value::Message(Key::ExampleMeterTokensBeacon),
                    ],
                    &[
                        Value::Text("tokens-over"),
                        Value::Text("194"),
                        Value::Message(Key::ExampleMeterTokensCorvus),
                    ],
                ]),
                body: &[Node::Component {
                    name: "Meter",
                    props: &[
                        ("name", Value::Bound(0)),
                        ("label", Value::Bound(2)),
                        ("value", Value::Bound(1)),
                        ("max", Value::Text("200")),
                        // Low is good on a quota, so `optimum` sits under
                        // `low` and the platform reads the range downward.
                        ("low", Value::Text("60")),
                        ("high", Value::Text("160")),
                        ("optimum", Value::Text("0")),
                        ("value_label", Value::Message(Key::ExampleMeterTokensValue)),
                    ],
                    children: &[],
                }],
            }],
        }],
    ),
    (
        r"Meter { name, label, value, min, max, unit, size, value_label }",
        // A SCALE THAT STARTS BELOW ZERO, which is the only thing `min`
        // is for. `min: "0"` is what absence already means, so a meter
        // declaring it shows the prop doing nothing.
        &[Node::Component {
            name: "Meter",
            props: &[
                ("name", Value::Text("freezer")),
                ("label", Value::Message(Key::ExampleMeterFreezer)),
                ("value", Value::Text("-4")),
                ("min", Value::Text("-20")),
                ("max", Value::Text("10")),
                ("unit", Value::Text("°C")),
                ("size", Value::Text("small")),
                ("value_label", Value::Message(Key::ExampleMeterFreezerValue)),
            ],
            children: &[],
        }],
    ),
    (
        r#"Meter { name, label, value, tone: "…", size: MeterSize::Large, value_label }"#,
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            // THREE CATEGORIES, THREE NAMES, THREE TONES A READER CAN TELL
            // APART. Three bars with one label is three bars nobody can
            // use, and two blues is one colour.
            //
            // AND THREE TONES THAT NAME A CATEGORY. `info`, `success` and
            // `warning` are verdicts, so painting a share of a disk with
            // them says one slice is good and another is trouble. These
            // three name what the bytes ARE, which is the whole point of a
            // tone on a bar with no good end.
            children: &[Node::Each {
                over: Each::Rows(&[
                    &[
                        Value::Text("share-cached"),
                        Value::Text("0.42"),
                        Value::Text("cached"),
                        Value::Message(Key::ExampleMeterShareCached),
                    ],
                    &[
                        Value::Text("share-archived"),
                        Value::Text("0.35"),
                        Value::Text("archived"),
                        Value::Message(Key::ExampleMeterShareArchived),
                    ],
                    &[
                        Value::Text("share-shared"),
                        Value::Text("0.23"),
                        Value::Text("shared"),
                        Value::Message(Key::ExampleMeterShareShared),
                    ],
                ]),
                body: &[Node::Component {
                    name: "Meter",
                    props: &[
                        ("name", Value::Bound(0)),
                        ("label", Value::Bound(3)),
                        ("value", Value::Bound(1)),
                        ("tone", Value::Bound(2)),
                        ("size", Value::Text("large")),
                        ("value_label", Value::Message(Key::ExampleMeterShareValue)),
                    ],
                    children: &[],
                }],
            }],
        }],
    ),
    (
        r"Meter { name, label, value, max, unit, hide_label: true, show_value: false, size, value_label }",
        // A READING OF ITS OWN. Two bars showing the same number is two
        // pictures of one thing, and a reader comparing this page's
        // examples has no way to tell which one they are hearing.
        &[Node::Component {
            name: "Meter",
            props: &[
                ("name", Value::Text("bare")),
                ("label", Value::Message(Key::ExampleMeterStorage)),
                ("value", Value::Text("8.6")),
                ("max", Value::Text("50")),
                ("unit", Value::Text("GB")),
                ("hide_label", Value::Bool(true)),
                ("show_value", Value::Bool(false)),
                ("size", Value::Text("small")),
                ("value_label", Value::Message(Key::ExampleMeterStorageValue)),
            ],
            children: &[],
        }],
    ),
];
