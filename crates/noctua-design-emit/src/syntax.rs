//! `dist/syntax.css` — what paints each token role, in every theme.
//!
//! # Why this is generated and not written in `shell.css`
//!
//! `docs-site/assets/shell.css` ships to the documentation site and nowhere
//! else. A consumer of the Dioxus crate or the React package would get a
//! component that renders `data-nd-tok` spans and paints none of them — the
//! component would be half-delivered, in the half nobody notices until they
//! look at it.
//!
//! # The one rule the whole file is shaped by
//!
//! **It declares no custom property at all.** `gates::frozen_color_indirection`
//! fires on any line starting `--` whose value contains `var(--nc-`, and it is
//! right to: a `var()` resolves where it is **declared**, so
//! `--nd-tok-keyword: var(--nc-…)` would freeze to whatever element carries it,
//! and a block inside a `ThemeScope` on another palette would paint the page's
//! colours. Every paint is written at the point of use. A test in this module
//! asserts it on the emitted text, because that gate never sees `dist/`.
//!
//! # How a block and a page settle which theme wins
//!
//! The component **always** stamps its attribute, even at the default — the
//! `Swatch` precedent — so exactly one theme value is ever in play on a block.
//! Every page-keyed rule is then guarded by `[…="auto"]`, which makes the two
//! families disjoint: specificity and source order never arbitrate, and a block
//! that named a theme cannot be reached by a page rule at all.

use std::fmt::Write as _;

use noctua_design_spec::Syntax;
use noctua_design_spec::grammar::Role;

use crate::{CommentStyle, EmittedFile, header_from};

/// The syntax specification path, as written in generated headers.
pub const SYNTAX_SPEC: &str = "spec/syntax.toml";

/// Emits the syntax stylesheet.
#[must_use]
pub fn emit(syntax: &Syntax) -> Vec<EmittedFile> {
    vec![EmittedFile::new("dist/syntax.css", css(syntax))]
}

fn css(syntax: &Syntax) -> String {
    let mut out = header_from(CommentStyle::Block, SYNTAX_SPEC);
    let class = &syntax.meta.class;
    let attribute = &syntax.meta.attribute;
    let token = &syntax.meta.token_attribute;
    let page = &syntax.meta.page_attribute;
    let auto = &syntax.meta.automatic;

    out.push_str(
        "\n/* The syntax themes.\n\
        \x20\n\
        \x20  NO CUSTOM PROPERTY IS DECLARED HERE, and that is the shape of the whole\n\
        \x20  file. A `var()` resolves where it is DECLARED, so `--x: var(--nc-…)`\n\
        \x20  freezes to whatever element carries it and a block inside a ThemeScope\n\
        \x20  on another palette would paint the page's colours. Every paint is\n\
        \x20  written at the point of use.\n\
        \x20\n\
        \x20  NOTHING HERE BRANCHES ON LIGHT AND DARK. `--nc-<family>-text-muted` is\n\
        \x20  declared per palette AND per mode by the colour system, so one mapping\n\
        \x20  is correct in both. A mode branch would also be unwritable: a scope in\n\
        \x20  system mode stamps no data-theme, the palette sheet decides the mode\n\
        \x20  with a compound SAME-ELEMENT selector, and any rule here is\n\
        \x20  ancestor-based. The two disagree for a system scope inside a light\n\
        \x20  page. */\n\n",
    );

    // The face, and the guard on it.
    let _ = writeln!(
        out,
        "/* One face, one weight ladder, no invention. The monospaced family ships\n\
        \x20  Regular, Bold and Italic — and NO Bold Italic — so a role asking for\n\
        \x20  both would get CSS font matching (style before weight), then Italic,\n\
        \x20  then a synthesised bold that smears the stems (NOCTUA.md §7.6 trap 2).\n\
        \x20  A spec validation refuses that pairing; this refuses the synthesis. */\n\
        .{class} {{\n  font-synthesis: none;\n}}\n"
    );

    signals(&mut out, syntax, class, token);
    pinned(&mut out, syntax, class, token);

    for (name, table) in &syntax.theme {
        let resting = *name == syntax.meta.resting;
        theme_rules(
            &mut out, syntax, class, attribute, token, name, table, resting,
        );
    }

    page_rules(&mut out, syntax, class, attribute, token, page, auto);
    automatic_rules(&mut out, syntax, class, attribute, token, auto);
    out
}

/// The signals every theme carries, whatever the colours are.
fn signals(out: &mut String, syntax: &Syntax, class: &str, token: &str) {
    if syntax.signal.is_empty() {
        return;
    }
    out.push_str(
        "\n/* The signals that are not colours. THEME-INVARIANT, because §9.8 requires\n\
        \x20  them whatever the palette is doing — and in `mono` they are the only\n\
        \x20  thing there is. */\n",
    );
    for (role, signal) in &syntax.signal {
        let mut body = String::new();
        if !signal.weight.is_empty() {
            let _ = writeln!(
                body,
                "  font-weight: var(--noctua-weight-{});",
                signal.weight
            );
        }
        if !signal.style.is_empty() {
            let _ = writeln!(body, "  font-style: {};", signal.style);
        }
        if !signal.decoration.is_empty() {
            let _ = writeln!(body, "  text-decoration: {};", signal.decoration);
        }
        if body.is_empty() {
            continue;
        }
        let _ = writeln!(out, ".{class} [{token}=\"{}\"] {{\n{body}}}", role.as_str());
    }
}

/// The roles whose colour is a meaning, painted once for every theme.
fn pinned(out: &mut String, syntax: &Syntax, class: &str, token: &str) {
    if syntax.pinned.is_empty() {
        return;
    }
    out.push_str(
        "\n/* The roles where colour MEANS something rather than categorises. Emitted\n\
        \x20  once rather than per theme, because a removed line is not a category a\n\
        \x20  reader is choosing a palette for: it is `danger` in every theme or the\n\
        \x20  colour is lying. The component owes the gutter sign in every theme for\n\
        \x20  the same reason. */\n",
    );
    for (role, value) in &syntax.pinned {
        let _ = writeln!(
            out,
            ".{class} [{token}=\"{}\"] {{\n  color: var({value});\n}}",
            role.as_str()
        );
    }
}

/// One theme's rules, grouped so identical paints share a selector list.
#[allow(clippy::too_many_arguments)]
fn theme_rules(
    out: &mut String,
    syntax: &Syntax,
    class: &str,
    attribute: &str,
    token: &str,
    name: &str,
    table: &indexmap::IndexMap<Role, String>,
    resting: bool,
) {
    let _ = writeln!(out, "\n/* --- {name} --- */");
    if resting {
        let _ = writeln!(
            out,
            "/* Also what `{}` resolves to when nothing overrides it. `{}` is spelled as\n\
            \x20  its own attribute value rather than left to the absence of one: the\n\
            \x20  component always stamps the prop, so exactly one value is ever in play\n\
            \x20  on a block and no two theme rules can both match it. */",
            syntax.meta.automatic, syntax.meta.automatic
        );
    }
    for (value, roles) in grouped(table) {
        let mut selectors: Vec<String> = Vec::new();
        for role in &roles {
            if resting {
                selectors.push(format!(
                    ".{class}[{attribute}=\"{}\"] [{token}=\"{}\"]",
                    syntax.meta.automatic,
                    role.as_str()
                ));
            }
            selectors.push(format!(
                ".{class}[{attribute}=\"{name}\"] [{token}=\"{}\"]",
                role.as_str()
            ));
        }
        let _ = writeln!(
            out,
            "{} {{\n  color: var({value});\n}}",
            selectors.join(",\n")
        );
    }
}

/// The reader's page-wide choice, which only reaches a block that deferred.
fn page_rules(
    out: &mut String,
    syntax: &Syntax,
    class: &str,
    attribute: &str,
    token: &str,
    page: &str,
    auto: &str,
) {
    let _ = writeln!(
        out,
        "\n/* --- the reader's page-wide choice ---\n\
        \x20\n\
        \x20  :root[…] and not […], because this setting is `scope = false`. Put the\n\
        \x20  attribute on a div and nothing happens, exactly as with backdrop and\n\
        \x20  pattern — the restriction is the selector, not a comment.\n\
        \x20\n\
        \x20  Every rule below is GUARDED BY [{attribute}=\"{auto}\"], so a block whose\n\
        \x20  prop names a theme is never matched by a page rule at all. The two\n\
        \x20  families are disjoint by construction, which is why nothing here depends\n\
        \x20  on specificity or on source order. */"
    );
    for (name, table) in &syntax.theme {
        if *name == syntax.meta.resting {
            // The resting rules above already are this theme, and they carry no
            // page selector — which is what makes them apply when the attribute
            // is ABSENT. `dist/bootstrap.js` stamps a setting's attribute only
            // once a value is stored, and `appearance.rs` shipped exactly that
            // hole once: nothing stamped `data-noctua-glass` until a reader
            // changed it, so on a first visit every frosted surface was flat.
            continue;
        }
        for (value, roles) in grouped(table) {
            let selectors: Vec<String> = roles
                .iter()
                .map(|role| {
                    format!(
                        ":root[{page}=\"{name}\"] .{class}[{attribute}=\"{auto}\"] [{token}=\"{}\"]",
                        role.as_str()
                    )
                })
                .collect();
            let _ = writeln!(
                out,
                "{} {{\n  color: var({value});\n}}",
                selectors.join(",\n")
            );
        }
    }
}

/// What `auto` answers to when the reader's system, not the reader, has spoken.
fn automatic_rules(
    out: &mut String,
    syntax: &Syntax,
    class: &str,
    attribute: &str,
    token: &str,
    auto: &str,
) {
    if syntax.automatic.is_empty() {
        return;
    }
    let _ = writeln!(
        out,
        "\n/* --- what `{auto}` answers to ---\n\
        \x20\n\
        \x20  CAPABILITY, NEVER APPEARANCE. Every one is a media feature, so there is\n\
        \x20  no cascade ambiguity and no ThemeScope interaction at all. A reader who\n\
        \x20  explicitly chose a theme keeps it: the page rules above are (0,6,0) and\n\
        \x20  win, deliberately — an explicit choice is not overridden by an inferred\n\
        \x20  one, the same way `light` is not overridden by prefers-color-scheme. */"
    );
    for entry in &syntax.automatic {
        let Some(table) = syntax.theme.get(&entry.theme) else {
            continue;
        };
        let _ = writeln!(out, "\n/* {} */", entry.because);
        let _ = writeln!(out, "@media {} {{", entry.when);
        for (value, roles) in grouped(table) {
            let selectors: Vec<String> = roles
                .iter()
                .map(|role| {
                    format!(
                        "  .{class}[{attribute}=\"{auto}\"] [{token}=\"{}\"]",
                        role.as_str()
                    )
                })
                .collect();
            let _ = writeln!(
                out,
                "{} {{\n    color: var({value});\n  }}",
                selectors.join(",\n")
            );
        }
        let _ = writeln!(out, "}}");
    }

    // The user agent has already decided; this only stops anything here from
    // fighting it.
    let _ = writeln!(
        out,
        "\n/* Forced colours are the user agent's decision, not this file's. */\n\
         @media (forced-colors: active) {{\n\
         \x20 .{class} [{token}] {{\n    forced-color-adjust: auto;\n  }}\n}}"
    );
}

/// Roles that share a paint, in the order the table declares them.
///
/// Grouping rather than one rule per role, because seventeen roles map onto
/// seven paints and a stylesheet that said so seventeen times would hide the
/// fact that they are seven.
fn grouped(table: &indexmap::IndexMap<Role, String>) -> Vec<(String, Vec<Role>)> {
    let mut out: Vec<(String, Vec<Role>)> = Vec::new();
    for (role, value) in table {
        match out.iter_mut().find(|(seen, _)| seen == value) {
            Some((_, roles)) => roles.push(*role),
            None => out.push((value.clone(), vec![*role])),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shipped() -> Syntax {
        let root = std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../.."));
        let vendor = noctua_design_spec::vendor::load(&root).expect("vendor");
        noctua_design_spec::syntax::load(root.join("spec/syntax.toml"), &vendor).expect("syntax")
    }

    /// The same rule `gates::frozen_color_indirection` enforces on hand-written
    /// source, asserted on generated output the gate never walks.
    #[test]
    fn the_emitted_file_declares_no_custom_property() {
        for line in css(&shipped()).lines() {
            assert!(
                !line.trim_start().starts_with("--"),
                "dist/syntax.css declares `{}`. A var() resolves where it is DECLARED, so a \
                 custom property here would freeze to whatever element carries it and a themed \
                 block would paint the page's colours",
                line.trim()
            );
        }
    }

    /// Every role the scanner can produce is painted by every theme, and the
    /// stylesheet is where that stops being a claim.
    #[test]
    fn every_theme_paints_every_role() {
        let syntax = shipped();
        let text = css(&syntax);
        for name in syntax.theme.keys() {
            for role in Role::ALL {
                let selector = format!(
                    "[data-nd-code-theme=\"{name}\"] [data-nd-tok=\"{}\"]",
                    role.as_str()
                );
                let pinned = syntax.pinned.contains_key(&role);
                assert!(
                    text.contains(&selector) || pinned,
                    "{name} does not paint {}",
                    role.as_str()
                );
            }
        }
    }

    /// A block that named a theme must be unreachable by a page rule, or the
    /// two would be settled by source order rather than by intent.
    #[test]
    fn every_page_rule_is_guarded_by_the_deferring_value() {
        let syntax = shipped();
        for line in css(&syntax).lines() {
            if line
                .trim_start()
                .starts_with(":root[data-noctua-code-theme")
            {
                assert!(
                    line.contains("[data-nd-code-theme=\"auto\"]"),
                    "a page rule reaches a block that named its own theme: {line}"
                );
            }
        }
    }

    /// The gate rejects `max-width` outright and requires every `min-width` to
    /// name a declared breakpoint. A theme has no business asking about either.
    #[test]
    fn no_media_query_here_asks_about_width() {
        assert!(!css(&shipped()).contains("width"));
    }

    /// `syntax` is `scope = false`, and this file is where that is enforced.
    ///
    /// It is the last page-only setting, and the only one whose payload is a
    /// `color` on a DESCENDANT rather than a custom property on the element
    /// itself. That shape cannot be scoped and `spec/theme.toml` says so at
    /// length: two bare rules of equal specificity both match a block between
    /// them, and **source order decides rather than proximity**. The `:root`
    /// anchor is what makes the restriction structural, so it is asserted
    /// here — where the rules actually are — rather than described.
    ///
    /// Read from the spec rather than assumed: if `syntax` is ever made
    /// scopable this test says which file has to change first.
    #[test]
    fn the_setting_is_reachable_only_from_the_root() {
        let root = std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../.."));
        let vendor = noctua_design_spec::vendor::load(&root).expect("vendor");
        let theme = noctua_design_spec::load_theme(root.join("spec/theme.toml"), &vendor)
            .expect("the theme model");
        let setting = &theme.setting["syntax"];
        let attribute = setting.attribute.as_deref().expect("syntax is stamped");
        assert!(
            !setting.scope,
            "syntax became scopable, and this file's `:root` anchor now contradicts \
             the spec — see the note under [setting.syntax]"
        );

        let css = css(&shipped());
        let mentions = css.matches(&format!("[{attribute}")).count();
        assert!(mentions > 0, "the setting reaches nothing at all");
        assert_eq!(
            css.matches(&format!(":root[{attribute}")).count(),
            mentions,
            "a {attribute} rule is not anchored at :root, so two of them can be \
             nested and the winner is whichever was emitted last"
        );
    }
}
