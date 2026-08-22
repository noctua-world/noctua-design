//! `List`, as trees.
//!
//! One entry per example in `examples::LIST`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Every `List` example. 7 of them.
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
    // FOUR LEVELS, AND THE KIND CHANGES AT EACH ONE. It used to be one list
    // inside another and it stopped there, which is the depth at which every
    // engine still agrees and therefore the depth that proves nothing. An
    // ordered list inside an unordered one inside an ordered one is where the
    // markers have to be read: a `<ul>` inside a `<ul>` is the browser's own
    // disc-then-circle, and a restarted `<ol>` counting from one under a
    // bullet is the case that looks wrong when the indent is wrong.
    (
        r"Ordered inside unordered inside ordered, four deep",
        &[Node::Component {
            name: "List",
            props: &[("kind", Value::Text("ordered"))],
            children: &[
                Node::Component {
                    name: "ListItem",
                    props: &[],
                    children: &[
                        Node::Text(Value::Message(Key::ExampleListOne)),
                        Node::Component {
                            name: "List",
                            props: &[],
                            children: &[
                                Node::Component {
                                    name: "ListItem",
                                    props: &[],
                                    children: &[
                                        Node::Text(Value::Message(Key::ExampleListTwo)),
                                        Node::Component {
                                            name: "List",
                                            props: &[("kind", Value::Text("ordered"))],
                                            children: &[
                                                Node::Component {
                                                    name: "ListItem",
                                                    props: &[],
                                                    children: &[
                                                        Node::Text(Value::Message(
                                                            Key::ExampleListThree,
                                                        )),
                                                        Node::Component {
                                                            name: "List",
                                                            props: &[],
                                                            children: &[
                                                                Node::Component {
                                                                    name: "ListItem",
                                                                    props: &[],
                                                                    children: &[Node::Text(
                                                                        Value::Message(
                                                                            Key::ExampleListOne,
                                                                        ),
                                                                    )],
                                                                },
                                                                Node::Component {
                                                                    name: "ListItem",
                                                                    props: &[],
                                                                    children: &[Node::Text(
                                                                        Value::Message(
                                                                            Key::ExampleListTwo,
                                                                        ),
                                                                    )],
                                                                },
                                                            ],
                                                        },
                                                    ],
                                                },
                                                Node::Component {
                                                    name: "ListItem",
                                                    props: &[],
                                                    children: &[Node::Text(Value::Message(
                                                        Key::ExampleListOne,
                                                    ))],
                                                },
                                            ],
                                        },
                                    ],
                                },
                                Node::Component {
                                    name: "ListItem",
                                    props: &[],
                                    children: &[Node::Text(Value::Message(Key::ExampleListThree))],
                                },
                            ],
                        },
                    ],
                },
                Node::Component {
                    name: "ListItem",
                    props: &[],
                    children: &[Node::Text(Value::Message(Key::ExampleListTwo))],
                },
            ],
        }],
    ),
    // THE FLAG IS ON THE OUTERMOST LIST AND ON NOTHING INSIDE IT. That is the
    // whole point of the prop: a caller writes it once and every list under it
    // takes the level the stylesheet counts for it.
    (
        r"List { levels: true } — a colour for each level",
        &[Node::Component {
            name: "List",
            props: &[("levels", Value::Bool(true))],
            children: &[
                Node::Component {
                    name: "ListItem",
                    props: &[],
                    children: &[
                        Node::Text(Value::Message(Key::ExampleListOne)),
                        Node::Component {
                            name: "List",
                            props: &[("kind", Value::Text("ordered"))],
                            children: &[Node::Component {
                                name: "ListItem",
                                props: &[],
                                children: &[
                                    Node::Text(Value::Message(Key::ExampleListTwo)),
                                    Node::Component {
                                        name: "List",
                                        props: &[],
                                        children: &[
                                            Node::Component {
                                                name: "ListItem",
                                                props: &[],
                                                children: &[
                                                    Node::Text(Value::Message(
                                                        Key::ExampleListThree,
                                                    )),
                                                    // The fourth level, and
                                                    // the one with no marker
                                                    // of its own: the rule
                                                    // down its edge is what
                                                    // says where it sits.
                                                    Node::Component {
                                                        name: "List",
                                                        props: &[("marker", Value::Text("none"))],
                                                        children: &[Node::Component {
                                                            name: "ListItem",
                                                            props: &[],
                                                            children: &[Node::Text(
                                                                Value::Message(Key::ExampleListOne),
                                                            )],
                                                        }],
                                                    },
                                                ],
                                            },
                                            Node::Component {
                                                name: "ListItem",
                                                props: &[],
                                                children: &[Node::Text(Value::Message(
                                                    Key::ExampleListOne,
                                                ))],
                                            },
                                        ],
                                    },
                                ],
                            }],
                        },
                    ],
                },
                Node::Component {
                    name: "ListItem",
                    props: &[],
                    children: &[Node::Text(Value::Message(Key::ExampleListTwo))],
                },
            ],
        }],
    ),
];
