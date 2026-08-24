//! Output targets for the non-color tokens.
//!
//! Every emitter takes the same validated [`Spec`] and writes it in one format.
//! Adding a target is one module and one line in [`emit_all`].
//!
//! **This crate computes nothing.** Values arrive validated and are formatted.
//! An emitter that derived a value would let the checked model and the shipped
//! output disagree about what a token is — the same rule `noctua-colors` holds
//! its emitters to, and for the same reason.
//!
//! # Generated-file headers
//!
//! Every emitted file opens with a header saying it is generated, from what,
//! and how to regenerate it. The header carries **no version and no hash**:
//! either would rewrite every generated file whenever anything changed, burying
//! a one-token diff in hundreds of lines of churn.

use noctua_design_spec::grammar::Grammar;
use noctua_design_spec::icons::{Icons, Resolved as Icon};
use noctua_design_spec::lineage::Cases;
use noctua_design_spec::{Catalogue, Component, Fonts, Manifest, Spec, Syntax, Theme, Vendor};

pub mod appearance;
pub mod component;
pub mod controls;
pub mod css;
pub mod docs;
pub mod fonts;
pub mod grammar;
pub mod i18n;
pub mod icons;
pub mod interpreter;
pub mod javascript;
pub mod lineage;
pub mod props;
pub mod registry;
pub mod runtime;
pub mod rust;
pub mod scope;
pub mod specimens;
pub mod styles;
pub mod syntax;
pub mod theme;
pub mod tones;
pub mod typescript;

/// One file an emitter produced.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmittedFile {
    /// Path relative to the repository root, using forward slashes.
    pub path: String,
    /// Complete file contents.
    pub contents: String,
}

impl EmittedFile {
    /// Builds a file.
    #[must_use]
    pub fn new(path: impl Into<String>, contents: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            contents: contents.into(),
        }
    }
}

/// The command that regenerates everything, quoted in every header.
pub const REGENERATE: &str = "cargo xtask build";

/// The token specification path, as written in generated headers.
pub const SPEC: &str = "spec/tokens.toml";

/// The font specification path, likewise.
pub const FONT_SPEC: &str = "spec/fonts.toml";

/// The theme specification path, likewise.
pub const THEME_SPEC: &str = "spec/theme.toml";

/// The message catalogue directory, likewise.
pub const MESSAGE_SPEC: &str = "spec/i18n/";

/// The component directory, likewise.
pub const COMPONENT_SPEC: &str = "spec/components/";

/// Everything the emitters read.
///
/// A bundle rather than a growing parameter list: the number of inputs is
/// going up — tokens, fonts, the vendored color output, and later the theme
/// model and the message catalogue — and threading each one through every
/// signature makes adding a target a refactor rather than a line.
#[derive(Debug, Clone, Copy)]
pub struct Inputs<'a> {
    /// The non-color token spec.
    pub spec: &'a Spec,
    /// The font layer.
    pub fonts: &'a Fonts,
    /// The validated `noctua-colors` output this repository vendors.
    pub vendor: &'a Vendor,
    /// The theme model.
    pub theme: &'a Theme,
    /// Every locale's messages.
    pub catalogue: &'a Catalogue,
    /// Every component's contract.
    pub components: &'a [Component],
    /// Each sliced family's manifest, read from beside its slices.
    pub sliced: &'a [Manifest],
    /// The icon set's declaration.
    pub icons: &'a Icons,
    /// Every icon, with its drawing read and checked.
    pub drawings: &'a [Icon],
    /// Every language a code block can be shown in.
    pub grammars: &'a [Grammar],
    /// What paints each token role, in every theme.
    pub syntax: &'a Syntax,
    /// The JavaScript half of the scanner, read from beside the Rust half.
    ///
    /// Vendored input rather than something this crate can produce: the two
    /// halves are hand-written together, and an emitter that generated one of
    /// them would be a third implementation to keep in step.
    pub scanner: &'a str,
    /// The expectation both `Lineage` layout engines are checked against.
    pub lineage: &'a Cases,
    /// The JavaScript half of the layout, read from beside the Rust half. Same
    /// reasoning as `scanner`.
    pub layout: &'a str,
    /// The camera, the keyboard walk and the switches — everything a `Lineage`
    /// gains when a script is running. Read from beside the layout.
    pub runtime: &'a str,
    /// The vendored three.js bundle, and its licence.
    pub three: &'a str,
    /// Likewise.
    pub three_licence: &'a str,
}

/// Runs every emitter, returning all files sorted by path.
#[must_use]
pub fn emit_all(inputs: Inputs<'_>) -> Vec<EmittedFile> {
    let Inputs {
        spec,
        fonts: font_layer,
        vendor,
        theme,
        catalogue,
        components,
        sliced,
        icons,
        drawings,
        grammars,
        syntax: syntax_layer,
        scanner,
        lineage: lineage_cases,
        layout,
        runtime,
        three,
        three_licence,
    } = inputs;

    let mut files = Vec::new();
    files.extend(appearance::emit(spec, theme));
    files.extend(component::emit(theme));
    files.extend(css::emit(spec));
    files.extend(docs::emit(components));
    files.extend(fonts::emit(spec, font_layer, sliced));
    files.extend(grammar::emit(grammars, scanner));
    files.extend(i18n::emit(catalogue));
    files.extend(icons::emit(icons, drawings));
    files.extend(interpreter::emit(components));
    files.extend(lineage::emit(
        lineage_cases,
        layout,
        runtime,
        three,
        three_licence,
    ));
    files.extend(props::emit(components, inputs.drawings));
    files.extend(registry::emit(components));
    // `crate::` because `runtime` is also the local holding `Lineage`'s own
    // runtime three lines up, and a reader should not have to know which
    // namespace wins to read this line.
    files.extend(crate::runtime::emit(vendor));
    files.push(fonts::preload_fragment(font_layer));
    files.extend(rust::emit(spec));
    files.extend(scope::emit(vendor));
    files.extend(specimens::emit(components));
    files.extend(styles::emit());
    files.extend(syntax::emit(syntax_layer));
    files.extend(theme::emit(theme, vendor));
    files.extend(tones::emit(vendor));
    files.extend(typescript::emit(spec));
    files.sort_by(|a, b| a.path.cmp(&b.path));
    files
}

/// The line every generated file leads with, in that file's comment syntax.
#[must_use]
pub fn header(comment: CommentStyle) -> String {
    header_from(comment, SPEC)
}

/// The same header, naming the specification this particular file came from.
///
/// The header is how a reader who opened a generated file finds the file they
/// were actually looking for. Naming the wrong one sends them to edit something
/// unrelated, discover it changes nothing, and conclude the build is broken.
#[must_use]
pub fn header_from(comment: CommentStyle, spec: &str) -> String {
    let lines = [
        "Generated by noctua-design — do not edit.".to_owned(),
        format!("Source: {spec}"),
        format!("Regenerate: {REGENERATE}"),
        String::new(),
        "Edits to this file will be overwritten. Change the spec instead.".to_owned(),
    ];

    match comment {
        CommentStyle::Block => {
            let body: String = lines
                .iter()
                .map(|line| {
                    if line.is_empty() {
                        "   *\n".to_owned()
                    } else {
                        format!("   * {line}\n")
                    }
                })
                .collect();
            format!("/*!\n{body}   */\n")
        }
        CommentStyle::Line(marker) => lines
            .iter()
            .map(|line| {
                if line.is_empty() {
                    format!("{marker}\n")
                } else {
                    format!("{marker} {line}\n")
                }
            })
            .collect(),
    }
}

/// How a target's language spells a comment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommentStyle {
    /// A `/* ... */` block.
    Block,
    /// Repeated line comments with the given marker, such as `//` or `#`.
    Line(&'static str),
}

/// Turns a token name into a Rust or TypeScript identifier.
///
/// `in_out` and `in-out` must produce the same constant, because the two
/// spellings are both natural in TOML and a reader should not have to know
/// which one the spec happened to use.
#[must_use]
pub fn identifier(name: &str) -> String {
    name.replace('-', "_")
}

/// Turns a token name into a `SCREAMING_SNAKE_CASE` constant name.
#[must_use]
pub fn constant(name: &str) -> String {
    identifier(name).to_uppercase()
}

/// Turns a token name into `camelCase`, for TypeScript.
#[must_use]
pub fn camel(name: &str) -> String {
    let mut out = String::with_capacity(name.len());
    let mut upper_next = false;
    for ch in name.chars() {
        if ch == '-' || ch == '_' {
            upper_next = true;
        } else if upper_next {
            out.extend(ch.to_uppercase());
            upper_next = false;
        } else {
            out.push(ch);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bundle_sliced() -> Vec<Manifest> {
        let root = std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../.."));
        let fonts = noctua_design_spec::load_fonts(root.join("spec/fonts.toml")).expect("fonts");
        fonts
            .sliced
            .iter()
            .map(|s| s.load(&root.join("assets/fonts")).expect("manifest"))
            .collect()
    }

    fn shipped() -> Bundle {
        let root = std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../.."));
        let vendor = noctua_design_spec::vendor::load(&root).expect("vendor");
        let vendor2 = noctua_design_spec::vendor::load(&root).expect("vendor");
        let theme =
            noctua_design_spec::load_theme(root.join("spec/theme.toml"), &vendor).expect("theme");
        (
            noctua_design_spec::load(root.join("spec/tokens.toml")).expect("tokens"),
            noctua_design_spec::load_fonts(root.join("spec/fonts.toml")).expect("fonts"),
            vendor,
            theme,
            noctua_design_spec::i18n::load_for(
                root.join("spec/i18n"),
                &noctua_design_spec::component::load_all(root.join("spec/components"))
                    .expect("the shipped components"),
            )
            .expect("catalogue"),
            noctua_design_spec::component::load_all(root.join("spec/components"))
                .expect("components"),
            noctua_design_spec::icons::load(
                root.join("spec/icons.toml"),
                root.join("assets/icons"),
            )
            .expect("icons"),
            noctua_design_spec::grammar::load_all(root.join("spec/grammars")).expect("grammars"),
            noctua_design_spec::syntax::load(root.join("spec/syntax.toml"), &vendor2)
                .expect("syntax"),
            noctua_design_spec::lineage::load(root.join("spec/lineage/cases.toml"))
                .expect("lineage cases"),
        )
    }

    type Bundle = (
        Spec,
        Fonts,
        Vendor,
        Theme,
        Catalogue,
        Vec<Component>,
        (Icons, Vec<Icon>),
        Vec<Grammar>,
        Syntax,
        Cases,
    );

    fn inputs(bundle: &Bundle) -> Inputs<'_> {
        Inputs {
            spec: &bundle.0,
            fonts: &bundle.1,
            vendor: &bundle.2,
            theme: &bundle.3,
            catalogue: &bundle.4,
            components: &bundle.5,
            sliced: &[],
            icons: &bundle.6.0,
            drawings: &bundle.6.1,
            grammars: &bundle.7,
            syntax: &bundle.8,
            scanner: SCANNER,
            lineage: &bundle.9,
            layout: LAYOUT,
            runtime: RUNTIME,
            three: THREE,
            three_licence: THREE_LICENCE,
        }
    }

    /// The JavaScript half, read the way `xtask` reads it.
    const SCANNER: &str = include_str!("../../noctua-design-grammars/src/scan.js");

    /// The other JavaScript half, read the same way.
    const LAYOUT: &str = include_str!("../../noctua-design-lineage/src/layout.js");

    /// The runtime and the bundle it imports, read the same way.
    const RUNTIME: &str = include_str!("../../noctua-design-lineage/src/runtime.js");
    const THREE: &str = include_str!("../../../assets/vendor/three/three-css3d.js");
    const THREE_LICENCE: &str = include_str!("../../../assets/vendor/three/LICENSE-three-MIT.txt");

    /// Copied from upstream, byte for byte, so nothing may be prepended to it.
    ///
    /// A generated header on somebody else's MIT licence would be a modified
    /// licence, and one on a minified bundle would be a bundle that no longer
    /// matches the file `assets/vendor/three/ATTRIBUTION.md` says how to
    /// rebuild. The claim these make instead is the stronger one, asserted
    /// below: they are their source, unchanged.
    const VENDORED: &str = "dist/vendor/";

    #[test]
    fn the_vendored_files_are_copies_and_nothing_else() {
        let bundle = shipped();
        let mut found = 0;
        for file in emit_all(inputs(&bundle)) {
            let Some(name) = file.path.strip_prefix(VENDORED) else {
                continue;
            };
            found += 1;
            let source = match name {
                "three-css3d.js" => THREE,
                "LICENSE-three-MIT.txt" => THREE_LICENCE,
                other => panic!("{other} is vendored and nothing says where from"),
            };
            assert_eq!(file.contents, source, "{} was not copied", file.path);
        }
        assert_eq!(found, 2, "the vendored set changed without this test");
    }

    #[test]
    fn every_file_carries_a_generated_header() {
        let bundle = shipped();
        for file in emit_all(inputs(&bundle)) {
            if file.path.starts_with(VENDORED) {
                continue;
            }
            assert!(
                file.contents.contains("do not edit"),
                "{} has no generated header",
                file.path
            );
            assert!(file.contents.contains(REGENERATE), "{}", file.path);
            // Each file names the spec it came from — the token spec or the
            // font spec. Asserting one would make the other's header wrong.
            assert!(
                [
                    SPEC,
                    FONT_SPEC,
                    THEME_SPEC,
                    MESSAGE_SPEC,
                    COMPONENT_SPEC,
                    icons::ICON_SPEC,
                    grammar::GRAMMAR_SPEC,
                    syntax::SYNTAX_SPEC,
                    lineage::LINEAGE_SPEC,
                    lineage::LINEAGE_SOURCE,
                    // The shipped browser behaviour names the file it was
                    // copied from, and the palettes name the vendored output
                    // they are a projection of.
                    noctua_design_runtime::ANCHOR_SOURCE,
                    noctua_design_runtime::COLOR_SOURCE,
                    noctua_design_runtime::PICKER_SOURCE,
                    noctua_design_runtime::MESSAGES_SOURCE,
                    noctua_design_runtime::INPUTS_SOURCE,
                    noctua_design_runtime::CAROUSEL_SOURCE,
                    noctua_design_runtime::SUMMARY_SOURCE,
                    noctua_design_runtime::TABLE_SOURCE,
                    noctua_design_runtime::INFO_SOURCE,
                    noctua_design_runtime::MENU_SOURCE,
                    noctua_design_runtime::MESSAGE_BAR_SOURCE,
                    noctua_design_runtime::DIALOG_SOURCE,
                    noctua_design_runtime::POPOVER_SOURCE,
                    noctua_design_runtime::TOOLTIP_SOURCE,
                    noctua_design_runtime::TOAST_SOURCE,
                    noctua_design_runtime::PANELS_SOURCE,
                    noctua_design_runtime::ROVING_SOURCE,
                    noctua_design_runtime::SELECT_SOURCE,
                    noctua_design_runtime::TAG_PICKER_SOURCE,
                    noctua_design_runtime::TREE_SOURCE,
                    noctua_design_runtime::TABS_SOURCE,
                    noctua_design_runtime::DATE_INPUT_SOURCE,
                    noctua_design_styles::SOURCE_DIR,
                    noctua_design_spec::vendor::VENDOR,
                ]
                .iter()
                .any(|spec| file.contents.contains(spec)),
                "{} names no specification, so a reader cannot find what generated it",
                file.path
            );
        }
    }

    /// A header carrying a version or a hash would rewrite every file on every
    /// release and bury a one-token diff.
    #[test]
    fn the_header_carries_no_version_and_no_hash() {
        let text = header(CommentStyle::Block);
        assert!(!text.contains(env!("CARGO_PKG_VERSION")), "{text}");
        assert!(!text.contains("blake3"), "{text}");
    }

    /// Two builds of the same spec must be byte-identical, or the committed
    /// output cannot be drift-checked.
    #[test]
    fn output_is_deterministic() {
        let bundle = shipped();
        assert_eq!(emit_all(inputs(&bundle)), emit_all(inputs(&bundle)));
    }

    /// Every emitter must produce something, or a target is silently dead.
    #[test]
    fn every_target_emits_at_least_one_file() {
        let (
            spec,
            font_layer,
            vendor,
            theme_model,
            catalogue,
            components,
            (icons, drawings),
            grammars,
            syntax_layer,
            lineage_cases,
        ) = shipped();
        assert!(
            !appearance::emit(&spec, &theme_model).is_empty(),
            "appearance"
        );
        assert!(!component::emit(&theme_model).is_empty(), "component");
        assert!(!icons::emit(&icons, &drawings).is_empty(), "icons");
        assert!(!grammar::emit(&grammars, SCANNER).is_empty(), "grammar");
        assert!(
            !lineage::emit(&lineage_cases, LAYOUT, RUNTIME, THREE, THREE_LICENCE).is_empty(),
            "lineage"
        );
        assert!(!css::emit(&spec).is_empty(), "css");
        assert!(
            !fonts::emit(&spec, &font_layer, &bundle_sliced()).is_empty(),
            "fonts"
        );
        assert!(!i18n::emit(&catalogue).is_empty(), "i18n");
        assert!(!interpreter::emit(&components).is_empty(), "interpreter");
        assert!(!props::emit(&components, &[]).is_empty(), "props");
        assert!(!rust::emit(&spec).is_empty(), "rust");
        assert!(!scope::emit(&vendor).is_empty(), "scope");
        assert!(!specimens::emit(&components).is_empty(), "specimens");
        assert!(!syntax::emit(&syntax_layer).is_empty(), "syntax");
        assert!(!theme::emit(&theme_model, &vendor).is_empty(), "theme");
        assert!(!typescript::emit(&spec).is_empty(), "typescript");
    }

    /// EVERY EMITTED `.js` IS JAVASCRIPT.
    ///
    /// Two of them were not, for an unknown length of time, with
    /// `cargo xtask check` green throughout: `table.js` bound a parameter named
    /// `export` and `lineage.js` declared `const filter` beside its own
    /// `filter` parameter. Both are early errors, so both files failed to load
    /// *entirely* — not a wrong attribute on one component, a `SyntaxError` for
    /// anybody who imported the package.
    ///
    /// Nothing caught it because nothing in this repository had ever read an
    /// emitted `.js` as code. The browser suite fetches a pre-built bundle, and
    /// the parity tests in `props.rs` compare the prop *surface*, which both
    /// files had exactly right.
    ///
    /// Read here rather than by `node --check`, because `NOCTUA.md §9.11`
    /// promises the Rust half tests with no Node installed — see
    /// [`crate::javascript`], which is the reader.
    #[test]
    fn every_emitted_javascript_parses() {
        let bundle = shipped();
        let mut read = 0usize;
        for file in emit_all(inputs(&bundle)) {
            // `dist/vendor/` is somebody else's minified bundle, copied byte
            // for byte and asserted to be a copy above. Reading it would be
            // this repository having an opinion about three.js.
            let javascript = std::path::Path::new(&file.path)
                .extension()
                .is_some_and(|kind| kind.eq_ignore_ascii_case("js"));
            // `docs-site/js/` is emitted too now. It was `packages/` alone,
            // and `registry.js` sat outside it for a year with a generated
            // header and no generator, so nothing here ever read it as code.
            let ours = file.path.starts_with("packages/") || file.path.starts_with("docs-site/js/");
            if !ours || !javascript {
                continue;
            }
            read += 1;
            if let Err(problem) = javascript::check(&file.contents) {
                panic!("{} is not JavaScript — {problem}", file.path);
            }
        }
        assert!(read > 0, "no JavaScript was read, so this proves nothing");
    }

    #[test]
    fn identifiers_are_normalized() {
        assert_eq!(identifier("in-out"), "in_out");
        assert_eq!(constant("in-out"), "IN_OUT");
        assert_eq!(camel("in-out"), "inOut");
        assert_eq!(camel("measure_lead"), "measureLead");
    }
}
