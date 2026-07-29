//! `dist/theme.css` — what every appearance setting actually does.
//!
//! Every rule here is keyed on an attribute the bootstrap already stamped, so
//! nothing needs JavaScript to apply and nothing can flash. A setting's visual
//! consequence is CSS; the script only writes the attribute.
//!
//! # Two selector shapes, and the difference is the whole scoping model
//!
//! **Scopable settings** use a bare attribute selector — `[data-noctua-radius]`
//! — so they reach any element, and a themed component carries its own corners
//! by plain custom-property inheritance.
//!
//! **Global settings** use `:root[…]` (0,2,0). Put `data-noctua-blackout` on a
//! `<div>` and nothing happens at all. The restriction is the selector, not a
//! comment somebody has to read.
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
        \x20  Scopable settings use a bare attribute selector and therefore reach any\n\
        \x20  element. Global ones use :root[...] — put one on a div and nothing\n\
        \x20  happens, which is the restriction being structural rather than\n\
        \x20  documented. */\n"
    )
    .expect("write");

    radius_presets(&mut out, spec, prefix);
    scrollbars(&mut out, prefix);
    glass(&mut out, spec, theme, prefix);
    blackout(&mut out, prefix);
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
/// system's brand colour by accident. Two neutrals and
/// `prefers-color-scheme` instead — which is also the only pair that reads at
/// 16 px against both a light and a dark browser chrome.
///
/// **This is a placeholder that behaves correctly, not a mark.** An aperture:
/// a ring and a pupil, which is as far as "noctua" should be taken without
/// somebody deciding to.
fn favicon() -> String {
    // An SVG has no line-comment form the gate reads, so the header is an XML
    // comment — and it is what exempts the two neutrals below from the colour
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
/// The thumb is mixed from the foreground colour at the point of use, so a
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
        "/* Glass. Dimensions only — the colour is mixed at the point of use, where\n\
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
        \x20  on one page, and a nested one is inert with nothing to report. */\n\
        .{prefix}-glass {{\n\
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
        .{prefix}-glass::after {{\n\
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

/// Pure black, and only where it cannot move a measured contrast pair.
fn blackout(out: &mut String, prefix: &str) {
    writeln!(
        out,
        "/* Blackout — the OLED setting.\n\
        \x20\n\
        \x20  It changes ONE variable, and only the document background reads it.\n\
        \x20  --nc-color-surface is untouched, so every pair the colour system\n\
        \x20  measured still holds, and every raised surface simply becomes more\n\
        \x20  visible against a darker page, which is the right direction.\n\
        \x20\n\
        \x20  :root[...] rather than [...]: this is a property of the page, and a\n\
        \x20  black island inside a themed component is not a theme. */\n"
    )
    .expect("write");

    writeln!(out, ":root {{").expect("write");
    writeln!(
        out,
        "  --{prefix}-page-background: var(--nc-color-surface);"
    )
    .expect("write");
    writeln!(out, "}}\n").expect("write");

    writeln!(out, "html {{").expect("write");
    writeln!(out, "  background: var(--{prefix}-page-background);").expect("write");
    writeln!(out, "}}\n").expect("write");

    // The one literal in this repository, and it is here because no ramp token
    // is pure black — `--nc-gray-1` is #030303, which still lights an OLED
    // pixel. Page background only; nothing that was measured reads it.
    writeln!(
        out,
        ":root[data-{prefix}-blackout=\"on\"][data-theme=\"dark\"] {{\n\
        \x20 --{prefix}-page-background: #000; /* allow-literal: blackout is defined as pure black, no ramp token is, and only the document background reads this */\n\
        }}\n"
    )
    .expect("write");

    writeln!(
        out,
        "@media (prefers-color-scheme: dark) {{\n\
        \x20 :root[data-{prefix}-blackout=\"on\"]:not([data-theme=\"light\"]) {{\n\
        \x20   --{prefix}-page-background: #000; /* allow-literal: as above, the system-preference half */\n\
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
        \x20  1. The ink is the FOREGROUND colour at {}%, so its polarity and its\n\
        \x20     palette follow automatically, and the worst case for any glyph is a\n\
        \x20     background pixel moved {}% toward the text colour. That bound is\n\
        \x20     measured by a test rather than asserted.\n\
        \x20  2. The layer is masked to nothing behind the reading column.\n\
        \x20\n\
        \x20  Geometry is percentages and spacing tokens, never raw lengths, so a\n\
        \x20  pattern scales with the scale. */\n",
        alpha("pattern", 4),
        alpha("pattern", 4)
    )
    .expect("write");

    writeln!(out, ":root {{").expect("write");
    writeln!(
        out,
        "  --{prefix}-pattern-ink: color-mix(in oklab, var(--nc-color-fg) {}%, transparent);",
        alpha("pattern", 4)
    )
    .expect("write");
    writeln!(
        out,
        "  --{prefix}-glow-ink: color-mix(in oklab, var(--nc-color-accent) {}%, transparent);",
        alpha("glow", 18)
    )
    .expect("write");
    writeln!(out, "  --{prefix}-grain-opacity: {}%;", alpha("grain", 6)).expect("write");
    writeln!(out, "}}\n").expect("write");

    // The patterns. Each is one `background-image` on the effect layer.
    let ink = format!("var(--{prefix}-pattern-ink)");
    let hairline = format!("var(--{prefix}-border-hairline)");
    let patterns: [(&str, String); 4] = [
        (
            "dots",
            format!(
                "background-image: radial-gradient(circle at 50% 50%, {ink} 0 9%, transparent 9%);\n  \
                 background-size: var(--{prefix}-space-6) var(--{prefix}-space-6);"
            ),
        ),
        (
            "mesh",
            format!(
                "background-image:\n    \
                 repeating-linear-gradient(to right, {ink} 0 {hairline}, transparent {hairline} var(--{prefix}-space-8)),\n    \
                 repeating-linear-gradient(to bottom, {ink} 0 {hairline}, transparent {hairline} var(--{prefix}-space-8));"
            ),
        ),
        (
            "checks",
            format!(
                "background-image: conic-gradient(from 90deg at 50% 50%, {ink} 0 25%, transparent 0 50%, {ink} 0 75%, transparent 0);\n  \
                 background-size: var(--{prefix}-space-8) var(--{prefix}-space-8);"
            ),
        ),
        (
            "diagonals",
            format!(
                "background-image: repeating-linear-gradient(45deg, {ink} 0 {hairline}, transparent {hairline} var(--{prefix}-space-3));"
            ),
        ),
    ];

    write_patterns(out, &patterns, prefix);
    write_ambient(out, spec, prefix);
}

/// The four geometric patterns, one `background-image` each.
fn write_patterns(out: &mut String, patterns: &[(&str, String); 4], prefix: &str) {
    for (name, rule) in patterns {
        writeln!(
            out,
            ":root[data-{prefix}-pattern=\"{name}\"] .{prefix}-fx {{\n  {rule}\n}}\n"
        )
        .expect("write");
    }
}

/// The aurora, the grain, and the ambient-motion switch.
fn write_ambient(out: &mut String, spec: &Spec, prefix: &str) {
    writeln!(
        out,
        "/* The aurora. Derived from the accent, so it follows the palette without\n\
        \x20  naming a colour. */\n\
        :root[data-{prefix}-glow=\"on\"] .{prefix}-fx::before {{\n\
        \x20 content: \"\";\n\
        \x20 position: absolute;\n\
        \x20 inset: 0;\n\
        \x20 background-image:\n\
        \x20   radial-gradient(60% 50% at 85% 12%, var(--{prefix}-glow-ink) 0, transparent 70%),\n\
        \x20   radial-gradient(50% 45% at 15% 88%, var(--{prefix}-glow-ink) 0, transparent 70%);\n\
        }}\n"
    )
    .expect("write");

    writeln!(
        out,
        "/* Film grain. One inline SVG turbulence, so it costs no request, tiled\n\
        \x20  small rather than rasterized full-viewport — which is a real frame cost\n\
        \x20  on a mid-range phone. Always BEHIND content: high-frequency noise over\n\
        \x20  glyphs damages legibility far more than a flat wash does. */\n\
        :root[data-{prefix}-grain=\"on\"] .{prefix}-grain {{\n\
        \x20 opacity: var(--{prefix}-grain-opacity);\n\
        \x20 background-image: url(\"data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg'%3E%3Cfilter id='n'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='0.8' numOctaves='3'/%3E%3CfeColorMatrix type='saturate' values='0'/%3E%3C/filter%3E%3Crect width='100%25' height='100%25' filter='url(%23n)'/%3E%3C/svg%3E\");\n\
        \x20 background-size: var(--{prefix}-space-48) var(--{prefix}-space-48);\n\
        }}\n"
    )
    .expect("write");

    let drift = spec
        .effect
        .duration
        .get("drift")
        .cloned()
        .unwrap_or_else(|| "42s".to_owned());
    let breathe = spec
        .effect
        .duration
        .get("breathe")
        .cloned()
        .unwrap_or_else(|| "18s".to_owned());

    writeln!(
        out,
        "/* Ambient motion.\n\
        \x20\n\
        \x20  These durations are in seconds and carry NO reduced-motion pair, unlike\n\
        \x20  every other duration in this system. That is deliberate:\n\
        \x20  `animation-duration: 0ms` on an infinite animation is not \"no motion\" —\n\
        \x20  it completes and restarts every frame, and engines disagree about what\n\
        \x20  renders. Collapsing the token would produce undefined behaviour,\n\
        \x20  silently, in exactly the population the token exists to protect.\n\
        \x20\n\
        \x20  So reduced motion is one explicit `animation: none`, and every animated\n\
        \x20  layer has a designed static end-state.\n\
        \x20\n\
        \x20  Motion belongs to the effect that has it, and there is no separate\n\
        \x20  switch for it. There used to be: `data-{prefix}-animate`, which only\n\
        \x20  ever appeared ANDed with glow or grain. Both default to off, so on a\n\
        \x20  default page the control changed an attribute that matched no selector\n\
        \x20  and did visibly nothing — which is worse than a missing control,\n\
        \x20  because it teaches the reader that the panel lies. An aurora drifts\n\
        \x20  because it is an aurora. */\n\
        @keyframes {prefix}-drift {{\n\
        \x20 from {{ transform: translate3d(0, 0, 0); }}\n\
        \x20 50%  {{ transform: translate3d(2%, -1.5%, 0); }}\n\
        \x20 to   {{ transform: translate3d(0, 0, 0); }}\n\
        }}\n\n\
        :root[data-{prefix}-glow=\"on\"] .{prefix}-fx::before {{\n\
        \x20 animation: {prefix}-drift {drift} ease-in-out infinite;\n\
        \x20 will-change: transform;\n\
        }}\n\n\
        :root[data-{prefix}-grain=\"on\"] .{prefix}-grain {{\n\
        \x20 animation: {prefix}-drift {breathe} steps(6) infinite;\n\
        }}\n\n\
        /* A page setting may only ever DISABLE motion. Someone whose system asked\n\
        \x20  for none must never have it re-enabled by a page. */\n\
        @media (prefers-reduced-motion: reduce) {{\n\
        \x20 .{prefix}-fx,\n\
        \x20 .{prefix}-fx::before,\n\
        \x20 .{prefix}-grain {{\n\
        \x20   animation: none !important;\n\
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

    /// A global setting must be unreachable from a subtree.
    ///
    /// Asserted on the selector rather than trusted: `:root[…]` is the whole
    /// mechanism, and a refactor to a bare attribute would silently make
    /// blackout scopable and produce black islands inside components.
    #[test]
    fn global_settings_are_root_only() {
        let css = css();
        for global in ["blackout", "pattern", "glow", "grain"] {
            let bare = format!("\n[data-noctua-{global}");
            assert!(
                !css.contains(&bare),
                "data-noctua-{global} is matched without :root, so it could be scoped"
            );
            assert!(
                css.contains(&format!(":root[data-noctua-{global}")),
                "data-noctua-{global} has no :root-anchored rule"
            );
        }
    }

    /// A scopable setting must be reachable from a subtree.
    #[test]
    fn scopable_settings_use_a_bare_attribute() {
        let css = css();
        for scopable in ["radius", "glass"] {
            assert!(
                css.contains(&format!("[data-noctua-{scopable}=")),
                "data-noctua-{scopable} has no rule"
            );
            assert!(
                !css.contains(&format!(":root[data-noctua-{scopable}=")),
                "data-noctua-{scopable} is :root-anchored, so a component cannot carry its own"
            );
        }
    }

    /// Glass must emit dimensions, never a pre-mixed colour.
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
                    "a glass token bakes in a colour token: {line}"
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
    fn blackout_changes_only_the_page_background() {
        let css = css();
        let blocks: Vec<&str> = css
            .split("blackout=\"on\"]")
            .skip(1)
            .map(|rest| rest.split('}').next().unwrap_or(""))
            .collect();
        assert!(!blocks.is_empty(), "blackout has no rule");
        for block in blocks {
            for line in block.lines().map(str::trim).filter(|l| l.contains(':')) {
                assert!(
                    line.starts_with("--noctua-page-background") || line.starts_with(':'),
                    "blackout sets something other than the page background: {line}"
                );
            }
        }
    }

    /// Ambient animation must not be driven by a collapsing duration token.
    #[test]
    fn ambient_motion_does_not_use_a_reduced_motion_pair() {
        let css = css();
        let keyframe_users: Vec<&str> = css
            .lines()
            .filter(|line| line.trim_start().starts_with("animation:"))
            .collect();
        assert!(!keyframe_users.is_empty(), "nothing is animated");
        for line in keyframe_users {
            assert!(
                !line.contains("--noctua-duration-"),
                "an infinite animation is driven by a collapsing token, which is \
                 undefined at 0ms: {line}"
            );
        }
        assert!(
            css.contains("animation: none !important"),
            "there is no explicit reduced-motion stop"
        );
    }

    /// The pattern ink follows the foreground, so its polarity and palette are
    /// automatic rather than declared twice.
    #[test]
    fn the_pattern_ink_is_a_wash_of_the_foreground() {
        assert!(css().contains("--noctua-pattern-ink: color-mix(in oklab, var(--nc-color-fg)"));
    }

    /// Grain must cost no request.
    #[test]
    fn the_grain_is_inline() {
        let css = css();
        assert!(css.contains("data:image/svg+xml"));
        assert!(!css.contains(".svg\")"), "the grain was moved into a file");
        // The `#` in the SVG must stay percent-encoded, or a colour gate would
        // see a hash where there is no colour.
        assert!(css.contains("url(%23n)"), "the SVG fragment is not encoded");
    }

    #[test]
    fn braces_balance() {
        let css = css();
        assert_eq!(css.matches('{').count(), css.matches('}').count());
    }
}
