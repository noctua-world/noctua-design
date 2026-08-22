//! `AvatarGroup`, as trees.
//!
//! One entry per example in `examples::AVATAR_GROUP`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Each, Node, Value};
use crate::trees::Tree;

/// The people the avatar examples are about.
///
/// Real names with real initials: three people who have a picture, and one
/// seat nobody has taken yet. The pictures are portraits the docs build
/// already copies, so the URLs answer on a published page and
/// `theme.spec.js` has nothing to refuse. They used to be the site's own mark,
/// which is a ring around a dot: every face carrying it drew a second circle
/// inside the avatar's own.
///
/// ONE PERSON, ONE PORTRAIT, AND THAT IS THE RULE THE WHOLE PAGE FOLLOWS. Ana
/// is `ada`, Bruno is `bob`, Carla is `cara`, everywhere either of them
/// appears. Three of these four faces used to be the same file, so a group
/// whose whole subject is who is standing in front of whom was three copies of
/// one stranger — and a reader cannot tell a stack of identical faces apart,
/// which is what `overlap` and `shape` are for. Diego is the one person with no
/// portrait, deliberately: he appears only where the subject is `tone`, and a
/// picture there would cover the thing being shown.
///
/// EACH FACE ALSO CARRIES ITS OWN COLOUR, because a picture is a layer over
/// letters rather than a branch away from them. Block the files and this is
/// still four people rather than four identical grey discs. `pending` keeps
/// the default, because nobody has taken that place yet.
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
                ("src", Value::Asset("assets/portraits/ada.svg")),
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
                ("src", Value::Asset("assets/portraits/bob.svg")),
                ("tone", Value::Text("accent")),
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
                ("src", Value::Asset("assets/portraits/cara.svg")),
                ("tone", Value::Text("success")),
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
                            ("src", Value::Asset("assets/portraits/ada.svg")),
                        ],
                        children: &[],
                    }],
                },
                // Bruno's own face, not Ana's again. Two children of a group
                // are two people, and the shortest example on the page is the
                // one where a repeated portrait is hardest to miss.
                Node::Element {
                    tag: "li",
                    attributes: &[],
                    children: &[Node::Component {
                        name: "Avatar",
                        props: &[
                            ("label", Value::Message(Key::ExampleAvatarBruno)),
                            ("initials", Value::Data("BS")),
                            ("src", Value::Asset("assets/portraits/bob.svg")),
                            ("tone", Value::Text("accent")),
                        ],
                        children: &[],
                    }],
                },
            ],
        }],
    ),
];
