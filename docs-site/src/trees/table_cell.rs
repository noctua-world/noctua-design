//! `TableCell`, as trees.
//!
//! One entry per example in `examples::TABLE_CELL`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.
//!
//! The invoice parts come from `super::table`, where the macros these
//! examples shared were hoisted to.

use noctua_design_i18n::Key;

use crate::tree::{Each, Node, Value};
use crate::trees::Tree;
use crate::trees::table::{AURORA, BRACKEN, CINTRA, INVOICE_HEADINGS, INVOICE_ROW};

/// Every `TableCell` example. 6 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"TableCell { align: ::Numeric, sort_value: "…" }"#,
        &[Node::Component {
            name: "Table",
            props: &[
                ("name", Value::Text("cell-numeric")),
                ("caption", Value::Message(Key::ExampleTableInvoices)),
                (
                    "sort_ascending_label",
                    Value::Message(Key::ExampleTableSortAscending),
                ),
                (
                    "sort_descending_label",
                    Value::Message(Key::ExampleTableSortDescending),
                ),
                (
                    "sort_cleared_label",
                    Value::Message(Key::ExampleTableSortCleared),
                ),
            ],
            children: &[
                Node::Component {
                    name: "TableColumn",
                    props: &[("name", Value::Text("client"))],
                    children: &[],
                },
                Node::Component {
                    name: "TableColumn",
                    props: &[
                        ("name", Value::Text("amount")),
                        ("width", Value::Text("fit")),
                        ("sortable", Value::Bool(true)),
                        ("sort_as", Value::Text("number")),
                    ],
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
                                props: &[("column", Value::Text("client"))],
                                children: &[Node::Text(Value::Message(Key::ExampleTableColClient))],
                            },
                            Node::Component {
                                name: "TableHeaderCell",
                                props: &[
                                    ("column", Value::Text("amount")),
                                    ("align", Value::Text("numeric")),
                                ],
                                children: &[Node::Text(Value::Message(Key::ExampleTableColAmount))],
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
                                Value::Data("Aurora Ltd"),
                                Value::Data("R$ 4.900,00"),
                                Value::Text("4900"),
                            ],
                            &[
                                Value::Data("Bracken & Co"),
                                Value::Data("R$ 1.240,00"),
                                Value::Text("1240"),
                            ],
                            &[
                                Value::Data("Cintra Studio"),
                                Value::Data("R$ 680,00"),
                                Value::Text("680"),
                            ],
                            &[
                                Value::Data("Delgado SA"),
                                Value::Data("R$ 12.310,00"),
                                Value::Text("12310"),
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
                                        ("column", Value::Text("client")),
                                        ("class", Value::Text("nd-identifier")),
                                    ],
                                    children: &[Node::Text(Value::Bound(0))],
                                },
                                Node::Component {
                                    name: "TableCell",
                                    props: &[
                                        ("column", Value::Text("amount")),
                                        ("class", Value::Text("nd-identifier")),
                                        ("align", Value::Text("numeric")),
                                        ("sort_value", Value::Bound(2)),
                                    ],
                                    children: &[Node::Text(Value::Bound(1))],
                                },
                            ],
                        }],
                    }],
                },
            ],
        }],
    ),
    (
        r"TableCell { wrap: ::Nowrap } beside TableCell { wrap: ::Truncate }",
        &[Node::Component {
            name: "Table",
            props: &[
                ("name", Value::Text("cell-wrap")),
                ("caption", Value::Message(Key::ExampleTableDeploys)),
            ],
            children: &[
                Node::Component {
                    name: "TableColumn",
                    props: &[
                        ("name", Value::Text("branch")),
                        ("width", Value::Text("fit")),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "TableColumn",
                    props: &[
                        ("name", Value::Text("log")),
                        ("width", Value::Text("medium")),
                    ],
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
                                props: &[("column", Value::Text("branch"))],
                                children: &[Node::Text(Value::Message(Key::ExampleTableColBranch))],
                            },
                            Node::Component {
                                name: "TableHeaderCell",
                                props: &[("column", Value::Text("log"))],
                                children: &[Node::Text(Value::Message(Key::ExampleTableColResult))],
                            },
                        ],
                    }],
                },
                Node::Component {
                    name: "TableSection",
                    props: &[],
                    children: &[Node::Component {
                        name: "TableRow",
                        props: &[],
                        children: &[
                            Node::Component {
                                name: "TableCell",
                                props: &[
                                    ("column", Value::Text("branch")),
                                    ("wrap", Value::Text("nowrap")),
                                    ("class", Value::Text("nd-identifier")),
                                ],
                                children: &[Node::Text(Value::Data("release/2026-03-candidate"))],
                            },
                            Node::Component {
                                name: "TableCell",
                                props: &[
                                    ("column", Value::Text("log")),
                                    ("wrap", Value::Text("truncate")),
                                ],
                                children: &[Node::Text(Value::Message(Key::ExampleTableLog))],
                            },
                        ],
                    }],
                },
            ],
        }],
    ),
    (
        r#"TableCell { span: "2", rows: "2", column: "…" }"#,
        &[Node::Component {
            name: "Table",
            props: &[
                ("name", Value::Text("cell-span")),
                ("caption", Value::Message(Key::ExampleTableDeploys)),
                ("rules", Value::Text("grid")),
            ],
            children: &[
                Node::Component {
                    name: "TableColumn",
                    props: &[
                        ("name", Value::Text("build")),
                        ("width", Value::Text("fit")),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "TableColumn",
                    props: &[
                        ("name", Value::Text("where")),
                        ("width", Value::Text("fit")),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "TableColumn",
                    props: &[("name", Value::Text("result"))],
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
                                props: &[("column", Value::Text("build"))],
                                children: &[Node::Text(Value::Message(Key::ExampleTableColBuild))],
                            },
                            Node::Component {
                                name: "TableHeaderCell",
                                props: &[("column", Value::Text("where"))],
                                children: &[Node::Text(Value::Message(Key::ExampleTableColWhere))],
                            },
                            Node::Component {
                                name: "TableHeaderCell",
                                props: &[("column", Value::Text("result"))],
                                children: &[Node::Text(Value::Message(Key::ExampleTableColResult))],
                            },
                        ],
                    }],
                },
                Node::Component {
                    name: "TableSection",
                    props: &[],
                    children: &[
                        Node::Component {
                            name: "TableRow",
                            props: &[],
                            children: &[
                                Node::Component {
                                    name: "TableHeaderCell",
                                    props: &[
                                        ("scope", Value::Text("row")),
                                        ("column", Value::Text("build")),
                                        ("rows", Value::Text("2")),
                                        ("class", Value::Text("nd-identifier")),
                                    ],
                                    children: &[
                                        Node::Text(Value::Data("#")),
                                        Node::Text(Value::Data("4213")),
                                    ],
                                },
                                Node::Component {
                                    name: "TableCell",
                                    props: &[("column", Value::Text("where"))],
                                    children: &[Node::Text(Value::Message(
                                        Key::ExampleTableWhereStaging,
                                    ))],
                                },
                                Node::Component {
                                    name: "TableCell",
                                    props: &[("column", Value::Text("result"))],
                                    children: &[Node::Component {
                                        name: "Tag",
                                        props: &[
                                            (
                                                "label",
                                                Value::Message(Key::ExampleTableResultPassing),
                                            ),
                                            ("tone", Value::Text("passing")),
                                            ("size", Value::Text("small")),
                                        ],
                                        children: &[],
                                    }],
                                },
                            ],
                        },
                        Node::Component {
                            name: "TableRow",
                            props: &[],
                            children: &[Node::Component {
                                name: "TableCell",
                                props: &[
                                    ("span", Value::Text("2")),
                                    ("column", Value::Text("where")),
                                ],
                                children: &[Node::Text(Value::Message(Key::ExampleTableLog))],
                            }],
                        },
                    ],
                },
            ],
        }],
    ),
    (
        r#"TableCell { label, tone: "…" } — the stacked layout's own label"#,
        &[Node::Component {
            name: "Table",
            props: &[
                ("name", Value::Text("cell-label")),
                ("caption", Value::Message(Key::ExampleTableTeam)),
                ("layout", Value::Text("stack")),
            ],
            children: &[
                Node::Component {
                    name: "TableColumn",
                    props: &[("name", Value::Text("person"))],
                    children: &[],
                },
                Node::Component {
                    name: "TableColumn",
                    props: &[("name", Value::Text("role")), ("width", Value::Text("fit"))],
                    children: &[],
                },
                Node::Component {
                    name: "TableColumn",
                    props: &[
                        ("name", Value::Text("access")),
                        ("width", Value::Text("fit")),
                    ],
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
                                props: &[("column", Value::Text("person"))],
                                children: &[Node::Text(Value::Message(Key::ExampleTableColPerson))],
                            },
                            Node::Component {
                                name: "TableHeaderCell",
                                props: &[("column", Value::Text("role"))],
                                children: &[Node::Text(Value::Message(Key::ExampleTableColRole))],
                            },
                            Node::Component {
                                name: "TableHeaderCell",
                                props: &[("column", Value::Text("access"))],
                                children: &[Node::Text(Value::Message(Key::ExampleTableColAccess))],
                            },
                        ],
                    }],
                },
                Node::Component {
                    name: "TableSection",
                    props: &[],
                    children: &[Node::Component {
                        name: "TableRow",
                        props: &[],
                        children: &[
                            Node::Component {
                                name: "TableHeaderCell",
                                props: &[
                                    ("scope", Value::Text("row")),
                                    ("column", Value::Text("person")),
                                    ("class", Value::Text("nd-identifier")),
                                ],
                                children: &[Node::Text(Value::Data("Grace Hopper"))],
                            },
                            Node::Component {
                                name: "TableCell",
                                props: &[
                                    ("column", Value::Text("role")),
                                    ("label", Value::Message(Key::ExampleTableColRole)),
                                ],
                                children: &[Node::Component {
                                    name: "Tag",
                                    props: &[
                                        ("label", Value::Message(Key::ExampleTableRoleMember)),
                                        ("tone", Value::Text("member")),
                                        ("variant", Value::Text("outline")),
                                        ("size", Value::Text("small")),
                                    ],
                                    children: &[],
                                }],
                            },
                            Node::Component {
                                name: "TableCell",
                                props: &[
                                    ("column", Value::Text("access")),
                                    ("label", Value::Message(Key::ExampleTableColAccess)),
                                    ("tone", Value::Text("muted")),
                                ],
                                children: &[Node::Text(Value::Message(
                                    Key::ExampleTableAccessWeek,
                                ))],
                            },
                        ],
                    }],
                },
            ],
        }],
    ),
    (
        r"Table { name, caption, search_label, search_placeholder, filtered_label }
TableCell { aggregate, aggregate_over }: a total that recounts",
        &[Node::Component {
            name: "Table",
            props: &[
                ("name", Value::Text("cell-total")),
                ("caption", Value::Message(Key::ExampleTableInvoices)),
                ("search_label", Value::Message(Key::ExampleTableSearch)),
                (
                    "search_placeholder",
                    Value::Message(Key::ExampleTableSearchPlaceholder),
                ),
                (
                    "search_clear_label",
                    Value::Message(Key::ExampleTableSearchClear),
                ),
                ("filtered_label", Value::Message(Key::ExampleTableFiltered)),
            ],
            children: &[
                Node::Component {
                    name: "TableColumn",
                    props: &[
                        ("name", Value::Text("invoice")),
                        ("width", Value::Text("fit")),
                        ("filterable", Value::Bool(true)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "TableColumn",
                    props: &[
                        ("name", Value::Text("client")),
                        ("filterable", Value::Bool(true)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "TableColumn",
                    props: &[("name", Value::Text("due")), ("width", Value::Text("fit"))],
                    children: &[],
                },
                Node::Component {
                    name: "TableColumn",
                    props: &[
                        ("name", Value::Text("status")),
                        ("width", Value::Text("fit")),
                        ("filterable", Value::Bool(true)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "TableColumn",
                    props: &[
                        ("name", Value::Text("amount")),
                        ("width", Value::Text("fit")),
                    ],
                    children: &[],
                },
                Node::Fragment(INVOICE_HEADINGS),
                Node::Component {
                    name: "TableSection",
                    props: &[],
                    children: &[
                        Node::Each {
                            over: Each::Rows(&[AURORA, BRACKEN, CINTRA]),
                            body: INVOICE_ROW,
                        },
                        Node::Component {
                            name: "TableRow",
                            props: &[("empty", Value::Bool(true))],
                            children: &[Node::Component {
                                name: "TableCell",
                                props: &[("span", Value::Text("5"))],
                                children: &[Node::Text(Value::Message(Key::ExampleTableEmpty))],
                            }],
                        },
                    ],
                },
                Node::Component {
                    name: "TableSection",
                    props: &[("kind", Value::Text("foot"))],
                    children: &[Node::Component {
                        name: "TableRow",
                        props: &[("emphasis", Value::Text("strong"))],
                        children: &[
                            Node::Component {
                                name: "TableHeaderCell",
                                props: &[
                                    ("scope", Value::Text("row")),
                                    ("column", Value::Text("invoice")),
                                    ("span", Value::Text("4")),
                                ],
                                children: &[Node::Text(Value::Message(Key::ExampleTableTotal))],
                            },
                            Node::Component {
                                name: "TableCell",
                                props: &[
                                    ("column", Value::Text("amount")),
                                    ("align", Value::Text("numeric")),
                                    ("aggregate", Value::Text("sum")),
                                    ("class", Value::Text("nd-identifier")),
                                ],
                                children: &[Node::Text(Value::Data("R$ 6.820,00"))],
                            },
                        ],
                    }],
                },
            ],
        }],
    ),
    (
        r"TableCell { ai_mark, ai_label }",
        &[Node::Component {
            name: "Table",
            props: &[
                ("name", Value::Text("cell-numeric-ai")),
                ("caption", Value::Message(Key::ExampleTableInvoices)),
                (
                    "sort_ascending_label",
                    Value::Message(Key::ExampleTableSortAscending),
                ),
                (
                    "sort_descending_label",
                    Value::Message(Key::ExampleTableSortDescending),
                ),
                (
                    "sort_cleared_label",
                    Value::Message(Key::ExampleTableSortCleared),
                ),
            ],
            children: &[
                Node::Component {
                    name: "TableColumn",
                    props: &[("name", Value::Text("client-ai"))],
                    children: &[],
                },
                Node::Component {
                    name: "TableColumn",
                    props: &[
                        ("name", Value::Text("amount-ai")),
                        ("width", Value::Text("fit")),
                        ("sortable", Value::Bool(true)),
                        ("sort_as", Value::Text("number")),
                    ],
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
                                props: &[("column", Value::Text("client"))],
                                children: &[Node::Text(Value::Message(Key::ExampleTableColClient))],
                            },
                            Node::Component {
                                name: "TableHeaderCell",
                                props: &[
                                    ("column", Value::Text("amount")),
                                    ("align", Value::Text("numeric")),
                                ],
                                children: &[Node::Text(Value::Message(Key::ExampleTableColAmount))],
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
                                Value::Data("Aurora Ltd"),
                                Value::Data("R$ 4.900,00"),
                                Value::Text("4900"),
                            ],
                            &[
                                Value::Data("Bracken & Co"),
                                Value::Data("R$ 1.240,00"),
                                Value::Text("1240"),
                            ],
                            &[
                                Value::Data("Cintra Studio"),
                                Value::Data("R$ 680,00"),
                                Value::Text("680"),
                            ],
                            &[
                                Value::Data("Delgado SA"),
                                Value::Data("R$ 12.310,00"),
                                Value::Text("12310"),
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
                                        ("column", Value::Text("client")),
                                        ("class", Value::Text("nd-identifier")),
                                    ],
                                    children: &[Node::Text(Value::Bound(0))],
                                },
                                Node::Component {
                                    name: "TableCell",
                                    props: &[
                                        ("ai_mark", Value::Message(Key::ExampleAiMark)),
                                        ("ai_label", Value::Message(Key::ExampleAiName)),
                                        ("column", Value::Text("amount")),
                                        ("class", Value::Text("nd-identifier")),
                                        ("align", Value::Text("numeric")),
                                        ("sort_value", Value::Bound(2)),
                                    ],
                                    children: &[Node::Text(Value::Bound(1))],
                                },
                            ],
                        }],
                    }],
                },
            ],
        }],
    ),
];
