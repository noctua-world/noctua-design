//! `TableHeaderCell`, as trees.
//!
//! One entry per example in `examples::TABLE_HEADER_CELL`, in the same order.
//! The caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_tree_belongs_to_its_example` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.
//!
//! The invoice parts come from `super::table`, where the macros these
//! examples shared were hoisted to.

use noctua_design_i18n::Key;

use crate::tree::{Each, Node, Value};
use crate::trees::Tree;
use crate::trees::table::{
    AURORA, BRACKEN, CINTRA, DELGADO, INVOICE_COLUMNS, INVOICE_HEADINGS, INVOICE_ROW,
};

/// Every `TableHeaderCell` example. 4 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r"TableHeaderCell { scope: ::Col | ::Row }",
        &[Node::Component {
            name: "Table",
            props: &[
                ("name", Value::Text("header-scope")),
                ("caption", Value::Message(Key::ExampleTableInvoices)),
            ],
            children: &[
                Node::Fragment(INVOICE_COLUMNS),
                Node::Fragment(INVOICE_HEADINGS),
                Node::Component {
                    name: "TableSection",
                    props: &[],
                    children: &[Node::Each {
                        over: Each::Rows(&[AURORA, DELGADO]),
                        body: INVOICE_ROW,
                    }],
                },
            ],
        }],
    ),
    (
        r"TableHeaderCell { column, sort: ::Ascending } — the order a server applied",
        &[Node::Component {
            name: "Table",
            props: &[
                ("name", Value::Text("header-sort")),
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
                Node::Fragment(INVOICE_COLUMNS),
                Node::Component {
                    name: "TableSection",
                    props: &[("kind", Value::Text("head"))],
                    children: &[Node::Component {
                        name: "TableRow",
                        props: &[],
                        children: &[
                            Node::Component {
                                name: "TableHeaderCell",
                                props: &[("column", Value::Text("invoice"))],
                                children: &[Node::Text(Value::Message(
                                    Key::ExampleTableColInvoice,
                                ))],
                            },
                            Node::Component {
                                name: "TableHeaderCell",
                                props: &[("column", Value::Text("client"))],
                                children: &[Node::Text(Value::Message(Key::ExampleTableColClient))],
                            },
                            Node::Component {
                                name: "TableHeaderCell",
                                props: &[
                                    ("column", Value::Text("due")),
                                    ("sort", Value::Text("ascending")),
                                ],
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
                },
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
        r#"TableHeaderCell { align, wrap, tone: "…" }"#,
        &[Node::Component {
            name: "Table",
            props: &[
                ("name", Value::Text("header-align")),
                ("caption", Value::Message(Key::ExampleTableInvoices)),
                ("rules", Value::Text("grid")),
            ],
            children: &[
                Node::Component {
                    name: "TableColumn",
                    props: &[
                        ("name", Value::Text("right")),
                        ("width", Value::Text("medium")),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "TableColumn",
                    props: &[
                        ("name", Value::Text("wrong")),
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
                        // The same words over two columns of the same numbers,
                        // and only one of them lines up.
                        // `wrap`, WHICH IS NOT THE DEFAULT. A heading is
                        // `nowrap` unless it says otherwise, so `nowrap` here
                        // showed the prop doing nothing at all.
                        children: &[
                            Node::Component {
                                name: "TableHeaderCell",
                                props: &[
                                    ("column", Value::Text("right")),
                                    ("align", Value::Text("numeric")),
                                    ("wrap", Value::Text("wrap")),
                                ],
                                children: &[Node::Text(Value::Message(Key::ExampleTableColAmount))],
                            },
                            Node::Component {
                                name: "TableHeaderCell",
                                props: &[
                                    ("column", Value::Text("wrong")),
                                    ("tone", Value::Text("caution")),
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
                            &[Value::Data("R$ 4.900,00")],
                            &[Value::Data("R$ 1.240,00")],
                            &[Value::Data("R$ 680,00")],
                        ]),
                        body: &[Node::Component {
                            name: "TableRow",
                            props: &[],
                            children: &[
                                Node::Component {
                                    name: "TableCell",
                                    props: &[
                                        ("column", Value::Text("right")),
                                        ("class", Value::Text("nd-identifier")),
                                        ("align", Value::Text("numeric")),
                                    ],
                                    children: &[Node::Text(Value::Bound(0))],
                                },
                                Node::Component {
                                    name: "TableCell",
                                    props: &[
                                        ("column", Value::Text("wrong")),
                                        ("class", Value::Text("nd-identifier")),
                                        ("align", Value::Text("numeric")),
                                    ],
                                    children: &[Node::Text(Value::Bound(0))],
                                },
                            ],
                        }],
                    }],
                },
            ],
        }],
    ),
    (
        r#"TableHeaderCell { span: "2", rows: "2", scope: ::Colgroup }"#,
        // THE SPANNING HEADING HAS TO BE TRUE. An earlier draft put *Result*
        // over the environment and the duration, which are not two halves of a
        // result — a two-level heading whose top level does not describe the
        // columns under it teaches a reader to ignore the top level. A build
        // waits and then it runs; both are durations, and *Duration* is what
        // they are two of.
        &[Node::Component {
            name: "Table",
            props: &[
                ("name", Value::Text("header-group")),
                ("caption", Value::Message(Key::ExampleTableDeploys)),
                ("rules", Value::Text("grid")),
                // Sized to its own content: five words spread across the full
                // width of the page put each value an inch from the heading
                // that names it, which is the distance a two-level head exists
                // to close.
                ("width", Value::Text("auto")),
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
                    props: &[("name", Value::Text("wait")), ("width", Value::Text("fit"))],
                    children: &[],
                },
                Node::Component {
                    name: "TableColumn",
                    props: &[("name", Value::Text("run")), ("width", Value::Text("fit"))],
                    children: &[],
                },
                Node::Component {
                    name: "TableSection",
                    props: &[("kind", Value::Text("head"))],
                    children: &[
                        Node::Component {
                            name: "TableRow",
                            props: &[],
                            children: &[
                                Node::Component {
                                    name: "TableHeaderCell",
                                    props: &[
                                        ("rows", Value::Text("2")),
                                        ("scope", Value::Text("rowgroup")),
                                        ("column", Value::Text("build")),
                                    ],
                                    children: &[Node::Text(Value::Message(
                                        Key::ExampleTableColBuild,
                                    ))],
                                },
                                Node::Component {
                                    name: "TableHeaderCell",
                                    props: &[
                                        ("span", Value::Text("2")),
                                        ("scope", Value::Text("colgroup")),
                                        ("align", Value::Text("center")),
                                    ],
                                    children: &[Node::Text(Value::Message(
                                        Key::ExampleTableColTook,
                                    ))],
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
                                        ("column", Value::Text("wait")),
                                        ("align", Value::Text("numeric")),
                                    ],
                                    children: &[Node::Text(Value::Message(
                                        Key::ExampleTableColWait,
                                    ))],
                                },
                                Node::Component {
                                    name: "TableHeaderCell",
                                    props: &[
                                        ("column", Value::Text("run")),
                                        ("align", Value::Text("numeric")),
                                    ],
                                    children: &[Node::Text(Value::Message(
                                        Key::ExampleTableColRun,
                                    ))],
                                },
                            ],
                        },
                    ],
                },
                Node::Component {
                    name: "TableSection",
                    props: &[],
                    children: &[Node::Each {
                        over: Each::Rows(&[
                            &[Value::Data("4213"), Value::Data("6s"), Value::Data("51s")],
                            &[Value::Data("4211"), Value::Data("2s"), Value::Data("12s")],
                        ]),
                        body: &[Node::Component {
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
                                        Value::Bound(0),
                                    ]))],
                                },
                                Node::Component {
                                    name: "TableCell",
                                    props: &[
                                        ("column", Value::Text("wait")),
                                        ("class", Value::Text("nd-identifier")),
                                        ("align", Value::Text("numeric")),
                                    ],
                                    children: &[Node::Text(Value::Bound(1))],
                                },
                                Node::Component {
                                    name: "TableCell",
                                    props: &[
                                        ("column", Value::Text("run")),
                                        ("class", Value::Text("nd-identifier")),
                                        ("align", Value::Text("numeric")),
                                    ],
                                    children: &[Node::Text(Value::Bound(2))],
                                },
                            ],
                        }],
                    }],
                },
            ],
        }],
    ),
];
