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
//! shell's lengths, the smallest target, the measure and the breakpoints, each
//! as the Tailwind v4 theme variable its utilities read.
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

/// The design tokens' half: one `@theme inline` block.
fn tokens(spec: &Spec, fonts: &Fonts) -> String {
    let prefix = &spec.meta.prefix;
    let mut out = String::new();
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
        \x20  and no bare --spacing multiplier is declared: a number that is no step\n\
        \x20  of the scale has no utility. */\n\
        @theme inline {{"
    );

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

    /// Every token the block points a utility at, by name.
    fn referenced(block: &str) -> BTreeSet<String> {
        block
            .split("var(")
            .skip(1)
            .filter_map(|rest| rest.split(')').next())
            .map(|name| name.trim().to_owned())
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

    /// The groups the bridge owes a utility for, and the three tokens in them
    /// that are no size: the scale's base, which would be Tailwind's
    /// multiplier, the prompt's count of lines, and a drawer's largest share
    /// as a number.
    const OWED: &[&str] = &[
        "--noctua-space-",
        "--noctua-radius-",
        "--noctua-size-",
        "--noctua-leading-",
        "--noctua-weight-",
        "--noctua-tracking-",
        "--noctua-measure",
        "--noctua-shell-",
        "--noctua-target-",
        "--noctua-font-",
    ];
    const NOT_A_SIZE: &[&str] = &[
        "--noctua-space-base",
        "--noctua-shell-prompt-max",
        "--noctua-shell-drawer-share-max",
    ];

    /// Every spacing, radius, type, font, shell and target token the shipped
    /// sheets declare has a utility, read from the sheets and not from the
    /// loops above.
    ///
    /// The first consumer found a bridge that mapped colours and nothing else:
    /// `gap-*`, `p-*` and `min-w-0` emitted no rule, so its layout was drawn
    /// with no gap at all. A token added to the spec and forgotten here is the
    /// same defect one token wide.
    #[test]
    fn every_size_and_type_token_has_a_utility() {
        let (spec, fonts) = (spec(), fonts());
        let (tokens, faces) = sheets(&spec, &fonts);
        let block = super::tokens(&spec, &fonts);
        let referenced = referenced(&block);

        let mut owed = 0usize;
        for name in declared(&tokens).union(&declared(&faces)) {
            if !OWED.iter().any(|group| name.starts_with(group))
                || NOT_A_SIZE.contains(&name.as_str())
            {
                continue;
            }
            owed += 1;
            assert!(
                referenced.contains(name),
                "{name} is declared by a shipped sheet and the Tailwind bridge maps \
                 no utility onto it"
            );
        }
        assert!(owed > 40, "only {owed} tokens were asked about");
        for name in NOT_A_SIZE {
            assert!(
                declared(&tokens).contains(*name),
                "{name} is excused and no sheet declares it"
            );
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
            "  --breakpoint-sm: 30rem;",
        ] {
            assert!(block.contains(line), "the bridge lacks `{}`", line.trim());
        }
        assert!(block.contains("@theme inline {"));
    }

    /// No bare multiplier: with one, `gap-5` would be a utility and five is
    /// no step of the scale.
    #[test]
    fn no_bare_spacing_multiplier_is_declared() {
        let block = super::tokens(&spec(), &fonts());
        assert!(!block.contains("--spacing:"));
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
