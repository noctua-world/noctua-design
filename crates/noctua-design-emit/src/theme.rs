//! The theme model, in every form the system needs it.
//!
//! One table in `spec/theme.toml` produces all of this:
//!
//! | Output | For |
//! |---|---|
//! | `crates/noctua-design-theme/src/lib.rs` | the Dioxus package |
//! | `packages/tokens/theme.js` + `.d.ts` | the React package |
//! | `dist/theme.json` | anything with neither toolchain — including the tests |
//! | `dist/bootstrap.js` + `.html` | the inline script that runs before first paint |
//! | `dist/base.html` | the base links, in the one order that works |
//!
//! Generating all five is what makes prop-surface parity mechanical: neither
//! package writes a value, a default or a key name down, so neither can
//! disagree with the other about one.

use std::fmt::Write as _;

use noctua_design_spec::{Theme, Vendor};

use crate::{CommentStyle, EmittedFile, THEME_SPEC, camel, header_from, identifier};

/// The colour sheets `base.html` links, shipped beside it.
///
/// **Why they are here and not only in `packages/tokens/vendor/`.** `base.html`
/// links `tokens/css/ramp.css` and one `theme-<palette>.css`, relative to
/// itself, and `spec/theme.toml`'s `stylesheet_base` — the same value the
/// generated `STYLESHEET_BASE` carries — is where the bootstrap looks for the
/// rest when a reader switches palette. A consumer who installed
/// from a registry has `dist/` and nothing else — so without these the first
/// three links 404 and the page has **no palette at all**, which is the failure
/// the whole scope-anchored default is arranged around.
///
/// Copied, never rewritten: they are `noctua-colors`' output, and this system
/// vendors that output rather than restating it (`NOCTUA.md §8.6`).
#[must_use]
pub fn palettes(theme: &Theme, sheets: &[(String, String)]) -> Vec<EmittedFile> {
    let base = &theme.meta.stylesheet_base;
    sheets
        .iter()
        .map(|(name, contents)| EmittedFile::new(format!("dist/{base}/{name}"), contents.clone()))
        .collect()
}

/// Emits every form of the theme model.
#[must_use]
pub fn emit(theme: &Theme, vendor: &Vendor) -> Vec<EmittedFile> {
    vec![
        EmittedFile::new(
            "crates/noctua-design-theme/src/lib.rs",
            rust_model(theme, vendor),
        ),
        EmittedFile::new("packages/tokens/theme.js", js_model(theme, vendor)),
        EmittedFile::new("packages/tokens/theme.d.ts", dts_model(theme, vendor)),
        EmittedFile::new("dist/theme.json", json_model(theme, vendor)),
        EmittedFile::new("dist/bootstrap.js", bootstrap(theme, vendor)),
        EmittedFile::new("dist/bootstrap.html", bootstrap_fragment(theme, vendor)),
        EmittedFile::new("dist/base.html", base_links(theme, vendor)),
    ]
}

/// Every attribute a scope element can carry, marker first.
///
/// One function, read by both models, so "what a scope is made of" is one fact
/// rather than two that agree today. The order is the marker and then the
/// scopable settings in the order the spec declares them; a setting with no
/// attribute of its own — `accent` and `saturation`, which reach the DOM
/// through `palette` — contributes nothing.
fn scope_attribute_names(theme: &Theme) -> Vec<String> {
    let mut names = vec![theme.meta.scope_attribute.clone()];
    names.extend(stamped(theme).filter_map(|(_, setting)| setting.attribute.clone()));
    names
}

/// The scopable settings that actually reach the DOM, in the spec's order.
///
/// `scope_attribute_names` and the generated `ScopeAttributes` are the same
/// list twice — the names a scope carries and the fields that hold them — so
/// they read it from here rather than each filtering `scopable` themselves.
fn stamped(theme: &Theme) -> impl Iterator<Item = (&String, &noctua_design_spec::Setting)> {
    theme
        .scopable()
        .filter(|(_, setting)| setting.attribute.is_some())
}

// --- Rust ------------------------------------------------------------------

fn rust_model(theme: &Theme, vendor: &Vendor) -> String {
    let mut out = header_from(CommentStyle::Line("//!"), THEME_SPEC);
    out.push_str(
        "//!\n\
         //! # The theme model\n\
         //!\n\
         //! Every enum, default, storage key and DOM attribute, generated from\n\
         //! `spec/theme.toml`. The TypeScript module beside it is generated from\n\
         //! the same table, so the two cannot disagree.\n\
         //!\n\
         //! `no_std`: this is data and string constants, and a consumer painting\n\
         //! in a `no_std` context should not be excluded from knowing its own\n\
         //! theme.\n\
         \n\
         #![no_std]\n\n",
    );

    for (name, setting) in &theme.setting {
        rust_enum(&mut out, name, setting, &setting.resolved_values(vendor));
    }

    rust_constants(&mut out, theme, vendor);
    out
}

/// One setting's enum, with its values, default, key and parser.
fn rust_enum(
    out: &mut String,
    name: &str,
    setting: &noctua_design_spec::Setting,
    values: &[String],
) {
    {
        let type_name = pascal(name);

        writeln!(out, "/// The `{name}` setting.").expect("write");
        writeln!(out, "///").expect("write");
        writeln!(
            out,
            "/// Stored under `{}`{}.",
            setting.key,
            setting
                .attribute
                .as_ref()
                .map_or_else(String::new, |a| format!(", stamped as `{a}`"))
        )
        .expect("write");
        writeln!(out, "#[derive(Debug, Clone, Copy, PartialEq, Eq)]").expect("write");
        writeln!(out, "pub enum {type_name} {{").expect("write");
        for value in values {
            writeln!(out, "    /// `{value}`.").expect("write");
            writeln!(out, "    {},", pascal(value)).expect("write");
        }
        writeln!(out, "}}\n").expect("write");
    }

    rust_impl(out, name, setting, values);
}

/// One setting's `impl` block: `ALL`, `DEFAULT`, `KEY`, `as_str`, `parse`.
fn rust_impl(
    out: &mut String,
    name: &str,
    setting: &noctua_design_spec::Setting,
    values: &[String],
) {
    {
        let type_name = pascal(name);
        writeln!(out, "impl {type_name} {{").expect("write");
        writeln!(out, "    /// Every value, in declaration order.").expect("write");
        writeln!(
            out,
            "    pub const ALL: [Self; {}] = [{}];",
            values.len(),
            values
                .iter()
                .map(|v| format!("Self::{}", pascal(v)))
                .collect::<Vec<_>>()
                .join(", ")
        )
        .expect("write");
        writeln!(out, "\n    /// The value in force when nothing is stored.").expect("write");
        writeln!(
            out,
            "    pub const DEFAULT: Self = Self::{};",
            pascal(&setting.default)
        )
        .expect("write");
        writeln!(out, "\n    /// The `localStorage` key.").expect("write");
        writeln!(
            out,
            "    pub const KEY: &'static str = \"{}\";",
            setting.key
        )
        .expect("write");

        if let Some(attribute) = &setting.attribute {
            writeln!(out, "\n    /// The DOM attribute this is stamped as.").expect("write");
            writeln!(
                out,
                "    pub const ATTRIBUTE: &'static str = \"{attribute}\";"
            )
            .expect("write");
        }

        writeln!(
            out,
            "\n    /// The value as it appears in storage and the DOM."
        )
        .expect("write");
        writeln!(out, "    #[must_use]").expect("write");
        writeln!(out, "    pub const fn as_str(self) -> &'static str {{").expect("write");
        writeln!(out, "        match self {{").expect("write");
        for value in values {
            writeln!(out, "            Self::{} => \"{value}\",", pascal(value)).expect("write");
        }
        writeln!(out, "        }}\n    }}").expect("write");

        writeln!(
            out,
            "\n    /// Parses a stored value, rejecting anything unrecognised.\n\
             \x20   ///\n\
             \x20   /// A stored value reaches an `href` and a DOM attribute, so it is\n\
             \x20   /// never trusted — a corrupt or hostile entry must degrade to the\n\
             \x20   /// default rather than be stamped."
        )
        .expect("write");
        writeln!(out, "    #[must_use]").expect("write");
        writeln!(out, "    pub fn parse(value: &str) -> Option<Self> {{").expect("write");
        writeln!(out, "        match value {{").expect("write");
        for value in values {
            writeln!(
                out,
                "            \"{value}\" => Some(Self::{}),",
                pascal(value)
            )
            .expect("write");
        }
        writeln!(out, "            _ => None,\n        }}\n    }}").expect("write");

        if let Some(omit) = &setting.omit_when {
            writeln!(
                out,
                "\n    /// Whether this value stamps NO attribute.\n\
                 \x20   ///\n\
                 \x20   /// `{omit}` is the absence of the attribute, not a value of it: the\n\
                 \x20   /// stylesheet resolves it in the cascade, where the answer exists\n\
                 \x20   /// before anything is painted."
            )
            .expect("write");
            writeln!(out, "    #[must_use]").expect("write");
            writeln!(out, "    pub const fn is_omitted(self) -> bool {{").expect("write");
            writeln!(out, "        matches!(self, Self::{})", pascal(omit)).expect("write");
            writeln!(out, "    }}").expect("write");
        }

        writeln!(out, "}}\n").expect("write");

        writeln!(out, "impl Default for {type_name} {{").expect("write");
        writeln!(
            out,
            "    fn default() -> Self {{\n        Self::DEFAULT\n    }}\n}}\n"
        )
        .expect("write");
    }
}

/// The scope attribute, the stylesheet base, and the href table — so a provider
/// builds an href the same way the bootstrap does.
fn rust_constants(out: &mut String, theme: &Theme, vendor: &Vendor) {
    writeln!(
        out,
        "/// The attribute a themed subtree carries.\n\
         ///\n\
         /// Its presence is what makes the semantic contract resolve again at\n\
         /// that element — see `dist/scope.css`.\n\
         pub const SCOPE_ATTRIBUTE: &str = \"{}\";\n",
        theme.meta.scope_attribute
    )
    .expect("write");

    writeln!(
        out,
        "/// Where palette stylesheets are served from, relative to the page.\n\
         pub const STYLESHEET_BASE: &str = \"{}\";\n",
        theme.meta.stylesheet_base
    )
    .expect("write");

    // One arm per palette, each carrying the whole href.
    //
    // A `format!` would need `alloc`, and building the string at each call site
    // is how the bootstrap and the runtime end up spelling it differently — a
    // 404 before paint, or the same sheet appended twice. A match is the only
    // version that is `no_std`, allocation-free, and impossible to misspell.
    writeln!(
        out,
        "/// The stylesheet that binds a palette to `[data-palette=\"…\"]`.\n\
         ///\n\
         /// Every palette has one, the default included — `noctua-colors` 0.2.1\n\
         /// added the last of them. Before that the default was `:root`-only,\n\
         /// which made it the one palette of thirty-nine that could not be\n\
         /// applied to a subtree.\n\
         #[must_use]\n\
         pub const fn stylesheet_href(palette: Palette) -> &'static str {{\n\
         \x20   match palette {{"
    )
    .expect("write");
    for name in &vendor.themes {
        writeln!(
            out,
            "        Palette::{} => \"{}/theme-{name}.css\",",
            pascal(name),
            theme.meta.stylesheet_base
        )
        .expect("write");
    }
    writeln!(out, "    }}\n}}").expect("write");

    rust_scope(out, theme);
}

/// The scope resolver — the one function that has to be right.
///
/// Every gotcha the scoped-theme design has to survive lives here, so it lives
/// in exactly one place and is tested without a browser or a framework. The
/// Dioxus and React wrappers call it and add nothing.
fn rust_scope(out: &mut String, theme: &Theme) {
    let scopable: Vec<(&String, &noctua_design_spec::Setting)> = theme.scopable().collect();

    writeln!(
        out,
        "\n/// What a themed subtree asks for. `None` means \"inherit\"."
    )
    .expect("write");
    writeln!(out, "#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]").expect("write");
    writeln!(out, "pub struct ScopeRequest {{").expect("write");
    for (name, _) in &scopable {
        writeln!(out, "    /// The `{name}` this scope asks for.").expect("write");
        writeln!(
            out,
            "    pub {}: Option<{}>,",
            identifier(name),
            pascal(name)
        )
        .expect("write");
    }
    writeln!(out, "}}\n").expect("write");

    writeln!(
        out,
        "/// What the surrounding page or scope already resolved to."
    )
    .expect("write");
    writeln!(out, "#[derive(Debug, Clone, Copy, PartialEq, Eq)]").expect("write");
    writeln!(out, "pub struct Resolved {{").expect("write");
    for (name, _) in &scopable {
        writeln!(out, "    /// The inherited `{name}`.").expect("write");
        writeln!(out, "    pub {}: {},", identifier(name), pascal(name)).expect("write");
    }
    writeln!(out, "}}\n").expect("write");

    writeln!(out, "impl Default for Resolved {{").expect("write");
    writeln!(out, "    fn default() -> Self {{\n        Self {{").expect("write");
    for (name, _) in &scopable {
        writeln!(
            out,
            "            {}: {}::DEFAULT,",
            identifier(name),
            pascal(name)
        )
        .expect("write");
    }
    writeln!(out, "        }}\n    }}\n}}\n").expect("write");

    rust_resolver(out, theme, &scopable);
}

/// `resolve`, `is_noop`, and the attribute list they produce.
fn rust_resolver(
    out: &mut String,
    theme: &Theme,
    scopable: &[(&String, &noctua_design_spec::Setting)],
) {
    writeln!(out, "impl ScopeRequest {{").expect("write");
    writeln!(
        out,
        "    /// Resolves this request against what it inherits."
    )
    .expect("write");
    writeln!(out, "    #[must_use]").expect("write");
    writeln!(
        out,
        "    pub fn resolve(self, inherited: Resolved) -> Resolved {{\n        Resolved {{"
    )
    .expect("write");
    for (name, _) in scopable {
        let id = identifier(name);
        writeln!(
            out,
            "            {id}: match self.{id} {{ Some(v) => v, None => inherited.{id} }},"
        )
        .expect("write");
    }
    writeln!(out, "        }}\n    }}\n").expect("write");

    // The no-op check: a scope that changes nothing must emit nothing.
    writeln!(
        out,
        "    /// Whether this scope changes anything at all.\n\
         \x20   ///\n\
         \x20   /// A scope that matches its surroundings must emit no attributes: the\n\
         \x20   /// semantic contract is 1,767 declarations, and re-resolving them for a\n\
         \x20   /// subtree that wanted the page's own theme is pure cost.\n\
         \x20   #[must_use]\n\
         \x20   pub fn is_noop(self, inherited: Resolved) -> bool {{\n\
         \x20       self.resolve(inherited) == inherited\n\
         \x20   }}\n\
         }}\n"
    )
    .expect("write");

    rust_attributes(out, theme);
}

/// `scope_attributes` and the struct it returns, one field per stamped setting.
fn rust_attributes(out: &mut String, theme: &Theme) {
    writeln!(
        out,
        "/// Every attribute a scope element must carry, as `(name, value)`.\n\
         ///\n\
         /// # All of them go on ONE element\n\
         ///\n\
         /// The color system's forced-mode selector is **compound and\n\
         /// same-element**:\n\
         ///\n\
         /// ```css\n\
         /// [data-palette=\"blue-vivid\"][data-theme=\"dark\"] {{ … }}\n\
         /// ```\n\
         ///\n\
         /// Split the two across a wrapper and a child and that selector matches\n\
         /// nothing — the child falls through to the *default* palette's unanchored\n\
         /// `[data-theme=\"dark\"]` and silently paints the wrong palette.\n\
         ///\n\
         /// # Mode and palette travel together\n\
         ///\n\
         /// For the same reason, stamping a mode without a palette is never right:\n\
         /// the compound needs both halves. So the palette is always emitted when\n\
         /// the scope is not a no-op, even when it equals the inherited one.\n\
         #[must_use]\n\
         pub fn scope_attributes(\n\
         \x20   request: ScopeRequest,\n\
         \x20   inherited: Resolved,\n\
         ) -> ::core::option::Option<ScopeAttributes> {{\n\
         \x20   if request.is_noop(inherited) {{\n\
         \x20       return ::core::option::Option::None;\n\
         \x20   }}\n\
         \x20   let r = request.resolve(inherited);\n\
         \x20   ::core::option::Option::Some(ScopeAttributes {{\n\
         \x20       scope: SCOPE_ATTRIBUTE,"
    )
    .expect("write");
    // ONE PASS PER STAMPED SETTING, FROM THE SPEC. The list used to be written
    // out here, five names deep, while everything around it came from
    // `scopable` — so a setting that became scopable compiled, stamped nothing,
    // and left a scope that resolved the semantic contract for a value it
    // never carried.
    for (name, setting) in stamped(theme) {
        let id = identifier(name);
        match setting.omit_when.as_deref() {
            // A value spelled as the ABSENCE of its attribute. Stamping the
            // word would be worse than stamping nothing: no stylesheet
            // understands it, and the cascade already answers.
            Some(omit) => writeln!(
                out,
                "        // `{omit}` stamps nothing: the cascade resolves it, and an\n\
                 \x20       // attribute no stylesheet understands would be worse than none.\n\
                 \x20       {id}: if r.{id}.is_omitted() {{ ::core::option::Option::None }}\n\
                 \x20             else {{ ::core::option::Option::Some(r.{id}.as_str()) }},"
            ),
            None => writeln!(out, "        {id}: r.{id}.as_str(),"),
        }
        .expect("write");
    }
    writeln!(
        out,
        "    }})\n\
         }}\n\n\
         /// The attributes, ready to be stamped on one element.\n\
         #[derive(Debug, Clone, Copy, PartialEq, Eq)]\n\
         pub struct ScopeAttributes {{\n\
         \x20   /// The marker that makes the semantic contract resolve here.\n\
         \x20   pub scope: &'static str,"
    )
    .expect("write");
    for (name, setting) in stamped(theme) {
        let id = identifier(name);
        let attribute = setting.attribute.as_deref().expect("stamped");
        match setting.omit_when.as_deref() {
            Some(omit) => writeln!(
                out,
                "    /// `{attribute}`, or `None` when the scope follows the {omit}.\n\
                 \x20   pub {id}: ::core::option::Option<&'static str>,"
            ),
            None => writeln!(
                out,
                "    /// `{attribute}`.\n\
                 \x20   pub {id}: &'static str,"
            ),
        }
        .expect("write");
    }
    writeln!(out, "}}").expect("write");

    // The same lists the JavaScript model publishes, from the same table. A
    // consumer that stamps a scope at runtime needs to be able to un-stamp it,
    // and reconstructing either set by hand is what went wrong.
    name_const(
        out,
        "/// Every attribute a scope element can carry, including the marker.\n\
         ///\n\
         /// Removing all of them is how a scope is taken back off. A scope left\n\
         /// half-removed keeps re-resolving the semantic contract at its own\n\
         /// element — against the system preference rather than the page — which\n\
         /// is worse than one that never resolved it at all.",
        "SCOPE_ATTRIBUTES",
        &scope_attribute_names(theme),
    );
    name_const(
        out,
        "/// Every setting a scope can carry, in the spec's own order.\n\
         ///\n\
         /// A setting absent here is `scope = false`: it is emitted at `:root`\n\
         /// and no element below it can hold a different value. Sealing a\n\
         /// subtree off from the page means carrying every one of these, so a\n\
         /// consumer that reconstructs the list by hand stops sealing the day a\n\
         /// setting is added.",
        "SCOPABLE_SETTINGS",
        &theme
            .scopable()
            .map(|(name, _)| name.clone())
            .collect::<Vec<_>>(),
    );
}

/// One `pub const NAME: [&str; N]`, documented.
fn name_const(out: &mut String, doc: &str, name: &str, values: &[String]) {
    let list = values
        .iter()
        .map(|value| format!("\"{value}\""))
        .collect::<Vec<_>>()
        .join(", ");
    writeln!(
        out,
        "\n\n{doc}\npub const {name}: [&str; {}] = [{list}];",
        values.len()
    )
    .expect("write");
}

// --- TypeScript ------------------------------------------------------------

fn js_model(theme: &Theme, vendor: &Vendor) -> String {
    let mut out = header_from(CommentStyle::Line("//"), THEME_SPEC);
    writeln!(
        out,
        "\n// Generated from spec/theme.toml, the same table the Rust"
    )
    .expect("write");
    writeln!(out, "// model comes from. The two cannot disagree.\n").expect("write");

    for (name, setting) in &theme.setting {
        js_table(&mut out, name, setting, &setting.resolved_values(vendor));
    }

    js_functions(&mut out, theme);
    out
}

/// One setting's table: its values, default, key, attribute and scope flag.
fn js_table(
    out: &mut String,
    name: &str,
    setting: &noctua_design_spec::Setting,
    values: &[String],
) {
    {
        writeln!(
            out,
            "export const {} = {{",
            camel(&format!("{name}-values"))
        )
        .expect("write");
        writeln!(
            out,
            "  values: [{}],",
            values
                .iter()
                .map(|v| format!("\"{v}\""))
                .collect::<Vec<_>>()
                .join(", ")
        )
        .expect("write");
        writeln!(out, "  default: \"{}\",", setting.default).expect("write");
        writeln!(out, "  key: \"{}\",", setting.key).expect("write");
        match &setting.attribute {
            Some(attribute) => writeln!(out, "  attribute: \"{attribute}\",").expect("write"),
            None => writeln!(out, "  attribute: null,").expect("write"),
        }
        match &setting.omit_when {
            Some(omit) => writeln!(out, "  omitWhen: \"{omit}\",").expect("write"),
            None => writeln!(out, "  omitWhen: null,").expect("write"),
        }
        writeln!(out, "  scope: {},", setting.scope).expect("write");
        writeln!(out, "}};\n").expect("write");
    }
}

/// The scope attribute, the href builder and the resolver.
fn js_functions(out: &mut String, theme: &Theme) {
    writeln!(
        out,
        "export const scopeAttribute = \"{}\";",
        theme.meta.scope_attribute
    )
    .expect("write");
    writeln!(
        out,
        "export const stylesheetBase = \"{}\";\n",
        theme.meta.stylesheet_base
    )
    .expect("write");

    writeln!(
        out,
        "/**\n\
         \x20* The stylesheet that binds a palette to `[data-palette=\"…\"]`.\n\
         \x20*\n\
         \x20* Built here rather than at each call site so the bootstrap and the\n\
         \x20* runtime switcher cannot spell it differently.\n\
         \x20*/\n\
         export function stylesheetHref(palette) {{\n\
         \x20 return stylesheetBase + \"/theme-\" + palette + \".css\";\n\
         }}\n"
    )
    .expect("write");

    // The same resolver as the Rust side, generated from the same table so the
    // two cannot drift. Every comment here is a rule that has a matching test.
    writeln!(
        out,
        "/**\n\
         \x20* Resolves a scope request against what it inherits.\n\
         \x20*\n\
         \x20* `undefined` means inherit. A nested scope resolves against its PARENT\n\
         \x20* scope's result, not against the page.\n\
         \x20*/\n\
         export function resolveScope(request, inherited) {{\n\
         \x20 const out = {{}};\n\
         \x20 for (const name of scopable_) {{\n\
         \x20   out[name] = request[name] == null ? inherited[name] : request[name];\n\
         \x20 }}\n\
         \x20 return out;\n\
         }}\n\n\
         /**\n\
         \x20* Every attribute a scope element must carry, or null when the scope\n\
         \x20* changes nothing.\n\
         \x20*\n\
         \x20* ALL OF THEM GO ON ONE ELEMENT. The color system's forced-mode selector\n\
         \x20* is compound and same-element — `[data-palette=\"x\"][data-theme=\"dark\"]`\n\
         \x20* — so splitting them across a wrapper and a child matches nothing, and\n\
         \x20* the child silently falls through to the default palette.\n\
         \x20*\n\
         \x20* For the same reason the palette is always stamped, even when it equals\n\
         \x20* the inherited one: the compound needs both halves.\n\
         \x20*\n\
         \x20* A no-op scope returns null. The semantic contract is 1,767\n\
         \x20* declarations, and re-resolving them for a subtree that asked for what\n\
         \x20* it already had is pure cost.\n\
         \x20*/\n\
         export function scopeAttributes(request, inherited) {{\n\
         \x20 const r = resolveScope(request, inherited);\n\
         \x20 if (scopable_.every((name) => r[name] === inherited[name])) return null;\n\
         \x20\n\
         \x20 const out = {{ [scopeAttribute]: \"\" }};\n\
         \x20 for (const name of scopable_) {{\n\
         \x20   const model = model_[name];\n\
         \x20   if (model.attribute == null) continue;\n\
         \x20   // `system` is the absence of the attribute, not a value of it.\n\
         \x20   if (model.omitWhen != null && r[name] === model.omitWhen) continue;\n\
         \x20   out[model.attribute] = r[name];\n\
         \x20 }}\n\
         \x20 return out;\n\
         }}\n"
    )
    .expect("write");

    // EVERY ATTRIBUTE A SCOPE CAN CARRY, AS A LIST.
    //
    // `scopeAttributes` says which to SET. Nothing said which to remove, so a
    // caller taking a scope back off had to reconstruct the set — and the one
    // that did guessed two of six, leaving `data-palette` behind. A leftover
    // `data-palette` is not inert: `dist/scope.css` matches
    // `[data-palette]:not(:root)`, so the whole semantic contract went on
    // resolving at that element, against the system preference rather than the
    // page, and the subtree was stuck in one mode for ever.
    //
    // Derived from the same table as the function above, so the two cannot
    // disagree about what a scope is, and asserted to cover every key the
    // function can return.
    writeln!(
        out,
        "/**\n\
         \x20* Every attribute `scopeAttributes` can return, including the marker.\n\
         \x20*\n\
         \x20* Removing all of them is how a scope is taken back OFF — and a scope\n\
         \x20* left half-removed keeps re-resolving the semantic contract at its own\n\
         \x20* element, which is worse than one that never resolved it at all.\n\
         \x20*/\n\
         export const scopeAttributeNames = [{}];\n",
        scope_attribute_names(theme)
            .iter()
            .map(|name| format!("\"{name}\""))
            .collect::<Vec<_>>()
            .join(", ")
    )
    .expect("write");

    // EXPORTED, because the shell had two hand-written copies of it — one to
    // read a scope's request off the DOM and one to build the document's
    // theme — and a setting added to the spec appeared in neither. Isolating a
    // frame from the page is exactly "carry every one of these", so the list
    // being generated is what makes a NEW setting isolated without new code.
    writeln!(
        out,
        "/**\n\
         \x20* Every setting a scope can carry, in the spec's own order.\n\
         \x20*\n\
         \x20* A setting absent here is `scope = false`: it is emitted at\n\
         \x20* `:root` and no element below it can hold a different value.\n\
         \x20*/\n\
         export const scopableSettings = [{}];\n",
        theme
            .scopable()
            .map(|(name, _)| format!("\"{name}\""))
            .collect::<Vec<_>>()
            .join(", ")
    )
    .expect("write");

    writeln!(out, "const scopable_ = scopableSettings;\n").expect("write");

    writeln!(out, "const model_ = {{").expect("write");
    for name in theme.setting.keys() {
        writeln!(out, "  {name}: {},", camel(&format!("{name}-values"))).expect("write");
    }
    writeln!(out, "}};").expect("write");
}

fn dts_model(theme: &Theme, vendor: &Vendor) -> String {
    let mut out = header_from(CommentStyle::Line("//"), THEME_SPEC);
    writeln!(out).expect("write");

    for (name, setting) in &theme.setting {
        let values = setting.resolved_values(vendor);
        writeln!(
            out,
            "export type {} = {};",
            pascal(name),
            values
                .iter()
                .map(|v| format!("\"{v}\""))
                .collect::<Vec<_>>()
                .join(" | ")
        )
        .expect("write");
    }
    writeln!(out).expect("write");

    for (name, _) in &theme.setting {
        writeln!(
            out,
            "export declare const {}: {{\n\
             \x20 readonly values: readonly {}[];\n\
             \x20 readonly default: {};\n\
             \x20 readonly key: string;\n\
             \x20 readonly attribute: string | null;\n\
             \x20 readonly omitWhen: {} | null;\n\
             \x20 readonly scope: boolean;\n\
             }};",
            camel(&format!("{name}-values")),
            pascal(name),
            pascal(name),
            pascal(name)
        )
        .expect("write");
    }

    writeln!(out, "\nexport declare const scopeAttribute: string;").expect("write");
    writeln!(out, "export declare const stylesheetBase: string;").expect("write");
    writeln!(
        out,
        "export declare function stylesheetHref(palette: Palette): string;"
    )
    .expect("write");
    writeln!(
        out,
        "export declare function resolveScope(\n\
         \x20 request: Partial<ScopeSettings>,\n\
         \x20 inherited: ScopeSettings,\n\
         ): ScopeSettings;"
    )
    .expect("write");
    writeln!(
        out,
        "/** Null when the scope changes nothing. */\n\
         export declare function scopeAttributes(\n\
         \x20 request: Partial<ScopeSettings>,\n\
         \x20 inherited: ScopeSettings,\n\
         ): Record<string, string> | null;\n\n\
         /** Every attribute `scopeAttributes` can return, including the marker. */\n\
         export declare const scopeAttributeNames: readonly string[];\n\n\
         /** Every setting a scope can carry. Absent means `scope = false`. */\n\
         export declare const scopableSettings: readonly (keyof ScopeSettings)[];"
    )
    .expect("write");

    writeln!(out, "\nexport interface Settings {{").expect("write");
    for name in theme.setting.keys() {
        writeln!(out, "  {}: {};", camel(name), pascal(name)).expect("write");
    }
    writeln!(out, "}}\n").expect("write");

    // The scopable subset, as its own type. A ThemeScope that offered a global
    // setting would be offering something the CSS cannot honour — the rule is
    // `:root`-anchored — so the type refuses it rather than the runtime.
    writeln!(out, "export interface ScopeSettings {{").expect("write");
    for (name, _) in theme.scopable() {
        writeln!(out, "  {}: {};", camel(name), pascal(name)).expect("write");
    }
    writeln!(out, "}}").expect("write");

    out
}

fn json_model(theme: &Theme, vendor: &Vendor) -> String {
    // JSON carries no comments, so the generated-file marker goes in a key.
    // Every other emitted file says "do not edit" in a header comment, and a
    // machine-readable file is the one most likely to be hand-patched during a
    // debugging session and then forgotten.
    let mut out = String::from("{\n");
    writeln!(
        out,
        "  \"$comment\": \"Generated by noctua-design — do not edit. \
         Source: {SPEC}. Regenerate: {REGENERATE}\",",
        SPEC = THEME_SPEC,
        REGENERATE = crate::REGENERATE
    )
    .expect("write");
    writeln!(
        out,
        "  \"scopeAttribute\": \"{}\",",
        theme.meta.scope_attribute
    )
    .expect("write");
    writeln!(
        out,
        "  \"stylesheetBase\": \"{}\",",
        theme.meta.stylesheet_base
    )
    .expect("write");
    writeln!(out, "  \"settings\": {{").expect("write");

    let last = theme.setting.len().saturating_sub(1);
    for (index, (name, setting)) in theme.setting.iter().enumerate() {
        let values = setting.resolved_values(vendor);
        writeln!(out, "    \"{name}\": {{").expect("write");
        writeln!(
            out,
            "      \"values\": [{}],",
            values
                .iter()
                .map(|v| format!("\"{v}\""))
                .collect::<Vec<_>>()
                .join(", ")
        )
        .expect("write");
        writeln!(out, "      \"default\": \"{}\",", setting.default).expect("write");
        writeln!(out, "      \"key\": \"{}\",", setting.key).expect("write");
        writeln!(
            out,
            "      \"attribute\": {},",
            setting
                .attribute
                .as_ref()
                .map_or_else(|| "null".to_owned(), |a| format!("\"{a}\""))
        )
        .expect("write");
        writeln!(
            out,
            "      \"omitWhen\": {},",
            setting
                .omit_when
                .as_ref()
                .map_or_else(|| "null".to_owned(), |o| format!("\"{o}\""))
        )
        .expect("write");
        // WHAT COMPUTES THIS ONE. Published because a reader of this file
        // cannot otherwise tell `palette` — which no menu offers a control for,
        // because `accent` and `saturation` are its inputs — from a scopable
        // setting a menu simply forgot. A browser test asserts every scopable
        // setting is offered on a frame, and without this it had to carry the
        // one exception written out by hand.
        writeln!(
            out,
            "      \"derivedFrom\": [{}],",
            setting
                .derived_from
                .iter()
                .map(|from| format!("\"{from}\""))
                .collect::<Vec<_>>()
                .join(", ")
        )
        .expect("write");
        writeln!(out, "      \"scope\": {}", setting.scope).expect("write");
        writeln!(out, "    }}{}", if index == last { "" } else { "," }).expect("write");
    }

    out.push_str("  }\n}\n");
    out
}

// --- The bootstrap ---------------------------------------------------------

/// The inline script, carrying all five lessons of `NOCTUA.md §9.6`.
fn bootstrap(theme: &Theme, vendor: &Vendor) -> String {
    let mut out = header_from(CommentStyle::Line("//"), THEME_SPEC);
    let base = &theme.meta.stylesheet_base;
    // THIS system's default palette, which is not necessarily the color
    // system's. It used to read `vendor.default_theme`, which is the one
    // palette `noctua-colors` anchors at `:root` — correct only for as long as
    // the two agreed, and silently wrong the moment they did not.
    let default = default_palette(theme);

    out.push_str(
        "\n// Runs before the first paint, inline in <head>, AFTER the base links.\n\
         //\n\
         // The order matters and its failure is silent: in system mode the base\n\
         // sheet's `:root:not([data-theme=\"light\"]):not(.light)` and a theme\n\
         // sheet's `[data-palette=\"x\"]:not(…):not(…)` are BOTH (0,3,0), so only\n\
         // source order separates them. A theme sheet linked first means a\n\
         // non-default palette under OS-dark silently paints the default's dark\n\
         // neutrals.\n\
         //\n\
         // Everything is wrapped in try/catch: localStorage throws in private\n\
         // browsing, and the system preference still has to work there.\n\n",
    );

    writeln!(out, "(function () {{").expect("write");
    writeln!(out, "  var r = document.documentElement;").expect("write");
    // Read outside the try: an attribute lookup cannot throw, and the palette
    // href below is useless without it.
    writeln!(
        out,
        "  var siteBase = r.getAttribute('data-noctua-base') || '';"
    )
    .expect("write");
    writeln!(out, "  try {{").expect("write");

    for (name, setting) in &theme.setting {
        let Some(attribute) = &setting.attribute else {
            continue;
        };
        let values = setting.resolved_values(vendor);
        let list = values
            .iter()
            .map(|v| format!("'{v}'"))
            .collect::<Vec<_>>()
            .join(",");

        writeln!(out, "\n    // {name}").expect("write");
        writeln!(
            out,
            "    var {} = localStorage.getItem('{}');",
            identifier(name),
            setting.key
        )
        .expect("write");
        // Validated against the generated list: a stored value reaches an
        // attribute and, for the palette, an href.
        writeln!(
            out,
            "    if ([{list}].indexOf({}) !== -1{}) {{",
            identifier(name),
            setting
                .omit_when
                .as_ref()
                .map_or_else(String::new, |o| format!(
                    " && {} !== '{o}'",
                    identifier(name)
                ))
        )
        .expect("write");
        writeln!(
            out,
            "      r.setAttribute('{attribute}', {});",
            identifier(name)
        )
        .expect("write");
        writeln!(out, "    }}").expect("write");

        if name == "palette" {
            // Stamped even when nothing is stored, because the base link set
            // names a SCOPE-ANCHORED stylesheet (see `base_links`). Without the
            // attribute that sheet matches nothing and the page has no palette:
            // the failure is total, silent, and looks like a missing stylesheet
            // rather than a missing attribute.
            writeln!(out, "    else {{").expect("write");
            writeln!(out, "      r.setAttribute('{attribute}', '{default}');").expect("write");
            writeln!(out, "    }}").expect("write");
            palette_sheet(&mut out, base, default, &list);
        }
    }

    out.push_str(
        "\n    // The page is now correct before anything paints. Remove the\n\
         \x20   // no-script marker last, so a stylesheet keyed on it never applies to\n\
         \x20   // an already-corrected document.\n\
         \x20   r.classList.remove('no-js');\n\
         \x20 } catch (e) {\n\
         \x20   // Private browsing. The system preference still applies, because\n\
         \x20   // `system` is the absence of the attribute rather than a value.\n\
         \x20 }\n\
         })();\n",
    );

    out
}

/// The palette branch of the bootstrap: the one setting whose value names a
/// file rather than only an attribute.
fn palette_sheet(out: &mut String, base: &str, default: &str, list: &str) {
    out.push_str(
        "\n    // Stamping the attribute is not enough: the stylesheet that\n\
         \x20   // DEFINES this palette is a separate file. Without injecting it the\n\
         \x20   // page paints in the default and snaps over when the right file\n\
         \x20   // arrives — on every reload.\n\
         \x20   //\n\
         \x20   // `blocking=\"render\"` is load-bearing and must be set BEFORE\n\
         \x20   // insertion: a stylesheet inserted by script is not render-blocking\n\
         \x20   // by default. Appended last, which is also what wins the (0,3,0) tie.\n\
         \x20   //\n\
         \x20   // The href is resolved against `data-noctua-base`, the same attribute\n\
         \x20   // the runtime reads, because a document sits at whatever depth the\n\
         \x20   // site's URL scheme puts it. A bare relative href here 404s SILENTLY —\n\
         \x20   // render-blocking or not, a sheet that never loads blocks nothing — and\n\
         \x20   // the page paints the default palette until a deferred script snaps it\n\
         \x20   // over. That is the exact flash this script exists to prevent, and it\n\
         \x20   // shipped, because the only test here read back the `blocking`\n\
         \x20   // attribute instead of resolving the URL.\n",
    );
    writeln!(
        out,
        "    if (palette && palette !== '{default}' && [{list}].indexOf(palette) !== -1) {{"
    )
    .expect("write");
    writeln!(out, "      var sheet = document.createElement('link');").expect("write");
    writeln!(out, "      sheet.rel = 'stylesheet';").expect("write");
    writeln!(out, "      sheet.id = 'noctua-palette-sheet';").expect("write");
    writeln!(out, "      sheet.setAttribute('blocking', 'render');").expect("write");
    writeln!(
        out,
        "      sheet.href = siteBase + '{base}/theme-' + palette + '.css';"
    )
    .expect("write");
    writeln!(out, "      document.head.appendChild(sheet);").expect("write");
    writeln!(out, "    }}").expect("write");
}

fn bootstrap_fragment(theme: &Theme, vendor: &Vendor) -> String {
    let script = bootstrap(theme, vendor);
    // Strip the `//` header — an HTML fragment carries its own.
    let body: String = script
        .lines()
        .skip_while(|line| line.starts_with("//") || line.trim().is_empty())
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "<!-- Generated by noctua-design — do not edit.\n\
        \x20    Source: spec/theme.toml\n\
        \x20    Regenerate: cargo xtask build\n\n\
        \x20    Inline this in <head>, AFTER the base links from base.html. It stamps\n\
        \x20    every theme attribute and injects the palette stylesheet before the\n\
        \x20    first paint, so a stored preference never flashes. -->\n\
        <script>\n{body}\n</script>\n"
    )
}

/// This system's default palette, from `spec/theme.toml`.
///
/// Falls back to the color system's own default only if the setting is absent
/// entirely, which `validate_theme` does not allow — the fallback exists so
/// this function has no `unwrap` in it, not because it can be reached.
fn default_palette(theme: &Theme) -> &str {
    theme
        .setting
        .get("palette")
        .map_or("", |setting| setting.default.as_str())
}

/// The base links, in the one order that works.
fn base_links(theme: &Theme, vendor: &Vendor) -> String {
    let base = &theme.meta.stylesheet_base;
    let default = default_palette(theme);
    // `noctua-colors` anchors exactly one palette at `:root` — its own default
    // — and every other at `[data-palette="<name>"]`. So the sheet that carries
    // OUR default is a different file depending on whether the two agree, and
    // getting it wrong is a page with no colors rather than a page with the
    // wrong ones.
    let sheet = if default == vendor.default_theme {
        default.to_owned()
    } else {
        format!("theme-{default}")
    };
    let note = if default == vendor.default_theme {
        "\x20    The default palette below is the color system's own, so it is\n\
        \x20    anchored at :root and applies with no attribute at all."
    } else {
        "\x20    THE DEFAULT PALETTE BELOW IS SCOPE-ANCHORED, so it applies only to an\n\
        \x20    element carrying data-palette=\"<that name>\". That is not an\n\
        \x20    optimisation to undo: the color system publishes a :root sheet for\n\
        \x20    its OWN default and for nothing else, and this design system's\n\
        \x20    default is a different palette.\n\n\
        \x20    Which is why the bootstrap stamps data-palette even when nothing is\n\
        \x20    stored. Link these files without running it, and without rendering\n\
        \x20    the attribute yourself, and the page has no palette whatsoever."
    };
    format!(
        "<!-- Generated by noctua-design — do not edit.\n\
        \x20    Source: spec/theme.toml\n\
        \x20    Regenerate: cargo xtask build\n\n\
        \x20    THE ORDER IS LOAD-BEARING, and getting it wrong fails silently.\n\n\
        \x20    In system mode the default palette's\n\
        \x20      :root:not([data-theme=\"light\"]):not(.light)\n\
        \x20    and a theme sheet's\n\
        \x20      [data-palette=\"x\"]:not([data-theme=\"light\"]):not(.light)\n\
        \x20    are BOTH specificity (0,3,0). Only source order separates them. The\n\
        \x20    default must come first, so a theme sheet appended later wins.\n\n\
        {note}\n\n\
        \x20    scope.css REPLACES the vendored contexts.css — it carries the same\n\
        \x20    declarations plus the selectors that make them resolve inside a\n\
        \x20    themed subtree. Linking both would double the cost for nothing.\n\n\
        \x20    components.css is LAST, so a component rule that ties with one in\n\
        \x20    theme.css on specificity wins — which is the relation those rules\n\
        \x20    were written in. A page with its own stylesheet links that after\n\
        \x20    this, and then IT wins, which is the point of being last here. -->\n\
        <link rel=\"stylesheet\" href=\"{base}/ramp.css\">\n\
        <link rel=\"stylesheet\" href=\"scope.css\">\n\
        <link rel=\"stylesheet\" href=\"{base}/{sheet}.css\" id=\"noctua-base-palette\">\n\
        <link rel=\"stylesheet\" href=\"tokens.css\">\n\
        <link rel=\"stylesheet\" href=\"tones.css\">\n\
        <link rel=\"stylesheet\" href=\"syntax.css\">\n\
        <link rel=\"stylesheet\" href=\"fonts.css\">\n\
        <link rel=\"stylesheet\" href=\"theme.css\">\n\
        <link rel=\"stylesheet\" href=\"components.css\">\n"
    )
}

/// `snake-case` or `kebab-case` to `PascalCase`.
fn pascal(name: &str) -> String {
    let camel = camel(name);
    let mut chars = camel.chars();
    chars.next().map_or_else(String::new, |first| {
        first.to_uppercase().collect::<String>() + chars.as_str()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shipped() -> (Theme, Vendor) {
        let root = noctua_design_spec::vendor::repository_root();
        let vendor = noctua_design_spec::vendor::load(&root).expect("vendor");
        let theme =
            noctua_design_spec::load_theme(root.join("spec/theme.toml"), &vendor).expect("theme");
        (theme, vendor)
    }

    fn file(name: &str) -> String {
        let (theme, vendor) = shipped();
        emit(&theme, &vendor)
            .into_iter()
            .find(|f| f.path == name)
            .unwrap_or_else(|| panic!("{name} should be emitted"))
            .contents
    }

    /// **The writer/reader key test**, in its strongest form.
    ///
    /// Every storage key must appear in the bootstrap, in the Rust model and in
    /// the TypeScript model. This exact contract silently broke in
    /// `noctua-colors` for a milestone, because the writer lived behind a
    /// control that was never rendered — so the bootstrap always found nothing
    /// and every reload lost the preference.
    #[test]
    fn the_stored_keys_are_the_keys_the_bootstrap_reads() {
        let (theme, _) = shipped();
        let bootstrap = file("dist/bootstrap.js");
        let rust = file("crates/noctua-design-theme/src/lib.rs");
        let js = file("packages/tokens/theme.js");

        for (name, setting) in &theme.setting {
            let key = &setting.key;
            // Only settings with an attribute are read by the bootstrap; the
            // rest are inputs the runtime resolves.
            if setting.attribute.is_some() {
                assert!(
                    bootstrap.contains(&format!("getItem('{key}')")),
                    "the bootstrap does not read `{key}` for setting `{name}`"
                );
            }
            assert!(
                rust.contains(&format!("\"{key}\"")),
                "Rust is missing {key}"
            );
            assert!(
                js.contains(&format!("\"{key}\"")),
                "TypeScript is missing {key}"
            );
        }
    }

    /// `system` must stamp no attribute at all.
    #[test]
    fn the_bootstrap_sets_no_attribute_for_the_omitted_value() {
        let bootstrap = file("dist/bootstrap.js");
        assert!(
            bootstrap.contains("mode !== 'system'"),
            "the bootstrap would stamp data-theme=\"system\", which no stylesheet understands"
        );
    }

    /// The injected sheet must be render-blocking. Necessary, and **not**
    /// sufficient — see the test below, which is the one that matters.
    #[test]
    fn the_injected_stylesheet_is_render_blocking() {
        let bootstrap = file("dist/bootstrap.js");
        assert!(bootstrap.contains("setAttribute('blocking', 'render')"));
        // Set before insertion — after it, the sheet is already not blocking.
        let set = bootstrap.find("setAttribute('blocking'").expect("present");
        let append = bootstrap.find("appendChild(sheet)").expect("present");
        assert!(set < append, "blocking is set after the sheet is inserted");
    }

    /// The palette href must resolve from whatever depth the page sits at.
    ///
    /// `blocking="render"` guarantees nothing on its own: a sheet that 404s
    /// blocks no paint. This repository shipped a bare `'tokens/css/theme-…'`
    /// href — correct only from the site root, wrong on every real page — for
    /// exactly as long as the only test here looked at the `blocking`
    /// attribute. A test that reads back a property the code under test just
    /// set is not a test.
    #[test]
    fn the_injected_stylesheet_resolves_against_the_document_base() {
        let bootstrap = file("dist/bootstrap.js");
        assert!(
            bootstrap.contains("getAttribute('data-noctua-base')"),
            "the bootstrap does not read the base the document declares"
        );
        assert!(
            bootstrap.contains("sheet.href = siteBase +"),
            "the palette href is not rebased, so it 404s on any page below the site root"
        );
    }

    /// A stored value reaches an attribute and an `href`. It is never trusted.
    #[test]
    fn the_bootstrap_validates_every_stored_value() {
        let bootstrap = file("dist/bootstrap.js");
        assert!(
            bootstrap.contains("indexOf(palette) !== -1"),
            "a stored palette becomes an href without being checked against the list"
        );
    }

    /// The base link order is the one that works.
    #[test]
    fn the_base_links_put_the_default_palette_before_any_theme_sheet() {
        let (theme, _) = shipped();
        // THIS system's default, not the color system's. The test read
        // `vendor.default_theme`, which is the palette `noctua-colors` anchors
        // at `:root` — the same value only for as long as the two agreed.
        let default = default_palette(&theme);
        let base = file("dist/base.html");

        // Only the <link> lines. The header comment explains the ordering and
        // names every file in it, so searching the whole document measures the
        // prose rather than the links.
        let links: Vec<&str> = base
            .lines()
            .filter(|line| line.trim_start().starts_with("<link"))
            .collect();
        let at = |needle: &str| {
            links
                .iter()
                .position(|line| line.contains(needle))
                .unwrap_or_else(|| panic!("no link for {needle} in {links:?}"))
        };

        assert!(at("ramp.css") < at("scope.css"), "the ramp must come first");
        assert!(
            at("scope.css") < at(&format!("{default}.css")),
            "the default palette must come after the contract, and before any theme \
             sheet the bootstrap appends — in system mode the two tie at (0,3,0) and \
             only source order separates them"
        );
        assert!(
            !links.iter().any(|line| line.contains("contexts.css")),
            "scope.css replaces contexts.css; linking both doubles the cost"
        );
    }

    /// A default that is not the color system's needs the attribute, and the
    /// bootstrap is what writes it.
    ///
    /// `noctua-colors` anchors one palette at `:root` and every other at
    /// `[data-palette="<name>"]`. So when this system's default is not that
    /// one, the base sheet applies to nothing until something stamps the
    /// attribute — and a page with no palette at all looks like a stylesheet
    /// that failed to load, not like a missing attribute.
    #[test]
    fn the_bootstrap_stamps_the_default_palette_when_nothing_is_stored() {
        let (theme, vendor) = shipped();
        let default = default_palette(&theme);
        let script = file("dist/bootstrap.html");

        if default == vendor.default_theme {
            return;
        }
        assert!(
            script.contains(&format!("r.setAttribute('data-palette', '{default}');")),
            "the base sheet is scope-anchored and nothing stamps the scope:\n{script}"
        );
        assert!(
            file("dist/base.html").contains(&format!("theme-{default}.css")),
            "the base link set does not name the scope-anchored default"
        );
    }

    /// Both languages must know the same values and defaults.
    #[test]
    fn both_languages_declare_the_same_values_and_defaults() {
        let (theme, vendor) = shipped();
        let rust = file("crates/noctua-design-theme/src/lib.rs");
        let js = file("packages/tokens/theme.js");

        for (_, setting) in &theme.setting {
            for value in setting.resolved_values(&vendor) {
                assert!(rust.contains(&format!("=> \"{value}\"")), "Rust: {value}");
                assert!(js.contains(&format!("\"{value}\"")), "TS: {value}");
            }
            assert!(js.contains(&format!("default: \"{}\"", setting.default)));
        }
    }

    /// THE LIST MUST COVER EVERY ATTRIBUTE THE FUNCTION CAN SET.
    ///
    /// This is the invariant that was broken, and it was broken by hand rather
    /// than by the generator: a caller taking a scope back off removed two of
    /// the six attributes `scopeAttributes` stamps. The four left behind kept
    /// `dist/scope.css` re-resolving the semantic contract at that element —
    /// `[data-palette]:not(:root)` is enough on its own — so the subtree
    /// resolved against the system preference instead of the page and could
    /// never be un-themed.
    #[test]
    fn the_attribute_list_covers_everything_a_scope_can_stamp() {
        let (theme, _) = shipped();
        let names = scope_attribute_names(&theme);

        assert!(
            names.contains(&theme.meta.scope_attribute),
            "the marker is an attribute a scope carries, and removing it is how \
             the contract stops resolving there"
        );
        for (name, setting) in theme.scopable() {
            let Some(attribute) = &setting.attribute else {
                // `accent` and `saturation` reach the DOM through `palette`.
                continue;
            };
            assert!(
                names.contains(attribute),
                "`{name}` is stamped as `{attribute}` and is not in the list, so \
                 nothing that takes a scope off would remove it"
            );
        }
        assert_eq!(
            names.len(),
            names
                .iter()
                .collect::<std::collections::BTreeSet<_>>()
                .len(),
            "a repeated attribute means one of them is being derived twice"
        );
    }

    /// Both packages publish the same list, because it is one fact.
    #[test]
    fn both_languages_publish_the_same_scope_attributes() {
        let (theme, _) = shipped();
        let rust = file("crates/noctua-design-theme/src/lib.rs");
        let js = file("packages/tokens/theme.js");
        let dts = file("packages/tokens/theme.d.ts");

        let names = scope_attribute_names(&theme);
        let list = names
            .iter()
            .map(|name| format!("\"{name}\""))
            .collect::<Vec<_>>()
            .join(", ");

        assert!(
            rust.contains(&format!(
                "SCOPE_ATTRIBUTES: [&str; {}] = [{list}]",
                names.len()
            )),
            "the Rust model does not publish the list:\n{rust}"
        );
        assert!(
            js.contains(&format!("export const scopeAttributeNames = [{list}];")),
            "the JavaScript model does not publish the list:\n{js}"
        );
        assert!(
            dts.contains("export declare const scopeAttributeNames"),
            "a consumer in TypeScript cannot see the list"
        );
    }

    /// A global setting must have no scope, in the model that both packages
    /// read — so neither can offer it on a `ThemeScope`.
    #[test]
    fn global_settings_are_marked_unscopable_in_both_models() {
        let (theme, _) = shipped();
        let json = file("dist/theme.json");
        for (name, setting) in theme.global() {
            let _ = name;
            assert!(!setting.scope);
        }
        assert!(json.contains("\"scope\": false"), "{json}");
        assert!(json.contains("\"scope\": true"));
    }

    /// The JavaScript resolver must carry the same three rules as the Rust one.
    ///
    /// Both are generated from the same table, but they are generated by
    /// *different code*, and a divergence would only show up in a browser. This
    /// asserts the rules are present in the emitted text; the Rust behaviour is
    /// covered by `crates/noctua-design-theme/tests/scope.rs`.
    #[test]
    fn the_javascript_resolver_carries_the_same_rules() {
        let js = file("packages/tokens/theme.js");
        assert!(
            js.contains("if (scopable_.every((name) => r[name] === inherited[name])) return null;"),
            "the no-op rule is missing"
        );
        assert!(
            js.contains("if (model.omitWhen != null && r[name] === model.omitWhen) continue;"),
            "the omitted-value rule is missing, so `system` would be stamped"
        );
        // The palette is emitted unconditionally for a non-no-op scope, which
        // is what keeps the compound selector matching.
        assert!(
            js.contains("out[model.attribute] = r[name];"),
            "attributes are not stamped from the resolved value"
        );
    }

    /// The href is built in one place per language.
    #[test]
    fn the_stylesheet_href_is_built_from_the_generated_base() {
        let js = file("packages/tokens/theme.js");
        assert!(js.contains("stylesheetBase + \"/theme-\" + palette + \".css\""));
        // The bootstrap interpolates the base at generation time, so the
        // emitted text is `'<base>/theme-' + palette + '.css'`.
        let (theme, _) = shipped();
        let bootstrap = file("dist/bootstrap.js");
        assert!(
            bootstrap.contains(&format!(
                "'{}/theme-' + palette + '.css'",
                theme.meta.stylesheet_base
            )),
            "the bootstrap does not build the href from the generated base"
        );
    }
}
