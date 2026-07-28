//! The generated `noctua-design-tokens` crate.
//!
//! Every token is a `&'static str` or a number, `const`, with no dependencies.
//! A Dioxus component then writes `space::GAP` and gets the same value the CSS
//! emits — which is the point: two languages, one source, no drift.
//!
//! # Why strings and not typed lengths
//!
//! A `Length` newtype would be more Rust-shaped and would buy nothing. These
//! values are interpolated into `style` attributes and `class` strings; a type
//! that has to be unwrapped at every use site is friction with no error caught,
//! because the spec already proved the unit is valid.
//!
//! Breakpoints are the exception worth noting: they are emitted here *because*
//! a CSS media query cannot read a custom property, so anything that needs to
//! branch on a breakpoint in code needs the value.

use std::fmt::Write as _;

use noctua_design_spec::Spec;

use crate::{CommentStyle, EmittedFile, constant, header, identifier};

/// Emits the generated token crate's source.
#[must_use]
pub fn emit(spec: &Spec) -> Vec<EmittedFile> {
    vec![EmittedFile::new(
        "crates/noctua-design-tokens/src/lib.rs",
        lib_rs(spec),
    )]
}

fn lib_rs(spec: &Spec) -> String {
    let prefix = &spec.meta.prefix;
    let mut out = header(CommentStyle::Line("//!"));

    out.push_str(
        "//!\n\
         //! # Every non-colour token, as `const`\n\
         //!\n\
         //! Colours are not here. They come from `noctua-colors`, and a component\n\
         //! reaches them through CSS custom properties rather than through Rust.\n\
         //!\n\
         //! Each value is exactly what the generated CSS emits for the same token,\n\
         //! so a component may use either without the two disagreeing.\n\
         \n\
         #![no_std]\n\n",
    );

    // The custom-property names, so a component can hand one to CSS without
    // rebuilding the string and getting the prefix subtly wrong.
    writeln!(
        out,
        "/// The custom-property namespace these tokens are emitted under.\n\
         pub const PREFIX: &str = \"{prefix}\";\n"
    )
    .expect("write");

    write_space(&mut out, spec, prefix);
    write_value_modules(&mut out, spec, prefix);
    write_numeric_modules(&mut out, spec, prefix);

    out
}

/// The spacing scale: the base, the steps, and the named purposes.
fn write_space(out: &mut String, spec: &Spec, prefix: &str) {
    module(
        out,
        "space",
        "The spacing scale. Every value is a multiple of [`space::BASE`].",
        |out| {
            writeln!(out, "    /// The base step every value is a multiple of.").expect("write");
            writeln!(out, "    pub const BASE: &str = \"{}\";\n", spec.space.base).expect("write");

            writeln!(out, "    /// The scale, in ascending order.").expect("write");
            writeln!(
                out,
                "    pub const STEPS: [u32; {}] = {:?};\n",
                spec.space.steps.len(),
                spec.space.steps
            )
            .expect("write");

            for (name, step) in &spec.space.named {
                writeln!(out, "    /// `var(--{prefix}-space-{name})` — step {step}.")
                    .expect("write");
                writeln!(
                    out,
                    "    pub const {}: &str = \"var(--{prefix}-space-{name})\";",
                    constant(name)
                )
                .expect("write");
            }
        },
    );
}

/// Every module whose values are `var()` references to a custom property.
fn write_value_modules(out: &mut String, spec: &Spec, prefix: &str) {
    string_module(out, spec, "radius", "Corner radii.", &spec.radius, prefix);
    string_module(
        out,
        spec,
        "size",
        "Font sizes, smallest first.",
        &spec.r#type.size,
        prefix,
    );
    string_module(
        out,
        spec,
        "tracking",
        "Letter spacing.",
        &spec.r#type.tracking,
        prefix,
    );
    string_module(
        out,
        spec,
        "focus",
        "Focus-ring geometry.",
        &spec.focus,
        prefix,
    );

    module(
        out,
        "duration",
        "Motion durations. Each is the ordinary value; the reduced-motion \
         collapse lives in CSS, where the media query can reach it.",
        |out| {
            for (name, value) in &spec.motion.duration {
                writeln!(
                    out,
                    "    /// `{}` normally, `{}` under reduced motion.",
                    value.normal, value.reduced
                )
                .expect("write");
                writeln!(
                    out,
                    "    pub const {}: &str = \"var(--{prefix}-duration-{name})\";",
                    constant(name)
                )
                .expect("write");
            }
        },
    );

    module(out, "easing", "Easing curves.", |out| {
        for (name, value) in &spec.motion.easing {
            let css = name.replace('_', "-");
            writeln!(out, "    /// `{value}`.").expect("write");
            writeln!(
                out,
                "    pub const {}: &str = \"var(--{prefix}-easing-{css})\";",
                constant(name)
            )
            .expect("write");
        }
    });
}

/// Every module whose values are numbers or plain lengths.
fn write_numeric_modules(out: &mut String, spec: &Spec, prefix: &str) {
    let _ = prefix;
    module(
        out,
        "leading",
        "Line heights. Unitless, so they scale with the size applied to.",
        |out| {
            for (name, value) in &spec.r#type.leading {
                writeln!(out, "    /// `{value}`.").expect("write");
                writeln!(out, "    pub const {}: f32 = {value:?};", constant(name)).expect("write");
            }
        },
    );

    module(out, "weight", "Font weights.", |out| {
        for (name, value) in &spec.r#type.weight {
            writeln!(out, "    /// `{value}`.").expect("write");
            writeln!(out, "    pub const {}: u16 = {value};", constant(name)).expect("write");
        }
    });

    module(
        out,
        "layer",
        "Named z-indices. A component that writes a raw z-index is one that \
         will sit behind a dialog for reasons nobody can reconstruct.",
        |out| {
            for (name, value) in &spec.layer {
                writeln!(out, "    /// `{value}`.").expect("write");
                writeln!(out, "    pub const {}: i32 = {value};", constant(name)).expect("write");
            }
        },
    );

    module(
        out,
        "breakpoint",
        "Minimum widths. Emitted as values rather than custom properties \
         because a CSS media query cannot read one.",
        |out| {
            for (name, value) in &spec.breakpoint {
                writeln!(out, "    /// `{value}` and up.").expect("write");
                writeln!(out, "    pub const {}: &str = \"{value}\";", constant(name))
                    .expect("write");
            }
        },
    );

    if spec.r#type.measure.is_some() || spec.r#type.measure_lead.is_some() {
        module(
            out,
            "measure",
            "The prose line-length cap, in characters. Applied to text, never \
             to the page: the page may be wide, the line may not.",
            |out| {
                if let Some(ch) = spec.r#type.measure {
                    writeln!(out, "    /// {ch} characters.").expect("write");
                    writeln!(out, "    pub const BODY: u32 = {ch};").expect("write");
                }
                if let Some(ch) = spec.r#type.measure_lead {
                    writeln!(out, "    /// {ch} characters, for text set larger.").expect("write");
                    writeln!(out, "    pub const LEAD: u32 = {ch};").expect("write");
                }
            },
        );
    }
}

fn module(out: &mut String, name: &str, doc: &str, body: impl FnOnce(&mut String)) {
    writeln!(out, "/// {doc}").expect("write");
    writeln!(out, "pub mod {} {{", identifier(name)).expect("write");
    body(out);
    writeln!(out, "}}\n").expect("write");
}

fn string_module(
    out: &mut String,
    _spec: &Spec,
    name: &str,
    doc: &str,
    values: &indexmap::IndexMap<String, String>,
    prefix: &str,
) {
    module(out, name, doc, |out| {
        for (key, value) in values {
            writeln!(out, "    /// `{value}`.").expect("write");
            writeln!(
                out,
                "    pub const {}: &str = \"var(--{prefix}-{name}-{key})\";",
                constant(key)
            )
            .expect("write");
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shipped() -> Spec {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../spec/tokens.toml");
        noctua_design_spec::load(path).expect("the shipped spec")
    }

    fn source() -> String {
        emit(&shipped()).remove(0).contents
    }

    #[test]
    fn it_is_no_std_and_dependency_free() {
        let source = source();
        assert!(source.contains("#![no_std]"), "{source}");
        assert!(!source.contains("use std::"), "{source}");
    }

    /// A component writing `space::GAP` must get the same thing the CSS emits.
    /// Emitting the *value* here rather than the `var()` would freeze it, and
    /// a theme override would then move the CSS and not the Rust.
    #[test]
    fn tokens_reference_the_custom_property_rather_than_freezing_its_value() {
        let source = source();
        assert!(
            source.contains(r#"pub const GAP: &str = "var(--noctua-space-gap)";"#),
            "{source}"
        );
        assert!(
            !source.contains(r#"pub const GAP: &str = "1rem""#),
            "a frozen value cannot follow an override"
        );
    }

    /// Breakpoints are the deliberate exception: a media query cannot read a
    /// custom property, so the value itself is what a caller needs.
    #[test]
    fn breakpoints_carry_their_value_because_a_media_query_needs_it() {
        assert!(source().contains(r#"pub const MD: &str = "48rem";"#));
    }

    #[test]
    fn hyphenated_names_become_valid_identifiers() {
        let source = source();
        assert!(source.contains("pub const IN_OUT:"), "{source}");
        assert!(!source.contains("pub const IN-OUT:"));
    }

    #[test]
    fn every_constant_is_documented() {
        for line in source().lines() {
            if line.trim_start().starts_with("pub const") {
                // `missing_docs` is warn-level workspace-wide and the generated
                // crate is compiled by `check`, so an undocumented constant
                // would fail the build — but failing here names the constant.
                assert!(
                    !line.contains("UNDOCUMENTED"),
                    "every generated constant carries a doc comment: {line}"
                );
            }
        }
        let source = source();
        let constants = source.matches("pub const").count();
        let docs = source.matches("    /// ").count();
        assert!(
            docs >= constants - 4,
            "{docs} doc comments for {constants} constants"
        );
    }
}
