//! `LineageFigure`, as trees.
//!
//! One entry per example in `examples::LINEAGE_FIGURE`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_tree_belongs_to_its_example` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.
//!
//! The scenes these figures stand in are `Lineage`'s own, read from
//! `super::lineage` rather than copied — a second copy of a graph is a second
//! copy that stops agreeing.

use noctua_design_i18n::Key;

use super::lineage::{SERVER, SLOTS};
use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Every `LineageFigure` example. 4 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"LineageFigure { scene: "…", at: "ada", shape: Card, tone: Accent, parallax: Subtle }"#,
        &[Node::Component {
            name: "Lineage",
            props: &[
                ("name", Value::Text("figure-basic")),
                ("label", Value::Message(Key::ExampleLineageSlots)),
                ("graph", Value::Text(SLOTS)),
                ("parallax", Value::Bool(true)),
            ],
            children: &[
                // AND THE ORIGIN, because the graph declares it a slot and a
                // slot nobody fills is a connection running into an empty box.
                Node::Component {
                    name: "LineageFigure",
                    props: &[
                        ("scene", Value::Text("figure-basic")),
                        ("at", Value::Text("noctua")),
                        ("shape", Value::Text("plaque")),
                    ],
                    children: &[Node::Component {
                        name: "Text",
                        props: &[
                            ("class", Value::Text("nd-identifier")),
                            ("size", Value::Text("small")),
                            ("weight", Value::Text("medium")),
                        ],
                        children: &[Node::Text(Value::Data("Noctua"))],
                    }],
                },
                Node::Component {
                    name: "LineageFigure",
                    props: &[
                        ("scene", Value::Text("figure-basic")),
                        ("at", Value::Text("ada")),
                        ("shape", Value::Text("card")),
                        ("tone", Value::Text("accent")),
                        // And the accent card LEANS, so the figure's own
                        // `parallax` keeps a written example.
                        ("parallax", Value::Text("subtle")),
                    ],
                    children: &[
                        Node::Component {
                            name: "Avatar",
                            props: &[
                                ("label", Value::Data("Ada Doe")),
                                ("src", Value::Asset("assets/portraits/ada.svg")),
                                ("size", Value::Text("small")),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "Text",
                            props: &[
                                ("class", Value::Text("nd-identifier")),
                                ("size", Value::Text("small")),
                                ("weight", Value::Text("medium")),
                            ],
                            children: &[Node::Text(Value::Data("Ada Doe"))],
                        },
                        Node::Component {
                            name: "Text",
                            props: &[
                                ("class", Value::Text("nd-identifier")),
                                ("size", Value::Text("caption")),
                                ("tone", Value::Text("muted")),
                            ],
                            children: &[Node::Text(Value::Data("1946"))],
                        },
                    ],
                },
                // THE SECOND SLOT, AND IT IS NOT DECORATION. The graph declares
                // two, and a slot nobody fills draws an empty box with the
                // connections still running into it. It also puts the tone
                // beside its absence, which is what the pair is showing.
                Node::Component {
                    name: "LineageFigure",
                    props: &[
                        ("scene", Value::Text("figure-basic")),
                        ("at", Value::Text("eve")),
                        ("shape", Value::Text("card")),
                    ],
                    children: &[
                        Node::Component {
                            name: "Avatar",
                            props: &[
                                ("label", Value::Data("Eve Marsh")),
                                ("src", Value::Asset("assets/portraits/eve.svg")),
                                ("size", Value::Text("small")),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "Text",
                            props: &[
                                ("class", Value::Text("nd-identifier")),
                                ("size", Value::Text("small")),
                                ("weight", Value::Text("medium")),
                            ],
                            children: &[Node::Text(Value::Data("Eve Marsh"))],
                        },
                        Node::Component {
                            name: "Text",
                            props: &[
                                ("class", Value::Text("nd-identifier")),
                                ("size", Value::Text("caption")),
                                ("tone", Value::Text("muted")),
                            ],
                            children: &[Node::Text(Value::Data("1948"))],
                        },
                    ],
                },
            ],
        }],
    ),
    (
        r"LineageFigure { shape: Plaque, tone: Warning }  ·  { shape: Bare }",
        &[Node::Component {
            name: "Lineage",
            props: &[
                ("name", Value::Text("figure-shapes")),
                ("label", Value::Message(Key::ExampleLineageSlots)),
                ("graph", Value::Text(SLOTS)),
            ],
            children: &[
                // A PLAQUE IS TWO CELLS WIDE, SO IT STANDS WHERE TWO CELLS
                // ARE FREE. At `ada` it reached across the union dot beside her
                // — a shape overlapping the structure it is standing on.
                Node::Component {
                    name: "LineageFigure",
                    props: &[
                        ("scene", Value::Text("figure-shapes")),
                        ("at", Value::Text("noctua")),
                        ("shape", Value::Text("plaque")),
                        ("tone", Value::Text("warning")),
                    ],
                    children: &[Node::Component {
                        name: "Text",
                        props: &[
                            ("class", Value::Text("nd-identifier")),
                            ("size", Value::Text("small")),
                            ("weight", Value::Text("medium")),
                        ],
                        children: &[Node::Text(Value::Data("Noctua"))],
                    }],
                },
                Node::Component {
                    name: "LineageFigure",
                    props: &[
                        ("scene", Value::Text("figure-shapes")),
                        ("at", Value::Text("ada")),
                    ],
                    children: &[
                        Node::Component {
                            name: "Avatar",
                            props: &[
                                ("label", Value::Data("Ada Doe")),
                                ("src", Value::Asset("assets/portraits/ada.svg")),
                                ("size", Value::Text("small")),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "Text",
                            props: &[
                                ("class", Value::Text("nd-identifier")),
                                ("size", Value::Text("small")),
                                ("weight", Value::Text("medium")),
                            ],
                            children: &[Node::Text(Value::Data("Ada Doe"))],
                        },
                    ],
                },
                Node::Component {
                    name: "LineageFigure",
                    props: &[
                        ("scene", Value::Text("figure-shapes")),
                        ("at", Value::Text("eve")),
                        ("shape", Value::Text("bare")),
                    ],
                    children: &[Node::Component {
                        name: "Avatar",
                        props: &[
                            ("label", Value::Data("Eve Marsh")),
                            ("src", Value::Asset("assets/portraits/eve.svg")),
                            ("size", Value::Text("large")),
                        ],
                        children: &[],
                    }],
                },
            ],
        }],
    ),
    (
        r#"LineageFigure { href: "…", label: "Ada Doe", occasion: "christmas" }"#,
        &[Node::Component {
            name: "Lineage",
            props: &[
                ("name", Value::Text("figure-link")),
                ("label", Value::Message(Key::ExampleLineageSlots)),
                ("graph", Value::Text(SLOTS)),
                ("occasion", Value::Text("christmas")),
            ],
            children: &[
                // AND THE ORIGIN, because the graph declares it a slot and a
                // slot nobody fills is a connection running into an empty box.
                Node::Component {
                    name: "LineageFigure",
                    props: &[
                        ("scene", Value::Text("figure-link")),
                        ("at", Value::Text("noctua")),
                        ("shape", Value::Text("plaque")),
                    ],
                    children: &[Node::Component {
                        name: "Text",
                        props: &[
                            ("class", Value::Text("nd-identifier")),
                            ("size", Value::Text("small")),
                            ("weight", Value::Text("medium")),
                        ],
                        children: &[Node::Text(Value::Data("Noctua"))],
                    }],
                },
                Node::Component {
                    name: "LineageFigure",
                    props: &[
                        ("scene", Value::Text("figure-link")),
                        ("at", Value::Text("eve")),
                        ("href", Value::Text("../tree/")),
                    ],
                    children: &[
                        Node::Component {
                            name: "Avatar",
                            props: &[
                                ("label", Value::Data("Eve Marsh")),
                                ("src", Value::Asset("assets/portraits/eve.svg")),
                            ],
                            children: &[],
                        },
                        Node::Component {
                            name: "Text",
                            props: &[
                                ("class", Value::Text("nd-identifier")),
                                ("size", Value::Text("small")),
                                ("weight", Value::Text("medium")),
                            ],
                            children: &[Node::Text(Value::Data("Eve Marsh"))],
                        },
                    ],
                },
                Node::Component {
                    name: "LineageFigure",
                    props: &[
                        ("scene", Value::Text("figure-link")),
                        ("at", Value::Text("ada")),
                        ("occasion", Value::Text("christmas")),
                        ("tone", Value::Text("danger")),
                        ("label", Value::Data("Ada Doe")),
                    ],
                    children: &[
                        Node::Component {
                            name: "Avatar",
                            props: &[
                                ("label", Value::Data("Ada Doe")),
                                ("src", Value::Asset("assets/portraits/ada-christmas.svg")),
                                ("size", Value::Text("small")),
                            ],
                            children: &[],
                        },
                        // The name FIRST — a card whose only text was "wearing a
                        // paper hat" never said whose head the hat was on.
                        Node::Component {
                            name: "Text",
                            props: &[
                                ("class", Value::Text("nd-identifier")),
                                ("size", Value::Text("small")),
                                ("weight", Value::Text("medium")),
                            ],
                            children: &[Node::Text(Value::Data("Ada Doe"))],
                        },
                        Node::Component {
                            name: "Text",
                            props: &[
                                ("size", Value::Text("caption")),
                                ("tone", Value::Text("muted")),
                            ],
                            children: &[Node::Text(Value::Message(Key::ExampleLineageHat))],
                        },
                    ],
                },
            ],
        }],
    ),
    (
        r#"LineageFigure { action: "lineage-swap", swaps: "doe:remote", set: "doe:local" }"#,
        &[Node::Component {
            name: "Lineage",
            props: &[
                ("name", Value::Text("figure-server")),
                ("label", Value::Message(Key::ExampleLineageServer)),
                ("graph", Value::Text(SERVER)),
                ("sets", Value::Text("doe:local")),
            ],
            children: &[Node::Component {
                name: "LineageFigure",
                props: &[
                    ("scene", Value::Text("figure-server")),
                    ("at", Value::Text("srv")),
                    ("shape", Value::Text("round")),
                    ("tone", Value::Text("success")),
                    ("action", Value::Text("lineage-swap")),
                    ("swaps", Value::Text("doe:remote")),
                    ("label", Value::Message(Key::ExampleLineageServer)),
                ],
                children: &[Node::Component {
                    name: "Text",
                    props: &[
                        ("class", Value::Text("nd-identifier")),
                        ("size", Value::Text("caption")),
                        ("weight", Value::Text("medium")),
                    ],
                    children: &[Node::Text(Value::Data("Doe"))],
                }],
            }],
        }],
    ),
];
