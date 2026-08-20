//! Every example, described as data.
//!
//! # Why these are here and not beside the closures
//!
//! `examples.rs` holds what an example *renders*, as a Dioxus closure. That is
//! the half only Rust can run, and it is why the React package this system
//! ships was rendered nowhere on the site that documents it — the audit found
//! six of its modules broken with the whole suite green.
//!
//! These are the same examples described rather than drawn, so the React half
//! can render them too. See `crate::tree` for the shape and the argument.
//!
//! # One file per component, on purpose
//!
//! 499 examples had to be transcribed, and a transcription is exactly the kind
//! of work that goes wrong quietly. Splitting them by component means each one
//! could be done and checked on its own, against
//! `every_tree_draws_what_its_closure_drew` — which renders both halves and
//! compares the markup byte for byte, in every locale.

use dioxus::prelude::*;

use crate::shell::Context;
use crate::tree::{Node, Scope};

mod abbr;
mod avatar;
mod avatar_group;
mod badge;
mod blockquote;
mod button;
mod button_group;
mod carousel;
mod carousel_slide;
mod checkbox;
mod code;
mod code_block;
mod color_picker;
mod date_input;
mod del;
mod dialog;
mod disclosure;
mod divider;
mod em;
mod error_summary;
mod error_summary_item;
mod field;
mod fieldset;
mod file_input;
mod heading;
mod icon;
mod image;
mod info;
mod input;
mod ins;
mod kbd;
mod label;
mod lineage;
mod lineage_figure;
mod link;
mod list;
mod list_item;
mod mark;
mod menu;
mod menu_item;
mod message_bar;
mod meter;
mod number_input;
mod otp_field;
mod overview_a;
mod overview_b;
mod overview_c;
mod overview_d;
mod pagination;
mod paragraph;
mod popover;
mod progress;
mod prose;
mod radio;
mod rating;
mod scope;
mod search;
mod search_group;
mod search_result;
mod segmented;
mod segmented_item;
mod select;
mod select_group;
mod select_option;
mod slider;
mod strong;
mod swatch;
mod switch;
mod tab;
mod tab_panel;
mod table;
mod table_cell;
mod table_column;
mod table_header_cell;
mod table_row;
mod table_section;
mod tabs;
mod tag;
mod tag_picker;
mod text;
mod textarea;
mod time;
mod time_input;
mod toast;
mod toolbar;
mod toolbar_group;
mod tooltip;
mod tree;
mod tree_item;
mod truncate;
mod visually_hidden;

/// One example, as its caption and its tree.
///
/// The caption is repeated from `examples.rs` rather than referenced, and that
/// is the point: it pins each tree to the example it belongs to. Positional
/// pairing alone would let a tree inserted in the wrong place draw somebody
/// else's picture and pass every test that only counts them.
pub(crate) type Tree = (&'static str, &'static [Node]);

/// One page's hero, as its slug, its caption and its tree.
pub(crate) type Hero = (&'static str, &'static str, &'static [Node]);

/// Every tree for a component, in the order `examples::for_component` gives
/// them.
///
/// An empty slice means none has been written yet, which is a state the
/// migration passes through and `every_example_has_a_tree` refuses at the end.
#[must_use]
pub(crate) fn for_component(name: &str) -> &'static [Tree] {
    match name {
        "Abbr" => abbr::EXAMPLES,
        "Avatar" => avatar::EXAMPLES,
        "AvatarGroup" => avatar_group::EXAMPLES,
        "Badge" => badge::EXAMPLES,
        "Blockquote" => blockquote::EXAMPLES,
        "Button" => button::EXAMPLES,
        "ButtonGroup" => button_group::EXAMPLES,
        "Carousel" => carousel::EXAMPLES,
        "CarouselSlide" => carousel_slide::EXAMPLES,
        "Checkbox" => checkbox::EXAMPLES,
        "Code" => code::EXAMPLES,
        "CodeBlock" => code_block::EXAMPLES,
        "ColorPicker" => color_picker::EXAMPLES,
        "DateInput" => date_input::EXAMPLES,
        "Del" => del::EXAMPLES,
        "Dialog" => dialog::EXAMPLES,
        "Disclosure" => disclosure::EXAMPLES,
        "Divider" => divider::EXAMPLES,
        "Em" => em::EXAMPLES,
        "ErrorSummary" => error_summary::EXAMPLES,
        "ErrorSummaryItem" => error_summary_item::EXAMPLES,
        "Field" => field::EXAMPLES,
        "Fieldset" => fieldset::EXAMPLES,
        "FileInput" => file_input::EXAMPLES,
        "Heading" => heading::EXAMPLES,
        "Icon" => icon::EXAMPLES,
        "Image" => image::EXAMPLES,
        "Info" => info::EXAMPLES,
        "Input" => input::EXAMPLES,
        "Ins" => ins::EXAMPLES,
        "Kbd" => kbd::EXAMPLES,
        "Label" => label::EXAMPLES,
        "Lineage" => lineage::EXAMPLES,
        "LineageFigure" => lineage_figure::EXAMPLES,
        "Link" => link::EXAMPLES,
        "List" => list::EXAMPLES,
        "ListItem" => list_item::EXAMPLES,
        "Mark" => mark::EXAMPLES,
        "Menu" => menu::EXAMPLES,
        "MenuItem" => menu_item::EXAMPLES,
        "MessageBar" => message_bar::EXAMPLES,
        "Meter" => meter::EXAMPLES,
        "NumberInput" => number_input::EXAMPLES,
        "OtpField" => otp_field::EXAMPLES,
        "Pagination" => pagination::EXAMPLES,
        "Paragraph" => paragraph::EXAMPLES,
        "Popover" => popover::EXAMPLES,
        "Progress" => progress::EXAMPLES,
        "Prose" => prose::EXAMPLES,
        "Radio" => radio::EXAMPLES,
        "Rating" => rating::EXAMPLES,
        "Search" => search::EXAMPLES,
        "SearchGroup" => search_group::EXAMPLES,
        "SearchResult" => search_result::EXAMPLES,
        "Segmented" => segmented::EXAMPLES,
        "SegmentedItem" => segmented_item::EXAMPLES,
        "Select" => select::EXAMPLES,
        "SelectGroup" => select_group::EXAMPLES,
        "SelectOption" => select_option::EXAMPLES,
        "Slider" => slider::EXAMPLES,
        "Strong" => strong::EXAMPLES,
        "Swatch" => swatch::EXAMPLES,
        "Switch" => switch::EXAMPLES,
        "Tab" => tab::EXAMPLES,
        "TabPanel" => tab_panel::EXAMPLES,
        "Table" => table::EXAMPLES,
        "TableCell" => table_cell::EXAMPLES,
        "TableColumn" => table_column::EXAMPLES,
        "TableHeaderCell" => table_header_cell::EXAMPLES,
        "TableRow" => table_row::EXAMPLES,
        "TableSection" => table_section::EXAMPLES,
        "Tabs" => tabs::EXAMPLES,
        "Tag" => tag::EXAMPLES,
        "TagPicker" => tag_picker::EXAMPLES,
        "Text" => text::EXAMPLES,
        "Textarea" => textarea::EXAMPLES,
        "Time" => time::EXAMPLES,
        "TimeInput" => time_input::EXAMPLES,
        "Toast" => toast::EXAMPLES,
        "Toolbar" => toolbar::EXAMPLES,
        "ToolbarGroup" => toolbar_group::EXAMPLES,
        "Tooltip" => tooltip::EXAMPLES,
        "Tree" => tree::EXAMPLES,
        "TreeItem" => tree_item::EXAMPLES,
        "Truncate" => truncate::EXAMPLES,
        "VisuallyHidden" => visually_hidden::EXAMPLES,
        _ => &[],
    }
}

/// Every group of heroes.
const HEROES: [&[Hero]; 4] = [
    overview_a::HEROES,
    overview_b::HEROES,
    overview_c::HEROES,
    overview_d::HEROES,
];

/// The tree for a page's hero, if one has been written.
#[must_use]
pub(crate) fn for_page(slug: &str) -> Option<Hero> {
    HEROES
        .iter()
        .flat_map(|group| group.iter())
        .find(|(page, _, _)| *page == slug)
        .copied()
}

/// The `ThemeScope` demonstration on `/theme-scope/`.
///
/// Not an example out of `examples.rs`, so it is reached by name rather than
/// by index. See `crate::trees::scope`.
pub(crate) const SCOPE: &[Node] = scope::SAMPLE;

/// The tree for one component example, or an empty slice if none is written.
#[must_use]
pub(crate) fn nodes(component: &str, index: usize) -> &'static [Node] {
    for_component(component)
        .get(index)
        .map_or(&[], |(_, nodes)| *nodes)
}

/// Draws an example.
pub(crate) fn draw(nodes: &'static [Node], context: Context) -> Element {
    crate::tree::render(nodes, Scope::new(context))
}

#[cfg(test)]
mod structure {
    use super::*;
    use crate::generated::catalogue::COMPONENTS;
    use crate::tree::{Node, TAGS, Value};

    /// Every node of every tree on the site, flattened.
    pub(super) fn everything() -> Vec<&'static Node> {
        let mut all = Vec::new();
        for component in COMPONENTS {
            for (_, nodes) in for_component(component.name) {
                walk(nodes, &mut all);
            }
        }
        for group in HEROES {
            for (_, _, nodes) in group {
                walk(nodes, &mut all);
            }
        }
        all
    }

    fn walk(nodes: &'static [Node], into: &mut Vec<&'static Node>) {
        for node in nodes {
            into.push(node);
            match node {
                Node::Component { children, .. } | Node::Element { children, .. } => {
                    walk(children, into);
                }
                Node::Fragment(nodes) | Node::Each { body: nodes, .. } => walk(nodes, into),
                Node::Text(_) | Node::Raw(_) => {}
            }
        }
    }

    /// A component a tree names must be one the catalogue carries.
    ///
    /// The interpreter's dispatch draws nothing for a name it does not know, so
    /// a typo would be an empty stage rather than a failure. This is what makes
    /// it a failure.
    #[test]
    fn every_component_a_tree_names_exists() {
        for node in everything() {
            let Node::Component { name, .. } = node else {
                continue;
            };
            assert!(
                COMPONENTS.iter().any(|entry| entry.name == *name),
                "a tree names `{name}`, which is not a component"
            );
        }
    }

    /// A prop a tree sets must be one that component declares.
    #[test]
    fn every_prop_a_tree_sets_is_declared() {
        for node in everything() {
            let Node::Component { name, props, .. } = node else {
                continue;
            };
            let Some(entry) = COMPONENTS.iter().find(|entry| entry.name == *name) else {
                continue;
            };
            for (prop, _) in *props {
                assert!(
                    // `class` is appended to every component by the generator
                    // rather than declared in any spec, so it is the one name
                    // that is legal everywhere and declared nowhere.
                    *prop == "class"
                        || entry
                            .props
                            .iter()
                            .any(|declared| declared.name == *prop && declared.kind != "children"),
                    "a tree sets `{name}.{prop}`, which it does not declare"
                );
            }
        }
    }

    /// An enum prop set to a value the spec does not declare is inert.
    ///
    /// `Scope::pick` returns `None` for a value no variant spells, and the
    /// interpreter then falls back to the default — so a misspelling draws the
    /// default silently. This is the only thing that makes it loud.
    #[test]
    fn every_enum_value_a_tree_sets_is_declared() {
        for node in everything() {
            let Node::Component { name, props, .. } = node else {
                continue;
            };
            let Some(entry) = COMPONENTS.iter().find(|entry| entry.name == *name) else {
                continue;
            };
            for (prop, value) in *props {
                let (Some(declared), Value::Text(spelling)) = (
                    entry
                        .props
                        .iter()
                        .find(|declared| declared.name == *prop && declared.kind == "enum"),
                    value,
                ) else {
                    continue;
                };
                assert!(
                    declared.values.contains(spelling),
                    "a tree sets `{name}.{prop}` to `{spelling}`, and the contract \
                     declares {:?}",
                    declared.values
                );
            }
        }
    }

    /// A bool prop set to anything but a bool.
    ///
    /// `Value::Text("true")` would work by accident today and stop working the
    /// day the reader is changed. The contract says bool, so the tree says bool.
    #[test]
    fn every_bool_a_tree_sets_is_written_as_one() {
        for node in everything() {
            let Node::Component { name, props, .. } = node else {
                continue;
            };
            let Some(entry) = COMPONENTS.iter().find(|entry| entry.name == *name) else {
                continue;
            };
            for (prop, value) in *props {
                let declared = entry
                    .props
                    .iter()
                    .find(|declared| declared.name == *prop && declared.kind == "bool");
                if declared.is_some() {
                    assert!(
                        matches!(value, Value::Bool(_) | Value::Bound(_)),
                        "a tree sets the bool `{name}.{prop}` to {value:?}"
                    );
                }
            }
        }
    }

    /// An icon a tree names must be one the system draws.
    ///
    /// It replaces a test that read `examples.rs` as text looking for
    /// `icon: "…"`. That worked while the examples were closures; with the
    /// closures gone it would have scanned a file with no icon in it and
    /// passed for ever, which is the vacuous-assertion shape this suite keeps
    /// finding. A tree walk cannot go quiet that way.
    #[test]
    fn every_icon_a_tree_names_exists() {
        let mut checked = 0usize;
        for node in everything() {
            let Node::Component { props, .. } = node else {
                continue;
            };
            for (prop, value) in *props {
                if !prop.contains("icon") {
                    continue;
                }
                let Value::Text(name) = value else { continue };
                assert!(
                    noctua_design::IconName::parse(name).is_some(),
                    "a tree names the icon `{name}`, which does not exist"
                );
                checked += 1;
            }
        }
        assert!(checked > 20, "only {checked} icons were checked");
    }

    /// A tone a tree names must be one the palette declares.
    ///
    /// Only where `tone` is a **text** prop, which is the free-form kind that
    /// takes any colour family the system has. Where it is an enum the contract
    /// names its values, `inherit` among them, and
    /// `every_enum_value_a_tree_sets_is_declared` already holds it — asking the
    /// palette about `inherit` is asking the wrong question.
    #[test]
    fn every_tone_a_tree_names_is_one_the_palette_declares() {
        for node in everything() {
            let Node::Component { name, props, .. } = node else {
                continue;
            };
            let Some(entry) = COMPONENTS.iter().find(|entry| entry.name == *name) else {
                continue;
            };
            for (prop, value) in *props {
                if *prop != "tone"
                    || !entry
                        .props
                        .iter()
                        .any(|declared| declared.name == "tone" && declared.kind == "text")
                {
                    continue;
                }
                let Value::Text(name) = value else { continue };
                assert!(
                    noctua_design::tones::known(name),
                    "a tree names the tone `{name}`, which the palette does not declare"
                );
            }
        }
    }

    /// A tag with no arm in `tree::draw` renders nothing at all.
    #[test]
    fn every_tag_a_tree_names_can_be_drawn() {
        for node in everything() {
            let Node::Element { tag, .. } = node else {
                continue;
            };
            assert!(
                TAGS.contains(tag),
                "a tree names `<{tag}>`, which `tree::draw` has no arm for"
            );
        }
    }

    /// Inner HTML belongs to the element that holds it.
    ///
    /// `Node::Raw` anywhere but as an element's only child is silently dropped
    /// by the interpreter, so this refuses it instead.
    #[test]
    fn raw_markup_is_an_elements_only_child() {
        fn check(nodes: &'static [Node]) {
            let raw = nodes
                .iter()
                .filter(|node| matches!(node, Node::Raw(_)))
                .count();
            assert!(
                raw == 0 || (raw == 1 && nodes.len() == 1),
                "a tree puts raw markup beside {} other nodes",
                nodes.len() - raw
            );
            for node in nodes {
                match node {
                    Node::Component { children, .. } | Node::Element { children, .. } => {
                        check(children);
                    }
                    Node::Fragment(nodes) | Node::Each { body: nodes, .. } => check(nodes),
                    Node::Text(_) | Node::Raw(_) => {}
                }
            }
        }
        for component in COMPONENTS {
            for (_, nodes) in for_component(component.name) {
                check(nodes);
            }
        }
        for group in HEROES {
            for (_, _, nodes) in group {
                check(nodes);
            }
        }
    }

    /// A sentence assembled from two catalogue lookups.
    ///
    /// `gates::translated_fragment` refuses this in source, and a `Join` is
    /// data, so it would walk straight past. Word order is not the same in
    /// every language: one sentence, one key, one translation.
    #[test]
    fn a_join_carries_at_most_one_message() {
        fn messages(value: &Value) -> usize {
            match value {
                Value::Message(_) => 1,
                Value::Join(values) => values.iter().map(messages).sum(),
                _ => 0,
            }
        }
        for node in everything() {
            let values: Vec<&Value> = match node {
                Node::Text(value) => vec![value],
                Node::Component { props, .. }
                | Node::Element {
                    attributes: props, ..
                } => props.iter().map(|(_, value)| value).collect(),
                _ => vec![],
            };
            for value in values {
                if let Value::Join(_) = value {
                    assert!(
                        messages(value) <= 1,
                        "a tree joins {} sentences into one run of text",
                        messages(value)
                    );
                }
            }
        }
    }

    /// A page title naming a page the site does not have reads as nothing.
    #[test]
    fn every_page_a_tree_names_exists() {
        fn slugs(value: &Value, into: &mut Vec<&'static str>) {
            match value {
                Value::PageTitle(slug) => into.push(slug),
                Value::Join(values) => values.iter().for_each(|value| slugs(value, into)),
                Value::Template(_, holes) => {
                    holes.iter().for_each(|(_, value)| slugs(value, into));
                }
                _ => {}
            }
        }
        let mut named = Vec::new();
        for node in everything() {
            match node {
                Node::Text(value) => slugs(value, &mut named),
                Node::Component { props, .. }
                | Node::Element {
                    attributes: props, ..
                } => {
                    props.iter().for_each(|(_, value)| slugs(value, &mut named));
                }
                _ => {}
            }
        }
        for slug in named {
            assert!(
                crate::site::by_slug(slug).is_some(),
                "a tree shows the title of `{slug}`, which is not a page"
            );
        }
    }
}

#[cfg(test)]
mod complete {
    use super::*;
    use crate::generated::catalogue::COMPONENTS;
    use crate::tree::{Each, Node, Value};

    /// Every example has a tree.
    ///
    /// The migration passed through a state where most did not, and the judge
    /// skipped an empty one so that each component could land on its own with
    /// the suite green. This is what closes that door: an example added from
    /// here on has to be written as data, because a closure alone fails here.
    #[test]
    fn every_example_has_a_tree() {
        for component in COMPONENTS {
            let examples = crate::examples::for_component(component.name);
            let trees = for_component(component.name);
            assert_eq!(
                trees.len(),
                examples.len(),
                "{} has {} examples and {} trees",
                component.name,
                examples.len(),
                trees.len()
            );
            for (index, (_, nodes)) in trees.iter().enumerate() {
                assert!(
                    !nodes.is_empty(),
                    "{} example {index} has an empty tree",
                    component.name
                );
            }
        }

        for page in crate::site::pages() {
            assert!(
                crate::examples::overview_for_page(page.slug).is_none()
                    || for_page(page.slug).is_some_and(|(_, _, nodes)| !nodes.is_empty()),
                "the hero on {} has no tree",
                page.slug
            );
        }
    }

    /// Every shape the IR declares is one a tree actually uses.
    ///
    /// `Node` and `Value` were designed from a census of all 499 examples
    /// before any of them moved, which is the right way round and also a way to
    /// be wrong: a shape the census predicted and nothing needs is a shape that
    /// would sit in the type for ever, generated into JSON that never carries
    /// it and documented as though somebody writes it.
    ///
    /// So this reads the other direction. A variant nothing constructs fails
    /// here and gets deleted.
    #[test]
    fn every_shape_the_ir_declares_is_one_a_tree_uses() {
        fn values(node: &'static Node) -> Vec<&'static Value> {
            match node {
                Node::Text(value) => vec![value],
                Node::Component { props, .. }
                | Node::Element {
                    attributes: props, ..
                } => props.iter().map(|(_, value)| value).collect(),
                _ => vec![],
            }
        }

        let mut seen: Vec<&str> = Vec::new();
        for node in super::structure::everything() {
            let shape = match node {
                Node::Component { .. } => "Node::Component",
                Node::Element { .. } => "Node::Element",
                Node::Text(_) => "Node::Text",
                Node::Fragment(_) => "Node::Fragment",
                Node::Raw(_) => "Node::Raw",
                Node::Each { over, .. } => {
                    let walk = match over {
                        Each::Values { .. } => "Each::Values",
                        Each::Languages => "Each::Languages",
                        Each::Rows(_) => "Each::Rows",
                    };
                    seen.push(walk);
                    "Node::Each"
                }
            };
            seen.push(shape);
            let mut pending = values(node);
            while let Some(value) = pending.pop() {
                let shape = match value {
                    Value::Text(_) => "Value::Text",
                    Value::Data(_) => "Value::Data",
                    Value::Message(_) => "Value::Message",
                    Value::Bool(_) => "Value::Bool",
                    Value::Asset(_) => "Value::Asset",
                    Value::PageTitle(_) => "Value::PageTitle",
                    Value::Bound(_) => "Value::Bound",
                    Value::Template(_, holes) => {
                        pending.extend(holes.iter().map(|(_, value)| value));
                        "Value::Template"
                    }
                    Value::Join(values) => {
                        pending.extend(values.iter());
                        "Value::Join"
                    }
                };
                seen.push(shape);
            }
        }

        // `Node::Raw` is the one shape the examples do not reach: it exists for
        // the variant grids, whose specimens are drawings the contract carries.
        let declared = [
            "Node::Component",
            "Node::Element",
            "Node::Text",
            "Node::Fragment",
            "Node::Each",
            "Each::Values",
            "Each::Languages",
            "Each::Rows",
            "Value::Text",
            "Value::Data",
            "Value::Message",
            "Value::Bool",
            "Value::Asset",
            "Value::PageTitle",
            "Value::Bound",
            "Value::Join",
            "Value::Template",
        ];
        let unused: Vec<&str> = declared
            .into_iter()
            .filter(|shape| !seen.contains(shape))
            .collect();
        assert!(
            unused.is_empty(),
            "the IR declares {unused:?}, and no tree on the site uses any of them"
        );
    }
}
