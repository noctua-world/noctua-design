//! `dist/tones.css` — every tone the colour system has, as one indirection.
//!
//! # The problem this fixes
//!
//! The vendored palette ships **349 semantic families** — `danger`, `warning`,
//! `info`, `pending`, `approved`, `shipped`, `overdue`, `degraded`, `canary`,
//! `chargeback` — each with a foreground, a `-bg`, a `-border`, a `-hover` and
//! an `on-`. The component layer used **nine** of them. Every component that
//! offered a `tone` offered a closed enum of four or five names, so a table of
//! invoices could not say *overdue* and a table of deployments could not say
//! *degraded*, and the answer was always to pick the nearest of `danger` and
//! `success` and lose the distinction the palette had already drawn.
//!
//! Opening the enum is not enough on its own: a component's stylesheet would
//! then need 349 rules, one per name, per surface it paints — and it would have
//! to be rewritten every time the palette gains a word.
//!
//! # The shape
//!
//! One block per family, declaring four properties with fixed names:
//!
//! ```css
//! [data-nd-tone="overdue"] {
//!   --nd-tone: var(--nc-color-overdue);
//!   --nd-tone-bg: var(--nc-color-overdue-bg);
//!   --nd-tone-border: var(--nc-color-overdue-border);
//!   --nd-tone-on: var(--nc-color-on-overdue);
//! }
//! ```
//!
//! A component then writes **one** rule — `background: var(--nd-tone-bg)` — and
//! every name in the palette works, including the ones added after the
//! component was written. The indirection is resolved at the element carrying
//! the attribute, which is below any `ThemeScope`, so a toned cell inside a
//! scoped subtree reads that subtree's palette for the same reason `scope.css`
//! exists.
//!
//! # The two names the palette does not have, and the one it has that is a trap
//!
//! `neutral` and `muted` are what this repository's components have always
//! meant by *quieter than the text around it*, and the palette declares
//! neither: there is no `--nc-color-neutral`. They are emitted here from the
//! structural tokens, so a `tone` prop is one vocabulary rather than two.
//!
//! **`default` is not among them, and that is deliberate.** The palette *does*
//! declare a `default` family — and it is an info blue
//! (`--nc-color-default: var(--nc-info-solid)`). A component whose untoned
//! state wrote `data-nd-tone="default"` would therefore tint every ordinary
//! cell blue. **No tone is the absence of the attribute**, never a name for it:
//! a component writes `data-nd-tone` only when a caller asked for one, and a
//! caller who writes `tone = "default"` gets the palette's blue, which is what
//! that word means to the colour system.
//!
//! # Why generated rather than written
//!
//! `NOCTUA.md §8.8` — this repository declares no colour of its own. Every
//! value here is a `var()` at a name the vendor wrote, transcribed. A family
//! added upstream appears on the next `cargo xtask export`; a family that
//! changes shape fails the build rather than drifting.

use std::collections::BTreeSet;
use std::fmt::Write as _;

use noctua_design_spec::Vendor;

use crate::{CommentStyle, EmittedFile, header};

/// The attribute a toned element carries.
pub const ATTRIBUTE: &str = "data-nd-tone";

/// What this repository's components have always meant by "no tone".
///
/// The palette declares no family for any of them, and every component that
/// offers a tone has offered at least one — so they are part of the vocabulary
/// whether or not the colour system agrees, and a caller should not have to
/// know which three of the list are ours.
/// `on-X` IS THE INK THAT READS ON A SOLID `X`, NEVER `X` ITSELF.
///
/// Both of these named the fill twice: `neutral` was `--nc-color-fg` for the
/// tone and `--nc-color-fg` again for the ink on it, and `muted` did the same
/// with `--nc-color-fg-muted`. Every component that paints
/// `background: var(--nd-tone); color: var(--nd-tone-on)` — which is what a
/// solid tone IS — therefore drew its text in the colour of its own fill at
/// those two names. A neutral badge's count measured 1.00:1 against the pill it
/// was written on: a blank disc, in both modes, on every page that used one.
///
/// The vendor's own families all pair a fill with its `on-` colour, and these
/// two have no vendor family to borrow from — so the pairing is made here, and
/// it is the page's own background. `--nc-color-fg` is chosen to read against
/// `--nc-color-surface`, so `--nc-color-surface` reads against it at exactly
/// the same ratio, in light and in dark, in all thirty-nine palettes.
const STRUCTURAL: [(&str, [&str; 4]); 2] = [
    (
        "neutral",
        [
            "--nc-color-fg",
            "--nc-color-surface-subtle",
            "--nc-color-border-strong",
            "--nc-color-surface",
        ],
    ),
    (
        "muted",
        [
            "--nc-color-fg-muted",
            "--nc-color-surface-subtle",
            "--nc-color-border",
            "--nc-color-surface",
        ],
    ),
];

/// Every family the vendor declares, in the order it declared them.
///
/// A family is a name with all four of `X`, `X-bg`, `X-border` and `on-X`. The
/// `-hover` is not required: a handful of families in the vendored output carry
/// one and the rest do not, and a tone is not a control.
#[must_use]
pub fn families(vendor: &Vendor) -> Vec<String> {
    let names: BTreeSet<&str> = vendor
        .contract
        .iter()
        .map(|(name, _)| name.as_str())
        .collect();

    let mut wanted = Vec::new();
    let mut seen = BTreeSet::new();
    for (name, _) in &vendor.contract {
        let Some(family) = name.strip_prefix("--nc-color-") else {
            continue;
        };
        // The four suffixed forms are found from the bare name, so a family is
        // counted once and in the position the vendor first mentioned it.
        if family.ends_with("-bg") || family.ends_with("-border") || family.ends_with("-hover") {
            continue;
        }
        if family.starts_with("on-") {
            continue;
        }
        let complete = names.contains(format!("--nc-color-{family}-bg").as_str())
            && names.contains(format!("--nc-color-{family}-border").as_str())
            && names.contains(format!("--nc-color-on-{family}").as_str());
        if complete && seen.insert(family.to_owned()) {
            wanted.push(family.to_owned());
        }
    }
    wanted
}

/// Every tone name a caller may write — the vendor's families and ours.
#[must_use]
pub fn all(vendor: &Vendor) -> Vec<String> {
    let mut names: Vec<String> = STRUCTURAL
        .iter()
        .map(|(name, _)| (*name).to_owned())
        .collect();
    for family in families(vendor) {
        if !names.contains(&family) {
            names.push(family);
        }
    }
    names
}

/// Emits `dist/tones.css`, the Rust list and the TypeScript list.
#[must_use]
pub fn emit(vendor: &Vendor) -> Vec<EmittedFile> {
    let families = families(vendor);
    let names = all(vendor);
    vec![
        EmittedFile::new("dist/tones.css", css(&families)),
        EmittedFile::new("crates/noctua-design/src/tones.rs", rust(&names)),
        EmittedFile::new("packages/tokens/tones.js", javascript(&names)),
        EmittedFile::new("packages/tokens/tones.d.ts", types(&names)),
    ]
}

fn css(families: &[String]) -> String {
    let mut out = header(CommentStyle::Block);
    writeln!(
        out,
        "\n/* Every tone, as one indirection.\n\
        \x20\n\
        \x20  A component writes var(--nd-tone-bg) once and every name in the\n\
        \x20  palette works, including the ones added after the component was.\n\
        \x20  Derived from packages/tokens/vendor/css/contexts.css.\n\
        \x20\n\
        \x20  {} families from the colour system, plus neutral and muted, which\n\
        \x20  the palette does not declare and this system has always meant by\n\
        \x20  \"quieter than the text around it\".\n\
        \x20\n\
        \x20  NO TONE IS THE ABSENCE OF THE ATTRIBUTE. The palette's own `default`\n\
        \x20  family is an info blue, so a component whose untoned state named it\n\
        \x20  would tint every ordinary cell. Regenerate rather than edit. */\n",
        families.len()
    )
    .expect("write");

    for (name, [fg, bg, border, on]) in STRUCTURAL {
        block(&mut out, name, fg, bg, border, on);
    }
    for family in families {
        block(
            &mut out,
            family,
            &format!("--nc-color-{family}"),
            &format!("--nc-color-{family}-bg"),
            &format!("--nc-color-{family}-border"),
            &format!("--nc-color-on-{family}"),
        );
    }
    out
}

fn block(out: &mut String, name: &str, fg: &str, bg: &str, border: &str, on: &str) {
    writeln!(out, "[{ATTRIBUTE}=\"{name}\"] {{").expect("write");
    writeln!(out, "  --nd-tone: var({fg});").expect("write");
    writeln!(out, "  --nd-tone-bg: var({bg});").expect("write");
    writeln!(out, "  --nd-tone-border: var({border});").expect("write");
    writeln!(out, "  --nd-tone-on: var({on});").expect("write");
    writeln!(out, "}}\n").expect("write");
}

fn rust(names: &[String]) -> String {
    let mut out = header(CommentStyle::Line("//!"));
    out.push_str(
        "//!\n\
         //! # Every tone a caller may write\n\
         //!\n\
         //! A `tone` prop is text rather than an enum, because the vocabulary\n\
         //! belongs to the colour system and not to any one component. This is\n\
         //! the list, so a caller's spelling can be checked against something\n\
         //! rather than against nothing.\n\n",
    );
    writeln!(
        out,
        "/// Every tone name, in the order the palette declares them."
    )
    .expect("write");
    writeln!(out, "pub const ALL: [&str; {}] = [", names.len()).expect("write");
    for name in names {
        writeln!(out, "    \"{name}\",").expect("write");
    }
    writeln!(out, "];\n").expect("write");
    out.push_str(
        "/// Whether a name is one the generated stylesheet paints.\n\
         ///\n\
         /// A tone nothing declares is an attribute that resolves to nothing —\n\
         /// which looks exactly like a tone that was never asked for, and is the\n\
         /// quietest way a status disappears.\n\
         #[must_use]\n\
         pub fn known(name: &str) -> bool {\n\
         \x20   ALL.contains(&name)\n\
         }\n",
    );
    out
}

fn javascript(names: &[String]) -> String {
    let mut out = header(CommentStyle::Line("//"));
    out.push_str("\n/** Every tone name, in the order the palette declares them. */\nexport const TONES = [\n");
    for name in names {
        writeln!(out, "  \"{name}\",").expect("write");
    }
    out.push_str("];\n\n/** Whether a name is one the generated stylesheet paints. */\nexport function knownTone(name) {\n  return TONES.includes(name);\n}\n");
    out
}

fn types(names: &[String]) -> String {
    let mut out = header(CommentStyle::Line("//"));
    writeln!(
        out,
        "\nexport declare const TONES: readonly [{}];",
        names
            .iter()
            .map(|name| format!("\"{name}\""))
            .collect::<Vec<_>>()
            .join(", ")
    )
    .expect("write");
    out.push_str("\nexport declare function knownTone(name: string): boolean;\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vendor() -> Vendor {
        noctua_design_spec::vendor::load(&noctua_design_spec::vendor::repository_root())
            .expect("the vendored output is valid")
    }

    #[test]
    fn every_family_is_complete() {
        // A family with a foreground and no background is a family a component
        // would paint half of — which reads as a tone that did not apply.
        let vendor = vendor();
        let names: Vec<&str> = vendor
            .contract
            .iter()
            .map(|(name, _)| name.as_str())
            .collect();
        for family in families(&vendor) {
            for suffix in ["-bg", "-border"] {
                assert!(
                    names.contains(&format!("--nc-color-{family}{suffix}").as_str()),
                    "--nc-color-{family}{suffix} is missing"
                );
            }
        }
    }

    #[test]
    fn no_tone_is_the_absence_of_the_attribute() {
        // The palette's own `default` is an info blue, so a component that
        // wrote `data-nd-tone="default"` for its untoned state would tint every
        // ordinary cell. The name is a legitimate family a caller may ask for;
        // it is never what "no tone" is spelled as.
        let vendor = vendor();
        assert!(
            families(&vendor).contains(&"default".to_owned()),
            "the palette stopped declaring `default`; the warning above is stale"
        );
        for (name, _) in STRUCTURAL {
            assert_ne!(name, "default");
        }
    }

    #[test]
    fn the_two_names_this_system_adds_are_not_the_palettes() {
        // If the palette ever declares `neutral`, ours has to go rather than
        // shadow it — two answers for one name is the drift this file exists to
        // prevent.
        let vendor = vendor();
        for (name, _) in STRUCTURAL {
            assert!(
                !families(&vendor).contains(&name.to_owned()),
                "the palette now declares `{name}`; drop this system's copy"
            );
        }
    }
}
