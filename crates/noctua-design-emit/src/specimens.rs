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
//! It nearly always is. Thirty-seven components used to draw a `specimen` the
//! contract carried — a **drawing** rather than a use of the component — on the
//! grounds that a live `<input>` in a grid is a control a reader can type into
//! by mistake. That trade bought nothing and cost the grids their subject:
//! `Meter` and `Progress` drew a `<span>` carrying the real element's class,
//! which no `::-webkit-meter-*` rule paints, so six sizes were six grey stubs;
//! `Badge` placed a badge against nothing and drew all four placements in one
//! spot; and `List { kind: description }` put `<li>` inside a `<dl>`, which is
//! markup no parser accepts.
//!
//! So a drawing is the FALLBACK now, and [`itself`] is the reach: what a
//! component needs beside the prop under test in order to draw itself. Two
//! components still use the fallback and that function says why.
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
//!
//! # Rules a new specimen has to keep
//!
//! Each of these has failed here, and none of them reports anything.
//!
//! - **A picture carries every other enum prop at its default**, because one
//!   attribute alone matches no compound rule and compound is how every tone is
//!   painted. **Never an `aria-*` one**: no rule in this system selects on ARIA,
//!   and a picture inventing `aria-sort="none"` announces itself as a column
//!   somebody can sort by. A picture of a list claims no role either, which is
//!   why the whole-page sweeps skip [`STAGE`].
//! - **A specimen's `name` is the CELL's, not the component's.** Every other
//!   required text prop takes the component's own name — the same string in
//!   every cell, which is what lets `variants.spec.js` measure two values
//!   against each other — but `name` is the hook every control derives its ids
//!   from, and two radio groups sharing a name are one group.
//! - **A specimen still has a name**, given by the component through the prop
//!   its contract requires. `every_specimen_of_a_named_control_has_a_name` is
//!   the gate.
//! - **The marker saying a cell shows names goes on the stage**, not on the
//!   component call: where a component wraps itself in a `Field`, the class
//!   lands on the control and the `<label>` is its SIBLING.
//! - **A grid repeats its own words on purpose**, so two values painted alike
//!   measure alike. That is the opposite of what the example sweeps demand, so
//!   those read `:not(.nd-variant-stage *)`.
//! - **A cell carries `.nd-table-value`** where a table is the subject, or
//!   every rule acting on the value rather than the cell does nothing here.
//! - **A group's own grid needs the reach in each MEMBER**, or three
//!   arrangements draw one picture.
//! - **The stage disables three things** — `container-type` (which IS
//!   `contain: inline-size`, so the intrinsic width is zero), the scroller's
//!   overflow, and `inline-size: 100%`. Any one left in place collapses the
//!   specimen to nothing.
//!
//! `variants.spec.js` asks the four questions a grid exists to answer: two
//! values of one prop drawing the same picture (its fingerprint reads
//! `::-webkit-*` parts, because `<meter>`, `<progress>` and a range `<input>`
//! are painted there and nowhere else); every cell draws something; every cell
//! carries the value its caption names; and no cell overhangs a frame that
//! cannot scroll at 320 pixels.

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
            .filter(|(prop_name, prop)| pictured(component, prop_name, prop))
            .map(|(prop_name, _)| prop_name.as_str())
            .collect();
        if !pictured.is_empty() {
            named.insert(format!("{}: {}", component.meta.name, pictured.join(", ")));
        }
    }

    let mut out = String::from(
        "//! # The grids that draw a picture rather than the component\n\
         //!\n\
         //! Every other grid on the site draws the real component, so this list is\n\
         //! the exception and the emitter that wrote it says why each entry is on\n\
         //! it. A picture is the contract's element carrying the contract's\n\
         //! attributes with the contract's `specimen` inside, and rendering one\n\
         //! shows the markup matches in both languages and nothing more about the\n\
         //! component.\n\
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

/// Whether this prop's grid draws a picture instead of the component.
///
/// One question, asked by the note the generated file carries and by the stage
/// that renders it. Two copies of it is how the note came to disagree with the
/// grid it describes.
fn pictured(component: &Component, prop_name: &str, prop: &Prop) -> bool {
    prop.values.first().is_some_and(|value| {
        itself(component, prop_name, value, "").is_none()
            && content(component, prop, value).is_drawing()
    })
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

// --- A component drawing itself ---------------------------------------------

/// What a component needs in order to draw ITSELF in the grid.
///
/// # Why a picture is the fallback and this is the reach
///
/// A grid of pictures documents a string builder's idea of the component, and
/// this repository keeps finding what that costs: `Switch`'s grid carried no
/// `role` and no `aria-checked` until it drew the real control, `Divider`'s
/// emitted an end tag for a void element, and `Meter`'s three sizes were three
/// grey stubs because a `<span class="nd-meter-bar">` is painted by none of the
/// `::-webkit-meter-*` rules that draw a real one.
///
/// # Why the table is here and not in the contract
///
/// What a component needs is more than one value. `Field` needs a control to
/// label, `Badge` needs something to sit on, `Tabs` needs tabs, `Table` needs
/// rows. That is a **tree**, and a tree written in TOML would be a second,
/// worse spelling of the one `docs-site/src/tree.rs` already declares. So it
/// lives beside the emitter that writes that tree, exactly as `controls.rs`
/// holds the body a contract cannot describe.
struct Drawn {
    /// Props beside the one under test, in the contract's own spelling.
    ///
    /// They beat what [`call`] would fill in, and nothing else here does.
    props: Vec<(&'static str, String)>,
    /// What goes inside.
    children: Vec<String>,
    /// What goes BESIDE it, inside the stage.
    ///
    /// One component needs it and it is `Tabs`: a strip is a table of contents
    /// with no script, and every tab is a link down to a panel that is its
    /// SIBLING rather than its child. A strip with no panels is three links
    /// pointing at nothing, which `tabs.spec.js` fails the build over — and
    /// rightly, because that is the control-that-lies shape again.
    after: Vec<String>,
    /// Whether the specimen shows a word that never went through the catalogue.
    ///
    /// `Aa`, `NA`, `#4213` and `51s` are names, and the pseudo-locale sweep is
    /// told so once, on the call, rather than on each of them. A specimen whose
    /// every word IS a catalogue sentence says [`Drawn::sentences`] instead and
    /// stays under the sweep.
    identifier: bool,
}

impl Drawn {
    /// A specimen whose content is names.
    fn names() -> Self {
        Self {
            props: Vec::new(),
            children: Vec::new(),
            after: Vec::new(),
            identifier: true,
        }
    }

    /// A specimen whose content is entirely sentences from the catalogue.
    fn sentences() -> Self {
        Self {
            identifier: false,
            ..Self::names()
        }
    }

    /// One prop, beside the one under test.
    fn prop(mut self, name: &'static str, value: String) -> Self {
        self.props.push((name, value));
        self
    }

    /// What goes inside.
    fn inside(mut self, children: Vec<String>) -> Self {
        self.children = children;
        self
    }

    /// What goes beside it, inside the stage.
    fn beside(mut self, after: Vec<String>) -> Self {
        self.after = after;
        self
    }
}

/// The words a specimen shows where its content is a name.
///
/// Three of them, because a set has to read as a set: one member cannot show
/// what `overlap`, `spacing` or `rules` do.
///
/// AND THE MIDDLE ONE IS LONGER. `Tabs { fill }` divides a strip between its
/// tabs, so three words of one length draw `hug`, `equal` and `stretch` as one
/// picture — the widths only differ when the content does.
const WORDS: [&str; 3] = ["Aa", "Bbbb", "Cc"];

/// How a component draws itself, or nothing where it cannot.
///
/// # The two that cannot, and why
///
/// * **`LineageFigure`** is a piece the layout engine places inside a scene, at
///   the cell that scene's graph gave it. Outside a scene it has nowhere to
///   stand and no coordinates to stand at, so `shape` and `tone` can only be
///   shown on the drawing of a piece. Its `scene` and `at` props say so: both
///   are required, and both name something a specimen has not got.
/// * **`Lineage`** is the scene, and four of its nine grids are about the
///   WINDOW rather than the drawing. `height` caps the field, `view` sets the
///   air around it, `tilt` turns the camera and `thickness` is read off an edge
///   at an angle — none of which a still with no window can show. Its specimen
///   is therefore drawn flat, at a fixed size, in a box the sample's own rules
///   give it, and `lineage.spec.js` fingerprints that sample rather than the
///   stage around it because that is the stricter question. A real scene per
///   cell would also be twenty-five perspective contexts on one page.
///
/// Everything else in the catalogue draws itself, and a component absent from
/// this table is one whose contract already says enough — `Heading`, `Button`
/// and the rest reach [`call`] through [`content`] with no help.
#[allow(
    clippy::too_many_lines,
    reason = "one arm per component, and the component set is the point"
)]
fn itself(component: &Component, prop_name: &str, value: &str, caption: &str) -> Option<Drawn> {
    let drawn = match component.meta.name.as_str() {
        "Avatar" => Drawn::names().prop("initials", data("NA")),
        // AND ITS MEMBERS ARE LIST ITEMS. The group is a `<ul>` and its overlap
        // is `li + li { margin-inline-start }`, so three bare avatars matched
        // nothing and `tight`, `loose` and `none` drew one picture — which is
        // what `variants.spec.js` failed on. `AvatarGroup` renders its children
        // straight into the list, so the `<li>` is the caller's, here as in
        // every example.
        "AvatarGroup" => Drawn::names()
            .prop("more", data("+3"))
            .inside(vec![member("AL"), member("BS")]),
        // SOMETHING TO SIT ON. A badge places itself against what it marks, so
        // a badge with nothing inside drew all four `placement` values in one
        // spot and all four looked the same.
        // AND ITS NAME IS A SENTENCE ABOUT THE COUNT. `label` is what a reader
        // who cannot see the corner is owed, and `tags.spec.js` reads it
        // against `count`: a badge drawing `3` and announcing `Badge` fails
        // that relation, and so does one announcing the bare digit — a number
        // read back is not a sentence. The catalogue already carries this one,
        // at this count, which is why the count is three.
        "Badge" => Drawn::names()
            .prop("count", data("3"))
            .prop("label", message("ExampleBadgeUnread"))
            .prop("tone", text("danger"))
            .inside(vec![avatar("NA")]),
        // `justify` says where the content sits when the button is wider than
        // it needs, so it needs two things to push apart.
        "Button" if prop_name == "justify" => Drawn::names().prop("icon", text("check")),
        // AND THE GROUP'S OWN `justify` NEEDS IT TOO, for the reason the line
        // above gives: the group states where the content of every button
        // sits, and a button holding one word has nowhere to sit differently.
        "ButtonGroup" if prop_name == "justify" => {
            Drawn::names().prop("labelledby", text(caption)).inside(
                WORDS
                    .iter()
                    .take(2)
                    .map(|word| marked_button(word))
                    .collect(),
            )
        }
        "ButtonGroup" => Drawn::names()
            .prop("labelledby", text(caption))
            .inside(WORDS.iter().take(2).map(|word| word_button(word)).collect()),
        "Checkbox" | "Radio" => Drawn::names()
            .prop("value", text("on"))
            .prop("label", data(WORDS[0])),
        // A FIELD LABELS ONE CONTROL OR SEVERAL, which is `kind`'s own
        // sentence and the only thing a still picture of it can show: both
        // arms draw the same `.nd-field`, and what differs is the `<label>`
        // over one answer against the `<legend>` over a set of them.
        "Field" => Drawn::names()
            .prop("control", text(&choice_id(caption, 0, WORDS[0])))
            .inside(
                WORDS
                    .iter()
                    .take(if value == "group" { WORDS.len() } else { 1 })
                    .enumerate()
                    .map(|(at, word)| field_choice(caption, at, word))
                    .collect(),
            ),
        "Image" => Drawn::names().prop("src", asset("assets/portraits/ada.svg")),
        // A BOX WITH SOMETHING IN IT. An empty control is the same picture at
        // every size, and the reader cannot tell a text box from a search box
        // from a picker by its outline alone.
        "Input" | "Search" | "Textarea" => Drawn::names().prop("value", data(WORDS[0])),
        // NO `value`, AND THAT IS DELIBERATE. A colour is the one thing this
        // repository may not write down (`AGENTS.md`, invariant 1), and the
        // chip a picker draws with nothing chosen is the picker's own answer
        // rather than a shade chosen here.
        //
        // THE TWO SLIDERS ARE NAMED, THOUGH. A picker renders its panel at
        // rest, hidden behind the trigger, and the saturation field and the hue
        // track inside it are real range inputs — nameless without these, which
        // `audit.spec.js`'s live-control sweep found the moment the grid drew
        // the real component instead of a chip.
        "ColorPicker" => Drawn::names()
            .prop("area_label", message("ExampleColorPickerArea"))
            .prop("hue_label", message("ExampleColorPickerHue")),
        // AND A LIST NEEDS OPTIONS. A picker's chosen values ARE its selected
        // options, so a picker with none draws an empty box that says nothing
        // about the tags it exists to hold.
        "Select" | "TagPicker" => Drawn::names().inside(
            WORDS
                .iter()
                .enumerate()
                .map(|(at, word)| {
                    component_node(
                        "SelectOption",
                        &[
                            ("value", text(word)),
                            ("label", data(word)),
                            ("selected", flag(at == 0)),
                        ],
                        &[],
                    )
                })
                .collect(),
        ),
        "Label" => Drawn::names().prop("text", data(WORDS[0])),
        "List" => Drawn::names().inside(
            WORDS
                .iter()
                .map(|word| {
                    let mut props = vec![];
                    // A DESCRIPTION LIST HAS TERMS. The picture put `<li>`
                    // inside a `<dl>`, which is markup no parser accepts and a
                    // picture of a list that is not one.
                    if value == "description" {
                        props.push(("term", data(word)));
                    }
                    component_node("ListItem", &props, &[node_text(&data(word))])
                })
                .collect(),
        ),
        // THE READING IS THE COMPONENT. `<meter>` and `<progress>` are painted
        // entirely into engine pseudo-elements, so a bar with no value is a
        // bar with nothing in it — which is what all six of these drew.
        "Meter" => Drawn::names()
            .prop("value", text("41.2"))
            .prop("max", text("50"))
            .prop("value_label", message("SpecimenBarValue")),
        "Progress" => Drawn::names()
            .prop("value", text("7"))
            .prop("max", text("12"))
            .prop("value_label", message("SpecimenBarValue")),
        "NumberInput" => Drawn::names().prop("value", data("42")),
        "OtpField" => Drawn::names()
            .prop("length", text("3"))
            .prop("value", data("421")),
        "Pagination" => Drawn::names()
            .prop("page", text("2"))
            .prop("pages", text("3"))
            .prop("href", text("#")),
        // AT THE SMALL SIZE, AND THAT IS THE WHOLE OF THIS ONE. The measure is
        // `66ch` of the text's own font, and this site's reading column is a
        // measure wide by design — so at body size the cap and the cell are
        // the same width and `default` and `none` drew one picture. Smaller
        // type puts the cap inside the cell, where a reader can see it land.
        "Prose" => Drawn::sentences().inside(vec![
            component_node(
                "Heading",
                &[("level", text("3")), ("size", text("h4"))],
                &[node_text(&format!("Value::Message({PANGRAM})"))],
            ),
            component_node(
                "Paragraph",
                &[("size", text("small"))],
                &[node_text("Value::Message(Key::ExampleListLong)")],
            ),
        ]),
        "Rating" => Drawn::names()
            .prop("value", text("3"))
            .prop("readonly", flag(true)),
        "Segmented" => Drawn::names().prop("labelledby", text(caption)).inside(
            WORDS
                .iter()
                .enumerate()
                .map(|(at, word)| {
                    component_node(
                        "SegmentedItem",
                        &[
                            ("value", text(word)),
                            ("label", data(word)),
                            ("selected", flag(at == 0)),
                        ],
                        &[],
                    )
                })
                .collect(),
        ),
        "Table" => Drawn::names()
            .prop("hide_caption", flag(true))
            .inside(vec![component_node("TableSection", &[], &rows())]),
        // ONE COMPONENT, TWO PROPS, TWO VALUES. `wrap` needs one long enough to
        // break and `align` needs one whose digits can line up — a build number
        // shows neither, and a currency in a `nowrap` cell shows nothing about
        // wrapping.
        //
        // AND A HEADING SAYS A SHORTER VERSION OF THE SAME THING, which is the
        // contract's own choice and load-bearing: `table.spec.js` fingerprints
        // every stage on the page by the words in it, so a heading row reading
        // exactly what the cell row reads is two examples drawing one thing.
        "TableCell" | "TableHeaderCell" => {
            let heading = component.meta.name == "TableHeaderCell";
            Drawn::names().inside(vec![node_text(&data(match (prop_name, heading) {
                ("wrap", false) => LONG_VALUE,
                ("wrap", true) => LONG_HEADING,
                ("align", false) => AMOUNT,
                ("align", true) => ROUND_AMOUNT,
                _ => BUILD,
            }))])
        }
        "TableRow" => Drawn::names().inside(cells()),
        // A `<colgroup>` paints nothing of its own, so the grid gives it a
        // background and a row to be measured against. Both are the stage's,
        // and the component under test is the real declaration.
        "TableColumn" => Drawn::names(),
        // AND THE PANELS COME WITH THE STRIP. With no script every panel is on
        // the page and each tab is a link down to the one it names, so a strip
        // on its own is three links pointing at nothing.
        "Tabs" => Drawn::names()
            .inside(
                WORDS
                    .iter()
                    .enumerate()
                    .map(|(at, word)| {
                        component_node(
                            "Tab",
                            &[
                                ("name", text(caption)),
                                ("value", text(word)),
                                ("label", data(word)),
                                ("selected", flag(at == 0)),
                            ],
                            &[],
                        )
                    })
                    .collect(),
            )
            .beside(
                WORDS
                    .iter()
                    .map(|word| {
                        component_node(
                            "TabPanel",
                            &[("name", text(caption)), ("value", text(word))],
                            &[node_text(&data(word))],
                        )
                    })
                    .collect(),
            ),
        "Tag" => Drawn::names().prop("label", data("Tag")),
        // THE CONTROLS SIT IN THE BAR, NOT IN A GROUP INSIDE IT. `orientation`
        // is `flex-direction` on the toolbar itself, and a toolbar holding one
        // `ToolbarGroup` has one child — so a column of one and a row of one
        // are the same picture, and `horizontal` and `vertical` drew it twice.
        "Toolbar" => Drawn::names()
            .prop("labelledby", text(caption))
            .inside(WORDS.iter().take(2).map(|word| bar_button(word)).collect()),
        "Tree" => Drawn::names().inside(vec![component_node(
            "TreeItem",
            &[
                ("label", data(WORDS[0])),
                ("value", text(WORDS[0])),
                ("branch", flag(true)),
                ("expanded", flag(true)),
            ],
            &[component_node(
                "TreeItem",
                &[("label", data(WORDS[1])), ("value", text(WORDS[1]))],
                &[],
            )],
        )]),
        "Truncate" => {
            Drawn::sentences().inside(vec![node_text("Value::Message(Key::ExampleListLong)")])
        }
        _ => return None,
    };
    Some(drawn)
}

/// One member of an `AvatarGroup`: an avatar, in the list item it belongs in.
fn member(initials: &str) -> String {
    element_node("li", &[], &[avatar(initials)])
}

/// One face: a member of a group, or the thing a `Badge` marks.
fn avatar(initials: &str) -> String {
    component_node(
        "Avatar",
        &[("label", data(initials)), ("initials", data(initials))],
        &[],
    )
}

/// One member of a `ButtonGroup`: the resting control the group joins.
fn word_button(label: &str) -> String {
    component_node(
        "Button",
        &[
            ("variant", text("outline")),
            ("tone", text("neutral")),
            ("label", data(label)),
        ],
        &[],
    )
}

/// The same member with something beside the word.
///
/// `ButtonGroup.justify` places what is inside every button, and one word has
/// no inside to place: `center`, `start` and `between` all draw a label in the
/// middle of a box, which is one picture three times over. Two things and the
/// room between them is what the prop's own sentence assumes, exactly as
/// `Button.justify` assumes it.
fn marked_button(label: &str) -> String {
    component_node(
        "Button",
        &[
            ("variant", text("outline")),
            ("tone", text("neutral")),
            ("icon", text("check")),
            ("label", data(label)),
        ],
        &[],
    )
}

/// One control in a bar, which is the quiet form a toolbar is made of.
fn bar_button(label: &str) -> String {
    component_node(
        "Button",
        &[
            ("variant", text("transparent")),
            ("tone", text("neutral")),
            ("size", text("small")),
            ("label", data(label)),
        ],
        &[],
    )
}

/// One answer inside a `Field`.
///
/// A checkbox draws its own label beside its own box, so it is a control that
/// needs nothing from the field around it — which is what lets the field's own
/// question be the only thing the picture is of.
fn field_choice(caption: &str, at: usize, label: &str) -> String {
    component_node(
        "Checkbox",
        &[
            ("name", text(&format!("{caption}-{at}"))),
            ("value", text(label)),
            ("label", data(label)),
            ("size", text("small")),
        ],
        &[],
    )
}

/// The id the checkbox above will render, so the field's `<label for>` can
/// point at it.
///
/// A SECOND STATEMENT OF `Checkbox`'S OWN RULE, and it is the only one in this
/// file. `Field` renders no control, so it cannot derive the id of the thing it
/// labels — the contract says as much in `control`'s own documentation, and a
/// caller has to know it. What holds the two together is
/// `inputs.spec.js`'s *every label points at a control that exists*, which is
/// how the dangling `for` this specimen first shipped was found.
fn choice_id(caption: &str, at: usize, label: &str) -> String {
    format!("nd-{caption}-{at}-{label}")
}

/// A long value, for the prop whose subject is what happens when one does not
/// fit.
const LONG_VALUE: &str = "release/2026-03-candidate-rollback";

/// A number with separators in it, for the prop whose subject is where the
/// digits sit. `numeric` switches on tabular figures and puts the units under
/// each other, and a value with no fraction shows neither.
const AMOUNT: &str = "R$ 1.240,00";

/// The same two, as a heading says them: a column is named, not measured.
const LONG_HEADING: &str = "release/2026-03-candidate";
/// The rounded amount a column heading carries.
const ROUND_AMOUNT: &str = "R$ 1.240";

/// The two cells a table specimen is measured with.
fn cells() -> Vec<String> {
    vec![
        component_node(
            "TableHeaderCell",
            &[("scope", text("row"))],
            &[node_text(&data(BUILD))],
        ),
        component_node("TableCell", &[], &[node_text(&data("51s"))]),
    ]
}

/// Three rows of them, so `rules`, `density` and `zebra` have somewhere to act.
fn rows() -> Vec<String> {
    ["#4213", "#4211", "#4209"]
        .iter()
        .zip(["51s", "12s", "48s"])
        .map(|(build, took)| {
            component_node(
                "TableRow",
                &[],
                &[
                    component_node(
                        "TableHeaderCell",
                        &[("scope", text("row"))],
                        &[node_text(&data(build))],
                    ),
                    component_node("TableCell", &[], &[node_text(&data(took))]),
                ],
            )
        })
        .collect()
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
                    (
                        "class",
                        text(&stage_class(component, prop_name, prop, value)),
                    ),
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

/// The stage's class, which says whether the cell shows names or sentences.
///
/// # Why the marker is on the STAGE and not on the component
///
/// A WORD THAT NEVER WENT THROUGH THE CATALOGUE HAS TO SAY SO. The pseudo-locale
/// sweep reads every run of text on every page and fails on one carrying no
/// brackets, which is how thirteen pages of English documentation shipped in
/// Portuguese. `Aa`, `NA`, `#4213` and `51s` are names, and `.nd-identifier` is
/// how the markup says so.
///
/// It used to go on the component call. That works for a component that draws
/// its own content and fails for the six that wrap themselves in a `Field`: the
/// class lands on `.nd-select`, and the `<label>` the field drew is its SIBLING,
/// outside the marked subtree. `/select/` shipped the word `Select` unbracketed
/// and the sweep caught it. The stage is the one box that contains the whole
/// cell, whatever the component built inside it.
fn stage_class(component: &Component, prop_name: &str, prop: &Prop, value: &str) -> String {
    // A component drawing itself says which it shows. Everything else shows a
    // name exactly when a required text prop was filled with the component's
    // own — `name` excepted, because a script hook is never drawn.
    let names = itself(component, prop_name, value, "").map_or_else(
        || {
            matches!(content(component, prop, value), Content::Name)
                || component.attributes().any(|(name, declared)| {
                    declared.required && declared.kind == Kind::Text && name != "name"
                })
        },
        |drawn| drawn.identifier,
    );
    if names {
        format!("{STAGE} {IDENTIFIER}")
    } else {
        STAGE.to_owned()
    }
}

/// What goes in the stage: the component, or the picture the contract carries.
fn stage(
    component: &Component,
    prop_name: &str,
    prop: &Prop,
    value: &str,
    caption: &str,
) -> String {
    // THE COMPONENT FIRST, ALWAYS. A contract's `specimen` is a drawing, and a
    // drawing is what a grid falls back to — never what it reaches for.
    if let Some(drawn) = itself(component, prop_name, value, caption) {
        let element = element_for(component, prop, value);
        let made = in_table(
            element,
            &call(
                component,
                prop_name,
                prop,
                value,
                None,
                Some(&drawn),
                caption,
            ),
        );
        if drawn.after.is_empty() {
            return made;
        }
        let mut inside = vec![made];
        inside.extend(drawn.after.iter().cloned());
        return element_node("div", &[], &inside);
    }
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
        Content::Icon(icon) => call(
            component,
            prop_name,
            prop,
            value,
            Some(&icon),
            None,
            caption,
        ),
        Content::Pangram | Content::Name | Content::Empty => {
            call(component, prop_name, prop, value, None, None, caption)
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
    drawn: Option<&Drawn>,
    caption: &str,
) -> String {
    let mut props: Vec<(&str, String)> = Vec::new();
    for (name, declared) in component.attributes() {
        if name == prop_name {
            props.push((name, text(value)));
        } else if let Some((_, set)) = drawn
            .into_iter()
            .flat_map(|drawn| &drawn.props)
            .find(|(prop, _)| prop == name)
        {
            props.push((name, set.clone()));
        } else if let Some(icon) = icon.filter(|_| name == "icon") {
            props.push((name, text(icon)));
        } else if declared.required && declared.kind == Kind::Text {
            // A SCRIPT HOOK IS UNIQUE TO THE CELL, AND EVERY OTHER REQUIRED
            // PROP TAKES THE COMPONENT'S OWN NAME.
            //
            // `name` is the one prop this system uses for a hook a script or a
            // form finds (`AGENTS.md`, *A script hook is a prop*), and every
            // control derives its own ids from it. One `name` across a whole
            // page of grids is one `id` repeated a dozen times, and for a
            // radio group it is worse: two groups sharing a name are one
            // group, so choosing in the second unchooses the first.
            //
            // Something has to fill the rest, and the alternative is a sentence
            // invented here, in English, for a page built in three languages.
            // The name answers `needs_name` honestly: a control whose label is
            // required is named by that prop, which is the component's own
            // answer rather than an attribute bolted on beside it.
            //
            // AND IT IS THE SAME STRING FOR EVERY VALUE, which is the part that
            // matters. A row of pictures differing in their words as well as in
            // the prop under test is a row where two values painted alike still
            // measure apart, and `variants.spec.js` reads exactly that
            // measurement. A `name` is never drawn, so it is exempt.
            if name == "name" {
                props.push((name, text(caption)));
            } else {
                props.push((name, text(&component.meta.name)));
            }
        }
    }

    // WHAT A COMPONENT DRAWING ITSELF SHOWS IS ITS OWN, WHOLE. The contract's
    // `specimen` is not consulted here at all — a table that supplies content
    // and a contract that supplies a picture of it are two answers to one
    // question, and reading both would let them disagree.
    let children = match drawn {
        Some(drawn) => drawn.children.clone(),
        None => match content(component, prop, value) {
            Content::Pangram => vec![node_text(&format!("Value::Message({PANGRAM})"))],
            Content::Name => vec![node_text(&text(&component.meta.name))],
            _ => vec![],
        },
    };
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
///
/// The real components, like everything else a grid draws. It was three
/// elements carrying the classes those components write, which is the same
/// drawing-instead-of-the-thing this file exists to end — one level down, where
/// it is easier to miss.
fn row() -> String {
    component_node(
        "TableSection",
        &[("class", text(IDENTIFIER))],
        &[component_node("TableRow", &[], &cells())],
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

/// `Value::Message(Key::…)`: a sentence, in the reader's language.
fn message(key: &str) -> String {
    format!("Value::Message(Key::{key})")
}

/// `Value::Data(…)`: content that is a name rather than a sentence.
fn data(value: &str) -> String {
    format!("Value::Data({value:?})")
}

/// `Value::Bool(…)`.
///
/// A bool prop written any other way is a tree the gates refuse, and rightly:
/// `Value::Text("true")` would work by accident until the reader changed.
fn flag(on: bool) -> String {
    format!("Value::Bool({on})")
}

/// `Value::Asset(…)`: a file under `assets/`, from the page's own depth.
fn asset(path: &str) -> String {
    format!("Value::Asset({path:?})")
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

    /// The four spellings a contract uses for a control's accessible name.
    ///
    /// Not a convention this file invents: it is read back out of the specs by
    /// the test below, which fails if a `needs_name` component has none of them.
    /// So a fifth spelling is loud rather than silently unnamed.
    const NAMING: [&str; 4] = ["label", "legend", "caption", "alt"];

    /// A control with no name is the defect this whole layer sits under.
    ///
    /// `IconButton` shipped six of them. There are three ways a specimen can
    /// have one, and every named control has to take one of them: the words
    /// inside it, the prop the contract carries for the purpose, or the caption
    /// beside it. This reads the emitted source rather than trusting the branch
    /// that wrote it.
    #[test]
    fn every_specimen_of_a_named_control_has_a_name() {
        let mut checked = 0usize;
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
                        .contains(&format!("(\"aria-labelledby\", Value::Text({caption:?}))"))
                        || source.contains(&format!("(\"labelledby\", Value::Text({caption:?}))"));
                    // THE VALUE IS NOT READ, ONLY THAT ONE WAS PASSED. A
                    // specimen that draws itself fills these with the words in
                    // its own picture — `Aa` on a `Checkbox`, `Tag` on a `Tag`
                    // — and a name is a name whichever of them it is.
                    let called = NAMING
                        .iter()
                        .any(|naming| source.contains(&format!("(\"{naming}\", Value::")));
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
                    checked += 1;
                }
            }
        }
        // A walk that found nothing would pass for ever, which is the shape
        // this suite keeps catching in itself.
        assert!(checked > 40, "only {checked} named controls were read");
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
