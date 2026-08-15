//! `Segmented`, as trees.
//!
//! One entry per example in `examples::SEGMENTED`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_tree_belongs_to_its_example` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Each, Node, Value};
use crate::trees::Tree;

/// Every `Segmented` example. 9 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"Segmented { name: "…", label: "…", SegmentedItem { … } }"#,
        &[Node::Component {
            name: "Segmented",
            props: &[
                ("name", Value::Text("example-mode")),
                ("label", Value::Message(Key::ThemeModeLabel)),
            ],
            children: &[Node::Each {
                over: Each::Rows(&[
                    &[
                        Value::Text("light"),
                        Value::Message(Key::ThemeModeLight),
                        Value::Bool(false),
                    ],
                    &[
                        Value::Text("dark"),
                        Value::Message(Key::ThemeModeDark),
                        Value::Bool(true),
                    ],
                    &[
                        Value::Text("system"),
                        Value::Message(Key::ThemeModeSystem),
                        Value::Bool(false),
                    ],
                ]),
                body: &[Node::Component {
                    name: "SegmentedItem",
                    props: &[
                        ("value", Value::Bound(0)),
                        ("label", Value::Bound(1)),
                        ("selected", Value::Bound(2)),
                    ],
                    children: &[],
                }],
            }],
        }],
    ),
    (
        r"Segmented { appearance: SegmentedAppearance::Spread, … }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[Node::Each {
                over: Each::Values {
                    component: "Segmented",
                    prop: "appearance",
                },
                body: &[Node::Component {
                    name: "Segmented",
                    props: &[
                        ("appearance", Value::Bound(0)),
                        (
                            "name",
                            Value::Join(&[Value::Text("example-appearance-"), Value::Bound(0)]),
                        ),
                        ("label", Value::Message(Key::ThemeGlassLabel)),
                    ],
                    children: &[Node::Each {
                        over: Each::Rows(&[
                            &[
                                Value::Text("off"),
                                Value::Message(Key::ThemeGlassOff),
                                Value::Bool(false),
                            ],
                            &[
                                Value::Text("subtle"),
                                Value::Message(Key::ThemeGlassSubtle),
                                Value::Bool(true),
                            ],
                            &[
                                Value::Text("full"),
                                Value::Message(Key::ThemeGlassFull),
                                Value::Bool(false),
                            ],
                        ]),
                        body: &[Node::Component {
                            name: "SegmentedItem",
                            props: &[
                                ("value", Value::Bound(0)),
                                ("label", Value::Bound(1)),
                                ("selected", Value::Bound(2)),
                            ],
                            children: &[],
                        }],
                    }],
                }],
            }],
        }],
    ),
    (
        r"Segmented { size: SegmentedSize::… }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[Node::Each {
                over: Each::Values {
                    component: "Segmented",
                    prop: "size",
                },
                body: &[Node::Component {
                    name: "Segmented",
                    props: &[
                        ("size", Value::Bound(0)),
                        (
                            "name",
                            Value::Join(&[Value::Text("example-size-"), Value::Bound(0)]),
                        ),
                        ("label", Value::Message(Key::ThemeFontLabel)),
                    ],
                    children: &[Node::Each {
                        over: Each::Rows(&[
                            &[
                                Value::Text("sans"),
                                Value::Message(Key::ThemeFontSans),
                                Value::Bool(true),
                            ],
                            &[
                                Value::Text("mono"),
                                Value::Message(Key::ThemeFontMono),
                                Value::Bool(false),
                            ],
                        ]),
                        body: &[Node::Component {
                            name: "SegmentedItem",
                            props: &[
                                ("value", Value::Bound(0)),
                                ("label", Value::Bound(1)),
                                ("selected", Value::Bound(2)),
                            ],
                            children: &[],
                        }],
                    }],
                }],
            }],
        }],
    ),
    (
        r"Segmented { shape: SegmentedShape::… }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[Node::Each {
                over: Each::Values {
                    component: "Segmented",
                    prop: "shape",
                },
                body: &[Node::Component {
                    name: "Segmented",
                    props: &[
                        ("shape", Value::Bound(0)),
                        (
                            "name",
                            Value::Join(&[Value::Text("example-shape-"), Value::Bound(0)]),
                        ),
                        ("label", Value::Message(Key::ThemeSaturationLabel)),
                    ],
                    children: &[Node::Each {
                        over: Each::Rows(&[
                            &[
                                Value::Text("balanced"),
                                Value::Message(Key::ThemeSaturationBalanced),
                                Value::Bool(true),
                            ],
                            &[
                                Value::Text("vivid"),
                                Value::Message(Key::ThemeSaturationVivid),
                                Value::Bool(false),
                            ],
                        ]),
                        body: &[Node::Component {
                            name: "SegmentedItem",
                            props: &[
                                ("value", Value::Bound(0)),
                                ("label", Value::Bound(1)),
                                ("selected", Value::Bound(2)),
                            ],
                            children: &[],
                        }],
                    }],
                }],
            }],
        }],
    ),
    (
        r"Segmented { orientation: SegmentedOrientation::Vertical, … }",
        &[Node::Component {
            name: "Segmented",
            props: &[
                ("name", Value::Text("example-vertical")),
                ("label", Value::Message(Key::ThemeRadiusLabel)),
                ("orientation", Value::Text("vertical")),
            ],
            children: &[Node::Each {
                over: Each::Rows(&[
                    &[
                        Value::Text("square"),
                        Value::Message(Key::ThemeRadiusSquare),
                        Value::Bool(true),
                    ],
                    &[
                        Value::Text("soft"),
                        Value::Message(Key::ThemeRadiusSoft),
                        Value::Bool(false),
                    ],
                    &[
                        Value::Text("round"),
                        Value::Message(Key::ThemeRadiusRound),
                        Value::Bool(false),
                    ],
                    &[
                        Value::Text("circular"),
                        Value::Message(Key::ThemeRadiusCircular),
                        Value::Bool(false),
                    ],
                ]),
                body: &[Node::Component {
                    name: "SegmentedItem",
                    props: &[
                        ("value", Value::Bound(0)),
                        ("label", Value::Bound(1)),
                        ("selected", Value::Bound(2)),
                    ],
                    children: &[],
                }],
            }],
        }],
    ),
    (
        r"Segmented { name, label, fill: SegmentedFill::Equal }
Segmented { name, label, fill: SegmentedFill::Stretch }",
        // BOTH VALUES, SIDE BY SIDE. The caption used to say `Equal` while
        // the body wrote `Stretch`, so the one value a reader came to see
        // was demonstrated nowhere on the site. `equal` gives every item
        // the width of the widest and `stretch` then takes the container,
        // which only shows when the two sit one above the other.
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-narrow nd-example-stack"))],
            children: &[Node::Each {
                over: Each::Rows(&[
                    &[Value::Text("example-fill-equal"), Value::Text("equal")],
                    &[Value::Text("example-fill-stretch"), Value::Text("stretch")],
                ]),
                body: &[Node::Component {
                    name: "Segmented",
                    props: &[
                        ("name", Value::Bound(0)),
                        ("label", Value::Message(Key::SettingsPatternLabel)),
                        ("fill", Value::Bound(1)),
                    ],
                    children: &[Node::Each {
                        over: Each::Rows(&[
                            &[
                                Value::Text("none"),
                                Value::Message(Key::SettingsPatternNone),
                                Value::Bool(true),
                            ],
                            &[
                                Value::Text("dots"),
                                Value::Message(Key::SettingsPatternDots),
                                Value::Bool(false),
                            ],
                            &[
                                Value::Text("mesh"),
                                Value::Message(Key::SettingsPatternMesh),
                                Value::Bool(false),
                            ],
                        ]),
                        body: &[Node::Component {
                            name: "SegmentedItem",
                            props: &[
                                ("value", Value::Bound(0)),
                                ("label", Value::Bound(1)),
                                ("selected", Value::Bound(2)),
                            ],
                            children: &[],
                        }],
                    }],
                }],
            }],
        }],
    ),
    (
        r#"Segmented { labelledby: "…", … }"#,
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-stack"))],
            children: &[
                Node::Component {
                    name: "Label",
                    props: &[
                        ("kind", Value::Text("name")),
                        ("for_control", Value::Text("example-visible")),
                        ("text", Value::Message(Key::ThemeSaturationLabel)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "Segmented",
                    props: &[
                        ("name", Value::Text("example-labelledby")),
                        ("labelledby", Value::Text("example-visible-label")),
                    ],
                    children: &[Node::Each {
                        over: Each::Rows(&[
                            &[
                                Value::Text("balanced"),
                                Value::Message(Key::ThemeSaturationBalanced),
                                Value::Bool(false),
                            ],
                            &[
                                Value::Text("vivid"),
                                Value::Message(Key::ThemeSaturationVivid),
                                Value::Bool(true),
                            ],
                            &[
                                Value::Text("sober"),
                                Value::Message(Key::ThemeSaturationSober),
                                Value::Bool(false),
                            ],
                        ]),
                        body: &[Node::Component {
                            name: "SegmentedItem",
                            props: &[
                                ("value", Value::Bound(0)),
                                ("label", Value::Bound(1)),
                                ("selected", Value::Bound(2)),
                            ],
                            children: &[],
                        }],
                    }],
                },
            ],
        }],
    ),
    (
        r"Segmented { glass: true, … }",
        &[Node::Component {
            name: "Segmented",
            props: &[
                ("name", Value::Text("example-glass")),
                ("glass", Value::Bool(true)),
                ("label", Value::Message(Key::ThemeSaturationLabel)),
            ],
            children: &[
                Node::Component {
                    name: "SegmentedItem",
                    props: &[
                        ("value", Value::Text("balanced")),
                        ("label", Value::Message(Key::ThemeSaturationBalanced)),
                        ("selected", Value::Bool(true)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "SegmentedItem",
                    props: &[
                        ("value", Value::Text("vivid")),
                        ("label", Value::Message(Key::ThemeSaturationVivid)),
                    ],
                    children: &[],
                },
            ],
        }],
    ),
    (
        r"Segmented { selection: SegmentedSelection::Quiet, … }",
        &[Node::Component {
            name: "Segmented",
            props: &[
                ("name", Value::Text("example-quiet")),
                ("selection", Value::Text("quiet")),
                ("size", Value::Text("small")),
                ("label", Value::Message(Key::ExampleWidthLabel)),
            ],
            children: &[Node::Each {
                over: Each::Rows(&[
                    &[
                        Value::Text("320"),
                        Value::Data("320"),
                        Value::Data("px"),
                        Value::Bool(false),
                    ],
                    &[
                        Value::Text("sm"),
                        Value::Data("30"),
                        Value::Data("rem"),
                        Value::Bool(true),
                    ],
                    &[
                        Value::Text("md"),
                        Value::Data("48"),
                        Value::Data("rem"),
                        Value::Bool(false),
                    ],
                ]),
                body: &[Node::Component {
                    name: "SegmentedItem",
                    props: &[
                        ("value", Value::Bound(0)),
                        ("class", Value::Text("nd-identifier")),
                        ("label", Value::Bound(1)),
                        ("detail", Value::Bound(2)),
                        ("selected", Value::Bound(3)),
                    ],
                    children: &[],
                }],
            }],
        }],
    ),
];
