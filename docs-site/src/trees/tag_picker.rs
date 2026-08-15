//! `TagPicker`, as trees.
//!
//! One entry per example in `examples::TAG_PICKER`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_tree_belongs_to_its_example` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Each, Node, Value};
use crate::trees::Tree;

/// The values six of these pickers offer, written once.
///
/// A `macro_rules!` in `examples.rs` before this, and a fragment here for the
/// same reason: six copies of one list is six lists that stop agreeing.
const TOPICS: &[Node] = &[
    Node::Component {
        name: "SelectGroup",
        props: &[("label", Value::Message(Key::ExamplePickerWriting))],
        children: &[
            Node::Component {
                name: "SelectOption",
                props: &[
                    ("class", Value::Text("nd-identifier")),
                    ("value", Value::Text("docs")),
                    ("label", Value::Data("docs")),
                ],
                children: &[],
            },
            Node::Component {
                name: "SelectOption",
                props: &[
                    ("class", Value::Text("nd-identifier")),
                    ("value", Value::Text("spec")),
                    ("label", Value::Data("spec")),
                ],
                children: &[],
            },
        ],
    },
    Node::Component {
        name: "SelectGroup",
        props: &[("label", Value::Message(Key::ExamplePickerBuilding))],
        children: &[
            Node::Component {
                name: "SelectOption",
                props: &[
                    ("class", Value::Text("nd-identifier")),
                    ("value", Value::Text("rust")),
                    ("label", Value::Data("rust")),
                ],
                children: &[],
            },
            Node::Component {
                name: "SelectOption",
                props: &[
                    ("class", Value::Text("nd-identifier")),
                    ("value", Value::Text("react")),
                    ("label", Value::Data("react")),
                ],
                children: &[],
            },
            Node::Component {
                name: "SelectOption",
                props: &[
                    ("class", Value::Text("nd-identifier")),
                    ("value", Value::Text("css")),
                    ("label", Value::Data("css")),
                ],
                children: &[],
            },
        ],
    },
];

/// Every `TagPicker` example. 5 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"TagPicker { name: "…", label: "…", hint: "…", placeholder: "…", status, create_label, empty_label, remove_label, added_label, removed_label }"#,
        &[Node::Component {
            name: "TagPicker",
            props: &[
                ("name", Value::Text("topics")),
                ("label", Value::Message(Key::ExamplePickerLabel)),
                ("placeholder", Value::Message(Key::ExamplePickerPlaceholder)),
                ("hint", Value::Message(Key::ExamplePickerHint)),
                ("status", Value::Message(Key::ExamplePickerStatus)),
                ("create_label", Value::Message(Key::ExamplePickerCreate)),
                ("empty_label", Value::Message(Key::ExamplePickerEmpty)),
                ("remove_label", Value::Message(Key::ExamplePickerRemove)),
                ("added_label", Value::Message(Key::ExamplePickerAdded)),
                ("removed_label", Value::Message(Key::ExamplePickerRemoved)),
            ],
            children: &[Node::Fragment(TOPICS)],
        }],
    ),
    (
        r#"TagPicker { creatable: false, separators: ",;", max: "3", max_label: "…" }"#,
        &[Node::Element {
            tag: "div",
            attributes: &[(
                "class",
                Value::Text("nd-example-stack nd-example-stack-wide"),
            )],
            children: &[
                Node::Component {
                    name: "TagPicker",
                    props: &[
                        ("name", Value::Text("from-the-list")),
                        ("creatable", Value::Bool(false)),
                        ("label", Value::Message(Key::ExamplePickerLabel)),
                        ("placeholder", Value::Message(Key::ExamplePickerPlaceholder)),
                        ("empty_label", Value::Message(Key::ExamplePickerEmpty)),
                        ("remove_label", Value::Message(Key::ExamplePickerRemove)),
                        ("status", Value::Message(Key::ExamplePickerStatus)),
                    ],
                    children: &[Node::Fragment(TOPICS)],
                },
                Node::Component {
                    name: "TagPicker",
                    props: &[
                        ("name", Value::Text("at-most-three")),
                        ("separators", Value::Text(",;")),
                        ("max", Value::Text("3")),
                        ("max_label", Value::Message(Key::ExamplePickerMax)),
                        ("label", Value::Message(Key::ExamplePickerLabel)),
                        ("placeholder", Value::Message(Key::ExamplePickerPlaceholder)),
                        ("create_label", Value::Message(Key::ExamplePickerCreate)),
                        ("remove_label", Value::Message(Key::ExamplePickerRemove)),
                        ("status", Value::Message(Key::ExamplePickerStatus)),
                    ],
                    children: &[Node::Fragment(TOPICS)],
                },
            ],
        }],
    ),
    (
        r#"TagPicker { pattern: "…", invalid_label: "…", clearable: true, clear_label: "…" }
TagPicker { paste: false, duplicates: true, … }"#,
        &[Node::Element {
            tag: "div",
            attributes: &[(
                "class",
                Value::Text("nd-example-stack nd-example-stack-wide"),
            )],
            children: &[
                // NO `paste: true` AND NO `duplicates: false` HERE. Those are
                // the defaults, so both showed the prop doing nothing, on the
                // one page a reader comes to to find out what they change.
                Node::Component {
                    name: "TagPicker",
                    props: &[
                        ("name", Value::Text("addresses")),
                        // The platform's own `pattern`, not an invention here.
                        ("pattern", Value::Text(r"[^@\s]+@[^@\s]+\.[^@\s]+")),
                        ("invalid_label", Value::Message(Key::ExamplePickerInvalid)),
                        ("clearable", Value::Bool(true)),
                        ("clear_label", Value::Message(Key::ExamplePickerClear)),
                        ("label", Value::Message(Key::ExamplePickerPeopleLabel)),
                        (
                            "placeholder",
                            Value::Message(Key::ExamplePickerPeoplePlaceholder),
                        ),
                        ("hint", Value::Message(Key::ExamplePickerPeopleHint)),
                        ("create_label", Value::Message(Key::ExamplePickerCreate)),
                        ("remove_label", Value::Message(Key::ExamplePickerRemove)),
                        ("status", Value::Message(Key::ExamplePickerStatus)),
                    ],
                    children: &[],
                },
                // THE TWO VALUES THAT ARE NOT THE DEFAULT, on a picker where
                // both are right: a written name may contain the separator,
                // and two guests may share a name.
                Node::Component {
                    name: "TagPicker",
                    props: &[
                        ("name", Value::Text("guests")),
                        ("paste", Value::Bool(false)),
                        ("duplicates", Value::Bool(true)),
                        ("label", Value::Message(Key::ExamplePickerNamesLabel)),
                        (
                            "placeholder",
                            Value::Message(Key::ExamplePickerNamesPlaceholder),
                        ),
                        ("hint", Value::Message(Key::ExamplePickerNamesHint)),
                        ("create_label", Value::Message(Key::ExamplePickerCreate)),
                        ("remove_label", Value::Message(Key::ExamplePickerRemove)),
                        ("status", Value::Message(Key::ExamplePickerStatus)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    // A REAL FORM AGAIN, because everything this component claims is a
    // claim about what a server receives — and because `required` on the
    // composer rather than on the hidden `<select>` is only checkable here.
    (
        r#"TagPicker { required: true, error: "…" } inside a <form>"#,
        &[Node::Element {
            tag: "form",
            attributes: &[
                ("class", Value::Text("nd-example-form")),
                ("method", Value::Text("get")),
            ],
            children: &[
                Node::Component {
                    name: "TagPicker",
                    props: &[
                        ("name", Value::Text("labels")),
                        ("required", Value::Bool(true)),
                        ("error", Value::Message(Key::ExamplePickerError)),
                        ("label", Value::Message(Key::ExamplePickerLabel)),
                        ("placeholder", Value::Message(Key::ExamplePickerPlaceholder)),
                        ("create_label", Value::Message(Key::ExamplePickerCreate)),
                        ("remove_label", Value::Message(Key::ExamplePickerRemove)),
                        ("status", Value::Message(Key::ExamplePickerStatus)),
                    ],
                    children: &[Node::Fragment(TOPICS)],
                },
                Node::Component {
                    name: "Button",
                    props: &[
                        ("submit", Value::Text("submit")),
                        ("label", Value::Message(Key::ExampleSelectSend)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r"TagPicker { size: TagPickerSize::… } / { loading: true, loading_label } / { disabled: true } / { glass: true }",
        &[Node::Element {
            tag: "div",
            attributes: &[(
                "class",
                Value::Text("nd-example-stack nd-example-stack-wide"),
            )],
            children: &[
                Node::Each {
                    over: Each::Values {
                        component: "TagPicker",
                        prop: "size",
                    },
                    body: &[Node::Component {
                        name: "TagPicker",
                        props: &[
                            ("size", Value::Bound(0)),
                            (
                                "name",
                                Value::Join(&[Value::Text("size-"), Value::Bound(0)]),
                            ),
                            ("label", Value::Message(Key::ExamplePickerLabel)),
                            ("placeholder", Value::Message(Key::ExamplePickerPlaceholder)),
                            ("remove_label", Value::Message(Key::ExamplePickerRemove)),
                            ("status", Value::Message(Key::ExamplePickerStatus)),
                        ],
                        children: &[Node::Fragment(TOPICS)],
                    }],
                },
                Node::Component {
                    name: "TagPicker",
                    props: &[
                        ("name", Value::Text("still-coming")),
                        ("loading", Value::Bool(true)),
                        ("loading_label", Value::Message(Key::SearchLoading)),
                        ("label", Value::Message(Key::ExamplePickerLabel)),
                        ("placeholder", Value::Message(Key::ExamplePickerPlaceholder)),
                        ("remove_label", Value::Message(Key::ExamplePickerRemove)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "TagPicker",
                    props: &[
                        ("name", Value::Text("refused")),
                        ("disabled", Value::Bool(true)),
                        ("label", Value::Message(Key::ExamplePickerLabel)),
                        ("placeholder", Value::Message(Key::ExamplePickerPlaceholder)),
                        ("remove_label", Value::Message(Key::ExamplePickerRemove)),
                    ],
                    children: &[Node::Fragment(TOPICS)],
                },
                Node::Component {
                    name: "TagPicker",
                    props: &[
                        ("name", Value::Text("frosted")),
                        ("glass", Value::Bool(true)),
                        ("label", Value::Message(Key::ExamplePickerLabel)),
                        ("placeholder", Value::Message(Key::ExamplePickerPlaceholder)),
                        ("remove_label", Value::Message(Key::ExamplePickerRemove)),
                    ],
                    children: &[Node::Fragment(TOPICS)],
                },
            ],
        }],
    ),
];
