//! Builds the documentation site: every page, in every locale, as static HTML.
//!
//! No `dx`, no wasm toolchain, no JavaScript runtime — an ordinary Rust binary
//! that renders `rsx!` to a string. That is a deliberate constraint rather than
//! a limitation accepted: `NOCTUA.md §9.11` bounds Node's role in this
//! repository to the React package and the conformance runner, and a
//! documentation site that needed a fourth toolchain to look at would be a site
//! nobody looks at.
//!
//! What ships is HTML that is correct before any script runs. The script adds
//! the settings; it does not construct the page.

mod index;
mod pages;
mod props;
mod shell;
mod site;

use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use dioxus::prelude::*;
use noctua_design_i18n::{Key, Locale};

use shell::Context;
use site::Page;

/// The base links, in the order the theme layer proved is the only one that
/// works, taken from the generated file rather than transcribed.
///
/// Transcribing them would put the ordering rule in two places, and the failure
/// mode when they disagree is a non-default palette silently painting the
/// default's dark neutrals — visible only under OS dark, only on a reload.
const BASE_LINKS: &str = include_str!("../../dist/base.html");

/// The pre-paint script, likewise taken whole.
const BOOTSTRAP: &str = include_str!("../../dist/bootstrap.html");

fn main() -> Result<(), String> {
    let root = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/.."));
    let out = root.join("docs-site/dist");

    if out.exists() {
        std::fs::remove_dir_all(&out)
            .map_err(|e| format!("could not clear {}: {e}", out.display()))?;
    }

    let mut written = 0usize;
    for locale in Locale::ALL {
        for page in site::PAGES {
            let path = out.join(locale.tag()).join(page.file());
            write(&path, &document(locale, page))?;
            written += 1;
        }
    }

    // The site root redirects to the default locale. A meta refresh rather than
    // a server rule, because the whole point of this build is a directory that
    // can be served by anything at all.
    write(&out.join("index.html"), &redirect())?;
    written += 1;

    written += copy_assets(&root, &out)?;

    println!("built {written} files into docs-site/dist");
    println!(
        "  {} pages × {} locales, plus assets",
        site::PAGES.len(),
        Locale::ALL.len()
    );
    Ok(())
}

/// One complete HTML document.
fn document(locale: Locale, page: Page) -> String {
    let context = Context { locale, page };
    let title = format!(
        "{} — {}",
        locale.message(page.title),
        locale.message(Key::AppName)
    );

    let mut html = String::with_capacity(64 * 1024);
    html.push_str("<!doctype html>\n");
    // `no-js` is removed by the bootstrap. Nothing is hidden by markup and
    // unhidden by script — the reverse, so a page with the script blocked is
    // complete rather than blank (`NOCTUA.md §9.6` lesson 4).
    let _ = writeln!(
        html,
        "<html lang=\"{}\" dir=\"{}\" class=\"no-js\" data-noctua-base=\"../\">",
        locale.bcp47(),
        locale.dir()
    );
    html.push_str("<head>\n<meta charset=\"utf-8\">\n");
    html.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n");
    let _ = write!(
        html,
        "<title>{}</title>\n<meta name=\"description\" content=\"{}\">\n",
        escape(&title),
        escape(locale.message(page.summary))
    );

    html.push_str(&rebase(BASE_LINKS));
    html.push_str("<link rel=\"stylesheet\" href=\"../assets/shell.css\">\n");
    html.push_str(BOOTSTRAP);
    html.push_str("\n</head>\n<body>\n");

    html.push_str(&render(context));
    html.push_str("<script type=\"module\" src=\"../assets/shell.js\"></script>\n");
    html.push_str("</body>\n</html>\n");
    html
}

/// Renders the whole body to HTML.
///
/// One `VirtualDom` for the entire page rather than one per region, and that is
/// not only tidier: `ThemeScope` publishes its resolved theme through context,
/// and context does not cross a render boundary. Rendering the regions
/// separately would give a scope on the page no way to reach a component in the
/// header — which is exactly the kind of thing that works until the first time
/// somebody needs it.
fn render(context: Context) -> String {
    let mut dom = VirtualDom::new_with_props(document_tree, DocumentProps { context });
    dom.rebuild_in_place();
    dioxus_ssr::render(&dom)
}

#[derive(Props, Clone, PartialEq)]
struct DocumentProps {
    context: Context,
}

#[expect(
    clippy::needless_pass_by_value,
    reason = "Dioxus calls a component with its props by value; taking a reference is not a signature the framework accepts"
)]
fn document_tree(props: DocumentProps) -> Element {
    // Destructured rather than borrowed: `Context` is `Copy`, and a component
    // that takes its props by reference is not a shape Dioxus's `Props` derive
    // produces.
    let DocumentProps { context } = props;
    rsx! {
        a { class: "nd-skip", href: "#nd-main", "{context.t(Key::AppSkip)}" }
        shell::Effects {}
        shell::Header { context }
        div { class: "nd-body",
            shell::Sidebar { context }
            shell::Scrim { context }
            main { id: "nd-main", class: "nd-main",
                pages::Body { context }
                footer { class: "nd-foot",
                    p { "{context.t(Key::FooterGenerated)}" }
                }
            }
        }
    }
}

/// Rewrites the generated base links so they resolve from a page one directory
/// below the site root.
///
/// The alternative — every page at the root — would mean three locales sharing
/// one namespace, and `colours.html` could belong to only one of them.
fn rebase(html: &str) -> String {
    html.replace("href=\"", "href=\"../")
}

/// The root document, which sends a reader to the default locale.
fn redirect() -> String {
    let target = format!("{}/index.html", Locale::DEFAULT.tag());
    format!(
        "<!doctype html>\n\
         <html lang=\"{lang}\">\n\
         <head>\n\
         <meta charset=\"utf-8\">\n\
         <meta http-equiv=\"refresh\" content=\"0; url={target}\">\n\
         <link rel=\"canonical\" href=\"{target}\">\n\
         <title>{name}</title>\n\
         </head>\n\
         <body><a href=\"{target}\">{name}</a></body>\n\
         </html>\n",
        lang = Locale::DEFAULT.bcp47(),
        name = Locale::DEFAULT.message(Key::AppName),
    )
}

/// Copies the generated stylesheets, the vendored palettes and the fonts.
fn copy_assets(root: &Path, out: &Path) -> Result<usize, String> {
    let mut count = 0usize;

    // The generated stylesheets, at the site root, which is where the base
    // links (rebased above) look for them.
    for name in ["tokens.css", "theme.css", "scope.css"] {
        count += copy(&root.join("dist").join(name), &out.join(name))?;
    }

    // `fonts.css` reaches its files with `url("../assets/fonts/…")`, which is
    // correct where it is generated — `dist/` and `assets/` are siblings there.
    // Here it sits at the site root, so the `../` has to go or every face would
    // be requested from above the root.
    let fonts = std::fs::read_to_string(root.join("dist/fonts.css"))
        .map_err(|e| format!("could not read dist/fonts.css: {e}"))?;
    write(
        &out.join("fonts.css"),
        &fonts.replace("url(\"../assets/fonts/", "url(\"assets/fonts/"),
    )?;
    count += 1;

    // The vendored colour output, under the path `spec/theme.toml` declares.
    count += copy_dir(
        &root.join("packages/tokens/vendor/css"),
        &out.join("tokens/css"),
        "css",
    )?;

    count += copy_dir(
        &root.join("assets/fonts"),
        &out.join("assets/fonts"),
        "woff2",
    )?;

    // The shell's own two files, and the generated modules the script imports
    // rather than restates.
    count += copy(
        &root.join("docs-site/assets/shell.css"),
        &out.join("assets/shell.css"),
    )?;
    count += copy(
        &root.join("docs-site/assets/shell.js"),
        &out.join("assets/shell.js"),
    )?;
    count += copy(
        &root.join("packages/tokens/theme.js"),
        &out.join("assets/theme.js"),
    )?;
    // The React example, bundled by esbuild. Optional on purpose: the site must
    // build on a machine that has never run `pnpm install`, because the Rust
    // half of this repository owes nothing to Node (`NOCTUA.md §9.11`). Without
    // it the implementation switch reports a failed load and the Dioxus example
    // — which is server-rendered and already on the page — keeps working.
    let react = root.join("docs-site/assets/example-react.js");
    if react.exists() {
        count += copy(&react, &out.join("assets/example-react.js"))?;
    } else {
        println!(
            "  note: docs-site/assets/example-react.js is absent — run `pnpm run build:example`"
        );
    }

    Ok(count)
}

fn copy_dir(from: &Path, to: &Path, extension: &str) -> Result<usize, String> {
    let entries =
        std::fs::read_dir(from).map_err(|e| format!("could not read {}: {e}", from.display()))?;
    let mut count = 0usize;
    for entry in entries {
        let path = entry.map_err(|e| format!("{e}"))?.path();
        if path.extension().is_some_and(|e| e == extension) {
            let name = path.file_name().unwrap_or_default();
            count += copy(&path, &to.join(name))?;
        }
    }
    Ok(count)
}

fn copy(from: &Path, to: &Path) -> Result<usize, String> {
    if let Some(parent) = to.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("{e}"))?;
    }
    std::fs::copy(from, to)
        .map_err(|e| format!("could not copy {} to {}: {e}", from.display(), to.display()))?;
    Ok(1)
}

fn write(path: &Path, contents: &str) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("could not create {}: {e}", parent.display()))?;
    }
    std::fs::write(path, contents).map_err(|e| format!("could not write {}: {e}", path.display()))
}

/// Escapes text for an attribute or a title.
fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_page_builds_in_every_locale() {
        for locale in Locale::ALL {
            for page in site::PAGES {
                let html = document(locale, page);
                assert!(html.starts_with("<!doctype html>"), "{}", page.slug);
                assert!(html.contains("</html>"), "{}", page.slug);
                assert!(
                    html.contains(&format!("lang=\"{}\"", locale.bcp47())),
                    "{} {}",
                    locale.tag(),
                    page.slug
                );
            }
        }
    }

    /// The rule the base file writes down in capitals, asserted rather than
    /// trusted: the default palette must be linked before anything that could
    /// tie with it at (0,3,0).
    #[test]
    fn the_base_link_order_survives_rebasing() {
        let head = rebase(BASE_LINKS);
        let default = head
            .find("ochre-balanced.css")
            .expect("the default palette");
        let scope = head.find("scope.css").expect("the scoped contract");
        let tokens = head.find("tokens.css").expect("the design tokens");
        assert!(scope < default, "scope.css must precede the palette");
        assert!(default < tokens, "the palette must precede the tokens");
    }

    /// Every href has to move exactly one level, or the assets 404 from a page
    /// inside a locale directory.
    #[test]
    fn rebasing_moves_every_href_and_nothing_else() {
        let head = rebase(BASE_LINKS);
        assert_eq!(
            head.matches("href=\"../").count(),
            BASE_LINKS.matches("href=\"").count()
        );
        assert!(!head.contains("href=\"../../"), "an href moved twice");
    }

    /// A page with the script blocked must still be a complete page.
    #[test]
    fn the_navigation_is_in_the_markup() {
        let html = document(Locale::En, site::PAGES[0]);
        let body = html.split("<body>").nth(1).expect("a body");
        assert!(
            body.contains("nd-nav-link"),
            "the navigation is not in the markup"
        );
        assert!(body.contains("nd-skip"), "there is no skip link");
    }

    #[test]
    fn the_scoped_theme_page_carries_a_real_nested_scope() {
        let page = *site::PAGES
            .iter()
            .find(|p| p.slug == "scoped-themes")
            .expect("the page");
        let html = document(Locale::En, page);
        // Two scope ELEMENTS, each with its palette and its mode on the same
        // element — which is the whole contract in one assertion.
        //
        // Matched with the `=""` rather than on the bare name: the attribute is
        // also one of the searchable names in the sidebar, and counting the
        // string alone made this test fail the moment search learned about it.
        assert_eq!(html.matches("data-noctua-scope=\"\"").count(), 2, "{html}");
        assert!(html.contains("data-palette=\"azure-vivid\""));
        assert!(html.contains("data-palette=\"rose-sober\""));
    }

    /// The site must be reproducible, or "it works on my machine" becomes a
    /// property of the documentation as well as of the build.
    #[test]
    fn the_same_inputs_produce_the_same_page() {
        for locale in Locale::ALL {
            for page in site::PAGES {
                assert_eq!(document(locale, page), document(locale, page));
            }
        }
    }

    #[test]
    fn the_language_control_points_at_the_same_page() {
        for page in site::PAGES {
            let html = document(Locale::En, page);
            assert!(
                html.contains(&format!("../{}/{}", Locale::PtBr.tag(), page.file())),
                "{} does not link to its translation",
                page.slug
            );
        }
    }
}
