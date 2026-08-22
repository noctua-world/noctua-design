//! `Lineage`, as trees.
//!
//! One entry per example in `examples::LINEAGE`, in the same order. The
//! caption is repeated here so the pairing is pinned to something a
//! reader can check: `every_example_has_a_tree` reads it against
//! the table's own, and a tree that slid a row apart fails there rather
//! than drawing the wrong picture.
//!
//! # The graphs
//!
//! Written out rather than generated: a reader who wants to know what the
//! language looks like is looking at exactly what was passed in. Where a
//! connection carries a *word* rather than a name or a date, the constant has
//! a hole in it and a `Value::Template` fills the hole — a bond's label is
//! content a reader sees, and content a reader sees is a message like every
//! other one on this site.
//!
//! The portraits are placeholders and nobody's likeness; the attribution
//! beside them says so. They are here because a scene of people reads
//! differently from a scene of initials, and this component is for scenes of
//! people.

use noctua_design_i18n::Key;

use crate::tree::{Node, Value};
use crate::trees::Tree;

/// Where a portrait lives, from a page two levels below the site root.
macro_rules! face {
    ($name:literal) => {
        concat!("image=../../assets/portraits/", $name, ".svg")
    };
}

/// Three generations, a union that carries children, and a second marriage.
///
/// A macro as well as a constant, because the adopted-child scene below is
/// this graph with two more lines on it, and `concat!` joins literals rather
/// than constants.
macro_rules! doe {
    () => {
        concat!(
            "figure gran   label=\"Gran Doe\"   meta=\"1918–2004\" terms=\"doe\" ",
            face!("gran"),
            "\n",
            "figure gramps label=\"Gramps Doe\" meta=\"1915–1998\" terms=\"doe\" ",
            face!("gramps"),
            "\n",
            "mark   u0 kind=union\n",
            "bond   gran u0 kind=union\n",
            "bond   gramps u0 kind=union\n",
            "figure ada label=\"Ada Doe\" meta=\"1946\" terms=\"doe\" ",
            face!("ada"),
            "\n",
            "figure bob label=\"Bob Doe\" meta=\"1949\" terms=\"doe\" ",
            face!("bob"),
            "\n",
            "bond   u0 ada\n",
            "bond   u0 bob\n",
            "figure eve label=\"Eve Marsh\" meta=\"1948\" ",
            face!("eve"),
            "\n",
            "mark   u1 kind=union\n",
            "bond   ada u1 kind=union\n",
            "bond   eve u1 kind=union\n",
            "figure cara label=\"Cara Doe\" meta=\"1972\" terms=\"doe\" ",
            face!("cara"),
            "\n",
            "bond   u1 cara\n",
        )
    };
}

/// Three generations, a union that carries children, and a second marriage.
pub(super) const DOE: &str = doe!();

/// The family above with its adopted child, whose word and whose sentence
/// are both messages — which is why the two are holes a template fills.
pub(super) const ADOPTED: &str = concat!(
    doe!(),
    "figure dan label=\"Dan Doe\" meta=\"1975\" terms=\"doe\" ",
    face!("dan"),
    "\n",
    "bond   u1 dan kind=adoption label=\"{word}\" note=\"{note}\"\n",
);

/// One person, two unions, children under each.
pub(super) const PARTNERS: &str = concat!(
    "figure ada label=\"Ada Doe\" meta=\"1946\" ",
    face!("ada"),
    "\n",
    "figure bob label=\"Bob Marsh\" meta=\"1943\" ",
    face!("bob"),
    "\n",
    "figure eve label=\"Eve Okonkwo\" meta=\"1951\" ",
    face!("eve"),
    "\n",
    "mark   u0 kind=union\n",
    "mark   u1 kind=union\n",
    "bond   ada u0 kind=union\n",
    "bond   bob u0 kind=union\n",
    "bond   ada u1 kind=union\n",
    "bond   eve u1 kind=union\n",
    "figure cara label=\"Cara\" meta=\"1969\" ",
    face!("cara"),
    "\n",
    "figure dan  label=\"Dan\"  meta=\"1971\" ",
    face!("dan"),
    "\n",
    "figure fay  label=\"Fay\"  meta=\"1980\" ",
    face!("gran"),
    "\n",
    "bond   u0 cara\n",
    "bond   u0 dan\n",
    "bond   u1 fay\n",
);

/// The Doe family and the Marsh family, joined by one marriage.
///
/// It exists for the filter: with `Doe` typed in, the whole Marsh side goes
/// — four people, not one — and that is what a cut marker is for. On the
/// plain family graph the filter removed a single figure, which is the one
/// case where a gap needs no explaining.
pub(super) const FILTERED: &str = concat!(
    "figure gran   label=\"Gran Doe\"   meta=\"1918–2004\" terms=\"doe\" ",
    face!("gran"),
    "\n",
    "figure gramps label=\"Gramps Doe\" meta=\"1915–1998\" terms=\"doe\" ",
    face!("gramps"),
    "\n",
    "mark   u0 kind=union\n",
    "bond   gran u0 kind=union\n",
    "bond   gramps u0 kind=union\n",
    "figure ada label=\"Ada Doe\" meta=\"1946\" terms=\"doe\" ",
    face!("ada"),
    "\n",
    "figure bob label=\"Bob Doe\" meta=\"1949\" terms=\"doe\" ",
    face!("bob"),
    "\n",
    "bond   u0 ada\n",
    "bond   u0 bob\n",
    "figure mgran label=\"Rui Marsh\"  meta=\"1920–2001\" ",
    face!("dan"),
    "\n",
    "figure mgram label=\"Fay Marsh\"  meta=\"1922–2010\" ",
    face!("gran"),
    "\n",
    "mark   um kind=union\n",
    "bond   mgran um kind=union\n",
    "bond   mgram um kind=union\n",
    "figure eve label=\"Eve Marsh\" meta=\"1948\" ",
    face!("eve"),
    "\n",
    "figure syb label=\"Syb Marsh\" meta=\"1951\" ",
    face!("cara"),
    "\n",
    "bond   um eve\n",
    "bond   um syb\n",
    "mark   u1 kind=union\n",
    "bond   ada u1 kind=union\n",
    "bond   eve u1 kind=union\n",
    "figure cara label=\"Cara Doe\" meta=\"1972\" terms=\"doe\" ",
    face!("cara"),
    "\n",
    "bond   u1 cara\n",
);

/// A branch on every side of the trunk, which is what the deep view is for.
///
/// The scene it replaced was three children of one couple, and three
/// children make one plane and two behind it — a demonstration of depth
/// with almost no depth in it. This has four branches, two generations
/// under two of them, and connections that leave the trunk in both
/// directions.
/// A branch on every side of the trunk, which is what the deep view is
/// for. Each union here shows the one recorded parent — the mark itself
/// says the other is unrecorded — because this example's whole subject is
/// branches-as-planes, and a two-cell "parent unknown" card on every
/// plane buried the depth it exists to show.
pub(super) const DEEP: &str = concat!(
    "figure bob label=\"Bob Marsh\" meta=\"1943\" ",
    face!("bob"),
    "\n",
    "figure ada label=\"Ada Doe\" meta=\"1946\" ",
    face!("ada"),
    "\n",
    "mark   u0 kind=union\n",
    "bond   ada u0 kind=union\n",
    "bond   bob u0 kind=union\n",
    "figure cara label=\"Cara\" meta=\"1969\" ",
    face!("cara"),
    "\n",
    "figure dan  label=\"Dan\"  meta=\"1971\" ",
    face!("dan"),
    "\n",
    "figure fay  label=\"Fay\"  meta=\"1974\" ",
    face!("gran"),
    "\n",
    "bond   u0 cara\n",
    "bond   u0 dan\n",
    "bond   u0 fay\n",
    "figure kit label=\"Kit\" meta=\"1995\" ",
    face!("eve"),
    "\n",
    "figure lev label=\"Lev\" meta=\"1998\" ",
    face!("dan"),
    "\n",
    "figure mia label=\"Mia\" meta=\"2001\" ",
    face!("ada"),
    "\n",
    "mark   u1 kind=union\n",
    "bond   cara u1 kind=union\n",
    "bond   u1 kit\n",
    "bond   u1 lev\n",
    "mark   u2 kind=union\n",
    "bond   fay u2 kind=union\n",
    "bond   u2 mia\n",
);

pub(super) const SEASON: &str = concat!(
    "figure ada label=\"Ada Doe\" meta=\"1946\" slot\n",
    "figure eve label=\"Eve Marsh\" meta=\"1948\" slot\n",
    "mark   u0 kind=union\n",
    "bond   ada u0 kind=union\n",
    "bond   eve u0 kind=union\n",
    "figure cara label=\"Cara Doe\" meta=\"1972\" slot\n",
    "bond   u0 cara\n",
);

/// A grandparent married to a grandchild: the union spans two rows, and
/// nothing about the descent moves to make it fit.
///
/// EVERY CHILD HERE HAS TWO PARENTS, and two of the four are nobody. A
/// person has two whether or not a family knows who they were, and a scene
/// that hangs a child off one of them is drawing a claim rather than a
/// gap — which is what a reader said about every example on this page. An
/// unknown parent is a figure like any other, with an em dash for a name.
///
/// TWO HOLES, TWO NAMES. One `{nobody}` stood in both cards and was filled
/// twice over, by position. A template fills a hole by name, so each card
/// carries a name of its own here.
pub(super) const SPANNING: &str = concat!(
    "figure gran label=\"Gran Doe\" meta=\"1918\" ",
    face!("gran"),
    "\n",
    "figure nan  kind=unknown span=1.5 label=\"{nobody-mother}\"\n",
    "mark   u1 kind=union\n",
    "bond   gran u1 kind=union\n",
    "bond   nan u1 kind=union\n",
    "figure ada  label=\"Ada Doe\"  meta=\"1946\" ",
    face!("ada"),
    "\n",
    "bond   u1 ada\n",
    "figure pat  kind=unknown span=1.5 label=\"{nobody-father}\"\n",
    "mark   u2 kind=union\n",
    "bond   ada u2 kind=union\n",
    "bond   pat u2 kind=union\n",
    "figure cara label=\"Cara Doe\" meta=\"1972\" ",
    face!("cara"),
    "\n",
    "bond   u2 cara\n",
    "mark   u0 kind=union\n",
    "bond   gran u0 kind=union\n",
    "bond   cara u0 kind=union\n",
    "figure kid label=\"Dan Doe\" meta=\"1996\" ",
    face!("dan"),
    "\n",
    "bond   u0 kid\n",
);

/// A child with two biological parents nobody knows and two who raised
/// them. Both couples are drawn, because both are true at once — which is
/// the case a family tree with one parent per line cannot state at all.
///
/// UNKNOWN IS A SENTENCE, NOT A DASH. A card with `—` on it says a field
/// was left empty; a card saying *biological father unknown* says what is
/// true about a person, which is what a reader asked for. The kind is what
/// makes the card two cells wide, so the sentence fits.
///
/// TWO HOLES, TWO NAMES. One `{nobody}` stood in both cards and was filled
/// twice over, by position. A template fills a hole by name, so each card
/// carries a name of its own here.
pub(super) const ADOPTION: &str = concat!(
    "figure bio1 kind=unknown span=1.5 label=\"{nobody-father}\"\n",
    "figure bio2 kind=unknown span=1.5 label=\"{nobody-mother}\"\n",
    "mark   ub kind=union\n",
    "bond   bio1 ub kind=union\n",
    "bond   bio2 ub kind=union\n",
    "figure ada label=\"Ada Doe\"  meta=\"1946\" ",
    face!("ada"),
    "\n",
    "figure eve label=\"Eve Marsh\" meta=\"1948\" ",
    face!("eve"),
    "\n",
    "mark   ua kind=union\n",
    "bond   ada ua kind=union\n",
    "bond   eve ua kind=union\n",
    "figure kid label=\"Rui Doe\" meta=\"1998\" ",
    face!("dan"),
    "\n",
    "bond   ub kid\n",
    "bond   ua kid kind=adoption label=\"{word}\"\n",
);

/// A donor, the parent who carried the child, and the partner who is not a
/// parent — three relationships that a tree of couples cannot tell apart
/// and that this one draws in three different lines.
pub(super) const DONOR: &str = concat!(
    "figure ada label=\"Ada Doe\" meta=\"1946\" ",
    face!("ada"),
    "\n",
    "figure donor kind=unknown span=1.5 label=\"{nobody}\"\n",
    "mark   u0 kind=union\n",
    "bond   ada u0 kind=union\n",
    "figure cara label=\"Cara Doe\" meta=\"1972\" ",
    face!("cara"),
    "\n",
    "bond   u0 cara\n",
    "figure eve label=\"Eve Marsh\" meta=\"1948\" ",
    face!("eve"),
    "\n",
    "bond   donor u0 kind=donor label=\"{donor}\"\n",
    "bond   ada eve kind=partner label=\"{partner}\"\n",
);

/// A company under an origin that stays at the top.
pub(super) const COMPANY: &str = "\
figure aviary label=\"Aviary Ltd\" meta=\"Since 1948\" origin shape=plaque span=2 tone=accent\n\
figure chair label=\"Ada Doe\" meta=\"Chair\" kind=role\n\
figure ops   label=\"Bob Marsh\" meta=\"Operations\" kind=role\n\
figure build label=\"Eve Okonkwo\" meta=\"Building\" kind=role\n\
figure a1 label=\"Night shift\" kind=team\n\
figure a2 label=\"Day shift\" kind=team\n\
figure b1 label=\"Workshop\" kind=team\n\
bond   ops a1 kind=report\n\
bond   ops a2 kind=report\n\
bond   build b1 kind=report\n";

/// Families with nothing between them.
///
/// THREE, NOT FOUR, and each one a couple. Four families of one parent and
/// one child fitted the window and said something false about all four;
/// three families of three say the true thing in the same width.
pub(super) const FOUR: &str = "\
figure a1 label=\"Ada Doe\" meta=\"1946\"\nfigure a2 label=\"Bob Doe\" meta=\"1944\"\n\
mark ua kind=union\nbond a1 ua kind=union\nbond a2 ua kind=union\n\
figure a3 label=\"Cara Doe\" meta=\"1972\"\nbond ua a3\n\
figure b1 label=\"Bob Marsh\" meta=\"1943\"\nfigure b2 label=\"Eve Marsh\" meta=\"1945\"\n\
mark ub kind=union\nbond b1 ub kind=union\nbond b2 ub kind=union\n\
figure b3 label=\"Fay Marsh\" meta=\"1969\"\nbond ub b3\n\
figure c1 label=\"Eve Okonkwo\" meta=\"1951\"\nfigure c2 label=\"Dan Okonkwo\" meta=\"1950\"\n\
mark uc kind=union\nbond c1 uc kind=union\nbond c2 uc kind=union\n\
figure c3 label=\"Ada Okonkwo\" meta=\"1980\"\nbond uc c3\n";

/// A branch that can come from somewhere else, and the switch that brings
/// it.
///
/// THE SWITCH IS NOT IN THE TREE. It used to hang off Ada by a dotted line,
/// which drew a server as a member of the family — it is a control, it
/// belongs beside the scene, and `cluster` is how a graph says *this stands
/// on its own*. Pressing it a second time puts the branch away again.
pub(super) const SERVER: &str = concat!(
    "figure ada label=\"Ada Doe\" meta=\"1946\" ",
    face!("ada"),
    "\n",
    "figure bob label=\"Bob Doe\" meta=\"1944\" ",
    face!("bob"),
    "\n",
    "mark   u0 kind=union\n",
    "bond   ada u0 kind=union\n",
    "bond   bob u0 kind=union\n",
    "figure c1 label=\"Cara Doe\" meta=\"1972\" ",
    face!("cara"),
    "\n",
    "bond   u0 c1\n",
    "figure c2 label=\"Rui Doe\" meta=\"1975\" set=doe:remote ",
    face!("dan"),
    "\n",
    "figure c3 label=\"Fay Doe\" meta=\"1978\" set=doe:remote ",
    face!("gran"),
    "\n",
    "bond   u0 c2 set=doe:remote\n",
    "bond   u0 c3 set=doe:remote\n",
    "mark   srv kind=source slot span=2 cluster=server\n",
);

/// Two partners, their union and their child — the smallest scene that
/// still has every kind of thing in it.
///
/// It exists because the examples about SIZE and THICKNESS were drawn on a
/// graph with a slot in it, and nothing filled the slot: each of them drew
/// a dotted connection running into an empty box. A slot is a promise that
/// a `LineageFigure` will arrive, and an example that does not keep it is
/// an example of the component failing.
pub(super) const PAIR: &str = concat!(
    "figure ada label=\"Ada Doe\" meta=\"1946\" ",
    face!("ada"),
    "\n",
    "figure eve label=\"Eve Marsh\" meta=\"1948\" ",
    face!("eve"),
    "\n",
    "mark   u0 kind=union\n",
    "bond   ada u0 kind=union\n",
    "bond   eve u0 kind=union\n",
    "figure cara label=\"Cara Doe\" meta=\"1972\" ",
    face!("cara"),
    "\n",
    "bond   u0 cara\n",
);

/// The owl and the founding couple the wild and queen scenes share — one
/// text, so the two can never drift apart by accident.
macro_rules! roost {
    () => {
        concat!(
            "figure noctua slot origin shape=bare span=2 reach=0.55 rank=0\n",
            "figure ada label=\"Ada Doe\" meta=\"1946\" shape=bare rank=2 ",
            face!("ada"),
            "\n",
            "figure eve label=\"Eve Marsh\" meta=\"1948\" shape=bare rank=2 ",
            face!("eve"),
            "\n",
            "mark   u0 kind=union\n",
            "bond   ada u0 kind=union\n",
            "bond   eve u0 kind=union\n",
        )
    };
}

/// The wild scene: nothing is in a box. Every figure is `shape=bare`,
/// the origin is the photographed owl, and the example that uses it also
/// sets `reach: Shy` and `lettering: Solid` — the three asks of one
/// message.
pub(super) const WILD: &str = concat!(
    roost!(),
    "figure cara label=\"Cara Doe\" meta=\"1972\" shape=bare ",
    face!("cara"),
    "\n",
    "figure dan label=\"Dan Doe\" meta=\"1975\" shape=bare ",
    face!("dan"),
    "\n",
    "bond   u0 cara\n",
    "bond   u0 dan\n",
);

/// The queen scene: the wild scene's roost with ONE child, so the doubled
/// model is the only thing that changed between the two examples a
/// reader compares — and the shorter family keeps a taller scene short.
pub(super) const QUEEN: &str = concat!(
    roost!(),
    "figure cara label=\"Cara Doe\" meta=\"1972\" shape=bare ",
    face!("cara"),
    "\n",
    "bond   u0 cara\n",
);

/// And with the two founders left to fill, for the examples whose subject
/// is what can go inside a figure. The origin draws itself here, because
/// an example is about one thing at a time and every other slot in it has
/// to be somebody's.
pub(super) const SLOTS: &str = concat!(
    // THE ORIGIN IS A SLOT, AND IT IS WHERE A PLAQUE GOES. A plaque is two
    // cells wide, and the only row with two free cells on it is the one the
    // origin stands alone on — beside a union dot it reached across the
    // structure it was standing on.
    "figure noctua origin shape=plaque span=2 slot\n",
    "figure ada label=\"Ada Doe\" meta=\"1946\" slot\n",
    "figure eve label=\"Eve Marsh\" meta=\"1948\" slot\n",
    "mark   u0 kind=union\n",
    "bond   ada u0 kind=union\n",
    "bond   eve u0 kind=union\n",
    "figure cara label=\"Cara Doe\" meta=\"1972\" ",
    face!("cara"),
    "\n",
    "bond   u0 cara\n",
);

/// A mythic origin, and the family that says it came from her.
pub(super) const OWL: &str = concat!(
    "figure noctua slot origin shape=round\n",
    "figure ada label=\"Ada Doe\" meta=\"1946\" slot\n",
    "figure eve label=\"Eve Marsh\" meta=\"1948\" slot\n",
    "mark   u0 kind=union\n",
    "bond   ada u0 kind=union\n",
    "bond   eve u0 kind=union\n",
    "figure cara label=\"Cara Doe\" meta=\"1972\" ",
    face!("cara"),
    "\n",
    "figure dan  label=\"Dan Doe\"  meta=\"1975\" ",
    face!("dan"),
    "\n",
    "bond   u0 cara\n",
    "bond   u0 dan\n",
);

/// Marks that are not unions: a milestone and a note, standing on the
/// structure rather than in it.
pub(super) const MARKED: &str = concat!(
    "figure ada label=\"Ada Doe\" meta=\"1946\" ",
    face!("ada"),
    "\n",
    "mark   note kind=milestone slot span=2\n",
    "bond   ada note kind=link\n",
    "figure eve label=\"Eve Marsh\" meta=\"1948\" ",
    face!("eve"),
    "\n",
    "mark   u0 kind=union\n",
    "bond   ada u0 kind=union\n",
    "bond   eve u0 kind=union\n",
    "figure cara label=\"Cara Doe\" meta=\"1972\" ",
    face!("cara"),
    "\n",
    "bond   u0 cara\n",
);

/// Ranks and widths said outright, for the cases inference cannot win.
pub(super) const PLACED: &str = "\
figure title label=\"The Doe line\" rank=0 span=3 shape=plaque tone=accent cluster=doe\n\
figure a label=\"Ada Doe\"  meta=\"1946\" rank=1 cluster=doe\n\
figure b label=\"Bob Doe\"  meta=\"1949\" rank=1 cluster=doe\n\
figure c label=\"Cara Doe\" meta=\"1972\" rank=3 cluster=doe\n\
bond   a c\n\
bond   title a\n\
bond   title b\n";

/// The scenes, in the order the page tells them, which between them set
/// every prop the contract declares and show every feature it has.
pub(crate) const EXAMPLES: &[Tree] = &[
    (
        r#"Lineage { name: "doe", label: "…", graph: "figure gran … bond u0 ada" }"#,
        &[Node::Component {
            name: "Lineage",
            props: &[
                ("name", Value::Text("doe")),
                ("label", Value::Message(Key::ExampleLineageDoe)),
                (
                    "graph",
                    Value::Template(
                        ADOPTED,
                        &[
                            ("word", Value::Message(Key::ExampleLineageAdopted)),
                            ("note", Value::Message(Key::ExampleLineageAdoptedNote)),
                        ],
                    ),
                ),
            ],
            children: &[],
        }],
    ),
    (
        r#"Lineage { graph: "wobble", empty_label: "…", height: Short }"#,
        &[Node::Component {
            name: "Lineage",
            props: &[
                ("name", Value::Text("nothing")),
                ("label", Value::Message(Key::ExampleLineageEmpty)),
                ("graph", Value::Text("wobble\n")),
                ("empty_label", Value::Message(Key::ExampleLineageEmpty)),
                ("height", Value::Text("short")),
            ],
            children: &[],
        }],
    ),
    (
        r"Lineage { parallax: true, LineageFigure { parallax: Deep } } — the origin",
        &[Node::Component {
            name: "Lineage",
            props: &[
                ("name", Value::Text("founders")),
                ("label", Value::Message(Key::ExampleLineageSlots)),
                ("graph", Value::Text(OWL)),
            ],
            children: &[
                Node::Component {
                    name: "LineageFigure",
                    props: &[
                        ("scene", Value::Text("founders")),
                        ("at", Value::Text("noctua")),
                        ("shape", Value::Text("round")),
                        ("tone", Value::Text("accent")),
                        ("parallax", Value::Text("deep")),
                        // The sentence is the figure's accessible name rather than a
                        // third line: a round figure grows to whatever is in it, and
                        // three lines of it reached into the row below.
                        ("label", Value::Message(Key::ExampleLineageOwl)),
                    ],
                    children: &[
                        Node::Component {
                            name: "Avatar",
                            props: &[
                                ("label", Value::Data("Noctua")),
                                ("src", Value::Asset("assets/portraits/noctua.svg")),
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
                            children: &[Node::Text(Value::Data("Noctua"))],
                        },
                    ],
                },
                Node::Component {
                    name: "LineageFigure",
                    props: &[
                        ("scene", Value::Text("founders")),
                        ("at", Value::Text("ada")),
                    ],
                    // SMALL, BECAUSE A FIGURE HAS ONE CELL. A portrait, a name
                    // and a line under it at the default size come to more than
                    // that, and the ceiling on a figure clips what is over it —
                    // which it has to, or the figure covers the connection that
                    // reaches it. See `.nd-lineage-figure` in `shell.css`.
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
                                ("size", Value::Text("caption")),
                                ("tone", Value::Text("muted")),
                            ],
                            children: &[Node::Text(Value::Message(Key::ExampleLineageFounder))],
                        },
                    ],
                },
                Node::Component {
                    name: "LineageFigure",
                    props: &[
                        ("scene", Value::Text("founders")),
                        ("at", Value::Text("eve")),
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
            ],
        }],
    ),
    (
        r#""bond ua kid kind=adoption": four true parents  ·  "kind=donor" and "kind=partner""#,
        &[
            Node::Component {
                name: "Lineage",
                props: &[
                    ("name", Value::Text("adoption")),
                    ("label", Value::Message(Key::ExampleLineageAdoption)),
                    (
                        "graph",
                        Value::Template(
                            ADOPTION,
                            &[
                                ("word", Value::Message(Key::ExampleLineageAdopted)),
                                ("nobody-father", Value::Message(Key::ExampleLineageNoFather)),
                                ("nobody-mother", Value::Message(Key::ExampleLineageNoMother)),
                            ],
                        ),
                    ),
                ],
                children: &[],
            },
            Node::Component {
                name: "Lineage",
                props: &[
                    ("name", Value::Text("donor")),
                    ("label", Value::Message(Key::ExampleLineageDonorScene)),
                    (
                        "graph",
                        Value::Template(
                            DONOR,
                            &[
                                ("donor", Value::Message(Key::ExampleLineageDonor)),
                                ("partner", Value::Message(Key::ExampleLineagePartner)),
                                ("nobody", Value::Message(Key::ExampleLineageNoParent)),
                            ],
                        ),
                    ),
                ],
                children: &[],
            },
        ],
    ),
    (
        r#""mark u1 kind=union" twice: one person, two unions  ·  "bond gran u0 kind=union" across generations"#,
        &[
            Node::Component {
                name: "Lineage",
                props: &[
                    ("name", Value::Text("partners")),
                    ("label", Value::Message(Key::ExampleLineagePartners)),
                    ("graph", Value::Text(PARTNERS)),
                ],
                children: &[],
            },
            Node::Component {
                name: "Lineage",
                props: &[
                    ("name", Value::Text("spanning")),
                    ("label", Value::Message(Key::ExampleLineageSpanning)),
                    (
                        "graph",
                        Value::Template(
                            SPANNING,
                            &[
                                ("nobody-mother", Value::Message(Key::ExampleLineageNoMother)),
                                ("nobody-father", Value::Message(Key::ExampleLineageNoFather)),
                            ],
                        ),
                    ),
                ],
                children: &[],
            },
        ],
    ),
    (
        r"Lineage { route: Elbow, height: Short }  ·  Lineage { route: Taut }",
        &[
            Node::Component {
                name: "Lineage",
                props: &[
                    ("name", Value::Text("route-elbow")),
                    ("label", Value::Message(Key::ExampleLineageRouteElbow)),
                    ("graph", Value::Text(PARTNERS)),
                    ("height", Value::Text("short")),
                ],
                children: &[],
            },
            Node::Component {
                name: "Lineage",
                props: &[
                    ("name", Value::Text("route-taut")),
                    ("label", Value::Message(Key::ExampleLineageRouteTaut)),
                    ("graph", Value::Text(PARTNERS)),
                    ("route", Value::Text("taut")),
                    ("height", Value::Text("short")),
                ],
                children: &[],
            },
        ],
    ),
    (
        r"Lineage { size: Small }  ·  Lineage { size: Large, tilt: Resting }",
        &[
            Node::Component {
                name: "Lineage",
                props: &[
                    ("name", Value::Text("size-small")),
                    ("label", Value::Message(Key::ExampleLineageSmall)),
                    ("graph", Value::Text(PAIR)),
                    ("size", Value::Text("small")),
                    ("height", Value::Text("short")),
                ],
                children: &[],
            },
            Node::Component {
                name: "Lineage",
                props: &[
                    ("name", Value::Text("size-large")),
                    ("label", Value::Message(Key::ExampleLineageLarge)),
                    ("graph", Value::Text(PAIR)),
                    ("size", Value::Text("large")),
                    ("tilt", Value::Text("resting")),
                    ("height", Value::Text("short")),
                ],
                children: &[],
            },
        ],
    ),
    (
        r"Lineage { line: Beam, tilt: Raked }  ·  { line: Ribbon }  ·  { thickness: None }",
        &[
            Node::Component {
                name: "Lineage",
                props: &[
                    ("name", Value::Text("beams")),
                    ("label", Value::Message(Key::ExampleLineageBeam)),
                    ("graph", Value::Text(PAIR)),
                    ("line", Value::Text("beam")),
                    ("tilt", Value::Text("raked")),
                    ("height", Value::Text("short")),
                ],
                children: &[],
            },
            Node::Component {
                name: "Lineage",
                props: &[
                    ("name", Value::Text("ribbons")),
                    ("label", Value::Message(Key::ExampleLineageRibbon)),
                    ("graph", Value::Text(PAIR)),
                    ("line", Value::Text("ribbon")),
                    ("tilt", Value::Text("raked")),
                    ("height", Value::Text("short")),
                ],
                children: &[],
            },
            Node::Component {
                name: "Lineage",
                props: &[
                    ("name", Value::Text("thin")),
                    ("label", Value::Message(Key::ExampleLineageFlat)),
                    ("graph", Value::Text(PAIR)),
                    ("thickness", Value::Text("none")),
                    ("tilt", Value::Text("raked")),
                    ("height", Value::Text("short")),
                ],
                children: &[],
            },
        ],
    ),
    (
        r"Lineage { view: Deep, tilt: Resting, height: Full }",
        &[Node::Component {
            name: "Lineage",
            props: &[
                ("name", Value::Text("deep")),
                ("label", Value::Message(Key::ExampleLineageDeep)),
                ("graph", Value::Text(DEEP)),
                ("view", Value::Text("deep")),
                // TURNED, BECAUSE THE CLAIM IS DEPTH. Straight on, four planes
                // one behind another are four rows of shrinking cards; a
                // resting pitch is the smallest camera that shows they are
                // planes at all, and it is the one place on this page where an
                // example opens at an angle on purpose.
                ("tilt", Value::Text("resting")),
                ("height", Value::Text("full")),
            ],
            children: &[],
        }],
    ),
    (
        r#"Lineage { filter: "Doe", filter_mode: Isolate, matches_label, elided_label }"#,
        &[
            Node::Component {
                name: "Lineage",
                props: &[
                    ("name", Value::Text("highlighted")),
                    ("label", Value::Message(Key::ExampleLineageFiltered)),
                    ("graph", Value::Text(FILTERED)),
                    ("filter", Value::Data("Doe")),
                    ("matches_label", Value::Message(Key::ExampleLineageMatches)),
                    ("height", Value::Text("short")),
                ],
                children: &[],
            },
            Node::Component {
                name: "Lineage",
                props: &[
                    ("name", Value::Text("isolated")),
                    ("label", Value::Message(Key::ExampleLineageFiltered)),
                    ("graph", Value::Text(FILTERED)),
                    ("filter", Value::Data("Doe")),
                    ("filter_mode", Value::Text("isolate")),
                    ("matches_label", Value::Message(Key::ExampleLineageMatches)),
                    // WHAT THE GAP IS CALLED. `isolate` leaves a marker standing
                    // where each removed group was; without this it is a numeral
                    // with no name, and a screen reader reads a number on its own.
                    ("elided_label", Value::Message(Key::ExampleLineageElided)),
                    ("height", Value::Text("short")),
                ],
                children: &[],
            },
        ],
    ),
    (
        r#"Lineage { pick: Both, form: "ties" }  ·  { camera_label, zoom_in_label, zoom_out_label, fit_label }"#,
        &[
            Node::Component {
                name: "Lineage",
                props: &[
                    ("name", Value::Text("picked")),
                    ("label", Value::Message(Key::ExampleLineagePicked)),
                    (
                        "graph",
                        Value::Template(
                            ADOPTED,
                            &[
                                ("word", Value::Message(Key::ExampleLineageAdopted)),
                                ("note", Value::Message(Key::ExampleLineageAdoptedNote)),
                            ],
                        ),
                    ),
                    ("pick", Value::Text("both")),
                    ("form", Value::Text("ties")),
                ],
                children: &[],
            },
            Node::Component {
                name: "Lineage",
                props: &[
                    ("name", Value::Text("camera")),
                    ("label", Value::Message(Key::ExampleLineageCamera)),
                    ("graph", Value::Text(DOE)),
                    ("camera_label", Value::Message(Key::ExampleLineageCamera)),
                    ("zoom_in_label", Value::Message(Key::ExampleLineageCloser)),
                    ("zoom_out_label", Value::Message(Key::ExampleLineageFurther)),
                    ("fit_label", Value::Message(Key::ExampleLineageFit)),
                ],
                children: &[],
            },
        ],
    ),
    (
        r#"Lineage { occasion: "christmas" } — every figure has a face for the day"#,
        &[Node::Component {
            name: "Lineage",
            props: &[
                ("name", Value::Text("season")),
                ("label", Value::Message(Key::ExampleLineageSeason)),
                ("graph", Value::Text(SEASON)),
                ("occasion", Value::Text("christmas")),
            ],
            // THE WHOLE SCENE KEEPS THE DAY, and it is the only thing this
            // example shows. It used to share a scene with the origin and
            // with the parallax, so the one figure wearing a hat sat under
            // an owl and beside four people who were not — which is a
            // demonstration of nothing. Three people, six figures, one
            // occasion.
            children: &[
                Node::Component {
                    name: "LineageFigure",
                    props: &[("scene", Value::Text("season")), ("at", Value::Text("ada"))],
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
                        ("scene", Value::Text("season")),
                        ("at", Value::Text("ada")),
                        ("occasion", Value::Text("christmas")),
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
                    props: &[("scene", Value::Text("season")), ("at", Value::Text("eve"))],
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
                    ],
                },
                Node::Component {
                    name: "LineageFigure",
                    props: &[
                        ("scene", Value::Text("season")),
                        ("at", Value::Text("eve")),
                        ("occasion", Value::Text("christmas")),
                    ],
                    children: &[
                        Node::Component {
                            name: "Avatar",
                            props: &[
                                ("label", Value::Data("Eve Marsh")),
                                ("src", Value::Asset("assets/portraits/eve-christmas.svg")),
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
                    ],
                },
                Node::Component {
                    name: "LineageFigure",
                    props: &[
                        ("scene", Value::Text("season")),
                        ("at", Value::Text("cara")),
                    ],
                    children: &[
                        Node::Component {
                            name: "Avatar",
                            props: &[
                                ("label", Value::Data("Cara Doe")),
                                ("src", Value::Asset("assets/portraits/cara.svg")),
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
                            children: &[Node::Text(Value::Data("Cara Doe"))],
                        },
                    ],
                },
                Node::Component {
                    name: "LineageFigure",
                    props: &[
                        ("scene", Value::Text("season")),
                        ("at", Value::Text("cara")),
                        ("occasion", Value::Text("christmas")),
                    ],
                    children: &[
                        Node::Component {
                            name: "Avatar",
                            props: &[
                                ("label", Value::Data("Cara Doe")),
                                ("src", Value::Asset("assets/portraits/cara-christmas.svg")),
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
                            children: &[Node::Text(Value::Data("Cara Doe"))],
                        },
                    ],
                },
            ],
        }],
    ),
    (
        r#"Lineage { sets: "doe:local", LineageFigure { action, swaps: "doe:remote" } }"#,
        &[Node::Component {
            name: "Lineage",
            props: &[
                ("name", Value::Text("server")),
                ("label", Value::Message(Key::ExampleLineageServer)),
                ("graph", Value::Text(SERVER)),
                ("height", Value::Text("short")),
            ],
            children: &[Node::Component {
                name: "LineageFigure",
                props: &[
                    ("scene", Value::Text("server")),
                    ("at", Value::Text("srv")),
                    ("tone", Value::Text("success")),
                    ("action", Value::Text("lineage-swap")),
                    ("swaps", Value::Text("doe:remote")),
                    ("label", Value::Message(Key::ExampleLineageServer)),
                ],
                // A SERVER IS A CARD WITH A NAME AND A STATUS — the round
                // version was reported as "ridiculously just a green
                // circle with another green circle inside".
                children: &[
                    Node::Component {
                        name: "Text",
                        props: &[
                            ("class", Value::Text("nd-identifier")),
                            ("size", Value::Text("small")),
                            ("weight", Value::Text("medium")),
                        ],
                        children: &[Node::Text(Value::Data("Doe"))],
                    },
                    Node::Component {
                        name: "Badge",
                        props: &[
                            ("label", Value::Message(Key::ExampleLineageOnline)),
                            ("dot", Value::Bool(true)),
                            ("tone", Value::Text("success")),
                        ],
                        children: &[],
                    },
                ],
            }],
        }],
    ),
    (
        r#"Lineage { graph: "…origin span=2" }  ·  "figure title rank=0 span=3 cluster=doe"  ·  three families"#,
        &[
            Node::Component {
                name: "Lineage",
                props: &[
                    ("name", Value::Text("aviary")),
                    ("label", Value::Message(Key::ExampleLineageCompany)),
                    ("graph", Value::Text(COMPANY)),
                ],
                children: &[],
            },
            Node::Component {
                name: "Lineage",
                props: &[
                    ("name", Value::Text("placed")),
                    ("label", Value::Message(Key::ExampleLineagePlaced)),
                    ("graph", Value::Text(PLACED)),
                    ("height", Value::Text("full")),
                ],
                children: &[],
            },
            Node::Component {
                name: "Lineage",
                props: &[
                    ("name", Value::Text("four")),
                    ("label", Value::Message(Key::ExampleLineageBranches)),
                    ("graph", Value::Text(FOUR)),
                ],
                children: &[],
            },
        ],
    ),
    (
        r#"…"mark note kind=milestone slot" — a note standing on the structure"#,
        &[Node::Component {
            name: "Lineage",
            props: &[
                ("name", Value::Text("marked")),
                ("label", Value::Message(Key::ExampleLineageMark)),
                ("graph", Value::Text(MARKED)),
                ("height", Value::Text("short")),
            ],
            children: &[Node::Component {
                name: "LineageFigure",
                props: &[
                    ("scene", Value::Text("marked")),
                    ("at", Value::Text("note")),
                    ("shape", Value::Text("card")),
                    ("tone", Value::Text("info")),
                ],
                children: &[
                    Node::Component {
                        name: "Text",
                        props: &[
                            ("size", Value::Text("caption")),
                            ("weight", Value::Text("medium")),
                        ],
                        children: &[Node::Text(Value::Message(Key::ExampleLineageMilestone))],
                    },
                    Node::Component {
                        name: "Text",
                        props: &[
                            ("class", Value::Text("nd-identifier")),
                            ("size", Value::Text("caption")),
                            ("tone", Value::Text("muted")),
                        ],
                        children: &[Node::Text(Value::Data("1970"))],
                    },
                ],
            }],
        }],
    ),
    (
        r#"Lineage { reach: Shy, lettering: Solid, graph: "…shape=bare…" }"#,
        &[Node::Component {
            name: "Lineage",
            props: &[
                ("name", Value::Text("wild")),
                ("label", Value::Message(Key::ExampleLineageWild)),
                ("graph", Value::Text(WILD)),
                ("reach", Value::Text("shy")),
                ("lettering", Value::Text("solid")),
                ("tilt", Value::Text("resting")),
            ],
            children: &[Node::Component {
                name: "LineageFigure",
                props: &[
                    ("scene", Value::Text("wild")),
                    ("at", Value::Text("noctua")),
                    ("shape", Value::Text("bare")),
                    ("label", Value::Message(Key::ExampleLineageOwl)),
                ],
                children: &[
                    // THE MODEL STAGE, WITH THE FILE AS ITS INPUT. The
                    // feature knows nothing about owls: `model.js` takes any
                    // `.glb` through `data-nd-model` plus optional
                    // `data-nd-model-*` parameters, and this example sets
                    // NONE of them — the point is that the defaults carry a
                    // real scanned, rigged, animated model. Until the file
                    // exists (see `assets/vendor/owl/ATTRIBUTION.md`) the
                    // portrait below keeps the stage.
                    Node::Element {
                        tag: "div",
                        attributes: &[
                            ("class", Value::Text("nd-model-stage")),
                            ("data-nd-model", Value::Text("assets/vendor/owl.glb")),
                        ],
                        children: &[Node::Element {
                            tag: "span",
                            attributes: &[("data-nd-model-flat", Value::Text(""))],
                            children: &[Node::Component {
                                name: "Avatar",
                                props: &[
                                    ("label", Value::Data("Noctua")),
                                    ("src", Value::Asset("assets/portraits/noctua.svg")),
                                    ("size", Value::Text("small")),
                                ],
                                children: &[],
                            }],
                        }],
                    },
                    Node::Component {
                        name: "Text",
                        props: &[
                            ("class", Value::Text("nd-identifier")),
                            ("size", Value::Text("small")),
                            ("weight", Value::Text("medium")),
                        ],
                        children: &[Node::Text(Value::Data("Noctua"))],
                    },
                ],
            }],
        }],
    ),
    (
        r#"div { "data-nd-model-size": "2", … }  Text { … }  Text { blurb }"#,
        &[Node::Component {
            name: "Lineage",
            props: &[
                ("name", Value::Text("queen")),
                ("label", Value::Message(Key::ExampleLineageQueen)),
                ("graph", Value::Text(QUEEN)),
                ("reach", Value::Text("shy")),
                ("lettering", Value::Text("solid")),
                ("tilt", Value::Text("resting")),
                // FULL, because the default window is a cap. `tall` holds
                // five pitches at most, and a doubled model's air makes
                // this drawing taller than that — inside a capped window
                // the family slid below the fold. A scene whose point is
                // one large figure shows all of itself.
                ("height", Value::Text("full")),
            ],
            children: &[Node::Component {
                name: "LineageFigure",
                props: &[
                    ("scene", Value::Text("queen")),
                    ("at", Value::Text("noctua")),
                    ("shape", Value::Text("bare")),
                    ("label", Value::Message(Key::ExampleLineageOwl)),
                ],
                children: &[
                    // THE SAME FILE, TWICE THE PRESENTATION: `size` is a
                    // multiplier on the fitted default, and the no-cut
                    // guarantee does not move — the canvas overdraws
                    // proportionally further instead.
                    Node::Element {
                        tag: "div",
                        attributes: &[
                            ("class", Value::Text("nd-model-stage")),
                            ("data-nd-model", Value::Text("assets/vendor/owl.glb")),
                            ("data-nd-model-size", Value::Text("2")),
                        ],
                        children: &[Node::Element {
                            tag: "span",
                            attributes: &[("data-nd-model-flat", Value::Text(""))],
                            children: &[Node::Component {
                                name: "Avatar",
                                props: &[
                                    ("label", Value::Data("Noctua")),
                                    ("src", Value::Asset("assets/portraits/noctua.svg")),
                                    ("size", Value::Text("small")),
                                ],
                                children: &[],
                            }],
                        }],
                    },
                    Node::Component {
                        name: "Text",
                        props: &[
                            ("class", Value::Text("nd-identifier")),
                            ("size", Value::Text("small")),
                            ("weight", Value::Text("medium")),
                        ],
                        children: &[Node::Text(Value::Data("Noctua"))],
                    },
                    // ANY component, in order: the description below the
                    // name is nothing but one more child the figure was
                    // handed — the scene does not care what it is.
                    Node::Component {
                        name: "Text",
                        props: &[
                            ("class", Value::Text("nd-lineage-blurb")),
                            ("size", Value::Text("small")),
                        ],
                        children: &[Node::Text(Value::Message(Key::ExampleLineageQueenCaption))],
                    },
                ],
            }],
        }],
    ),
];
