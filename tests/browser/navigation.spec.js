// Following a link without throwing the page away.
//
// THE CLAIM THIS FILE IS NAMED AFTER: a page reached by clicking is the same
// page as one reached by loading it. Everything else here is a consequence of
// that, and the first test states it directly — `document.body.outerHTML` after
// a click compared with `document.body.outerHTML` after a `goto`, byte for
// byte. It is the one assertion that covers the whole class of "the router
// forgot to update X" without anybody having to think of X first, and it is
// what the sidebar's `aria-current`, the language menu's three hrefs and the
// title are checked by rather than each being remembered separately.
//
// The two things it cannot see are checked beside it: `<title>` and the
// description are not in `<body>`, and "the sidebar was not reloaded" is a
// statement about element IDENTITY that no amount of matching HTML can make.

import { expect, test } from "@playwright/test";

import { PAGE, url, showParts } from "./pages.js";

/** A sidebar row, by the page it leads to. Every href is `../<slug>/`. */
const row = (page, slug) => page.locator(`.nd-nav-link[href="../${slug}"]`);

/**
 * The whole page, as the browser has it after everything has run.
 *
 * `outerHTML` rather than a list of properties: a router's failures are things
 * it did not do, and a list can only contain what somebody thought of.
 */
const shot = (page) => page.evaluate(() => document.body.outerHTML);

/**
 * What legitimately differs between arriving and loading, normalised away.
 *
 * ONE ENTRY, AND IT IS THE ROUTER'S OWN. `land()` puts `tabindex="-1"` on the
 * heading it focuses, because a client-side navigation moves nobody by itself
 * and a reader on the keyboard would otherwise be left standing in the sidebar.
 * A full load has the browser do that, and the browser needs no attribute.
 *
 * Anything else appearing here is a defect until somebody writes down why it is
 * not. That is the entire value of a byte-for-byte comparison, and softening it
 * one regex at a time is how it stops being worth running.
 */
const settle = (html) => html.replace(/ tabindex="-1"/g, "");

test("a page reached by clicking is the page reached by loading it", async ({ page }) => {
  await page.goto(url(PAGE.button));
  await row(page, PAGE.select).click();
  await expect(page).toHaveURL(new RegExp(`/en/${PAGE.select}$`));
  const arrived = await shot(page);

  await page.goto(url(PAGE.select));
  const loaded = await shot(page);

  expect(settle(arrived)).toBe(settle(loaded));
});

test("the head moves with the body", async ({ page }) => {
  await page.goto(url(PAGE.button));
  const description = () => page.locator('meta[name="description"]').getAttribute("content");
  const was = await description();

  await row(page, PAGE.select).click();
  await expect(page).toHaveTitle(/^Select — /);
  expect(await description()).not.toBe(was);

  // The language menu names THIS page in the other language. A menu still
  // pointing at the page the reader came from is the quietest way to send
  // somebody to the wrong document, because the control looks correct.
  for (const locale of ["en", "pt-BR"]) {
    await expect(page.locator(`[data-nd-locale="${locale}"]`))
      .toHaveAttribute("href", `../../${locale}/${PAGE.select}`);
  }

  await page.goto(url(PAGE.select));
  const loaded = await description();
  await page.goto(url(PAGE.button));
  await row(page, PAGE.select).click();
  await expect.poll(description).toBe(loaded);
});

// --- The thing this was asked for -------------------------------------------

test("the sidebar is the same element, at the same scroll offset", async ({ page }) => {
  await page.goto(url(PAGE.button));

  const before = await page.$("#nd-nav");
  // Down a list of sixty-four rows, which is where a reader comparing two
  // components actually is. A sidebar that reloads comes back at the top, and
  // the reader has to find their place again on every single page.
  //
  // Scrolled to the row that is about to be CLICKED, and that is not a
  // convenience: Playwright brings a target into view before pressing it, so
  // scrolling anywhere else would have the click move the sidebar itself and
  // the assertion below would be measuring the test rather than the router.
  const offset = await page.evaluate((slug) => {
    const nav = document.querySelector("#nd-nav");
    nav.querySelector(`a[href="../${slug}"]`).scrollIntoView({ block: "center" });
    return nav.scrollTop;
  }, PAGE.select);
  expect(offset).toBeGreaterThan(0);

  await row(page, PAGE.select).click();
  await expect(page.locator("h1")).toHaveText("Select");

  const after = await page.$("#nd-nav");
  // IDENTITY, NOT EQUALITY. A sidebar rebuilt from identical markup would pass
  // every assertion about its contents and would still have blinked, and would
  // still have lost the offset below.
  expect(await before.evaluate((was, is) => was === is, after)).toBe(true);
  expect(await page.evaluate(() => document.querySelector("#nd-nav").scrollTop)).toBe(offset);
});

test("the current row moves, and only the current row", async ({ page }) => {
  await page.goto(url(PAGE.button));
  await expect(row(page, PAGE.button)).toHaveAttribute("aria-current", "page");

  await row(page, PAGE.select).click();
  await expect(row(page, PAGE.select)).toHaveAttribute("aria-current", "page");
  await expect(row(page, PAGE.button)).not.toHaveAttribute("aria-current", "page");
  expect(await page.locator(".nd-nav-link--current").count()).toBe(1);
});

test("one request per navigation, and the shell is not fetched again", async ({ page }) => {
  await page.goto(url(PAGE.button));
  await page.waitForLoadState("networkidle");

  const asked = [];
  page.on("request", (request) => asked.push(new URL(request.url()).pathname));
  await row(page, PAGE.select).click();
  await expect(page.locator("h1")).toHaveText("Select");
  await page.waitForLoadState("networkidle");

  // The page itself, and whatever a font stack asks for once it sees the new
  // text. Never the stylesheets, never `shell.js`, and never the eighteen
  // runtime modules it imports — which is the whole saving.
  expect(asked).toContain(`/en/${PAGE.select}`);
  expect(asked.filter((path) => path.endsWith(".css"))).toEqual([]);
  expect(asked.filter((path) => path.endsWith(".js"))).toEqual([]);
});

// --- History ----------------------------------------------------------------

test("back and forward land on the right page, at the right offset", async ({ page }) => {
  await page.goto(url(PAGE.button));
  await showParts(page);
  const y = await page.evaluate(() => {
    scrollTo(0, 400);
    return scrollY;
  });
  expect(y).toBeGreaterThan(0);

  await row(page, PAGE.select).click();
  await expect(page.locator("h1")).toHaveText("Select");
  expect(await page.evaluate(() => scrollY)).toBe(0);

  await page.goBack();
  await expect(page.locator("h1")).toHaveText("Button");
  // WHERE THEY WERE, NOT THE TOP. The browser's own restoration is off — it
  // would have measured against the page being left — so this offset is
  // entirely the router's, and it is the half that is easy to forget.
  await expect.poll(() => page.evaluate(() => scrollY)).toBe(y);

  await page.goForward();
  await expect(page.locator("h1")).toHaveText("Select");
  await expect(page).toHaveURL(new RegExp(`/en/${PAGE.select}$`));
});

test("a reload lands where the reader was", async ({ page }) => {
  // THE HALF THAT IS EASY TO BREAK AND IMPOSSIBLE TO NOTICE. Taking
  // `scrollRestoration` off `auto` — which Back and Forward need — takes the
  // browser's reload restoration with it, and the obvious replacement does not
  // work: measured in Chrome, a `replaceState` in `pagehide` succeeds in the
  // outgoing document and the reloaded one still reads the value from before
  // it. So the offset is written while the reader scrolls, and this is the test
  // that says the writing actually happens.
  await page.goto(url(PAGE.tables));

  // AND THE PAGE HAS TO BE THE SAME PAGE ON BOTH SIDES OF THE RELOAD. This
  // test used to open the folds and scroll 1200, which only worked because
  // every table on the page was drawing its caption as a wrapped column of
  // text a hundred pixels wide: the folded page happened to be tall enough.
  // The folds are opened by a test rather than by the document, so a reload
  // closes them — and an offset restored into a page that shrank under the
  // reader is clamped, which reads as "restoration is broken" and is not.
  const target = await page.evaluate(() =>
    Math.round(Math.min(1200, document.documentElement.scrollHeight - innerHeight) * 0.75),
  );
  // A floor. Nothing is proved by restoring an offset of nothing.
  expect(target).toBeGreaterThan(200);

  await page.mouse.wheel(0, target);
  await expect.poll(() => page.evaluate(() => history.state?.y)).toBeGreaterThan(0);
  const y = await page.evaluate(() => scrollY);
  expect(y).toBe(target);

  await page.reload();
  await expect.poll(() => page.evaluate(() => scrollY)).toBe(y);
});

test("a fragment on the page showing is the browser's, not the router's", async ({ page }) => {
  await page.goto(url(PAGE.tables));
  await showParts(page);
  const id = await page.locator(".nd-doc-section[id]").first().getAttribute("id");

  await page.evaluate((hash) => { location.hash = hash; }, id);
  await expect(page).toHaveURL(new RegExp(`#${id}$`));

  // Going back from a fragment is a `popstate` for a page already on the
  // screen. Refetching there would throw away every fold the reader opened to
  // get to it — so the assertion is that the folds are still open.
  const open = await page.locator("details[open]").count();
  await page.goBack();
  await expect(page).toHaveURL(new RegExp(`/en/${PAGE.tables}$`));
  expect(await page.locator("details[open]").count()).toBe(open);
});

test("a link carrying a fragment opens what it points into", async ({ page }) => {
  // The id is read off the destination itself, so this cannot drift from what
  // the build writes — and it has to be a section inside a FOLD, because a
  // target nothing is hiding proves nothing about opening anything.
  await page.goto(url(PAGE.tables));
  const id = await page.evaluate(() =>
    [...document.querySelectorAll(".nd-doc-section[id]")]
      .find((section) => section.closest("details"))?.id);
  expect(id).toBeTruthy();

  // BUILT RATHER THAN FOUND. No page on this site links into another page's
  // folds, and the case is real the moment somebody shares one. It is what
  // decides the order inside `go`: `revealTarget` runs in `adopt`, so the
  // history entry has to be written before the swap or it opens nothing.
  await page.goto(url(PAGE.button));
  await page.evaluate(({ slug, hash }) => {
    const link = document.createElement("a");
    link.href = `../${slug}#${hash}`;
    link.id = "nd-test-fragment";
    link.textContent = "go";
    document.querySelector("#nd-main").prepend(link);
  }, { slug: PAGE.tables, hash: id });

  await page.click("#nd-test-fragment");
  await expect(page).toHaveURL(new RegExp(`/en/${PAGE.tables}#${id}$`));
  await expect.poll(() => page.evaluate((hash) =>
    document.getElementById(hash)?.closest("details")?.open, id)).toBe(true);
  // And the page did not go to the top over the answer.
  expect(await page.evaluate(() => scrollY)).toBeGreaterThan(0);
});

// --- What a second render is allowed to leave behind ------------------------

test("ten navigations leave one date input, stepping once", async ({ page }) => {
  // `release.spec.js` asserts this by calling `enhanceAll` ten times, which is
  // the mechanism. This asserts it through the real path — the router — because
  // an enhancer that is released correctly and a router that forgets to release
  // it produce exactly the same bug.
  await page.goto(url(PAGE.dateInput));
  for (let pass = 0; pass < 10; pass += 1) {
    await row(page, PAGE.button).click();
    await expect(page.locator("h1")).toHaveText("Button");
    await row(page, PAGE.dateInput).click();
    await expect(page.locator("h1")).toHaveText("DateInput");
  }
  await showParts(page);

  const day = page.locator('[data-nd-date-input="visit"] [data-nd-part="day"] input');
  await day.click();
  await day.press("ArrowUp");
  await expect(day).toHaveValue("1");
});

test("the frames of a page left behind are no longer watched", async ({ page }) => {
  // The `IntersectionObserver` in `implementation.js` is a strong reference
  // from the browser, and `watching` is a strong `Set` on top of it. Neither
  // lets go when the markup does, so without `forget` a reader who visited ten
  // pages kept ten pages of detached frames alive — each still reachable by a
  // later page-wide press, which would mount React into a subtree nobody can
  // see.
  await page.goto(url(PAGE.button));
  const before = await page.evaluate(() => document.querySelectorAll("[data-nd-frame]").length);
  expect(before).toBeGreaterThan(0);

  await row(page, PAGE.select).click();
  await expect(page.locator("h1")).toHaveText("Select");

  // The detached frames are unreachable from the document, which is the only
  // statement a test can make about them without a heap snapshot — and the
  // observer is what would have kept them reachable.
  expect(await page.evaluate(() => [...document.querySelectorAll("[data-nd-frame]")]
    .every((frame) => frame.isConnected))).toBe(true);
});

test("a table reached by clicking still sorts", async ({ page }) => {
  // One component with a shipped runtime, driven after arriving by click. The
  // table is the loudest of them — it rewrites its own roles, arms a resize
  // observer and installs a comparator — so a page it was never enhanced on
  // would be a page of dead controls that look alive.
  await page.goto(url(PAGE.button));
  await row(page, PAGE.tables).click();
  await expect(page.locator("h1")).toHaveText("Table");
  await showParts(page);

  // The same table and the same control `table.spec.js` drives, so this asserts
  // the route rather than a second reading of what sorting means.
  const table = page.locator('[data-nd-table="invoices-sorted"]');
  await table.locator('[data-nd-table-sort="amount"]').click();
  await expect(table.locator('th[data-nd-column="amount"]')).toHaveAttribute(
    "aria-sort",
    "ascending",
  );
});

// --- The two surfaces a navigation has to close -----------------------------

test("following a search result closes the palette", async ({ page }) => {
  await page.goto(url(PAGE.button));
  await page.locator('[data-nd-search="docs"] [data-nd-search-open]').click();
  const dialog = page.locator("#nd-search-docs-dialog");
  await expect(dialog).toBeVisible();

  await page.locator("#nd-search-docs").fill("Select");
  const hit = dialog.locator("#nd-search-docs-results .nd-search-result:not([hidden]) a").first();
  await expect(hit).toBeVisible();
  await hit.click();

  // What closed this dialog until now was the page going away.
  await expect(dialog).toBeHidden();
  await expect(page.locator("h1")).toHaveText("Select");
});

test("following a link in the drawer closes the drawer", async ({ page }) => {
  await page.setViewportSize({ width: 360, height: 800 });
  await page.goto(url(PAGE.button));
  await page.locator('[data-nd-action="menu"]').click();
  await expect(page.locator("html")).toHaveAttribute("data-nd-nav", "open");

  await row(page, PAGE.select).click();
  await expect(page.locator("h1")).toHaveText("Select");
  await expect(page.locator("html")).not.toHaveAttribute("data-nd-nav", "open");
});

// --- Where the reader ends up -----------------------------------------------

test("focus lands on the page's own name", async ({ page }) => {
  await page.goto(url(PAGE.button));
  await row(page, PAGE.select).click();
  await expect(page.locator("h1")).toBeFocused();
});

// --- What the router is not --------------------------------------------------

test("with no script every link still navigates", async ({ browser }) => {
  const context = await browser.newContext({ javaScriptEnabled: false });
  const page = await context.newPage();
  await page.goto(url(PAGE.button));
  await row(page, PAGE.select).click();
  await expect(page).toHaveURL(new RegExp(`/en/${PAGE.select}$`));
  await expect(page.locator("h1")).toHaveText("Select");
  await context.close();
});

test("a reader whose language points elsewhere keeps the full load", async ({ browser }) => {
  // `pt-BR` asked for, `en` served, and no referrer — which is a shared link,
  // and the one case the inline resolver leaves where it landed. Its rule is
  // about how a document was ENTERED, so it has no answer for a navigation that
  // never happens: the router reads what that script resolved and steps aside.
  const context = await browser.newContext({ locale: "pt-BR" });
  const page = await context.newPage();
  await page.goto(url(PAGE.button));
  await expect(page.locator("html")).toHaveAttribute("data-noctua-locale-wanted", "pt-BR");

  // A full load, so the inline resolver runs again — and this time the referrer
  // says the reader followed a link inside the site, which is exactly when it
  // is allowed to move them.
  await row(page, PAGE.select).click();
  await expect(page).toHaveURL(new RegExp(`/pt-BR/${PAGE.select}$`));
  await context.close();
});

test("a reader whose language is the one served is routed", async ({ browser }) => {
  const context = await browser.newContext({ locale: "en-US" });
  const page = await context.newPage();
  await page.goto(url(PAGE.button));
  await expect(page.locator("html")).toHaveAttribute("data-noctua-locale-wanted", "en");

  const before = await page.$("#nd-nav");
  await row(page, PAGE.select).click();
  await expect(page.locator("h1")).toHaveText("Select");
  expect(await before.evaluate((was, is) => was === is, await page.$("#nd-nav"))).toBe(true);
  await context.close();
});

test("a link to another locale is the browser's", async ({ page }) => {
  await page.goto(url(PAGE.button));

  // The language menu crosses locales, and a document in another language is a
  // different `lang`, a different `dir` and a different word in every control
  // in the header — so "reload only the content" has nothing left to mean.
  await page.locator(".nd-locale > summary").click();
  await page.locator('[data-nd-locale="pt-BR"]').click();
  await page.waitForURL(new RegExp(`/pt-BR/${PAGE.button}$`));
  await expect(page.locator("html")).toHaveAttribute("lang", "pt-BR");
  await expect(page.locator("#nd-nav")).toBeVisible();
});

test("the pseudo-locale can be browsed, which it could not be before", async ({ page }) => {
  // `/qps/` is generated to be INSPECTED rather than read, and no browser asks
  // for it — so the inline resolver always wanted something else, and every link
  // out of it bounced to `/en/`. The one locale that exists to show what happens
  // when a string is 40% longer could only be opened one page at a time, by URL.
  //
  // A locale nobody is offered has nothing for that rule to resolve. It is
  // skipped where `here` is not in `data-noctua-locales`, so `qps` navigates
  // like any other locale — and because the router installs exactly where the
  // resolver stamps agreement, it is on the routed path too.
  await page.goto(url(PAGE.button, "qps"));
  await expect(page.locator("html")).toHaveAttribute("data-noctua-locale-wanted", "qps");

  const before = await page.$("#nd-nav");
  await row(page, PAGE.select).click();
  await expect(page).toHaveURL(new RegExp(`/qps/${PAGE.select}$`));
  expect(await before.evaluate((was, is) => was === is, await page.$("#nd-nav"))).toBe(true);
  // Every string on the page is still bracketed, which is the whole reason to
  // be here: a run that arrived unbracketed came from the script rather than
  // from the catalogue.
  await expect(page.locator(".nd-lead")).toContainText("⟦");
});

test("a reader IS still moved out of a locale they were offered", async ({ browser }) => {
  // The other half of the rule above, and the one that would silently go with
  // it: skipping the resolution for `qps` must not skip it for `pt-BR`, which
  // is offered and is exactly where a reader with an English browser should be
  // moved on a followed link.
  const context = await browser.newContext({ locale: "en-US" });
  const page = await context.newPage();
  await page.goto(url(PAGE.button, "pt-BR"));
  await expect(page.locator("html")).toHaveAttribute("data-noctua-locale-wanted", "en");
  await row(page, PAGE.select).click();
  await expect(page).toHaveURL(new RegExp(`/en/${PAGE.select}$`));
  await context.close();
});

// --- The sweep, which is what a hand pass would have looked at ---------------
//
// FOUR WIDTHS, THREE LOCALES, TWO MODES AND SIX PAGES, each arrived at by
// clicking. The browser's own language is matched to the site's on purpose: with
// an `en-US` browser on `/pt-BR/` the inline resolver moves the reader and the
// router never installs, so a sweep that did not match would be measuring the
// redirect and reporting it as Portuguese.
//
// `qps` IS THE ROW THAT MATTERS MOST AND IT COULD NOT EXIST UNTIL NOW. Every
// string in it is a third longer than the English it was generated from, which
// is what a real translation does and what English hides — and the pseudo-locale
// could not be browsed at all while the resolver bounced every link out of it.
// No browser locale asks for `qps`, so it is matched to nothing: the resolver
// has nothing to resolve for a locale nobody is offered, which is exactly why
// this row now works.

const WIDTHS = [320, 768, 1024, 1440];
const SITES = [["en", "en-US"], ["pt-BR", "pt-BR"], ["qps", "en-US"]];
const WALK = [PAGE.button, PAGE.tables, PAGE.colorPicker, PAGE.lineage, PAGE.icons, PAGE.intro];

for (const [locale, asked] of SITES) {
  for (const width of WIDTHS) {
    test(`${locale} at ${width}px: six pages, two modes, one sidebar`, async ({ browser }) => {
      const context = await browser.newContext({
        locale: asked,
        viewport: { width, height: 900 },
      });
      const page = await context.newPage();
      const errors = [];
      page.on("pageerror", (error) => errors.push(`pageerror: ${error}`));
      page.on("console", (message) => {
        if (message.type() === "error") errors.push(`console: ${message.text()}`);
      });

      await page.goto(url(PAGE.intro, locale));
      await expect(page.locator("html")).toHaveAttribute("data-noctua-locale-wanted", locale);

      for (const mode of ["light", "dark"]) {
        await page.evaluate((it) => document.documentElement.setAttribute("data-theme", it), mode);
        for (const slug of WALK) {
          const where = `${locale} ${width} ${mode} ${slug}`;
          if (width < 1024) await page.locator('[data-nd-action="menu"]').click();
          const nav = row(page, slug);
          // Brought into view first, or the offset below measures Playwright's
          // own scroll rather than the router's leaving it alone.
          await nav.scrollIntoViewIfNeeded();
          const was = await page.$("#nd-nav");
          const offset = await page.evaluate(() => document.querySelector("#nd-nav").scrollTop);
          await nav.click();
          await expect(page.locator("h1")).toBeVisible();

          expect(await was.evaluate((a, b) => a === b, await page.$("#nd-nav")),
            `${where}: the sidebar was rebuilt`).toBe(true);
          if (width >= 1024) {
            expect(await page.evaluate(() => document.querySelector("#nd-nav").scrollTop),
              `${where}: the sidebar moved`).toBe(offset);
          } else {
            // A drawer left open over the page a reader has just asked for.
            expect(await page.evaluate(() =>
              document.documentElement.getAttribute("data-nd-nav")),
            `${where}: the drawer stayed open`).toBeNull();
          }

          // No page on this site scrolls sideways, whatever it arrived by.
          const over = await page.evaluate(() =>
            document.documentElement.scrollWidth - document.documentElement.clientWidth);
          expect(over, `${where}: scrolls sideways by ${over}px`).toBeLessThanOrEqual(0);
        }
      }
      expect(errors, `${locale} at ${width}px`).toEqual([]);
      await context.close();
    });
  }
}

test("the theme menu still themes the page after a navigation", async ({ page }) => {
  await page.goto(url(PAGE.button));
  await page.locator(".nd-settings > summary").click();
  await expect(page.locator(".nd-settings .nd-panel")).toBeVisible();

  await row(page, PAGE.select).click();
  await expect(page.locator("h1")).toHaveText("Select");
  // CLOSED, exactly as a full load leaves it: a click on a link in the
  // navigation is a click outside the panel, and the handler that closes it on
  // one is older than the router. Its open state is deliberately never stored.
  await expect(page.locator(".nd-settings .nd-panel")).toBeHidden();

  await page.locator(".nd-settings > summary").click();
  await page.locator('.nd-settings [data-nd-group="mode"] [data-nd-value="dark"]').click();
  await expect(page.locator("html")).toHaveAttribute("data-theme", "dark");
});

test("a frame arrived at can still theme its own stage", async ({ page }) => {
  // The registry `resolveScopes` walks is a module-level list, and it used to be
  // filled once at load. A stage that arrived in a second render was in nobody's
  // list: it resolved against nothing, and its own controls reported the page.
  await page.goto(url(PAGE.intro));
  await row(page, PAGE.button).click();
  await expect(page.locator("h1")).toHaveText("Button");

  await page.locator('[data-nd-action="frame-mode"]').first().click();
  await expect(page.locator("[data-nd-frame-stage]").first())
    .toHaveAttribute("data-theme", /light|dark/);
});
