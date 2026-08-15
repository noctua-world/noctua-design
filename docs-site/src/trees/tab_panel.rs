//! `TabPanel`, as trees.
//!
//! One entry per example in `examples::TAB_PANEL`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_tree_belongs_to_its_example` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Each, Node, Value};
use crate::trees::Tree;

/// Every `TabPanel` example. 4 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r"TabPanel { name, value, … } after the strip",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[
                Node::Component {
                    name: "Tabs",
                    props: &[
                        ("name", Value::Text("panel-plain")),
                        ("label", Value::Message(Key::ExampleTabsLabel)),
                    ],
                    children: &[
                        Node::Component {
                            name: "Tab",
                            props: &[
                                ("name", Value::Text("panel-plain")),
                                ("value", Value::Text("summary")),
                                ("label", Value::Message(Key::ExampleTabPanelSummary)),
                                ("selected", Value::Bool(true)),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "Tab",
                            props: &[
                                ("name", Value::Text("panel-plain")),
                                ("value", Value::Text("detail")),
                                ("label", Value::Message(Key::ExampleTabPanelDetail)),
                            ],
                            children: &[],
                        },
                    ],
                },
                Node::Component {
                    name: "TabPanel",
                    props: &[
                        ("name", Value::Text("panel-plain")),
                        ("value", Value::Text("summary")),
                    ],
                    children: &[Node::Component {
                        name: "Paragraph",
                        props: &[],
                        children: &[Node::Text(Value::Message(Key::ExampleTabPanelSummaryBody))],
                    }],
                },
                Node::Component {
                    name: "TabPanel",
                    props: &[
                        ("name", Value::Text("panel-plain")),
                        ("value", Value::Text("detail")),
                    ],
                    children: &[Node::Component {
                        name: "Paragraph",
                        props: &[],
                        children: &[Node::Text(Value::Message(Key::ExampleTabPanelDetailBody))],
                    }],
                },
            ],
        }],
    ),
    (
        r"TabPanel { … } holding more than a sentence",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[
                Node::Component {
                    name: "Tabs",
                    props: &[
                        ("name", Value::Text("panel-rich")),
                        ("label", Value::Message(Key::ExampleTabsLabel)),
                    ],
                    children: &[
                        Node::Component {
                            name: "Tab",
                            props: &[
                                ("name", Value::Text("panel-rich")),
                                ("value", Value::Text("summary")),
                                ("label", Value::Message(Key::ExampleTabPanelSummary)),
                                ("selected", Value::Bool(true)),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "Tab",
                            props: &[
                                ("name", Value::Text("panel-rich")),
                                ("value", Value::Text("detail")),
                                ("label", Value::Message(Key::ExampleTabPanelDetail)),
                            ],
                            children: &[],
                        },
                    ],
                },
                Node::Component {
                    name: "TabPanel",
                    props: &[
                        ("name", Value::Text("panel-rich")),
                        ("value", Value::Text("summary")),
                    ],
                    children: &[Node::Component {
                        name: "List",
                        props: &[],
                        children: &[
                            Node::Component {
                                name: "ListItem",
                                props: &[],
                                children: &[Node::Text(Value::Message(Key::ExampleTabsItemsBody))],
                            },
                            Node::Component {
                                name: "ListItem",
                                props: &[],
                                children: &[Node::Text(Value::Message(
                                    Key::ExampleTabsDeliveryBody,
                                ))],
                            },
                            Node::Component {
                                name: "ListItem",
                                props: &[],
                                children: &[Node::Text(Value::Message(
                                    Key::ExampleTabsPaymentBody,
                                ))],
                            },
                        ],
                    }],
                },
                Node::Component {
                    name: "TabPanel",
                    props: &[
                        ("name", Value::Text("panel-rich")),
                        ("value", Value::Text("detail")),
                    ],
                    children: &[Node::Component {
                        name: "Paragraph",
                        props: &[],
                        children: &[Node::Text(Value::Message(Key::ExampleTabPanelDetailBody))],
                    }],
                },
            ],
        }],
    ),
    // THE PANEL SHOWN IS THE ONE THE `value` NAMES, not the first one written.
    // This example used to be example 1 again with a different `name`, under a
    // caption claiming every panel is visible with no script — which the
    // rendered page, where the script runs, disproves four inches below the
    // sentence.
    (
        r#"Tab { value: "delivery", selected: true } / TabPanel { name, value } three times"#,
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[
                Node::Component {
                    name: "Tabs",
                    props: &[
                        ("name", Value::Text("panel-chosen")),
                        ("label", Value::Message(Key::ExampleTabsLabel)),
                    ],
                    children: &[Node::Each {
                        over: Each::Rows(&[
                            &[
                                Value::Text("items"),
                                Value::Message(Key::ExampleTabsItems),
                                Value::Bool(false),
                            ],
                            &[
                                Value::Text("delivery"),
                                Value::Message(Key::ExampleTabsDelivery),
                                Value::Bool(true),
                            ],
                            &[
                                Value::Text("payment"),
                                Value::Message(Key::ExampleTabsPayment),
                                Value::Bool(false),
                            ],
                        ]),
                        body: &[Node::Component {
                            name: "Tab",
                            props: &[
                                ("name", Value::Text("panel-chosen")),
                                ("value", Value::Bound(0)),
                                ("label", Value::Bound(1)),
                                ("selected", Value::Bound(2)),
                            ],
                            children: &[],
                        }],
                    }],
                },
                Node::Each {
                    over: Each::Rows(&[
                        &[
                            Value::Text("items"),
                            Value::Message(Key::ExampleTabsItemsBody),
                        ],
                        &[
                            Value::Text("delivery"),
                            Value::Message(Key::ExampleTabsDeliveryBody),
                        ],
                        &[
                            Value::Text("payment"),
                            Value::Message(Key::ExampleTabsPaymentBody),
                        ],
                    ]),
                    body: &[Node::Component {
                        name: "TabPanel",
                        props: &[
                            ("name", Value::Text("panel-chosen")),
                            ("value", Value::Bound(0)),
                        ],
                        children: &[Node::Component {
                            name: "Paragraph",
                            props: &[],
                            children: &[Node::Text(Value::Bound(1))],
                        }],
                    }],
                },
            ],
        }],
    ),
    // A SET INSIDE A PANEL, which works because both ids come from a `name`
    // and this one differs from the set around it.
    (
        r"TabPanel { … Tabs { name: … } }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[
                Node::Component {
                    name: "Tabs",
                    props: &[
                        ("name", Value::Text("panel-outer")),
                        ("label", Value::Message(Key::ExampleTabsLabel)),
                    ],
                    children: &[Node::Component {
                        name: "Tab",
                        props: &[
                            ("name", Value::Text("panel-outer")),
                            ("value", Value::Text("summary")),
                            ("label", Value::Message(Key::ExampleTabPanelSummary)),
                            ("selected", Value::Bool(true)),
                        ],
                        children: &[],
                    }],
                },
                Node::Component {
                    name: "TabPanel",
                    props: &[
                        ("name", Value::Text("panel-outer")),
                        ("value", Value::Text("summary")),
                    ],
                    children: &[
                        Node::Component {
                            name: "Tabs",
                            props: &[
                                ("name", Value::Text("panel-inner")),
                                ("label", Value::Message(Key::ExampleTabsLabel)),
                            ],
                            children: &[
                                Node::Component {
                                    name: "Tab",
                                    props: &[
                                        ("name", Value::Text("panel-inner")),
                                        ("value", Value::Text("items")),
                                        ("label", Value::Message(Key::ExampleTabsItems)),
                                        ("selected", Value::Bool(true)),
                                    ],
                                    children: &[],
                                },
                                Node::Component {
                                    name: "Tab",
                                    props: &[
                                        ("name", Value::Text("panel-inner")),
                                        ("value", Value::Text("delivery")),
                                        ("label", Value::Message(Key::ExampleTabsDelivery)),
                                    ],
                                    children: &[],
                                },
                            ],
                        },
                        Node::Component {
                            name: "TabPanel",
                            props: &[
                                ("name", Value::Text("panel-inner")),
                                ("value", Value::Text("items")),
                            ],
                            children: &[Node::Component {
                                name: "Paragraph",
                                props: &[],
                                children: &[Node::Text(Value::Message(Key::ExampleTabsItemsBody))],
                            }],
                        },
                        Node::Component {
                            name: "TabPanel",
                            props: &[
                                ("name", Value::Text("panel-inner")),
                                ("value", Value::Text("delivery")),
                            ],
                            children: &[Node::Component {
                                name: "Paragraph",
                                props: &[],
                                children: &[Node::Text(Value::Message(
                                    Key::ExampleTabsDeliveryBody,
                                ))],
                            }],
                        },
                    ],
                },
            ],
        }],
    ),
];
