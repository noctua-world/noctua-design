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

mod defaults;
mod examples;
mod generated;
mod index;
mod pages;
mod props;
mod scripts;
mod serialise;
mod shell;
mod site;
mod theme_menu;
mod tree;
mod trees;

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
        for page in site::pages() {
            let path = out.join(locale.tag()).join(page.file());
            write(&path, &document(locale, page))?;
            written += 1;
        }
        // `/<locale>/` is the URL people paste, so it has to resolve — but it
        // is a door, not a room. It sends the reader to the first entry in
        // their sidebar and carries no title, no prose and no sidebar entry of
        // its own.
        let first = site::first(locale);
        write(
            &out.join(locale.tag()).join("index.html"),
            &redirect_to(
                locale,
                &first.dir(),
                &locale_script(locale.tag(), &door_targets("../"), true),
            ),
        )?;
        written += 1;

        // THE SEARCH INDEX, ONE FILE PER LOCALE, FETCHED ON FIRST USE.
        //
        // These entries used to ship as hidden `<a>` elements in the sidebar of
        // every document — thirty-five kilobytes each, on every page load, for
        // a list most readers never open. Here they are one file the browser
        // caches for the whole visit, and the palette asks for it the first
        // time somebody searches.
        write(
            &out.join("assets")
                .join(format!("search-{}.json", locale.tag())),
            &index::json(locale),
        )?;
        written += 1;

        // EVERY EXAMPLE, AS DATA, FOR THE READER WHO ASKS FOR REACT.
        //
        // Beside the search index and for the same reason: a file the browser
        // caches for the visit, rather than a copy inside every document for a
        // feature most readers never invoke. `docs-site/src/serialise.rs` is
        // the argument in full.
        write(
            &out.join("assets")
                .join(format!("examples-{}.json", locale.tag())),
            &serialise::json(locale),
        )?;
        written += 1;
    }

    // The site root redirects to the default locale. A meta refresh rather than
    // a server rule, because the whole point of this build is a directory that
    // can be served by anything at all.
    write(&out.join("index.html"), &redirect())?;
    written += 1;

    // GitHub Pages runs Jekyll unless told not to, and Jekyll drops any path
    // beginning with an underscore. Nothing here has one today; the marker
    // costs a byte and removes the need to remember that on the day one does.
    write(&out.join(".nojekyll"), "")?;
    written += 1;

    written += copy_assets(&root, &out)?;

    println!("built {written} files into docs-site/dist");
    println!(
        "  {} pages × {} locales, plus assets",
        site::pages().len(),
        Locale::ALL.len()
    );
    Ok(())
}

/// Whether the React bundle exists on this machine.
///
/// # Why the page asks, rather than the script finding out
///
/// `NOCTUA.md §9.11` says the Rust half of this repository owes nothing to
/// Node, so the site has to build on a machine that has never run `pnpm
/// install`. It does — and until now the implementation control was rendered
/// anyway, and failed once for every reader who pressed it.
///
/// A control offered where it does nothing is a control that lies, which is a
/// rule this codebase states in three other places. So the build asks once, and
/// a site built without the bundle carries no switch at all.
pub(crate) fn react_available() -> bool {
    static KNOWN: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *KNOWN.get_or_init(|| {
        PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/.."))
            .join("docs-site/assets/react/example-react.js")
            .exists()
    })
}

/// One complete HTML document.
fn document(locale: Locale, page: Page) -> String {
    let context = Context { locale, page };
    let title = format!("{} — {}", page.title(locale), locale.message(Key::AppName));

    let mut html = String::with_capacity(64 * 1024);
    html.push_str("<!doctype html>\n");
    // `no-js` is removed by the bootstrap. Nothing is hidden by markup and
    // unhidden by script — the reverse, so a page with the script blocked is
    // complete rather than blank (`NOCTUA.md §9.6` lesson 4).
    let _ = writeln!(
        html,
        // `data-palette` is server-rendered rather than left to the bootstrap.
        // The base link set names the DEFAULT palette's scope-anchored
        // stylesheet — `theme-<default>.css`, the only shape `noctua-colors`
        // publishes for anything but its own default — so the attribute is what
        // makes it apply. Without it here, a reader with the script blocked
        // would get a page with no palette at all.
        //
        // `data-noctua-locales` is the offered set, read by `shell.js` so a
        // locale added to `spec/i18n/` needs no edit there.
        // `data-noctua-locale` is the tag this document was BUILT for, which is
        // not the same string as `lang`: `lang` is BCP 47 for the browser, and
        // the tag is what names this locale's own files. The example trees are
        // one of those files, so reading `lang` would ask for the wrong one the
        // day the two spellings differ.
        "<html lang=\"{}\" dir=\"{}\" class=\"no-js\" data-noctua-base=\"{}\" \
         data-palette=\"{}\" data-noctua-locale=\"{}\" data-noctua-locales=\"{}\">",
        locale.bcp47(),
        locale.dir(),
        site::BASE,
        noctua_design_theme::Palette::DEFAULT.as_str(),
        locale.tag(),
        Locale::OFFERED
            .into_iter()
            .map(Locale::tag)
            .collect::<Vec<_>>()
            .join(" ")
    );
    html.push_str("<head>\n<meta charset=\"utf-8\">\n");
    html.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n");
    let _ = write!(
        html,
        "<title>{}</title>\n<meta name=\"description\" content=\"{}\">\n",
        escape(&title),
        escape(locale.message(page.summary))
    );

    // Declared, so the browser stops asking for /favicon.ico and getting a 404
    // on every single page load. An SVG: one file for every size, and it
    // follows the reader's system between light and dark.
    let _ = writeln!(
        html,
        "<link rel=\"icon\" href=\"{}favicon.svg\" type=\"image/svg+xml\">",
        site::BASE
    );
    html.push_str(&rebase(BASE_LINKS));
    let _ = writeln!(
        html,
        "<link rel=\"stylesheet\" href=\"{}assets/shell.css\">",
        site::BASE
    );
    // The language first: a redirect makes everything below it wasted work.
    html.push_str(&locale_script(
        locale.tag(),
        &Locale::OFFERED
            .iter()
            .map(|other| (other.tag(), page.locale_href(*other)))
            .collect::<Vec<_>>(),
        false,
    ));
    html.push_str(BOOTSTRAP);
    html.push_str("\n</head>\n<body>\n");

    html.push_str(&render(context));
    let _ = writeln!(
        html,
        "<script type=\"module\" src=\"{}assets/shell.js\"></script>",
        site::BASE
    );
    html.push_str("</body>\n</html>\n");
    html
}

/// Resolves the reader's language before anything paints.
///
/// Inline, in `<head>`, and above the theme bootstrap — for the same reason the
/// theme bootstrap is inline at all. A redirect that fires after first paint is
/// a page the reader has already seen in the wrong language, which is worse
/// than the flash it looks like.
///
/// # It never builds a URL out of anything a reader controls
///
/// The destinations are written **here**, at build time, one per offered
/// locale. The script chooses one from that table; it never concatenates a
/// stored value or a `navigator.languages` entry into a path. A stored value
/// that is neither [`defaults::AUTOMATIC_LOCALE`] nor an offered tag becomes
/// `auto`, and an `auto` that matches nothing means *stay where you are*. Both
/// directions fail closed (`NOCTUA.md §2.4`).
///
/// # And it never overrides a locale the reader asked for
///
/// A URL that was not reached from inside this site names the language it
/// names, and a link shared with somebody must open in the language it was
/// shared in. `document.referrer` is the only thing that separates the two, and
/// it is the whole discriminator: browsing the site follows the preference, a
/// pasted or shared link does not move. A **door** — `/` and `/<locale>/`, both
/// of which exist only to send a reader somewhere — is exempt, because sending
/// a reader somewhere is the entire content of the document.
///
/// [`defaults::AUTOMATIC_LOCALE`]: crate::defaults::AUTOMATIC_LOCALE
fn locale_script(here: &str, targets: &[(&str, String)], door: bool) -> String {
    let table = targets
        .iter()
        .map(|(tag, href)| format!("\"{tag}\":\"{href}\""))
        .collect::<Vec<_>>()
        .join(",");
    let offered = Locale::OFFERED
        .iter()
        .map(|locale| format!("\"{}\"", locale.tag()))
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "<script>\n\
         (function () {{\n\
        \x20 // THE SCRIPT IS HERE, SAID BEFORE THE FIRST PAINT.\n\
        \x20 //\n\
        \x20 // One attribute, and it buys the one thing a deferred script\n\
        \x20 // cannot: a control that only exists WITH a script can have its\n\
        \x20 // space reserved by the stylesheet before anything is drawn. The\n\
        \x20 // palette trigger ships `hidden` — correctly, since a button that\n\
        \x20 // opens a modal nothing can fill is a button that does nothing —\n\
        \x20 // and revealing it after paint dropped the navigation forty-eight\n\
        \x20 // pixels on every page load. That is a layout shift, which\n\
        \x20 // `NOCTUA.md \u{a7}9.6` forbids, and `theme.spec.js` measured it.\n\
        \x20 //\n\
        \x20 // It goes at the top of the file that already runs before paint\n\
        \x20 // rather than in a second inline script: two of them is two things\n\
        \x20 // that can be ordered wrongly.\n\
        \x20 //\n\
        \x20 // `scripting`, not `script`: `data-nd-script` is the typography\n\
        \x20 // page's, one per writing system, and stamping it on the root made\n\
        \x20 // `[data-nd-script]` resolve to fifteen elements instead of\n\
        \x20 // fourteen. Two tests said so at once.\n\
        \x20 document.documentElement.setAttribute('data-nd-scripting', '');\n\
        \x20 var targets = {{{table}}};\n\
        \x20 var offered = [{offered}];\n\
        \x20 var here = \"{here}\";\n\
        \x20 var door = {door};\n\
        \x20 try {{\n\
        \x20   var stored = localStorage.getItem('{key}');\n\
        \x20   if (stored !== '{auto}' && offered.indexOf(stored) === -1) stored = '{auto}';\n\
        \x20   var want = null;\n\
        \x20   if (stored === '{auto}') {{\n\
        \x20     var asked = navigator.languages || [navigator.language || ''];\n\
        \x20     for (var i = 0; i < asked.length && want === null; i++) {{\n\
        \x20       var tag = String(asked[i] || '').toLowerCase();\n\
        \x20       for (var j = 0; j < offered.length; j++) {{\n\
        \x20         if (offered[j].toLowerCase() === tag) want = offered[j];\n\
        \x20       }}\n\
        \x20       if (want !== null) break;\n\
        \x20       // pt-PT asks for Portuguese, and pt-BR is the Portuguese\n\
        \x20       // this site has. An exact tag wins first, across the whole\n\
        \x20       // list, before any primary subtag is considered.\n\
        \x20       var primary = tag.split('-')[0];\n\
        \x20       for (var k = 0; k < offered.length; k++) {{\n\
        \x20         if (want === null && offered[k].toLowerCase().split('-')[0] === primary) {{\n\
        \x20           want = offered[k];\n\
        \x20         }}\n\
        \x20       }}\n\
        \x20     }}\n\
        \x20     // A URL the reader did not reach BY FOLLOWING A LINK inside\n\
        \x20     // this site names the language it names.\n\
        \x20     //\n\
        \x20     // The referrer alone is not the test. A reload sets it to the\n\
        \x20     // page itself, so reloading a page in a language you chose\n\
        \x20     // would bounce you out of it; and `back_forward` sets it to\n\
        \x20     // wherever you came from originally, so going back would\n\
        \x20     // redirect you forward again and the button would appear\n\
        \x20     // broken. Only a fresh `navigate` counts.\n\
        \x20     var entry = performance.getEntriesByType('navigation')[0];\n\
        \x20     var followed =\n\
        \x20       (!entry || entry.type === 'navigate') &&\n\
        \x20       document.referrer.indexOf(location.origin + '/') === 0;\n\
        \x20     if (!door && !followed) want = null;\n\
        \x20   }} else {{\n\
        \x20     want = stored;\n\
        \x20   }}\n\
        \x20   if (want !== null && want !== here && targets[want]) {{\n\
        \x20     location.replace(targets[want]);\n\
        \x20   }}\n\
        \x20 }} catch (e) {{\n\
        \x20   // No storage. The served locale stands, which is the correct\n\
        \x20   // degradation: this script only ever moves a reader who asked.\n\
        \x20 }}\n\
         }})();\n\
         </script>\n",
        key = defaults::LOCALE_KEY,
        auto = defaults::AUTOMATIC_LOCALE,
    )
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
        shell::Header { context }
        div { class: "nd-body",
            shell::Sidebar { context }
            shell::Scrim { context }
            // No footer. The one that was here said the same sentence on every
            // page — that the site is generated from the same specification the
            // system is — which is a fact about the build, not something a
            // reader came for.
            main { id: "nd-main", class: "nd-main", pages::Body { context } }
        }
    }
}

/// Rewrites the generated base links so they resolve from a page at
/// [`site::BASE`] — `<locale>/<slug>/index.html`, two levels down.
///
/// The alternative — every page at the root — would mean three locales sharing
/// one namespace, and `colors` could belong to only one of them.
///
/// Only **document-relative** hrefs move. A blind replace turns `https://…`
/// into `../https://…` and `/x` into `..//x`, which works right up until a base
/// link points off-site — and then fails silently, because a stylesheet that
/// 404s produces no error a reader can see.
fn rebase(html: &str) -> String {
    const MARK: &str = "href=\"";
    let mut out = String::with_capacity(html.len() + 64);
    let mut rest = html;
    while let Some(at) = rest.find(MARK) {
        out.push_str(&rest[..at + MARK.len()]);
        rest = &rest[at + MARK.len()..];
        if is_document_relative(rest) {
            out.push_str(site::BASE);
        }
    }
    out.push_str(rest);
    out
}

/// True when an href resolves against the document rather than against a host,
/// the site root, or the page itself — the only kind that has to move when the
/// document moves.
fn is_document_relative(href: &str) -> bool {
    if href.starts_with('/') || href.starts_with('#') {
        return false;
    }
    let scheme = href
        .find(|c: char| !c.is_ascii_alphanumeric() && !matches!(c, '+' | '-' | '.'))
        .unwrap_or(href.len());
    !(scheme > 0 && href[scheme..].starts_with(':'))
}

/// The root document, which sends a reader to their language.
///
/// The `<meta refresh>` still names the default locale, because that is the
/// answer with no script; [`locale_script`] gets there first when there is one.
fn redirect() -> String {
    let locale = Locale::DEFAULT;
    redirect_to(
        locale,
        &format!("{}/{}", locale.tag(), site::first(locale).dir()),
        // `here` is empty: the root belongs to no locale, so every resolved
        // answer differs from it and the door always opens somewhere.
        &locale_script("", &door_targets(""), true),
    )
}

/// Where each locale's front door leads, from a document `prefix` levels up.
///
/// The first entry in that locale's own sidebar, not a fixed page — the order
/// is the reader's alphabet, so the front door differs by locale. Never
/// `<locale>/` itself, which is this same door and would loop.
fn door_targets(prefix: &str) -> Vec<(&'static str, String)> {
    Locale::OFFERED
        .iter()
        .map(|locale| {
            (
                locale.tag(),
                format!("{prefix}{}/{}", locale.tag(), site::first(*locale).dir()),
            )
        })
        .collect()
}

/// A door: no title beyond the site's name, no content, no sidebar entry.
///
/// Deliberately not a page. The site had a landing page whose entire content
/// described the other pages — which is what the sidebar is — so it was
/// deleted rather than rewritten.
fn redirect_to(locale: Locale, target: &str, script: &str) -> String {
    format!(
        "<!doctype html>\n\
         <html lang=\"{lang}\">\n\
         <head>\n\
         <meta charset=\"utf-8\">\n\
         {script}\
         <meta http-equiv=\"refresh\" content=\"0; url={target}\">\n\
         <link rel=\"canonical\" href=\"{target}\">\n\
         <title>{name}</title>\n\
         </head>\n\
         <body><a href=\"{target}\">{name}</a></body>\n\
         </html>\n",
        lang = locale.bcp47(),
        name = locale.message(Key::AppName),
    )
}

/// Copies the generated stylesheets, the vendored palettes and the fonts.
/// The shipped browser behaviour, whole.
///
/// Each of these is a `dist/` file because the component it belongs to ships —
/// a colour picker that only picks colours inside this site is not a colour
/// picker — and `shell.js` imports every one of them from beside itself rather
/// than carrying a copy. Its own function so the list is one place and a
/// seventh file is one line.
fn copy_runtime(root: &Path, out: &Path) -> Result<usize, String> {
    let mut count = 0;
    for file in [
        "anchor.js",
        "color.js",
        "messages.js",
        "color-picker.js",
        "inputs.js",
        "carousel.js",
        "error-summary.js",
        "table.js",
        "info.js",
        "menu.js",
        "message-bar.js",
        "dialog.js",
        "popover.js",
        "tooltip.js",
        "toast.js",
    ] {
        count += copy(
            &root.join("dist").join(file),
            &out.join("assets").join(file),
        )?;
    }
    Ok(count)
}

fn copy_assets(root: &Path, out: &Path) -> Result<usize, String> {
    let mut count = 0usize;

    // The generated stylesheets, at the site root, which is where the base
    // links (rebased above) look for them.
    for name in [
        "tokens.css",
        "tones.css",
        "theme.css",
        "scope.css",
        "syntax.css",
        "favicon.svg",
    ] {
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

    // The vendored color output, under the path `spec/theme.toml` declares.
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

    // The shell's own files, and the generated modules the script imports
    // rather than restates.
    count += copy(
        &root.join("docs-site/assets/shell.css"),
        &out.join("assets/shell.css"),
    )?;
    count += copy(
        &root.join("docs-site/assets/shell.js"),
        &out.join("assets/shell.js"),
    )?;
    // The implementation switch. A file of its own because it runs only for a
    // reader who asks for it, and because everything it owns — a stored
    // preference, a lazy import, an observer and a failure mode — is easier to
    // reason about, and to switch off, in one place.
    count += copy(
        &root.join("docs-site/assets/implementation.js"),
        &out.join("assets/implementation.js"),
    )?;
    // The model stage: any glTF, file and parameters as inputs — the seed of
    // the upload feature, and the reason it must know nothing about owls.
    count += copy(
        &root.join("docs-site/assets/model.js"),
        &out.join("assets/model.js"),
    )?;
    // The matcher, which `shell.js` imports. A file of its own because it
    // touches no DOM and carries no word in any language — which is what will
    // let it move into a behaviour package unchanged on the day there is one.
    count += copy(
        &root.join("docs-site/assets/search-match.js"),
        &out.join("assets/search-match.js"),
    )?;
    // The rows the `deploys` example fetches when a reader asks for more. A
    // real file at a real address, because a *load more* whose next rows are
    // already in the page is a demonstration of nothing — and it is `<tr>`
    // markup rather than JSON for the reason `Table.more_src` gives.
    count += copy(
        &root.join("docs-site/assets/deploys-2.html"),
        &out.join("assets/deploys-2.html"),
    )?;
    count += copy(
        &root.join("packages/tokens/theme.js"),
        &out.join("assets/theme.js"),
    )?;
    // The `Lineage` runtime, and the three.js bundle it fetches on the first
    // interaction. Both are `dist/` files — this component's behaviour SHIPS —
    // and they keep their relative layout here, because the runtime imports
    // `./vendor/three-css3d.js` and that path has to mean the same thing in
    // `dist/` and on the site.
    count += copy(
        &root.join("dist/lineage.js"),
        &out.join("assets/lineage.js"),
    )?;
    count += copy(
        &root.join("dist/vendor/three-css3d.js"),
        &out.join("assets/vendor/three-css3d.js"),
    )?;
    count += copy_runtime(root, out)?;
    // One file per palette, and the picker fetches the ONE in force. All of
    // them together is 627 KB; the one a reader opened the Noctua tab to see is
    // sixteen. `noctua-colors` split its own stylesheets the same way and
    // recorded the same arithmetic.
    count += copy_dir(
        &root.join("dist/palettes"),
        &out.join("assets/palettes"),
        "json",
    )?;
    count += copy(
        &root.join("dist/vendor/LICENSE-three-MIT.txt"),
        &out.join("assets/vendor/LICENSE-three-MIT.txt"),
    )?;
    // AND THE WEBGL HALF, WHICH IS NOT A `dist/` FILE. The owl on the `Lineage`
    // page is an example of what a consumer may put inside a figure, not part
    // of the component — so it is copied from `assets/vendor/` straight to the
    // site, and a consumer of `Lineage` never fetches it. See its attribution.
    count += copy(
        &root.join("assets/vendor/three/three-webgl.js"),
        &out.join("assets/vendor/three-webgl.js"),
    )?;
    // The photographed owl the wild example loads — a scanned model, same
    // rule as the renderer above: an example's asset, never `dist/`'s. Copied
    // only when it exists, because the scan arrives from the reader (see
    // `assets/vendor/owl/ATTRIBUTION.md`) and a site built before it lands
    // simply keeps the flat portrait.
    if root.join("assets/vendor/owl/owl.glb").exists() {
        count += copy(
            &root.join("assets/vendor/owl/owl.glb"),
            &out.join("assets/vendor/owl.glb"),
        )?;
    }
    // The portraits the `Lineage` examples stand at their figures, so the
    // documentation shows a scene of people rather than a scene of initials.
    // Placeholders rather than photographs of anybody — see their attribution.
    count += copy_dir(
        &root.join("docs-site/assets/portraits"),
        &out.join("assets/portraits"),
        "svg",
    )?;
    // The React examples, bundled by esbuild: one entry, two shared chunks, and
    // two more fetched only by the pages that name a code block or a lineage.
    //
    // Optional on purpose: the site must build on a machine that has never run
    // `pnpm install`, because the Rust half of this repository owes nothing to
    // Node (`NOCTUA.md §9.11`). Without it the page renders no implementation
    // control at all rather than one that fails once per reader — see
    // `react_is_available`, which is what `document` reads to decide.
    count += copy_react(root, out)?;

    Ok(count)
}

/// The React bundle, copied where it exists and reported on where it does not.
///
/// Its own function because `copy_assets` is at clippy's line ceiling and this
/// is the one asset with something to say about itself.
fn copy_react(root: &Path, out: &Path) -> Result<usize, String> {
    let react = root.join("docs-site/assets/react");
    if !react.is_dir() {
        println!(
            "  note: docs-site/assets/react is absent — run \
             `pnpm --filter @noctua-world/docs-site run build:example`"
        );
        return Ok(0);
    }
    let count = copy_dir(&react, &out.join("assets/react"), "js")?;
    if let Some(stale) = react_is_stale(root, &react) {
        println!(
            "  note: the React bundle is older than {stale} — run \
             `pnpm --filter @noctua-world/docs-site run build:example`"
        );
    }
    Ok(count)
}

/// The newest file the React bundle packages, if the bundle is older than it.
///
/// # Why this is a note and not a failure
///
/// The bundle is gitignored, esbuild builds it, and `NOCTUA.md §9.11` says the
/// Rust half of this repository owes nothing to Node — so a machine that has
/// never run `pnpm install` has to be able to build the whole site. It cannot
/// rebuild the bundle, and failing here would make Node a requirement through
/// the back door.
///
/// # Why it is worth saying at all
///
/// The bundle went **five days stale** without anybody noticing, so every React
/// example in the browser was running a five-day-old package: a change to a
/// component reached the Dioxus half of every page and the React half of none,
/// and the one test that compares them compares a single example. That is the
/// same shape as a generated file drifting from its spec, and this repository
/// fails the build on that.
///
/// # The list of sources is esbuild's, not a guess
///
/// The first version of this walked `packages/react/src`, `packages/tokens` and
/// `docs-site/js` and asked which was newest. It cried wolf on the very first
/// build, naming `packages/tokens/i18n.js` — 1.2 MB of message catalogue that
/// the bundle **deliberately never imports**, and that a build assertion keeps
/// out of it. A guessed dependency list is a warning nobody believes twice.
///
/// `--metafile` writes exactly what was read, so that is what is read here.
fn react_is_stale(root: &Path, bundle: &Path) -> Option<String> {
    // `root` is `<manifest>/..`, which is not canonical, so a `strip_prefix`
    // against it silently fails and the note reads `../packages/…`.
    let here = root.canonicalize().ok()?;
    let built = newest_js(bundle)?.0;
    let meta = std::fs::read_to_string(bundle.join("meta.json")).ok()?;

    let mut worst: Option<(std::time::SystemTime, String)> = None;
    // EVERY QUOTED PATH IN THE FILE, not the first of each `"inputs"` block.
    //
    // Slicing to the next `}` was the obvious thing and it is wrong: that brace
    // closes the FIRST input's own object, so exactly one of a hundred and
    // three was ever read, and touching any other left the note silent. A
    // parser would be a dependency for one field (`NOCTUA.md §2.3`), so this
    // reads every string that names a `.js` and lets the filesystem say which
    // of them exist. The outputs are in there too and resolve to the bundle
    // itself, which is the baseline being compared against.
    for key in meta.split('"').skip(1).step_by(2) {
        // `Path::extension` rather than `ends_with`, which clippy reads as a
        // case-sensitive extension test — and it is right that this is one.
        if Path::new(key).extension().is_none_or(|e| e != "js") {
            continue;
        }
        // Paths are relative to `docs-site`, which is where esbuild ran.
        let path = root.join("docs-site").join(key);
        let Ok(when) = std::fs::metadata(&path).and_then(|m| m.modified()) else {
            continue;
        };
        if when > built && worst.as_ref().is_none_or(|(seen, _)| when > *seen) {
            worst = Some((
                when,
                path.canonicalize()
                    .ok()
                    .and_then(|p| p.strip_prefix(&here).ok().map(Path::to_path_buf))
                    .unwrap_or_else(|| PathBuf::from(key))
                    .display()
                    .to_string(),
            ));
        }
    }
    worst.map(|(_, what)| what)
}

/// The newest `.js` directly inside a directory, with the file it came from.
fn newest_js(dir: &Path) -> Option<(std::time::SystemTime, PathBuf)> {
    std::fs::read_dir(dir)
        .ok()?
        .flatten()
        .filter(|entry| entry.path().extension().is_some_and(|e| e == "js"))
        .filter_map(|entry| {
            let when = entry.metadata().and_then(|m| m.modified()).ok()?;
            Some((when, entry.path()))
        })
        .max_by_key(|(when, _)| *when)
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
            for page in site::pages() {
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
        // Read from the theme model rather than spelled here: the file this
        // names changes with the default palette, and a literal would keep
        // passing against a sheet the build no longer links.
        let palette = noctua_design_theme::Palette::DEFAULT.as_str();
        let default = head
            .find(&format!("{palette}.css"))
            .unwrap_or_else(|| panic!("the default palette, {palette}, is not linked:\n{head}"));
        let scope = head.find("scope.css").expect("the scoped contract");
        let tokens = head.find("tokens.css").expect("the design tokens");
        assert!(scope < default, "scope.css must precede the palette");
        assert!(default < tokens, "the palette must precede the tokens");
    }

    /// The default palette's sheet is scope-anchored, so the attribute that
    /// makes it apply has to be in the markup.
    ///
    /// Without it every page has no palette at all — not the wrong colors, no
    /// colors — and the failure looks like a missing stylesheet rather than a
    /// missing attribute.
    #[test]
    fn every_page_carries_the_default_palette_attribute() {
        let palette = noctua_design_theme::Palette::DEFAULT.as_str();
        for locale in Locale::ALL {
            for page in site::pages() {
                let html = document(locale, page);
                assert!(
                    html.contains(&format!("data-palette=\"{palette}\"")),
                    "{} in {} does not stamp the default palette",
                    page.slug,
                    locale.tag()
                );
            }
        }
    }

    /// The language layer is a Rust constant and a JavaScript one, and they
    /// have to be the same string.
    ///
    /// A storage key spelled two ways is a preference that is written and never
    /// read back — which reports nothing, looks like the setting not sticking,
    /// and is invisible in a review because neither file is wrong on its own.
    #[test]
    fn the_locale_layer_agrees_with_the_script() {
        let script = include_str!("../assets/shell.js");
        for (constant, value) in [
            ("LOCALE_KEY", defaults::LOCALE_KEY),
            ("AUTOMATIC_LOCALE", defaults::AUTOMATIC_LOCALE),
        ] {
            assert!(
                script.contains(&format!("const {constant} = \"{value}\";")),
                "shell.js does not declare {constant} as {value:?}"
            );
        }

        // And the inline resolver in <head> reads the same key.
        let html = document(Locale::DEFAULT, site::first(Locale::DEFAULT));
        assert!(
            html.contains(&format!("localStorage.getItem('{}')", defaults::LOCALE_KEY)),
            "the inline locale resolver reads a different key"
        );
    }

    /// The resolver may only ever reach a destination this build wrote.
    ///
    /// `NOCTUA.md §2.4`: nothing a reader controls — a stored value, a
    /// `navigator.languages` entry — is concatenated into a path. The script
    /// picks from a table written here, and this is the assertion that the
    /// table is all it has.
    #[test]
    fn the_locale_resolver_builds_no_url_of_its_own() {
        let html = document(Locale::En, site::first(Locale::En));
        // Matched on the read, not on the key: `data-noctua-locales` on <html>
        // contains `noctua-locale` as a substring, so a looser predicate finds
        // the document's own head and passes without ever seeing the script.
        let needle = format!("localStorage.getItem('{}')", defaults::LOCALE_KEY);
        let script = html
            .split("<script>")
            .find(|part| part.contains(&needle))
            .expect("the locale resolver");
        assert!(
            script.contains("targets[want]"),
            "the resolver does not read its destination from the table"
        );
        for forbidden in ["location.href =", "'/' + want", "want + '/'"] {
            assert!(
                !script.contains(forbidden),
                "the resolver builds a URL from a value it does not control: {forbidden}"
            );
        }
    }

    /// Every href has to move to the base exactly once, or the assets 404 from
    /// a page inside its own directory.
    #[test]
    fn rebasing_moves_every_href_and_nothing_else() {
        let head = rebase(BASE_LINKS);
        let marker = format!("href=\"{}", site::BASE);
        assert_eq!(
            head.matches(marker.as_str()).count(),
            BASE_LINKS.matches("href=\"").count()
        );
        assert!(
            !head.contains(&format!("href=\"{}{}", site::BASE, site::BASE)),
            "an href moved twice"
        );
    }

    /// EVERY SENTENCE ON EVERY PAGE COMES FROM THE CATALOGUE.
    ///
    /// # What the pseudo-locale is for, made mechanical
    ///
    /// `qps` is English with every vowel doubled and the whole message wrapped
    /// in `⟦…⟧`, generated from the source rather than written. Its purpose was
    /// always this: **anything not bracketed never went through the
    /// catalogue.** But checking it meant opening `/qps/` and reading, which
    /// nobody does twice — and thirteen component pages shipped English prop
    /// documentation in Portuguese for a whole stage.
    ///
    /// So the page is rendered and every text node is examined. What is skipped
    /// is skipped for a stated reason, and each exemption is an element or a
    /// class the markup carries deliberately:
    ///
    /// - `<code>`, `<pre>`, `<kbd>` and `<time>` — a token name, a fragment of
    ///   source, a key cap and a machine-readable stamp are identifiers.
    /// - `.nd-identifier` — the marker for everything else that is a name
    ///   rather than a sentence: a language's own name, a font family, a line of
    ///   Devanagari. If a page shows something untranslated, it says so here.
    ///
    /// A failure names the page and the text, because "something is not
    /// translated" sends somebody hunting and the sentence itself does not.
    #[test]
    fn every_sentence_on_every_page_is_translated() {
        let mut examined = 0usize;
        for page in site::pages() {
            let html = document(Locale::Qps, page);
            for text in visible_text(&html) {
                examined += 1;
                assert!(
                    text.contains('⟦'),
                    "{} shows text that never went through the catalogue: {text:?}",
                    page.slug
                );
            }
        }
        // A scanner that found nothing would pass this test on every page,
        // for ever, and report a property it never checked.
        assert!(
            examined > 500,
            "only {examined} runs of text were examined; the scanner is not reading the page"
        );
    }

    /// Every run of reader-facing text in a document.
    ///
    /// Deliberately a small scanner rather than a parser: this walks tags to
    /// find text between them, skips the subtrees named above, and drops
    /// anything with no letters in it. A dependency to read HTML this build
    /// wrote itself would be a dependency for one test (`NOCTUA.md §2.3`).
    fn visible_text(html: &str) -> Vec<String> {
        // Elements whose CONTENT is an identifier rather than a sentence: a
        // token name, a fragment of source, a key cap, a machine-readable
        // stamp, and the name of a work or a person.
        const SKIP: [&str; 5] = ["code", "pre", "kbd", "time", "cite"];
        // And the two whose content is not markup at all — see below.
        const RAW: [&str; 2] = ["script", "style"];

        let mut out = Vec::new();
        let mut rest = html;
        // The tag name of the subtree being skipped, and how deep we are in it.
        let mut skipping: Option<(String, usize)> = None;

        while let Some(open) = rest.find('<') {
            let text = &rest[..open];
            if skipping.is_none() && text.chars().any(char::is_alphabetic) {
                out.push(text.trim().to_owned());
            }
            rest = &rest[open + 1..];

            // A comment ends at `-->`, not at the first `>` — and the base
            // links carry a long one explaining the palette ordering, which a
            // naive scan reads as a tag and then as page text.
            if let Some(body) = rest.strip_prefix("!--") {
                rest = body.find("-->").map_or("", |at| &body[at + 3..]);
                continue;
            }

            let Some(close) = rest.find('>') else { break };
            let tag = &rest[..close];
            rest = &rest[close + 1..];

            let closing = tag.starts_with('/');
            let name: String = tag
                .trim_start_matches('/')
                .chars()
                .take_while(char::is_ascii_alphanumeric)
                .flat_map(char::to_lowercase)
                .collect();
            if name.is_empty() {
                continue;
            }

            // A void element closes itself, so it can never open a subtree.
            let void = tag.ends_with('/') || matches!(name.as_str(), "br" | "hr" | "img" | "input");

            // A `.nd-message` subtree is ONE message, however many runs it was
            // split into. `pages::Message` sets identifiers inside a sentence
            // as code, which cuts the sentence at every backtick — and the
            // pseudo-locale's brackets are only on the first and last piece.
            //
            // `.nd-slider-marks` IS THE SAME SHAPE FROM THE OTHER END.
            // `Slider.mark_labels` is one message the caller writes as
            // `Quiet|Loud`, and the component draws a tick per part — so the
            // brackets again land on the first and the last. It used to be the
            // runtime that split it, which is precisely the defect: the scale
            // a caller asked for was drawn by nobody until a script ran, and
            // this scanner passed because there was nothing on the page.
            if !closing
                && !void
                && skipping.is_none()
                && (tag.contains("nd-message") || tag.contains("nd-slider-marks"))
            {
                let (inside, after) = subtree(rest, &name);
                rest = after;
                let joined = strip_tags(inside);
                if joined.chars().any(char::is_alphabetic) {
                    out.push(joined);
                }
                continue;
            }

            // `<script>` AND `<style>` HOLD RAW TEXT, NOT MARKUP.
            //
            // Their content is skipped to the literal closing tag rather than
            // scanned, because scanning it does not work and fails silently in
            // the worst direction: `for (var i = 0; i < asked.length …` in the
            // locale resolver reads as an opening tag, the scan for its `>`
            // runs past `</script>`, and the skip never closes. Everything
            // after the first inline script was then invisible to this test —
            // which is to say, the whole page. The floor above is what caught
            // it; without the floor it would have passed for ever.
            if !closing && !void && RAW.contains(&name.as_str()) {
                let end = format!("</{name}>");
                rest = rest.find(&end).map_or("", |at| &rest[at + end.len()..]);
                continue;
            }

            if let Some((open_name, depth)) = &mut skipping {
                if &name == open_name && !void {
                    if closing {
                        *depth -= 1;
                        if *depth == 0 {
                            skipping = None;
                        }
                    } else {
                        *depth += 1;
                    }
                }
            } else {
                let identifier = tag.contains("nd-identifier");
                if !closing && !void && (SKIP.contains(&name.as_str()) || identifier) {
                    skipping = Some((name, 1));
                }
            }
        }
        out
    }

    /// Everything up to the tag that closes an already-open `<name>`, and
    /// whatever follows it.
    fn subtree<'a>(rest: &'a str, name: &str) -> (&'a str, &'a str) {
        let open = format!("<{name}");
        let close = format!("</{name}>");
        let mut depth = 1usize;
        let mut at = 0usize;
        while depth > 0 {
            let next_open = rest[at..].find(&open).map(|i| at + i);
            let Some(next_close) = rest[at..].find(&close).map(|i| at + i) else {
                return (&rest[at..], "");
            };
            if next_open.is_some_and(|i| i < next_close) {
                depth += 1;
                at = next_open.expect("checked") + open.len();
            } else {
                depth -= 1;
                at = next_close + close.len();
                if depth == 0 {
                    return (&rest[..next_close], &rest[at..]);
                }
            }
        }
        (rest, "")
    }

    /// The text of a fragment, with every tag taken out.
    fn strip_tags(html: &str) -> String {
        let mut out = String::with_capacity(html.len());
        let mut rest = html;
        while let Some(open) = rest.find('<') {
            out.push_str(&rest[..open]);
            rest = &rest[open + 1..];
            match rest.find('>') {
                Some(close) => rest = &rest[close + 1..],
                None => break,
            }
        }
        out.push_str(rest);
        out.split_whitespace().collect::<Vec<_>>().join(" ")
    }

    /// The scanner has to be right, or the test above proves nothing.
    #[test]
    fn the_text_scanner_reads_a_split_message_as_one() {
        let html = "<p><span class=\"nd-message\">⟦aa <code>x</code> bb⟧</span></p>";
        assert_eq!(visible_text(html), vec!["⟦aa x bb⟧"]);
    }

    /// The scanner has to be right, or the test above proves nothing.
    #[test]
    fn the_text_scanner_skips_what_it_says_it_skips() {
        let html = "<p>⟦sentence⟧ <code>--token</code> and \
                    <span class=\"nd-identifier\">Português</span></p>\
                    <pre><span>raw</span></pre><p>⟦another⟧</p>";
        assert_eq!(visible_text(html), vec!["⟦sentence⟧", "and", "⟦another⟧"]);
    }

    /// And it must not lose a nested tag inside a skipped subtree, or the text
    /// after that subtree is read as though it were inside it.
    #[test]
    fn the_text_scanner_closes_a_nested_subtree_once() {
        let html = "<code><span>a</span><span>b</span></code><p>⟦after⟧</p>";
        assert_eq!(visible_text(html), vec!["⟦after⟧"]);
    }

    /// An href that does not resolve against the document must not move.
    #[test]
    fn rebasing_leaves_absolute_hrefs_alone() {
        let input = "<a href=\"https://example.invalid/x\"><a href=\"/root\">\
                     <a href=\"#frag\"><a href=\"mailto:a@b.c\"><link href=\"tokens.css\">";
        let out = rebase(input);
        assert!(out.contains("href=\"https://example.invalid/x\""));
        assert!(out.contains("href=\"/root\""));
        assert!(out.contains("href=\"#frag\""));
        assert!(out.contains("href=\"mailto:a@b.c\""));
        assert!(
            out.contains(&format!("href=\"{}tokens.css\"", site::BASE)),
            "{out}"
        );
    }

    /// The stored palette's stylesheet is injected by the bootstrap, from a
    /// page one directory down. If that href is not rebased it 404s, the page
    /// paints the default palette, and a deferred script snaps it over after
    /// first paint — the flash the bootstrap exists to prevent.
    #[test]
    fn the_bootstrap_resolves_the_palette_against_the_declared_base() {
        let html = document(Locale::PtBr, site::first(Locale::En));
        let base = html
            .split("data-noctua-base=\"")
            .nth(1)
            .and_then(|rest| rest.split('"').next())
            .expect("the document declares a base");
        assert!(
            html.contains("getAttribute('data-noctua-base')"),
            "the bootstrap ignores the base the document declares"
        );
        assert!(
            html.contains("sheet.href = siteBase +"),
            "the palette href is not rebased and will 404 from {base}"
        );
    }

    /// A page with the script blocked must still be a complete page.
    #[test]
    fn the_navigation_is_in_the_markup() {
        let html = document(Locale::En, site::first(Locale::En));
        let body = html.split("<body>").nth(1).expect("a body");
        assert!(
            body.contains("nd-nav-link"),
            "the navigation is not in the markup"
        );
        assert!(body.contains("nd-skip"), "there is no skip link");
    }

    #[test]
    fn the_scoped_theme_page_carries_a_real_nested_scope() {
        let page = site::THEME_SCOPE;
        let html = document(Locale::En, page);
        // Two scope ELEMENTS, each with its palette and its mode on the same
        // element — which is the whole contract in one assertion.
        //
        // Matched with the `=""` rather than on the bare name: the attribute is
        // also one of the searchable names in the sidebar, and counting the
        // string alone made this test fail the moment search learned about it.
        // Three: the demo scope, the scope nested inside it, and the one in
        // the example frame — the same scope again, in a frame that can mount
        // it in either implementation.
        assert_eq!(html.matches("data-noctua-scope=\"\"").count(), 3, "{html}");
        assert!(html.contains("data-palette=\"azure-vivid\""));
        assert!(html.contains("data-palette=\"rose-sober\""));
    }

    /// The site must be reproducible, or "it works on my machine" becomes a
    /// property of the documentation as well as of the build.
    #[test]
    fn the_same_inputs_produce_the_same_page() {
        for locale in Locale::ALL {
            for page in site::pages() {
                assert_eq!(document(locale, page), document(locale, page));
            }
        }
    }

    #[test]
    fn the_language_control_points_at_the_same_page() {
        for page in site::pages() {
            let html = document(Locale::En, page);
            assert!(
                html.contains(&page.locale_href(Locale::PtBr)),
                "{} does not link to its translation",
                page.slug
            );
        }
    }

    /// Resolves an href against the directory the document sits in, collapsing
    /// `..`. Returns `None` if it climbs above the site root.
    fn resolve_from(dir: &str, href: &str) -> Option<String> {
        let mut parts: Vec<&str> = dir.split('/').filter(|s| !s.is_empty()).collect();
        for segment in href.split('/') {
            match segment {
                "" | "." => {}
                ".." => {
                    parts.pop()?;
                }
                other => parts.push(other),
            }
        }
        Some(parts.join("/"))
    }

    /// Every internal link, resolved the way a browser would.
    ///
    /// The suite that existed typed URLs into `page.goto` instead of following
    /// the site's own links, so nothing ever exercised an href — which is how
    /// `index.html` stayed in every URL and how a palette stylesheet 404ed on
    /// every load. This one costs no browser and no built directory.
    #[test]
    fn every_internal_link_resolves_to_something_the_build_writes() {
        // What the build writes, in the same shape a resolved href produces.
        let mut written: Vec<String> = Vec::new();
        for locale in Locale::ALL {
            for page in site::pages() {
                written.push(format!("{}/{}", locale.tag(), page.dir()));
            }
            written.push(format!("{}/", locale.tag()));
        }

        for locale in Locale::ALL {
            for page in site::pages() {
                let dir = format!("{}/{}", locale.tag(), page.slug);
                for href in hrefs(&document(locale, page)) {
                    // Off-site, in-page and root-absolute links resolve against
                    // something other than this document; `rebase` leaves them
                    // alone and so does this.
                    if !is_document_relative(&href) {
                        continue;
                    }
                    assert!(
                        !href.contains(".html"),
                        "{dir} links to {href}, and a reader should never see a file name"
                    );
                    let resolved = resolve_from(&dir, &href)
                        .unwrap_or_else(|| panic!("{dir} links to {href}, which escapes the site"));
                    // Asset paths are covered by the browser suite's "no
                    // response ≥ 400" spec, which runs against the real files.
                    // What this asserts is the part that has no file on disk to
                    // check against: a link to a PAGE.
                    if href.ends_with('/') {
                        assert!(
                            written.contains(&format!("{resolved}/")),
                            "{dir} links to {href}, which is not a page the build writes"
                        );
                    }
                }
            }
        }
    }

    /// Every `href="…"` in a document, in order.
    fn hrefs(html: &str) -> Vec<String> {
        html.split("href=\"")
            .skip(1)
            .filter_map(|rest| rest.split('"').next())
            .map(str::to_owned)
            .collect()
    }
}
