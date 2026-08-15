//! Validation for the theme model.
//!
//! Every check corresponds to a failure that is silent at runtime: a default
//! that is not a legal value, a key two settings share, an attribute that
//! collides with one the color system already owns.

use crate::Vendor;
use crate::error::{Problem, did_you_mean};
use crate::theme::Theme;

/// Attributes the color system defines. A setting claiming one of these would
/// fight the vendored stylesheets for the same selector.
const RESERVED: [&str; 2] = ["data-theme", "data-palette"];

/// Checks a parsed theme model against the vendored axes.
pub(crate) fn check(theme: &Theme, vendor: &Vendor) -> Vec<Problem> {
    let mut problems = Vec::new();
    let mut keys: Vec<&str> = Vec::new();
    let mut attributes: Vec<&str> = Vec::new();

    for (name, setting) in &theme.setting {
        let values = setting.resolved_values(vendor);

        if values.is_empty() {
            problems.push(
                Problem::whole_file(format!("setting `{name}` has no values"))
                    .fix("give it `values = [...]` or `values_from = \"axes.<axis>\"`"),
            );
            continue;
        }

        // A default outside the value list means the very first paint uses a
        // value nothing else in the system recognises.
        if !values.contains(&setting.default) {
            let mut problem = Problem::whole_file(format!(
                "setting `{name}` defaults to `{}`, which is not one of its values",
                setting.default
            ));
            problem = match did_you_mean(&setting.default, values.iter().map(String::as_str)) {
                Some(near) => problem.fix(format!("did you mean `{near}`?")),
                None => problem.fix(format!("its values are: {}", values.join(", "))),
            };
            problems.push(problem);
        }

        // `omit_when` names the value that stamps no attribute. Naming one that
        // does not exist means the attribute is always stamped, and the value
        // it was meant to suppress reaches the DOM.
        if let Some(omit) = &setting.omit_when
            && !values.contains(omit)
        {
            problems.push(
                Problem::whole_file(format!(
                    "setting `{name}` omits on `{omit}`, which is not one of its values"
                ))
                .fix(format!("its values are: {}", values.join(", "))),
            );
        }

        // Two settings sharing a key means writing one silently overwrites the
        // other, and the bootstrap restores whichever wrote last.
        if keys.contains(&setting.key.as_str()) {
            problems.push(
                Problem::whole_file(format!("two settings share the key `{}`", setting.key))
                    .fix("one storage key per setting, so a corrupt value degrades one thing"),
            );
        }
        keys.push(&setting.key);

        if let Some(attribute) = &setting.attribute {
            if attributes.contains(&attribute.as_str()) {
                problems.push(
                    Problem::whole_file(format!("two settings stamp `{attribute}`"))
                        .fix("one attribute per setting, or they overwrite each other"),
                );
            }
            attributes.push(attribute);

            // `data-theme` and `data-palette` are the color system's, and this
            // model deliberately reuses them — but only from the settings that
            // mean exactly what the vendored CSS expects.
            let expected = match name.as_str() {
                "mode" => Some("data-theme"),
                "palette" => Some("data-palette"),
                _ => None,
            };
            if RESERVED.contains(&attribute.as_str()) && expected != Some(attribute.as_str()) {
                problems.push(
                    Problem::whole_file(format!(
                        "setting `{name}` stamps `{attribute}`, which noctua-colors defines"
                    ))
                    .fix(
                        "only `mode` may stamp data-theme and only `palette` may stamp \
                         data-palette; anything else would fight the vendored stylesheets",
                    ),
                );
            }
        }

        for source in &setting.derived_from {
            if !theme.setting.contains_key(source) {
                problems.push(
                    Problem::whole_file(format!(
                        "setting `{name}` is derived from `{source}`, which does not exist"
                    ))
                    .fix("name a declared setting"),
                );
            }
        }
    }

    // The one structural requirement: a scope needs both halves.
    //
    // A theme file's forced-mode block is COMPOUND and same-element —
    // `[data-palette="x"][data-theme="dark"]` — so a scope that can set a mode
    // but not a palette can never match one.
    for required in ["mode", "palette"] {
        match theme.setting.get(required) {
            Some(setting) if setting.scope => {}
            Some(_) => problems.push(
                Problem::whole_file(format!("`{required}` is not scopable")).fix(
                    "a themed subtree needs both a palette and a mode: the vendored \
                         forced-mode selector is compound and same-element, so one without \
                         the other matches nothing",
                ),
            ),
            None => problems.push(
                Problem::whole_file(format!("there is no `{required}` setting"))
                    .fix("the theme model needs one; it is what the color system reads"),
            ),
        }
    }

    problems
}
