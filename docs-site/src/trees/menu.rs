//! `Menu`, as trees.
//!
//! One entry per example in `examples::MENU`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Each, Node, Value};
use crate::trees::Tree;

/// The button that opens a menu, drawn by seven examples across two files.
///
/// A macro rather than a `const` fragment, because the id the button controls
/// is the one thing that differs from example to example, and a
/// `&'static [Node]` takes no argument. `examples.rs` carried the same macro
/// for the same reason: seven written-out copies of one button are seven
/// copies that stop agreeing.
macro_rules! menu_button {
    ($controls:literal) => {
        crate::tree::Node::Component {
            name: "Button",
            props: &[
                ("menu", crate::tree::Value::Bool(true)),
                ("controls", crate::tree::Value::Text($controls)),
                ("variant", crate::tree::Value::Text("outline")),
                ("tone", crate::tree::Value::Text("neutral")),
                (
                    "label",
                    crate::tree::Value::Message(noctua_design_i18n::Key::ExampleMenuOpen),
                ),
            ],
            children: &[],
        }
    };
}

pub(crate) use menu_button;

/// Every `Menu` example. 4 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"Menu { name, label } under a Button { menu: true, controls: "…" }"#,
        &[Node::Element {
            tag: "div",
            attributes: &[],
            children: &[
                menu_button!("example-menu-actions"),
                Node::Component {
                    name: "Menu",
                    props: &[
                        ("name", Value::Text("example-menu-actions")),
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
                        Node::Component {
                            name: "MenuItem",
                            props: &[("label", Value::Message(Key::ExampleMenuArchive))],
                            children: &[],
                        },
                    ],
                },
            ],
        }],
    ),
    (
        r"Menu { placement: MenuPlacement::End }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-end"))],
            children: &[
                menu_button!("example-menu-end"),
                Node::Component {
                    name: "Menu",
                    props: &[
                        ("name", Value::Text("example-menu-end")),
                        ("label", Value::Message(Key::ExampleMenuLabel)),
                        ("placement", Value::Text("end")),
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
    // A MENU LONG ENOUGH FOR THE WALK TO BE WORTH ANYTHING. This example
    // used to be example 1 with a fourth item, under a caption promising
    // *every key a menu owes* and markup showing none of them. Thirteen
    // items with thirteen different first letters is where Home, End and
    // the letter jump stop being claims and start being visible.
    (
        r"Menu { name, label } with thirteen MenuItem { label }",
        &[Node::Element {
            tag: "div",
            attributes: &[],
            children: &[
                Node::Component {
                    name: "Button",
                    props: &[
                        ("menu", Value::Bool(true)),
                        ("controls", Value::Text("example-menu-keys")),
                        ("variant", Value::Text("outline")),
                        ("tone", Value::Text("neutral")),
                        ("label", Value::Message(Key::ExampleMenuAccentOpen)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Menu",
                    props: &[
                        ("name", Value::Text("example-menu-keys")),
                        ("label", Value::Message(Key::ExampleMenuAccentLabel)),
                    ],
                    children: &[Node::Each {
                        over: Each::Rows(&[
                            &[Value::Message(Key::ThemeAccentOchre)],
                            &[Value::Message(Key::ThemeAccentAmber)],
                            &[Value::Message(Key::ThemeAccentLime)],
                            &[Value::Message(Key::ThemeAccentJade)],
                            &[Value::Message(Key::ThemeAccentTeal)],
                            &[Value::Message(Key::ThemeAccentAzure)],
                            &[Value::Message(Key::ThemeAccentBlue)],
                            &[Value::Message(Key::ThemeAccentIndigo)],
                            &[Value::Message(Key::ThemeAccentViolet)],
                            &[Value::Message(Key::ThemeAccentMagenta)],
                            &[Value::Message(Key::ThemeAccentRose)],
                            &[Value::Message(Key::ThemeAccentClay)],
                            &[Value::Message(Key::ThemeAccentUmber)],
                        ]),
                        body: &[Node::Component {
                            name: "MenuItem",
                            props: &[("label", Value::Bound(0))],
                            children: &[],
                        }],
                    }],
                },
            ],
        }],
    ),
    (
        r"Menu, twice on one page",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-row"))],
            children: &[
                menu_button!("example-menu-first"),
                Node::Component {
                    name: "Menu",
                    props: &[
                        ("name", Value::Text("example-menu-first")),
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
                menu_button!("example-menu-second"),
                Node::Component {
                    name: "Menu",
                    props: &[
                        ("name", Value::Text("example-menu-second")),
                        ("label", Value::Message(Key::ExampleMenuLabel)),
                    ],
                    children: &[
                        Node::Component {
                            name: "MenuItem",
                            props: &[("label", Value::Message(Key::ExampleMenuArchive))],
                            children: &[],
                        },
                        Node::Component {
                            name: "MenuItem",
                            props: &[("label", Value::Message(Key::ExampleMenuDelete))],
                            children: &[],
                        },
                    ],
                },
            ],
        }],
    ),
];
