//! The Tailwind bridge: the vendor's colour mapping, and this system's own
//! tokens after it.
//!
//! # One file, two authors
//!
//! **This system styles nothing with Tailwind, and an application built on it
//! may.** `dist/tailwind/bridge.css` is the one file such an application
//! imports after `tailwindcss`. Its first part is `noctua-colors`' mapping of
//! every `--color-*` utility name onto a `--nc-*` token, copied byte for byte
//! as the colour sheets are. Its second part is written here, from
//! `spec/tokens.toml` and `spec/fonts.toml`: the spacing scale, the radii, the
//! type sizes, line heights, weights and letter spacing, the font roles, the
//! shell's lengths, the smallest target, the measure, the easings and the
//! breakpoints, each as the Tailwind v4 theme variable its utilities read, and
//! then the durations, the layers, the border widths and the focus ring, each
//! as a utility of its own.
//!
//! The bridge alone and not `theme.css`, the vendor's entry point: that file
//! imports the whole colour system by a path relative to the vendor's own
//! tree, and the sheets it would reach are already in `dist/` under other
//! names.
//!
//! # Why a step is a name and the multiplier is absent
//!
//! Tailwind v4 answers `gap-4` in two ways: by a theme variable named
//! `--spacing-4`, or by multiplying a bare `--spacing`. This block declares
//! the first for every step of the scale and never the second, so `gap-4` is
//! `var(--noctua-space-4)` and `gap-5`, which is no step of the scale, emits
//! nothing. A bare `--spacing` would make every number a utility, and a
//! number between two steps is the token literal invariant 2 refuses.
//!
//! **Leaving it undeclared was not enough.** `@import "tailwindcss"`, the
//! import every guide shows, brings Tailwind's own `theme.css` and its
//! `--spacing: 0.25rem`, and under it `gap-5`, `p-0.5` and `w-96` were
//! utilities again (measured on 4.3.5). So the block says `--spacing: initial`,
//! which takes the multiplier out of the theme whichever import came first.
//! It takes out that one variable and no other: Tailwind's default type
//! sizes, radii and palette are still there under the full import, and
//! `tailwindcss/utilities.css` alone is the import that brings none of them.
//!
//! # What Tailwind has no theme variable for
//!
//! A duration, a layer and a border width are bare numbers to Tailwind v4:
//! `duration-150`, `z-10`, `border-2`. No theme namespace names them, so a
//! theme block cannot point them at a token, and the number an application
//! would write is the literal this file exists to keep out. A `duration-150`
//! also stays 150 milliseconds under `prefers-reduced-motion`, where
//! `--noctua-duration-*` collapses. So each is a named `@utility` after the
//! block: `duration-fast`, `z-dialog`, `border-hairline` and its sides,
//! `outline-focus`, `outline-offset-focus`. An easing has a namespace,
//! `--ease-*`, and is in the block.
//!
//! **Every token the sheets declare is either reached or excused by name**
//! (`EXCUSED` in the tests). A group this file has never heard of fails the
//! build, so leaving one out is a decision somebody wrote down.
//!
//! # `inline`, and the one block that cannot be
//!
//! `@theme inline` writes the `var()` into the utility instead of copying the
//! value to `:root`, so a utility follows a token wherever the token is
//! declared again: a radius preset on a scope, the reduced-motion layer. A
//! breakpoint is the exception. A media query cannot read a custom property,
//! so each is written as the length the spec declares, which this emitter
//! reads from the same table `dist/tokens.css` is written from.

use std::fmt::Write as _;

use noctua_design_spec::{Fonts, Spec};

use crate::EmittedFile;

/// Where the Tailwind bridge ships.
pub const BRIDGE: &str = "dist/tailwind/bridge.css";

/// The line the design tokens' half of the bridge starts with.
///
/// Public so a test can split the file where the vendor's bytes end.
pub const SEAM: &str = "/* The design tokens, appended by noctua-design — do not edit.";

/// The bridge: the vendor's mapping, then the design tokens'.
#[must_use]
pub fn bridge(vendor: &str, spec: &Spec, fonts: &Fonts) -> EmittedFile {
    let mut out = String::with_capacity(vendor.len() + 4096);
    out.push_str(vendor);
    if !vendor.ends_with('\n') {
        out.push('\n');
    }
    out.push('\n');
    out.push_str(&tokens(spec, fonts));
    EmittedFile::new(BRIDGE, out)
}

/// Whether a token's value is a length or a percentage, and so can stand
/// where Tailwind expects a size.
///
/// The shell holds two numbers with no unit, the most lines a prompt grows to
/// and a drawer's largest share, and a number is no size.
fn is_length(value: &str) -> bool {
    let value = value.trim();
    value.parse::<f64>().is_err()
}

/// The sides Tailwind draws a border on, as the infix of its utility and the
/// property each sets: `border-b-hairline` is `border-bottom-width`.
const BORDER_SIDES: &[(&str, &[&str])] = &[
    ("", &["border"]),
    ("-x", &["border-inline"]),
    ("-y", &["border-block"]),
    ("-s", &["border-inline-start"]),
    ("-e", &["border-inline-end"]),
    ("-t", &["border-top"]),
    ("-r", &["border-right"]),
    ("-b", &["border-bottom"]),
    ("-l", &["border-left"]),
];

/// The utility a focus token is, and the property it sets. A focus token this
/// table does not name has no utility, and the test that asks for one fails.
const FOCUS: &[(&str, &str, &str)] = &[
    ("width", "outline-focus", "outline-width"),
    ("offset", "outline-offset-focus", "outline-offset"),
];

/// The named utilities for what Tailwind has no theme variable for.
fn utilities(spec: &Spec, out: &mut String) {
    let prefix = &spec.meta.prefix;
    let _ = writeln!(
        out,
        "\n/* What Tailwind has no theme variable for, as a utility of its own by the\n\
        \x20  token's name. A duration follows prefers-reduced-motion, as the token\n\
        \x20  does: duration-fast, z-dialog, border-hairline, border-b-hairline,\n\
        \x20  outline-focus, outline-offset-focus. */"
    );
    for name in spec.motion.duration.keys() {
        // `--tw-duration` is what Tailwind's own `transition` reads.
        let _ = writeln!(
            out,
            "@utility duration-{name} {{\n\
            \x20 --tw-duration: var(--{prefix}-duration-{name});\n\
            \x20 transition-duration: var(--{prefix}-duration-{name});\n\
            }}"
        );
    }
    for name in spec.layer.keys() {
        let _ = writeln!(
            out,
            "@utility z-{name} {{\n\x20 z-index: var(--{prefix}-layer-{name});\n}}"
        );
    }
    for name in spec.border.keys() {
        for (side, properties) in BORDER_SIDES {
            let _ = writeln!(out, "@utility border{side}-{name} {{");
            for property in *properties {
                // The style Tailwind's own border utilities keep, and a line
                // where none of them has declared it.
                let _ = writeln!(
                    out,
                    "  {property}-style: var(--tw-border-style, solid);\n\
                    \x20 {property}-width: var(--{prefix}-border-{name});"
                );
            }
            out.push_str("}\n");
        }
    }
    for (name, utility, property) in FOCUS {
        if spec.focus.contains_key(*name) {
            let _ = writeln!(
                out,
                "@utility {utility} {{\n\x20 {property}: var(--{prefix}-focus-{name});\n}}"
            );
        }
    }
}

/// The header of the design tokens' half, and the theme block opened with the
/// multiplier taken out.
fn open_theme(prefix: &str, out: &mut String) {
    let _ = writeln!(
        out,
        "{SEAM}\n\
        \x20* Source: spec/tokens.toml, spec/fonts.toml\n\
        \x20* Regenerate: cargo xtask build\n\
        \x20*\n\
        \x20* Edits to this file will be overwritten. Change the spec instead.\n\
        \x20*/\n\
        \n\
        /* Every size, radius and type utility, pointing at the --{prefix}-* token\n\
        \x20  behind it. dist/tokens.css and dist/fonts.css declare the tokens, so a\n\
        \x20  page links those two and imports this file after Tailwind.\n\
        \x20\n\
        \x20  A step of the spacing scale is a name here (gap-4, p-inset, min-w-0),\n\
        \x20  and the bare --spacing multiplier is taken out of the theme: a number\n\
        \x20  that is no step of the scale (gap-5) has no utility, whether Tailwind's\n\
        \x20  own theme.css was imported before this file or not. The rest of that\n\
        \x20  theme is untouched: import tailwindcss/utilities.css alone to have no\n\
        \x20  type size, radius or colour but the tokens'. */\n\
        @theme inline {{"
    );

    let _ = writeln!(
        out,
        "  /* No multiplier: `initial` removes the one Tailwind's theme.css declares. */\n\
        \x20 --spacing: initial;\n"
    );
}

/// The design tokens' half: one `@theme inline` block, and the utilities
/// Tailwind has no theme variable for.
fn tokens(spec: &Spec, fonts: &Fonts) -> String {
    let prefix = &spec.meta.prefix;
    let mut out = String::new();
    open_theme(prefix, &mut out);

    let _ = writeln!(out, "  /* Spacing: the steps, then the named purposes. */");
    for step in &spec.space.steps {
        let _ = writeln!(out, "  --spacing-{step}: var(--{prefix}-space-{step});");
    }
    for name in spec.space.named.keys() {
        let _ = writeln!(out, "  --spacing-{name}: var(--{prefix}-space-{name});");
    }

    let _ = writeln!(
        out,
        "\n  /* The shell's lengths and the smallest target, as sizes: h-shell-line,\n\
        \x20    w-shell-rail, min-h-target-min. */"
    );
    for (name, value) in &spec.shell {
        if is_length(value) {
            let _ = writeln!(
                out,
                "  --spacing-shell-{name}: var(--{prefix}-shell-{name});"
            );
        }
    }
    for name in spec.target.keys() {
        let _ = writeln!(
            out,
            "  --spacing-target-{name}: var(--{prefix}-target-{name});"
        );
    }

    let _ = writeln!(out, "\n  /* Radii: rounded-control. */");
    for name in spec.radius.keys() {
        let _ = writeln!(out, "  --radius-{name}: var(--{prefix}-radius-{name});");
    }

    let _ = writeln!(out, "\n  /* Font roles: font-ui, font-mono. */");
    for name in fonts.stack.keys() {
        let _ = writeln!(out, "  --font-{name}: var(--{prefix}-font-{name});");
    }

    let _ = writeln!(out, "\n  /* Type sizes: text-body, text-caption. */");
    for name in spec.r#type.size.keys() {
        let _ = writeln!(out, "  --text-{name}: var(--{prefix}-size-{name});");
    }

    let _ = writeln!(out, "\n  /* Line heights: leading-body. */");
    for name in spec.r#type.leading.keys() {
        let _ = writeln!(out, "  --leading-{name}: var(--{prefix}-leading-{name});");
    }

    let _ = writeln!(out, "\n  /* Weights: font-semibold. */");
    for name in spec.r#type.weight.keys() {
        let _ = writeln!(
            out,
            "  --font-weight-{name}: var(--{prefix}-weight-{name});"
        );
    }

    let _ = writeln!(out, "\n  /* Letter spacing: tracking-caps. */");
    for name in spec.r#type.tracking.keys() {
        let _ = writeln!(out, "  --tracking-{name}: var(--{prefix}-tracking-{name});");
    }

    if spec.r#type.measure.is_some() || spec.r#type.measure_lead.is_some() {
        let _ = writeln!(out, "\n  /* The prose line-length cap: max-w-measure. */");
        if spec.r#type.measure.is_some() {
            let _ = writeln!(out, "  --container-measure: var(--{prefix}-measure);");
        }
        if spec.r#type.measure_lead.is_some() {
            let _ = writeln!(
                out,
                "  --container-measure-lead: var(--{prefix}-measure-lead);"
            );
        }
    }

    if !spec.motion.easing.is_empty() {
        let _ = writeln!(out, "\n  /* Easings: ease-out, ease-in-out. */");
        for name in spec.motion.easing.keys() {
            let name = name.replace('_', "-");
            let _ = writeln!(out, "  --ease-{name}: var(--{prefix}-easing-{name});");
        }
    }

    if !spec.breakpoint.is_empty() {
        let _ = writeln!(
            out,
            "\n  /* Breakpoints, as the lengths the spec declares: a media query cannot\n\
            \x20    read a custom property. Minimum widths, as everywhere here. */"
        );
        for (name, value) in &spec.breakpoint {
            let _ = writeln!(out, "  --breakpoint-{name}: {value};");
        }
    }

    out.push_str("}\n");
    utilities(spec, &mut out);
    out
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    fn spec() -> Spec {
        noctua_design_spec::load(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../spec/tokens.toml"
        ))
        .expect("the shipped tokens")
    }

    fn fonts() -> Fonts {
        noctua_design_spec::load_fonts(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../spec/fonts.toml"
        ))
        .expect("the shipped fonts")
    }

    /// Every custom property a sheet declares, by name.
    fn declared(sheet: &str) -> BTreeSet<String> {
        sheet
            .lines()
            .filter_map(|line| {
                let line = line.trim_start();
                let (name, _) = line.split_once(':')?;
                name.starts_with("--").then(|| name.to_owned())
            })
            .collect()
    }

    /// Every design token the block points a utility at, by name. Tailwind's
    /// own variables (`--tw-border-style`) are its to declare.
    fn referenced(block: &str) -> BTreeSet<String> {
        block
            .split("var(")
            .skip(1)
            .filter_map(|rest| rest.split([')', ',']).next())
            .map(|name| name.trim().to_owned())
            .filter(|name| name.starts_with("--noctua-"))
            .collect()
    }

    /// The token sheet and the font sheet, as a consumer links them.
    fn sheets(spec: &Spec, fonts: &Fonts) -> (String, String) {
        let tokens = crate::css::emit(spec)
            .into_iter()
            .find(|file| file.path == "dist/tokens.css")
            .expect("the token sheet")
            .contents;
        let faces = crate::fonts::emit(spec, fonts, &[])
            .into_iter()
            .find(|file| file.path == "dist/fonts.css")
            .expect("the font sheet")
            .contents;
        (tokens, faces)
    }

    /// What the bridge reaches with no utility, each by name and for a reason.
    /// A name ending in `-` is a whole group. Everything else a shipped sheet
    /// declares is owed a utility, so a token in a group nobody has listed
    /// here fails the build until somebody decides.
    const EXCUSED: &[(&str, &str)] = &[
        (
            "--noctua-space-base",
            "the scale's base, which as a utility would be Tailwind's multiplier",
        ),
        (
            "--noctua-shell-prompt-max",
            "a count of lines with no unit, and no size",
        ),
        (
            "--noctua-shell-drawer-share-max",
            "a drawer's largest share as a number with no unit, and no size",
        ),
        (
            "--noctua-breakpoint-",
            "a media query cannot read a custom property, so each is written as its length",
        ),
        (
            "--noctua-scene-",
            "what a Lineage cell is worth in three dimensions, which no box is laid out by",
        ),
    ];

    fn excused(name: &str) -> bool {
        EXCUSED.iter().any(|(entry, _)| {
            if entry.ends_with('-') {
                name.starts_with(entry)
            } else {
                name == *entry
            }
        })
    }

    /// Every token the shipped sheets declare has a utility or is excused by
    /// name, read from the sheets and not from the loops above.
    ///
    /// The first consumer found a bridge that mapped colours and nothing else:
    /// `gap-*`, `p-*` and `min-w-0` emitted no rule, so its layout was drawn
    /// with no gap at all. A token added to the spec and forgotten here is the
    /// same defect one token wide. A review then found the motion, layer,
    /// border and focus tokens with no utility and no test asking for one,
    /// because this test asked only about the groups it listed: so it lists
    /// what is excused instead, and asks about everything else.
    #[test]
    fn every_token_has_a_utility_or_is_excused() {
        let (spec, fonts) = (spec(), fonts());
        let (tokens, faces) = sheets(&spec, &fonts);
        let block = super::tokens(&spec, &fonts);
        let referenced = referenced(&block);
        let declared: BTreeSet<String> = declared(&tokens)
            .union(&declared(&faces))
            .filter(|name| name.starts_with("--noctua-"))
            .cloned()
            .collect();

        let mut owed = 0usize;
        for name in &declared {
            if excused(name) {
                continue;
            }
            owed += 1;
            assert!(
                referenced.contains(name),
                "{name} is declared by a shipped sheet, and the Tailwind bridge neither maps \
                 a utility onto it nor excuses it"
            );
        }
        assert!(owed > 60, "only {owed} tokens were asked about");
        for (entry, reason) in EXCUSED {
            assert_ne!(*reason, "");
            assert!(
                declared.iter().any(|name| excused(name)
                    && (name == entry || (entry.ends_with('-') && name.starts_with(entry)))),
                "{entry} is excused and no sheet declares it"
            );
        }
        // One of each group that had none.
        for name in [
            "--noctua-easing-in-out",
            "--noctua-duration-fast",
            "--noctua-layer-dialog",
            "--noctua-border-hairline",
            "--noctua-focus-width",
            "--noctua-focus-offset",
        ] {
            assert!(declared.contains(name), "{name} is no token any more");
            assert!(referenced.contains(name), "{name} has no utility");
        }
    }

    /// Every token the block names is one `tokens.css` or `fonts.css`
    /// declares: a `var()` naming anything else is dropped with no error and
    /// the utility draws nothing.
    #[test]
    fn the_block_names_only_tokens_the_sheets_declare() {
        let (spec, fonts) = (spec(), fonts());
        let (tokens, faces) = sheets(&spec, &fonts);
        let known: BTreeSet<String> = declared(&tokens)
            .union(&declared(&faces))
            .cloned()
            .collect();
        let block = super::tokens(&spec, &fonts);
        let referenced = referenced(&block);
        assert!(!referenced.is_empty());
        for name in referenced {
            assert!(
                known.contains(&name),
                "the bridge points at {name}, which neither sheet declares"
            );
        }
    }

    /// The names Tailwind v4 reads, spelt as it reads them, for one token of
    /// each group. A utility is found by its theme variable's name, so a
    /// namespace misspelt maps onto nothing and nothing says so.
    #[test]
    fn the_theme_variables_are_in_tailwinds_namespaces() {
        let block = super::tokens(&spec(), &fonts());
        for line in [
            "  --spacing-0: var(--noctua-space-0);",
            "  --spacing-4: var(--noctua-space-4);",
            "  --spacing-gap: var(--noctua-space-gap);",
            "  --spacing-shell-line: var(--noctua-shell-line);",
            "  --spacing-shell-rail: var(--noctua-shell-rail);",
            "  --spacing-target-min: var(--noctua-target-min);",
            "  --radius-control: var(--noctua-radius-control);",
            "  --font-ui: var(--noctua-font-ui);",
            "  --font-mono: var(--noctua-font-mono);",
            "  --text-body: var(--noctua-size-body);",
            "  --leading-body: var(--noctua-leading-body);",
            "  --font-weight-semibold: var(--noctua-weight-semibold);",
            "  --tracking-caps: var(--noctua-tracking-caps);",
            "  --container-measure: var(--noctua-measure);",
            "  --ease-out: var(--noctua-easing-out);",
            "  --ease-in-out: var(--noctua-easing-in-out);",
            "  --ease-linear: var(--noctua-easing-linear);",
            "  --breakpoint-sm: 30rem;",
        ] {
            assert!(block.contains(line), "the bridge lacks `{}`", line.trim());
        }
        assert!(block.contains("@theme inline {"));
    }

    /// What Tailwind has no theme variable for is a utility by the token's
    /// name, written after the theme block and never inside it.
    #[test]
    fn what_tailwind_has_no_namespace_for_is_a_named_utility() {
        let block = super::tokens(&spec(), &fonts());
        let (theme, after) = block
            .split_once("\n}\n")
            .expect("the end of the theme block");
        assert!(!theme.contains("@utility"));
        for rule in [
            "@utility duration-fast {\n  --tw-duration: var(--noctua-duration-fast);\n  \
             transition-duration: var(--noctua-duration-fast);\n}\n",
            "@utility z-dialog {\n  z-index: var(--noctua-layer-dialog);\n}\n",
            "@utility border-hairline {\n  border-style: var(--tw-border-style, solid);\n  \
             border-width: var(--noctua-border-hairline);\n}\n",
            "@utility border-b-strong {\n  border-bottom-style: var(--tw-border-style, solid);\n  \
             border-bottom-width: var(--noctua-border-strong);\n}\n",
            "@utility border-s-rule {\n  border-inline-start-style: var(--tw-border-style, solid);\n  \
             border-inline-start-width: var(--noctua-border-rule);\n}\n",
            "@utility outline-focus {\n  outline-width: var(--noctua-focus-width);\n}\n",
            "@utility outline-offset-focus {\n  outline-offset: var(--noctua-focus-offset);\n}\n",
        ] {
            assert!(after.contains(rule), "the bridge lacks:\n{rule}");
        }
        // And no length, count or time of its own: every declaration is a token.
        for line in after.lines().filter(|line| line.ends_with(';')) {
            assert!(
                line.contains("var(--noctua-") || line.contains("var(--tw-border-style, solid)"),
                "`{}` is no token",
                line.trim()
            );
        }
    }

    /// No bare multiplier: with one, `gap-5` would be a utility and five is
    /// no step of the scale. It is taken out and not merely left out, because
    /// `@import "tailwindcss"` declares one before this file is read, and
    /// `initial` in a theme block is how Tailwind is told to forget it.
    #[test]
    fn no_bare_spacing_multiplier_is_declared() {
        let block = super::tokens(&spec(), &fonts());
        let multipliers: Vec<&str> = block
            .lines()
            .map(str::trim)
            .filter(|line| line.starts_with("--spacing:"))
            .collect();
        assert_eq!(multipliers, ["--spacing: initial;"]);
        let theme = block
            .split_once("@theme inline {")
            .expect("the theme block")
            .1;
        let theme = theme.split_once("\n}\n").expect("its end").0;
        assert!(
            theme.contains("--spacing: initial;"),
            "the multiplier is taken out outside the theme block, where it is a plain \
             custom property and Tailwind does not read it"
        );
        assert!(!block.contains("--spacing-5:"));
    }

    /// A breakpoint is written as the spec's own length, so it cannot differ
    /// from the one the component rules are gated to.
    #[test]
    fn a_breakpoint_is_the_specs_length() {
        let spec = spec();
        let block = super::tokens(&spec, &fonts());
        assert!(!spec.breakpoint.is_empty());
        for (name, value) in &spec.breakpoint {
            assert!(block.contains(&format!("  --breakpoint-{name}: {value};\n")));
        }
    }

    /// The vendor's bytes come first and whole, and the design tokens' block
    /// follows them.
    #[test]
    fn the_vendors_mapping_is_kept_byte_for_byte() {
        let vendor = "/* the vendor's */\n@theme inline {\n  --color-fg: var(--nc-color-fg);\n}\n";
        let file = bridge(vendor, &spec(), &fonts());
        assert_eq!(file.path, BRIDGE);
        let (before, after) = file
            .contents
            .split_once(SEAM)
            .expect("the seam between the two halves");
        assert_eq!(before, format!("{vendor}\n"));
        assert!(after.contains("--spacing-4: var(--noctua-space-4);"));
    }
}
