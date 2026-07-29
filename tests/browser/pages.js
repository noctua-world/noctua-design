// The site's URLs, in one place.
//
// Every spec imports from here rather than typing a path, so changing the URL
// scheme is one edit instead of forty-eight. It was forty-eight, spread over
// six files, and that is what made a wrong URL cheap to ship and expensive to
// correct.
//
// There is deliberately no entry for `/<locale>/`. It is a redirect document —
// a door, not a page — and a spec that navigates to it loses its execution
// context the moment the meta refresh fires, which surfaces as the useless
// "Execution context was destroyed" rather than as anything about the site.

/** The locales that are built, including the pseudo-locale. */
export const LOCALES = ["en", "pt-BR", "qps"];

/** The locales a reader is offered. `qps` exists to be inspected, not read. */
export const OFFERED = ["en", "pt-BR"];

/** Every real page, by the name a spec wants to say. */
export const PAGE = {
  colours: "colours/",
  components: "components/",
  effects: "effects/",
  icons: "icons/",
  scoped: "scoped-themes/",
  spacing: "spacing/",
  typography: "typography/",
};

/** The URL of a page in a locale. */
export const url = (page, locale = "en") => `/${locale}/${page}`;

/**
 * A page for tests about the *shell* rather than about content.
 *
 * Spacing on purpose: it is the lightest of them — no palette grid, no font
 * specimens — so a test about the header is not also a test of 39 stylesheets.
 */
export const ANY = PAGE.spacing;
