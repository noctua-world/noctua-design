//! CSS custom properties.
//!
//! # The reduced-motion layer is why durations are pairs
//!
//! Every duration token is emitted twice: its ordinary value at `:root`, and
//! its collapse inside `@media (prefers-reduced-motion: reduce)`. A component
//! then writes `var(--noctua-duration-base)` and honours the preference
//! without knowing the preference exists.
//!
//! The alternative — a component checking the media query itself — puts the
//! same conditional in every component and makes forgetting it silent.
//!
//! # `:where(:root)` for nothing here
//!
//! Unlike `noctua-colors`' semantic layer, these are plain `:root` rules.
//! Nothing overrides a design token from a second stylesheet, so there is no
//! specificity tie to break, and `:where()` would only make it harder for a
//! consumer to override one deliberately.

use std::fmt::Write as _;

use noctua_design_spec::Spec;

use crate::{CommentStyle, EmittedFile, header};

/// Emits `dist/tokens.css`.
#[must_use]
pub fn emit(spec: &Spec) -> Vec<EmittedFile> {
    let prefix = &spec.meta.prefix;
    let mut out = header(CommentStyle::Block);

    writeln!(
        out,
        "\n/* Every non-color token, as CSS custom properties.\n\
        \x20\n\
        \x20  Colors are NOT here. They come from noctua-colors as --nc-* and are\n\
        \x20  vendored separately; this file and that one never define the same\n\
        \x20  name. */\n"
    )
    .expect("string write");

    writeln!(out, ":root {{").expect("string write");

    writeln!(
        out,
        "  /* Spacing. Every value is a multiple of the base. */"
    )
    .expect("string write");
    writeln!(out, "  --{prefix}-space-base: {};", spec.space.base).expect("string write");
    for step in &spec.space.steps {
        writeln!(
            out,
            "  --{prefix}-space-{step}: {};",
            multiply(&spec.space.base, *step)
        )
        .expect("string write");
    }

    if !spec.space.named.is_empty() {
        writeln!(
            out,
            "\n  /* Named purposes. A component asks for a purpose, not a number. */"
        )
        .expect("string write");
        for (name, step) in &spec.space.named {
            writeln!(
                out,
                "  --{prefix}-space-{name}: var(--{prefix}-space-{step});"
            )
            .expect("string write");
        }
    }

    write_scales(&mut out, spec, prefix);
    write_motion_and_layers(&mut out, spec, prefix);

    writeln!(out, "}}").expect("string write");

    write_reduced_motion(&mut out, spec, prefix);
    write_breakpoint_reference(&mut out, spec);

    vec![EmittedFile::new("dist/tokens.css", out)]
}

/// Radii, type sizes, line heights, weights, tracking and the measure.
fn write_scales(out: &mut String, spec: &Spec, prefix: &str) {
    section(out, "Radii");
    for (name, value) in &spec.radius {
        writeln!(out, "  --{prefix}-radius-{name}: {value};").expect("string write");
    }

    section(out, "Type sizes");
    for (name, value) in &spec.r#type.size {
        writeln!(out, "  --{prefix}-size-{name}: {value};").expect("string write");
    }

    section(out, "Line heights, unitless so they scale with the size");
    for (name, value) in &spec.r#type.leading {
        writeln!(out, "  --{prefix}-leading-{name}: {value};").expect("string write");
    }

    section(out, "Weights");
    for (name, value) in &spec.r#type.weight {
        writeln!(out, "  --{prefix}-weight-{name}: {value};").expect("string write");
    }

    section(out, "Letter spacing");
    for (name, value) in &spec.r#type.tracking {
        writeln!(out, "  --{prefix}-tracking-{name}: {value};").expect("string write");
    }

    if spec.r#type.measure.is_some() || spec.r#type.measure_lead.is_some() {
        section(
            out,
            "The prose line-length cap. Applied to text, never to the page",
        );
        if let Some(ch) = spec.r#type.measure {
            writeln!(out, "  --{prefix}-measure: {ch}ch;").expect("string write");
        }
        if let Some(ch) = spec.r#type.measure_lead {
            writeln!(out, "  --{prefix}-measure-lead: {ch}ch;").expect("string write");
        }
    }
}

/// Durations, easings, layers and the focus ring.
fn write_motion_and_layers(out: &mut String, spec: &Spec, prefix: &str) {
    section(out, "Motion");
    for (name, duration) in &spec.motion.duration {
        writeln!(out, "  --{prefix}-duration-{name}: {};", duration.normal).expect("string write");
    }
    for (name, value) in &spec.motion.easing {
        writeln!(
            out,
            "  --{prefix}-easing-{}: {value};",
            name.replace('_', "-")
        )
        .expect("string write");
    }

    section(out, "Layers");
    for (name, value) in &spec.layer {
        writeln!(out, "  --{prefix}-layer-{name}: {value};").expect("string write");
    }

    section(out, "Border widths");
    for (name, value) in &spec.border {
        writeln!(out, "  --{prefix}-border-{name}: {value};").expect("string write");
    }

    section(out, "Focus ring geometry");
    for (name, value) in &spec.focus {
        writeln!(out, "  --{prefix}-focus-{name}: {value};").expect("string write");
    }

    section(out, "The scene: what a Lineage cell is worth");
    for (name, value) in &spec.scene {
        writeln!(out, "  --{prefix}-scene-{name}: {value};").expect("string write");
    }

    // BREAKPOINTS AS LENGTHS, WHICH IS NOT THE SAME AS BREAKPOINTS AS QUERIES.
    //
    // A media query cannot read a custom property, and that is why the
    // reference comment at the foot of this file exists and why the values are
    // compiled into the Rust and TypeScript modules. It is not a reason to
    // withhold them from CSS entirely: a viewport rig that resizes a box to
    // `sm` needs the LENGTH, and the alternative is a component adding up
    // spacing steps until they come to 30rem — which is a literal with extra
    // arithmetic, and drifts silently the day a breakpoint moves.
    section(
        out,
        "Breakpoints, as lengths — see the note at the foot of this file",
    );
    for (name, value) in &spec.breakpoint {
        writeln!(out, "  --{prefix}-breakpoint-{name}: {value};").expect("string write");
    }
}

/// The reduced-motion override layer.
fn write_reduced_motion(out: &mut String, spec: &Spec, prefix: &str) {
    // The whole reason durations are pairs. A component writes the token and
    // gets the preference honoured; nothing has to remember to ask.
    if !spec.motion.duration.is_empty() {
        writeln!(
            out,
            "\n/* Reduced motion. Declared once here so that honouring the preference\n\
            \x20  is a property of the token rather than something every component has\n\
            \x20  to remember — and so forgetting it is impossible rather than silent.\n\
            \x20\n\
            \x20  Not every collapse is 0ms: a color or opacity change at zero reads as\n\
            \x20  a glitch, while a movement at zero is exactly what was asked for. */"
        )
        .expect("string write");
        writeln!(out, "@media (prefers-reduced-motion: reduce) {{").expect("string write");
        writeln!(out, "  :root {{").expect("string write");
        for (name, duration) in &spec.motion.duration {
            writeln!(out, "    --{prefix}-duration-{name}: {};", duration.reduced)
                .expect("string write");
        }
        writeln!(out, "  }}").expect("string write");
        writeln!(out, "}}").expect("string write");
    }
}

/// Breakpoints, as a comment.
fn write_breakpoint_reference(out: &mut String, spec: &Spec) {
    // A media query cannot read a custom property, so the values are ALSO
    // compiled into the Rust and TypeScript modules and written out here for a
    // reader. The custom properties above are for the other use — a length.
    if !spec.breakpoint.is_empty() {
        writeln!(
            out,
            "\n/* Breakpoints, for reference. A media query cannot read a custom\n\
            \x20  property, so a QUERY cannot be written from the tokens above — they\n\
            \x20  are compiled into the generated Rust and TypeScript modules instead,\n\
            \x20  and the custom properties exist for the other use, as a length.\n\
            \x20\n\
            \x20  Mobile-first: every one is a MINIMUM width."
        )
        .expect("string write");
        for (name, value) in &spec.breakpoint {
            writeln!(out, "     {name}: {value}").expect("string write");
        }
        writeln!(out, "  */").expect("string write");
    }
}

fn section(out: &mut String, title: &str) {
    writeln!(out, "\n  /* {title}. */").expect("string write");
}

/// Multiplies a length by an integer, keeping its unit.
///
/// The arithmetic is done here rather than emitted as `calc()` for a reason
/// worth stating: `calc()` would be resolved by the browser at use time, so
/// every one of these tokens would carry a runtime computation, and a reader
/// inspecting `--noctua-space-6` in devtools would see an expression rather
/// than a value.
///
/// Falls back to `calc()` only when the base is not a plain number-plus-unit,
/// which the spec's unit check makes unreachable for a valid spec — but a
/// silently wrong multiplication would be worse than an honest expression.
fn multiply(base: &str, factor: u32) -> String {
    if factor == 0 {
        return "0".to_owned();
    }
    if factor == 1 {
        return base.to_owned();
    }

    let base = base.trim();
    let split = base
        .find(|c: char| !c.is_ascii_digit() && c != '.' && c != '-')
        .unwrap_or(base.len());
    let (number, unit) = base.split_at(split);

    match number.parse::<f64>() {
        Ok(value) if !unit.is_empty() => {
            let scaled = value * f64::from(factor);
            // Trim the trailing zeros a float formatter leaves behind, so the
            // output reads as a value someone wrote rather than as arithmetic.
            let text = format!("{scaled:.4}");
            let text = text.trim_end_matches('0').trim_end_matches('.');
            format!("{text}{unit}")
        }
        _ => format!("calc({base} * {factor})"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shipped() -> Spec {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../spec/tokens.toml");
        noctua_design_spec::load(path).expect("the shipped spec")
    }

    fn css() -> String {
        emit(&shipped()).remove(0).contents
    }

    #[test]
    fn the_scale_is_multiplied_out_rather_than_left_as_calc() {
        let css = css();
        assert!(css.contains("--noctua-space-4: 1rem;"), "{css}");
        assert!(css.contains("--noctua-space-0: 0;"));
        assert!(
            !css.contains("calc("),
            "a valid spec should never need calc()"
        );
    }

    /// The token a component writes must exist, and so must the collapse the
    /// media query overrides it with.
    #[test]
    fn every_duration_has_a_reduced_motion_collapse() {
        let spec = shipped();
        let css = css();
        let reduced = css
            .split("prefers-reduced-motion")
            .nth(1)
            .expect("a reduced-motion block");

        for name in spec.motion.duration.keys() {
            assert!(
                css.contains(&format!("--noctua-duration-{name}:")),
                "{name} has no ordinary value"
            );
            assert!(
                reduced.contains(&format!("--noctua-duration-{name}:")),
                "{name} is not collapsed under reduced motion"
            );
        }
    }

    /// A named purpose is an indirection onto a step, not a copy of its value.
    /// Copying would let the two drift when the scale moves.
    #[test]
    fn named_spaces_reference_the_step_rather_than_repeat_it() {
        let css = css();
        assert!(
            css.contains("--noctua-space-gap: var(--noctua-space-4);"),
            "{css}"
        );
    }

    #[test]
    fn the_measure_is_emitted_in_ch() {
        assert!(css().contains("--noctua-measure: 66ch;"));
    }

    /// This repository defines no color. If one appears in the generated CSS
    /// it came from the token spec, which is exactly what the source gate
    /// forbids — caught here too, because this file is what ships.
    #[test]
    fn the_generated_css_defines_no_color() {
        let css = css();
        for needle in ["#", "rgb(", "hsl(", "oklch("] {
            assert!(
                !css.contains(needle),
                "the generated token CSS contains {needle:?}, and colors belong to noctua-colors"
            );
        }
    }

    #[test]
    fn braces_balance() {
        let css = css();
        assert_eq!(
            css.matches('{').count(),
            css.matches('}').count(),
            "unbalanced braces would let the browser decide the layout"
        );
    }

    #[test]
    fn multiplication_keeps_the_unit() {
        assert_eq!(multiply("0.25rem", 4), "1rem");
        assert_eq!(multiply("0.25rem", 6), "1.5rem");
        assert_eq!(multiply("0.25rem", 1), "0.25rem");
        assert_eq!(multiply("0.25rem", 0), "0");
        assert_eq!(multiply("2px", 3), "6px");
    }

    /// An unparseable base must produce an honest expression rather than a
    /// silently wrong number.
    #[test]
    fn an_unparseable_base_falls_back_to_calc() {
        assert_eq!(multiply("var(--x)", 2), "calc(var(--x) * 2)");
    }
}
