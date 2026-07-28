//! The generated `packages/tokens` module.
//!
//! The React half of the design system, and anything else in the JavaScript
//! ecosystem that wants the tokens without parsing CSS.
//!
//! Emitted as `.js` plus a hand-shaped `.d.ts` rather than as `.ts`, for the
//! same reason `noctua-colors` does it: a consumer should not need a
//! TypeScript toolchain to use a data module, and a `.d.ts` gives full types to
//! the ones that have one.
//!
//! `as const` throughout, so a consumer's autocomplete offers the literal token
//! names rather than `string`.

use std::fmt::Write as _;

use noctua_design_spec::Spec;

use crate::{CommentStyle, EmittedFile, camel, header};

/// Emits the JavaScript module and its type declarations.
#[must_use]
pub fn emit(spec: &Spec) -> Vec<EmittedFile> {
    vec![
        EmittedFile::new("packages/tokens/index.js", index_js(spec)),
        EmittedFile::new("packages/tokens/index.d.ts", index_d_ts(spec)),
    ]
}

fn index_js(spec: &Spec) -> String {
    let prefix = &spec.meta.prefix;
    let mut out = header(CommentStyle::Line("//"));

    writeln!(
        out,
        "\n// Every non-colour token. Colours are not here — they come from\n\
         // noctua-colors and are vendored under packages/tokens/vendor/.\n"
    )
    .expect("write");

    writeln!(out, "export const prefix = {};\n", json(prefix)).expect("write");

    // Spacing carries both halves: the numeric scale, for code that computes,
    // and the `var()` references, for code that styles.
    writeln!(out, "export const space = {{").expect("write");
    writeln!(out, "  base: {},", json(&spec.space.base)).expect("write");
    writeln!(out, "  steps: {:?},", spec.space.steps).expect("write");
    for (name, _) in &spec.space.named {
        writeln!(out, "  {}: \"var(--{prefix}-space-{name})\",", camel(name)).expect("write");
    }
    writeln!(out, "}};\n").expect("write");

    var_object(&mut out, "radius", &spec.radius, prefix, "radius");
    var_object(&mut out, "size", &spec.r#type.size, prefix, "size");
    var_object(
        &mut out,
        "tracking",
        &spec.r#type.tracking,
        prefix,
        "tracking",
    );
    var_object(&mut out, "focus", &spec.focus, prefix, "focus");

    writeln!(out, "export const leading = {{").expect("write");
    for (name, value) in &spec.r#type.leading {
        writeln!(out, "  {}: {value},", camel(name)).expect("write");
    }
    writeln!(out, "}};\n").expect("write");

    writeln!(out, "export const weight = {{").expect("write");
    for (name, value) in &spec.r#type.weight {
        writeln!(out, "  {}: {value},", camel(name)).expect("write");
    }
    writeln!(out, "}};\n").expect("write");

    writeln!(out, "export const duration = {{").expect("write");
    for (name, _) in &spec.motion.duration {
        writeln!(
            out,
            "  {}: \"var(--{prefix}-duration-{name})\",",
            camel(name)
        )
        .expect("write");
    }
    writeln!(out, "}};\n").expect("write");

    writeln!(out, "export const easing = {{").expect("write");
    for name in spec.motion.easing.keys() {
        let css = name.replace('_', "-");
        writeln!(out, "  {}: \"var(--{prefix}-easing-{css})\",", camel(name)).expect("write");
    }
    writeln!(out, "}};\n").expect("write");

    writeln!(out, "export const layer = {{").expect("write");
    for (name, value) in &spec.layer {
        writeln!(out, "  {}: {value},", camel(name)).expect("write");
    }
    writeln!(out, "}};\n").expect("write");

    // Values, not `var()`: a media query cannot read a custom property, so a
    // caller matching on a breakpoint needs the length itself.
    writeln!(
        out,
        "// Values rather than var() references: a media query cannot read a\n\
         // custom property, so anything branching on a breakpoint needs the length."
    )
    .expect("write");
    writeln!(out, "export const breakpoint = {{").expect("write");
    for (name, value) in &spec.breakpoint {
        writeln!(out, "  {}: {},", camel(name), json(value)).expect("write");
    }
    writeln!(out, "}};\n").expect("write");

    if spec.r#type.measure.is_some() || spec.r#type.measure_lead.is_some() {
        writeln!(out, "export const measure = {{").expect("write");
        if let Some(ch) = spec.r#type.measure {
            writeln!(out, "  body: {ch},").expect("write");
        }
        if let Some(ch) = spec.r#type.measure_lead {
            writeln!(out, "  lead: {ch},").expect("write");
        }
        writeln!(out, "}};").expect("write");
    }

    out
}

fn index_d_ts(spec: &Spec) -> String {
    let mut out = header(CommentStyle::Line("//"));
    writeln!(out).expect("write");

    writeln!(out, "export declare const prefix: string;\n").expect("write");

    writeln!(out, "export declare const space: {{").expect("write");
    writeln!(out, "  readonly base: string;").expect("write");
    writeln!(out, "  readonly steps: readonly number[];").expect("write");
    for name in spec.space.named.keys() {
        writeln!(out, "  readonly {}: string;", camel(name)).expect("write");
    }
    writeln!(out, "}};\n").expect("write");

    declare(&mut out, "radius", spec.radius.keys(), "string");
    declare(&mut out, "size", spec.r#type.size.keys(), "string");
    declare(&mut out, "tracking", spec.r#type.tracking.keys(), "string");
    declare(&mut out, "focus", spec.focus.keys(), "string");
    declare(&mut out, "leading", spec.r#type.leading.keys(), "number");
    declare(&mut out, "weight", spec.r#type.weight.keys(), "number");
    declare(&mut out, "duration", spec.motion.duration.keys(), "string");
    declare(&mut out, "easing", spec.motion.easing.keys(), "string");
    declare(&mut out, "layer", spec.layer.keys(), "number");
    declare(&mut out, "breakpoint", spec.breakpoint.keys(), "string");

    if spec.r#type.measure.is_some() || spec.r#type.measure_lead.is_some() {
        writeln!(out, "export declare const measure: {{").expect("write");
        if spec.r#type.measure.is_some() {
            writeln!(out, "  readonly body: number;").expect("write");
        }
        if spec.r#type.measure_lead.is_some() {
            writeln!(out, "  readonly lead: number;").expect("write");
        }
        writeln!(out, "}};").expect("write");
    }

    out
}

fn var_object(
    out: &mut String,
    name: &str,
    values: &indexmap::IndexMap<String, String>,
    prefix: &str,
    css_group: &str,
) {
    writeln!(out, "export const {name} = {{").expect("write");
    for key in values.keys() {
        writeln!(
            out,
            "  {}: \"var(--{prefix}-{css_group}-{key})\",",
            camel(key)
        )
        .expect("write");
    }
    writeln!(out, "}};\n").expect("write");
}

fn declare<'a>(out: &mut String, name: &str, keys: impl Iterator<Item = &'a String>, ty: &str) {
    writeln!(out, "export declare const {name}: {{").expect("write");
    for key in keys {
        writeln!(out, "  readonly {}: {ty};", camel(key)).expect("write");
    }
    writeln!(out, "}};\n").expect("write");
}

/// Quotes a string as JSON, so a value containing a quote cannot break out.
///
/// Every value here comes from a validated spec and none contains a quote
/// today, which is exactly why this is worth doing now: the day one does, the
/// failure would be a syntactically broken module rather than an error.
fn json(value: &str) -> String {
    let mut out = String::with_capacity(value.len() + 2);
    out.push('"');
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            _ => out.push(ch),
        }
    }
    out.push('"');
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shipped() -> Spec {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../spec/tokens.toml");
        noctua_design_spec::load(path).expect("the shipped spec")
    }

    fn files() -> (String, String) {
        let mut emitted = emit(&shipped());
        let js = emitted.remove(0).contents;
        let dts = emitted.remove(0).contents;
        (js, dts)
    }

    #[test]
    fn it_emits_a_module_and_its_types() {
        let emitted = emit(&shipped());
        assert_eq!(emitted[0].path, "packages/tokens/index.js");
        assert_eq!(emitted[1].path, "packages/tokens/index.d.ts");
    }

    /// Same rule as the Rust target: reference the property, do not freeze the
    /// value, or an override moves the CSS and not the module.
    #[test]
    fn tokens_reference_the_custom_property() {
        let (js, _) = files();
        assert!(js.contains(r#"gap: "var(--noctua-space-gap)","#), "{js}");
    }

    #[test]
    fn breakpoints_carry_their_value() {
        let (js, _) = files();
        assert!(js.contains(r#"md: "48rem","#), "{js}");
    }

    #[test]
    fn hyphenated_names_become_camel_case() {
        let (js, dts) = files();
        assert!(js.contains("inOut:"), "{js}");
        assert!(dts.contains("readonly inOut:"), "{dts}");
    }

    /// Every export in the module must be declared, or a TypeScript consumer
    /// gets `any` for it and the types are decoration.
    #[test]
    fn every_export_is_declared() {
        let (js, dts) = files();
        for line in js.lines() {
            if let Some(rest) = line.strip_prefix("export const ")
                && let Some(name) = rest.split_whitespace().next()
            {
                assert!(
                    dts.contains(&format!("export declare const {name}")),
                    "{name} is exported but not declared"
                );
            }
        }
    }

    /// The module is data. A quote in a value would otherwise break it.
    #[test]
    fn values_are_json_quoted() {
        assert_eq!(json(r#"a"b"#), r#""a\"b""#);
        assert_eq!(json("plain"), r#""plain""#);
    }
}
