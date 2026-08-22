//! `SegmentedItem`, as trees.
//!
//! One entry per example in `examples::SEGMENTED_ITEM`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Each, Node, Value};
use crate::trees::Tree;

/// Every `SegmentedItem` example. 7 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"SegmentedItem { value: "…", label: "…" }"#,
        // ONE ITEM STATES THE ANSWER. A radiogroup with nothing chosen is a
        // legal state and the group is still one tab stop, but the first
        // picture of this component should show what a radio group looks
        // like when it has been answered.
        &[Node::Component {
            name: "Segmented",
            props: &[
                ("name", Value::Text("example-item")),
                ("label", Value::Message(Key::ThemeSaturationLabel)),
            ],
            children: &[
                Node::Component {
                    name: "SegmentedItem",
                    props: &[
                        ("value", Value::Text("balanced")),
                        ("label", Value::Message(Key::ThemeSaturationBalanced)),
                        ("selected", Value::Bool(true)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "SegmentedItem",
                    props: &[
                        ("value", Value::Text("vivid")),
                        ("label", Value::Message(Key::ThemeSaturationVivid)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r"SegmentedItem { selected: true, … }",
        &[Node::Component {
            name: "Segmented",
            props: &[
                ("name", Value::Text("example-selected")),
                ("label", Value::Message(Key::ThemeSaturationLabel)),
            ],
            children: &[
                Node::Component {
                    name: "SegmentedItem",
                    props: &[
                        ("value", Value::Text("vivid")),
                        ("label", Value::Message(Key::ThemeSaturationVivid)),
                        ("selected", Value::Bool(true)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "SegmentedItem",
                    props: &[
                        ("value", Value::Text("sober")),
                        ("label", Value::Message(Key::ThemeSaturationSober)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r"SegmentedItem { icon: …, hide_label: true, … }",
        &[Node::Component {
            name: "Segmented",
            props: &[
                ("name", Value::Text("example-icons")),
                ("label", Value::Message(Key::ThemeModeLabel)),
            ],
            children: &[Node::Each {
                over: Each::Rows(&[
                    &[
                        Value::Text("light"),
                        Value::Text("sun"),
                        Value::Message(Key::ThemeModeLight),
                        Value::Bool(false),
                    ],
                    &[
                        Value::Text("dark"),
                        Value::Text("moon"),
                        Value::Message(Key::ThemeModeDark),
                        Value::Bool(false),
                    ],
                    &[
                        Value::Text("system"),
                        Value::Text("monitor"),
                        Value::Message(Key::ThemeModeSystem),
                        Value::Bool(true),
                    ],
                ]),
                body: &[Node::Component {
                    name: "SegmentedItem",
                    props: &[
                        ("value", Value::Bound(0)),
                        ("icon", Value::Bound(1)),
                        ("hide_label", Value::Bool(true)),
                        ("label", Value::Bound(2)),
                        ("selected", Value::Bound(3)),
                    ],
                    children: &[],
                }],
            }],
        }],
    ),
    (
        r"SegmentedItem { disabled: true, … }",
        &[Node::Component {
            name: "Segmented",
            props: &[
                ("name", Value::Text("example-disabled")),
                ("label", Value::Message(Key::ThemeFontLabel)),
            ],
            children: &[
                Node::Component {
                    name: "SegmentedItem",
                    props: &[
                        ("value", Value::Text("sans")),
                        ("label", Value::Message(Key::ThemeFontSans)),
                        ("selected", Value::Bool(true)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "SegmentedItem",
                    props: &[
                        ("value", Value::Text("mono")),
                        ("label", Value::Message(Key::ThemeFontMono)),
                        ("disabled", Value::Bool(true)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r"SegmentedItem { Swatch { … } }",
        &[Node::Component {
            name: "Segmented",
            props: &[
                ("name", Value::Text("example-accent")),
                ("label", Value::Message(Key::ThemeAccentLabel)),
                ("appearance", Value::Text("spread")),
            ],
            children: &[Node::Each {
                over: Each::Rows(&[
                    &[
                        Value::Text("rose-vivid"),
                        Value::Message(Key::ThemeAccentRose),
                        Value::Bool(false),
                    ],
                    &[
                        Value::Text("jade-balanced"),
                        Value::Message(Key::ThemeAccentJade),
                        Value::Bool(true),
                    ],
                    &[
                        Value::Text("indigo-sober"),
                        Value::Message(Key::ThemeAccentIndigo),
                        Value::Bool(false),
                    ],
                ]),
                body: &[Node::Component {
                    name: "SegmentedItem",
                    props: &[
                        ("value", Value::Bound(0)),
                        ("label", Value::Bound(1)),
                        ("selected", Value::Bound(2)),
                    ],
                    children: &[Node::Component {
                        name: "Swatch",
                        props: &[("palette", Value::Bound(0)), ("size", Value::Text("small"))],
                        children: &[],
                    }],
                }],
            }],
        }],
    ),
    (
        r"SegmentedItem { glass: true, … }",
        &[Node::Component {
            name: "Segmented",
            props: &[
                ("name", Value::Text("example-item-glass")),
                ("label", Value::Message(Key::ThemeSaturationLabel)),
            ],
            children: &[
                Node::Component {
                    name: "SegmentedItem",
                    props: &[
                        ("value", Value::Text("balanced")),
                        ("glass", Value::Bool(true)),
                        ("label", Value::Message(Key::ThemeSaturationBalanced)),
                        ("selected", Value::Bool(true)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "SegmentedItem",
                    props: &[
                        ("value", Value::Text("vivid")),
                        ("glass", Value::Bool(true)),
                        ("label", Value::Message(Key::ThemeSaturationVivid)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r#"SegmentedItem { label: "320", detail: "px" }"#,
        &[Node::Component {
            name: "Segmented",
            props: &[
                ("name", Value::Text("example-detail")),
                ("label", Value::Message(Key::ExampleWidthLabel)),
            ],
            children: &[Node::Each {
                over: Each::Rows(&[
                    &[
                        Value::Text("320"),
                        Value::Data("320"),
                        Value::Data("px"),
                        Value::Bool(false),
                    ],
                    &[
                        Value::Text("sm"),
                        Value::Data("30"),
                        Value::Data("rem"),
                        Value::Bool(true),
                    ],
                    &[
                        Value::Text("md"),
                        Value::Data("48"),
                        Value::Data("rem"),
                        Value::Bool(false),
                    ],
                ]),
                // A length is an identifier, on both lines. The scanner
                // that finds untranslated prose is right to ask, and
                // marking it is how the width control answers.
                body: &[Node::Component {
                    name: "SegmentedItem",
                    props: &[
                        ("value", Value::Bound(0)),
                        ("class", Value::Text("nd-identifier")),
                        ("label", Value::Bound(1)),
                        ("detail", Value::Bound(2)),
                        ("selected", Value::Bound(3)),
                    ],
                    children: &[],
                }],
            }],
        }],
    ),
];
