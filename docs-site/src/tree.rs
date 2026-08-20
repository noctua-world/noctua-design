//! An example, described rather than drawn.
//!
//! # Why an example is data
//!
//! `NOCTUA.md §9.3` requires the Dioxus and the React halves of this system to
//! behave identically, and the components keep that promise by construction:
//! both are generated from `spec/components/<name>.toml`, so they cannot
//! declare different props or write different attributes.
//!
//! The **examples** had no such thing. Every one of them was a Rust closure
//! returning `rsx!`, which only Dioxus can run, so the React package was
//! rendered nowhere on the site that documents it. The audit recorded what that
//! costs: six React modules were broken, two of them did not parse, and the
//! whole suite was green — because nothing ever rendered them.
//!
//! So an example is a **tree** here, and the tree is rendered twice. Rust walks
//! it through the generated interpreter and gets the real typed components;
//! the browser walks the same tree, serialised, through
//! `docs-site/js/example-react.js` and gets the React ones. One authored
//! description, two implementations, and no third place for them to disagree.
//!
//! # Why this shape and not another
//!
//! The 499 examples were surveyed before this was designed, and they turned out
//! to be far simpler than a general rendering language: **no hooks, no event
//! handlers, no branching, and no closures inside any body.** Their only
//! dynamic inputs were a message lookup, a page title in the reader's language,
//! and a handful of asset URLs. So [`Node`] has six shapes and [`Value`] ten,
//! and that is the whole of it. A richer IR would be a language nothing writes.

use dioxus::prelude::*;
use noctua_design::IconName;
use noctua_design::ThemeScope;
use noctua_design_grammars::Language;
use noctua_design_i18n::Key;

use crate::shell::Context;

/// One node of an example.
// `Raw` is the one shape no example reaches: it is the variant grids', whose
// specimens are drawings the contract carries rather than uses of a component.
// `every_shape_the_ir_declares_is_one_a_tree_uses` holds every other one to
// being constructed somewhere, and excludes this by name.
#[allow(
    dead_code,
    reason = "Raw belongs to the variant grids, not the examples"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Node {
    /// A component of the design system, by the name the catalogue gives it.
    ///
    /// The name is a string rather than a type because the same tree has to be
    /// readable by JavaScript, where there are no types to name. The gates make
    /// the string as safe as a type: one of them refuses a name the catalogue
    /// does not carry, and another refuses a prop that component never declared.
    Component {
        /// `Button`, `Table`, `SelectOption` — the catalogue's spelling.
        name: &'static str,
        /// What the call sets, in any order.
        props: &'static [(&'static str, Value)],
        /// What goes inside it.
        children: &'static [Node],
    },
    /// Plain markup, for the arranging an example does around the component.
    ///
    /// `div.nd-example-row` and its handful of relatives. These are the
    /// example's own scaffolding rather than anything this system ships, which
    /// is why they are a separate shape: a gate can then ask that every
    /// component in a tree is a real one without tripping over a `<div>`.
    Element {
        /// The tag. One of the eleven [`element`] draws.
        tag: &'static str,
        /// The attributes, **in source order** — see [`element`].
        attributes: &'static [(&'static str, Value)],
        /// What goes inside it.
        children: &'static [Node],
    },
    /// A run of text.
    Text(Value),
    /// Several nodes where the shape wanted one.
    ///
    /// This is what a shared fragment is made of. Four table examples draw the
    /// same set of columns, and they did it by calling one macro, so the four
    /// could not drift apart by accident. A tree with no fragment would have to
    /// write those columns out four times and lose that.
    Fragment(&'static [Node]),
    /// The same body once per value of a set the system already declares.
    ///
    /// **This is not a convenience.** `Button { variant }` for every variant is
    /// drawn from `ButtonVariant::ALL` precisely so that adding a variant to
    /// `spec/components/button.toml` adds it to the example, and flattening the
    /// loop into six written-out calls would quietly end that. The same
    /// argument is written out at the `Language::ALL` example, whose comment
    /// says a second copy of `spec/grammars/` is a second copy that stops
    /// agreeing.
    Each {
        /// What is being walked.
        over: Each,
        /// The body, once per row, with [`Value::Bound`] standing for the row.
        body: &'static [Node],
    },
    /// Markup the contract itself declares.
    ///
    /// Only for a specimen: about thirty components declare a `specimen` in
    /// their spec that is a **drawing** rather than a use of the component — an
    /// inert box standing in for a control that would otherwise be a nameless
    /// live one on a page that is not a form. It is legal only as the single
    /// child of an [`Node::Element`], because inner HTML is an attribute of the
    /// element that holds it and not a node in its own right.
    Raw(&'static str),
}

/// What an [`Node::Each`] walks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Each {
    /// Every value of an enum prop, read from the contract.
    ///
    /// The one binding is the value as the DOM spells it, so
    /// `Value::Bound(0)` is `"solid"`.
    Values {
        /// The component that declares the prop.
        component: &'static str,
        /// The prop.
        prop: &'static str,
    },
    /// Every language the syntax highlighter knows.
    ///
    /// Two bindings: `Value::Bound(0)` is the name a caller writes, and
    /// `Value::Bound(1)` is how it is displayed.
    Languages,
    /// A literal table, one row per turn.
    Rows(&'static [&'static [Value]]),
}

/// A value in a tree: what a prop is set to, or what a text node says.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Value {
    /// An identifier: a prop value, a class, an id, a form field name.
    ///
    /// Never a sentence. Identifiers are the same in every locale, which is
    /// what `NOCTUA.md §1` says and what makes them safe to write here.
    Text(&'static str),
    /// Literal content that is data rather than a sentence.
    ///
    /// A person's name, an invoice number, a branch, an amount. It reaches the
    /// page as it is written, and the page marks it as an identifier so that
    /// `every_sentence_on_every_page_is_translated` knows an unbracketed word
    /// there is deliberate rather than a missed translation.
    Data(&'static str),
    /// A sentence, by its key, in the reader's language.
    Message(Key),
    /// A literal with holes.
    ///
    /// For the graph a `Lineage` draws, which is a long constant with a few
    /// translated words set into it. The constant is a drawing, not a sentence,
    /// so it is not in the catalogue; the words in it are.
    Template(&'static str, &'static [(&'static str, Value)]),
    /// Yes or no.
    Bool(bool),
    /// A file under `assets/`, written from the page's own depth.
    ///
    /// `{locale}` in the path becomes the locale tag, for the one asset that
    /// has a copy per language.
    Asset(&'static str),
    /// A page's title, in the reader's language.
    PageTitle(&'static str),
    /// The row the innermost [`Node::Each`] is on, by column.
    ///
    /// The innermost, so a nested walk shadows the one around it, which is the
    /// rule every language uses and the one a reader will assume.
    Bound(usize),
    /// Several values, run together.
    ///
    /// A gate holds this to **at most one** message: `Join` is data, so two
    /// messages inside one would assemble a sentence from two catalogue lookups
    /// and walk straight past `gates::translated_fragment`, which refuses
    /// exactly that in source. The legitimate use is one message with literal
    /// punctuation beside it.
    Join(&'static [Value]),
}

/// Where a node is being rendered: the locale, the page, and the row.
///
/// Threaded rather than global, because [`Node::Each`] nests and the row has to
/// shadow. It is `Copy` and two words wide, so passing it costs nothing.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Scope {
    /// The locale and page being built.
    pub context: Context,
    /// The row of the innermost walk, if there is one.
    row: Option<Row>,
}

/// One turn of an [`Node::Each`].
#[derive(Debug, Clone, Copy)]
enum Row {
    /// A row of the literal table.
    Cells(&'static [Value]),
    /// One string, from a set the system declares.
    One(&'static str),
    /// A language: what a caller writes, and how it is shown.
    Language(&'static str, &'static str),
}

impl Scope {
    /// A scope with no walk in progress.
    pub(crate) const fn new(context: Context) -> Self {
        Self { context, row: None }
    }

    /// What a value says, in this locale, on this row.
    #[must_use]
    pub(crate) fn text(self, value: &Value) -> String {
        match value {
            Value::Text(text) | Value::Data(text) => (*text).to_owned(),
            Value::Message(key) => self.context.t(*key).to_owned(),
            Value::Template(text, holes) => {
                let mut out = (*text).to_owned();
                for (name, value) in *holes {
                    out = out.replace(&format!("{{{name}}}"), &self.text(value));
                }
                out
            }
            Value::Bool(flag) => if *flag { "true" } else { "false" }.to_owned(),
            Value::Asset(path) => format!(
                "{}{}",
                crate::site::BASE,
                path.replace("{locale}", self.context.locale.tag())
            ),
            Value::PageTitle(slug) => crate::site::by_slug(slug)
                .map(|page| page.title(self.context.locale))
                .unwrap_or_default()
                .to_owned(),
            Value::Bound(column) => self.bound(*column),
            Value::Join(values) => values.iter().map(|value| self.text(value)).collect(),
        }
    }

    /// What the innermost walk's row says in this column.
    fn bound(self, column: usize) -> String {
        match self.row {
            Some(Row::Cells(cells)) => cells.get(column).map(|cell| self.text(cell)),
            Some(Row::One(value)) => (column == 0).then(|| (*value).to_owned()),
            Some(Row::Language(name, shown)) => match column {
                0 => Some((*name).to_owned()),
                1 => Some((*shown).to_owned()),
                _ => None,
            },
            None => None,
        }
        .unwrap_or_default()
    }

    /// One scope per turn of a walk.
    ///
    /// Scopes rather than rows, so [`Row`] stays private: the serialiser
    /// flattens an [`Node::Each`] before the tree crosses to the browser, and
    /// it needs the same walk this file renders from — not a second one.
    #[must_use]
    pub(crate) fn rows(self, over: Each) -> Vec<Self> {
        rows(over).into_iter().map(|row| self.on(row)).collect()
    }

    /// The same scope, on a row.
    const fn on(self, row: Row) -> Self {
        Self {
            context: self.context,
            row: Some(row),
        }
    }

    // --- Reading a prop, in each of the four types a prop can have ---------
    //
    // The whole surface, and it really is only four: the spec's `type` is
    // `text`, `bool`, `enum` or `children`, and the only Rust type outside that
    // is `IconName`, which eleven components take where the spec says `text`.
    // The generated interpreter calls these and nothing else.

    /// What a prop is set to, or nothing if the call left it out.
    #[must_use]
    pub(crate) fn find(self, props: &[(&'static str, Value)], name: &str) -> Option<String> {
        props
            .iter()
            .find(|(prop, _)| *prop == name)
            .map(|(_, value)| self.text(value))
    }

    /// Whether a bool prop is on, or nothing if the call left it out.
    ///
    /// A bool written as anything but `Value::Bool` is a tree the gates refuse,
    /// so this reads the flag rather than parsing the word back out of it.
    #[must_use]
    pub(crate) fn flag(self, props: &[(&'static str, Value)], name: &str) -> Option<bool> {
        match props.iter().find(|(prop, _)| *prop == name)?.1 {
            Value::Bool(flag) => Some(flag),
            // A walk's row can carry a bool, which is how a variant grid draws
            // both halves of a bool prop from one body.
            other => match self.text(&other).as_str() {
                "true" => Some(true),
                "false" => Some(false),
                _ => None,
            },
        }
    }

    /// The icon a prop names, or nothing if the call left it out.
    ///
    /// A name that is not an icon is `None` rather than a panic here, and a
    /// gate refuses it at build time instead — the same division the React half
    /// makes, where TypeScript refuses it and the runtime throws.
    #[must_use]
    pub(crate) fn icon(self, props: &[(&'static str, Value)], name: &str) -> Option<IconName> {
        IconName::parse(&self.find(props, name)?)
    }

    /// The enum value a prop names, or nothing if the call left it out.
    ///
    /// Generic over the enum so there is **one** of these rather than 133.
    /// Every generated enum carries `ALL` and `as_str`, and `as_str` coerces to
    /// a function pointer, so the pair is all this needs. That is also why no
    /// `from_str` was added to the published components: it would put the value
    /// strings in a second place beside `as_str`, and two copies stop agreeing.
    #[must_use]
    pub(crate) fn pick<T: Copy>(
        self,
        props: &[(&'static str, Value)],
        name: &str,
        all: &[T],
        spelling: fn(T) -> &'static str,
    ) -> Option<T> {
        let wanted = self.find(props, name)?;
        all.iter().copied().find(|value| spelling(*value) == wanted)
    }
}

/// Renders a tree.
pub(crate) fn render(nodes: &'static [Node], scope: Scope) -> Element {
    let drawn: Vec<Element> = nodes.iter().map(|node| one(node, scope)).collect();
    rsx! { {drawn.into_iter()} }
}

/// Renders one node.
fn one(node: &'static Node, scope: Scope) -> Element {
    match node {
        Node::Component {
            name,
            props,
            children,
        } if *name == "ThemeScope" => theme_scope(props, children, scope),
        Node::Component {
            name,
            props,
            children,
        } => crate::generated::interpreter::component(name, props, children, scope),
        Node::Element {
            tag,
            attributes,
            children,
        } => element(tag, attributes, children, scope),
        Node::Text(value) => {
            let text = scope.text(value);
            rsx! { "{text}" }
        }
        Node::Fragment(nodes) => render(nodes, scope),
        Node::Each { over, body } => {
            let drawn: Vec<Element> = scope
                .rows(*over)
                .into_iter()
                .map(|row| render(body, row))
                .collect();
            rsx! { {drawn.into_iter()} }
        }
        // Alone it is nothing: inner HTML belongs to the element that holds it.
        // `element` below lifts it onto its parent, and a gate refuses it
        // anywhere else.
        Node::Raw(_) => rsx! {},
    }
}

/// `ThemeScope`, which the generated interpreter cannot carry.
///
/// It is the one component a tree can name that is not in
/// `spec/components/`: it comes from `spec/theme.toml`, which describes the
/// settings a scope can ask for rather than a component's props. So the arm is
/// written here, once, beside the shapes it uses.
///
/// Leaving it out is what took the React `ThemeScope` off the site entirely for
/// a few hours, which is the condition that let six other React modules rot.
fn theme_scope(
    props: &'static [(&'static str, Value)],
    children: &'static [Node],
    scope: Scope,
) -> Element {
    use noctua_design::model::{
        Accent, Backdrop, Font, Glass, Grain, Mode, Palette, Pattern, Radius, Saturation,
    };
    rsx! {
        ThemeScope {
            mode: scope.pick(props, "mode", &Mode::ALL, Mode::as_str),
            accent: scope.pick(props, "accent", &Accent::ALL, Accent::as_str),
            saturation: scope.pick(props, "saturation", &Saturation::ALL, Saturation::as_str),
            palette: scope.pick(props, "palette", &Palette::ALL, Palette::as_str),
            font: scope.pick(props, "font", &Font::ALL, Font::as_str),
            glass: scope.pick(props, "glass", &Glass::ALL, Glass::as_str),
            radius: scope.pick(props, "radius", &Radius::ALL, Radius::as_str),
            backdrop: scope.pick(props, "backdrop", &Backdrop::ALL, Backdrop::as_str),
            pattern: scope.pick(props, "pattern", &Pattern::ALL, Pattern::as_str),
            grain: scope.pick(props, "grain", &Grain::ALL, Grain::as_str),
            class: scope.find(props, "class").unwrap_or_default(),
            {render(children, scope)}
        }
    }
}

/// Every row of a walk.
fn rows(over: Each) -> Vec<Row> {
    match over {
        Each::Values { component, prop } => crate::generated::catalogue::COMPONENTS
            .iter()
            .find(|entry| entry.name == component)
            .and_then(|entry| entry.props.iter().find(|entry| entry.name == prop))
            .map(|prop| prop.values.iter().map(|value| Row::One(value)).collect())
            .unwrap_or_default(),
        Each::Languages => Language::ALL
            .iter()
            .map(|language| Row::Language(language.as_str(), language.display()))
            .collect(),
        Each::Rows(rows) => rows.iter().map(|cells| Row::Cells(cells)).collect(),
    }
}

/// Renders one plain element.
///
/// # Why every attribute goes through the spread
///
/// `rsx!` needs an attribute's **name** to be written into the source, and a
/// tree carries names as data. The escape is `..attrs`, which takes a
/// `Vec<Attribute>` and lets the name be a value.
///
/// It has to be all of them or none. Spread attributes are written out in
/// vector order immediately after the tag, so mixing a written `class:` with a
/// spread would put the two in an order that depends on which is which — and
/// this file's whole safety net is that the markup comes out byte for byte the
/// same as the closure it replaced. Hence: everything in the spread, in the
/// order the tree lists it.
///
/// A bool stays a bool rather than becoming `"true"`, because the two render
/// differently — `novalidate=true` unquoted against `novalidate="true"` — and
/// eight examples on this site carry one.
fn element(
    tag: &'static str,
    attributes: &'static [(&'static str, Value)],
    children: &'static [Node],
    scope: Scope,
) -> Element {
    let mut attrs: Vec<Attribute> = attributes
        .iter()
        .map(|(name, value)| match value {
            Value::Bool(flag) => Attribute::new(name, *flag, None, false),
            other => Attribute::new(name, scope.text(other), None, false),
        })
        .collect();

    // A drawing the contract carries, lifted onto the element that holds it.
    // `dangerous_inner_html` is matched by name in the SSR renderer, so it
    // arrives through the spread like everything else.
    if let [Node::Raw(markup)] = children {
        attrs.push(Attribute::new(
            "dangerous_inner_html",
            (*markup).to_owned(),
            None,
            false,
        ));
        return draw(tag, attrs, rsx! {});
    }

    draw(tag, attrs, render(children, scope))
}

/// Every tag a tree may name.
///
/// A written-out match rather than anything cleverer, because `rsx!` needs the
/// tag in the source. A tag that is not here renders nothing and a gate refuses
/// it, so the next one arrives as a build failure rather than as a hole in a
/// page.
///
/// **The table parts and the figure belong to the variant grids**, not to the
/// examples. A specimen of a `<td>` has to arrive inside a `<tr>` inside a
/// `<table>` or the parser throws the start tag away, and every specimen is
/// drawn in a `<figure>` captioned with the value it shows. See
/// `crates/noctua-design-emit/src/specimens.rs`.
fn draw(tag: &'static str, attrs: Vec<Attribute>, inside: Element) -> Element {
    match tag {
        "a" => rsx! { a { ..attrs, {inside} } },
        "button" => rsx! { button { ..attrs, {inside} } },
        "colgroup" => rsx! { colgroup { ..attrs, {inside} } },
        "div" => rsx! { div { ..attrs, {inside} } },
        "dl" => rsx! { dl { ..attrs, {inside} } },
        "fieldset" => rsx! { fieldset { ..attrs, {inside} } },
        "figcaption" => rsx! { figcaption { ..attrs, {inside} } },
        "figure" => rsx! { figure { ..attrs, {inside} } },
        "form" => rsx! { form { ..attrs, {inside} } },
        "h4" => rsx! { h4 { ..attrs, {inside} } },
        "input" => rsx! { input { ..attrs } },
        "label" => rsx! { label { ..attrs, {inside} } },
        "legend" => rsx! { legend { ..attrs, {inside} } },
        "li" => rsx! { li { ..attrs, {inside} } },
        "nav" => rsx! { nav { ..attrs, {inside} } },
        "ol" => rsx! { ol { ..attrs, {inside} } },
        "p" => rsx! { p { ..attrs, {inside} } },
        "section" => rsx! { section { ..attrs, {inside} } },
        "span" => rsx! { span { ..attrs, {inside} } },
        "table" => rsx! { table { ..attrs, {inside} } },
        "tbody" => rsx! { tbody { ..attrs, {inside} } },
        "td" => rsx! { td { ..attrs, {inside} } },
        "th" => rsx! { th { ..attrs, {inside} } },
        "tr" => rsx! { tr { ..attrs, {inside} } },
        "ul" => rsx! { ul { ..attrs, {inside} } },
        _ => rsx! {},
    }
}

/// Every tag [`draw`] has an arm for.
///
/// Named here so a gate can read it, rather than being a fact only the match
/// knows. A list that has to agree with a match beside it is a list that goes
/// stale, so the test below reads this one against that one.
// Read by the gates rather than by the site, which is what it is for: a list
// that has to agree with a match beside it is a list that goes stale.
#[allow(
    dead_code,
    reason = "the gates read it; the render path reads the match"
)]
pub(crate) const TAGS: [&str; 25] = [
    "a",
    "button",
    "colgroup",
    "div",
    "dl",
    "fieldset",
    "figcaption",
    "figure",
    "form",
    "h4",
    "input",
    "label",
    "legend",
    "li",
    "nav",
    "ol",
    "p",
    "section",
    "span",
    "table",
    "tbody",
    "td",
    "th",
    "tr",
    "ul",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_tag_this_file_names_is_one_it_draws() {
        // `draw` returns an empty element for a tag it does not know, which is
        // silent. This is what makes it loud: the source of the match is read
        // for each name, so a tag added to the list and not to the match fails
        // here rather than emptying a page.
        let source = include_str!("tree.rs");
        for tag in TAGS {
            assert!(
                source.contains(&format!("\"{tag}\" => rsx! {{")),
                "`{tag}` is in TAGS and has no arm in `draw`"
            );
        }
    }

    #[test]
    fn the_tags_are_sorted_and_unique() {
        let mut sorted = TAGS;
        sorted.sort_unstable();
        assert_eq!(TAGS.to_vec(), sorted.to_vec(), "TAGS is out of order");
        let mut seen = sorted.to_vec();
        seen.dedup();
        assert_eq!(seen.len(), TAGS.len(), "TAGS repeats a tag");
    }
}
