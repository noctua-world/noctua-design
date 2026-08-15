//! `AvatarGroup`, as trees.
//!
//! One entry per example in `examples::AVATAR_GROUP`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_tree_belongs_to_its_example` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Each, Node, Value};
use crate::trees::Tree;

/// The people the avatar examples are about.
///
/// Real names with real initials, and one of each shape the fallback takes: a
/// picture that loads, letters, and nothing at all. The pictures are the site's
/// own mark, because it is a file the build already writes — an example
/// pointing at a URL nothing answers would be a permanent 404 on a published
/// page, and `theme.spec.js` refuses those in writing.
///
/// One fragment rather than four copies, for the reason the macro it replaces
/// existed: three examples show the same group, and three copies stop agreeing.
const PEOPLE: &[Node] = &[
    Node::Element {
        tag: "li",
        attributes: &[],
        children: &[Node::Component {
            name: "Avatar",
            props: &[
                ("label", Value::Message(Key::ExampleAvatarAna)),
                ("initials", Value::Data("AL")),
                ("src", Value::Asset("favicon.svg")),
            ],
            children: &[],
        }],
    },
    Node::Element {
        tag: "li",
        attributes: &[],
        children: &[Node::Component {
            name: "Avatar",
            props: &[
                ("label", Value::Message(Key::ExampleAvatarBruno)),
                ("initials", Value::Data("BS")),
            ],
            children: &[],
        }],
    },
    Node::Element {
        tag: "li",
        attributes: &[],
        children: &[Node::Component {
            name: "Avatar",
            props: &[
                ("label", Value::Message(Key::ExampleAvatarCarla)),
                ("initials", Value::Data("CM")),
            ],
            children: &[],
        }],
    },
    Node::Element {
        tag: "li",
        attributes: &[],
        children: &[Node::Component {
            name: "Avatar",
            props: &[("label", Value::Message(Key::ExampleAvatarPending))],
            children: &[],
        }],
    },
];

/// Every `AvatarGroup` example. 4 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"AvatarGroup { label: "…", more: "+5", more_label: "…", more_href: "…" }"#,
        &[Node::Component {
            name: "AvatarGroup",
            props: &[
                ("label", Value::Message(Key::ExampleAvatarReviewers)),
                ("more", Value::Data("+5")),
                ("more_label", Value::Message(Key::ExampleAvatarMore)),
                ("more_href", Value::Text("../avatars/")),
            ],
            children: &[Node::Fragment(PEOPLE)],
        }],
    ),
    (
        r"AvatarGroup { overlap: AvatarGroupOverlap::… }",
        &[Node::Element {
            tag: "div",
            attributes: &[(
                "class",
                Value::Text("nd-example-stack nd-example-stack-wide"),
            )],
            children: &[Node::Each {
                over: Each::Values {
                    component: "AvatarGroup",
                    prop: "overlap",
                },
                body: &[Node::Component {
                    name: "AvatarGroup",
                    props: &[
                        ("overlap", Value::Bound(0)),
                        ("label", Value::Message(Key::ExampleAvatarReviewers)),
                    ],
                    children: &[Node::Fragment(PEOPLE)],
                }],
            }],
        }],
    ),
    (
        r"AvatarGroup { size: AvatarGroupSize::…, shape: AvatarGroupShape::… }",
        &[Node::Element {
            tag: "div",
            attributes: &[(
                "class",
                Value::Text("nd-example-stack nd-example-stack-wide"),
            )],
            children: &[
                Node::Each {
                    over: Each::Values {
                        component: "AvatarGroup",
                        prop: "size",
                    },
                    body: &[Node::Component {
                        name: "AvatarGroup",
                        props: &[
                            ("size", Value::Bound(0)),
                            ("label", Value::Message(Key::ExampleAvatarReviewers)),
                            ("more", Value::Data("+2")),
                            ("more_label", Value::Message(Key::ExampleAvatarMore)),
                        ],
                        children: &[Node::Fragment(PEOPLE)],
                    }],
                },
                Node::Each {
                    over: Each::Values {
                        component: "AvatarGroup",
                        prop: "shape",
                    },
                    body: &[Node::Component {
                        name: "AvatarGroup",
                        props: &[
                            ("shape", Value::Bound(0)),
                            ("size", Value::Text("small")),
                            ("label", Value::Message(Key::ExampleAvatarReviewers)),
                        ],
                        children: &[Node::Fragment(PEOPLE)],
                    }],
                },
            ],
        }],
    ),
    (
        r"AvatarGroup { children }",
        &[Node::Component {
            name: "AvatarGroup",
            props: &[("label", Value::Message(Key::ExampleAvatarReviewers))],
            children: &[
                Node::Element {
                    tag: "li",
                    attributes: &[],
                    children: &[Node::Component {
                        name: "Avatar",
                        props: &[
                            ("label", Value::Message(Key::ExampleAvatarAna)),
                            ("initials", Value::Data("AL")),
                        ],
                        children: &[],
                    }],
                },
                Node::Element {
                    tag: "li",
                    attributes: &[],
                    children: &[Node::Component {
                        name: "Avatar",
                        props: &[
                            ("label", Value::Message(Key::ExampleAvatarBruno)),
                            ("initials", Value::Data("BS")),
                        ],
                        children: &[],
                    }],
                },
            ],
        }],
    ),
];
