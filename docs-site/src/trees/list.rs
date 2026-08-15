//! `List`, as trees.
//!
//! One entry per example in `examples::LIST`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_tree_belongs_to_its_example` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Every `List` example. 6 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    // NO `kind` IN THE CALL AND NONE IN THE CAPTION. `unordered` is the
    // default, so the caption named a prop the body never passed. The two
    // examples below pass the two values that change the element.
    (
        r"List { ListItem { … } × 3 }",
        &[Node::Component {
            name: "List",
            props: &[],
            children: &[
                Node::Component {
                    name: "ListItem",
                    props: &[],
                    children: &[Node::Text(Value::Message(Key::ExampleListOne))],
                },
                Node::Component {
                    name: "ListItem",
                    props: &[],
                    children: &[Node::Text(Value::Message(Key::ExampleListTwo))],
                },
                Node::Component {
                    name: "ListItem",
                    props: &[],
                    children: &[Node::Text(Value::Message(Key::ExampleListThree))],
                },
            ],
        }],
    ),
    (
        r"List { kind: ListKind::Ordered, … }",
        &[Node::Component {
            name: "List",
            props: &[("kind", Value::Text("ordered"))],
            children: &[
                Node::Component {
                    name: "ListItem",
                    props: &[],
                    children: &[Node::Text(Value::Message(Key::ExampleListOne))],
                },
                Node::Component {
                    name: "ListItem",
                    props: &[],
                    children: &[Node::Text(Value::Message(Key::ExampleListTwo))],
                },
                Node::Component {
                    name: "ListItem",
                    props: &[],
                    children: &[Node::Text(Value::Message(Key::ExampleListThree))],
                },
            ],
        }],
    ),
    (
        r"List { kind: ListKind::Description, … }",
        &[Node::Component {
            name: "List",
            props: &[("kind", Value::Text("description"))],
            children: &[
                Node::Component {
                    name: "ListItem",
                    props: &[("term", Value::Message(Key::ThemeAccentLabel))],
                    children: &[Node::Text(Value::Message(Key::ExampleTermAccent))],
                },
                Node::Component {
                    name: "ListItem",
                    props: &[("term", Value::Message(Key::ThemeGlassLabel))],
                    children: &[Node::Text(Value::Message(Key::ExampleTermGlass))],
                },
            ],
        }],
    ),
    (
        r"List { marker: ListMarker::None, … }",
        &[Node::Component {
            name: "List",
            props: &[("marker", Value::Text("none"))],
            children: &[
                Node::Component {
                    name: "ListItem",
                    props: &[],
                    children: &[Node::Text(Value::Message(Key::ExampleListOne))],
                },
                Node::Component {
                    name: "ListItem",
                    props: &[],
                    children: &[Node::Text(Value::Message(Key::ExampleListTwo))],
                },
            ],
        }],
    ),
    (
        r"List { spacing: ListSpacing::Loose, … }",
        &[Node::Component {
            name: "List",
            props: &[("spacing", Value::Text("loose"))],
            children: &[
                Node::Component {
                    name: "ListItem",
                    props: &[],
                    children: &[Node::Text(Value::Message(Key::ExampleListOne))],
                },
                Node::Component {
                    name: "ListItem",
                    props: &[],
                    children: &[Node::Text(Value::Message(Key::ExampleListTwo))],
                },
            ],
        }],
    ),
    (
        r"A list inside a list",
        &[Node::Component {
            name: "List",
            props: &[],
            children: &[Node::Component {
                name: "ListItem",
                props: &[],
                children: &[
                    Node::Text(Value::Message(Key::ExampleListOne)),
                    Node::Component {
                        name: "List",
                        props: &[("spacing", Value::Text("tight"))],
                        children: &[
                            Node::Component {
                                name: "ListItem",
                                props: &[],
                                children: &[Node::Text(Value::Message(Key::ExampleListTwo))],
                            },
                            Node::Component {
                                name: "ListItem",
                                props: &[],
                                children: &[Node::Text(Value::Message(Key::ExampleListThree))],
                            },
                        ],
                    },
                ],
            }],
        }],
    ),
];
