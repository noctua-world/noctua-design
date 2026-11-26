//! `OptionItem`, as trees.
//!
//! One entry per example in `examples::OPTION_ITEM`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.
//!
//! A NAME THAT IS MARKED IS DATA. `hits` are positions in the text, so a name
//! that changed with the locale would be marked on other letters in each one.
//! The marked rows use a command's dotted name, which is the same in every
//! language (`NOCTUA.md §12.4`), and keep the sentence for the other line.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Every `OptionItem` example. 5 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"OptionItem { value: "…", label: "…", detail: "…" }"#,
        &[Node::Component {
            name: "OptionList",
            props: &[
                ("name", Value::Text("rows")),
                ("label", Value::Message(Key::ExampleOptionListCommands)),
            ],
            children: &[
                Node::Component {
                    name: "OptionItem",
                    props: &[
                        ("value", Value::Text("settings")),
                        ("label", Value::Message(Key::ExampleOptionListSettings)),
                        (
                            "detail",
                            Value::Message(Key::ExampleOptionListSettingsDetail),
                        ),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "OptionItem",
                    props: &[
                        ("value", Value::Text("help")),
                        ("label", Value::Message(Key::ExampleOptionListHelp)),
                        ("detail", Value::Message(Key::ExampleOptionListHelpDetail)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r#"OptionItem { label: "window.close", hits: "7-9" }"#,
        &[Node::Component {
            name: "OptionList",
            props: &[
                ("name", Value::Text("marked")),
                ("label", Value::Message(Key::ExampleOptionListCommands)),
            ],
            children: &[
                Node::Component {
                    name: "OptionItem",
                    props: &[
                        ("class", Value::Text("nd-identifier")),
                        ("value", Value::Text("window.close")),
                        ("label", Value::Data("window.close")),
                        ("hits", Value::Text("7-9")),
                        ("detail", Value::Message(Key::ExampleOptionListCloseWindow)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "OptionItem",
                    props: &[
                        ("class", Value::Text("nd-identifier")),
                        ("value", Value::Text("page.close")),
                        ("label", Value::Data("page.close")),
                        ("hits", Value::Text("5-7")),
                        ("detail", Value::Message(Key::ExampleOptionListClosePage)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r#"OptionItem { label: "window.close", hits: "0-2|", detail: "close quit exit", detail_hits: "|11-12" }"#,
        &[Node::Component {
            name: "OptionList",
            props: &[
                ("name", Value::Text("second-line")),
                ("label", Value::Message(Key::ExampleOptionListCommands)),
            ],
            children: &[
                Node::Component {
                    name: "OptionItem",
                    props: &[
                        ("class", Value::Text("nd-identifier")),
                        ("value", Value::Text("window.close")),
                        ("label", Value::Data("window.close")),
                        ("hits", Value::Text("0-2|")),
                        ("detail", Value::Data("close quit exit")),
                        ("detail_hits", Value::Text("|11-12")),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "OptionItem",
                    props: &[
                        ("class", Value::Text("nd-identifier")),
                        ("value", Value::Text("window.hide")),
                        ("label", Value::Data("window.hide")),
                        ("hits", Value::Text("0-2|")),
                        ("detail", Value::Data("hide minimize exclude")),
                        ("detail_hits", Value::Text("|14-15")),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r#"OptionItem { shortcut: "s", icon }"#,
        &[Node::Component {
            name: "OptionList",
            props: &[
                ("name", Value::Text("hinted")),
                ("label", Value::Message(Key::ExampleOptionListMenu)),
            ],
            children: &[
                Node::Component {
                    name: "OptionItem",
                    props: &[
                        ("value", Value::Text("settings")),
                        ("shortcut", Value::Message(Key::ExampleOptionListKeyS)),
                        ("icon", Value::Text("palette")),
                        ("label", Value::Message(Key::ExampleOptionListSettings)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "OptionItem",
                    props: &[
                        ("value", Value::Text("help")),
                        ("shortcut", Value::Message(Key::ExampleOptionListKeyH)),
                        ("icon", Value::Text("info")),
                        ("label", Value::Message(Key::ExampleOptionListHelp)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r#"OptionItem { id: "…", active: true } / OptionItem { selected: true } / OptionItem { disabled: true }"#,
        &[Node::Component {
            name: "OptionList",
            props: &[
                ("name", Value::Text("states")),
                ("label", Value::Message(Key::ExampleOptionListMenu)),
            ],
            children: &[
                Node::Component {
                    name: "OptionItem",
                    props: &[
                        ("value", Value::Text("settings")),
                        ("id", Value::Text("example-option-active")),
                        ("active", Value::Bool(true)),
                        ("label", Value::Message(Key::ExampleOptionListSettings)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "OptionItem",
                    props: &[
                        ("value", Value::Text("help")),
                        ("selected", Value::Bool(true)),
                        ("label", Value::Message(Key::ExampleOptionListHelp)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "OptionItem",
                    props: &[
                        ("value", Value::Text("dev")),
                        ("disabled", Value::Bool(true)),
                        ("label", Value::Message(Key::ExampleOptionListDev)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
];
