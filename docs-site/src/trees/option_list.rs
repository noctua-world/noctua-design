//! `OptionList`, as trees.
//!
//! One entry per example in `examples::OPTION_LIST`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Every `OptionList` example. 5 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"OptionList { name: "…", label: "…", OptionItem { value: "…", label: "…" } }"#,
        &[Node::Component {
            name: "OptionList",
            props: &[
                ("name", Value::Text("walk")),
                ("label", Value::Message(Key::ExampleOptionListCommands)),
            ],
            children: &[
                Node::Component {
                    name: "OptionItem",
                    props: &[
                        ("value", Value::Text("settings")),
                        ("label", Value::Message(Key::ExampleOptionListSettings)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "OptionItem",
                    props: &[
                        ("value", Value::Text("help")),
                        ("label", Value::Message(Key::ExampleOptionListHelp)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "OptionItem",
                    props: &[
                        ("value", Value::Text("sidebar")),
                        ("label", Value::Message(Key::ExampleOptionListSidebar)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r"OptionList { layout: OptionListLayout::Columns, size: OptionListSize::Small }",
        &[Node::Component {
            name: "OptionList",
            props: &[
                ("name", Value::Text("leader")),
                ("label", Value::Message(Key::ExampleOptionListMenu)),
                ("layout", Value::Text("columns")),
                ("size", Value::Text("small")),
            ],
            children: &[
                Node::Component {
                    name: "OptionItem",
                    props: &[
                        ("value", Value::Text("sidebar")),
                        ("shortcut", Value::Message(Key::ExampleOptionListKeySpace)),
                        ("label", Value::Message(Key::ExampleOptionListSidebar)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "OptionItem",
                    props: &[
                        ("value", Value::Text("settings")),
                        ("shortcut", Value::Message(Key::ExampleOptionListKeyS)),
                        ("label", Value::Message(Key::ExampleOptionListSettings)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "OptionItem",
                    props: &[
                        ("value", Value::Text("help")),
                        ("shortcut", Value::Message(Key::ExampleOptionListKeyH)),
                        ("label", Value::Message(Key::ExampleOptionListHelp)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "OptionItem",
                    props: &[
                        ("value", Value::Text("dev")),
                        ("shortcut", Value::Message(Key::ExampleOptionListKeyD)),
                        ("label", Value::Message(Key::ExampleOptionListDev)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "OptionItem",
                    props: &[
                        ("value", Value::Text("close")),
                        ("shortcut", Value::Message(Key::ExampleOptionListKeyQ)),
                        ("label", Value::Message(Key::ExampleOptionListClosePage)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "OptionItem",
                    props: &[
                        ("value", Value::Text("shortcuts")),
                        ("shortcut", Value::Data("/")),
                        ("label", Value::Message(Key::ExampleOptionListShortcuts)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r"OptionList { size: OptionListSize::Large }",
        &[Node::Component {
            name: "OptionList",
            props: &[
                ("name", Value::Text("large")),
                ("label", Value::Message(Key::ExampleOptionListCommands)),
                ("size", Value::Text("large")),
            ],
            children: &[
                Node::Component {
                    name: "OptionItem",
                    props: &[
                        ("value", Value::Text("settings")),
                        ("label", Value::Message(Key::ExampleOptionListSettings)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "OptionItem",
                    props: &[
                        ("value", Value::Text("help")),
                        ("label", Value::Message(Key::ExampleOptionListHelp)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "OptionItem",
                    props: &[
                        ("value", Value::Text("sidebar")),
                        ("label", Value::Message(Key::ExampleOptionListSidebar)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r#"OptionList { focusable: false } beside <input aria-controls="nd-option-list-…">"#,
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[
                Node::Component {
                    name: "Label",
                    props: &[
                        ("text", Value::Message(Key::ExampleOptionListFilter)),
                        ("for_control", Value::Text("example-option-list-box")),
                    ],
                    children: &[],
                },
                // A BARE `<input>`, because no component here writes
                // `aria-controls` on a text box yet and the example is about
                // the list: the box names it, and the list answers its arrows.
                Node::Element {
                    tag: "input",
                    attributes: &[
                        ("class", Value::Text("nd-input-box")),
                        ("id", Value::Text("example-option-list-box")),
                        ("type", Value::Text("text")),
                        ("role", Value::Text("combobox")),
                        ("aria-expanded", Value::Text("true")),
                        ("aria-autocomplete", Value::Text("list")),
                        ("aria-controls", Value::Text("nd-option-list-driven")),
                        ("autocomplete", Value::Text("off")),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "OptionList",
                    props: &[
                        ("name", Value::Text("driven")),
                        ("label", Value::Message(Key::ExampleOptionListCommands)),
                        ("focusable", Value::Bool(false)),
                    ],
                    children: &[
                        Node::Component {
                            name: "OptionItem",
                            props: &[
                                ("value", Value::Text("settings")),
                                ("label", Value::Message(Key::ExampleOptionListSettings)),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "OptionItem",
                            props: &[
                                ("value", Value::Text("help")),
                                ("label", Value::Message(Key::ExampleOptionListHelp)),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "OptionItem",
                            props: &[
                                ("value", Value::Text("sidebar")),
                                ("label", Value::Message(Key::ExampleOptionListSidebar)),
                            ],
                            children: &[],
                        },
                    ],
                },
            ],
        }],
    ),
    (
        r#"OptionList { empty_label: "…" }"#,
        &[Node::Component {
            name: "OptionList",
            props: &[
                ("name", Value::Text("nothing")),
                ("label", Value::Message(Key::ExampleOptionListCommands)),
                ("empty_label", Value::Message(Key::ExampleOptionListEmpty)),
            ],
            children: &[],
        }],
    ),
];
