//! `Avatar`, as trees.
//!
//! One entry per example in `examples::AVATAR`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_tree_belongs_to_its_example` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Each, Node, Value};
use crate::trees::Tree;

/// Every `Avatar` example. 4 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"Avatar { label: "…", src: "…" } / { initials: "…" } / { icon: IconDrawing::MONITOR }"#,
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-row"))],
            children: &[
                Node::Component {
                    name: "Avatar",
                    props: &[
                        ("label", Value::Message(Key::ExampleAvatarAna)),
                        ("initials", Value::Data("AL")),
                        // A PORTRAIT, NOT THE SITE'S OWN MARK. The favicon is a
                        // ring around a dot, so `object-fit: cover` drew a
                        // second circle inside the avatar's own and the reader
                        // saw two rings where a face belongs. These are files
                        // the docs build already copies, so the URL still
                        // answers and the picture is a picture of somebody.
                        ("src", Value::Asset("assets/portraits/ada.svg")),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Avatar",
                    props: &[
                        ("label", Value::Message(Key::ExampleAvatarBruno)),
                        ("initials", Value::Data("BS")),
                    ],
                    children: &[],
                },
                // NOT `user`, WHICH IS WHAT ABSENCE ALREADY DRAWS. An example
                // setting a prop to its own default shows the prop doing
                // nothing. This account is a machine, so it gets a machine.
                Node::Component {
                    name: "Avatar",
                    props: &[
                        ("label", Value::Message(Key::ExampleAvatarDeployBot)),
                        ("icon", Value::Text("monitor")),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r"Avatar { size: AvatarSize::… } / Avatar { shape: AvatarShape::… }",
        &[
            Node::Element {
                tag: "div",
                attributes: &[("class", Value::Text("nd-example-row"))],
                children: &[Node::Each {
                    over: Each::Values {
                        component: "Avatar",
                        prop: "size",
                    },
                    body: &[Node::Component {
                        name: "Avatar",
                        props: &[
                            ("size", Value::Bound(0)),
                            ("initials", Value::Data("CM")),
                            ("label", Value::Message(Key::ExampleAvatarCarla)),
                        ],
                        children: &[],
                    }],
                }],
            },
            Node::Element {
                tag: "div",
                attributes: &[("class", Value::Text("nd-example-row"))],
                children: &[Node::Each {
                    over: Each::Values {
                        component: "Avatar",
                        prop: "shape",
                    },
                    body: &[Node::Component {
                        name: "Avatar",
                        props: &[
                            ("shape", Value::Bound(0)),
                            ("initials", Value::Data("CM")),
                            ("label", Value::Message(Key::ExampleAvatarCarla)),
                        ],
                        children: &[],
                    }],
                }],
            },
        ],
    ),
    (
        r"Avatar { tone: AvatarTone::… }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-row"))],
            // FOUR SETS OF INITIALS ARE FOUR PEOPLE, so each carries its
            // own name. One name over four faces makes three of them
            // announce somebody who is not there.
            children: &[Node::Each {
                over: Each::Rows(&[
                    &[
                        Value::Text("neutral"),
                        Value::Data("AL"),
                        Value::Message(Key::ExampleAvatarAna),
                    ],
                    &[
                        Value::Text("accent"),
                        Value::Data("BS"),
                        Value::Message(Key::ExampleAvatarBruno),
                    ],
                    &[
                        Value::Text("success"),
                        Value::Data("CM"),
                        Value::Message(Key::ExampleAvatarCarla),
                    ],
                    &[
                        Value::Text("danger"),
                        Value::Data("DR"),
                        Value::Message(Key::ExampleAvatarDiego),
                    ],
                ]),
                body: &[Node::Component {
                    name: "Avatar",
                    props: &[
                        ("tone", Value::Bound(0)),
                        ("initials", Value::Bound(1)),
                        ("label", Value::Bound(2)),
                    ],
                    children: &[],
                }],
            }],
        }],
    ),
    (
        r#"Avatar { href: "…" } / Avatar { decorative: true }"#,
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-row"))],
            children: &[
                // ONE PERSON, ONE PORTRAIT. Ana is `ada` and Bruno is `bob`
                // wherever either of them stands on this site, so two avatars
                // side by side are two people a reader can tell apart — which
                // is the whole claim an avatar makes.
                Node::Component {
                    name: "Avatar",
                    props: &[
                        ("href", Value::Text("../button/")),
                        ("initials", Value::Data("AL")),
                        ("src", Value::Asset("assets/portraits/ada.svg")),
                        ("label", Value::Message(Key::ExampleAvatarAna)),
                    ],
                    children: &[],
                },
                // Beside a name that is already on the screen, so the picture
                // is a picture of a word the reader has just heard.
                Node::Element {
                    tag: "span",
                    attributes: &[("class", Value::Text("nd-example-byline"))],
                    children: &[
                        Node::Component {
                            name: "Avatar",
                            props: &[
                                ("decorative", Value::Bool(true)),
                                ("initials", Value::Data("BS")),
                                ("src", Value::Asset("assets/portraits/bob.svg")),
                                ("label", Value::Message(Key::ExampleAvatarBruno)),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "Text",
                            props: &[],
                            children: &[Node::Text(Value::Message(Key::ExampleAvatarBruno))],
                        },
                    ],
                },
            ],
        }],
    ),
];
