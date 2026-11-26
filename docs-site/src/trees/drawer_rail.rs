//! `DrawerRail`, as trees.
//!
//! One entry per example in `examples::DRAWER_RAIL`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Every `DrawerRail` example. 4 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r"DrawerRail { Toolbar { density: ToolbarDensity::Rail, … } }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-window"))],
            children: &[
                Node::Element {
                    tag: "div",
                    attributes: &[("class", Value::Text("nd-example-work"))],
                    children: &[Node::Component {
                        name: "Paragraph",
                        props: &[],
                        children: &[Node::Text(Value::Message(Key::ExampleDrawerWork))],
                    }],
                },
                Node::Component {
                    name: "Drawer",
                    props: &[
                        ("name", Value::Text("rail-controls")),
                        ("label", Value::Message(Key::ExampleDrawerLabel)),
                    ],
                    children: &[
                        Node::Component {
                            name: "DrawerRail",
                            props: &[("side", Value::Text("start"))],
                            children: &[Node::Component {
                                name: "Toolbar",
                                props: &[
                                    ("density", Value::Text("rail")),
                                    ("label", Value::Message(Key::ExampleRailControls)),
                                ],
                                children: &[
                                    Node::Component {
                                        name: "ToolbarGroup",
                                        props: &[],
                                        children: &[
                                            Node::Component {
                                                name: "Tooltip",
                                                props: &[
                                                    ("name", Value::Text("rail-controls-collapse")),
                                                    (
                                                        "text",
                                                        Value::Message(Key::ExampleRailCollapse),
                                                    ),
                                                ],
                                                children: &[Node::Component {
                                                    name: "Button",
                                                    props: &[
                                                        ("variant", Value::Text("transparent")),
                                                        ("tone", Value::Text("neutral")),
                                                        ("size", Value::Text("small")),
                                                        ("hide_label", Value::Bool(true)),
                                                        ("icon", Value::Text("chevrons-down")),
                                                        (
                                                            "label",
                                                            Value::Message(
                                                                Key::ExampleRailCollapse,
                                                            ),
                                                        ),
                                                    ],
                                                    children: &[],
                                                }],
                                            },
                                            Node::Component {
                                                name: "Tooltip",
                                                props: &[
                                                    ("name", Value::Text("rail-controls-close")),
                                                    ("text", Value::Message(Key::ExampleRailClose)),
                                                ],
                                                children: &[Node::Component {
                                                    name: "Button",
                                                    props: &[
                                                        ("variant", Value::Text("transparent")),
                                                        ("tone", Value::Text("neutral")),
                                                        ("size", Value::Text("small")),
                                                        ("hide_label", Value::Bool(true)),
                                                        ("icon", Value::Text("close")),
                                                        (
                                                            "label",
                                                            Value::Message(Key::ExampleRailClose),
                                                        ),
                                                    ],
                                                    children: &[],
                                                }],
                                            },
                                        ],
                                    },
                                    Node::Component {
                                        name: "ToolbarGroup",
                                        props: &[],
                                        children: &[Node::Component {
                                            name: "Tooltip",
                                            props: &[
                                                ("name", Value::Text("rail-controls-direction")),
                                                ("text", Value::Message(Key::ExampleRailDirection)),
                                            ],
                                            children: &[Node::Component {
                                                name: "Button",
                                                props: &[
                                                    ("variant", Value::Text("transparent")),
                                                    ("tone", Value::Text("neutral")),
                                                    ("size", Value::Text("small")),
                                                    ("hide_label", Value::Bool(true)),
                                                    ("icon", Value::Text("arrow-up-down")),
                                                    (
                                                        "label",
                                                        Value::Message(Key::ExampleRailDirection),
                                                    ),
                                                ],
                                                children: &[],
                                            }],
                                        }],
                                    },
                                ],
                            }],
                        },
                        Node::Element {
                            tag: "div",
                            attributes: &[("class", Value::Text("nd-example-pane"))],
                            children: &[Node::Component {
                                name: "Paragraph",
                                props: &[],
                                children: &[Node::Text(Value::Message(
                                    Key::ExampleRailCommandsBody,
                                ))],
                            }],
                        },
                    ],
                },
            ],
        }],
    ),
    (
        r"DrawerRail { side: DrawerRailSide::End, Tabs { appearance: TabsAppearance::Rail, … } }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-window"))],
            children: &[
                Node::Element {
                    tag: "div",
                    attributes: &[("class", Value::Text("nd-example-work"))],
                    children: &[Node::Component {
                        name: "Paragraph",
                        props: &[],
                        children: &[Node::Text(Value::Message(Key::ExampleDrawerWork))],
                    }],
                },
                Node::Component {
                    name: "Drawer",
                    props: &[
                        ("name", Value::Text("rail-tabs")),
                        ("label", Value::Message(Key::ExampleDrawerLabel)),
                    ],
                    children: &[
                        Node::Element {
                            tag: "div",
                            attributes: &[("class", Value::Text("nd-example-pane"))],
                            children: &[
                                Node::Component {
                                    name: "TabPanel",
                                    props: &[
                                        ("name", Value::Text("rail-tabs-pages")),
                                        ("value", Value::Text("commands")),
                                    ],
                                    children: &[Node::Component {
                                        name: "Paragraph",
                                        props: &[],
                                        children: &[Node::Text(Value::Message(
                                            Key::ExampleRailCommandsBody,
                                        ))],
                                    }],
                                },
                                Node::Component {
                                    name: "TabPanel",
                                    props: &[
                                        ("name", Value::Text("rail-tabs-pages")),
                                        ("value", Value::Text("settings")),
                                    ],
                                    children: &[Node::Component {
                                        name: "Paragraph",
                                        props: &[],
                                        children: &[Node::Text(Value::Message(
                                            Key::ExampleRailSettingsBody,
                                        ))],
                                    }],
                                },
                            ],
                        },
                        Node::Component {
                            name: "DrawerRail",
                            props: &[("side", Value::Text("end"))],
                            children: &[Node::Component {
                                name: "Tabs",
                                props: &[
                                    ("name", Value::Text("rail-tabs-pages")),
                                    ("label", Value::Message(Key::ExampleRailPages)),
                                    ("appearance", Value::Text("rail")),
                                    ("size", Value::Text("small")),
                                ],
                                children: &[
                                    Node::Component {
                                        name: "Tooltip",
                                        props: &[
                                            ("name", Value::Text("rail-tabs-pages-commands")),
                                            ("text", Value::Message(Key::ExampleRailCommands)),
                                            ("placement", Value::Text("end")),
                                        ],
                                        children: &[Node::Component {
                                            name: "Tab",
                                            props: &[
                                                ("name", Value::Text("rail-tabs-pages")),
                                                ("value", Value::Text("commands")),
                                                ("icon", Value::Text("terminal")),
                                                ("label", Value::Message(Key::ExampleRailCommands)),
                                                ("selected", Value::Bool(true)),
                                            ],
                                            children: &[],
                                        }],
                                    },
                                    Node::Component {
                                        name: "Tooltip",
                                        props: &[
                                            ("name", Value::Text("rail-tabs-pages-settings")),
                                            ("text", Value::Message(Key::ExampleRailSettings)),
                                            ("placement", Value::Text("end")),
                                        ],
                                        children: &[Node::Component {
                                            name: "Tab",
                                            props: &[
                                                ("name", Value::Text("rail-tabs-pages")),
                                                ("value", Value::Text("settings")),
                                                ("icon", Value::Text("settings")),
                                                ("label", Value::Message(Key::ExampleRailSettings)),
                                            ],
                                            children: &[],
                                        }],
                                    },
                                ],
                            }],
                        },
                    ],
                },
            ],
        }],
    ),
    (
        r"DrawerRail { … } / DrawerRail { side: DrawerRailSide::End, … }",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-window"))],
            children: &[
                Node::Element {
                    tag: "div",
                    attributes: &[("class", Value::Text("nd-example-work"))],
                    children: &[Node::Component {
                        name: "Paragraph",
                        props: &[],
                        children: &[Node::Text(Value::Message(Key::ExampleDrawerWork))],
                    }],
                },
                Node::Component {
                    name: "Drawer",
                    props: &[
                        ("name", Value::Text("rail-both")),
                        ("label", Value::Message(Key::ExampleDrawerLabel)),
                    ],
                    children: &[
                        Node::Component {
                            name: "DrawerRail",
                            props: &[("side", Value::Text("start"))],
                            children: &[Node::Component {
                                name: "Toolbar",
                                props: &[
                                    ("density", Value::Text("rail")),
                                    ("label", Value::Message(Key::ExampleRailControls)),
                                ],
                                children: &[
                                    Node::Component {
                                        name: "ToolbarGroup",
                                        props: &[],
                                        children: &[
                                            Node::Component {
                                                name: "Tooltip",
                                                props: &[
                                                    ("name", Value::Text("rail-both-collapse")),
                                                    (
                                                        "text",
                                                        Value::Message(Key::ExampleRailCollapse),
                                                    ),
                                                ],
                                                children: &[Node::Component {
                                                    name: "Button",
                                                    props: &[
                                                        ("variant", Value::Text("transparent")),
                                                        ("tone", Value::Text("neutral")),
                                                        ("size", Value::Text("small")),
                                                        ("hide_label", Value::Bool(true)),
                                                        ("icon", Value::Text("chevrons-down")),
                                                        (
                                                            "label",
                                                            Value::Message(
                                                                Key::ExampleRailCollapse,
                                                            ),
                                                        ),
                                                    ],
                                                    children: &[],
                                                }],
                                            },
                                            Node::Component {
                                                name: "Tooltip",
                                                props: &[
                                                    ("name", Value::Text("rail-both-close")),
                                                    ("text", Value::Message(Key::ExampleRailClose)),
                                                ],
                                                children: &[Node::Component {
                                                    name: "Button",
                                                    props: &[
                                                        ("variant", Value::Text("transparent")),
                                                        ("tone", Value::Text("neutral")),
                                                        ("size", Value::Text("small")),
                                                        ("hide_label", Value::Bool(true)),
                                                        ("icon", Value::Text("close")),
                                                        (
                                                            "label",
                                                            Value::Message(Key::ExampleRailClose),
                                                        ),
                                                    ],
                                                    children: &[],
                                                }],
                                            },
                                        ],
                                    },
                                    Node::Component {
                                        name: "ToolbarGroup",
                                        props: &[],
                                        children: &[Node::Component {
                                            name: "Tooltip",
                                            props: &[
                                                ("name", Value::Text("rail-both-direction")),
                                                ("text", Value::Message(Key::ExampleRailDirection)),
                                            ],
                                            children: &[Node::Component {
                                                name: "Button",
                                                props: &[
                                                    ("variant", Value::Text("transparent")),
                                                    ("tone", Value::Text("neutral")),
                                                    ("size", Value::Text("small")),
                                                    ("hide_label", Value::Bool(true)),
                                                    ("icon", Value::Text("arrow-up-down")),
                                                    (
                                                        "label",
                                                        Value::Message(Key::ExampleRailDirection),
                                                    ),
                                                ],
                                                children: &[],
                                            }],
                                        }],
                                    },
                                ],
                            }],
                        },
                        Node::Element {
                            tag: "div",
                            attributes: &[("class", Value::Text("nd-example-pane"))],
                            children: &[
                                Node::Component {
                                    name: "TabPanel",
                                    props: &[
                                        ("name", Value::Text("rail-both-pages")),
                                        ("value", Value::Text("commands")),
                                    ],
                                    children: &[Node::Component {
                                        name: "Paragraph",
                                        props: &[],
                                        children: &[Node::Text(Value::Message(
                                            Key::ExampleRailCommandsBody,
                                        ))],
                                    }],
                                },
                                Node::Component {
                                    name: "TabPanel",
                                    props: &[
                                        ("name", Value::Text("rail-both-pages")),
                                        ("value", Value::Text("settings")),
                                    ],
                                    children: &[Node::Component {
                                        name: "Paragraph",
                                        props: &[],
                                        children: &[Node::Text(Value::Message(
                                            Key::ExampleRailSettingsBody,
                                        ))],
                                    }],
                                },
                            ],
                        },
                        Node::Component {
                            name: "DrawerRail",
                            props: &[("side", Value::Text("end"))],
                            children: &[Node::Component {
                                name: "Tabs",
                                props: &[
                                    ("name", Value::Text("rail-both-pages")),
                                    ("label", Value::Message(Key::ExampleRailPages)),
                                    ("appearance", Value::Text("rail")),
                                    ("size", Value::Text("small")),
                                ],
                                children: &[
                                    Node::Component {
                                        name: "Tooltip",
                                        props: &[
                                            ("name", Value::Text("rail-both-pages-commands")),
                                            ("text", Value::Message(Key::ExampleRailCommands)),
                                            ("placement", Value::Text("end")),
                                        ],
                                        children: &[Node::Component {
                                            name: "Tab",
                                            props: &[
                                                ("name", Value::Text("rail-both-pages")),
                                                ("value", Value::Text("commands")),
                                                ("icon", Value::Text("terminal")),
                                                ("label", Value::Message(Key::ExampleRailCommands)),
                                                ("selected", Value::Bool(true)),
                                            ],
                                            children: &[],
                                        }],
                                    },
                                    Node::Component {
                                        name: "Tooltip",
                                        props: &[
                                            ("name", Value::Text("rail-both-pages-settings")),
                                            ("text", Value::Message(Key::ExampleRailSettings)),
                                            ("placement", Value::Text("end")),
                                        ],
                                        children: &[Node::Component {
                                            name: "Tab",
                                            props: &[
                                                ("name", Value::Text("rail-both-pages")),
                                                ("value", Value::Text("settings")),
                                                ("icon", Value::Text("settings")),
                                                ("label", Value::Message(Key::ExampleRailSettings)),
                                            ],
                                            children: &[],
                                        }],
                                    },
                                ],
                            }],
                        },
                    ],
                },
            ],
        }],
    ),
    (
        r"DrawerRail { … } with more controls than the drawer is tall",
        &[Node::Element {
            tag: "div",
            attributes: &[("class", Value::Text("nd-example-window"))],
            children: &[
                Node::Element {
                    tag: "div",
                    attributes: &[("class", Value::Text("nd-example-work"))],
                    children: &[Node::Component {
                        name: "Paragraph",
                        props: &[],
                        children: &[Node::Text(Value::Message(Key::ExampleDrawerWork))],
                    }],
                },
                Node::Component {
                    name: "Drawer",
                    props: &[
                        ("name", Value::Text("rail-tall")),
                        ("label", Value::Message(Key::ExampleDrawerLabel)),
                    ],
                    children: &[
                        Node::Component {
                            name: "DrawerRail",
                            props: &[("side", Value::Text("start"))],
                            children: &[Node::Component {
                                name: "Toolbar",
                                props: &[
                                    ("density", Value::Text("rail")),
                                    ("label", Value::Message(Key::ExampleRailControls)),
                                ],
                                children: &[
                                    Node::Component {
                                        name: "ToolbarGroup",
                                        props: &[],
                                        children: &[
                                            Node::Component {
                                                name: "Tooltip",
                                                props: &[
                                                    ("name", Value::Text("rail-tall-collapse")),
                                                    (
                                                        "text",
                                                        Value::Message(Key::ExampleRailCollapse),
                                                    ),
                                                ],
                                                children: &[Node::Component {
                                                    name: "Button",
                                                    props: &[
                                                        ("variant", Value::Text("transparent")),
                                                        ("tone", Value::Text("neutral")),
                                                        ("size", Value::Text("small")),
                                                        ("hide_label", Value::Bool(true)),
                                                        ("icon", Value::Text("chevrons-down")),
                                                        (
                                                            "label",
                                                            Value::Message(
                                                                Key::ExampleRailCollapse,
                                                            ),
                                                        ),
                                                    ],
                                                    children: &[],
                                                }],
                                            },
                                            Node::Component {
                                                name: "Tooltip",
                                                props: &[
                                                    ("name", Value::Text("rail-tall-close")),
                                                    ("text", Value::Message(Key::ExampleRailClose)),
                                                ],
                                                children: &[Node::Component {
                                                    name: "Button",
                                                    props: &[
                                                        ("variant", Value::Text("transparent")),
                                                        ("tone", Value::Text("neutral")),
                                                        ("size", Value::Text("small")),
                                                        ("hide_label", Value::Bool(true)),
                                                        ("icon", Value::Text("close")),
                                                        (
                                                            "label",
                                                            Value::Message(Key::ExampleRailClose),
                                                        ),
                                                    ],
                                                    children: &[],
                                                }],
                                            },
                                        ],
                                    },
                                    Node::Component {
                                        name: "ToolbarGroup",
                                        props: &[],
                                        children: &[
                                            Node::Component {
                                                name: "Tooltip",
                                                props: &[
                                                    ("name", Value::Text("rail-tall-direction")),
                                                    (
                                                        "text",
                                                        Value::Message(Key::ExampleRailDirection),
                                                    ),
                                                ],
                                                children: &[Node::Component {
                                                    name: "Button",
                                                    props: &[
                                                        ("variant", Value::Text("transparent")),
                                                        ("tone", Value::Text("neutral")),
                                                        ("size", Value::Text("small")),
                                                        ("hide_label", Value::Bool(true)),
                                                        ("icon", Value::Text("arrow-up-down")),
                                                        (
                                                            "label",
                                                            Value::Message(
                                                                Key::ExampleRailDirection,
                                                            ),
                                                        ),
                                                    ],
                                                    children: &[],
                                                }],
                                            },
                                            Node::Component {
                                                name: "Tooltip",
                                                props: &[
                                                    ("name", Value::Text("rail-tall-commands")),
                                                    (
                                                        "text",
                                                        Value::Message(Key::ExampleRailCommands),
                                                    ),
                                                ],
                                                children: &[Node::Component {
                                                    name: "Button",
                                                    props: &[
                                                        ("variant", Value::Text("transparent")),
                                                        ("tone", Value::Text("neutral")),
                                                        ("size", Value::Text("small")),
                                                        ("hide_label", Value::Bool(true)),
                                                        ("icon", Value::Text("terminal")),
                                                        (
                                                            "label",
                                                            Value::Message(
                                                                Key::ExampleRailCommands,
                                                            ),
                                                        ),
                                                    ],
                                                    children: &[],
                                                }],
                                            },
                                            Node::Component {
                                                name: "Tooltip",
                                                props: &[
                                                    ("name", Value::Text("rail-tall-settings")),
                                                    (
                                                        "text",
                                                        Value::Message(Key::ExampleRailSettings),
                                                    ),
                                                ],
                                                children: &[Node::Component {
                                                    name: "Button",
                                                    props: &[
                                                        ("variant", Value::Text("transparent")),
                                                        ("tone", Value::Text("neutral")),
                                                        ("size", Value::Text("small")),
                                                        ("hide_label", Value::Bool(true)),
                                                        ("icon", Value::Text("settings")),
                                                        (
                                                            "label",
                                                            Value::Message(
                                                                Key::ExampleRailSettings,
                                                            ),
                                                        ),
                                                    ],
                                                    children: &[],
                                                }],
                                            },
                                            Node::Component {
                                                name: "Tooltip",
                                                props: &[
                                                    ("name", Value::Text("rail-tall-registries")),
                                                    (
                                                        "text",
                                                        Value::Message(Key::ExampleRailRegistries),
                                                    ),
                                                ],
                                                children: &[Node::Component {
                                                    name: "Button",
                                                    props: &[
                                                        ("variant", Value::Text("transparent")),
                                                        ("tone", Value::Text("neutral")),
                                                        ("size", Value::Text("small")),
                                                        ("hide_label", Value::Bool(true)),
                                                        ("icon", Value::Text("database")),
                                                        (
                                                            "label",
                                                            Value::Message(
                                                                Key::ExampleRailRegistries,
                                                            ),
                                                        ),
                                                    ],
                                                    children: &[],
                                                }],
                                            },
                                            Node::Component {
                                                name: "Tooltip",
                                                props: &[
                                                    ("name", Value::Text("rail-tall-search")),
                                                    ("text", Value::Message(Key::SearchLabel)),
                                                ],
                                                children: &[Node::Component {
                                                    name: "Button",
                                                    props: &[
                                                        ("variant", Value::Text("transparent")),
                                                        ("tone", Value::Text("neutral")),
                                                        ("size", Value::Text("small")),
                                                        ("hide_label", Value::Bool(true)),
                                                        ("icon", Value::Text("search")),
                                                        ("label", Value::Message(Key::SearchLabel)),
                                                    ],
                                                    children: &[],
                                                }],
                                            },
                                            Node::Component {
                                                name: "Tooltip",
                                                props: &[
                                                    ("name", Value::Text("rail-tall-copy")),
                                                    ("text", Value::Message(Key::ExampleCopy)),
                                                ],
                                                children: &[Node::Component {
                                                    name: "Button",
                                                    props: &[
                                                        ("variant", Value::Text("transparent")),
                                                        ("tone", Value::Text("neutral")),
                                                        ("size", Value::Text("small")),
                                                        ("hide_label", Value::Bool(true)),
                                                        ("icon", Value::Text("copy")),
                                                        ("label", Value::Message(Key::ExampleCopy)),
                                                    ],
                                                    children: &[],
                                                }],
                                            },
                                            Node::Component {
                                                name: "Tooltip",
                                                props: &[
                                                    ("name", Value::Text("rail-tall-export")),
                                                    (
                                                        "text",
                                                        Value::Message(Key::ExampleTooltipExport),
                                                    ),
                                                ],
                                                children: &[Node::Component {
                                                    name: "Button",
                                                    props: &[
                                                        ("variant", Value::Text("transparent")),
                                                        ("tone", Value::Text("neutral")),
                                                        ("size", Value::Text("small")),
                                                        ("hide_label", Value::Bool(true)),
                                                        ("icon", Value::Text("download")),
                                                        (
                                                            "label",
                                                            Value::Message(
                                                                Key::ExampleTooltipExport,
                                                            ),
                                                        ),
                                                    ],
                                                    children: &[],
                                                }],
                                            },
                                            Node::Component {
                                                name: "Tooltip",
                                                props: &[
                                                    ("name", Value::Text("rail-tall-refresh")),
                                                    (
                                                        "text",
                                                        Value::Message(Key::ExampleTooltipRefresh),
                                                    ),
                                                ],
                                                children: &[Node::Component {
                                                    name: "Button",
                                                    props: &[
                                                        ("variant", Value::Text("transparent")),
                                                        ("tone", Value::Text("neutral")),
                                                        ("size", Value::Text("small")),
                                                        ("hide_label", Value::Bool(true)),
                                                        ("icon", Value::Text("refresh-cw")),
                                                        (
                                                            "label",
                                                            Value::Message(
                                                                Key::ExampleTooltipRefresh,
                                                            ),
                                                        ),
                                                    ],
                                                    children: &[],
                                                }],
                                            },
                                            Node::Component {
                                                name: "Tooltip",
                                                props: &[
                                                    ("name", Value::Text("rail-tall-preview")),
                                                    (
                                                        "text",
                                                        Value::Message(Key::ExampleTooltipLock),
                                                    ),
                                                ],
                                                children: &[Node::Component {
                                                    name: "Button",
                                                    props: &[
                                                        ("variant", Value::Text("transparent")),
                                                        ("tone", Value::Text("neutral")),
                                                        ("size", Value::Text("small")),
                                                        ("hide_label", Value::Bool(true)),
                                                        ("icon", Value::Text("eye")),
                                                        (
                                                            "label",
                                                            Value::Message(Key::ExampleTooltipLock),
                                                        ),
                                                    ],
                                                    children: &[],
                                                }],
                                            },
                                            Node::Component {
                                                name: "Tooltip",
                                                props: &[
                                                    ("name", Value::Text("rail-tall-light")),
                                                    ("text", Value::Message(Key::ThemeModeLight)),
                                                ],
                                                children: &[Node::Component {
                                                    name: "Button",
                                                    props: &[
                                                        ("variant", Value::Text("transparent")),
                                                        ("tone", Value::Text("neutral")),
                                                        ("size", Value::Text("small")),
                                                        ("hide_label", Value::Bool(true)),
                                                        ("icon", Value::Text("sun")),
                                                        (
                                                            "label",
                                                            Value::Message(Key::ThemeModeLight),
                                                        ),
                                                    ],
                                                    children: &[],
                                                }],
                                            },
                                            Node::Component {
                                                name: "Tooltip",
                                                props: &[
                                                    ("name", Value::Text("rail-tall-dark")),
                                                    ("text", Value::Message(Key::ThemeModeDark)),
                                                ],
                                                children: &[Node::Component {
                                                    name: "Button",
                                                    props: &[
                                                        ("variant", Value::Text("transparent")),
                                                        ("tone", Value::Text("neutral")),
                                                        ("size", Value::Text("small")),
                                                        ("hide_label", Value::Bool(true)),
                                                        ("icon", Value::Text("moon")),
                                                        (
                                                            "label",
                                                            Value::Message(Key::ThemeModeDark),
                                                        ),
                                                    ],
                                                    children: &[],
                                                }],
                                            },
                                            Node::Component {
                                                name: "Tooltip",
                                                props: &[
                                                    ("name", Value::Text("rail-tall-pages")),
                                                    ("text", Value::Message(Key::ExampleRailPages)),
                                                ],
                                                children: &[Node::Component {
                                                    name: "Button",
                                                    props: &[
                                                        ("variant", Value::Text("transparent")),
                                                        ("tone", Value::Text("neutral")),
                                                        ("size", Value::Text("small")),
                                                        ("hide_label", Value::Bool(true)),
                                                        ("icon", Value::Text("list")),
                                                        (
                                                            "label",
                                                            Value::Message(Key::ExampleRailPages),
                                                        ),
                                                    ],
                                                    children: &[],
                                                }],
                                            },
                                            Node::Component {
                                                name: "Tooltip",
                                                props: &[
                                                    ("name", Value::Text("rail-tall-keys")),
                                                    (
                                                        "text",
                                                        Value::Message(Key::ExampleRailControls),
                                                    ),
                                                ],
                                                children: &[Node::Component {
                                                    name: "Button",
                                                    props: &[
                                                        ("variant", Value::Text("transparent")),
                                                        ("tone", Value::Text("neutral")),
                                                        ("size", Value::Text("small")),
                                                        ("hide_label", Value::Bool(true)),
                                                        ("icon", Value::Text("keyboard")),
                                                        (
                                                            "label",
                                                            Value::Message(
                                                                Key::ExampleRailControls,
                                                            ),
                                                        ),
                                                    ],
                                                    children: &[],
                                                }],
                                            },
                                            Node::Component {
                                                name: "Tooltip",
                                                props: &[
                                                    ("name", Value::Text("rail-tall-sidebar")),
                                                    (
                                                        "text",
                                                        Value::Message(Key::ExampleFooterSidebar),
                                                    ),
                                                ],
                                                children: &[Node::Component {
                                                    name: "Button",
                                                    props: &[
                                                        ("variant", Value::Text("transparent")),
                                                        ("tone", Value::Text("neutral")),
                                                        ("size", Value::Text("small")),
                                                        ("hide_label", Value::Bool(true)),
                                                        ("icon", Value::Text("panel-left")),
                                                        (
                                                            "label",
                                                            Value::Message(
                                                                Key::ExampleFooterSidebar,
                                                            ),
                                                        ),
                                                    ],
                                                    children: &[],
                                                }],
                                            },
                                        ],
                                    },
                                ],
                            }],
                        },
                        Node::Element {
                            tag: "div",
                            attributes: &[("class", Value::Text("nd-example-pane"))],
                            children: &[Node::Component {
                                name: "Paragraph",
                                props: &[],
                                children: &[Node::Text(Value::Message(
                                    Key::ExampleRailCommandsBody,
                                ))],
                            }],
                        },
                    ],
                },
            ],
        }],
    ),
];
