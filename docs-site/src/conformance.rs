//! The page the conformance runner drives — `NOCTUA.md §9.3` Leg 3.
//!
//! # Why it is a page and not a test fixture
//!
//! Because the two implementations have to be mounted the way a reader's
//! browser mounts them, and this site is the only place that does both: the
//! Dioxus half is server-rendered here exactly as every example is, and the
//! React half is mounted from the same tree by the same bundle. A fixture that
//! stood the components up some other way would be a third mounting path, and
//! the thing being checked is precisely that the two agree.
//!
//! # Why it is not in the site
//!
//! It is written to `conformance/index.html` and appears in no index, no
//! sidebar and no search. It is apparatus, not documentation — a reader who
//! found it would learn nothing, and every page that IS documentation answers
//! to `every_component_has_examples` and the page-shape audit, which this
//! deliberately does not.

use std::fmt::Write as _;

use dioxus::prelude::*;

use crate::generated::cases::CASES;
use crate::shell::Context;
use crate::tree::Scope;

/// The whole page, as HTML.
#[must_use]
pub(crate) fn document(base_links: &str, bootstrap: &str) -> String {
    // English, and one page's context: a case names no sentence — its values
    // are `Value::Data`, literal by construction — so there is nothing here for
    // a locale to change.
    let scope = Scope::new(Context {
        locale: noctua_design_i18n::Locale::En,
        page: crate::site::THEME_SCOPE,
    });
    let mut html = String::with_capacity(256 * 1024);
    html.push_str("<!doctype html>\n<html lang=\"en\" data-nd-scripting=\"false\">\n<head>\n");
    html.push_str("<meta charset=\"utf-8\">\n");
    html.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n");
    html.push_str("<title>Conformance cases</title>\n");
    // The same nine sheets a consumer links, in the same order, because a case
    // about geometry is a case about the stylesheet that produced it.
    html.push_str(&base_links.replace("href=\"", "href=\"../"));
    html.push_str(bootstrap);
    html.push_str("</head>\n<body>\n");
    html.push_str(
        "<p>Apparatus. Every behavioural case, mounted twice. \
         <code>tests/conformance/cases.spec.js</code> is what reads it.</p>\n",
    );

    for (component, id, tree) in CASES {
        let drawn = dioxus_ssr::render_element(rsx! {
            {crate::tree::render(tree, scope)}
        });
        // EACH HALF IN ITS OWN FORM, because a radio group is scoped by the
        // form it is in and by the document otherwise — so two halves of one
        // case, both rendering `name="size"`, are one group, and the browser
        // moves the chosen answer to whichever was parsed last. The components
        // are right and the apparatus was wrong.
        //
        // Except where the component draws a form itself: `Search` does when it
        // is given an `action`, and a form inside a form is markup the parser
        // discards.
        // A TABLE PART NEEDS A TABLE, or the parser throws the tag away.
        //
        // `<th>` inside a `<div>` is not markup a browser keeps: the element is
        // dropped and its contents are hoisted, so the case would be about a
        // `<div>` that was never rendered. React builds nodes rather than
        // parsing, so it would have kept the `<th>` — and the two halves would
        // have differed for a reason neither component is responsible for.
        let (open, close) = if drawn.starts_with("<th") || drawn.starts_with("<td") {
            ("<table><tbody><tr", "</tr></tbody></table>")
        } else if drawn.starts_with("<tr") {
            ("<table><tbody", "</tbody></table>")
        } else if drawn.contains("<form") {
            ("<div", "</div>")
        } else {
            ("<form onsubmit=\"return false\"", "</form>")
        };
        let _ = write!(
            html,
            "<section data-nd-case=\"{id}\" data-nd-component=\"{component}\">\n\
             {open} data-nd-impl=\"dioxus\">"
        );
        html.push_str(&drawn);
        let _ = write!(
            html,
            "{close}\n{open} data-nd-impl=\"react\">{close}\n</section>\n"
        );
    }

    // The React half, mounted from the same trees. The bundle is optional —
    // `NOCTUA.md §9.11` keeps the Rust half free of Node — so a page without it
    // still carries the Dioxus half, and the runner says which halves it saw
    // rather than passing quietly on one.
    html.push_str(
        "<script type=\"module\">\n\
         import * as behaviours from \"./enhance.js\";\n\
         const trees = await (await fetch(\"./cases-trees.json\")).json();\n\
         // `?raw` mounts both halves and turns NOTHING on.\n\
         //\n\
         // The shape comparison is about what the two implementations DRAW, and\n\
         // an enhancer rearranges what it finds — so comparing after they run\n\
         // compares two scripts' output and calls a different insertion order a\n\
         // divergence. The cases themselves load the page without it, because a\n\
         // case about a keyboard needs the keyboard.\n\
         const asked = new URLSearchParams(location.search);\n\
         // `?only=<id>` keeps one case and removes the rest, BEFORE anything is\n\
         // turned on. Every behaviour here is delegated and several are\n\
         // document-wide, so a page holding every case at once is a page where\n\
         // one case's key press is another case's handler's business. The\n\
         // runner drives one at a time; the shape comparison wants them all.\n\
         const only = asked.get(\"only\");\n\
         if (only) {\n\
        \x20 for (const section of document.querySelectorAll(\"[data-nd-case]\")) {\n\
        \x20   if (section.dataset.ndCase !== only) section.remove();\n\
        \x20 }\n\
         }\n\
         const raw = asked.has(\"raw\");\n\
         const hooks = raw ? { enhance() {}, release() {} } : behaviours;\n\
         hooks.enhance(document);\n\
         try {\n\
        \x20 const react = await import(\"../assets/react/example-react.js\");\n\
        \x20 await react.preload(Object.values(trees).flatMap((tree) => names(tree)));\n\
        \x20 for (const section of document.querySelectorAll(\"[data-nd-case]\")) {\n\
        \x20   const tree = trees[section.dataset.ndCase];\n\
        \x20   const slot = section.querySelector('[data-nd-impl=\"react\"]');\n\
        \x20   if (tree && slot) react.mount(slot, tree, hooks);\n\
        \x20 }\n\
        \x20 document.documentElement.dataset.ndReact = \"mounted\";\n\
         } catch (error) {\n\
        \x20 document.documentElement.dataset.ndReact = \"absent\";\n\
        \x20 document.documentElement.dataset.ndReactWhy = String(error).slice(0, 120);\n\
         }\n\
         // The tree's own spelling: `n` is the component, `c` what is inside.\n\
         function names(node) {\n\
        \x20 if (Array.isArray(node)) return node.flatMap(names);\n\
        \x20 if (!node || typeof node !== \"object\") return [];\n\
        \x20 return [node.n, ...names(node.c ?? [])].filter(Boolean);\n\
         }\n\
         </script>\n",
    );
    html.push_str("</body>\n</html>\n");
    html
}
