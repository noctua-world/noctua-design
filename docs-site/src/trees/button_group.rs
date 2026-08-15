//! `ButtonGroup`, as trees.
//!
//! One entry per example in `examples::BUTTON_GROUP`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_tree_belongs_to_its_example` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Each, Node, Value};
use crate::trees::Tree;

/// Every `ButtonGroup` example. 5 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"ButtonGroup { label: "…", Button { … } }"#,
        &[Node::Component {
            name: "ButtonGroup",
            props: &[("label", Value::Message(Key::ExampleGroupLabel))],
            children: &[
                Node::Component {
                    name: "Button",
                    props: &[
                        ("variant", Value::Text("outline")),
                        ("tone", Value::Text("neutral")),
                        ("label", Value::Message(Key::ExampleGroupPrevious)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Button",
                    props: &[
                        ("variant", Value::Text("outline")),
                        ("tone", Value::Text("neutral")),
                        ("label", Value::Message(Key::ExampleGroupNext)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r"ButtonGroup { attached: false, … }",
        &[Node::Component {
            name: "ButtonGroup",
            props: &[
                ("label", Value::Message(Key::ExampleGroupLabel)),
                ("attached", Value::Bool(false)),
            ],
            children: &[
                Node::Component {
                    name: "Button",
                    props: &[("label", Value::Message(Key::ExampleButtonSave))],
                    children: &[],
                },
                Node::Component {
                    name: "Button",
                    props: &[
                        ("variant", Value::Text("transparent")),
                        ("tone", Value::Text("neutral")),
                        ("label", Value::Message(Key::ExampleButtonCancel)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r"ButtonGroup { orientation: ButtonGroupOrientation::Vertical, … }",
        &[Node::Component {
            name: "ButtonGroup",
            props: &[
                ("label", Value::Message(Key::ExampleGroupLabel)),
                ("orientation", Value::Text("vertical")),
            ],
            children: &[Node::Each {
                over: Each::Rows(&[
                    &[Value::Message(Key::ExampleButtonSave)],
                    &[Value::Message(Key::ExampleButtonPublish)],
                    &[Value::Message(Key::ExampleButtonCancel)],
                ]),
                body: &[Node::Component {
                    name: "Button",
                    props: &[
                        ("variant", Value::Text("outline")),
                        ("tone", Value::Text("neutral")),
                        ("justify", Value::Text("start")),
                        ("label", Value::Bound(0)),
                    ],
                    children: &[],
                }],
            }],
        }],
    ),
    (
        r"ButtonGroup { fill: true, … }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-narrow"))],
            children: &[Node::Component {
                name: "ButtonGroup",
                props: &[
                    ("label", Value::Message(Key::ExampleGroupLabel)),
                    ("fill", Value::Bool(true)),
                ],
                children: &[
                    Node::Component {
                        name: "Button",
                        props: &[
                            ("variant", Value::Text("outline")),
                            ("tone", Value::Text("neutral")),
                            ("label", Value::Message(Key::ExampleGroupPrevious)),
                        ],
                        children: &[],
                    },
                    Node::Component {
                        name: "Button",
                        props: &[
                            ("variant", Value::Text("outline")),
                            ("tone", Value::Text("neutral")),
                            ("label", Value::Message(Key::ExampleGroupNext)),
                        ],
                        children: &[],
                    },
                ],
            }],
        }],
    ),
    (
        r#"ButtonGroup { labelledby: "…", … } — a split button"#,
        &[Node::Element {
            tag: "div",
            attributes: &[],
            children: &[
                Node::Component {
                    name: "Label",
                    props: &[
                        ("kind", Value::Text("name")),
                        ("for_control", Value::Text("example-split")),
                        ("text", Value::Message(Key::ExampleGroupSplitLabel)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "ButtonGroup",
                    props: &[("labelledby", Value::Text("example-split-label"))],
                    children: &[
                        Node::Component {
                            name: "Button",
                            props: &[("label", Value::Message(Key::ExampleButtonPublish))],
                            children: &[],
                        },
                        Node::Component {
                            name: "Button",
                            props: &[
                                ("menu", Value::Bool(true)),
                                ("hide_label", Value::Bool(true)),
                                ("icon", Value::Text("chevron-down")),
                                ("controls", Value::Text("example-split-menu")),
                                ("label", Value::Message(Key::ExampleButtonMore)),
                            ],
                            children: &[],
                        },
                    ],
                },
                // A REAL `Menu`, AND IT USED TO BE A HIDDEN `<div>`. The
                // second half of a split button announced `aria-haspopup` at a
                // paragraph, so `menu.js` skipped it and the control opened
                // nothing — the exact defect that page documents, demonstrated
                // on the page documenting it.
                Node::Component {
                    name: "Menu",
                    props: &[
                        ("name", Value::Text("example-split-menu")),
                        ("label", Value::Message(Key::ExampleMenuLabel)),
                    ],
                    children: &[
                        Node::Component {
                            name: "MenuItem",
                            props: &[("label", Value::Message(Key::ExampleMenuDuplicate))],
                            children: &[],
                        },
                        Node::Component {
                            name: "MenuItem",
                            props: &[("label", Value::Message(Key::ExampleMenuRename))],
                            children: &[],
                        },
                    ],
                },
            ],
        }],
    ),
];
