//! Semantic validation.
//!
//! Parsing catches a malformed file. This catches a well-formed file that says
//! something impossible: a named space pointing at a step that does not exist,
//! a line height of zero, a duration with no unit.
//!
//! Everything is checked before anything is reported, so one pass surfaces
//! every problem rather than one per edit.

use crate::error::Problem;
use crate::model::Spec;

/// Checks a parsed spec, returning every problem found.
pub(crate) fn check(spec: &Spec) -> Vec<Problem> {
    let mut problems = Vec::new();

    check_prefix(spec, &mut problems);
    check_space(spec, &mut problems);
    check_type(spec, &mut problems);
    check_motion(spec, &mut problems);
    check_units(spec, &mut problems);

    problems
}

/// The prefix is pasted straight into a custom-property name.
///
/// `--{prefix}-space-4`. Anything outside this alphabet produces a property
/// name a browser silently ignores, which is the worst kind of wrong: the
/// stylesheet parses, the token is undefined, and nothing says so.
fn check_prefix(spec: &Spec, problems: &mut Vec<Problem>) {
    let prefix = &spec.meta.prefix;
    let usable = !prefix.is_empty()
        && !prefix.starts_with('-')
        && !prefix.ends_with('-')
        && prefix
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-');

    if !usable {
        problems.push(
            Problem::whole_file(format!("`{prefix}` is not a usable custom-property prefix"))
                .fix("use lowercase letters, digits and interior hyphens, as in `noctua`"),
        );
    }

    // `nc` is `noctua-colors`' namespace. Sharing it would make "which
    // repository owns this token" unanswerable, and would let a design token
    // silently shadow a colour token of the same name.
    if prefix == "nc" {
        problems.push(
            Problem::whole_file("`nc` is the namespace `noctua-colors` emits into")
                .fix("pick another prefix; the two token sets must stay distinguishable"),
        );
    }
}

fn check_space(spec: &Spec, problems: &mut Vec<Problem>) {
    let space = &spec.space;

    if space.steps.is_empty() {
        problems.push(
            Problem::whole_file("the spacing scale has no steps")
                .fix("give it multipliers, as in `steps = [0, 1, 2, 4, 8]`"),
        );
        return;
    }

    if !space.steps.windows(2).all(|w| w[0] < w[1]) {
        problems.push(
            Problem::whole_file("the spacing steps are not in ascending order")
                .fix("a scale is read as a sequence; sort it and remove any duplicate"),
        );
    }

    // A named purpose that points nowhere emits a token whose value is a
    // multiplication by a step that was never declared.
    //
    // The suggestion is the numerically nearest step, not the most similar
    // string. Edit distance is the right tool for a misspelled identifier and
    // the wrong one for a number: from `3`, every one-digit step is distance 1,
    // so it would suggest whichever happened to be listed first.
    for (name, step) in &space.named {
        if !space.steps.contains(step) {
            let nearest = space
                .steps
                .iter()
                .min_by_key(|candidate| candidate.abs_diff(*step));
            let problem = Problem::whole_file(format!(
                "space.named.{name} is step {step}, which is not in the scale"
            ));
            problems.push(match nearest {
                Some(near) => problem.fix(format!(
                    "the nearest declared step is {near}; the scale is {}",
                    space
                        .steps
                        .iter()
                        .map(u32::to_string)
                        .collect::<Vec<_>>()
                        .join(", ")
                )),
                None => problem.fix("the scale has no steps at all"),
            });
        }
    }
}

fn check_type(spec: &Spec, problems: &mut Vec<Problem>) {
    let typography = &spec.r#type;

    for (name, value) in &typography.leading {
        if *value <= 0.0 {
            problems.push(
                Problem::whole_file(format!("type.leading.{name} is {value}"))
                    .fix("line height is a positive multiple of the font size, as in `1.6`"),
            );
        }
        // A unitless line height below 1 puts successive lines on top of each
        // other. It is legal CSS, so nothing downstream will object.
        if *value > 0.0 && *value < 1.0 {
            problems.push(
                Problem::whole_file(format!(
                    "type.leading.{name} is {value}, which is less than the font size"
                ))
                .fix("lines would overlap; 1.15 is about as tight as a display size goes"),
            );
        }
    }

    for (name, value) in &typography.weight {
        if !(1..=1000).contains(value) {
            problems.push(
                Problem::whole_file(format!("type.weight.{name} is {value}"))
                    .fix("CSS font weights run from 1 to 1000; the usual set is 400 to 700"),
            );
        }
    }

    // The measure is the one §9.7 rule with a number attached, and a measure
    // outside the readable band silently defeats it.
    for (label, value) in [
        ("measure", typography.measure),
        ("measure_lead", typography.measure_lead),
    ] {
        if let Some(ch) = value
            && !(20..=120).contains(&ch)
        {
            problems.push(
                Problem::whole_file(format!("type.{label} is {ch} characters"))
                    .fix("readable line length runs about 45–75 characters (NOCTUA.md §9.7)"),
            );
        }
    }
}

fn check_motion(spec: &Spec, problems: &mut Vec<Problem>) {
    for (name, duration) in &spec.motion.duration {
        for (which, value) in [("normal", &duration.normal), ("reduced", &duration.reduced)] {
            if !is_duration(value) {
                problems.push(
                    Problem::whole_file(format!(
                        "motion.duration.{name}.{which} is `{value}`, which is not a duration"
                    ))
                    .fix("give it a unit: `180ms` or `0.18s`"),
                );
            }
        }
    }
}

/// Lengths must carry a unit, or be an unambiguous zero.
///
/// CSS accepts a bare `0` and rejects every other bare number. A `radius` of
/// `8` produces an invalid declaration the browser drops without a word.
fn check_units(spec: &Spec, problems: &mut Vec<Problem>) {
    let mut lengths: Vec<(String, &String)> = Vec::new();
    lengths.push(("space.base".to_owned(), &spec.space.base));
    lengths.extend(spec.radius.iter().map(|(k, v)| (format!("radius.{k}"), v)));

    // Every preset's values are lengths too, and a preset with a unitless
    // number produces a declaration the browser drops without a word.
    for (preset, radii) in &spec.preset.radius {
        lengths.extend(
            radii
                .iter()
                .map(|(k, v)| (format!("preset.radius.{preset}.{k}"), v)),
        );
    }

    lengths.extend(spec.border.iter().map(|(k, v)| (format!("border.{k}"), v)));
    lengths.extend(
        spec.glass
            .blur
            .iter()
            .map(|(k, v)| (format!("glass.blur.{k}"), v)),
    );
    lengths.extend(
        spec.r#type
            .size
            .iter()
            .map(|(k, v)| (format!("type.size.{k}"), v)),
    );
    lengths.extend(
        spec.breakpoint
            .iter()
            .map(|(k, v)| (format!("breakpoint.{k}"), v)),
    );
    lengths.extend(spec.focus.iter().map(|(k, v)| (format!("focus.{k}"), v)));

    for (path, value) in lengths {
        if !is_length(value) {
            problems.push(
                Problem::whole_file(format!("{path} is `{value}`, which is not a length"))
                    .fix("give it a unit — `0.5rem`, `2px`, `9999px` — or write a bare `0`"),
            );
        }
    }
}

/// The CSS length units this format accepts.
const UNITS: [&str; 8] = ["rem", "em", "px", "ch", "vh", "vw", "%", "pt"];

fn is_length(value: &str) -> bool {
    let value = value.trim();
    if value == "0" {
        return true;
    }
    UNITS.iter().any(|unit| {
        value
            .strip_suffix(unit)
            .is_some_and(|number| !number.is_empty() && number.parse::<f64>().is_ok())
    })
}

fn is_duration(value: &str) -> bool {
    let value = value.trim();
    ["ms", "s"].iter().any(|unit| {
        value
            .strip_suffix(unit)
            .is_some_and(|number| !number.is_empty() && number.parse::<f64>().is_ok())
    })
}
