//! `OptionGroup`, as trees.
//!
//! One entry per example in `examples::OPTION_GROUP`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Every `OptionGroup` example. 4 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"OptionGroup { label: "…", OptionItem { … } }"#,
        &[Node::Component {
            name: "OptionList",
            props: &[
                ("name", Value::Text("grouped")),
                ("label", Value::Message(Key::ExampleOptionListCommands)),
            ],
            children: &[
                Node::Component {
                    name: "OptionGroup",
                    props: &[("label", Value::Message(Key::ExampleOptionListGroupWindow))],
                    children: &[
                        Node::Component {
                            name: "OptionItem",
                            props: &[
                                ("class", Value::Text("nd-identifier")),
                                ("value", Value::Text("window.close")),
                                ("label", Value::Data("window.close")),
                                ("detail", Value::Message(Key::ExampleOptionListCloseWindow)),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "OptionItem",
                            props: &[
                                ("class", Value::Text("nd-identifier")),
                                ("value", Value::Text("window.hide")),
                                ("label", Value::Data("window.hide")),
                                ("detail", Value::Message(Key::ExampleOptionListHideWindow)),
                            ],
                            children: &[],
                        },
                    ],
                },
                Node::Component {
                    name: "OptionGroup",
                    props: &[("label", Value::Message(Key::ExampleOptionListGroupPage))],
                    children: &[Node::Component {
                        name: "OptionItem",
                        props: &[
                            ("class", Value::Text("nd-identifier")),
                            ("value", Value::Text("page.close")),
                            ("label", Value::Data("page.close")),
                            ("detail", Value::Message(Key::ExampleOptionListClosePage)),
                        ],
                        children: &[],
                    }],
                },
            ],
        }],
    ),
    (
        r"OptionItem { … } beside OptionGroup { … }",
        &[Node::Component {
            name: "OptionList",
            props: &[
                ("name", Value::Text("mixed")),
                ("label", Value::Message(Key::ExampleOptionListMenu)),
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
                    name: "OptionGroup",
                    props: &[("label", Value::Message(Key::ExampleOptionListGroupPage))],
                    children: &[
                        Node::Component {
                            name: "OptionItem",
                            props: &[
                                ("value", Value::Text("close")),
                                ("label", Value::Message(Key::ExampleOptionListClosePage)),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "OptionItem",
                            props: &[
                                ("value", Value::Text("shortcuts")),
                                ("label", Value::Message(Key::ExampleOptionListShortcuts)),
                            ],
                            children: &[],
                        },
                    ],
                },
            ],
        }],
    ),
    (
        r"OptionList { layout: OptionListLayout::Columns, OptionGroup { … } }",
        &[Node::Component {
            name: "OptionList",
            props: &[
                ("name", Value::Text("group-columns")),
                ("label", Value::Message(Key::ExampleOptionListMenu)),
                ("layout", Value::Text("columns")),
            ],
            children: &[
                Node::Component {
                    name: "OptionGroup",
                    props: &[("label", Value::Message(Key::ExampleOptionListGroupWindow))],
                    children: &[
                        Node::Component {
                            name: "OptionItem",
                            props: &[
                                ("value", Value::Text("close-window")),
                                ("shortcut", Value::Message(Key::ExampleOptionListKeyQ)),
                                ("label", Value::Message(Key::ExampleOptionListCloseWindow)),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "OptionItem",
                            props: &[
                                ("value", Value::Text("hide-window")),
                                ("shortcut", Value::Message(Key::ExampleOptionListKeyH)),
                                ("label", Value::Message(Key::ExampleOptionListHideWindow)),
                            ],
                            children: &[],
                        },
                    ],
                },
                Node::Component {
                    name: "OptionGroup",
                    props: &[("label", Value::Message(Key::ExampleOptionListGroupPage))],
                    children: &[
                        Node::Component {
                            name: "OptionItem",
                            props: &[
                                ("value", Value::Text("close-page")),
                                ("shortcut", Value::Message(Key::ExampleOptionListKeyD)),
                                ("label", Value::Message(Key::ExampleOptionListClosePage)),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "OptionItem",
                            props: &[
                                ("value", Value::Text("shortcuts")),
                                ("shortcut", Value::Message(Key::ExampleOptionListKeyS)),
                                ("label", Value::Message(Key::ExampleOptionListShortcuts)),
                            ],
                            children: &[],
                        },
                    ],
                },
            ],
        }],
    ),
    (
        r#"OptionGroup { label: "…", OptionItem { hits: "…" } }"#,
        &[Node::Component {
            name: "OptionList",
            props: &[
                ("name", Value::Text("group-filtered")),
                ("label", Value::Message(Key::ExampleOptionListMenu)),
            ],
            children: &[
                Node::Component {
                    name: "OptionGroup",
                    props: &[("label", Value::Message(Key::ExampleOptionListGroupWindow))],
                    children: &[Node::Component {
                        name: "OptionItem",
                        props: &[
                            ("class", Value::Text("nd-identifier")),
                            ("value", Value::Text("window.close")),
                            ("label", Value::Data("window.close")),
                            ("hits", Value::Text("7-9")),
                        ],
                        children: &[],
                    }],
                },
                Node::Component {
                    name: "OptionGroup",
                    props: &[("label", Value::Message(Key::ExampleOptionListGroupPage))],
                    children: &[Node::Component {
                        name: "OptionItem",
                        props: &[
                            ("class", Value::Text("nd-identifier")),
                            ("value", Value::Text("page.close")),
                            ("label", Value::Data("page.close")),
                            ("hits", Value::Text("5-7")),
                        ],
                        children: &[],
                    }],
                },
            ],
        }],
    ),
];
