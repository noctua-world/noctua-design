//! The component rules that **ship**, copied into `dist/components.css`.
//!
//! # Why this exists
//!
//! `dist/` carried the token layer, the faces, the appearance layer and the
//! scoped contract — everything a page needs to be *themed* — and none of the
//! rules that make an `.nd-heading` a heading. Those lived in the documentation
//! site's own stylesheet, so a consumer installing either package got correctly
//! themed, **unstyled** components.
//!
//! `syntax.rs` won this argument once already, for one component's tokens: *a
//! consumer of the Dioxus crate or the React package would get a component that
//! renders `data-nd-tok` spans and paints none of them.* This is that argument
//! for all of them.
//!
//! # Nothing here is computed
//!
//! The rules are `crates/noctua-design-styles/src/*.css`, hand-written, and
//! this module concatenates them **in the order that crate declares**, which is
//! the order they were written in and therefore the order they resolve in.
//! `noctua_design_styles::sources` hands them over as `include_str!` constants,
//! so a file renamed out from under this is a compile error naming the file
//! rather than a stylesheet that quietly stops carrying a component.

use std::fmt::Write as _;

use noctua_design_styles::{SOURCE_DIR, sources};

use crate::{CommentStyle, EmittedFile, header_from};

/// Emits the one stylesheet a consumer links.
///
/// One file rather than one per component, because choosing which to link
/// requires knowing which rules a component leans on — `.nd-panel` is shared by
/// four of them, `.nd-ai-slot` by forty-five — and getting that wrong is a
/// component that is silently unstyled, which is the failure this whole file
/// exists to remove.
#[must_use]
pub fn emit() -> Vec<EmittedFile> {
    let mut out = header_from(CommentStyle::Block, SOURCE_DIR);
    for (path, text) in sources() {
        let _ = write!(
            out,
            "\n/* --- {path} ---\n   COPIED, NOT GENERATED. Edit the file named above. */\n{text}"
        );
    }
    vec![EmittedFile::new("dist/components.css", out)]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The sheet is its sources and nothing else.
    ///
    /// Not tidiness. `gates::unknown_token` collects `--name:` declarations
    /// from the sources it walks, and it skips `dist/`. A declaration this
    /// emitter invented would exist nowhere that gate reads, so every `var()`
    /// naming it would be reported as a token nobody declares — or worse, a
    /// real one would go unchecked. Copy, and the question cannot arise.
    #[test]
    fn the_sheet_is_its_sources_and_nothing_else() {
        let sheet = &emit()[0].contents;
        for (path, text) in sources() {
            assert!(sheet.contains(text), "{path} is not in the sheet verbatim");
        }
    }

    /// The cascade is the array's order, and a reader of the sheet can see it.
    #[test]
    fn the_sheet_carries_the_files_in_the_order_the_crate_declares() {
        let sheet = &emit()[0].contents;
        let mut at = 0;
        for (path, _) in sources() {
            let found = sheet[at..]
                .find(path)
                .unwrap_or_else(|| panic!("{path} is out of order in the sheet"));
            at += found;
        }
    }
}

/// Reading what the shipped code can put in the DOM, and what the two
/// stylesheets say about it.
///
/// These live here rather than in `xtask` because the question they answer is
/// this module's: the sheet it emits must carry every component rule, and the
/// site's own stylesheet must carry none.
#[cfg(test)]
mod split {
    use std::collections::BTreeSet;
    use std::path::{Path, PathBuf};

    fn root() -> PathBuf {
        PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../.."))
    }

    /// Every `nd-` token in a whitespace-separated class list.
    fn tokens(list: &str, into: &mut BTreeSet<String>) {
        for token in list.split_whitespace() {
            if token.starts_with("nd-")
                && token
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
            {
                into.insert(token.to_owned());
            }
        }
    }

    /// The string literal after each occurrence of `marker`.
    fn after(text: &str, marker: &str, into: &mut BTreeSet<String>) {
        let mut rest = text;
        while let Some(at) = rest.find(marker) {
            rest = &rest[at + marker.len()..];
            if let Some(end) = rest.find(['"', '`']) {
                tokens(&rest[..end], into);
            }
        }
    }

    fn files(dir: &Path, extension: &str, into: &mut Vec<String>) {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                files(&path, extension, into);
            } else if path.extension().is_some_and(|e| e == extension) {
                into.push(std::fs::read_to_string(&path).expect("readable"));
            }
        }
    }

    /// Every class the shipped components and the shipped runtime can write.
    ///
    /// Four sources, each read where it is written rather than guessed from a
    /// naming rule: a contract's own `[meta] class`, the two generated
    /// implementations, and the behaviour modules. Class contexts only — an id
    /// like `nd-info-<name>` is not a class, and letting ids in would make the
    /// residue gate below wave through rules nothing can match.
    fn emittable() -> BTreeSet<String> {
        let root = root();
        let mut out = BTreeSet::new();

        let mut specs = Vec::new();
        files(&root.join("spec/components"), "toml", &mut specs);
        for text in &specs {
            for line in text.lines() {
                if let Some(rest) = line.trim().strip_prefix("class = \"") {
                    if let Some(end) = rest.find('"') {
                        tokens(&rest[..end], &mut out);
                    }
                }
            }
        }

        let mut dioxus = Vec::new();
        files(&root.join("crates/noctua-design/src"), "rs", &mut dioxus);
        for text in &dioxus {
            after(text, "class: \"", &mut out);
            after(text, "format!(\"", &mut out);
        }

        let mut react = Vec::new();
        files(&root.join("packages/react/src"), "js", &mut react);
        for text in &react {
            after(text, "className: \"", &mut out);
            after(text, "className: `", &mut out);
            after(text, "[\"", &mut out);
        }

        let mut runtime = Vec::new();
        files(
            &root.join("crates/noctua-design-runtime/src"),
            "js",
            &mut runtime,
        );
        for text in &runtime {
            // A behaviour module writes classes in three shapes, and holds them
            // in config objects in a fourth. A literal whose every token is an
            // `nd-` name is a class list in all four.
            for literal in text.split('"').skip(1).step_by(2) {
                if !literal.is_empty()
                    && literal
                        .split_whitespace()
                        .all(|t| t.starts_with("nd-") && !t.contains('.'))
                {
                    tokens(literal, &mut out);
                }
            }
            // ...and the ones it only reads, which a component must have drawn.
            after(text, "querySelector(\".", &mut out);
            after(text, "querySelectorAll(\".", &mut out);
        }
        out
    }

    /// A selector list split on its own commas — never on one inside `:is(…)`,
    /// where the comma separates arguments and the halves are not selectors.
    fn top_level_commas(prelude: &str) -> Vec<String> {
        let mut out = Vec::new();
        let mut depth = 0usize;
        let mut current = String::new();
        for c in prelude.chars() {
            match c {
                '(' => {
                    depth += 1;
                    current.push(c);
                }
                ')' => {
                    depth = depth.saturating_sub(1);
                    current.push(c);
                }
                ',' if depth == 0 => out.push(std::mem::take(&mut current)),
                _ => current.push(c),
            }
        }
        out.push(current);
        out
    }

    /// A stylesheet with its comments taken out.
    fn without_comments(css: &str) -> String {
        let mut out = String::with_capacity(css.len());
        let mut rest = css;
        while let Some(at) = rest.find("/*") {
            out.push_str(&rest[..at]);
            match rest[at..].find("*/") {
                Some(end) => rest = &rest[at + end + 2..],
                None => return out,
            }
        }
        out.push_str(rest);
        out
    }

    /// The `.nd-*` classes named in every selector of a stylesheet, one entry
    /// per selector, with at-rule bodies read as the selectors they contain.
    fn selectors(css: &str) -> Vec<(String, BTreeSet<String>)> {
        let mut out = Vec::new();
        let bytes: Vec<char> = css.chars().collect();
        let mut i = 0;
        let mut start = 0;
        while i < bytes.len() {
            if bytes[i] == '/' && bytes.get(i + 1) == Some(&'*') {
                let mut j = i + 2;
                while j + 1 < bytes.len() && !(bytes[j] == '*' && bytes[j + 1] == '/') {
                    j += 1;
                }
                i = j + 2;
                continue;
            }
            if bytes[i] == '{' {
                // The comments were stepped over rather than removed, so a
                // sentence between two rules would otherwise read as a
                // selector — and one of them mentions `.nd-rule`.
                let prelude = without_comments(&bytes[start..i].iter().collect::<String>());
                let mut depth = 1;
                let mut j = i + 1;
                while j < bytes.len() && depth > 0 {
                    if bytes[j] == '/' && bytes.get(j + 1) == Some(&'*') {
                        let mut k = j + 2;
                        while k + 1 < bytes.len() && !(bytes[k] == '*' && bytes[k + 1] == '/') {
                            k += 1;
                        }
                        j = k + 2;
                        continue;
                    }
                    match bytes[j] {
                        '{' => depth += 1,
                        '}' => depth -= 1,
                        _ => {}
                    }
                    j += 1;
                }
                if prelude.trim_start().starts_with('@') {
                    if !prelude.trim_start().starts_with("@keyframes") {
                        let inner: String = bytes[i + 1..j.saturating_sub(1)].iter().collect();
                        out.extend(selectors(&inner));
                    }
                } else {
                    for one in top_level_commas(&prelude) {
                        let one = one.trim();
                        if one.is_empty() {
                            continue;
                        }
                        let mut named = BTreeSet::new();
                        let mut rest = one;
                        while let Some(at) = rest.find(".nd-") {
                            rest = &rest[at + 1..];
                            let end = rest
                                .find(|c: char| {
                                    !(c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
                                })
                                .unwrap_or(rest.len());
                            named.insert(rest[..end].to_owned());
                        }
                        out.push((one.to_owned(), named));
                    }
                }
                i = j;
                start = j;
                continue;
            }
            if bytes[i] == '}' {
                start = i + 1;
            }
            i += 1;
        }
        out
    }

    /// A harvest that quietly stopped matching would wave everything through.
    #[test]
    fn the_class_harvest_still_finds_what_it_is_meant_to() {
        let found = emittable();
        assert!(
            found.len() > 400,
            "the harvest found only {} class names, which is too few to be right",
            found.len()
        );
        // One name per shape it is written in, so a harvester that stops
        // matching one of the four fails loudly rather than shrinking.
        for name in [
            "nd-switch",       // a contract's own [meta] class
            "nd-switch-track", // a literal in the generated Dioxus
            "nd-button-label", // a literal in the generated React
            "nd-note-close",   // a class list in a runtime module
        ] {
            assert!(found.contains(name), "{name} fell out of the class harvest");
        }
    }

    /// No component rule is left in the documentation site's stylesheet.
    ///
    /// This is the gate the whole split exists for: a rule whose selector names
    /// only classes the components emit belongs to the components, and a
    /// consumer who installs the packages has to receive it.
    ///
    /// The site's own chrome passes without an exception list, because a chrome
    /// class is one nothing emits. So do the four families the site's script
    /// builds — `.nd-select-list` and its kind — which is exactly right while
    /// that script is the only thing that draws them: the day a behaviour moves
    /// into `dist/`, its classes enter the harvest and this test asks for its
    /// rules in the same change.
    #[test]
    fn no_component_rule_is_left_in_the_documentation_shell() {
        let shell = std::fs::read_to_string(root().join("docs-site/assets/shell.css"))
            .expect("the shell stylesheet");
        let emittable = emittable();
        let mut left = Vec::new();
        let mut checked = 0usize;
        for (selector, named) in selectors(&shell) {
            if named.is_empty() {
                continue; // element and attribute selectors: decided by hand
            }
            checked += 1;
            if named.iter().all(|class| emittable.contains(class)) {
                left.push(selector);
            }
        }
        assert!(checked > 100, "only {checked} selectors were read");
        assert!(
            left.is_empty(),
            "these rules style components and ship nowhere:\n  {}",
            left.join("\n  ")
        );
    }

    /// And the sheet that ships carries no rule for the documentation site.
    #[test]
    fn the_shipped_sheet_names_only_classes_a_component_can_emit() {
        let emittable = emittable();
        let mut foreign = Vec::new();
        let mut checked = 0usize;
        for (path, text) in super::sources() {
            for (selector, named) in selectors(text) {
                checked += 1;
                for class in named {
                    if !emittable.contains(&class) {
                        foreign.push(format!("{path}: {selector}"));
                    }
                }
            }
        }
        assert!(checked > 100, "only {checked} selectors were read");
        assert!(
            foreign.is_empty(),
            "these shipped rules name classes nothing emits:\n  {}",
            foreign.join("\n  ")
        );
    }
}
