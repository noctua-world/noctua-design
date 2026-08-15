//! `Tabs`, as trees.
//!
//! One entry per example in `examples::TABS`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_tree_belongs_to_its_example` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Each, Node, Value};
use crate::trees::Tree;

/// One strip's panels, built once so eight examples differ by one prop.
///
/// A fragment rather than eight copies: what each example is about is the prop
/// it passes, and a reader comparing them should not have to diff three panels
/// to find it. `examples::tab_set` made the same argument as a function, and
/// this is that function with its arguments carried by a one-row walk — column
/// 0 is the set's name, and one column per panel body after it.
pub(crate) const PANELS: &[Node] = &[
    Node::Component {
        name: "TabPanel",
        props: &[("name", Value::Bound(0)), ("value", Value::Text("items"))],
        children: &[Node::Component {
            name: "Paragraph",
            props: &[],
            children: &[Node::Text(Value::Bound(1))],
        }],
    },
    Node::Component {
        name: "TabPanel",
        props: &[
            ("name", Value::Bound(0)),
            ("value", Value::Text("delivery")),
        ],
        children: &[Node::Component {
            name: "Paragraph",
            props: &[],
            children: &[Node::Text(Value::Bound(2))],
        }],
    },
    Node::Component {
        name: "TabPanel",
        props: &[("name", Value::Bound(0)), ("value", Value::Text("payment"))],
        children: &[Node::Component {
            name: "Paragraph",
            props: &[],
            children: &[Node::Text(Value::Bound(3))],
        }],
    },
];

/// Every `Tabs` example. 4 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    // ONE STRIP, THREE PANELS. With no script every panel is on the page and
    // each tab is a link down to the one it names.
    (
        r"Tabs { name, label } / Tab { selected: true } / TabPanel { … }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[
                Node::Component {
                    name: "Tabs",
                    props: &[
                        ("name", Value::Text("tabs-plain")),
                        ("label", Value::Message(Key::ExampleTabsLabel)),
                    ],
                    children: &[
                        Node::Component {
                            name: "Tab",
                            props: &[
                                ("name", Value::Text("tabs-plain")),
                                ("value", Value::Text("items")),
                                ("label", Value::Message(Key::ExampleTabsItems)),
                                ("selected", Value::Bool(true)),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "Tab",
                            props: &[
                                ("name", Value::Text("tabs-plain")),
                                ("value", Value::Text("delivery")),
                                ("label", Value::Message(Key::ExampleTabsDelivery)),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "Tab",
                            props: &[
                                ("name", Value::Text("tabs-plain")),
                                ("value", Value::Text("payment")),
                                ("label", Value::Message(Key::ExampleTabsPayment)),
                            ],
                            children: &[],
                        },
                    ],
                },
                Node::Each {
                    over: Each::Rows(&[&[
                        Value::Text("tabs-plain"),
                        Value::Message(Key::ExampleTabsItemsBody),
                        Value::Message(Key::ExampleTabsDeliveryBody),
                        Value::Message(Key::ExampleTabsPaymentBody),
                    ]]),
                    body: PANELS,
                },
            ],
        }],
    ),
    // MANUAL ACTIVATION, which is the one place a tab differs from a radio:
    // the arrows move the focus and wait to be told.
    (
        r"Tabs { activation: Manual, appearance: Enclosed, labelledby }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[
                Node::Component {
                    name: "Label",
                    props: &[
                        ("kind", Value::Text("name")),
                        ("text", Value::Message(Key::ExampleTabsLabel)),
                        ("for_control", Value::Text("tabs-manual")),
                    ],
                    children: &[],
                },
                Node::Element {
                    tag: "div",
                    attributes: &[("class", Value::Text("nd-example-stack"))],
                    children: &[
                        Node::Component {
                            name: "Tabs",
                            props: &[
                                ("name", Value::Text("tabs-manual")),
                                ("labelledby", Value::Text("tabs-manual-label")),
                                ("activation", Value::Text("manual")),
                                ("appearance", Value::Text("enclosed")),
                            ],
                            children: &[
                                Node::Component {
                                    name: "Tab",
                                    props: &[
                                        ("name", Value::Text("tabs-manual")),
                                        ("value", Value::Text("items")),
                                        ("label", Value::Message(Key::ExampleTabsItems)),
                                        ("selected", Value::Bool(true)),
                                    ],
                                    children: &[],
                                },
                                Node::Component {
                                    name: "Tab",
                                    props: &[
                                        ("name", Value::Text("tabs-manual")),
                                        ("value", Value::Text("delivery")),
                                        ("label", Value::Message(Key::ExampleTabsDelivery)),
                                    ],
                                    children: &[],
                                },
                                Node::Component {
                                    name: "Tab",
                                    props: &[
                                        ("name", Value::Text("tabs-manual")),
                                        ("value", Value::Text("payment")),
                                        ("label", Value::Message(Key::ExampleTabsPayment)),
                                    ],
                                    children: &[],
                                },
                            ],
                        },
                        Node::Each {
                            over: Each::Rows(&[&[
                                Value::Text("tabs-manual"),
                                Value::Message(Key::ExampleTabsItemsBody),
                                Value::Message(Key::ExampleTabsDeliveryBody),
                                Value::Message(Key::ExampleTabsPaymentBody),
                            ]]),
                            body: PANELS,
                        },
                    ],
                },
            ],
        }],
    ),
    (
        r"Tabs { orientation: Vertical, size: Small }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[
                Node::Component {
                    name: "Tabs",
                    props: &[
                        ("name", Value::Text("tabs-upright")),
                        ("label", Value::Message(Key::ExampleTabsLabel)),
                        ("orientation", Value::Text("vertical")),
                        ("size", Value::Text("small")),
                    ],
                    children: &[
                        Node::Component {
                            name: "Tab",
                            props: &[
                                ("name", Value::Text("tabs-upright")),
                                ("value", Value::Text("items")),
                                ("label", Value::Message(Key::ExampleTabsItems)),
                                ("selected", Value::Bool(true)),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "Tab",
                            props: &[
                                ("name", Value::Text("tabs-upright")),
                                ("value", Value::Text("delivery")),
                                ("label", Value::Message(Key::ExampleTabsDelivery)),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "Tab",
                            props: &[
                                ("name", Value::Text("tabs-upright")),
                                ("value", Value::Text("payment")),
                                ("label", Value::Message(Key::ExampleTabsPayment)),
                            ],
                            children: &[],
                        },
                    ],
                },
                Node::Each {
                    over: Each::Rows(&[&[
                        Value::Text("tabs-upright"),
                        Value::Message(Key::ExampleTabsItemsBody),
                        Value::Message(Key::ExampleTabsDeliveryBody),
                        Value::Message(Key::ExampleTabsPaymentBody),
                    ]]),
                    body: PANELS,
                },
            ],
        }],
    ),
    (
        r"Tabs { fill: Equal }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[
                Node::Component {
                    name: "Tabs",
                    props: &[
                        ("name", Value::Text("tabs-equal")),
                        ("label", Value::Message(Key::ExampleTabsLabel)),
                        ("fill", Value::Text("equal")),
                    ],
                    children: &[
                        Node::Component {
                            name: "Tab",
                            props: &[
                                ("name", Value::Text("tabs-equal")),
                                ("value", Value::Text("items")),
                                ("label", Value::Message(Key::ExampleTabsItems)),
                                ("selected", Value::Bool(true)),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "Tab",
                            props: &[
                                ("name", Value::Text("tabs-equal")),
                                ("value", Value::Text("delivery")),
                                ("label", Value::Message(Key::ExampleTabsDelivery)),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "Tab",
                            props: &[
                                ("name", Value::Text("tabs-equal")),
                                ("value", Value::Text("payment")),
                                ("label", Value::Message(Key::ExampleTabsPayment)),
                            ],
                            children: &[],
                        },
                    ],
                },
                Node::Each {
                    over: Each::Rows(&[&[
                        Value::Text("tabs-equal"),
                        Value::Message(Key::ExampleTabsItemsBody),
                        Value::Message(Key::ExampleTabsDeliveryBody),
                        Value::Message(Key::ExampleTabsPaymentBody),
                    ]]),
                    body: PANELS,
                },
            ],
        }],
    ),
];
