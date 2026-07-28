//! Validation for the font layer.
//!
//! Every check here corresponds to a failure that is **silent in a browser**.
//! A wrong `@font-face` does not error: the text renders in a fallback, or at
//! the wrong weight, or as tofu, and it looks like a design decision.

use crate::error::Problem;
use crate::fonts::Fonts;

/// Checks a parsed font layer, returning every problem found.
pub(crate) fn check(fonts: &Fonts) -> Vec<Problem> {
    let mut problems = Vec::new();

    for face in &fonts.face {
        let name = format!("{} ({})", face.family, face.file);

        // A range this face does not have means the browser fetches it for a
        // codepoint it cannot render, and shows tofu rather than falling
        // through to the next family.
        if !fonts.range.contains_key(&face.range) {
            problems.push(
                Problem::whole_file(format!(
                    "{name} uses range `{}`, which is not declared",
                    face.range
                ))
                .fix(format!(
                    "the ranges declared are: {}",
                    fonts.range.keys().cloned().collect::<Vec<_>>().join(", ")
                )),
            );
        }

        if !["normal", "italic", "oblique"].contains(&face.style.as_str()) {
            problems.push(
                Problem::whole_file(format!("{name} has style `{}`", face.style))
                    .fix("a font style is `normal`, `italic` or `oblique`"),
            );
        }

        if !["auto", "block", "swap", "fallback", "optional"].contains(&face.display.as_str()) {
            problems.push(
                Problem::whole_file(format!("{name} has display `{}`", face.display))
                    .fix("font-display is auto, block, swap, fallback or optional"),
            );
        }

        // The pairing that makes `optional` work at all.
        //
        // `optional` uses a face ONLY if it is ready by the first paint and
        // never swaps afterwards — which is exactly why it avoids the reflow
        // `swap` causes, and exactly why an `optional` face that is not
        // preloaded will essentially never render. The two fields are one
        // decision, so they are checked as one.
        if face.display == "optional" && !face.preload {
            problems.push(
                Problem::whole_file(format!(
                    "{name} is `font-display: optional` but is not preloaded"
                ))
                .fix(
                    "`optional` uses a face only if it is ready by the first paint, so an \
                     un-preloaded one never renders. Preload it, or use `swap`",
                ),
            );
        }

        // The reverse is not an error but is always a mistake worth naming:
        // preloading a face the browser is free to swap in later spends the
        // bytes without buying the guarantee.
        if face.preload && face.display != "optional" {
            problems.push(
                Problem::whole_file(format!(
                    "{name} is preloaded but is `font-display: {}`",
                    face.display
                ))
                .fix(
                    "preloading buys the no-reflow guarantee that only `optional` cashes in; \
                     with any other value the bytes are spent for nothing",
                ),
            );
        }

        // A static face with no declared weight renders at whatever the file's
        // default instance is, which for four of the Noto families is Thin.
        if !face.is_variable_weight() && face.weight.is_none() {
            problems.push(
                Problem::whole_file(format!("{name} is static but declares no weight")).fix(
                    "give it `weight = 400`. Without one the browser uses the file's \
                         default instance, which for Noto JP, KR, SC and Hebrew is Thin — \
                         hairline text with a fabricated bold on top (NOCTUA.md §7.6)",
                ),
            );
        }

        for axis in &face.axes {
            if !["wght", "wdth", "slnt", "ital", "opsz"].contains(&axis.as_str()) {
                problems.push(
                    Problem::whole_file(format!("{name} lists axis `{axis}`"))
                        .fix("the registered axes are wght, wdth, slnt, ital and opsz"),
                );
            }
        }
    }

    // A stack that names no family is a variable that resolves to nothing.
    for (role, stack) in &fonts.stack {
        if stack.trim().is_empty() {
            problems.push(
                Problem::whole_file(format!("the `{role}` stack is empty"))
                    .fix("a stack is a comma-separated family list, ending in a generic"),
            );
        }
    }

    problems
}
