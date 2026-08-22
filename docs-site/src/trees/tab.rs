//! `Tab`, as trees.
//!
//! One entry per example in `examples::TAB`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.
//!
//! The panels come from `super::tabs`, where the helper these examples shared
//! with it was hoisted to.

use noctua_design_i18n::Key;

use crate::tree::{Each, Node, Value};
use crate::trees::Tree;
use crate::trees::tabs::PANELS;

/// Every `Tab` example. 4 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r"Tab { name, value, label, selected: true }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[
                Node::Component {
                    name: "Tabs",
                    props: &[
                        ("name", Value::Text("tab-plain")),
                        ("label", Value::Message(Key::ExampleTabsLabel)),
                    ],
                    children: &[
                        Node::Component {
                            name: "Tab",
                            props: &[
                                ("name", Value::Text("tab-plain")),
                                ("value", Value::Text("items")),
                                ("label", Value::Message(Key::ExampleTabSettings)),
                                ("selected", Value::Bool(true)),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "Tab",
                            props: &[
                                ("name", Value::Text("tab-plain")),
                                ("value", Value::Text("delivery")),
                                ("label", Value::Message(Key::ExampleTabPeople)),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "Tab",
                            props: &[
                                ("name", Value::Text("tab-plain")),
                                ("value", Value::Text("payment")),
                                ("label", Value::Message(Key::ExampleTabBilling)),
                            ],
                            children: &[],
                        },
                    ],
                },
                Node::Each {
                    over: Each::Rows(&[&[
                        Value::Text("tab-plain"),
                        Value::Message(Key::ExampleTabSettingsBody),
                        Value::Message(Key::ExampleTabPeopleBody),
                        Value::Message(Key::ExampleTabBillingBody),
                    ]]),
                    body: PANELS,
                },
            ],
        }],
    ),
    (
        r#"Tab { icon: "settings" }"#,
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[
                Node::Component {
                    name: "Tabs",
                    props: &[
                        ("name", Value::Text("tab-icon")),
                        ("label", Value::Message(Key::ExampleTabsLabel)),
                    ],
                    children: &[
                        Node::Component {
                            name: "Tab",
                            props: &[
                                ("name", Value::Text("tab-icon")),
                                ("value", Value::Text("items")),
                                ("label", Value::Message(Key::ExampleTabSettings)),
                                ("icon", Value::Text("settings")),
                                ("selected", Value::Bool(true)),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "Tab",
                            props: &[
                                ("name", Value::Text("tab-icon")),
                                ("value", Value::Text("delivery")),
                                ("label", Value::Message(Key::ExampleTabPeople)),
                                ("icon", Value::Text("user")),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "Tab",
                            props: &[
                                ("name", Value::Text("tab-icon")),
                                ("value", Value::Text("payment")),
                                ("label", Value::Message(Key::ExampleTabBilling)),
                                ("icon", Value::Text("download")),
                            ],
                            children: &[],
                        },
                    ],
                },
                Node::Each {
                    over: Each::Rows(&[&[
                        Value::Text("tab-icon"),
                        Value::Message(Key::ExampleTabSettingsBody),
                        Value::Message(Key::ExampleTabPeopleBody),
                        Value::Message(Key::ExampleTabBillingBody),
                    ]]),
                    body: PANELS,
                },
            ],
        }],
    ),
    (
        r#"Tab { count: "4" }"#,
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[
                Node::Component {
                    name: "Tabs",
                    props: &[
                        ("name", Value::Text("tab-count")),
                        ("label", Value::Message(Key::ExampleTabsLabel)),
                    ],
                    children: &[
                        Node::Component {
                            name: "Tab",
                            props: &[
                                ("name", Value::Text("tab-count")),
                                ("value", Value::Text("items")),
                                ("label", Value::Message(Key::ExampleTabSettings)),
                                ("selected", Value::Bool(true)),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "Tab",
                            props: &[
                                ("name", Value::Text("tab-count")),
                                ("value", Value::Text("delivery")),
                                ("label", Value::Message(Key::ExampleTabPeople)),
                                ("count", Value::Text("4")),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "Tab",
                            props: &[
                                ("name", Value::Text("tab-count")),
                                ("value", Value::Text("payment")),
                                ("label", Value::Message(Key::ExampleTabBilling)),
                                ("count", Value::Text("0")),
                            ],
                            children: &[],
                        },
                    ],
                },
                Node::Each {
                    over: Each::Rows(&[&[
                        Value::Text("tab-count"),
                        Value::Message(Key::ExampleTabSettingsBody),
                        Value::Message(Key::ExampleTabPeopleBody),
                        Value::Message(Key::ExampleTabBillingBody),
                    ]]),
                    body: PANELS,
                },
            ],
        }],
    ),
    (
        r"Tab { disabled: true }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[
                Node::Component {
                    name: "Tabs",
                    props: &[
                        ("name", Value::Text("tab-off")),
                        ("label", Value::Message(Key::ExampleTabsLabel)),
                    ],
                    children: &[
                        Node::Component {
                            name: "Tab",
                            props: &[
                                ("name", Value::Text("tab-off")),
                                ("value", Value::Text("items")),
                                ("label", Value::Message(Key::ExampleTabSettings)),
                                ("selected", Value::Bool(true)),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "Tab",
                            props: &[
                                ("name", Value::Text("tab-off")),
                                ("value", Value::Text("delivery")),
                                ("label", Value::Message(Key::ExampleTabPeople)),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "Tab",
                            props: &[
                                ("name", Value::Text("tab-off")),
                                ("value", Value::Text("payment")),
                                ("label", Value::Message(Key::ExampleTabBilling)),
                                ("disabled", Value::Bool(true)),
                            ],
                            children: &[],
                        },
                    ],
                },
                Node::Each {
                    over: Each::Rows(&[&[
                        Value::Text("tab-off"),
                        Value::Message(Key::ExampleTabSettingsBody),
                        Value::Message(Key::ExampleTabPeopleBody),
                        Value::Message(Key::ExampleTabBillingBody),
                    ]]),
                    body: PANELS,
                },
            ],
        }],
    ),
];
