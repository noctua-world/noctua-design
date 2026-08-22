//! `Tag`, as trees.
//!
//! One entry per example in `examples::TAG`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Each, Node, Value};
use crate::trees::Tree;

/// Every `Tag` example. 7 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"Tag { label: "…", tone: "…", variant: TagVariant::… }"#,
        &[
            Node::Element {
                tag: "div",
                attributes: &[("class", Value::Text("nd-example-row"))],
                // ONE OF THESE IS NOT ONE OF `Button`'S FOUR. `shipped` is a
                // family the colour system has always declared and no component
                // could name until `tone` stopped being a closed list.
                children: &[Node::Each {
                    over: Each::Rows(&[
                        &[Value::Text("neutral"), Value::Message(Key::ExampleTagDraft)],
                        &[Value::Text("accent"), Value::Message(Key::ExampleTagReview)],
                        &[
                            Value::Text("shipped"),
                            Value::Message(Key::ExampleTagShipped),
                        ],
                        &[
                            Value::Text("danger"),
                            Value::Message(Key::ExampleTagBlocked),
                        ],
                    ]),
                    body: &[Node::Component {
                        name: "Tag",
                        props: &[("tone", Value::Bound(0)), ("label", Value::Bound(1))],
                        children: &[],
                    }],
                }],
            },
            Node::Element {
                tag: "div",
                attributes: &[("class", Value::Text("nd-example-row"))],
                children: &[Node::Each {
                    over: Each::Rows(&[
                        &[Value::Text("soft"), Value::Message(Key::ExampleTagDraft)],
                        &[Value::Text("solid"), Value::Message(Key::ExampleTagReview)],
                        &[
                            Value::Text("outline"),
                            Value::Message(Key::ExampleTagShipped),
                        ],
                    ]),
                    body: &[Node::Component {
                        name: "Tag",
                        props: &[
                            ("variant", Value::Bound(0)),
                            ("tone", Value::Text("accent")),
                            ("label", Value::Bound(1)),
                        ],
                        children: &[],
                    }],
                }],
            },
        ],
    ),
    (
        r#"Tag { icon: IconDrawing::CHECK, swatch: "…", image: "…" }"#,
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-row"))],
            children: &[
                Node::Component {
                    name: "Tag",
                    props: &[
                        ("icon", Value::Text("check")),
                        ("tone", Value::Text("success")),
                        ("label", Value::Message(Key::ExampleTagShipped)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Tag",
                    props: &[
                        ("class", Value::Text("nd-identifier")),
                        ("swatch", Value::Text("teal-balanced")),
                        ("label", Value::Data("teal-balanced")),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Tag",
                    props: &[
                        ("class", Value::Text("nd-identifier")),
                        ("swatch", Value::Text("clay-sober")),
                        ("label", Value::Data("clay-sober")),
                    ],
                    children: &[],
                },
                // The site's own mark, which is a file the build already
                // writes — an example that pointed at a URL nothing serves
                // would be an example of the fallback.
                Node::Component {
                    name: "Tag",
                    props: &[
                        ("image", Value::Asset("favicon.svg")),
                        ("label", Value::Message(Key::AppName)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r#"Tag { href: "…" } / Tag { pressed: TagPressed::On } / Tag { disabled: true }"#,
        &[
            Node::Element {
                tag: "div",
                attributes: &[("class", Value::Text("nd-example-row"))],
                children: &[
                    Node::Component {
                        name: "Tag",
                        props: &[
                            ("class", Value::Text("nd-identifier")),
                            ("href", Value::Text("../button/")),
                            ("label", Value::Data("Button")),
                        ],
                        children: &[],
                    },
                    Node::Component {
                        name: "Tag",
                        props: &[
                            ("class", Value::Text("nd-identifier")),
                            ("href", Value::Text("../select/")),
                            ("label", Value::Data("Select")),
                        ],
                        children: &[],
                    },
                ],
            },
            Node::Element {
                tag: "div",
                attributes: &[("class", Value::Text("nd-example-row"))],
                children: &[
                    Node::Component {
                        name: "Tag",
                        props: &[
                            ("pressed", Value::Text("on")),
                            ("label", Value::Message(Key::ExampleTagShipped)),
                        ],
                        children: &[],
                    },
                    Node::Component {
                        name: "Tag",
                        props: &[
                            ("pressed", Value::Text("off")),
                            ("label", Value::Message(Key::ExampleTagReview)),
                        ],
                        children: &[],
                    },
                    Node::Component {
                        name: "Tag",
                        props: &[
                            ("pressed", Value::Text("off")),
                            ("disabled", Value::Bool(true)),
                            ("label", Value::Message(Key::ExampleTagBlocked)),
                        ],
                        children: &[],
                    },
                ],
            },
        ],
    ),
    (
        r#"Tag { remove_label: "…", remove_name: "…", value: "…" } inside a <form>"#,
        // A REAL FORM, so the claim is made rather than described: taking a tag
        // off posts which one, and the server renders the list again. Press one
        // with the script blocked and it still works.
        &[Node::Element {
            tag: "form",
            attributes: &[
                ("class", Value::Text("nd-example-form")),
                ("method", Value::Text("get")),
            ],
            children: &[Node::Element {
                tag: "div",
                attributes: &[("class", Value::Text("nd-example-row"))],
                children: &[Node::Each {
                    over: Each::Rows(&[
                        &[Value::Text("draft"), Value::Message(Key::ExampleTagDraft)],
                        &[Value::Text("review"), Value::Message(Key::ExampleTagReview)],
                        &[
                            Value::Text("shipped"),
                            Value::Message(Key::ExampleTagShipped),
                        ],
                    ]),
                    body: &[Node::Component {
                        name: "Tag",
                        props: &[
                            ("value", Value::Bound(0)),
                            ("label", Value::Bound(1)),
                            ("remove_label", Value::Message(Key::ExampleTagRemove)),
                            ("remove_name", Value::Text("remove")),
                        ],
                        children: &[],
                    }],
                }],
            }],
        }],
    ),
    (
        r"Tag { size: TagSize::… } / Tag { shape: TagShape::… }",
        &[
            Node::Element {
                tag: "div",
                attributes: &[("class", Value::Text("nd-example-row"))],
                children: &[Node::Each {
                    over: Each::Values {
                        component: "Tag",
                        prop: "size",
                    },
                    body: &[Node::Component {
                        name: "Tag",
                        props: &[
                            ("size", Value::Bound(0)),
                            ("label", Value::Message(Key::ExampleTagReview)),
                            ("remove_label", Value::Message(Key::ExampleTagRemove)),
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
                        component: "Tag",
                        prop: "shape",
                    },
                    body: &[Node::Component {
                        name: "Tag",
                        props: &[
                            ("shape", Value::Bound(0)),
                            ("variant", Value::Text("outline")),
                            ("label", Value::Message(Key::ExampleTagDraft)),
                        ],
                        children: &[],
                    }],
                }],
            },
        ],
    ),
    (
        r"Tag { glass: true }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-row"))],
            // THE WORD AND THE COLOUR SAY THE SAME THING. Four pills
            // reading *In review* with one painted `danger` and one
            // `shipped` is the page teaching the opposite of the rule two
            // examples above it.
            children: &[Node::Each {
                over: Each::Rows(&[
                    &[Value::Text("accent"), Value::Message(Key::ExampleTagReview)],
                    &[Value::Text("neutral"), Value::Message(Key::ExampleTagDraft)],
                    &[
                        Value::Text("danger"),
                        Value::Message(Key::ExampleTagBlocked),
                    ],
                    &[
                        Value::Text("shipped"),
                        Value::Message(Key::ExampleTagShipped),
                    ],
                ]),
                body: &[Node::Component {
                    name: "Tag",
                    props: &[
                        ("tone", Value::Bound(0)),
                        ("glass", Value::Bool(true)),
                        ("variant", Value::Text("outline")),
                        ("label", Value::Bound(1)),
                    ],
                    children: &[],
                }],
            }],
        }],
    ),
    (
        r"Tag { ai_mark, ai_label, ai_note }",
        &[
            Node::Element {
                tag: "div",
                attributes: &[("class", Value::Text("nd-example-row"))],
                // ONE OF THESE IS NOT ONE OF `Button`'S FOUR. `shipped` is a
                // family the colour system has always declared and no component
                // could name until `tone` stopped being a closed list.
                children: &[Node::Each {
                    over: Each::Rows(&[
                        &[Value::Text("neutral"), Value::Message(Key::ExampleTagDraft)],
                        &[Value::Text("accent"), Value::Message(Key::ExampleTagReview)],
                        &[
                            Value::Text("shipped"),
                            Value::Message(Key::ExampleTagShipped),
                        ],
                        &[
                            Value::Text("danger"),
                            Value::Message(Key::ExampleTagBlocked),
                        ],
                    ]),
                    body: &[Node::Component {
                        name: "Tag",
                        props: &[
                            ("tone", Value::Bound(0)),
                            ("label", Value::Bound(1)),
                            ("ai_mark", Value::Message(Key::ExampleAiMark)),
                            ("ai_label", Value::Message(Key::ExampleAiName)),
                            ("ai_note", Value::Message(Key::ExampleAiNote)),
                        ],
                        children: &[],
                    }],
                }],
            },
            Node::Element {
                tag: "div",
                attributes: &[("class", Value::Text("nd-example-row"))],
                children: &[Node::Each {
                    over: Each::Rows(&[
                        &[Value::Text("soft"), Value::Message(Key::ExampleTagDraft)],
                        &[Value::Text("solid"), Value::Message(Key::ExampleTagReview)],
                        &[
                            Value::Text("outline"),
                            Value::Message(Key::ExampleTagShipped),
                        ],
                    ]),
                    body: &[Node::Component {
                        name: "Tag",
                        props: &[
                            ("variant", Value::Bound(0)),
                            ("tone", Value::Text("accent")),
                            ("label", Value::Bound(1)),
                        ],
                        children: &[],
                    }],
                }],
            },
        ],
    ),
];
