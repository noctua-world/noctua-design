//! `Prose`, as trees.
//!
//! One entry per example in `examples::PROSE`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.

use noctua_design_i18n::Key;

use crate::tree::{Each, Node, Value};
use crate::trees::Tree;

/// Every `Prose` example. 5 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r"Prose { … }",
        &[Node::Component {
            name: "Prose",
            props: &[],
            children: &[
                Node::Component {
                    name: "Heading",
                    props: &[("level", Value::Text("3")), ("size", Value::Text("h3"))],
                    children: &[Node::Text(Value::Message(Key::ExampleHeadingSection))],
                },
                Node::Component {
                    name: "Paragraph",
                    props: &[],
                    children: &[Node::Text(Value::Message(Key::ExampleListLong))],
                },
                Node::Component {
                    name: "Paragraph",
                    props: &[],
                    children: &[Node::Text(Value::Message(Key::ExampleListLong))],
                },
            ],
        }],
    ),
    (
        r"Prose { measure: ProseMeasure::Lead, … }",
        &[Node::Component {
            name: "Prose",
            props: &[("measure", Value::Text("lead"))],
            children: &[Node::Component {
                name: "Paragraph",
                props: &[("size", Value::Text("lead"))],
                children: &[Node::Text(Value::Message(Key::ExampleListLong))],
            }],
        }],
    ),
    (
        r"Prose { measure: ProseMeasure::None, … }",
        &[Node::Component {
            name: "Prose",
            props: &[("measure", Value::Text("none"))],
            children: &[Node::Component {
                name: "Paragraph",
                props: &[],
                children: &[Node::Text(Value::Message(Key::ExampleListLong))],
            }],
        }],
    ),
    (
        r"A whole reading column",
        &[Node::Component {
            name: "Prose",
            props: &[],
            children: &[
                Node::Component {
                    name: "Heading",
                    props: &[("level", Value::Text("3")), ("size", Value::Text("h3"))],
                    children: &[Node::Text(Value::Message(Key::ExampleHeadingSection))],
                },
                Node::Component {
                    name: "Paragraph",
                    props: &[],
                    children: &[Node::Text(Value::Message(Key::ExampleListLong))],
                },
                Node::Component {
                    name: "List",
                    props: &[("spacing", Value::Text("tight"))],
                    children: &[
                        Node::Component {
                            name: "ListItem",
                            props: &[],
                            children: &[Node::Text(Value::Message(Key::ExampleListOne))],
                        },
                        Node::Component {
                            name: "ListItem",
                            props: &[],
                            children: &[Node::Text(Value::Message(Key::ExampleListTwo))],
                        },
                    ],
                },
                Node::Component {
                    name: "Blockquote",
                    props: &[],
                    children: &[Node::Component {
                        name: "Paragraph",
                        props: &[],
                        children: &[Node::Text(Value::Message(Key::ExampleQuoteShort))],
                    }],
                },
            ],
        }],
    ),
    // THE CLOSED MARKDOWN SUBSET, EVERY CONSTRUCT OF IT, as the interface draws
    // a help page: headings, paragraphs, emphasis, strong, inline code, a link,
    // a nested list, a quotation, a block of code, a table and a break. It is
    // the fixture a change to any of them is looked at against.
    (
        r"Prose { flow: true, … } — every construct of the Markdown subset",
        &[Node::Component {
            name: "Prose",
            props: &[("flow", Value::Bool(true))],
            children: &[
                // `#` and `##` are this column's third and fourth levels: the
                // page around it owns the first two.
                Node::Component {
                    name: "Heading",
                    props: &[("level", Value::Text("3")), ("size", Value::Text("h3"))],
                    children: &[Node::Text(Value::Message(Key::ExampleProseFlowHeading))],
                },
                // A paragraph, and inline code.
                Node::Component {
                    name: "Paragraph",
                    props: &[],
                    children: &[
                        Node::Text(Value::Join(&[
                            Value::Message(Key::ExampleProseFlowPress),
                            Value::Text(" "),
                        ])),
                        Node::Component {
                            name: "Code",
                            props: &[("class", Value::Text("nd-identifier"))],
                            children: &[Node::Text(Value::Data("Leader h"))],
                        },
                        Node::Text(Value::Data(".")),
                    ],
                },
                // Emphasis and strong.
                Node::Component {
                    name: "Paragraph",
                    props: &[],
                    children: &[
                        Node::Text(Value::Join(&[
                            Value::Message(Key::ExampleProseFlowQuick),
                            Value::Text(" "),
                        ])),
                        Node::Component {
                            name: "Em",
                            props: &[],
                            children: &[Node::Text(Value::Message(Key::ExampleProseFlowQuickEm))],
                        },
                        Node::Text(Value::Join(&[
                            Value::Data(". "),
                            Value::Message(Key::ExampleProseFlowEscape),
                            Value::Text(" "),
                        ])),
                        Node::Component {
                            name: "Strong",
                            props: &[],
                            children: &[Node::Text(Value::Message(
                                Key::ExampleProseFlowEscapeStrong,
                            ))],
                        },
                        Node::Text(Value::Data(".")),
                    ],
                },
                // A link.
                Node::Component {
                    name: "Paragraph",
                    props: &[],
                    children: &[
                        Node::Text(Value::Join(&[
                            Value::Message(Key::ExampleProseFlowLinkLead),
                            Value::Text(" "),
                        ])),
                        Node::Component {
                            name: "Link",
                            props: &[("href", Value::Text("../key-capture/"))],
                            children: &[Node::Text(Value::Message(Key::ExampleProseFlowLink))],
                        },
                        Node::Text(Value::Data(".")),
                    ],
                },
                Node::Component {
                    name: "Heading",
                    props: &[("level", Value::Text("4")), ("size", Value::Text("h4"))],
                    children: &[Node::Text(Value::Message(Key::ExampleProseFlowMenu))],
                },
                // A list, with a list inside one of its items.
                Node::Component {
                    name: "List",
                    props: &[],
                    children: &[
                        Node::Component {
                            name: "ListItem",
                            props: &[],
                            children: &[
                                Node::Component {
                                    name: "Code",
                                    props: &[("class", Value::Text("nd-identifier"))],
                                    children: &[Node::Text(Value::Data("h"))],
                                },
                                Node::Text(Value::Join(&[
                                    Value::Text(" "),
                                    Value::Message(Key::ExampleProseFlowHelp),
                                ])),
                            ],
                        },
                        Node::Component {
                            name: "ListItem",
                            props: &[],
                            children: &[
                                Node::Component {
                                    name: "Code",
                                    props: &[("class", Value::Text("nd-identifier"))],
                                    children: &[Node::Text(Value::Data("d"))],
                                },
                                Node::Text(Value::Join(&[
                                    Value::Text(" "),
                                    Value::Message(Key::ExampleProseFlowDev),
                                ])),
                                Node::Component {
                                    name: "List",
                                    props: &[],
                                    children: &[
                                        Node::Component {
                                            name: "ListItem",
                                            props: &[],
                                            children: &[Node::Text(Value::Message(
                                                Key::ExampleProseFlowLogs,
                                            ))],
                                        },
                                        Node::Component {
                                            name: "ListItem",
                                            props: &[],
                                            children: &[Node::Text(Value::Message(
                                                Key::ExampleProseFlowRegistries,
                                            ))],
                                        },
                                    ],
                                },
                            ],
                        },
                        Node::Component {
                            name: "ListItem",
                            props: &[],
                            children: &[
                                Node::Component {
                                    name: "Code",
                                    props: &[("class", Value::Text("nd-identifier"))],
                                    children: &[Node::Text(Value::Data("q"))],
                                },
                                Node::Text(Value::Join(&[
                                    Value::Text(" "),
                                    Value::Message(Key::ExampleProseFlowClose),
                                ])),
                            ],
                        },
                    ],
                },
                // A quotation.
                Node::Component {
                    name: "Blockquote",
                    props: &[],
                    children: &[Node::Component {
                        name: "Paragraph",
                        props: &[],
                        children: &[Node::Text(Value::Message(Key::ExampleProseFlowQuote))],
                    }],
                },
                // A fenced block of code.
                Node::Component {
                    name: "CodeBlock",
                    props: &[
                        (
                            "code",
                            Value::Data(
                                "[leader]\nchord = \"Alt+Super+Space\"\nmenu = [\"h\", \"d\", \"q\"]",
                            ),
                        ),
                        ("language", Value::Text("toml")),
                    ],
                    children: &[],
                },
                // A table.
                Node::Component {
                    name: "Table",
                    props: &[
                        ("name", Value::Text("prose-keys")),
                        ("caption", Value::Message(Key::ExampleProseFlowCaption)),
                    ],
                    children: &[
                        Node::Component {
                            name: "TableColumn",
                            props: &[("name", Value::Text("key")), ("width", Value::Text("fit"))],
                            children: &[],
                        },
                        Node::Component {
                            name: "TableColumn",
                            props: &[("name", Value::Text("opens"))],
                            children: &[],
                        },
                        Node::Component {
                            name: "TableSection",
                            props: &[("kind", Value::Text("head"))],
                            children: &[Node::Component {
                                name: "TableRow",
                                props: &[],
                                children: &[
                                    Node::Component {
                                        name: "TableHeaderCell",
                                        props: &[("column", Value::Text("key"))],
                                        children: &[Node::Text(Value::Message(
                                            Key::ExampleProseFlowColKey,
                                        ))],
                                    },
                                    Node::Component {
                                        name: "TableHeaderCell",
                                        props: &[("column", Value::Text("opens"))],
                                        children: &[Node::Text(Value::Message(
                                            Key::ExampleProseFlowColOpens,
                                        ))],
                                    },
                                ],
                            }],
                        },
                        Node::Component {
                            name: "TableSection",
                            props: &[],
                            children: &[Node::Each {
                                over: Each::Rows(&[
                                    &[
                                        Value::Data("h"),
                                        Value::Message(Key::ExampleProseFlowHelpPage),
                                    ],
                                    &[
                                        Value::Data("d"),
                                        Value::Message(Key::ExampleProseFlowDevPages),
                                    ],
                                    &[
                                        Value::Data("q"),
                                        Value::Message(Key::ExampleProseFlowClosePage),
                                    ],
                                ]),
                                body: &[Node::Component {
                                    name: "TableRow",
                                    props: &[],
                                    children: &[
                                        Node::Component {
                                            name: "TableHeaderCell",
                                            props: &[
                                                ("scope", Value::Text("row")),
                                                ("column", Value::Text("key")),
                                                ("class", Value::Text("nd-identifier")),
                                            ],
                                            children: &[Node::Text(Value::Bound(0))],
                                        },
                                        Node::Component {
                                            name: "TableCell",
                                            props: &[("column", Value::Text("opens"))],
                                            children: &[Node::Text(Value::Bound(1))],
                                        },
                                    ],
                                }],
                            }],
                        },
                    ],
                },
                // A break, and the paragraph after it.
                Node::Component {
                    name: "Divider",
                    props: &[],
                    children: &[],
                },
                Node::Component {
                    name: "Paragraph",
                    props: &[],
                    children: &[Node::Text(Value::Message(Key::ExampleProseFlowEnd))],
                },
            ],
        }],
    ),
];
