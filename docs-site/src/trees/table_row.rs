//! `TableRow`, as trees.
//!
//! One entry per example in `examples::TABLE_ROW`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.
//!
//! The invoice and deployment parts come from `super::table`, where the
//! macros these examples shared were hoisted to.

use noctua_design_i18n::Key;

use crate::tree::{Each, Node, Value};
use crate::trees::Tree;
use crate::trees::table::{
    AURORA, DEPLOY_COLUMNS, DEPLOY_HEADINGS, DEPLOY_ROW, INVOICE_HEADINGS, INVOICE_ROW,
    PASSING_4210, PASSING_4213,
};

/// Every `TableRow` example. 5 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"TableRow { tone: "…" } — a wash and a rule down the leading edge"#,
        &[Node::Component {
            name: "Table",
            props: &[
                ("name", Value::Text("row-tone")),
                ("caption", Value::Message(Key::ExampleTableDeploys)),
            ],
            children: &[
                Node::Fragment(DEPLOY_COLUMNS),
                Node::Fragment(DEPLOY_HEADINGS),
                Node::Component {
                    name: "TableSection",
                    props: &[],
                    children: &[
                        Node::Each {
                            over: Each::Rows(&[PASSING_4213]),
                            body: DEPLOY_ROW,
                        },
                        Node::Component {
                            name: "TableRow",
                            props: &[("tone", Value::Text("failed"))],
                            children: &[
                                Node::Component {
                                    name: "TableHeaderCell",
                                    props: &[
                                        ("scope", Value::Text("row")),
                                        ("column", Value::Text("build")),
                                        ("class", Value::Text("nd-identifier")),
                                    ],
                                    children: &[
                                        Node::Text(Value::Data("#")),
                                        Node::Text(Value::Data("4211")),
                                    ],
                                },
                                Node::Component {
                                    name: "TableCell",
                                    props: &[
                                        ("column", Value::Text("branch")),
                                        ("class", Value::Text("nd-identifier")),
                                    ],
                                    children: &[Node::Text(Value::Data("fix/anchor"))],
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
                                    props: &[
                                        ("column", Value::Text("took")),
                                        ("class", Value::Text("nd-identifier")),
                                        ("align", Value::Text("numeric")),
                                    ],
                                    children: &[Node::Text(Value::Data("12s"))],
                                },
                                Node::Component {
                                    name: "TableCell",
                                    props: &[("column", Value::Text("result"))],
                                    children: &[Node::Component {
                                        name: "Tag",
                                        props: &[
                                            (
                                                "label",
                                                Value::Message(Key::ExampleTableResultFailed),
                                            ),
                                            ("tone", Value::Text("failed")),
                                            ("size", Value::Text("small")),
                                        ],
                                        children: &[],
                                    }],
                                },
                            ],
                        },
                        Node::Each {
                            over: Each::Rows(&[PASSING_4210]),
                            body: DEPLOY_ROW,
                        },
                    ],
                },
            ],
        }],
    ),
    (
        r"TableRow { name, expandable, expand_label } with a detail_for row",
        &[Node::Component {
            name: "Table",
            props: &[
                ("name", Value::Text("row-expand")),
                ("caption", Value::Message(Key::ExampleTableDeploys)),
                ("density", Value::Text("compact")),
            ],
            children: &[
                Node::Component {
                    name: "TableColumn",
                    props: &[("name", Value::Text("open")), ("width", Value::Text("fit"))],
                    children: &[],
                },
                Node::Fragment(DEPLOY_COLUMNS),
                Node::Fragment(DEPLOY_HEADINGS),
                Node::Component {
                    name: "TableSection",
                    props: &[],
                    children: &[
                        Node::Component {
                            name: "TableRow",
                            props: &[
                                ("name", Value::Text("b4213")),
                                ("expandable", Value::Bool(true)),
                                ("expand_label", Value::Message(Key::ExampleTableLogOpen)),
                            ],
                            children: &[
                                Node::Component {
                                    name: "TableHeaderCell",
                                    props: &[
                                        ("scope", Value::Text("row")),
                                        ("column", Value::Text("build")),
                                        ("class", Value::Text("nd-identifier")),
                                    ],
                                    children: &[
                                        Node::Text(Value::Data("#")),
                                        Node::Text(Value::Data("4213")),
                                    ],
                                },
                                Node::Component {
                                    name: "TableCell",
                                    props: &[
                                        ("column", Value::Text("branch")),
                                        ("class", Value::Text("nd-identifier")),
                                    ],
                                    children: &[Node::Text(Value::Data("main"))],
                                },
                                Node::Component {
                                    name: "TableCell",
                                    props: &[("column", Value::Text("where"))],
                                    children: &[Node::Text(Value::Message(
                                        Key::ExampleTableWhereProduction,
                                    ))],
                                },
                                Node::Component {
                                    name: "TableCell",
                                    props: &[
                                        ("column", Value::Text("took")),
                                        ("class", Value::Text("nd-identifier")),
                                        ("align", Value::Text("numeric")),
                                    ],
                                    children: &[Node::Text(Value::Data("51s"))],
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
                            props: &[("detail_for", Value::Text("b4213"))],
                            children: &[Node::Component {
                                name: "TableCell",
                                props: &[
                                    ("span", Value::Text("6")),
                                    ("class", Value::Text("nd-identifier")),
                                ],
                                children: &[Node::Text(Value::Message(Key::ExampleTableLog))],
                            }],
                        },
                        Node::Component {
                            name: "TableRow",
                            props: &[
                                ("name", Value::Text("b4211")),
                                ("expandable", Value::Bool(true)),
                                ("expand_label", Value::Message(Key::ExampleTableLogOpen)),
                            ],
                            children: &[
                                Node::Component {
                                    name: "TableHeaderCell",
                                    props: &[
                                        ("scope", Value::Text("row")),
                                        ("column", Value::Text("build")),
                                        ("class", Value::Text("nd-identifier")),
                                    ],
                                    children: &[
                                        Node::Text(Value::Data("#")),
                                        Node::Text(Value::Data("4211")),
                                    ],
                                },
                                Node::Component {
                                    name: "TableCell",
                                    props: &[
                                        ("column", Value::Text("branch")),
                                        ("class", Value::Text("nd-identifier")),
                                    ],
                                    children: &[Node::Text(Value::Data("fix/anchor"))],
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
                                    props: &[
                                        ("column", Value::Text("took")),
                                        ("class", Value::Text("nd-identifier")),
                                        ("align", Value::Text("numeric")),
                                    ],
                                    children: &[Node::Text(Value::Data("12s"))],
                                },
                                Node::Component {
                                    name: "TableCell",
                                    props: &[("column", Value::Text("result"))],
                                    children: &[Node::Component {
                                        name: "Tag",
                                        props: &[
                                            (
                                                "label",
                                                Value::Message(Key::ExampleTableResultFailed),
                                            ),
                                            ("tone", Value::Text("failed")),
                                            ("size", Value::Text("small")),
                                        ],
                                        children: &[],
                                    }],
                                },
                            ],
                        },
                        Node::Component {
                            name: "TableRow",
                            props: &[("detail_for", Value::Text("b4211"))],
                            children: &[Node::Component {
                                name: "TableCell",
                                props: &[
                                    ("span", Value::Text("6")),
                                    ("class", Value::Text("nd-identifier")),
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
        r"TableRow { select: TableRowSelect::One, select_name, select_label, selected, disabled }",
        &[Node::Component {
            name: "Table",
            props: &[
                ("name", Value::Text("row-one")),
                ("caption", Value::Message(Key::ExampleTableTeam)),
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
                    name: "TableSection",
                    props: &[("kind", Value::Text("head"))],
                    children: &[Node::Component {
                        name: "TableRow",
                        props: &[],
                        children: &[
                            Node::Component {
                                name: "TableHeaderCell",
                                props: &[("column", Value::Text("choose"))],
                                children: &[],
                            },
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
                                Value::Bool(false),
                            ],
                            &[
                                Value::Text("alan"),
                                Value::Data("Alan Turing"),
                                Value::Message(Key::ExampleTableRoleAdmin),
                                Value::Text("admin"),
                                Value::Bool(false),
                            ],
                            &[
                                Value::Text("katherine"),
                                Value::Data("Katherine Johnson"),
                                Value::Message(Key::ExampleTableRoleGuest),
                                Value::Text("guest"),
                                Value::Bool(true),
                            ],
                        ]),
                        body: &[Node::Component {
                            name: "TableRow",
                            props: &[
                                ("name", Value::Bound(0)),
                                ("select", Value::Text("one")),
                                ("select_name", Value::Text("owner")),
                                (
                                    "select_label",
                                    Value::Message(Key::ExampleTableChoosePerson),
                                ),
                                ("disabled", Value::Bound(4)),
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
                            ],
                        }],
                    }],
                },
            ],
        }],
    ),
    (
        r"TableRow { emphasis: ::Subtle | ::Strong } and TableCell { aggregate }",
        // THE SAME TWO WEIGHTS AS THE INVOICE FOOT, WITH NO MONEY IN IT.
        // `emphasis` says how much weight a row carries and nothing about what
        // it means — these two close a group of builds and then the fleet,
        // which is a subtotal and a total in a ledger's words and neither in
        // this table's.
        &[Node::Component {
            name: "Table",
            props: &[
                ("name", Value::Text("row-emphasis")),
                ("caption", Value::Message(Key::ExampleTableDeploys)),
            ],
            children: &[
                Node::Component {
                    name: "TableColumn",
                    props: &[("name", Value::Text("build"))],
                    children: &[],
                },
                Node::Component {
                    name: "TableColumn",
                    props: &[("name", Value::Text("took")), ("width", Value::Text("fit"))],
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
                                props: &[
                                    ("column", Value::Text("took")),
                                    ("align", Value::Text("numeric")),
                                ],
                                children: &[Node::Text(Value::Message(Key::ExampleTableColTook))],
                            },
                        ],
                    }],
                },
                // A row here is a whole environment: its name, the sentence
                // that closes it, its total, and then the two builds in it.
                // The builds are written out because a row of this walk cannot
                // carry a table of its own for a second walk to read.
                Node::Each {
                    over: Each::Rows(&[
                        &[
                            Value::Text("production"),
                            Value::Message(Key::ExampleTableWhereProduction),
                            Value::Data("99s"),
                            Value::Data("4213"),
                            Value::Data("51s"),
                            Value::Data("4209"),
                            Value::Data("48s"),
                        ],
                        &[
                            Value::Text("staging"),
                            Value::Message(Key::ExampleTableWhereStaging),
                            Value::Data("60s"),
                            Value::Data("4212"),
                            Value::Data("48s"),
                            Value::Data("4211"),
                            Value::Data("12s"),
                        ],
                    ]),
                    body: &[Node::Component {
                        name: "TableSection",
                        props: &[("name", Value::Bound(0))],
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
                                            ("class", Value::Text("nd-identifier")),
                                        ],
                                        children: &[Node::Text(Value::Join(&[
                                            Value::Data("#"),
                                            Value::Bound(3),
                                        ]))],
                                    },
                                    Node::Component {
                                        name: "TableCell",
                                        props: &[
                                            ("column", Value::Text("took")),
                                            ("class", Value::Text("nd-identifier")),
                                            ("align", Value::Text("numeric")),
                                        ],
                                        children: &[Node::Text(Value::Bound(4))],
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
                                            ("column", Value::Text("build")),
                                            ("class", Value::Text("nd-identifier")),
                                        ],
                                        children: &[Node::Text(Value::Join(&[
                                            Value::Data("#"),
                                            Value::Bound(5),
                                        ]))],
                                    },
                                    Node::Component {
                                        name: "TableCell",
                                        props: &[
                                            ("column", Value::Text("took")),
                                            ("class", Value::Text("nd-identifier")),
                                            ("align", Value::Text("numeric")),
                                        ],
                                        children: &[Node::Text(Value::Bound(6))],
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
                                            ("column", Value::Text("build")),
                                        ],
                                        children: &[Node::Text(Value::Bound(1))],
                                    },
                                    Node::Component {
                                        name: "TableCell",
                                        props: &[
                                            ("column", Value::Text("took")),
                                            ("class", Value::Text("nd-identifier")),
                                            ("align", Value::Text("numeric")),
                                            ("aggregate", Value::Text("sum")),
                                            ("aggregate_over", Value::Text("section")),
                                        ],
                                        children: &[Node::Text(Value::Bound(2))],
                                    },
                                ],
                            },
                        ],
                    }],
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
                                    ("column", Value::Text("build")),
                                ],
                                children: &[Node::Text(Value::Message(
                                    Key::ExampleTableAllEnvironments,
                                ))],
                            },
                            Node::Component {
                                name: "TableCell",
                                props: &[
                                    ("column", Value::Text("took")),
                                    ("class", Value::Text("nd-identifier")),
                                    ("align", Value::Text("numeric")),
                                    ("aggregate", Value::Text("sum")),
                                ],
                                children: &[Node::Text(Value::Data("159s"))],
                            },
                        ],
                    }],
                },
            ],
        }],
    ),
    (
        r"Table { name, caption, search_label, search_placeholder, filtered_label }
TableRow { empty: true }: the caller's own sentence",
        &[Node::Component {
            name: "Table",
            props: &[
                ("name", Value::Text("row-empty")),
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
                            over: Each::Rows(&[AURORA]),
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
];
