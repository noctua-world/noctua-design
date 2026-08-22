// Following a link without throwing the page away.
//
// # What this is for
//
// Every page of this site is a whole document, and the documents are nearly the
// same document. Diffing everything outside `<main>` between two built pages
// gives five differences and nothing else: the `<title>`, the description, the
// three hrefs in the language menu, and the two sidebar rows that trade
// `nd-nav-link--current` between them. Every stylesheet link, the header, the
// scrim and the whole of the navigation are byte-identical.
//
// So a full load rebuilds the sidebar the reader is navigating BY — losing
// wherever they had scrolled it to — re-parses a stylesheet the browser already
// has, and re-runs a module graph that had not moved, in order to change the
// region to the right of it. This replaces that region and leaves the rest
// alone.
//
// # It is an interception of a real link, and nothing else
//
// There is no route table here, no matcher and no rendering. Every destination
// is an `<a href>` the build wrote, pointing at a static file that exists; with
// the script blocked, or with this module absent, every one of them navigates.
// That is the same standing the language menu has (`shell.rs`) and the same
// rule the search palette follows — an enhancement is allowed to be faster,
// never to be the only way.
//
// # Its own module
//
// `shell.js` is the page's behaviour. This owns a history entry, a fetch, a
// parser and a failure mode, which is the same four-things argument that put
// the implementation switch in a file of its own. It reaches nothing inside
// `shell.js`: what it is given is `adopt` and `abandon`, and what it hands back
// is markup.
//
// # What routing changes for every enhancer on this site
//
// A behaviour that cannot be taken back off is a real LEAK now, not a note in a
// header comment. Three shapes fail here and nowhere else:
//
//  1. AN ENHANCER WITH NO RELEASE. `enhanceAll`/`releaseAll` in `shell.js` are
//     the two lists and `adopt`/`abandon` are what this file calls. Every new
//     enhancer belongs in both. `release.spec.js` and `navigation.spec.js` each
//     ask in a different way, and the second asks through the real path.
//  2. A FUNCTION CALLED WITH THE STATE IT IS ALREADY IN MUST DO NOTHING.
//     Anything that exchanges rather than sets has this shape: a toggle that
//     swapped its two labels on every call was invisible while a link took the
//     page with it, and became a control announcing the opposite of what it
//     does the moment the page stopped being thrown away.
//  3. A REGISTRY FILLED AT MODULE LEVEL IS FILLED ONCE, so anything that
//     arrives in a second render is in nobody's list. Fill it from a function
//     the router can call again.
//
// THE CLAIM TO KEEP: a page reached by clicking is `document.body.outerHTML`-
// identical to the same page reached by loading it. `navigation.spec.js`
// asserts exactly that, with one documented exception. If you change what the
// shell renders per page, that test is what tells you this file has to copy it.
//
// # It never resolves the language
//
// The inline script in `<head>` does, and stamps `data-noctua-locale-wanted`;
// this module refuses to install when that differs from `data-noctua-locale`,
// so a reader whose preference points elsewhere keeps the full load and the
// redirect that has always followed it.
//
// And that resolution has nothing to say about `qps`. A locale nobody is
// offered is not a reader's: the pseudo-locale is absent from the language
// menu, from `data-noctua-locales` and from every link on the site, so the only
// way to be there is to have typed it — but it is a routed locale like any
// other once you are. A resolver that bounces every link out of `/qps/` makes
// the one locale that exists to break layouts unbrowsable.

/// The region that changes, and the only one.
const MAIN = "#nd-main";

/// What the head of a fetched document is read for.
///
/// The measured list, not a guessed one — see the note at the top of this file.
/// Each is COPIED from the fetched document rather than recomputed here: a rule
/// that derives a title from a slug is a second renderer, and the day it
/// disagrees with `main.rs` the page lies about what it is showing.
const HEAD = ["title", 'meta[name="description"]'];

/// The language menu's entries, whose hrefs name THIS page in another locale.
const LOCALE_LINK = "[data-nd-locale]";

/// A sidebar row, and the two attributes that say which one is current.
const NAV_LINK = ".nd-nav-link";

let hooks = null;
let localeRoot = null;

/// The page on the screen, which is not always the page in the address bar.
///
/// A fragment link pushes a history entry of its own, so going back from
/// `/table/#sorting` to `/table/` is a `popstate` for a page already on the
/// screen. Refetching there would throw away every fold the reader had opened
/// to answer a question the browser has already answered.
let showing = null;

/// The navigation in flight, so a reader who clicks twice gets the second page.
///
/// Not a lock: a second click during a slow fetch is a reader who changed their
/// mind, and refusing it would leave them looking at a page they no longer
/// want. The first fetch is abandoned instead.
let inflight = null;

/**
 * Wires the router, unless this document is one a redirect is about to leave.
 *
 * # The language is resolved once, in `<head>`, and this must not restate it
 *
 * `main.rs`'s inline script sends a reader to their stored locale before the
 * first paint, and its rule depends on whether the navigation was FOLLOWED from
 * inside the site — a distinction a client-side navigation would quietly erase,
 * because there is no navigation left for it to inspect. A reader with `pt-BR`
 * stored who opened a shared `/en/…` link is redirected on their next click
 * today; with this module installed and unaware, they would stay in English for
 * the rest of the visit.
 *
 * Copying that resolution into JavaScript here would be a second copy of it,
 * which is the thing this repository refuses everywhere else. So the inline
 * script STAMPS WHAT IT RESOLVED, and this reads the one attribute: where the
 * reader's preference points somewhere other than the document they are on, the
 * router does not install at all and every click is a full load, decided by the
 * script that has always decided it.
 */
export function install(given) {
  hooks = given;

  const root = document.documentElement;
  const here = root.dataset.noctuaLocale;
  const wanted = root.dataset.noctuaLocaleWanted;
  if (!here || (wanted && wanted !== here)) return;

  // The site root, from the one place it is already written. `data-noctua-base`
  // is `../../` — every document sits at `<locale>/<slug>/`, which `site.rs`
  // asserts — so this resolves to the same absolute URL from every page, and
  // the locale root under it is what decides whether a link is ours.
  const base = new URL(root.dataset.noctuaBase || "./", location.href);
  localeRoot = new URL(`${here}/`, base);
  showing = location.pathname;

  takeOverScrolling();
  document.addEventListener("click", onClick);
  addEventListener("popstate", onPop);
}

/// How long after the last scroll event the offset is written down.
///
/// A pause, not a frame: this is a `replaceState` per scroll gesture rather than
/// per scroll event, and a reader flicking down a page produces one write.
const SETTLE = 150; // allow-literal: a debounce for a history write, not a drawn duration — it reaches no stylesheet and no token describes "how long a scroll has stopped for"

/**
 * Scroll position becomes this file's, in all three directions.
 *
 * The browser restores it on a back or a forward BEFORE the content it belongs
 * to exists, so it would measure against the page being left. Turning that off
 * means turning it off for the ordinary cases too — Back, Forward *and* the
 * reload the browser used to handle for free. Taking half of this over is what
 * would lose a reader their place.
 *
 * # THE OFFSET IS WRITTEN WHILE THE READER SCROLLS, AND THAT IS NOT A CHOICE
 *
 * The obvious place is `pagehide`, and it does not work: measured in Chrome, a
 * `replaceState` there succeeds in the outgoing document and the reloaded one
 * still reads the value from before it. The entry's state is snapshotted when
 * the reload is *initiated*, which is earlier than any unload handler. So the
 * only write that survives a reload is one that already happened, and the
 * history entry has to know where the reader is at all times rather than being
 * told on the way out.
 *
 * `pagehide` is kept anyway, for the gesture that never settled.
 *
 * # A fragment outranks a stored offset
 *
 * `shell.js`'s `revealTarget` opens whatever the address bar points into and
 * scrolls there, and a restoration on top of that would undo it.
 */
function takeOverScrolling() {
  const y = history.state?.y;
  if ("scrollRestoration" in history) history.scrollRestoration = "manual";

  let settling = 0;
  const remember = () => {
    clearTimeout(settling);
    // `scrollY` is read when the timer fires rather than when it is set, so a
    // write scheduled just before a navigation commits lands on the new entry
    // with the new page's offset — which is the value that entry wants anyway.
    settling = setTimeout(() => history.replaceState({ y: scrollY }, ""), SETTLE);
  };
  addEventListener("scroll", remember, { passive: true });
  addEventListener("pagehide", () => history.replaceState({ y: scrollY }, ""));

  if (typeof y !== "number" || location.hash) {
    history.replaceState({ y: scrollY }, "");
    return;
  }
  // NOT NOW. Modules run before the document has its final height, so scrolling
  // here lands short on any page taller than what has been laid out.
  addEventListener("load", () => {
    scrollTo(0, y);
    // AND AGAIN ONCE THE FACES ARE IN. A page set in a fallback face is a
    // different height from the same page set in Noto, and `load` does not wait
    // for a font. Only where the reader has not moved since: putting somebody
    // back who has already started reading is worse than landing them short.
    const landed = scrollY;
    void document.fonts?.ready.then(() => {
      if (scrollY === landed) scrollTo(0, y);
    });
  }, { once: true });
}

/**
 * Whether this click is one the browser should keep.
 *
 * Every test here is about the reader's intent or the platform's contract, and
 * failing any of them means doing nothing at all — the event is not touched, so
 * the browser does exactly what it would have done with this file absent.
 */
function ordinary(event, link) {
  // A middle click opens a tab and a modified click is a reader asking for
  // something other than *go there*: a new window, a download, a bookmark.
  if (event.button !== 0) return false;
  if (event.metaKey || event.ctrlKey || event.shiftKey || event.altKey) return false;
  // Something upstream — a component's own handler — has already answered.
  if (event.defaultPrevented) return false;
  if (link.target || link.hasAttribute("download")) return false;
  // `href` reflects the resolved absolute URL, so this covers `mailto:` and
  // every other scheme without naming one.
  return link.origin === location.origin;
}

/**
 * The page this link leads to, or `null` where it leads somewhere else.
 *
 * A DESTINATION IS A PAGE IN THIS LOCALE, and the locale half is not a
 * restriction that could be lifted later. A document in another language is a
 * different `lang`, a different `dir`, and a different word in every control in
 * the header and every row in the sidebar — so "reload only the content" has
 * nothing left to mean there, and a full load is both simpler and correct.
 *
 * A fragment on the page the reader is already on is left to the browser, which
 * scrolls and updates the address bar; `shell.js` listens for `hashchange` and
 * opens whatever the fragment landed inside.
 */
function destination(link) {
  const url = new URL(link.href);
  if (!url.href.startsWith(localeRoot.href)) return null;
  // One segment under the locale, ending in a slash: `<slug>/`. A deeper path
  // is an asset — `assets/deploys-2.html` is fetched by a table on one of these
  // pages — and a bare file name is not a page this build writes.
  if (!/^[^/]+\/$/.test(url.pathname.slice(localeRoot.pathname.length))) return null;
  if (url.pathname === location.pathname) return null;
  return url;
}

function onClick(event) {
  const link = event.target.closest?.("a[href]");
  if (!link || !ordinary(event, link)) return;
  const url = destination(link);
  if (!url) return;
  event.preventDefault();
  void go(url);
}

/**
 * Goes there: fetch, then history, then swap.
 *
 * THE HISTORY ENTRY IS WRITTEN AFTER THE CONTENT ARRIVES AND BEFORE IT IS PUT
 * IN, and both halves of that are load-bearing. Pushing on the press would put
 * the address bar ahead of the page, so a failed fetch would leave the reader
 * on the old page at the new URL with a back button that goes somewhere they
 * have never been. Pushing after the swap would run `adopt` — and with it
 * `revealTarget` — against the OLD address, so a link carrying a fragment would
 * land nowhere and open nothing.
 */
async function go(url) {
  const main = document.querySelector(MAIN);
  if (!main) return;

  // Where the reader is leaving from, kept on the entry they are leaving.
  history.replaceState({ y: scrollY }, "");

  // NOTHING IS CLOSED HERE, AND THAT IS DELIBERATE. A link can sit inside a
  // dialog, a menu, a popover or a fold, and a router that reached in to shut
  // each of them would need a line per surface and would be wrong about the
  // ones it did not know. What a surface does when the answer it was opened for
  // is chosen belongs to that surface: the ⌘K palette closes and clears itself
  // where its own result list is clicked, and the drawer already closed itself
  // on any link inside the navigation long before this file existed.
  const arrived = await load(main, url);
  if (!arrived) return;

  history.pushState({ y: 0 }, "", url);
  showing = url.pathname;
  swap(main, arrived);
  land(main);
}

/**
 * Fetches a page and parses it, or finishes the navigation the hard way.
 *
 * THE BROWSER FINISHES WHAT THIS COULD NOT. A router that reports its own
 * failure is a router asking the reader to understand it; `location.assign`
 * puts them exactly where the link said, and if the network is the problem they
 * get the browser's own page about it, in their own language.
 *
 * A page on this site can be 1.8 MB, so `aria-busy` is on `<main>` for the
 * whole of it — the platform's own way to say *this content is not the answer
 * yet*, needing no spinner, no timer and no sentence that would have to exist
 * in three locales to say something the reader can already see.
 */
async function load(main, url) {
  const attempt = new AbortController();
  inflight?.abort();
  inflight = attempt;
  main.setAttribute("aria-busy", "true");
  try {
    const response = await fetch(url, { signal: attempt.signal });
    if (!response.ok) throw new Error(`${url.pathname}: ${response.status}`);
    // INERT. `DOMParser` builds a document with no browsing context: the two
    // inline scripts in its head do not run, the module at the foot of its body
    // is not fetched, and no image or font in it is requested until a node is
    // adopted into this document. That is what makes swapping a page cost one
    // request rather than one request and a second theme bootstrap.
    const arrived = new DOMParser().parseFromString(await response.text(), "text/html");
    if (!arrived.querySelector(MAIN)) throw new Error(`${url.pathname}: no ${MAIN}`);
    return arrived;
  } catch (error) {
    // An abandoned fetch is a reader who clicked again, not a failure. The
    // second navigation owns the page from here.
    if (attempt.signal.aborted) return null;
    console.error("[noctua] falling back to a full load", error);
    location.assign(url);
    return null;
  } finally {
    if (inflight === attempt) {
      inflight = null;
      main.removeAttribute("aria-busy");
    }
  }
}

/**
 * Puts the new page in, and the old one out.
 *
 * `<main>` ITSELF STAYS. Its children are replaced, so `#nd-main` — which the
 * skip link at the top of every page points at, and which `aria-busy` above is
 * written on — is one element for the life of the visit.
 */
function swap(main, arrived) {
  hooks.abandon(main);
  main.replaceChildren(...arrived.querySelector(MAIN).childNodes);
  hooks.adopt(main);

  for (const selector of HEAD) {
    const from = arrived.querySelector(selector);
    const to = document.querySelector(selector);
    // A `<title>` carries text and a `<meta>` carries an attribute. Both are
    // replaced outright rather than read for their one field, which is what
    // makes this a list of selectors instead of a list of special cases.
    if (from && to) to.replaceWith(from.cloneNode(true));
  }

  // The language menu names THIS page in the other locale, so its three hrefs
  // move with the page. Zipped by position: both documents are the same locale
  // built from the same list, so the two sets are the same length in the same
  // order, and matching them by anything else would be a rule with nothing
  // asserting it.
  copyAttributes(arrived, LOCALE_LINK, ["href"]);
  // And the row a reader is standing on. Both attributes, because the class
  // draws it and `aria-current` announces it, and a page that moved one of them
  // would be a page that looks right and reads wrong.
  copyAttributes(arrived, NAV_LINK, ["class", "aria-current"]);
}

/** Copies one attribute set from the fetched document onto the live one. */
function copyAttributes(arrived, selector, names) {
  const from = arrived.querySelectorAll(selector);
  const to = document.querySelectorAll(selector);
  if (from.length !== to.length) return;
  for (const [at, element] of to.entries()) {
    for (const name of names) {
      const value = from[at].getAttribute(name);
      if (value === null) element.removeAttribute(name);
      else element.setAttribute(name, value);
    }
  }
}

/**
 * Arriving: the top of the page, and the focus on what the reader came for.
 *
 * A CLIENT-SIDE NAVIGATION MOVES NOBODY BY ITSELF. The browser's own is what
 * puts the caret at the start of the new document and lets a screen reader
 * announce it; replacing a subtree does neither, so a reader using the keyboard
 * would be left standing on a link in a sidebar that now leads somewhere else,
 * with nothing said about where they are.
 *
 * The heading, not a live region. `<h1 class="nd-title">` is the page's own name
 * in the reader's own language, already on the page and already translated — so
 * announcing it needs no new sentence in the catalogue, and focusing it puts the
 * keyboard at the top of the content rather than at the top of the chrome.
 * `tabindex="-1"` is written here rather than in the markup because it belongs
 * to this behaviour, and a page with no script should not carry it.
 *
 * A LINK WITH A FRAGMENT ASKED FOR SOMEWHERE ELSE. `adopt` has already run
 * `revealTarget`, which opens every fold above the target and scrolls to it, so
 * this must not scroll to the top over the answer — and the focus belongs on
 * what was asked for rather than on the page's name.
 */
function land(main) {
  const at = location.hash && document.getElementById(decodeURIComponent(location.hash.slice(1)));
  if (!at) scrollTo(0, 0);
  const arriving = at || main.querySelector("h1") || main;
  arriving.setAttribute("tabindex", "-1");
  arriving.focus({ preventScroll: true });
}

/**
 * Back and forward: the same swap, with the scroll put back rather than reset.
 *
 * A `popstate` for a fragment on the page already showing is not ours. The
 * browser has moved already, `hashchange` fires, and `shell.js` opens whatever
 * it landed in — while refetching here would throw away every fold the reader
 * had opened to get there.
 *
 * FOCUS IS NOT MOVED. A browser going back does not move it either, and a
 * reader who pressed Back to check something has not asked to be sent to the
 * top of the page.
 */
function onPop() {
  const main = document.querySelector(MAIN);
  if (!main || location.pathname === showing) return;
  void back(main, new URL(location.href), history.state?.y ?? 0);
}

async function back(main, url, y) {
  const arrived = await load(main, url);
  if (!arrived) return;
  showing = url.pathname;
  swap(main, arrived);
  // WHERE THEY WERE, NOT THE TOP. That is the whole difference between going
  // back and going somewhere: a reader returning to a page they had read half
  // of expects to be half way down it.
  scrollTo(0, y);
}
