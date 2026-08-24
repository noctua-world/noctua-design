//! Every example, as JSON, for the browser to render with React.
//!
//! # Why a file per locale rather than markup in the page
//!
//! The site has already made this call once, for the search index, and the
//! reason is written down at `main::main`: those entries used to ship inside
//! every document, thirty-five kilobytes each, for a list most readers never
//! open. Here they are one file the browser caches for the whole visit.
//!
//! The same arithmetic applies twice over here. A component page carries up to
//! twenty-two examples, `/lineage/` carries a graph string per example that is
//! not in its HTML at all today, and perhaps one reader in twenty ever asks for
//! React. Inlining them measured at three to six percent of every page's
//! transfer, for ever, for a feature almost nobody uses.
//!
//! The cost of the sidecar is one more request on the first switch, and it is
//! not really a cost: it is fetched **beside** the React bundle rather than
//! after it, so the first switch waits for the slower of the two, not for both.
//!
//! # Everything is resolved here
//!
//! The page is built once per locale, so by the time a tree reaches this file
//! there is nothing left to decide: messages are looked up, page titles are in
//! the reader's language, asset paths carry their `../../`, and the loops are
//! walked. What crosses to the browser is a flat tree of components, markup and
//! text. That is also what keeps `packages/tokens/i18n.js` — 1.2 MB of message
//! catalogue — out of the bundle for ever.

use std::fmt::Write as _;

use noctua_design_i18n::Locale;

use crate::generated::catalogue::{COMPONENTS, DocComponent};
use crate::shell::Context;
use crate::site::Page;
use crate::tree::{Node, Scope, Value};

/// Every example on the site, keyed by the frame that shows it.
///
/// The key is the frame's own `name`, which `pages.rs` builds from the
/// component's slug and the example's index. It is not a second naming scheme:
/// it is the one already in the markup, so the browser finds a frame's tree by
/// reading the frame.
#[must_use]
pub(crate) fn json(locale: Locale) -> String {
    let mut out = String::with_capacity(256 * 1024);
    out.push('{');
    let mut first = true;

    for page in crate::site::pages() {
        let context = Context { locale, page };
        if let Some((_, _, nodes)) = crate::trees::for_page(page.slug) {
            entry(
                &mut out,
                &mut first,
                &format!("{}-overview", page.slug),
                nodes,
                context,
            );
        }
    }

    // The `ThemeScope` demonstration, which is not one of the 499.
    entry(
        &mut out,
        &mut first,
        "scope-0",
        crate::trees::SCOPE,
        Context {
            locale,
            page: crate::site::THEME_SCOPE,
        },
    );

    for component in COMPONENTS {
        let context = Context {
            locale,
            page: page_for(&component),
        };
        for (index, (_, nodes)) in crate::trees::for_component(component.name)
            .iter()
            .enumerate()
        {
            if nodes.is_empty() {
                continue;
            }
            entry(
                &mut out,
                &mut first,
                &format!("{}-{index}", component.slug),
                nodes,
                context,
            );
        }

        // And the variant grids, which are examples the contract wrote. Each is
        // one frame, so each is one entry, keyed by the name `pages.rs` already
        // gives that frame.
        for prop in component
            .props
            .iter()
            .filter(|prop| crate::pages::drawn(prop))
        {
            grid(&mut out, &mut first, &component, prop, context);
        }
    }

    out.push('}');
    out
}

/// The page a component is documented on.
fn page_for(component: &DocComponent) -> Page {
    crate::site::by_slug(component.page.unwrap_or(component.slug))
        .unwrap_or_else(|| panic!("{} is documented on no page", component.name))
}

/// One variant grid: every value of one prop, side by side.
///
/// # Why the row is written here and the figures are not
///
/// A specimen is generated with the caption that names it, because the caption
/// is the accessible name of any picture that has no other, and one description
/// of that pair is what stops the two renderers disagreeing. The row around them
/// is the page's own arrangement, and `pages::ComponentSections` draws the same
/// one — a `div` with a class, holding whatever the values come to.
fn grid(
    out: &mut String,
    first: &mut bool,
    component: &DocComponent,
    prop: &crate::generated::catalogue::DocProp,
    context: Context,
) {
    if !*first {
        out.push(',');
    }
    *first = false;
    string(out, &format!("{}-{}", component.slug, prop.name));
    out.push_str(r#":[{"k":"e","n":"div","p":{"className":"nd-variant-row"},"c":["#);
    let scope = Scope::new(context);
    let mut inner = true;
    for value in prop.values {
        for node in crate::generated::specimens::nodes(component.name, prop.name, value) {
            one(out, node, scope, &mut inner);
        }
    }
    out.push_str("]}]");
}

pub(crate) fn entry(
    out: &mut String,
    first: &mut bool,
    name: &str,
    nodes: &[Node],
    context: Context,
) {
    if !*first {
        out.push(',');
    }
    *first = false;
    string(out, name);
    out.push(':');
    array(out, nodes, Scope::new(context));
}

fn array(out: &mut String, nodes: &[Node], scope: Scope) {
    out.push('[');
    let mut first = true;
    for node in nodes {
        one(out, node, scope, &mut first);
    }
    out.push(']');
}

/// One node, and everything an `Each` or a `Fragment` stands for.
///
/// Both are flattened here rather than crossing to the browser, because both
/// are resolved by the time the page is built and a shape the interpreter never
/// meets is a shape it cannot get wrong.
fn one(out: &mut String, node: &Node, scope: Scope, first: &mut bool) {
    match node {
        Node::Fragment(nodes) => {
            for node in *nodes {
                one(out, node, scope, first);
            }
        }
        Node::Each { over, body } => {
            for row in scope.rows(*over) {
                for node in *body {
                    one(out, node, row, first);
                }
            }
        }
        // Lifted onto its parent by `element` below, exactly as the Rust half
        // lifts it onto the element that holds it.
        Node::Raw(_) => {}
        Node::Text(value) => {
            comma(out, first);
            out.push_str(r#"{"k":"t","v":"#);
            string(out, &scope.text(value));
            out.push('}');
        }
        Node::Component {
            name,
            props,
            children,
        } => {
            comma(out, first);
            let _ = write!(out, r#"{{"k":"c","n":"{name}","p":{{"#);
            let declared = COMPONENTS.iter().find(|entry| entry.name == *name);
            let mut inner = true;
            for (prop, value) in *props {
                comma(out, &mut inner);
                string(out, &react_name(prop));
                out.push(':');
                // A bool prop reaches React as a bool. The contract is what says
                // which props those are, so this cannot disagree with the Rust
                // half about `disabled` any more than the generator can.
                let boolean = declared.is_some_and(|entry| {
                    entry
                        .props
                        .iter()
                        .any(|declared| declared.name == *prop && declared.kind == "bool")
                });
                // An icon prop carries a NAME on the Rust side and a DRAWING on
                // the React one, so this is where the name becomes one.
                if is_icon(name, prop) {
                    drawing(out, &scope.text(value));
                } else {
                    value_json(out, value, scope, boolean);
                }
            }
            out.push_str("},\"c\":");
            array(out, children, scope);
            out.push('}');
        }
        Node::Element {
            tag,
            attributes,
            children,
        } => {
            comma(out, first);
            let _ = write!(out, r#"{{"k":"e","n":"{tag}","p":{{"#);
            let mut inner = true;
            for (attribute, value) in *attributes {
                comma(out, &mut inner);
                string(out, &react_name(attribute));
                out.push(':');
                value_json(out, value, scope, matches!(value, Value::Bool(_)));
            }
            if let [Node::Raw(markup)] = children {
                comma(out, &mut inner);
                out.push_str(r#""dangerouslySetInnerHTML":{"__html":"#);
                string(out, markup);
                out.push('}');
                out.push_str("},\"c\":[]}");
                return;
            }
            out.push_str("},\"c\":");
            array(out, children, scope);
            out.push('}');
        }
    }
}

fn value_json(out: &mut String, value: &Value, scope: Scope, boolean: bool) {
    let text = scope.text(value);
    if boolean {
        out.push_str(if text == "true" { "true" } else { "false" });
    } else {
        string(out, &text);
    }
}

fn comma(out: &mut String, first: &mut bool) {
    if !*first {
        out.push(',');
    }
    *first = false;
}

/// What React calls the same prop.
///
/// The spec spells props in snake case and React in camel, which is the
/// translation `props.rs` already makes when it generates the React component —
/// so this is the same rule, not a second one. `class` and `for` are the two
/// React renamed outright.
fn react_name(name: &str) -> String {
    match name {
        "class" => return "className".to_owned(),
        "for" => return "htmlFor".to_owned(),
        _ => {}
    }
    // An attribute with a dash is an HTML attribute — `aria-label`,
    // `data-nd-size` — and React takes those verbatim.
    if name.contains('-') {
        return name.to_owned();
    }
    let mut out = String::with_capacity(name.len());
    let mut upper = false;
    for ch in name.chars() {
        if ch == '_' {
            upper = true;
        } else if upper {
            out.extend(ch.to_uppercase());
            upper = false;
        } else {
            out.push(ch);
        }
    }
    out
}

/// A JSON string.
///
/// `<` is written as `<`, which is legal JSON and costs five bytes for a
/// character that hardly ever appears in prose. It means this text can never
/// close a `<script>` element, which matters if these trees are ever inlined —
/// and it matters more than it looks, because the failure would be silent in
/// both the browser and the pseudo-locale scanner that reads for missed
/// translations.
fn string(out: &mut String, text: &str) {
    out.push('"');
    for ch in text.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '<' => out.push_str("\\u003c"),
            ch if (ch as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", ch as u32);
            }
            ch => out.push(ch),
        }
    }
    out.push('"');
}

/// The React call for an example's subject, as a caption.
///
/// # Why this is one call and not the whole tree
///
/// The caption band is a discreet header over the stage, and the Dioxus caption
/// in it is one line naming the component the example is about. It is already
/// lossy on purpose — half of them elide props with `…` — because it is a
/// caption rather than a transcription.
///
/// Rendering a whole tree as JSX would put two hundred lines where one line was
/// and bury the example under its own source. So this finds the same subject the
/// Dioxus caption names and writes that call, which is the same promise kept in
/// the other language.
#[must_use]
pub(crate) fn jsx(nodes: &[Node], subject: &str, context: Context) -> Option<String> {
    let outer = Scope::new(context);
    let (node, scope) = find(nodes, subject, outer).or_else(|| find(nodes, "", outer))?;
    let Node::Component {
        name,
        props,
        children,
    } = node
    else {
        return None;
    };

    let declared = COMPONENTS.iter().find(|entry| entry.name == *name);
    let mut out = format!("<{name}");
    for (prop, value) in *props {
        let text = scope.text(value);
        let boolean = declared.is_some_and(|entry| {
            entry
                .props
                .iter()
                .any(|declared| declared.name == *prop && declared.kind == "bool")
        });
        let name = react_name(prop);
        if boolean {
            // `<Button disabled>` is how React spells true, and writing
            // `disabled={false}` is how it spells a prop worth mentioning that
            // is off. Neither is `disabled="false"`, which is truthy.
            if text == "true" {
                let _ = write!(out, " {name}");
            } else {
                let _ = write!(out, " {name}={{false}}");
            }
        } else {
            let _ = write!(out, " {name}=\"{text}\"");
        }
    }
    // The children are the example, and the example is on the screen under it.
    if children.is_empty() {
        out.push_str(" />");
    } else {
        let _ = write!(out, ">…</{name}>");
    }
    Some(out)
}

/// The first component in a tree, by name, or the first of any name.
///
/// The scope travels with it. A subject inside a walk reads its row through
/// `Value::Bound`, and resolving that outside the walk gave `weight=""` on the
/// first caption this was tried on — a caption that named a prop and showed it
/// empty, which is worse than naming nothing.
fn find<'a>(nodes: &'a [Node], subject: &str, scope: Scope) -> Option<(&'a Node, Scope)> {
    for node in nodes {
        match node {
            Node::Component { name, children, .. } => {
                if subject.is_empty() || *name == subject {
                    return Some((node, scope));
                }
                if let Some(found) = find(children, subject, scope) {
                    return Some(found);
                }
            }
            Node::Element { children, .. } => {
                if let Some(found) = find(children, subject, scope) {
                    return Some(found);
                }
            }
            Node::Fragment(nodes) => {
                if let Some(found) = find(nodes, subject, scope) {
                    return Some(found);
                }
            }
            Node::Each { over, body } => {
                // The first turn, because a caption shows one call and a walk
                // draws the same call several times over.
                if let Some(row) = scope.rows(*over).into_iter().next()
                    && let Some(found) = find(body, subject, row)
                {
                    return Some(found);
                }
            }
            Node::Text(_) | Node::Raw(_) => {}
        }
    }
    None
}

/// Whether a prop of this component carries an icon name.
fn is_icon(component: &str, prop: &str) -> bool {
    crate::generated::icon_props::ICON_PROPS
        .iter()
        .any(|(name, props)| *name == component && props.contains(&prop))
}

/// One icon, as the React half needs it: the drawing, not its name.
///
/// **Resolved here rather than in the browser.** React's `Icon` takes a
/// drawing, by `NOCTUA.md §9.5`'s rule that one icon is one module and a table
/// of every drawing is what made one icon cost all of them — so a name is not
/// something the browser can turn into a drawing without importing one module
/// per name. It was a name until 2026-08-24, and every React example carrying
/// an icon threw `Icon needs a drawing` and drew nothing; the conformance
/// runner is what finally asked.
fn drawing(out: &mut String, name: &str) {
    let Some(icon) = noctua_design_icons::IconDrawing::parse(name) else {
        string(out, name);
        return;
    };
    out.push('{');
    string(out, "name");
    out.push(':');
    string(out, icon.name);
    out.push(',');
    string(out, "viewBox");
    out.push(':');
    string(out, icon.view_box);
    out.push(',');
    string(out, "filled");
    out.push_str(if icon.filled { ":true," } else { ":false," });
    string(out, "body");
    out.push(':');
    string(out, icon.body);
    out.push('}');
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_tree_can_close_a_script_element() {
        for locale in Locale::ALL {
            let text = json(locale);
            assert!(
                !text.contains('<'),
                "{}'s examples carry a `<`, which could close the element \
                 holding them",
                locale.tag()
            );
        }
    }

    #[test]
    fn a_prop_reaches_react_by_the_name_react_uses() {
        assert_eq!(react_name("hide_label"), "hideLabel");
        assert_eq!(react_name("icon_end"), "iconEnd");
        assert_eq!(react_name("class"), "className");
        assert_eq!(react_name("for"), "htmlFor");
        assert_eq!(react_name("aria-label"), "aria-label");
        assert_eq!(react_name("data-nd-size"), "data-nd-size");
        assert_eq!(react_name("label"), "label");
    }
}
