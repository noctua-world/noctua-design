//! `ToolbarGroup`, as trees.
//!
//! One entry per example in `examples::TOOLBAR_GROUP`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;
use crate::trees::toolbar::CONTROLS;

/// Every `ToolbarGroup` example. 4 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r"ToolbarGroup { … }
ToolbarGroup { align: ToolbarGroupAlign::End, … }",
        // NO `align: Start` ON THE FIRST GROUP. `start` is the default, so
        // writing it showed the prop doing nothing while the caption
        // claimed `End`. The value that does the work is on the group it
        // moves.
        &[Node::Component {
            name: "Toolbar",
            props: &[("label", Value::Message(Key::ExampleToolbarLabel))],
            children: &[
                Node::Component {
                    name: "ToolbarGroup",
                    props: &[],
                    children: &[Node::Fragment(CONTROLS)],
                },
                Node::Component {
                    name: "ToolbarGroup",
                    props: &[("align", Value::Text("end"))],
                    children: &[Node::Fragment(CONTROLS)],
                },
            ],
        }],
    ),
    (
        r"ToolbarGroup { … }",
        &[Node::Component {
            name: "Toolbar",
            props: &[("label", Value::Message(Key::ExampleToolbarLabel))],
            children: &[Node::Component {
                name: "ToolbarGroup",
                props: &[],
                children: &[Node::Fragment(CONTROLS)],
            }],
        }],
    ),
    (
        r#"ToolbarGroup { label: "…", … }"#,
        &[Node::Component {
            name: "Toolbar",
            props: &[("label", Value::Message(Key::ExampleToolbarLabel))],
            children: &[Node::Component {
                name: "ToolbarGroup",
                props: &[("label", Value::Message(Key::ExampleToolbarView))],
                children: &[Node::Fragment(CONTROLS)],
            }],
        }],
    ),
    // THREE GROUPS, AND ONE `align` PLACES TWO OF THEM. Example 1 is two
    // groups at the two ends and this used to be the identical arrangement
    // with different buttons in it. The claim worth its own example is the
    // rest of the sentence in the contract: `end` moves this group **and
    // everything after it**.
    (
        r"ToolbarGroup { … } × 3, with align: ToolbarGroupAlign::End on the second",
        &[Node::Component {
            name: "Toolbar",
            props: &[("label", Value::Message(Key::ExampleToolbarLabel))],
            children: &[
                Node::Component {
                    name: "ToolbarGroup",
                    props: &[],
                    children: &[Node::Component {
                        name: "Button",
                        props: &[
                            ("variant", Value::Text("outline")),
                            ("tone", Value::Text("neutral")),
                            ("size", Value::Text("small")),
                            ("label", Value::Message(Key::ExampleGroupPrevious)),
                        ],
                        children: &[],
                    }],
                },
                Node::Component {
                    name: "ToolbarGroup",
                    props: &[("align", Value::Text("end"))],
                    children: &[Node::Component {
                        name: "Button",
                        props: &[
                            ("variant", Value::Text("outline")),
                            ("tone", Value::Text("neutral")),
                            ("size", Value::Text("small")),
                            ("label", Value::Message(Key::ExampleGroupNext)),
                        ],
                        children: &[],
                    }],
                },
                Node::Component {
                    name: "ToolbarGroup",
                    props: &[],
                    children: &[Node::Fragment(CONTROLS)],
                },
            ],
        }],
    ),
];
