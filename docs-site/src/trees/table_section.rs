//! `TableSection`, as trees.
//!
//! One entry per example in `examples::TABLE_SECTION`, in the same order. The
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
    AURORA, BRACKEN, CANCELED_4210, DEPLOY_COLUMNS, DEPLOY_HEADINGS, DEPLOY_ROW, FAILED_4211,
    INVOICE_COLUMNS, INVOICE_HEADINGS, INVOICE_ROW, PASSING_4209, PASSING_4213, RUNNING_4212,
};

/// Every `TableSection` example. 4 of them.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r"TableSection { kind: TableSectionKind::Head }",
        &[Node::Component {
            name: "Table",
            props: &[
                ("name", Value::Text("section-head")),
                ("caption", Value::Message(Key::ExampleTableDeploys)),
            ],
            children: &[
                Node::Fragment(DEPLOY_COLUMNS),
                Node::Fragment(DEPLOY_HEADINGS),
                Node::Component {
                    name: "TableSection",
                    props: &[],
                    children: &[Node::Each {
                        over: Each::Rows(&[PASSING_4213, FAILED_4211]),
                        body: DEPLOY_ROW,
                    }],
                },
            ],
        }],
    ),
    (
        r"TableSection { kind: TableSectionKind::Foot } with an aggregate",
        &[Node::Component {
            name: "Table",
            props: &[
                ("name", Value::Text("section-foot")),
                ("caption", Value::Message(Key::ExampleTableInvoices)),
            ],
            children: &[
                Node::Fragment(INVOICE_COLUMNS),
                Node::Fragment(INVOICE_HEADINGS),
                Node::Component {
                    name: "TableSection",
                    props: &[],
                    children: &[Node::Each {
                        over: Each::Rows(&[AURORA, BRACKEN]),
                        body: INVOICE_ROW,
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
                                children: &[Node::Text(Value::Data("R$ 6.140,00"))],
                            },
                        ],
                    }],
                },
            ],
        }],
    ),
    (
        r"TableSection { name, label, collapsible } — several groups, each folding",
        &[Node::Component {
            name: "Table",
            props: &[
                ("name", Value::Text("section-groups")),
                ("caption", Value::Message(Key::ExampleTableDeploys)),
                ("density", Value::Text("compact")),
            ],
            children: &[
                Node::Fragment(DEPLOY_COLUMNS),
                Node::Fragment(DEPLOY_HEADINGS),
                Node::Component {
                    name: "TableSection",
                    props: &[
                        ("name", Value::Text("today")),
                        ("label", Value::Message(Key::ExampleTableGroupToday)),
                        ("collapsible", Value::Bool(true)),
                    ],
                    children: &[
                        Node::Each {
                            over: Each::Rows(&[PASSING_4213, RUNNING_4212]),
                            body: DEPLOY_ROW,
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
                                        ("span", Value::Text("3")),
                                    ],
                                    children: &[Node::Text(Value::Message(
                                        Key::ExampleTableInThisGroup,
                                    ))],
                                },
                                Node::Component {
                                    name: "TableCell",
                                    props: &[
                                        ("column", Value::Text("took")),
                                        ("align", Value::Text("numeric")),
                                        ("aggregate", Value::Text("count")),
                                        ("aggregate_over", Value::Text("section")),
                                        ("class", Value::Text("nd-identifier")),
                                    ],
                                    children: &[Node::Text(Value::Data("2"))],
                                },
                                Node::Component {
                                    name: "TableCell",
                                    props: &[("column", Value::Text("result"))],
                                    children: &[],
                                },
                            ],
                        },
                    ],
                },
                Node::Component {
                    name: "TableSection",
                    props: &[
                        ("name", Value::Text("yesterday")),
                        ("label", Value::Message(Key::ExampleTableGroupYesterday)),
                        ("collapsible", Value::Bool(true)),
                    ],
                    children: &[
                        Node::Each {
                            over: Each::Rows(&[FAILED_4211, CANCELED_4210, PASSING_4209]),
                            body: DEPLOY_ROW,
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
                                        ("span", Value::Text("3")),
                                    ],
                                    children: &[Node::Text(Value::Message(
                                        Key::ExampleTableInThisGroup,
                                    ))],
                                },
                                Node::Component {
                                    name: "TableCell",
                                    props: &[
                                        ("column", Value::Text("took")),
                                        ("align", Value::Text("numeric")),
                                        ("aggregate", Value::Text("count")),
                                        ("aggregate_over", Value::Text("section")),
                                        ("class", Value::Text("nd-identifier")),
                                    ],
                                    children: &[Node::Text(Value::Data("3"))],
                                },
                                Node::Component {
                                    name: "TableCell",
                                    props: &[("column", Value::Text("result"))],
                                    children: &[],
                                },
                            ],
                        },
                    ],
                },
            ],
        }],
    ),
    (
        r"TableSection { children } — a table with no head at all",
        &[Node::Component {
            name: "Table",
            props: &[
                ("name", Value::Text("section-none")),
                ("caption", Value::Message(Key::ExampleTableInvoices)),
                ("width", Value::Text("auto")),
            ],
            children: &[
                Node::Component {
                    name: "TableColumn",
                    props: &[
                        ("name", Value::Text("field")),
                        ("width", Value::Text("fit")),
                    ],
                    children: &[],
                },
                Node::Component {
                    name: "TableColumn",
                    props: &[("name", Value::Text("value"))],
                    children: &[],
                },
                Node::Component {
                    name: "TableSection",
                    props: &[],
                    children: &[Node::Each {
                        over: Each::Rows(&[
                            &[
                                Value::Message(Key::ExampleTableColInvoice),
                                Value::Data("INV-0044"),
                            ],
                            &[
                                Value::Message(Key::ExampleTableColClient),
                                Value::Data("Bracken & Co"),
                            ],
                            &[
                                Value::Message(Key::ExampleTableColDue),
                                Value::Data("11 Mar"),
                            ],
                            &[
                                Value::Message(Key::ExampleTableColAmount),
                                Value::Data("R$ 1.240,00"),
                            ],
                        ]),
                        body: &[Node::Component {
                            name: "TableRow",
                            props: &[],
                            // NO `wrap`. `nowrap` is the default on a heading,
                            // so the value wrote what absence already writes.
                            children: &[
                                Node::Component {
                                    name: "TableHeaderCell",
                                    props: &[
                                        ("scope", Value::Text("row")),
                                        ("column", Value::Text("field")),
                                    ],
                                    children: &[Node::Text(Value::Bound(0))],
                                },
                                Node::Component {
                                    name: "TableCell",
                                    props: &[
                                        ("column", Value::Text("value")),
                                        ("class", Value::Text("nd-identifier")),
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
