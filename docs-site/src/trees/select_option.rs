//! `SelectOption`, as trees.
//!
//! One entry per example in `examples::SELECT_OPTION`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_tree_belongs_to_its_example` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;
use crate::trees::select::{AMBER_DETAILED, CLAY_DETAILED, OCHRE_DETAILED};

/// Every `SelectOption` example. 4 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"SelectOption { value: "…", label: "…" }"#,
        &[Node::Component {
            name: "Select",
            props: &[
                ("name", Value::Text("option-plain")),
                ("label", Value::Message(Key::ExampleSelectPaletteLabel)),
                (
                    "placeholder",
                    Value::Message(Key::ExampleSelectPalettePlaceholder),
                ),
            ],
            children: &[
                Node::Component {
                    name: "SelectOption",
                    props: &[
                        ("class", Value::Text("nd-identifier")),
                        ("value", Value::Text("ochre-balanced")),
                        ("label", Value::Data("Ochre")),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "SelectOption",
                    props: &[
                        ("class", Value::Text("nd-identifier")),
                        ("value", Value::Text("teal-balanced")),
                        ("label", Value::Data("Teal")),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r#"SelectOption { detail: "…", swatch: "…" }"#,
        &[Node::Component {
            name: "Select",
            props: &[
                ("name", Value::Text("option-rich")),
                ("label", Value::Message(Key::ExampleSelectPaletteLabel)),
                (
                    "placeholder",
                    Value::Message(Key::ExampleSelectPalettePlaceholder),
                ),
                ("searchable", Value::Bool(true)),
                (
                    "search_label",
                    Value::Message(Key::ExampleSelectSearchLabel),
                ),
                ("empty_label", Value::Message(Key::ExampleSelectEmptyLabel)),
            ],
            children: &[
                Node::Fragment(AMBER_DETAILED),
                Node::Fragment(OCHRE_DETAILED),
                Node::Fragment(CLAY_DETAILED),
            ],
        }],
    ),
    (
        r"SelectOption { icon: IconName::Sun, … }",
        &[Node::Component {
            name: "Select",
            props: &[
                ("name", Value::Text("option-icon")),
                ("label", Value::Message(Key::ExampleSelectModeLabel)),
            ],
            children: &[
                Node::Component {
                    name: "SelectOption",
                    props: &[
                        ("value", Value::Text("light")),
                        ("label", Value::Message(Key::ExampleSelectModeLight)),
                        ("icon", Value::Text("sun")),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "SelectOption",
                    props: &[
                        ("value", Value::Text("dark")),
                        ("label", Value::Message(Key::ExampleSelectModeDark)),
                        ("icon", Value::Text("moon")),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "SelectOption",
                    props: &[
                        ("value", Value::Text("system")),
                        ("label", Value::Message(Key::ExampleSelectModeSystem)),
                        ("icon", Value::Text("monitor")),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r"SelectOption { selected: true, disabled: true }",
        &[Node::Component {
            name: "Select",
            props: &[
                ("name", Value::Text("option-states")),
                ("label", Value::Message(Key::ExampleSelectModeLabel)),
            ],
            children: &[
                Node::Component {
                    name: "SelectOption",
                    props: &[
                        ("value", Value::Text("light")),
                        ("label", Value::Message(Key::ExampleSelectModeLight)),
                        ("icon", Value::Text("sun")),
                        ("selected", Value::Bool(true)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "SelectOption",
                    props: &[
                        ("value", Value::Text("dark")),
                        ("label", Value::Message(Key::ExampleSelectModeDark)),
                        ("icon", Value::Text("moon")),
                        ("disabled", Value::Bool(true)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
];
