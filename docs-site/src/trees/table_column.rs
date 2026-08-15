//! `TableColumn`, as trees.
//!
//! One entry per example in `examples::TABLE_COLUMN`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_tree_belongs_to_its_example` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.
//!
//! The invoice and deployment parts come from `super::table`, where the
//! macros these examples shared were hoisted to.

use noctua_design_i18n::Key;

use crate::tree::{Each, Node, Value};
use crate::trees::Tree;
use crate::trees::table::{
    AURORA, BRACKEN, CINTRA, DELGADO, DEPLOY_HEADINGS, DEPLOY_ROW, FAILED_4211, INVOICE_HEADINGS,
    INVOICE_ROW, PASSING_4213, QUEUED_4209, RUNNING_4212_RELEASE,
};

/// Every `TableColumn` example. 5 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r"TableColumn { low, high, tone_below, tone_within, tone_above }",
        &[Node::Component {
            name: "Table",
            props: &[
                ("name", Value::Text("column-bands")),
                ("caption", Value::Message(Key::ExampleTableUsage)),
            ],
            children: &[
                Node::Component {
                    name: "TableColumn",
                    props: &[("name", Value::Text("project"))],
                    children: &[],
                },
                Node::Component {
                    name: "TableColumn",
                    // One reading per band, which is the only way three
                    // thresholds can be seen to do anything.
                    props: &[
                        ("name", Value::Text("storage")),
                        ("width", Value::Text("fit")),
                        ("sortable", Value::Bool(true)),
                        ("sort_as", Value::Text("number")),
                        ("low", Value::Text("20")),
                        ("high", Value::Text("45")),
                        ("tone_below", Value::Text("healthy")),
                        ("tone_within", Value::Text("caution")),
                        ("tone_above", Value::Text("critical")),
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
                                props: &[
                                    ("column", Value::Text("storage")),
                                    ("align", Value::Text("numeric")),
                                ],
                                children: &[Node::Text(Value::Message(
                                    Key::ExampleTableColStorage,
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
                                Value::Data("atlas"),
                                Value::Data("12,4 GB"),
                                Value::Text("12.4"),
                            ],
                            &[
                                Value::Data("borealis"),
                                Value::Data("33,1 GB"),
                                Value::Text("33.1"),
                            ],
                            &[
                                Value::Data("cygnus"),
                                Value::Data("48,6 GB"),
                                Value::Text("48.6"),
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
                                        ("column", Value::Text("project")),
                                        ("class", Value::Text("nd-identifier")),
                                    ],
                                    children: &[Node::Text(Value::Bound(0))],
                                },
                                Node::Component {
                                    name: "TableCell",
                                    props: &[
                                        ("column", Value::Text("storage")),
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
        r"TableColumn { name, width: TableColumnWidth::Fit | ::Wide }",
        &[Node::Component {
            name: "Table",
            props: &[
                ("name", Value::Text("column-width")),
                ("caption", Value::Message(Key::ExampleTableDeploys)),
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
                        ("name", Value::Text("branch")),
                        ("width", Value::Text("fit")),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "TableColumn",
                    props: &[("name", Value::Text("log"))],
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
                    children: &[Node::Each {
                        over: Each::Rows(&[
                            &[Value::Data("4213"), Value::Data("main")],
                            &[Value::Data("4211"), Value::Data("fix/anchor")],
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
                                        ("column", Value::Text("branch")),
                                        ("class", Value::Text("nd-identifier")),
                                    ],
                                    children: &[Node::Text(Value::Bound(1))],
                                },
                                Node::Component {
                                    name: "TableCell",
                                    props: &[("column", Value::Text("log"))],
                                    children: &[Node::Text(Value::Message(Key::ExampleTableLog))],
                                },
                            ],
                        }],
                    }],
                },
            ],
        }],
    ),
    (
        r"TableColumn { sortable, sort_as: ::Number | ::Date }",
        &[Node::Component {
            name: "Table",
            props: &[
                ("name", Value::Text("column-sort")),
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
                        over: Each::Rows(&[CINTRA, AURORA, DELGADO]),
                        body: INVOICE_ROW,
                    }],
                },
            ],
        }],
    ),
    (
        r"TableColumn { filterable, hideable, hidden }",
        &[Node::Component {
            name: "Table",
            props: &[
                ("name", Value::Text("column-hide")),
                ("caption", Value::Message(Key::ExampleTableInvoices)),
                ("search_label", Value::Message(Key::ExampleTableSearch)),
                (
                    "search_placeholder",
                    Value::Message(Key::ExampleTableSearchPlaceholder),
                ),
                ("filtered_label", Value::Message(Key::ExampleTableFiltered)),
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
                        ("filterable", Value::Bool(true)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "TableColumn",
                    props: &[
                        ("name", Value::Text("client")),
                        ("filterable", Value::Bool(true)),
                        ("hideable", Value::Bool(true)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "TableColumn",
                    props: &[
                        ("name", Value::Text("due")),
                        ("width", Value::Text("fit")),
                        ("hideable", Value::Bool(true)),
                        ("hidden", Value::Bool(true)),
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
                        ("hideable", Value::Bool(true)),
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
        r"TableColumn { resizable, pinned } and the three status tones",
        &[Node::Component {
            name: "Table",
            props: &[
                ("name", Value::Text("column-pin")),
                ("caption", Value::Message(Key::ExampleTableDeploys)),
                ("resize_label", Value::Message(Key::ExampleTableResize)),
                ("resized_label", Value::Message(Key::ExampleTableResized)),
            ],
            children: &[
                Node::Component {
                    name: "TableColumn",
                    props: &[
                        ("name", Value::Text("build")),
                        ("width", Value::Text("fit")),
                        ("pinned", Value::Bool(true)),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "TableColumn",
                    props: &[
                        ("name", Value::Text("branch")),
                        ("pinned", Value::Bool(true)),
                        ("resizable", Value::Bool(true)),
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
                Node::Fragment(DEPLOY_HEADINGS),
                Node::Component {
                    name: "TableSection",
                    props: &[],
                    children: &[Node::Each {
                        over: Each::Rows(&[
                            PASSING_4213,
                            RUNNING_4212_RELEASE,
                            FAILED_4211,
                            QUEUED_4209,
                        ]),
                        body: DEPLOY_ROW,
                    }],
                },
            ],
        }],
    ),
];
