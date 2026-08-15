//! The variant grids, as trees.
//!
//! # What this replaces
//!
//! A specimen used to be a **string** the documentation site assembled at
//! render time: the element, the root class, the prop's attribute, and every
//! other enum prop at its default. That string was a picture of what a string
//! builder believed the component would render, and the two could disagree with
//! nothing to report it. They did, three times over. `data-nd-data-nd-size` on
//! fourteen pages, four tones drawn as four untouched grey buttons, and
//! `data-nd-kind="ordered"` on a `<ul>`. Each was a grid faithfully documenting
//! a bug in the grid.
//!
//! A specimen is a tree here, so the page renders the **real typed component**
//! through `docs-site/src/generated/interpreter.rs`, and the browser renders the
//! same tree with the React one. Neither of them writes the attributes. The
//! component does.
//!
//! # Why a component call is not always possible
//!
//! About thirty components carry a `specimen` in their contract that is a
//! **drawing** rather than a use of the component. A live `<input>` in a grid is
//! a control a reader can type into by mistake, so those contracts ship an inert
//! figure instead. A drawing is `Node::Raw` on the component's own element,
//! which is what the string used to build, and rendering it in React proves the
//! markup matches and nothing more.
//!
//! The generated file names every one of them, so which grids show a component
//! and which show only a picture is a fact a reader can look up.
//!
//! # Why the figure is generated with the specimen
//!
//! The caption is the specimen's accessible name wherever the control has no
//! other, so the two travel together. Generating the figure once means the page
//! and the React mount draw it from one description, rather than from a Rust
//! copy and a JSON copy that can drift.

use std::collections::BTreeSet;
use std::fmt::Write as _;

use noctua_design_spec::Component;
use noctua_design_spec::component::{Kind, Prop, slug};

use crate::{COMPONENT_SPEC, CommentStyle, EmittedFile, constant, controls, header_from};

/// The class the stylesheet paints a figure with.
///
/// These four are the documentation site's own rather than the design system's.
/// They are here because a picture and the caption naming it are one figure, and
/// one description of that figure is what stops two renderers disagreeing.
const FIGURE: &str = "nd-variant-item";
/// The box the specimen is drawn in.
const STAGE: &str = "nd-variant-stage";
/// The caption under it.
const LABEL: &str = "nd-variant-label";
/// The marker saying a run of text is a name rather than a sentence.
const IDENTIFIER: &str = "nd-identifier";

/// The pangram, for a specimen whose element is a block.
///
/// A message rather than a constant, because a Portuguese page printing an
/// English pangram is what this key already exists to prevent.
const PANGRAM: &str = "Key::TypeSpecimenLine";

/// The build number a column specimen counts rows for.
const BUILD: &str = "#4213"; // allow-literal: a build number in a specimen, not a colour

/// Elements that hold no content of their own.
///
/// A void element cannot contain its own figure. `<input class="nd-input">fig`
/// closes itself and the figure lands beside it, so every rule shaped
/// `.nd-input[data-nd-size] .nd-input-box` matches nothing. `<meter>` and
/// `<progress>` are replaced rather than void and fail the same way, because an
/// engine that supports them draws a shadow tree and never renders the fallback.
/// `<textarea>` is the loudest of the three: its content is the **value**, so a
/// drawing inside one is shown to the reader as prose.
const REPLACED: [&str; 9] = [
    "input", "img", "br", "hr", "col", "embed", "meter", "progress", "textarea",
];

/// Elements a specimen reads better inside with a sentence than with a name.
const BLOCK: [&str; 13] = [
    "p",
    "div",
    "blockquote",
    "ul",
    "ol",
    "dl",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "pre",
];

/// Clippy's ceiling for one function, in the lookup's own arms.
const LONG: usize = 100;

/// The grids, as one file.
#[must_use]
pub fn emit(components: &[Component]) -> Vec<EmittedFile> {
    vec![EmittedFile::new(
        "docs-site/src/generated/specimens.rs",
        specimens(components),
    )]
}

fn specimens(components: &[Component]) -> String {
    let mut out = header_from(CommentStyle::Line("//!"), COMPONENT_SPEC);
    out.push_str(
        "//!\n\
         //! Every variant grid, as trees.\n\
         //!\n\
         //! One constant per component, prop and value, and a lookup over the three.\n\
         //! See `crate::tree` for what a tree is, and the emitter that wrote this for\n\
         //! why a specimen is one.\n\
         //!\n",
    );
    out.push_str(&drawings_note(components));
    out.push_str("\nuse noctua_design_i18n::Key;\n\nuse crate::tree::{Node, Value};\n\n");

    let mut arms = String::new();
    for component in components {
        for (prop_name, prop) in drawn(component) {
            for value in &prop.values {
                let name = constant_for(component, prop_name, value);
                let _ = writeln!(
                    out,
                    "/// `{}.{prop_name}` at `{value}`.\nconst {name}: &[Node] = &[{}];\n",
                    component.meta.name,
                    figure(component, prop_name, prop, value)
                );
                let _ = writeln!(
                    arms,
                    "        (\"{}\", \"{prop_name}\", \"{value}\") => {name},",
                    component.meta.name
                );
            }
        }
    }

    out.push_str(
        "/// The specimen for one value of one prop, or nothing where no grid is drawn.\n\
         ///\n\
         /// A prop the contract marks `variants = false` has no grid, and neither has\n\
         /// one that is not an enum. Both answer with an empty tree, which draws an\n\
         /// empty figure rather than a wrong one.\n",
    );
    if arms.lines().count() > LONG {
        out.push_str(
            "#[allow(\n    clippy::too_many_lines,\n    reason = \"one arm per specimen, \
             and the specimen set is the point\"\n)]\n",
        );
    }
    out.push_str(
        "pub(crate) fn nodes(component: &str, prop: &str, value: &str) -> &'static [Node] {\n    \
         match (component, prop, value) {\n",
    );
    out.push_str(&arms);
    out.push_str("        _ => &[],\n    }\n}\n");
    out
}

/// Which components are documented by a picture rather than by themselves.
///
/// Written into the generated file so the answer is where a reader is standing
/// when the question occurs to them. A drawing shows what the contract says the
/// markup is, and checking it against the component means opening the component.
fn drawings_note(components: &[Component]) -> String {
    let mut named: BTreeSet<String> = BTreeSet::new();
    for component in components {
        // Per prop rather than per component, because a prop may carry a
        // specimen of its own where the component's cannot show it. `Button` is
        // the case: four of its grids are the real control, and `justify` is a
        // picture, because one glyph has nothing to be pushed apart from.
        let pictured: Vec<&str> = drawn(component)
            .filter(|(_, prop)| {
                prop.values
                    .first()
                    .is_some_and(|value| content(component, prop, value).is_drawing())
            })
            .map(|(prop_name, _)| prop_name.as_str())
            .collect();
        if !pictured.is_empty() {
            named.insert(format!("{}: {}", component.meta.name, pictured.join(", ")));
        }
    }

    let mut out = String::from(
        "//! # The grids that draw a picture rather than the component\n\
         //!\n\
         //! Each of these carries a `specimen` its own props cannot take: markup in\n\
         //! nearly every case, because a live control in a grid is one a reader can\n\
         //! operate by mistake, and for `Rating` the name of an icon it draws itself.\n\
         //! Their grids draw the element the contract names, carrying the attributes\n\
         //! the contract declares, with that picture inside. Rendering one shows the\n\
         //! markup matches in both languages and nothing more about the component.\n\
         //!\n\
         //! The prop is named because a prop may carry a picture where the component\n\
         //! carries none, and the other way round.\n\
         //!\n",
    );
    for name in named {
        let _ = writeln!(out, "//! - `{name}`");
    }
    out
}

/// Every prop of a component that gets a grid.
///
/// The same question `pages::drawn` asks, and it has to stay the same one. A
/// prop this walks and the page does not is a specimen generated for a grid
/// nobody draws, and the other way round is a grid with nothing in it.
fn drawn(component: &Component) -> impl Iterator<Item = (&String, &Prop)> {
    component
        .attributes()
        .filter(|(_, prop)| prop.kind == Kind::Enum && prop.variants)
}

/// `BUTTON_VARIANT_SOLID`, the constant one specimen is bound to.
fn constant_for(component: &Component, prop: &str, value: &str) -> String {
    let name = format!(
        "{}_{}_{}",
        constant(&slug(&component.meta.name)),
        constant(prop),
        constant(value)
    );
    name.replace(|ch: char| !ch.is_ascii_alphanumeric(), "_")
}

/// What a specimen puts inside the component.
enum Content {
    /// Markup the contract carries, because a live control here is a control a
    /// reader can operate by mistake.
    Drawing(String),
    /// An icon the component declares a prop for.
    Icon(String),
    /// An icon the component has no prop for, so the grid draws it inside.
    LooseIcon(String),
    /// The pangram, which is the sentence a block element is read with.
    Pangram,
    /// The component's own name, which is what a reader is looking for.
    Name,
    /// Nothing at all.
    Empty,
}

impl Content {
    /// Whether the grid draws a picture instead of the component.
    const fn is_drawing(&self) -> bool {
        matches!(self, Self::Drawing(_) | Self::LooseIcon(_))
    }
}

/// What this specimen shows, from the contract alone.
///
/// A prop's own specimen beats the component's. One component has one specimen
/// and two props can want opposite things from it: `TableCell.align` needs a
/// value short enough to leave room to move in, and `TableCell.wrap` needs one
/// long enough to break.
fn content(component: &Component, prop: &Prop, value: &str) -> Content {
    let specimen = prop
        .specimen
        .as_deref()
        .or(component.meta.specimen.as_deref());
    match specimen {
        // A drawing, and the `<` is the whole test: the only other thing a
        // specimen can be is the name of an icon.
        Some(markup) if markup.starts_with('<') => Content::Drawing(markup.to_owned()),
        Some("") => Content::Empty,
        Some(icon) if takes_icon(component) => Content::Icon(icon.to_owned()),
        Some(icon) => Content::LooseIcon(icon.to_owned()),
        None if !component.takes_children() => Content::Empty,
        None if BLOCK.contains(&element_for(component, prop, value)) => Content::Pangram,
        None => Content::Name,
    }
}

/// Whether the component draws an icon a caller names.
///
/// `Rating` is the case this exists for. Its specimen is a star and it has no
/// icon prop, because the stars are its own body. So the star is drawn inside
/// the picture rather than handed to the component.
fn takes_icon(component: &Component) -> bool {
    component.prop.contains_key("icon") && controls::dioxus_type(component, "icon").is_some()
}

/// The element this variant renders.
///
/// A prop carrying `elements` picks it, position by position, and every other
/// prop gets the component's default. Reading it from the contract is what stops
/// a page showing `data-nd-kind="ordered"` on a `<ul>`, which is what it showed
/// while that prop was inert.
fn element_for<'a>(component: &'a Component, prop: &'a Prop, value: &str) -> &'a str {
    prop.values
        .iter()
        .position(|candidate| candidate == value)
        .and_then(|at| prop.elements.get(at))
        .map_or(component.meta.element.as_str(), String::as_str)
}

/// One specimen: the figure, the stage it is drawn in, and its caption.
fn figure(component: &Component, prop_name: &str, prop: &Prop, value: &str) -> String {
    let slug = slug(&component.meta.name);
    let element = element_for(component, prop, value);
    // THE CAPTION IS THE SPECIMEN'S ACCESSIBLE NAME.
    //
    // A picture of a control that must have a name has none of its own, because
    // its only content is the contract's figure. `IconButton` shipped six
    // nameless controls that way and nothing caught it. The caption already says
    // which value this is, in the one word that is the same in every locale, so
    // it is pointed at rather than copied.
    let caption = format!("nd-variant-{slug}-{prop_name}-{value}");

    let mut label = vec![component_node(
        "Code",
        &[("class", text(IDENTIFIER))],
        &[node_text(&text(value))],
    )];
    // The element, said out loud where the class does not say it, and always for
    // a prop that CHOOSES the element, because that is the whole variant.
    if component.meta.class.is_none() || !prop.elements.is_empty() {
        label.push(component_node(
            "VisuallyHidden",
            &[("class", text(IDENTIFIER))],
            &[node_text(&text(element))],
        ));
    }

    element_node(
        "figure",
        &[("class", text(FIGURE))],
        &[
            element_node(
                "div",
                &[
                    ("class", text(STAGE)),
                    // WHICH PROP THIS IS A PICTURE OF. The stylesheet reads it
                    // for the rare prop whose sentence assumes a box the stage
                    // does not give it, and the browser sweep that fails when
                    // two values of one prop draw the same thing needs to know
                    // where one prop's row ends and the next begins.
                    ("data-nd-prop", text(prop_name)),
                    // And whose prop it is. A collection page carries several
                    // components' grids in one flow, and two of them may declare
                    // the same name.
                    ("data-nd-slug", text(&slug)),
                ],
                &[stage(component, prop_name, prop, value, &caption)],
            ),
            element_node(
                "figcaption",
                &[("class", text(LABEL)), ("id", text(&caption))],
                &label,
            ),
        ],
    )
}

/// What goes in the stage: the component, or the picture the contract carries.
fn stage(
    component: &Component,
    prop_name: &str,
    prop: &Prop,
    value: &str,
    caption: &str,
) -> String {
    match content(component, prop, value) {
        Content::Drawing(markup) => {
            // A CELL'S CONTENT SITS IN THE BOX THE COMPONENT PUTS IT IN.
            // `TableCell` wraps its children in `.nd-table-value`, and every
            // rule that acts on the value rather than on the cell selects that
            // box. Without it the `truncate` specimen wrapped like `wrap`, and
            // the grid drew three values as two pictures.
            let inside = if matches!(element_for(component, prop, value), "td" | "th") {
                format!("<div class=\"nd-table-value\">{markup}</div>")
            } else {
                markup
            };
            picture(component, prop_name, prop, value, caption, &raw(&inside))
        }
        Content::LooseIcon(icon) => {
            let drawn = component_node("Icon", &[("icon", text(&icon))], &[]);
            picture(component, prop_name, prop, value, caption, &drawn)
        }
        Content::Icon(icon) => call(component, prop_name, prop, value, Some(&icon)),
        Content::Pangram | Content::Name | Content::Empty => {
            call(component, prop_name, prop, value, None)
        }
    }
}

/// The component itself, with one prop set to one value.
///
/// Every other attribute is the component's own. A specimen used to write each
/// enum prop's default beside the one under test, because half the stylesheet is
/// compound and `data-nd-tone` alone matches no rule. A real call needs none of
/// that: the component writes its own defaults, and its bools with them, which
/// the string never did.
fn call(
    component: &Component,
    prop_name: &str,
    prop: &Prop,
    value: &str,
    icon: Option<&str>,
) -> String {
    let mut props: Vec<(&str, String)> = Vec::new();
    let mut named = false;
    for (name, declared) in component.attributes() {
        if name == prop_name {
            props.push((name, text(value)));
        } else if let Some(icon) = icon.filter(|_| name == "icon") {
            props.push((name, text(icon)));
        } else if declared.required && declared.kind == Kind::Text {
            // A REQUIRED PROP THE GRID DOES NOT EXERCISE TAKES THE COMPONENT'S
            // OWN NAME.
            //
            // Something has to fill it, and the alternative is a sentence
            // invented here, in English, for a page built in three languages.
            // The name answers `needs_name` honestly: a control whose label is
            // required is named by that prop, which is the component's own
            // answer rather than an attribute bolted on beside it.
            //
            // AND IT IS THE SAME STRING FOR EVERY VALUE, which is the part that
            // matters. A row of pictures differing in their words as well as in
            // the prop under test is a row where two values painted alike still
            // measure apart, and `variants.spec.js` reads exactly that
            // measurement.
            props.push((name, text(&component.meta.name)));
            named = true;
        }
    }

    let content = content(component, prop, value);
    let children = match content {
        Content::Pangram => vec![node_text(&format!("Value::Message({PANGRAM})"))],
        Content::Name => vec![node_text(&text(&component.meta.name))],
        _ => vec![],
    };
    // A WORD THAT NEVER WENT THROUGH THE CATALOGUE SAYS SO.
    //
    // The pseudo-locale sweep reads every run of text on every page and fails on
    // one carrying no brackets, which is how thirteen pages of English
    // documentation shipped in Portuguese. A specimen showing the component's
    // name, or a control named after the value under it, is showing an
    // identifier, and this is the markup saying so.
    if named || matches!(content, Content::Name) {
        props.push(("class", text(IDENTIFIER)));
    }

    component_node(&component.meta.name, &props, &children)
}

/// The element the contract names, carrying the picture the contract carries.
fn picture(
    component: &Component,
    prop_name: &str,
    prop: &Prop,
    value: &str,
    caption: &str,
    inside: &str,
) -> String {
    let element = element_for(component, prop, value);
    let mut attributes: Vec<(&str, String)> = Vec::new();
    if let Some(class) = component.meta.class.as_deref() {
        attributes.push(("class", text(class)));
    }
    // An unset `type` inside a form is `submit`. Nothing on these pages is in a
    // form today, and "today" is the word that makes it worth writing: a grid
    // ships markup, and markup gets copied.
    if element == "button" && prop.attribute.as_deref() != Some("type") && prop_name != "type" {
        // allow-literal: an HTML attribute value rather than a token, and the default this element should always carry
        attributes.push(("type", text("button")));
    }
    if component.a11y.needs_name {
        attributes.push(("aria-labelledby", text(caption)));
    }
    // AND EVERY OTHER ENUM PROP AT ITS DEFAULT, WHICH IS WHAT THE COMPONENT
    // WOULD HAVE WRITTEN.
    //
    // Half the stylesheet is compound.
    // `.nd-button[data-nd-variant="solid"][data-nd-tone="danger"]` is the shape
    // every tone in this system is painted with, so a picture carrying
    // `data-nd-tone` alone matches no rule at all.
    //
    // ENUMS ONLY, and the exclusion is the interesting half. `pressed`,
    // `expanded` and `current` are bools whose absence is meaningful, so one
    // written at its default would make the picture state the opposite of the
    // contract it illustrates.
    for (name, other) in component.attributes() {
        if name == prop_name || other.kind != Kind::Enum || !other.elements.is_empty() {
            continue;
        }
        let Some(default) = other.default.as_deref() else {
            continue;
        };
        let attribute = other.attribute.as_deref().unwrap_or(name);
        // `type` is already written above, with the reason a picture of a
        // control has to carry it at all.
        if attribute == "type" && element == "button" {
            continue;
        }
        // AND NEVER AN ARIA ATTRIBUTE. The defaults are here so compound CSS
        // matches, no rule in this system selects on an `aria-*` name, and what
        // a specimen invented was `aria-sort="none"` on eight still `<th>`
        // pictures. Each announced itself as a column somebody can sort by.
        if attribute.starts_with("aria-") {
            continue;
        }
        attributes.push((attribute, text(default)));
    }
    // A prop that chooses the element carries no attribute at all.
    if prop.elements.is_empty() {
        attributes.push((prop.attribute.as_deref().unwrap_or(prop_name), text(value)));
    }

    // So a void or replaced element with something to show becomes a `<span>`
    // carrying the same class and the same attributes. Nothing is lost, because
    // the real control draws nothing here anyway.
    let tag = if REPLACED.contains(&element) {
        "span"
    } else {
        element
    };
    in_table(tag, &element_node(tag, &attributes, &[inside.to_owned()]))
}

/// A table part, put back inside a table.
///
/// # Why a specimen of a `<td>` is otherwise invisible
///
/// The browser parses a stage's content in the **"in body" insertion mode**,
/// where a `<caption>`, `<colgroup>`, `<thead>`, `<tbody>`, `<tfoot>`, `<tr>`,
/// `<td>` or `<th>` start tag is a parse error that is ignored. Every variant of
/// every table part would render nothing at all.
///
/// And it would render nothing on a page that otherwise looks finished. The
/// caption under the empty box still names the value, so the grid reads as a
/// component whose variants do nothing rather than as markup the parser threw
/// away.
fn in_table(element: &str, node: &str) -> String {
    let inside = node.to_owned();
    match element {
        // A `Table` always renders inside its scroller, so a specimen of one has
        // to as well. `height` caps the SCROLLER, and four heights drawn as four
        // identical boxes would document a prop that does nothing.
        "table" => element_node("div", &[("class", text("nd-table-scroll"))], &[inside]),
        // A COLUMN NEEDS A ROW TO PAINT. A `<colgroup>` alone in a table with no
        // rows is a declaration about nothing, and the grid drew five blank
        // boxes for `width`. The row is the same two identifiers every other
        // specimen in this family uses.
        "colgroup" => element_node("table", &[("class", text("nd-table"))], &[inside, row()]),
        "caption" | "thead" | "tbody" | "tfoot" => {
            element_node("table", &[("class", text("nd-table"))], &[inside])
        }
        "tr" => element_node(
            "table",
            &[("class", text("nd-table"))],
            &[element_node("tbody", &[], &[inside])],
        ),
        // A cell needs BOTH ancestors. A `<td>` inside a bare `<table>` is the
        // same ignored start tag, because "in table" drops one too and only "in
        // row" accepts it.
        //
        // AND A CELL NEEDS ROOM TO ALIGN IN. Wrapped in a table sized to its own
        // content, `start`, `center` and `end` are three identical pictures. The
        // class gives the wrapper a width and the stylesheet decides how much.
        "td" | "th" => element_node(
            "table",
            &[("class", text("nd-table nd-table-specimen"))],
            &[element_node(
                "tbody",
                &[],
                &[element_node("tr", &[], &[inside])],
            )],
        ),
        _ => inside,
    }
}

/// The row a column specimen is measured against.
fn row() -> String {
    element_node(
        "tbody",
        &[],
        &[element_node(
            "tr",
            &[("class", text("nd-table-row"))],
            &[
                element_node(
                    "th",
                    &[
                        ("class", text("nd-table-header-cell nd-identifier")),
                        ("scope", text("row")),
                    ],
                    &[node_text(&text(BUILD))],
                ),
                element_node(
                    "td",
                    &[("class", text("nd-table-cell nd-identifier"))],
                    &[node_text(&text("51s"))],
                ),
            ],
        )],
    )
}

// --- One node, as the source that builds it ---------------------------------

/// `Node::Element { … }`.
fn element_node(tag: &str, attributes: &[(&str, String)], children: &[String]) -> String {
    format!(
        "Node::Element {{ tag: {tag:?}, attributes: &[{}], children: &[{}] }}",
        pairs(attributes),
        children.join(", ")
    )
}

/// `Node::Component { … }`.
fn component_node(name: &str, props: &[(&str, String)], children: &[String]) -> String {
    format!(
        "Node::Component {{ name: {name:?}, props: &[{}], children: &[{}] }}",
        pairs(props),
        children.join(", ")
    )
}

/// `Node::Text(…)`.
fn node_text(value: &str) -> String {
    format!("Node::Text({value})")
}

/// `Node::Raw(…)`.
fn raw(markup: &str) -> String {
    format!("Node::Raw({markup:?})")
}

/// `Value::Text(…)`.
fn text(value: &str) -> String {
    format!("Value::Text({value:?})")
}

fn pairs(values: &[(&str, String)]) -> String {
    values
        .iter()
        .map(|(name, value)| format!("({name:?}, {value})"))
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shipped() -> Vec<Component> {
        let root = std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../.."));
        noctua_design_spec::component::load_all(root.join("spec/components")).expect("components")
    }

    #[test]
    fn every_value_of_every_drawn_prop_has_a_specimen() {
        let components = shipped();
        let source = specimens(&components);
        let mut found = 0usize;
        for component in &components {
            for (prop_name, prop) in drawn(component) {
                for value in &prop.values {
                    found += 1;
                    assert!(
                        source.contains(&format!(
                            "(\"{}\", \"{prop_name}\", \"{value}\") =>",
                            component.meta.name
                        )),
                        "{}.{prop_name} at {value} has no specimen",
                        component.meta.name
                    );
                }
            }
        }
        // A walk that found nothing would pass on every component, for ever.
        assert!(found > 100, "only {found} specimens were read");
    }

    /// A control with no name is the defect this whole layer sits under.
    ///
    /// `IconButton` shipped six of them. There are three ways a specimen can
    /// have one, and every named control has to take one of them: the words
    /// inside it, the prop the contract requires for the purpose, or the caption
    /// beside it. This reads the emitted source rather than trusting the branch
    /// that wrote it.
    #[test]
    fn every_specimen_of_a_named_control_has_a_name() {
        for component in shipped() {
            if !component.a11y.needs_name {
                continue;
            }
            for (prop_name, prop) in drawn(&component) {
                for value in &prop.values {
                    let caption = format!(
                        "nd-variant-{}-{prop_name}-{value}",
                        slug(&component.meta.name)
                    );
                    // THE PICTURE, NOT THE FIGURE AROUND IT. Read over the whole
                    // figure this passes on everything, because the caption is a
                    // `Code` with the value inside it and every specimen has one.
                    let source = stage(&component, prop_name, prop, value, &caption);
                    let pointed = source
                        .contains(&format!("(\"aria-labelledby\", Value::Text({caption:?}))"));
                    let called = source.contains(&format!(
                        "(\"label\", Value::Text({:?}))",
                        component.meta.name
                    ));
                    // `Link` is the third: its name is the words it shows, which
                    // is why a control with visible text needs no label of its
                    // own. The caption used to be written over that name, so
                    // every link in the grid announced itself as the value.
                    let spoken = source.contains("Node::Text(");
                    assert!(
                        pointed || called || spoken,
                        "{}.{prop_name} at {value} draws a control with no accessible name",
                        component.meta.name
                    );
                }
            }
        }
    }

    /// A table part outside a table is a start tag the parser throws away.
    #[test]
    fn every_table_part_is_wrapped_in_a_table() {
        for element in [
            "caption", "colgroup", "thead", "tbody", "tfoot", "tr", "td", "th",
        ] {
            let wrapped = in_table(element, "PART");
            assert!(
                wrapped.starts_with("Node::Element { tag: \"table\""),
                "{element} would be thrown away by the parser: {wrapped}"
            );
        }
        // A cell needs a row as well: "in table" drops a `<td>` just as "in
        // body" does, and only "in row" accepts one.
        for cell in ["td", "th"] {
            assert!(
                in_table(cell, "PART").contains("tag: \"tr\""),
                "a {cell} was given a table with no row in it"
            );
        }
        // And a `Table` arrives inside its scroller, because `height` caps the
        // scroller and would otherwise document four identical boxes.
        assert!(in_table("table", "PART").contains("nd-table-scroll"));
    }

    /// And nothing else is wrapped, so this costs every other component nothing.
    #[test]
    fn nothing_but_a_table_part_is_wrapped() {
        for element in ["div", "span", "button", "p", "li", "input"] {
            assert_eq!(in_table(element, "PART"), "PART");
        }
    }

    /// A specimen that is not a drawing names one icon.
    ///
    /// A name that is not an icon reaches `Icon`, whose own `icon` prop is
    /// required, and the site panics as it builds. This says which contract to
    /// open when it does.
    #[test]
    fn a_specimen_that_is_not_a_drawing_names_an_icon() {
        for component in shipped() {
            let Some(specimen) = component.meta.specimen.as_deref() else {
                continue;
            };
            if specimen.is_empty() || specimen.starts_with('<') {
                continue;
            }
            assert!(
                !specimen.contains(' ') && !specimen.contains('<'),
                "{}'s specimen is neither markup nor an icon name: {specimen}",
                component.meta.name
            );
        }
    }
}
