//! `Table`, as trees.
//!
//! One entry per example in `examples::TABLE`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.
//!
//! # Why the parts below are shared
//!
//! `examples.rs` builds every table on this page from six `macro_rules!`, so
//! that the columns four tables declare and the row a dozen tables draw could
//! not drift apart by accident. A tree with no fragment would write each of
//! them out again per example and lose exactly that, so each macro is a
//! `const` here and the five sibling files splice it.

use noctua_design_i18n::Key;

use crate::tree::{Each, Node, Value};
use crate::trees::Tree;

/// The five columns the invoice tables share.
pub(crate) const INVOICE_COLUMNS: &[Node] = &[
    Node::Component {
        name: "TableColumn",
        props: &[
            ("name", Value::Text("invoice")),
            ("width", Value::Text("fit")),
        ],
        children: &[],
    },
    Node::Component {
        name: "TableColumn",
        props: &[("name", Value::Text("client"))],
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
];

/// The heading row the invoice tables share.
pub(crate) const INVOICE_HEADINGS: &[Node] = &[Node::Component {
    name: "TableSection",
    props: &[("kind", Value::Text("head"))],
    children: &[Node::Component {
        name: "TableRow",
        props: &[],
        children: &[
            Node::Component {
                name: "TableHeaderCell",
                props: &[("column", Value::Text("invoice"))],
                children: &[Node::Text(Value::Message(Key::ExampleTableColInvoice))],
            },
            Node::Component {
                name: "TableHeaderCell",
                props: &[("column", Value::Text("client"))],
                children: &[Node::Text(Value::Message(Key::ExampleTableColClient))],
            },
            Node::Component {
                name: "TableHeaderCell",
                props: &[("column", Value::Text("due"))],
                children: &[Node::Text(Value::Message(Key::ExampleTableColDue))],
            },
            Node::Component {
                name: "TableHeaderCell",
                props: &[("column", Value::Text("status"))],
                children: &[Node::Text(Value::Message(Key::ExampleTableColStatus))],
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
}];

/// One invoice, since a dozen rows differ only in five strings.
///
/// The columns are the ones [`AURORA`] and its neighbours are written in.
pub(crate) const INVOICE_ROW: &[Node] = &[Node::Component {
    name: "TableRow",
    props: &[],
    children: &[
        Node::Component {
            name: "TableHeaderCell",
            props: &[
                ("scope", Value::Text("row")),
                ("column", Value::Text("invoice")),
                ("class", Value::Text("nd-identifier")),
            ],
            children: &[Node::Text(Value::Bound(0))],
        },
        // EVERY CELL CARRIES ITS COLUMN'S NAME. The stacked layout has no
        // heading row left, and a value with no label under it is a run of
        // data a reader has to guess the shape of.
        Node::Component {
            name: "TableCell",
            props: &[
                ("column", Value::Text("client")),
                ("class", Value::Text("nd-identifier")),
                ("label", Value::Message(Key::ExampleTableColClient)),
            ],
            children: &[Node::Text(Value::Bound(1))],
        },
        Node::Component {
            name: "TableCell",
            props: &[
                ("column", Value::Text("due")),
                ("class", Value::Text("nd-identifier")),
                ("label", Value::Message(Key::ExampleTableColDue)),
                ("sort_value", Value::Bound(3)),
            ],
            children: &[Node::Text(Value::Bound(2))],
        },
        Node::Component {
            name: "TableCell",
            props: &[
                ("column", Value::Text("status")),
                ("label", Value::Message(Key::ExampleTableColStatus)),
            ],
            children: &[Node::Component {
                name: "Tag",
                props: &[
                    ("label", Value::Bound(4)),
                    ("tone", Value::Bound(5)),
                    ("size", Value::Text("small")),
                ],
                children: &[],
            }],
        },
        Node::Component {
            name: "TableCell",
            props: &[
                ("column", Value::Text("amount")),
                ("class", Value::Text("nd-identifier")),
                ("label", Value::Message(Key::ExampleTableColAmount)),
                ("align", Value::Text("numeric")),
                ("sort_value", Value::Bound(7)),
            ],
            children: &[Node::Text(Value::Bound(6))],
        },
    ],
}];

/// The paid invoice, in [`INVOICE_ROW`]'s columns.
pub(crate) const AURORA: &[Value] = &[
    Value::Data("INV-0043"),
    Value::Data("Aurora Ltd"),
    Value::Data("3 Mar"),
    Value::Text("2026-03-03"),
    Value::Message(Key::ExampleTableStatusPaid),
    Value::Text("paid"),
    Value::Data("R$ 4.900,00"),
    Value::Text("4900"),
];

/// The overdue invoice.
pub(crate) const BRACKEN: &[Value] = &[
    Value::Data("INV-0044"),
    Value::Data("Bracken & Co"),
    Value::Data("11 Mar"),
    Value::Text("2026-03-11"),
    Value::Message(Key::ExampleTableStatusOverdue),
    Value::Text("overdue"),
    Value::Data("R$ 1.240,00"),
    Value::Text("1240"),
];

/// The pending invoice.
pub(crate) const CINTRA: &[Value] = &[
    Value::Data("INV-0045"),
    Value::Data("Cintra Studio"),
    Value::Data("24 Mar"),
    Value::Text("2026-03-24"),
    Value::Message(Key::ExampleTableStatusPending),
    Value::Text("pending"),
    Value::Data("R$ 680,00"),
    Value::Text("680"),
];

/// The refunded invoice.
pub(crate) const DELGADO: &[Value] = &[
    Value::Data("INV-0046"),
    Value::Data("Delgado SA"),
    Value::Data("29 Mar"),
    Value::Text("2026-03-29"),
    Value::Message(Key::ExampleTableStatusRefunded),
    Value::Text("refunded"),
    Value::Data("R$ 320,00"),
    Value::Text("320"),
];

/// The five columns the deployment tables share.
pub(crate) const DEPLOY_COLUMNS: &[Node] = &[
    // NO `filterable` HERE. It says which columns the search box reads, and
    // **the box is what `Table { search_label }` adds**. None of the seven
    // tables built from these columns passes one, so the flag reached nothing
    // at all on any of them.
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
        props: &[("name", Value::Text("branch"))],
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
        props: &[("name", Value::Text("took")), ("width", Value::Text("fit"))],
        children: &[],
    },
    Node::Component {
        name: "TableColumn",
        props: &[
            ("name", Value::Text("result")),
            ("width", Value::Text("fit")),
        ],
        children: &[],
    },
];

/// The heading row the deployment tables share.
pub(crate) const DEPLOY_HEADINGS: &[Node] = &[Node::Component {
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
                props: &[("column", Value::Text("branch"))],
                children: &[Node::Text(Value::Message(Key::ExampleTableColBranch))],
            },
            Node::Component {
                name: "TableHeaderCell",
                props: &[("column", Value::Text("where"))],
                children: &[Node::Text(Value::Message(Key::ExampleTableColWhere))],
            },
            Node::Component {
                name: "TableHeaderCell",
                props: &[
                    ("column", Value::Text("took")),
                    ("align", Value::Text("numeric")),
                ],
                children: &[Node::Text(Value::Message(Key::ExampleTableColTook))],
            },
            Node::Component {
                name: "TableHeaderCell",
                props: &[("column", Value::Text("result"))],
                children: &[Node::Text(Value::Message(Key::ExampleTableColResult))],
            },
        ],
    }],
}];

/// One deployment, since a dozen rows differ only in five strings.
///
/// The columns are the ones [`PASSING_4213`] and its neighbours are written in.
pub(crate) const DEPLOY_ROW: &[Node] = &[Node::Component {
    name: "TableRow",
    props: &[],
    children: &[
        Node::Component {
            name: "TableHeaderCell",
            props: &[
                ("scope", Value::Text("row")),
                ("column", Value::Text("build")),
                ("class", Value::Text("nd-identifier")),
            ],
            children: &[Node::Text(Value::Data("#")), Node::Text(Value::Bound(0))],
        },
        // See `INVOICE_ROW`: every cell carries its column's name, for the
        // stacked layout that has no heading row left.
        Node::Component {
            name: "TableCell",
            props: &[
                ("column", Value::Text("branch")),
                ("class", Value::Text("nd-identifier")),
                ("label", Value::Message(Key::ExampleTableColBranch)),
                ("wrap", Value::Text("nowrap")),
            ],
            children: &[Node::Text(Value::Bound(1))],
        },
        Node::Component {
            name: "TableCell",
            props: &[
                ("column", Value::Text("where")),
                ("label", Value::Message(Key::ExampleTableColWhere)),
            ],
            children: &[Node::Text(Value::Bound(2))],
        },
        Node::Component {
            name: "TableCell",
            props: &[
                ("column", Value::Text("took")),
                ("class", Value::Text("nd-identifier")),
                ("label", Value::Message(Key::ExampleTableColTook)),
                ("align", Value::Text("numeric")),
            ],
            children: &[Node::Text(Value::Bound(3))],
        },
        Node::Component {
            name: "TableCell",
            props: &[
                ("column", Value::Text("result")),
                ("label", Value::Message(Key::ExampleTableColResult)),
            ],
            children: &[Node::Component {
                name: "Tag",
                props: &[
                    ("label", Value::Bound(4)),
                    ("tone", Value::Bound(5)),
                    ("size", Value::Text("small")),
                ],
                children: &[],
            }],
        },
    ],
}];

/// The build that passed, in [`DEPLOY_ROW`]'s columns.
pub(crate) const PASSING_4213: &[Value] = &[
    Value::Data("4213"),
    Value::Data("main"),
    Value::Message(Key::ExampleTableWhereProduction),
    Value::Data("51s"),
    Value::Message(Key::ExampleTableResultPassing),
    Value::Text("passing"),
];

/// The build still running, from `main`.
pub(crate) const RUNNING_4212: &[Value] = &[
    Value::Data("4212"),
    Value::Data("main"),
    Value::Message(Key::ExampleTableWhereStaging),
    Value::Data("48s"),
    Value::Message(Key::ExampleTableResultRunning),
    Value::Text("running"),
];

/// The same build, from the release branch that shows a pinned column moving.
pub(crate) const RUNNING_4212_RELEASE: &[Value] = &[
    Value::Data("4212"),
    Value::Data("release/2026-03"),
    Value::Message(Key::ExampleTableWhereStaging),
    Value::Data("48s"),
    Value::Message(Key::ExampleTableResultRunning),
    Value::Text("running"),
];

/// The build that failed.
pub(crate) const FAILED_4211: &[Value] = &[
    Value::Data("4211"),
    Value::Data("fix/anchor"),
    Value::Message(Key::ExampleTableWhereStaging),
    Value::Data("12s"),
    Value::Message(Key::ExampleTableResultFailed),
    Value::Text("failed"),
];

/// The build somebody stopped.
pub(crate) const CANCELED_4210: &[Value] = &[
    Value::Data("4210"),
    Value::Data("main"),
    Value::Message(Key::ExampleTableWhereProduction),
    Value::Data("1m 04s"),
    Value::Message(Key::ExampleTableResultCanceled),
    Value::Text("canceled"),
];

/// The build that came up degraded.
pub(crate) const DEGRADED_4210: &[Value] = &[
    Value::Data("4210"),
    Value::Data("main"),
    Value::Message(Key::ExampleTableWhereProduction),
    Value::Data("1m 04s"),
    Value::Message(Key::ExampleTableResultDegraded),
    Value::Text("degraded"),
];

/// The same build, passing, for the tables that show a row tone instead.
pub(crate) const PASSING_4210: &[Value] = &[
    Value::Data("4210"),
    Value::Data("main"),
    Value::Message(Key::ExampleTableWhereProduction),
    Value::Data("1m 04s"),
    Value::Message(Key::ExampleTableResultPassing),
    Value::Text("passing"),
];

/// The build still waiting.
pub(crate) const QUEUED_4209: &[Value] = &[
    Value::Data("4209"),
    Value::Data("main"),
    Value::Message(Key::ExampleTableWhereProduction),
    Value::Data("0s"),
    Value::Message(Key::ExampleTableResultQueued),
    Value::Text("queued"),
];

/// The build that closes yesterday's group.
pub(crate) const PASSING_4209: &[Value] = &[
    Value::Data("4209"),
    Value::Data("main"),
    Value::Message(Key::ExampleTableWhereProduction),
    Value::Data("58s"),
    Value::Message(Key::ExampleTableResultPassing),
    Value::Text("passing"),
];

/// Every `Table` example. 13 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r"Table { name, caption, note } with a Tag in a cell",
        &[Node::Component {
            name: "Table",
            props: &[
                ("name", Value::Text("invoices")),
                ("caption", Value::Message(Key::ExampleTableInvoices)),
                ("note", Value::Message(Key::ExampleTableInvoicesNote)),
            ],
            children: &[
                Node::Fragment(INVOICE_COLUMNS),
                Node::Fragment(INVOICE_HEADINGS),
                Node::Component {
                    name: "TableSection",
                    props: &[],
                    children: &[Node::Each {
                        over: Each::Rows(&[AURORA, BRACKEN, CINTRA]),
                        body: INVOICE_ROW,
                    }],
                },
            ],
        }],
    ),
    (
        r#"TableColumn { low, tone_below, tone_within } and TableCell { tone: "…" }"#,
        &[Node::Component {
            name: "Table",
            props: &[
                ("name", Value::Text("balances")),
                ("caption", Value::Message(Key::ExampleTableInvoices)),
            ],
            children: &[
                Node::Component {
                    name: "TableColumn",
                    props: &[("name", Value::Text("client"))],
                    children: &[],
                },
                Node::Component {
                    name: "TableColumn",
                    // NO CONDITION IS WRITTEN ANYWHERE. The column declares
                    // where the boundary is and which tone each side wears.
                    props: &[
                        ("name", Value::Text("balance")),
                        ("width", Value::Text("fit")),
                        ("sortable", Value::Bool(true)),
                        ("sort_as", Value::Text("number")),
                        ("low", Value::Text("0")),
                        ("tone_below", Value::Text("overdue")),
                        ("tone_within", Value::Text("paid")),
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
                                    ("column", Value::Text("balance")),
                                    ("align", Value::Text("numeric")),
                                ],
                                children: &[
                                    Node::Text(Value::Message(Key::ExampleTableColBalance)),
                                    Node::Component {
                                        name: "Info",
                                        props: &[
                                            ("name", Value::Text("balance")),
                                            ("label", Value::Message(Key::ExampleTableBalanceInfo)),
                                            ("placement", Value::Text("end")),
                                        ],
                                        children: &[Node::Text(Value::Message(
                                            Key::ExampleTableBalanceInfoNote,
                                        ))],
                                    },
                                ],
                            },
                        ],
                    }],
                },
                Node::Component {
                    name: "TableSection",
                    props: &[],
                    children: &[
                        Node::Each {
                            over: Each::Rows(&[
                                &[
                                    Value::Data("Aurora Ltd"),
                                    Value::Data("R$ 4.900,00"),
                                    Value::Text("4900"),
                                ],
                                &[
                                    Value::Data("Bracken & Co"),
                                    Value::Data("-R$ 240,00"),
                                    Value::Text("-240"),
                                ],
                                &[
                                    Value::Data("Cintra Studio"),
                                    Value::Data("R$ 680,00"),
                                    Value::Text("680"),
                                ],
                                &[
                                    Value::Data("Delgado SA"),
                                    Value::Data("-R$ 1.115,00"),
                                    Value::Text("-1115"),
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
                                            ("column", Value::Text("balance")),
                                            ("class", Value::Text("nd-identifier")),
                                            ("align", Value::Text("numeric")),
                                            ("sort_value", Value::Bound(2)),
                                        ],
                                        children: &[Node::Text(Value::Bound(1))],
                                    },
                                ],
                            }],
                        },
                        // A CELL'S OWN TONE WINS. This one is negative and
                        // would have been painted `overdue` by the column; the
                        // server knows it has been written off, and says so.
                        Node::Component {
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
                                    children: &[Node::Text(Value::Data("Enseada Lda"))],
                                },
                                Node::Component {
                                    name: "TableCell",
                                    props: &[
                                        ("column", Value::Text("balance")),
                                        ("class", Value::Text("nd-identifier")),
                                        ("tone", Value::Text("archived")),
                                        ("align", Value::Text("numeric")),
                                        ("sort_value", Value::Text("-90")),
                                    ],
                                    children: &[Node::Text(Value::Data("-R$ 90,00"))],
                                },
                            ],
                        },
                    ],
                },
            ],
        }],
    ),
    (
        r"Table { name, caption, search_label, search_placeholder, filtered_label }
TableRow { emphasis } and TableCell { aggregate }: subtotal, tax, total",
        &[Node::Component {
            name: "Table",
            props: &[
                ("name", Value::Text("invoice-totals")),
                ("caption", Value::Message(Key::ExampleTableInvoices)),
                ("search_label", Value::Message(Key::ExampleTableSearch)),
                (
                    "search_placeholder",
                    Value::Message(Key::ExampleTableSearchPlaceholder),
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
                    children: &[
                        Node::Component {
                            name: "TableRow",
                            props: &[("emphasis", Value::Text("subtle"))],
                            children: &[
                                Node::Component {
                                    name: "TableHeaderCell",
                                    props: &[
                                        ("scope", Value::Text("row")),
                                        ("column", Value::Text("invoice")),
                                        ("span", Value::Text("4")),
                                    ],
                                    children: &[Node::Text(Value::Message(
                                        Key::ExampleTableSubtotal,
                                    ))],
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
                        },
                        Node::Component {
                            name: "TableRow",
                            props: &[("emphasis", Value::Text("subtle"))],
                            children: &[
                                Node::Component {
                                    name: "TableHeaderCell",
                                    props: &[
                                        ("scope", Value::Text("row")),
                                        ("column", Value::Text("invoice")),
                                        ("span", Value::Text("4")),
                                    ],
                                    children: &[Node::Text(Value::Message(Key::ExampleTableTax))],
                                },
                                Node::Component {
                                    name: "TableCell",
                                    props: &[
                                        ("column", Value::Text("amount")),
                                        ("class", Value::Text("nd-identifier")),
                                        ("align", Value::Text("numeric")),
                                    ],
                                    children: &[Node::Text(Value::Data("R$ 1.568,60"))],
                                },
                            ],
                        },
                        Node::Component {
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
                                        ("class", Value::Text("nd-identifier")),
                                        ("align", Value::Text("numeric")),
                                    ],
                                    children: &[Node::Text(Value::Data("R$ 8.388,60"))],
                                },
                            ],
                        },
                    ],
                },
            ],
        }],
    ),
    (
        r"A Meter, a Progress and an Info, all inside cells",
        &[Node::Component {
            name: "Table",
            props: &[
                ("name", Value::Text("usage")),
                ("caption", Value::Message(Key::ExampleTableUsage)),
                ("density", Value::Text("comfortable")),
            ],
            children: &[
                Node::Component {
                    name: "TableColumn",
                    props: &[("name", Value::Text("project"))],
                    children: &[],
                },
                Node::Component {
                    name: "TableColumn",
                    props: &[
                        ("name", Value::Text("storage")),
                        ("width", Value::Text("medium")),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "TableColumn",
                    props: &[
                        ("name", Value::Text("tokens")),
                        ("width", Value::Text("medium")),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "TableColumn",
                    props: &[
                        ("name", Value::Text("import")),
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
                                props: &[("column", Value::Text("project"))],
                                children: &[Node::Text(Value::Message(
                                    Key::ExampleTableColProject,
                                ))],
                            },
                            Node::Component {
                                name: "TableHeaderCell",
                                props: &[("column", Value::Text("storage"))],
                                children: &[Node::Text(Value::Message(
                                    Key::ExampleTableColStorage,
                                ))],
                            },
                            Node::Component {
                                name: "TableHeaderCell",
                                props: &[("column", Value::Text("tokens"))],
                                children: &[
                                    Node::Text(Value::Message(Key::ExampleTableColTokens)),
                                    Node::Component {
                                        name: "Info",
                                        props: &[
                                            ("name", Value::Text("tokens")),
                                            ("label", Value::Message(Key::ExampleTableTokensInfo)),
                                        ],
                                        children: &[Node::Text(Value::Message(
                                            Key::ExampleTableTokensInfoNote,
                                        ))],
                                    },
                                ],
                            },
                            Node::Component {
                                name: "TableHeaderCell",
                                props: &[("column", Value::Text("import"))],
                                children: &[Node::Text(Value::Message(Key::ExampleTableColImport))],
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
                                        ("column", Value::Text("project")),
                                        ("class", Value::Text("nd-identifier")),
                                    ],
                                    children: &[Node::Text(Value::Data("atlas"))],
                                },
                                Node::Component {
                                    name: "TableCell",
                                    props: &[("column", Value::Text("storage"))],
                                    children: &[Node::Component {
                                        name: "Meter",
                                        props: &[
                                            ("name", Value::Text("atlas-storage")),
                                            ("label", Value::Message(Key::ExampleTableColStorage)),
                                            ("value", Value::Text("41.2")),
                                            ("max", Value::Text("50")),
                                            ("unit", Value::Text("GB")),
                                            ("low", Value::Text("10")),
                                            ("high", Value::Text("40")),
                                            ("optimum", Value::Text("0")),
                                            ("size", Value::Text("small")),
                                            ("hide_label", Value::Bool(true)),
                                            (
                                                "value_label",
                                                Value::Message(Key::ExampleTableStorageValue),
                                            ),
                                        ],
                                        children: &[],
                                    }],
                                },
                                Node::Component {
                                    name: "TableCell",
                                    props: &[("column", Value::Text("tokens"))],
                                    children: &[Node::Component {
                                        name: "Meter",
                                        props: &[
                                            ("name", Value::Text("atlas-tokens")),
                                            ("label", Value::Message(Key::ExampleTableColTokens)),
                                            ("value", Value::Text("48")),
                                            ("max", Value::Text("200")),
                                            ("low", Value::Text("60")),
                                            ("high", Value::Text("160")),
                                            ("optimum", Value::Text("0")),
                                            ("size", Value::Text("small")),
                                            ("hide_label", Value::Bool(true)),
                                            (
                                                "value_label",
                                                Value::Message(Key::ExampleTableTokensValue),
                                            ),
                                        ],
                                        children: &[],
                                    }],
                                },
                                Node::Component {
                                    name: "TableCell",
                                    props: &[("column", Value::Text("import"))],
                                    children: &[Node::Component {
                                        name: "Progress",
                                        props: &[
                                            ("name", Value::Text("atlas-import")),
                                            ("label", Value::Message(Key::ExampleTableColImport)),
                                            ("value", Value::Text("7")),
                                            ("max", Value::Text("12")),
                                            ("size", Value::Text("small")),
                                            ("hide_label", Value::Bool(true)),
                                            (
                                                "value_label",
                                                Value::Message(Key::ExampleTableImportValue),
                                            ),
                                        ],
                                        children: &[],
                                    }],
                                },
                            ],
                        },
                        Node::Component {
                            name: "TableRow",
                            props: &[],
                            children: &[
                                Node::Component {
                                    name: "TableHeaderCell",
                                    props: &[
                                        ("scope", Value::Text("row")),
                                        ("column", Value::Text("project")),
                                        ("class", Value::Text("nd-identifier")),
                                    ],
                                    children: &[Node::Text(Value::Data("borealis"))],
                                },
                                Node::Component {
                                    name: "TableCell",
                                    props: &[("column", Value::Text("storage"))],
                                    children: &[Node::Component {
                                        name: "Meter",
                                        props: &[
                                            ("name", Value::Text("borealis-storage")),
                                            ("label", Value::Message(Key::ExampleTableColStorage)),
                                            ("value", Value::Text("48.6")),
                                            ("max", Value::Text("50")),
                                            ("unit", Value::Text("GB")),
                                            ("low", Value::Text("10")),
                                            ("high", Value::Text("40")),
                                            ("optimum", Value::Text("0")),
                                            ("size", Value::Text("small")),
                                            ("hide_label", Value::Bool(true)),
                                            (
                                                "value_label",
                                                Value::Message(Key::ExampleTableStorageValue),
                                            ),
                                        ],
                                        children: &[],
                                    }],
                                },
                                Node::Component {
                                    name: "TableCell",
                                    props: &[("column", Value::Text("tokens"))],
                                    children: &[Node::Component {
                                        name: "Meter",
                                        props: &[
                                            ("name", Value::Text("borealis-tokens")),
                                            ("label", Value::Message(Key::ExampleTableColTokens)),
                                            ("value", Value::Text("121")),
                                            ("max", Value::Text("200")),
                                            ("low", Value::Text("60")),
                                            ("high", Value::Text("160")),
                                            ("optimum", Value::Text("0")),
                                            ("size", Value::Text("small")),
                                            ("hide_label", Value::Bool(true)),
                                            (
                                                "value_label",
                                                Value::Message(Key::ExampleTableTokensValue),
                                            ),
                                        ],
                                        children: &[],
                                    }],
                                },
                                Node::Component {
                                    name: "TableCell",
                                    props: &[("column", Value::Text("import"))],
                                    children: &[Node::Component {
                                        name: "Progress",
                                        props: &[
                                            ("name", Value::Text("borealis-import")),
                                            ("label", Value::Message(Key::ExampleTableColImport)),
                                            ("size", Value::Text("small")),
                                            ("hide_label", Value::Bool(true)),
                                            (
                                                "busy_label",
                                                Value::Message(Key::ExampleTableImportBusy),
                                            ),
                                        ],
                                        children: &[],
                                    }],
                                },
                            ],
                        },
                    ],
                },
            ],
        }],
    ),
    (
        r"Table { search_label, search_placeholder, filtered_label, summary_label }",
        &[Node::Component {
            name: "Table",
            props: &[
                ("name", Value::Text("searchable")),
                ("caption", Value::Message(Key::ExampleTableInvoices)),
                ("search_label", Value::Message(Key::ExampleTableSearch)),
                (
                    "search_placeholder",
                    Value::Message(Key::ExampleTableSearchPlaceholder),
                ),
                ("filtered_label", Value::Message(Key::ExampleTableFiltered)),
                ("summary_label", Value::Message(Key::ExampleTableSummary)),
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
                            over: Each::Rows(&[AURORA, BRACKEN, CINTRA, DELGADO]),
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
            ],
        }],
    ),
    (
        r"Table { export, export_label, csv_label, json_label, print_label, exported_label, copy, copy_label, copied_label, selected_label }",
        &[Node::Component {
            name: "Table",
            props: &[
                ("name", Value::Text("team")),
                ("caption", Value::Message(Key::ExampleTableTeam)),
                ("export", Value::Bool(true)),
                ("export_label", Value::Message(Key::ExampleTableExport)),
                ("csv_label", Value::Message(Key::ExampleTableExportCsv)),
                ("json_label", Value::Message(Key::ExampleTableExportJson)),
                ("print_label", Value::Message(Key::ExampleTableExportPrint)),
                ("exported_label", Value::Message(Key::ExampleTableExported)),
                ("copy", Value::Bool(true)),
                ("copy_label", Value::Message(Key::ExampleTableExportCopy)),
                ("copied_label", Value::Message(Key::ExampleTableCopied)),
                ("selected_label", Value::Message(Key::ExampleTableChosen)),
            ],
            children: &[
                Node::Component {
                    name: "TableColumn",
                    props: &[
                        ("name", Value::Text("choose")),
                        ("width", Value::Text("fit")),
                    ],
                    children: &[],
                },
                // No `filterable`: this table draws no search box either.
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
                        props: &[
                            ("select", Value::Text("all")),
                            ("select_name", Value::Text("person")),
                            (
                                "select_label",
                                Value::Message(Key::ExampleTableChooseEveryone),
                            ),
                        ],
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
                    children: &[Node::Each {
                        over: Each::Rows(&[
                            &[
                                Value::Text("ada"),
                                Value::Data("Ada Lovelace"),
                                Value::Message(Key::ExampleTableRoleOwner),
                                Value::Text("authorized"),
                                Value::Message(Key::ExampleTableAccessToday),
                                Value::Bool(true),
                                Value::Bool(false),
                            ],
                            &[
                                Value::Text("alan"),
                                Value::Data("Alan Turing"),
                                Value::Message(Key::ExampleTableRoleAdmin),
                                Value::Text("admin"),
                                Value::Message(Key::ExampleTableAccessToday),
                                Value::Bool(false),
                                Value::Bool(false),
                            ],
                            &[
                                Value::Text("grace"),
                                Value::Data("Grace Hopper"),
                                Value::Message(Key::ExampleTableRoleMember),
                                Value::Text("member"),
                                Value::Message(Key::ExampleTableAccessWeek),
                                Value::Bool(false),
                                Value::Bool(false),
                            ],
                            &[
                                Value::Text("katherine"),
                                Value::Data("Katherine Johnson"),
                                Value::Message(Key::ExampleTableRoleGuest),
                                Value::Text("guest"),
                                Value::Message(Key::ExampleTableAccessNever),
                                Value::Bool(false),
                                Value::Bool(true),
                            ],
                        ]),
                        body: &[Node::Component {
                            name: "TableRow",
                            props: &[
                                ("name", Value::Bound(0)),
                                ("select", Value::Text("many")),
                                ("select_name", Value::Text("person")),
                                (
                                    "select_label",
                                    Value::Message(Key::ExampleTableChoosePerson),
                                ),
                                ("selected", Value::Bound(5)),
                                ("disabled", Value::Bound(6)),
                            ],
                            children: &[
                                Node::Component {
                                    name: "TableHeaderCell",
                                    props: &[
                                        ("scope", Value::Text("row")),
                                        ("column", Value::Text("person")),
                                        ("class", Value::Text("nd-identifier")),
                                    ],
                                    children: &[Node::Text(Value::Bound(1))],
                                },
                                Node::Component {
                                    name: "TableCell",
                                    props: &[("column", Value::Text("role"))],
                                    children: &[Node::Component {
                                        name: "Tag",
                                        props: &[
                                            ("label", Value::Bound(2)),
                                            ("tone", Value::Bound(3)),
                                            ("variant", Value::Text("outline")),
                                            ("size", Value::Text("small")),
                                        ],
                                        children: &[],
                                    }],
                                },
                                Node::Component {
                                    name: "TableCell",
                                    props: &[("column", Value::Text("access"))],
                                    children: &[Node::Text(Value::Bound(4))],
                                },
                            ],
                        }],
                    }],
                },
            ],
        }],
    ),
    (
        r"Table { more_src, more_label, more_status_label, total } — load more",
        &[Node::Component {
            name: "Table",
            props: &[
                ("name", Value::Text("deploys")),
                ("caption", Value::Message(Key::ExampleTableDeploys)),
                ("density", Value::Text("compact")),
                ("more_src", Value::Asset("assets/deploys-2.html")),
                ("more_label", Value::Message(Key::ExampleTableMore)),
                (
                    "more_status_label",
                    Value::Message(Key::ExampleTableMoreStatus),
                ),
                ("total", Value::Text("8")),
                ("summary_label", Value::Message(Key::ExampleTableSummary)),
            ],
            children: &[
                Node::Fragment(DEPLOY_COLUMNS),
                Node::Fragment(DEPLOY_HEADINGS),
                Node::Component {
                    name: "TableSection",
                    props: &[],
                    children: &[Node::Each {
                        over: Each::Rows(&[PASSING_4213, RUNNING_4212, FAILED_4211, DEGRADED_4210]),
                        body: DEPLOY_ROW,
                    }],
                },
            ],
        }],
    ),
    (
        r"Table { density, rules, zebra, hover }",
        &[Node::Component {
            name: "Table",
            props: &[
                ("name", Value::Text("deploys-dense")),
                ("caption", Value::Message(Key::ExampleTableDeploys)),
                ("density", Value::Text("compact")),
                ("rules", Value::Text("grid")),
                ("zebra", Value::Bool(true)),
                ("hover", Value::Bool(true)),
            ],
            children: &[
                Node::Fragment(DEPLOY_COLUMNS),
                Node::Fragment(DEPLOY_HEADINGS),
                Node::Component {
                    name: "TableSection",
                    props: &[],
                    children: &[Node::Each {
                        over: Each::Rows(&[PASSING_4213, RUNNING_4212, FAILED_4211, CANCELED_4210]),
                        body: DEPLOY_ROW,
                    }],
                },
            ],
        }],
    ),
    (
        r"Table { sticky_header: true, height: TableHeight::Short }",
        &[Node::Component {
            name: "Table",
            props: &[
                ("name", Value::Text("deploys-tall")),
                ("caption", Value::Message(Key::ExampleTableDeploys)),
                ("density", Value::Text("compact")),
                ("sticky_header", Value::Bool(true)),
                ("height", Value::Text("short")),
            ],
            children: &[
                Node::Fragment(DEPLOY_COLUMNS),
                Node::Fragment(DEPLOY_HEADINGS),
                Node::Component {
                    name: "TableSection",
                    props: &[],
                    // Nine of the same build, because a header pinned to the
                    // top of a box that never scrolls does nothing to see.
                    children: &[Node::Each {
                        over: Each::Rows(&[
                            PASSING_4213,
                            &[
                                Value::Data("4212"),
                                Value::Data("main"),
                                Value::Message(Key::ExampleTableWhereProduction),
                                Value::Data("51s"),
                                Value::Message(Key::ExampleTableResultPassing),
                                Value::Text("passing"),
                            ],
                            &[
                                Value::Data("4211"),
                                Value::Data("main"),
                                Value::Message(Key::ExampleTableWhereProduction),
                                Value::Data("51s"),
                                Value::Message(Key::ExampleTableResultPassing),
                                Value::Text("passing"),
                            ],
                            &[
                                Value::Data("4210"),
                                Value::Data("main"),
                                Value::Message(Key::ExampleTableWhereProduction),
                                Value::Data("51s"),
                                Value::Message(Key::ExampleTableResultPassing),
                                Value::Text("passing"),
                            ],
                            &[
                                Value::Data("4209"),
                                Value::Data("main"),
                                Value::Message(Key::ExampleTableWhereProduction),
                                Value::Data("51s"),
                                Value::Message(Key::ExampleTableResultPassing),
                                Value::Text("passing"),
                            ],
                            &[
                                Value::Data("4208"),
                                Value::Data("main"),
                                Value::Message(Key::ExampleTableWhereProduction),
                                Value::Data("51s"),
                                Value::Message(Key::ExampleTableResultPassing),
                                Value::Text("passing"),
                            ],
                            &[
                                Value::Data("4207"),
                                Value::Data("main"),
                                Value::Message(Key::ExampleTableWhereProduction),
                                Value::Data("51s"),
                                Value::Message(Key::ExampleTableResultPassing),
                                Value::Text("passing"),
                            ],
                            &[
                                Value::Data("4206"),
                                Value::Data("main"),
                                Value::Message(Key::ExampleTableWhereProduction),
                                Value::Data("51s"),
                                Value::Message(Key::ExampleTableResultPassing),
                                Value::Text("passing"),
                            ],
                            &[
                                Value::Data("4205"),
                                Value::Data("main"),
                                Value::Message(Key::ExampleTableWhereProduction),
                                Value::Data("51s"),
                                Value::Message(Key::ExampleTableResultPassing),
                                Value::Text("passing"),
                            ],
                        ]),
                        body: DEPLOY_ROW,
                    }],
                },
            ],
        }],
    ),
    (
        r"Table { hide_caption: true, width: TableWidth::Auto, layout: TableLayout::Stack }",
        &[Node::Component {
            name: "Table",
            props: &[
                ("name", Value::Text("invoices-stacked")),
                ("caption", Value::Message(Key::ExampleTableInvoices)),
                ("hide_caption", Value::Bool(true)),
                ("width", Value::Text("auto")),
                ("layout", Value::Text("stack")),
            ],
            children: &[
                Node::Fragment(INVOICE_COLUMNS),
                Node::Fragment(INVOICE_HEADINGS),
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
                                    ("column", Value::Text("invoice")),
                                    ("class", Value::Text("nd-identifier")),
                                ],
                                children: &[Node::Text(Value::Data("INV-0044"))],
                            },
                            Node::Component {
                                name: "TableCell",
                                props: &[
                                    ("column", Value::Text("client")),
                                    ("class", Value::Text("nd-identifier")),
                                    ("label", Value::Message(Key::ExampleTableColClient)),
                                ],
                                children: &[Node::Text(Value::Data("Bracken & Co"))],
                            },
                            Node::Component {
                                name: "TableCell",
                                props: &[
                                    ("column", Value::Text("due")),
                                    ("class", Value::Text("nd-identifier")),
                                    ("label", Value::Message(Key::ExampleTableColDue)),
                                    ("sort_value", Value::Text("2026-03-11")),
                                ],
                                children: &[Node::Text(Value::Data("11 Mar"))],
                            },
                            Node::Component {
                                name: "TableCell",
                                props: &[
                                    ("column", Value::Text("status")),
                                    ("label", Value::Message(Key::ExampleTableColStatus)),
                                ],
                                children: &[Node::Component {
                                    name: "Tag",
                                    props: &[
                                        ("label", Value::Message(Key::ExampleTableStatusOverdue)),
                                        ("tone", Value::Text("overdue")),
                                        ("size", Value::Text("small")),
                                    ],
                                    children: &[],
                                }],
                            },
                            Node::Component {
                                name: "TableCell",
                                props: &[
                                    ("column", Value::Text("amount")),
                                    ("class", Value::Text("nd-identifier")),
                                    ("label", Value::Message(Key::ExampleTableColAmount)),
                                    ("align", Value::Text("numeric")),
                                ],
                                children: &[Node::Text(Value::Data("R$ 1.240,00"))],
                            },
                        ],
                    }],
                },
            ],
        }],
    ),
    (
        r"Table { resize_label, resized_label, columns_label, column_shown_label, column_hidden_label }",
        &[Node::Component {
            name: "Table",
            props: &[
                ("name", Value::Text("invoices-wide")),
                ("caption", Value::Message(Key::ExampleTableInvoices)),
                ("resize_label", Value::Message(Key::ExampleTableResize)),
                ("resized_label", Value::Message(Key::ExampleTableResized)),
                ("columns_label", Value::Message(Key::ExampleTableColumns)),
                (
                    "column_shown_label",
                    Value::Message(Key::ExampleTableColumnShown),
                ),
                (
                    "column_hidden_label",
                    Value::Message(Key::ExampleTableColumnHidden),
                ),
            ],
            children: &[
                Node::Component {
                    name: "TableColumn",
                    props: &[
                        ("name", Value::Text("invoice")),
                        ("width", Value::Text("fit")),
                        ("pinned", Value::Bool(true)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "TableColumn",
                    props: &[
                        ("name", Value::Text("client")),
                        ("pinned", Value::Bool(true)),
                        ("resizable", Value::Bool(true)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "TableColumn",
                    props: &[
                        ("name", Value::Text("due")),
                        ("width", Value::Text("fit")),
                        ("hideable", Value::Bool(true)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "TableColumn",
                    props: &[
                        ("name", Value::Text("status")),
                        ("width", Value::Text("fit")),
                        ("hideable", Value::Bool(true)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "TableColumn",
                    props: &[
                        ("name", Value::Text("amount")),
                        ("width", Value::Text("fit")),
                        ("hideable", Value::Bool(true)),
                        ("resizable", Value::Bool(true)),
                    ],
                    children: &[],
                },
                Node::Fragment(INVOICE_HEADINGS),
                Node::Component {
                    name: "TableSection",
                    props: &[],
                    children: &[Node::Each {
                        over: Each::Rows(&[AURORA, BRACKEN]),
                        body: INVOICE_ROW,
                    }],
                },
            ],
        }],
    ),
    (
        r"Table { sort_ascending_label, sort_descending_label, sort_cleared_label }",
        &[Node::Component {
            name: "Table",
            props: &[
                ("name", Value::Text("invoices-sorted")),
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
                    props: &[
                        ("name", Value::Text("invoice")),
                        ("width", Value::Text("fit")),
                        ("sortable", Value::Bool(true)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "TableColumn",
                    props: &[
                        ("name", Value::Text("client")),
                        ("sortable", Value::Bool(true)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "TableColumn",
                    props: &[
                        ("name", Value::Text("due")),
                        ("width", Value::Text("fit")),
                        ("sortable", Value::Bool(true)),
                        ("sort_as", Value::Text("date")),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "TableColumn",
                    props: &[
                        ("name", Value::Text("status")),
                        ("width", Value::Text("fit")),
                    ],
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
                Node::Fragment(INVOICE_HEADINGS),
                Node::Component {
                    name: "TableSection",
                    props: &[],
                    children: &[Node::Each {
                        over: Each::Rows(&[CINTRA, AURORA, DELGADO, BRACKEN]),
                        body: INVOICE_ROW,
                    }],
                },
            ],
        }],
    ),
    (
        r"Table { ai_mark, ai_label, ai_note }",
        &[Node::Component {
            name: "Table",
            props: &[
                ("ai_mark", Value::Message(Key::ExampleAiMark)),
                ("ai_label", Value::Message(Key::ExampleAiName)),
                ("ai_note", Value::Message(Key::ExampleAiNote)),
                ("name", Value::Text("invoices-ai")),
                ("caption", Value::Message(Key::ExampleTableInvoices)),
                ("note", Value::Message(Key::ExampleTableInvoicesNote)),
            ],
            children: &[
                Node::Fragment(INVOICE_COLUMNS),
                Node::Fragment(INVOICE_HEADINGS),
                Node::Component {
                    name: "TableSection",
                    props: &[],
                    children: &[Node::Each {
                        over: Each::Rows(&[AURORA, BRACKEN, CINTRA]),
                        body: INVOICE_ROW,
                    }],
                },
            ],
        }],
    ),
];
