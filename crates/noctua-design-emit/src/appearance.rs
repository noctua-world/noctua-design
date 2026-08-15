//! `dist/theme.css` — what every appearance setting actually does.
//!
//! Every rule here is keyed on an attribute the bootstrap already stamped, so
//! nothing needs JavaScript to apply and nothing can flash. A setting's visual
//! consequence is CSS; the script only writes the attribute.
//!
//! # One selector shape, and the one setting that cannot have it
//!
//! **Scopable settings** use a bare attribute selector — `[data-noctua-radius]`
//! — so they reach any element, and a themed subtree carries its own corners by
//! plain custom-property inheritance. Every setting emitted here has that
//! shape, including the backdrop, the pattern, the aurora and the grain, which
//! did not until each was given a custom property of its own to declare.
//!
//! The one that is still page-only is `syntax`, and it is not emitted here —
//! its payload is a `color` on a descendant, which has no nearest-ancestor
//! rule, so `dist/syntax.css` anchors it at `:root` and a test in that module
//! holds it there. The restriction is the selector, not a comment somebody has
//! to read.
//!
//! # Glass is never pre-mixed here, and that is not an oversight
//!
//! `--noctua-glass-surface: color-mix(in oklab, var(--nc-color-surface) 88%,
//! transparent)` declared at `:root` would freeze to the *page's* palette, for
//! exactly the reason `--nc-color-surface` itself does: a `var()` resolves
//! where it is declared. A themed component would show the page's frost.
//!
//! So this file emits only dimensionless numbers, and the `color-mix()` is
//! written at the point of use, where the palette is already correct.

use std::fmt::Write as _;

use noctua_design_spec::{Spec, Theme};

use crate::{CommentStyle, EmittedFile, header};

/// Emits `dist/theme.css`.
#[must_use]
pub fn emit(spec: &Spec, theme: &Theme) -> Vec<EmittedFile> {
    let prefix = &spec.meta.prefix;
    let mut out = header(CommentStyle::Block);

    writeln!(
        out,
        "\n/* What every appearance setting does.\n\
        \x20\n\
        \x20  Each rule is keyed on an attribute the inline bootstrap stamped before\n\
        \x20  the first paint, so nothing here needs script and nothing flashes.\n\
        \x20\n\
        \x20  Every setting here uses a bare attribute selector and therefore reaches\n\
        \x20  any element: the attribute declares a custom property and one\n\
        \x20  unconditional rule reads it, so the nearest declaring ancestor wins by\n\
        \x20  construction and a subtree can cancel what is above it. The one\n\
        \x20  page-only setting, `syntax`, is in dist/syntax.css and anchored at\n\
        \x20  :root, which is the restriction being structural rather than\n\
        \x20  documented. */\n"
    )
    .expect("write");

    radius_presets(&mut out, spec, prefix);
    scrollbars(&mut out, prefix);
    glass(&mut out, spec, theme, prefix);
    backdrop(&mut out, prefix);
    effects(&mut out, spec, prefix);

    vec![
        EmittedFile::new("dist/theme.css", out),
        EmittedFile::new("dist/favicon.svg", favicon()),
    ]
}

/// The tab icon.
///
/// # Why it is here at all
///
/// The site shipped without one, so every page load asked for `/favicon.ico`
/// and got a 404. The browser asks unprompted; there is no way to decline
/// except by answering.
///
/// # Why it is monochrome
///
/// A favicon is a static file: it cannot read a custom property, so it cannot
/// follow the reader's palette, and picking one accent would make ochre the
/// system's brand color by accident. Two neutrals and
/// `prefers-color-scheme` instead — which is also the only pair that reads at
/// 16 px against both a light and a dark browser chrome.
///
/// **This is a placeholder that behaves correctly, not a mark.** An aperture:
/// a ring and a pupil, which is as far as "noctua" should be taken without
/// somebody deciding to.
fn favicon() -> String {
    // An SVG has no line-comment form the gate reads, so the header is an XML
    // comment — and it is what exempts the two neutrals below from the color
    // gate, exactly as it exempts every other generated file.
    let mut out = String::from("<!--\n");
    for line in header(CommentStyle::Line("")).lines() {
        let _ = writeln!(out, "{}", line.trim_start());
    }
    out.push_str("-->\n");
    out.push_str(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 32 32\">\n\
        \x20 <style>\n\
        \x20   :root { color-scheme: light dark }\n\
        \x20   .ink { fill: none; stroke: #1a1a1a; stroke-width: 2.5 }\n\
        \x20   .pupil { fill: #1a1a1a; stroke: none }\n\
        \x20   @media (prefers-color-scheme: dark) {\n\
        \x20     .ink { stroke: #f2f2f2 }\n\
        \x20     .pupil { fill: #f2f2f2 }\n\
        \x20   }\n\
        \x20 </style>\n\
        \x20 <circle class=\"ink\" cx=\"16\" cy=\"16\" r=\"12\"/>\n\
        \x20 <circle class=\"pupil\" cx=\"16\" cy=\"16\" r=\"4.5\"/>\n\
        </svg>\n",
    );
    out
}

/// The corner presets, as attribute-scoped overrides of the base radii.
fn radius_presets(out: &mut String, spec: &Spec, prefix: &str) {
    if spec.preset.radius.is_empty() {
        return;
    }

    writeln!(
        out,
        "/* Corner presets. Emitted AFTER :root in the base tokens, and as a bare\n\
        \x20  attribute selector rather than :root[data-...] — which is what lets a\n\
        \x20  themed component carry its own corners, by inheritance, with no script. */\n"
    )
    .expect("write");

    for (preset, radii) in &spec.preset.radius {
        writeln!(out, "[data-{prefix}-radius=\"{preset}\"] {{").expect("write");
        for (name, value) in radii {
            writeln!(out, "  --{prefix}-radius-{name}: {value};").expect("write");
        }
        writeln!(out, "}}\n").expect("write");
    }
}

/// The scrollbar, once, for everything.
///
/// A browser's default scrollbar is a piece of somebody else's design system
/// sitting inside this one, and on a dark page a light one is the brightest
/// thing on screen. Two declarations cover every engine: the standard
/// properties, and the `WebKit` pseudo-elements for the classic bar.
///
/// The thumb is mixed from the foreground color at the point of use, so a
/// scroller inside a themed subtree gets that theme's thumb rather than the
/// page's.
fn scrollbars(out: &mut String, prefix: &str) {
    writeln!(
        out,
        "/* One scrollbar treatment, for every scroller in the system.\n\
        \x20\n\
        \x20  `:where(*)` and not `:where(:root)`, because `scrollbar-width` is NOT an\n\
        \x20  inherited property. Set on the root alone it styles the page's own bar and\n\
        \x20  nothing else — every panel, drawer and code block keeps the browser's\n\
        \x20  default, which is the one a reader actually complains about. Zero\n\
        \x20  specificity, so any surface can still opt out. */\n\
        :where(*) {{\n\
        \x20 scrollbar-width: thin;\n\
        \x20 scrollbar-color: color-mix(in oklab, var(--nc-color-fg) 24%, transparent)\n\
        \x20                  transparent;\n\
        }}\n\n\
        ::-webkit-scrollbar {{\n\
        \x20 inline-size: var(--{prefix}-space-2);\n\
        \x20 block-size: var(--{prefix}-space-2);\n\
        }}\n\n\
        ::-webkit-scrollbar-track {{\n\
        \x20 background: transparent;\n\
        }}\n\n\
        ::-webkit-scrollbar-thumb {{\n\
        \x20 background: color-mix(in oklab, var(--nc-color-fg) 24%, transparent);\n\
        \x20 border-radius: var(--{prefix}-radius-pill);\n\
        }}\n\n\
        ::-webkit-scrollbar-thumb:hover {{\n\
        \x20 background: color-mix(in oklab, var(--nc-color-fg) 38%, transparent);\n\
        }}\n"
    )
    .expect("write");
}

/// The frosted surface, as dimensions only.
fn glass(out: &mut String, spec: &Spec, theme: &Theme, prefix: &str) {
    if spec.glass.blur.is_empty() {
        return;
    }

    writeln!(
        out,
        "/* Glass. Dimensions only — the color is mixed at the point of use, where\n\
        \x20  the palette is already correct. A pre-mixed surface declared here would\n\
        \x20  freeze to the page's palette and a themed component would show the\n\
        \x20  page's frost.\n\
        \x20\n\
        \x20  Use them as:\n\
        \x20\n\
        \x20      background: color-mix(in oklab, var(--nc-color-surface)\n\
        \x20                            var(--{prefix}-glass-opacity), transparent);\n\
        \x20      backdrop-filter: blur(var(--{prefix}-glass-blur)); */\n"
    )
    .expect("write");

    // THE DEFAULT LEVEL IS ALSO DECLARED AT THE ROOT, and leaving it out is a
    // bug this shipped once.
    //
    // Every other block here is a bare attribute selector, which is what makes
    // glass scopable. But nothing stamps `data-noctua-glass` until a reader
    // changes it — `system`-style absence is the normal state — so on a first
    // visit no block matched and the two tokens were simply *undefined*. A
    // `backdrop-filter: blur(var(--noctua-glass-blur))` with an undefined
    // custom property is invalid at computed-value time, so it becomes `none`:
    // every frosted surface in the system silently rendered flat, and nothing
    // reported anything.
    //
    // `:where()` keeps it at zero specificity, so `[data-noctua-glass="off"]`
    // on the root still wins.
    if let Some(setting) = theme.setting.get("glass") {
        let level = &setting.default;
        if spec.glass.blur.contains_key(level) {
            writeln!(out, ":where(:root) {{").expect("write");
            level_tokens(out, spec, prefix, level);
            writeln!(out, "}}\n").expect("write");
        }
    }

    // `off` is the absence of the tokens, so a surface that asks for them
    // falls back to opaque rather than to a half-applied frost.
    writeln!(out, "[data-{prefix}-glass=\"off\"] {{").expect("write");
    for (name, value) in [
        ("blur", "0"),
        ("opacity", "100%"),
        ("saturate", "1"),
        ("brightness", "1"),
        ("edge", "0%"),
        ("sheen", "0%"),
    ] {
        writeln!(out, "  --{prefix}-glass-{name}: {value};").expect("write");
    }
    writeln!(out, "}}\n").expect("write");

    for level in spec.glass.blur.keys() {
        writeln!(out, "[data-{prefix}-glass=\"{level}\"] {{").expect("write");
        level_tokens(out, spec, prefix, level);
        writeln!(out, "}}\n").expect("write");
    }

    surface(out, prefix);

    // Transparency is an accessibility preference, exactly as motion is.
    writeln!(
        out,
        "/* Honoured for the same reason durations carry a reduced pair: a reader who\n\
        \x20  asked their system for less transparency did not ask this page. */\n\
        @media (prefers-reduced-transparency: reduce) {{\n\
        \x20 :root {{\n\
        \x20   --{prefix}-glass-blur: 0;\n\
        \x20   --{prefix}-glass-opacity: 100%;\n\
        \x20   --{prefix}-glass-saturate: 1;\n\
        \x20   --{prefix}-glass-brightness: 1;\n\
        \x20   --{prefix}-glass-edge: 0%;\n\
        \x20   --{prefix}-glass-sheen: 0%;\n\
        \x20 }}\n\
        }}\n"
    )
    .expect("write");
}

/// Every dimension of one glass level.
fn level_tokens(out: &mut String, spec: &Spec, prefix: &str, level: &str) {
    let blur = spec.glass.blur.get(level).cloned().unwrap_or_default();
    let opacity = spec.glass.opacity.get(level).copied().unwrap_or(100);
    let saturate = spec
        .glass
        .saturate
        .get(level)
        .cloned()
        .unwrap_or_else(|| "1".to_owned());
    let brightness = spec
        .glass
        .brightness
        .get(level)
        .cloned()
        .unwrap_or_else(|| "1".to_owned());
    let edge = spec.glass.edge.get(level).copied().unwrap_or(0);
    let sheen = spec.glass.sheen.get(level).copied().unwrap_or(0);

    writeln!(out, "  --{prefix}-glass-blur: {blur};").expect("write");
    writeln!(out, "  --{prefix}-glass-opacity: {opacity}%;").expect("write");
    writeln!(out, "  --{prefix}-glass-saturate: {saturate};").expect("write");
    writeln!(out, "  --{prefix}-glass-brightness: {brightness};").expect("write");
    writeln!(out, "  --{prefix}-glass-edge: {edge}%;").expect("write");
    writeln!(out, "  --{prefix}-glass-sheen: {sheen}%;").expect("write");
}

/// The one glass recipe. Every frosted surface in the system takes this class.
///
/// # Why it is emitted here rather than written in a stylesheet
///
/// It was written by hand, four times: the header's backdrop, the drawer, the
/// settings panel and an unused utility, each repeating the same literal
/// `blur(…) saturate(1.4)` over the same `color-mix`. They drifted — the drawer
/// dropped its glass entirely above `lg` — and the sidebar and the panel ended
/// up looking like different materials on the same page, which is what a reader
/// notices first. One definition, and a source gate that forbids a second.
///
/// # Why the `color-mix` is inside a class and not a token
///
/// `var()` resolves where it is DECLARED. A `--noctua-glass-surface` at `:root`
/// would freeze to the page's palette and hand every themed component the
/// page's frost. Inside `.{prefix}-glass` it resolves on the element that
/// carries the class, which is where the palette is already correct.
fn surface(out: &mut String, prefix: &str) {
    writeln!(
        out,
        "/* The one glass recipe. Adding a second `backdrop-filter` anywhere is a\n\
        \x20  source-gate finding: four hand-written copies drifted into two materials\n\
        \x20  on one page, and a nested one is inert with nothing to report.\n\
        \x20\n\
        \x20  TWO SPELLINGS, ONE RECIPE. The class is what the site's own chrome\n\
        \x20  carries; `[data-nd-glass=\"true\"]` is what a COMPONENT carries, from a\n\
        \x20  `glass` prop its contract declares. They are in one selector rather\n\
        \x20  than two rules for the reason the gate exists: a second copy is a\n\
        \x20  second material the day one of them is tuned. */\n\
        .{prefix}-glass,\n\
        [data-nd-glass=\"true\"] {{\n\
        \x20 background: color-mix(in oklab, var(--nc-color-surface)\n\
        \x20                       var(--{prefix}-glass-opacity), transparent);\n\
        \x20 backdrop-filter: blur(var(--{prefix}-glass-blur))\n\
        \x20                  saturate(var(--{prefix}-glass-saturate))\n\
        \x20                  brightness(var(--{prefix}-glass-brightness));\n\
        \x20 -webkit-backdrop-filter: blur(var(--{prefix}-glass-blur))\n\
        \x20                          saturate(var(--{prefix}-glass-saturate))\n\
        \x20                          brightness(var(--{prefix}-glass-brightness));\n\
        \x20 /* The lit edge. A border rather than a shadow: §9.7 forbids elevated\n\
        \x20    surfaces, and a browser test asserts nothing in the shell casts one. */\n\
        \x20 border-block-start: var(--{prefix}-border-hairline) solid\n\
        \x20   color-mix(in oklab, var(--nc-color-surface-raised)\n\
        \x20             var(--{prefix}-glass-edge), transparent);\n\
        \x20 position: relative;\n\
        }}\n\n\
        /* The sheen, as a DESCENDANT. A pseudo-element is not an ancestor, so it\n\
        \x20  cannot stop its own parent's backdrop from sampling. */\n\
        .{prefix}-glass::after,\n\
        [data-nd-glass=\"true\"]::after {{\n\
        \x20 content: \"\";\n\
        \x20 position: absolute;\n\
        \x20 inset: 0;\n\
        \x20 border-radius: inherit;\n\
        \x20 pointer-events: none;\n\
        \x20 background: linear-gradient(to bottom,\n\
        \x20   color-mix(in oklab, var(--nc-color-surface-raised)\n\
        \x20             var(--{prefix}-glass-sheen), transparent) 0,\n\
        \x20   transparent var(--{prefix}-space-8));\n\
        }}\n"
    )
    .expect("write");
}

/// The scope list at zero specificity, with `suffix` anded onto it.
///
/// `:where()` is the point: these are BASE declarations, the value in force
/// when nothing more specific answers, so they must not outweigh the values a
/// reader actually picked. The list itself is `dist/scope.css`'s, because
/// "everywhere the semantic contract resolves" and "everywhere a backdrop can
/// be" are the same set by construction.
fn anywhere(suffix: &str) -> String {
    format!(
        ":where({}){suffix}",
        crate::scope::SELECTOR.replace('\n', " ")
    )
}

/// Where the ambient settings are actually painted.
///
/// The page and a scope, and nothing else. The rest of `SELECTOR` is there so
/// the semantic contract RE-RESOLVES at an element that carries a palette or a
/// mode by hand; painting a backdrop over it as well would give any consumer
/// who ever wrote `data-palette` on a `<span>` an opaque box they never asked
/// for.
const PAINTED: &str = ":root,\n[data-noctua-scope]";

/// Pure black, and only where it cannot move a measured contrast pair.
fn backdrop(out: &mut String, prefix: &str) {
    writeln!(
        out,
        "/* The backdrop \u{2014} what the page is painted with.\n\
        \x20\n\
        \x20  It changes ONE variable, and only the document background reads it.\n\
        \x20  --nc-color-surface is untouched, so every pair the color system\n\
        \x20  measured still holds, and every raised surface simply becomes more\n\
        \x20  or less visible against it \u{2014} which is the whole point.\n\
        \x20\n\
        \x20  Five of the six are tokens the color system already solved in both\n\
        \x20  modes, so a value that means `a colder dark` in dark means `the\n\
        \x20  same neutral` in light without a second rule saying so.\n\
        \x20\n\
        \x20  DECLARED WHEREVER A SCOPE CAN BE, not only at :root. It used to be\n\
        \x20  :root-only, with the reasoning that a black island inside a themed\n\
        \x20  component is not a theme \u{2014} which was right about black and wrong\n\
        \x20  about the property. A var() resolves where it is DECLARED, so a\n\
        \x20  :root-only page background is frozen to the root's palette and mode:\n\
        \x20  a themed frame reading it painted the page's colour under its own\n\
        \x20  dark, which is dark ink on white. The same list dist/scope.css uses,\n\
        \x20  for the same reason it uses it. */\n"
    )
    .expect("write");

    // LIGHT IS PURE WHITE AND DARK IS THE PALETTE'S PAGE, so the base is
    // mode-dependent where it used to be flat. The lightest step of the neutral
    // ramp is #fdfdfd, and a page is the one surface that should be able to be
    // the paper itself.
    writeln!(
        out,
        "{} {{\n\
        \x20 --{prefix}-backdrop: #fff; /* allow-literal: the page is paper; no ramp token is pure white, and this is the base every rule below overrides */\n\
        }}\n",
        anywhere("")
    )
    .expect("write");

    dark_rule(
        out,
        prefix,
        None,
        &format!("--{prefix}-backdrop: var(--nc-color-surface);"),
    );

    // The four that mean something in both modes, in one rule each. No mode
    // condition: the token already carries it.
    for (value, token) in [
        ("cool", "--nc-color-surface-cool"),
        ("tinted", "--nc-color-accent-bg"),
        ("soft", "--nc-color-surface-subtle"),
        ("softer", "--nc-color-surface-raised"),
    ] {
        writeln!(
            out,
            "[data-{prefix}-backdrop=\"{value}\"] {{\n\
            \x20 --{prefix}-backdrop: var({token});\n\
            }}\n"
        )
        .expect("write");
    }

    // Black is the one value with no light meaning, so it is the one that needs
    // both halves of the dark condition written out. No ramp token is pure
    // black either \u{2014} `--nc-gray-1` is #030303, which still lights an OLED pixel.
    dark_rule(
        out,
        prefix,
        Some("black"),
        &format!(
            "--{prefix}-backdrop: #000; /* allow-literal: this value IS pure black, and no ramp token is \u{2014} --nc-gray-1 is #030303, which still lights an OLED pixel */"
        ),
    );
}

/// One declaration, applied in dark however the reader got there.
///
/// Dark is two conditions that must never disagree: the attribute a reader
/// chose, and the system preference they did not. Writing them apart is how one
/// of them ends up with a rule the other does not have.
fn dark_rule(out: &mut String, prefix: &str, value: Option<&str>, declaration: &str) {
    // THREE WEIGHTS, AND THEY ARE THE WHOLE MECHANISM.
    //
    // The base pair is (0,0,0) — `anywhere()` wraps the scope list in
    // `:where()` — so it is the floor and nothing else has to beat it by luck.
    // A named backdrop is (0,1,0) and therefore wins over the base dark rule
    // outright. `black` is (0,2,0), because it is the one value that also names
    // a mode.
    //
    // This used to be a tie settled by source order, which held only while the
    // base was `:root`-anchored: the moment the base took the scope list it
    // gained an attribute selector and quietly outweighed every named value, so
    // a dark page asking for `cool` got the plain surface instead.
    //
    // The media branch's `:not([data-theme="light"])` is only sound because
    // `scope_attributes` stamps the RESOLVED mode on the same element and never
    // leaves it to inheritance: if the resolved mode is light, `data-theme` is
    // there to be excluded. That is a contract dependency, and it is the thing
    // that breaks if `omit_when` ever changes.
    let each = |suffix: &str| {
        value.map_or_else(
            || anywhere(&format!(":where({suffix})")),
            |value| format!("[data-{prefix}-backdrop=\"{value}\"]{suffix}"),
        )
    };
    let forced = each("[data-theme=\"dark\"]");
    let media = each(":not([data-theme=\"light\"])");
    writeln!(
        out,
        "{forced} {{\n\
        \x20 {declaration}\n\
        }}\n\
        \n\
        @media (prefers-color-scheme: dark) {{\n\
        \x20 {media} {{\n\
        \x20   {declaration}\n\
        \x20 }}\n\
        }}\n"
    )
    .expect("write");
}

/// Patterns, aurora, grain, and the ambient-motion switch.
fn effects(out: &mut String, spec: &Spec, prefix: &str) {
    let alpha =
        |name: &str, fallback: u32| spec.effect.alpha.get(name).copied().unwrap_or(fallback);

    writeln!(
        out,
        "/* Background patterns and effects.\n\
        \x20\n\
        \x20  Two guarantees keep them from hurting readability, and they are\n\
        \x20  independent so either alone is still a real bound:\n\
        \x20\n\
        \x20  1. The ink is the FOREGROUND color at {}%, so its polarity and its\n\
        \x20     palette follow automatically, and the worst case for any glyph is a\n\
        \x20     background pixel moved {}% toward the text color. That bound is\n\
        \x20     measured by a test rather than asserted.\n\
        \x20  2. The layer is masked to nothing behind the reading column.\n\
        \x20\n\
        \x20  Geometry is percentages and spacing tokens, never raw lengths, so a\n\
        \x20  pattern scales with the scale. */\n",
        alpha("pattern", 4),
        alpha("pattern", 4)
    )
    .expect("write");

    // THE STRENGTH IS A TOKEN; THE COLOUR IS NOT.
    //
    // These were `--…-pattern-ink: color-mix(in oklab, var(--nc-color-fg) 4%,
    // transparent)` at `:root` — a custom property holding a `var(--nc-…)`,
    // which is the one thing `gates::frozen_color_indirection` refuses. A
    // `var()` resolves where it is DECLARED, so the ink was frozen to the
    // root's palette and mode however deep the element reading it was. It
    // passed only because `dist/` is not walked by the gates.
    //
    // Harmless while there was exactly one effects layer, at page level. The
    // moment a scope paints its own, the ink would be the PAGE's foreground —
    // so a light scope on a dark page draws near-white on near-white and the
    // setting silently does nothing.
    //
    // What survives as a token is the dimensionless percentage, which carries
    // no colour and can be read anywhere; the `color-mix` moves to the point of
    // use. `glass()` above already makes exactly this trade.
    //
    // The grain is absent here, and that is not an omission: its ceiling is
    // baked into the picture by `write_ambient`, because a background layer has
    // no opacity of its own. A property declared here and read by nothing would
    // be a ceiling that looks enforced and is not.
    writeln!(out, "{} {{", anywhere("")).expect("write");
    writeln!(out, "  --{prefix}-pattern-alpha: {}%;", alpha("pattern", 4)).expect("write");
    writeln!(out, "}}\n").expect("write");

    // The patterns. Each declares one `background-image` for the layer to read.
    let ink = format!(
        "color-mix(in oklab, var(--nc-color-fg) var(--{prefix}-pattern-alpha), transparent)"
    );
    // A DOT COVERS A FORTIETH OF ITS TILE. The other three cover most of theirs,
    // so at one shared alpha the grid patterns read as texture and the dots read
    // as a smudge — a setting a reader turns on and cannot tell they turned on.
    // The ceiling is per-pixel and this is a second one, declared in
    // `spec/tokens.toml` beside the first and measured by the same test.
    let dots =
        format!("color-mix(in oklab, var(--nc-color-fg) var(--{prefix}-dots-alpha), transparent)");
    let hairline = format!("var(--{prefix}-border-hairline)");
    // EVERY ONE DECLARES BOTH, so a value cannot inherit half of another. A
    // pattern that named only its image would keep whatever size the scope
    // above it had chosen, which is two settings out of one attribute.
    let patterns: [(&str, String); 4] = [
        (
            "dots",
            format!(
                "--{prefix}-dots-alpha: {}%;\n  \
                 --{prefix}-pattern-image: radial-gradient(circle at 50% 50%, {dots} 0 14%, transparent 14%);\n  \
                 --{prefix}-pattern-size: var(--{prefix}-space-6) var(--{prefix}-space-6);",
                alpha("dots", 10)
            ),
        ),
        (
            "mesh",
            format!(
                "--{prefix}-pattern-image:\n    \
                 repeating-linear-gradient(to right, {ink} 0 {hairline}, transparent {hairline} var(--{prefix}-space-8)),\n    \
                 repeating-linear-gradient(to bottom, {ink} 0 {hairline}, transparent {hairline} var(--{prefix}-space-8));\n  \
                 --{prefix}-pattern-size: auto;"
            ),
        ),
        (
            "checks",
            format!(
                "--{prefix}-pattern-image: conic-gradient(from 90deg at 50% 50%, {ink} 0 25%, transparent 0 50%, {ink} 0 75%, transparent 0);\n  \
                 --{prefix}-pattern-size: var(--{prefix}-space-8) var(--{prefix}-space-8);"
            ),
        ),
        (
            "diagonals",
            format!(
                "--{prefix}-pattern-image: repeating-linear-gradient(45deg, {ink} 0 {hairline}, transparent {hairline} var(--{prefix}-space-3));\n  \
                 --{prefix}-pattern-size: auto;"
            ),
        ),
    ];

    write_patterns(out, &patterns, prefix);
    write_ambient(out, spec, prefix);
}

/// The four geometric patterns, each declared as a value the layer reads.
///
/// # Why the attribute declares rather than paints
///
/// These were `:root[data-…-pattern="dots"] .noctua-fx { background-image: … }`
/// — the payload on a DESCENDANT. That shape cannot be scoped, and
/// `spec/theme.toml`'s own note about `syntax` already says why: two bare rules
/// of equal specificity both match, and **source order decides, not proximity**,
/// so a scope asking for `mesh` inside a page asking for `dots` would get
/// whichever the emitter happened to write last. `none` was the *absence* of a
/// rule, so a scope could not cancel what was above it at all.
///
/// Declaring an inherited custom property and reading it once, unconditionally,
/// is the shape `radius` and `glass` already have: the nearest declaring
/// ancestor wins by construction, and `none` becomes a real value.
fn write_patterns(out: &mut String, patterns: &[(&str, String); 4], prefix: &str) {
    for (name, rule) in patterns {
        writeln!(out, "[data-{prefix}-pattern=\"{name}\"] {{\n  {rule}\n}}\n").expect("write");
    }
    // The one that cancels. Absence used to mean this, and absence is also how
    // a subtree says "whatever is above me" — the same spelling for opposite
    // facts, which is the defect this whole shape removes.
    writeln!(
        out,
        "[data-{prefix}-pattern=\"none\"] {{\n           --{prefix}-pattern-image: none;\n           --{prefix}-pattern-size: auto;\n}}\n"
    )
    .expect("write");
}

/// The grain, and the one rule that paints the page.
///
/// There was an aurora here — two soft radial washes of the accent, the only
/// ambient effect that moved, and the only one that had to carry its own
/// contrast ceiling and its own probe test because its lobes were sized as a
/// share of the box and so covered the content inside a narrow scope. It was
/// removed on the developer's instruction, and with it went the last animation
/// on any page: the `drift` keyframes, the `--…-glow-motion` property, the
/// `[effect.blur] glow` and `[effect.duration]` tokens, and the reduced-motion
/// rule that existed to switch it off.
fn write_ambient(out: &mut String, spec: &Spec, prefix: &str) {
    let alpha = spec.effect.alpha.get("grain").copied().unwrap_or(6);

    // THE OPACITY IS BAKED INTO THE PICTURE, and that is the one thing this
    // shape costs. The grain used to be its own element, so it could carry an
    // `opacity` of its own; a background LAYER cannot, and `opacity` on the
    // scope element would fade the content as well. So the ceiling from
    // `spec/tokens.toml` is written into the SVG's own `opacity` — still one
    // number, still from the spec, just applied a step earlier.
    let grain = f64::from(alpha) / 100.0;
    writeln!(
        out,
        "/* Film grain. One inline SVG turbulence, so it costs no request, tiled\n\
        \x20  small rather than rasterized full-viewport — which is a real frame cost\n\
        \x20  on a mid-range phone. Always BEHIND content: high-frequency noise over\n\
        \x20  glyphs damages legibility far more than a flat wash does.\n\
        \x20\n\
        \x20  The {alpha}% ceiling is spec/tokens.toml's [effect.alpha] grain, carried\n\
        \x20  in the picture because a background layer has no opacity of its own. */\n\
        [data-{prefix}-grain=\"on\"] {{\n\
        \x20 --{prefix}-grain-image: url(\"data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg'%3E%3Cfilter id='n'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='0.8' numOctaves='3'/%3E%3CfeColorMatrix type='saturate' values='0'/%3E%3C/filter%3E%3Crect width='100%25' height='100%25' filter='url(%23n)' opacity='{grain}'/%3E%3C/svg%3E\");\n\
        }}\n\n\
        [data-{prefix}-grain=\"off\"] {{\n\
        \x20 --{prefix}-grain-image: none;\n\
        }}\n"
    )
    .expect("write");

    paint(out, prefix);

    writeln!(
        out,
        "/* NOTHING ON A PAGE MOVES ANY MORE, and that is the whole of what used\n\
        \x20  to be here. There was an `@keyframes {prefix}-drift` driving the\n\
        \x20  aurora's second background-position pair, a note explaining why its\n\
        \x20  duration could not live in `[motion.duration]` — an infinite\n\
        \x20  animation collapsed to `0ms` restarts every frame rather than\n\
        \x20  stopping — and a `prefers-reduced-motion` rule switching it off. The\n\
        \x20  aurora is gone, so all three went with it: a backdrop, a pattern and\n\
        \x20  a grain are three still layers, and reduced motion has nothing here\n\
        \x20  left to ask for. */\n"
    )
    .expect("write");
}

/// The one rule that paints all four ambient settings.
///
/// # Why one rule, on the element that carries the attributes
///
/// Every one of these used to be painted on a DESCENDANT, and two of them on
/// dedicated elements the site had to remember to render. That shape put the
/// four settings out of reach of a subtree — `spec/theme.toml` says why at
/// length — and it made `ThemeScope { pattern }` a prop that compiled, stamped
/// an attribute and painted nothing for anyone outside this repository.
///
/// Reading the declared values here instead means the nearest declaring
/// ancestor wins by construction, and it means there is no extra element: a
/// background is behind content by definition, so there is no `z-index`, no
/// `isolation`, and none of the stacking-order class of bug that cost the
/// effects layer three regressions.
///
/// # The order of the layers
///
/// First listed paints on top: grain over pattern, which is the order the
/// elements they replaced had. The `var()` fallbacks are the unscoped default a
/// scopable setting still owes — a `var()` naming a property nothing declared
/// makes the whole declaration invalid, which would take the backdrop down with
/// it.
fn paint(out: &mut String, prefix: &str) {
    writeln!(
        out,
        "/* The page, and every region that asked to be a page in miniature.\n\
        \x20\n\
        \x20  `background-attachment: fixed` on the root keeps what the fixed\n\
        \x20  elements this replaced did: a texture anchored to the viewport\n\
        \x20  rather than to the top of a very long document. A scope keeps the\n\
        \x20  default, because a scope's texture belongs to the scope's own box. */\n\
        {PAINTED} {{\n\
        \x20 background-color: var(--{prefix}-backdrop);\n\
        \x20 background-image:\n    \
        var(--{prefix}-grain-image, none),\n    \
        var(--{prefix}-pattern-image, none);\n\
        \x20 background-size:\n    \
        var(--{prefix}-space-48) var(--{prefix}-space-48),\n    \
        var(--{prefix}-pattern-size, auto);\n\
        }}\n\n\
        :root {{\n\
        \x20 background-attachment: fixed;\n\
        }}\n\n\
        /* FORCED COLORS REPLACES THE PALETTE, AND THESE ARE NOT PART OF IT. The\n\
        \x20  UA drops `background-image` for a gradient, but the grain is a data\n\
        \x20  URI and survives — so a reader who asked their system for two colors\n\
        \x20  would get two colors and a film of noise. All three go together. */\n\
        @media (forced-colors: active) {{\n\
        {PAINTED} {{\n\
        \x20   background-image: none;\n\
        \x20 }}\n\
        }}\n"
    )
    .expect("write");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shipped() -> Spec {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../spec/tokens.toml");
        noctua_design_spec::load(path).expect("the shipped spec")
    }

    fn model() -> Theme {
        let root = std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../.."));
        let vendor = noctua_design_spec::vendor::load(&root).expect("vendor");
        noctua_design_spec::load_theme(root.join("spec/theme.toml"), &vendor).expect("theme")
    }

    fn css() -> String {
        emit(&shipped(), &model()).remove(0).contents
    }

    /// The sheet with its comments removed.
    fn bare(css: &str) -> String {
        let mut out = String::new();
        let mut rest = css;
        while let Some(at) = rest.find("/*") {
            out.push_str(&rest[..at]);
            rest = rest[at..]
                .find("*/")
                .map_or("", |end| &rest[at + end + 2..]);
        }
        out.push_str(rest);
        out
    }

    /// Splits at the commas that separate a selector list, and no others.
    ///
    /// `:where(a, b)` is ONE selector with a comma in it. A naive split turns
    /// the scope list into four fragments that match nothing and quietly makes
    /// every assertion below vacuous.
    fn list(selector: &str) -> Vec<String> {
        let mut out = Vec::new();
        let mut depth = 0_i32;
        let mut current = String::new();
        for ch in selector.chars() {
            match ch {
                '(' => {
                    depth += 1;
                    current.push(ch);
                }
                ')' => {
                    depth -= 1;
                    current.push(ch);
                }
                ',' if depth == 0 => out.push(core::mem::take(&mut current)),
                _ => current.push(ch),
            }
        }
        out.push(current);
        out.into_iter()
            .map(|one| one.split_whitespace().collect::<Vec<_>>().join(" "))
            .filter(|one| !one.is_empty())
            .collect()
    }

    /// Every selector the sheet declares anything under, at-rules unwrapped.
    fn selectors(css: &str) -> Vec<String> {
        let css = bare(css);
        let mut out = Vec::new();
        let mut current = String::new();
        for ch in css.chars() {
            match ch {
                '{' => {
                    let selector = core::mem::take(&mut current);
                    // `@media`, `@keyframes` and `@supports` are conditions,
                    // not subjects. What is inside them is picked up on the
                    // next pass round this loop.
                    if !selector.trim_start().starts_with('@') {
                        out.extend(list(&selector));
                    }
                }
                '}' | ';' => current.clear(),
                _ => current.push(ch),
            }
        }
        out
    }

    /// The compounds of one selector — what a combinator separates.
    fn compounds(selector: &str) -> Vec<&str> {
        selector
            .split([' ', '>', '+', '~'])
            .filter(|part| !part.is_empty())
            .collect()
    }

    /// A SETTING'S REACH IS THE SELECTOR THAT CARRIES IT, AND NOTHING ELSE.
    ///
    /// Read off the emitted sheet and compared against `scope` in the spec,
    /// per setting, rather than against a list written out here — a list is
    /// what let `backdrop`, `pattern`, `glow` and `grain` be described as
    /// unscopable in four places while one of them quietly became scopable.
    ///
    /// Three properties, and each one is a defect that has actually shipped:
    ///
    /// 1. `scope = false` must have no rule whose subject can be below the
    ///    root, or the restriction is a comment rather than a mechanism.
    /// 2. `scope = true` must have at least one rule whose subject can be, or
    ///    it is a prop that compiles, stamps an attribute and paints nothing.
    /// 3. A scopable setting's payload must land on the element carrying the
    ///    attribute and never on a descendant. That is the shape `syntax`
    ///    cannot have: two bare rules of equal specificity both match, and
    ///    **source order decides rather than proximity**, so a scope inside a
    ///    differently-set page gets whichever the emitter wrote last.
    #[test]
    fn a_settings_reach_is_the_selector_that_carries_it() {
        let theme = model();
        let selectors = selectors(&css());
        for (name, setting) in &theme.setting {
            let Some(attribute) = &setting.attribute else {
                continue;
            };
            // `="` and not just `[`: a bare `[data-palette]` inside the scope
            // list is the CONDITION that a scope is present, not a rule about
            // any one palette. Matching it would make this test claim things
            // about settings whose payload is in another sheet entirely.
            let marker = format!("[{attribute}=");
            let carrying: Vec<&String> = selectors
                .iter()
                .filter(|one| one.contains(&marker))
                .collect();
            // A setting whose payload lives elsewhere — `syntax` in
            // dist/syntax.css, `font` in dist/fonts.css — says nothing here.
            if carrying.is_empty() {
                continue;
            }

            let mut below_root = 0_usize;
            for one in &carrying {
                let all = compounds(one);
                let (subject, rest) = all.split_last().expect("a selector has a compound");
                assert!(
                    !rest.iter().any(|part| part.contains(&marker)),
                    "{name}'s payload lands on a descendant of the element carrying \
                     it, so two of them cannot be nested: {one}"
                );
                if subject.contains(&marker) && !subject.contains(":root") {
                    below_root += 1;
                }
            }
            if setting.scope {
                assert!(
                    below_root > 0,
                    "{name} is scope = true but every rule is anchored at :root, \
                     so a subtree carrying the attribute gets nothing"
                );
            } else {
                assert_eq!(
                    below_root, 0,
                    "{name} is scope = false but has a rule that can match below \
                     the root, so it could be scoped by accident"
                );
            }
        }
    }

    /// Glass must emit dimensions, never a pre-mixed color.
    ///
    /// A `color-mix` on a `--nc-` token declared at `:root` freezes to the
    /// page's palette, so a themed component would show the page's frost. This
    /// is the general form of the bug `dist/scope.css` exists to fix.
    /// The bug this caught: every glass block was a bare attribute selector, so
    /// until a reader changed the setting nothing matched, the two tokens were
    /// undefined, and `blur(var(--noctua-glass-blur))` was invalid at
    /// computed-value time — which resolves to `none`. Every frosted surface in
    /// the system rendered flat on a first visit, silently.
    #[test]
    fn the_default_glass_level_is_declared_without_an_attribute() {
        let css = css();
        let theme = model();
        let level = &theme.setting["glass"].default;
        let spec = shipped();
        let blur = &spec.glass.blur[level];

        // Found by CONTENT, not by position. There is more than one
        // `:where(:root)` block now — the scrollbar treatment is another — and
        // a test that took the first one would break for a reason that has
        // nothing to do with what it is checking.
        let root = css
            .split(":where(:root) {")
            .skip(1)
            .filter_map(|rest| rest.split('}').next())
            .find(|block| block.contains("--noctua-glass-blur"))
            .expect("glass has no unscoped default, so a page with nothing stored has no glass");
        assert!(
            root.contains(&format!("--noctua-glass-blur: {blur}")),
            "the unscoped default is not the default level: {root}"
        );
    }

    #[test]
    fn glass_emits_no_pre_mixed_surface() {
        let css = css();
        for line in css.lines() {
            let line = line.trim();
            if line.starts_with("--noctua-glass-") {
                assert!(
                    !line.contains("--nc-"),
                    "a glass token bakes in a color token: {line}"
                );
            }
        }
    }

    /// Every preset must name the same radii, or a preset silently drops one
    /// and the component using it falls back to the base value.
    #[test]
    fn every_preset_covers_every_radius() {
        let spec = shipped();
        for (name, preset) in &spec.preset.radius {
            for base in spec.radius.keys() {
                assert!(
                    preset.contains_key(base),
                    "preset `{name}` does not set `{base}`"
                );
            }
        }
    }

    /// The default preset must equal the base radii exactly, or "no preset"
    /// and "the default preset" mean different things.
    #[test]
    fn the_default_preset_matches_the_base_radii() {
        let spec = shipped();
        let soft = spec.preset.radius.get("soft").expect("the soft preset");
        for (name, value) in &spec.radius {
            assert_eq!(soft.get(name), Some(value), "preset soft.{name}");
        }
    }

    /// Blackout must touch the page background and nothing else.
    #[test]
    fn every_backdrop_changes_only_the_page_background() {
        let css = css();
        // Every value's own rule, plus the base — the whole point of the
        // setting is that it moves ONE variable, so that not one of the pairs
        // the colour system measured moves with it.
        let blocks: Vec<&str> = css
            .split("-backdrop=\"")
            .skip(1)
            .map(|rest| rest.split('}').next().unwrap_or(""))
            .collect();
        assert!(
            blocks.len() >= 5,
            "the backdrop has {} rules, which is fewer than it has values",
            blocks.len()
        );
        for block in blocks {
            // A DECLARATION IS A LINE THAT STARTS WITH `--`. Filtering on `:`
            // instead catches the tail of the media query's own selector, and
            // the test then fails on its own parsing rather than on the CSS.
            for line in block.lines().map(str::trim).filter(|l| l.starts_with("--")) {
                assert!(
                    line.starts_with("--noctua-backdrop"),
                    "a backdrop sets something other than the page background: {line}"
                );
            }
        }
    }

    #[test]
    fn the_page_is_paper_in_light_and_the_palette_in_dark() {
        let css = css();
        // Two literals live in this file and nowhere else, and they are the two
        // ends a page can be. The base is white; dark asks the palette.
        assert!(
            css.contains("--noctua-backdrop: #fff;"),
            "light does not default to pure white"
        );
        assert!(
            css.contains("--noctua-backdrop: var(--nc-color-surface);"),
            "dark does not default to the palette's own page"
        );
        // And `black` is the only value that needs a mode to mean anything.
        for value in ["cool", "tinted", "soft", "softer"] {
            let rule = format!("\n[data-noctua-backdrop=\"{value}\"] {{");
            let at = css.find(&rule).expect("the value has no rule");
            let block = &css[at..css[at..].find('}').map_or(css.len(), |end| at + end)];
            assert!(
                !block.contains("data-theme") && !block.contains("prefers-color-scheme"),
                "{value} is written per mode, and its token already carries the mode"
            );
        }
    }

    /// Nothing a page is painted with moves.
    ///
    /// This used to assert the opposite half: that the aurora's infinite drift
    /// was not driven by a `--noctua-duration-` token, because those collapse
    /// under reduced motion and `animation-duration: 0ms` on an infinite
    /// animation is not "no motion" — it restarts every frame, and engines
    /// disagree about what renders.
    ///
    /// The aurora was removed, and with it the only animation in this sheet. So
    /// the test inverts rather than disappears: a backdrop, a pattern and a
    /// grain are three still layers, and the next person to animate one has to
    /// come back here and answer the reduced-motion question on purpose.
    #[test]
    fn nothing_a_page_is_painted_with_moves() {
        let css = css();
        for line in css.lines() {
            let line = line.trim_start();
            assert!(
                !line.starts_with("animation:") && !line.starts_with("@keyframes"),
                "the appearance sheet animates again: {line} — an infinite ambient \
                 animation needs an explicit reduced-motion stop, and a duration \
                 token cannot drive it because those collapse to 0ms"
            );
        }
    }

    /// The ink follows the foreground, and is mixed WHERE IT IS USED.
    ///
    /// It used to be a custom property holding `var(--nc-color-fg)`, which is
    /// the frozen indirection `gates::frozen_color_indirection` refuses: a
    /// `var()` resolves where it is declared, so the ink was the root's
    /// foreground however deep the element reading it was. It passed only
    /// because `dist/` is not walked. This asserts both halves — the strength
    /// is a token, and no colour is held in one.
    #[test]
    fn the_ink_is_a_wash_of_the_foreground_mixed_at_the_point_of_use() {
        let css = css();
        assert!(css.contains("--noctua-pattern-alpha: "));
        assert!(css.contains("color-mix(in oklab, var(--nc-color-fg) var(--noctua-pattern-alpha)"));
        for frozen in ["--noctua-pattern-ink:", "--noctua-dots-ink:"] {
            assert!(
                !css.contains(frozen),
                "{frozen} holds a `var(--nc-…)`, which freezes the colour to wherever it is declared"
            );
        }
    }

    /// Grain must cost no request.
    #[test]
    fn the_grain_is_inline() {
        let css = css();
        assert!(css.contains("data:image/svg+xml"));
        assert!(!css.contains(".svg\")"), "the grain was moved into a file");
        // The `#` in the SVG must stay percent-encoded, or a color gate would
        // see a hash where there is no color.
        assert!(css.contains("url(%23n)"), "the SVG fragment is not encoded");
    }

    #[test]
    fn braces_balance() {
        let css = css();
        assert_eq!(css.matches('{').count(), css.matches('}').count());
    }
}
