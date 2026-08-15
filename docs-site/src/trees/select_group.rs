//! `SelectGroup`, as trees.
//!
//! One entry per example in `examples::SELECT_GROUP`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_tree_belongs_to_its_example` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;
use crate::trees::select::{AMBER, AZURE, CLAY, INDIGO, OCHRE, TEAL};

/// Every `SelectGroup` example. 4 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"SelectGroup { label: "…" }"#,
        &[Node::Component {
            name: "Select",
            props: &[
                ("name", Value::Text("group-one")),
                ("label", Value::Message(Key::ExampleSelectPaletteLabel)),
                (
                    "placeholder",
                    Value::Message(Key::ExampleSelectPalettePlaceholder),
                ),
            ],
            children: &[Node::Component {
                name: "SelectGroup",
                props: &[("label", Value::Message(Key::ExampleSelectWarm))],
                children: &[Node::Fragment(OCHRE), Node::Fragment(AMBER)],
            }],
        }],
    ),
    (
        r"Two runs, one list",
        &[Node::Component {
            name: "Select",
            props: &[
                ("name", Value::Text("group-two")),
                ("label", Value::Message(Key::ExampleSelectPaletteLabel)),
                (
                    "placeholder",
                    Value::Message(Key::ExampleSelectPalettePlaceholder),
                ),
            ],
            children: &[
                Node::Component {
                    name: "SelectGroup",
                    props: &[("label", Value::Message(Key::ExampleSelectWarm))],
                    children: &[Node::Fragment(OCHRE), Node::Fragment(CLAY)],
                },
                Node::Component {
                    name: "SelectGroup",
                    props: &[("label", Value::Message(Key::ExampleSelectCool))],
                    children: &[Node::Fragment(TEAL), Node::Fragment(INDIGO)],
                },
            ],
        }],
    ),
    (
        r"SelectGroup { disabled: true, … }",
        &[Node::Component {
            name: "Select",
            props: &[
                ("name", Value::Text("group-refused")),
                ("label", Value::Message(Key::ExampleSelectPaletteLabel)),
                (
                    "placeholder",
                    Value::Message(Key::ExampleSelectPalettePlaceholder),
                ),
            ],
            children: &[
                Node::Component {
                    name: "SelectGroup",
                    props: &[("label", Value::Message(Key::ExampleSelectWarm))],
                    children: &[Node::Fragment(OCHRE)],
                },
                Node::Component {
                    name: "SelectGroup",
                    props: &[
                        ("label", Value::Message(Key::ExampleSelectCool)),
                        ("disabled", Value::Bool(true)),
                    ],
                    children: &[Node::Fragment(TEAL), Node::Fragment(AZURE)],
                },
            ],
        }],
    ),
    (
        r"A run beside ungrouped options",
        &[Node::Component {
            name: "Select",
            props: &[
                ("name", Value::Text("group-mixed")),
                ("label", Value::Message(Key::ExampleSelectPaletteLabel)),
                (
                    "placeholder",
                    Value::Message(Key::ExampleSelectPalettePlaceholder),
                ),
            ],
            children: &[
                Node::Fragment(OCHRE),
                Node::Component {
                    name: "SelectGroup",
                    props: &[("label", Value::Message(Key::ExampleSelectCool))],
                    children: &[Node::Fragment(TEAL), Node::Fragment(AZURE)],
                },
            ],
        }],
    ),
];
