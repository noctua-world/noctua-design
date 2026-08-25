//! A consumer of the Dioxus package, from outside the workspace.
//!
//! # What it proves
//!
//! That the package is usable by someone who is not this repository: it depends
//! on the crate by path from its own workspace, renders components, and then
//! asks the question that matters — **is every class it just rendered actually
//! styled by the stylesheet the packages ship?**
//!
//! That question is the one the whole `dist/components.css` layer exists to
//! answer, and it is the one nothing inside the repository could ask honestly,
//! because inside the repository the documentation site's own stylesheet is on
//! every page.
//!
//! It asks it of `noctua-design-dist`, the package that carries those files to
//! a registry — so what is checked here is the copy a consumer installs, not
//! the directory this repository builds.
//!
//! # Why it renders to a string rather than to a browser
//!
//! Because what a browser would add here is paint, and paint is what
//! `tests/browser/consumer.spec.js` checks by opening the sibling
//! `consumer-web/` example. This half is the compile and the markup: that the
//! crate builds outside the workspace at all, that `rsx!` accepts the props,
//! and that the classes and the stylesheet agree.

use dioxus::prelude::*;
use noctua_design::components::{Badge, Button, ButtonVariant, Heading, Paragraph, Switch, Tag};

fn main() {
    let markup = dioxus_ssr::render_element(rsx! {
        div {
            Heading { "A consumer outside the workspace" }
            Paragraph { "Six components, rendered by a project that has only the crate." }
            Button { label: "Save" }
            Button { label: "Cancel", variant: ButtonVariant::Outline }
            Switch { name: "mail", label: "Send me mail", on: "on", off: "off" }
            Tag { label: "overdue", tone: "overdue" }
            Badge { label: "3 unread messages", count: "3" }
        }
    });

    println!("{markup}\n");

    // The stylesheet, from the package — written out exactly as a consumer's
    // `build.rs` would write it, and read back from where it landed. Reading
    // `../../dist/components.css` would have been shorter and would have proved
    // something no consumer can do.
    let out = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("target/static");
    noctua_design_dist::write_to(&out).expect("write the shipped files out");
    let sheet = std::fs::read_to_string(out.join("components.css"))
        .expect("components.css — the stylesheet a consumer links");

    // Every ROOT class this markup carries has to be painted by the sheet.
    //
    // Root classes rather than every class, because some are deliberately
    // unstyled: `.nd-button-label` is a plain span the button's own rules lay
    // out, and a design system that had a rule for every span it renders would
    // be one nobody could override. What a consumer is owed is that the thing
    // they placed on the page is drawn.
    let mut painted = 0usize;
    for root in [
        "nd-heading",
        "nd-paragraph",
        "nd-button",
        "nd-switch",
        "nd-tag",
        "nd-badge",
    ] {
        assert!(
            markup.contains(&format!("class=\"{root}")),
            "this example stopped rendering {root}"
        );
        assert!(
            sheet.contains(&format!(".{root}")),
            "components.css has no rule for .{root}, so a consumer gets it unstyled"
        );
        painted += 1;
    }

    // And the sheet is the whole layer rather than a stub of one.
    let rendered = classes(&markup);
    let styled = rendered
        .iter()
        .filter(|class| sheet.contains(&format!(".{class}")))
        .count();
    assert!(
        styled * 2 > rendered.len(),
        "only {styled} of {} rendered classes are styled",
        rendered.len()
    );
    println!(
        "{painted} component(s) rendered, every root class painted, \
         {styled} of {} classes styled by the shipped components.css.",
        rendered.len()
    );
}

/// Every `nd-` class in a rendered fragment.
fn classes(markup: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = markup;
    while let Some(at) = rest.find("class=\"") {
        rest = &rest[at + 7..];
        let Some(end) = rest.find('"') else { break };
        for token in rest[..end].split_whitespace() {
            if token.starts_with("nd-") && !out.iter().any(|seen| seen == token) {
                out.push(token.to_owned());
            }
        }
        rest = &rest[end..];
    }
    out
}
