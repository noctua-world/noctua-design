# docs-site — what is real, and what is a placeholder

This file exists so nobody has to guess. Read it before adding a page.

## What is a placeholder

**`/intro/`.** Its body is the words *Lorem ipsum*, through the catalogue like
every other string. It is the front door and the first sidebar entry, and it is
waiting for somebody to write an introduction.

That is the whole list. Everything else below is real.

## What is real

**The shell.** Header, sidebar, mobile drawer, the partial scrim, the settings
panel, the language menu, the effect layers, search. All of it is measured
rather than asserted — see "Verified" below, including that each frosted surface
actually *samples* rather than merely declaring `backdrop-filter`.

**The theme runtime.** All thirty-nine palettes, three modes, two typefaces,
three glass levels, four corner presets, four background patterns, the aurora
and the grain. Every one of them is generated from `spec/theme.toml`; nothing on
this site writes a setting name down.

**Every component has worked examples, and they are all in one frame.**
`docs-site/src/examples.rs` carries a table per component — the real component
rendered, with the call that produced it as its caption — and
`every_component_has_examples` fails the build for a component that has none.
The derived variant grid (every value of every enum prop) sits in the same
frame, because a variant *is* an example and a second layout for it would be a
second set of controls.

The frame is the abstraction `NOCTUA.md §9.10` calls for, it hosts **every**
demonstration on the site — the typography specimens and the icon gallery
included — and it documents itself at `/example-frame/`.

**Its four controls.** The viewport control is a `Segmented` of the system's
own breakpoints, right-aligned in a `ToolbarGroup { align: End }`, and the
number beside it is the stage *measuring itself* rather than the control
repeating its own promise. The appearance toggle flips the mode inside the
frame only, by stamping a scope on the stage — the examples obey and the page
does not move. The implementation switch appears only where it can be honoured.
The reset puts all three back, including the source shown under an example.

**Every part of it is a component.** The bar is a `Toolbar`, so it is one tab
stop and the arrows walk it; the controls are `Segmented` and `Button`. It was
assembled by hand until Stage 6, and the cost of that is in the bug list below.

**Three locales, and nothing untranslated.** English, Portuguese, and a
generated pseudo-locale at `/qps/` that brackets and lengthens every string.
`every_sentence_on_every_page_is_translated` renders every page in `qps` and
asserts every text run is bracketed, skipping only `<code>`, `<pre>`, `<kbd>`,
`<time>`, `<cite>` and anything carrying `.nd-identifier` — which is the class a
page uses to say *this is a name, not a sentence*. Open `/qps/` at 320 px as
well: anything that overflows only fitted because English is short.

**Both packages, mounted side by side.** `ThemeScope`'s frame `import()`s a
React bundle that is **not requested on a normal page view**, mounts the
generated React component in place of the Dioxus one, and stamps attributes
byte-identical to what the server rendered. Prop-surface parity *demonstrated*
rather than asserted. The implementation switch appears **only** on a frame
that can honour it — one that offers it and cannot is a control that lies.

**Search covers the system's own names, and it is a palette now.** Tokens,
props, every component, every page and all thirty-nine palettes, each pointing
at the page that explains it, built from the generated crates so an upstream
addition is searchable without an edit here. **A name with no page is not
listed**: a hit that lands somewhere with no answer on it costs a reader more
than no hit at all. The pages are on the list because a palette that cannot
answer *where is the typography page* is only half a palette.

**The index is a file, and the sidebar is a trigger.** It used to be three
hundred and thirteen hidden `<a>` elements rendered into every document of every
locale, filtered by toggling `hidden` on three hundred and forty-seven elements
per keystroke, in a two-hundred-and-forty-pixel column that could not show a
ranked result — its name, the letters that matched, and the page it leads to.
Filtering the navigation also removes the rows a reader is navigating *by*. So
the sidebar opens a ⌘K palette over `Search`'s `dialog` presentation, reading
`dist/assets/search-<locale>.json` on the first search and caching it for the
rest of the visit. `en/intro/index.html` fell from 75 426 bytes to 42 868 raw
and from 9 998 to 7 740 gzipped. It is also the honest half of `Search`'s
contract — an index at a URL is the form that stops working with the script
blocked — so the trigger ships hidden until the script reveals it, into a slot
the stylesheet has already reserved.

## How it is built

```sh
cargo xtask docs      # writes docs-site/dist/
pnpm --filter @noctua-world/docs-site run build:example   # rebuilds the React bundle
```

`cargo xtask docs` is an ordinary Rust binary. No `dx`, no wasm toolchain, no
JavaScript runtime — it renders `rsx!` to a string with `dioxus-ssr`. The output
is static files any server can serve, and `docs-site/dist/` is **not** committed:
several megabytes of copied palette stylesheets, all reproducible from sources
that are.

The React bundle is **optional**. `cargo xtask docs` notes its absence and
carries on, because the Rust half of this repository owes nothing to Node
(`NOCTUA.md §9.11`) and a site that would not build without `pnpm install` would
break that. Without it the implementation switch reports a failed load and the
server-rendered Dioxus example keeps working.

The page is correct before any script runs. `assets/shell.js` adds the settings
to a complete page; it never constructs one.

## Verified, in a real browser

Run against the built site by `cargo xtask docs --verify`. Not a list of things
somebody once checked: a suite that fails the build.

```sh
cargo xtask docs --verify     # build the site, then run the suite
pnpm run test:browser         # the suite alone
```

It uses the system's Chrome (`NOCTUA_CHROME` overrides the path) rather than
Playwright's download, which cannot find its libraries on a Nix system and exits
before Playwright can say why.

Every color in the suite is read from **screenshot pixels** decoded in a canvas,
never from `getComputedStyle` — the same rule the `color_read_from_dom` gate
imposes on the shell. The places that read a serialised color do so only to
paint it, and carry an `allow-literal` saying so.

What it covers:

| | |
|---|---|
| A scope's `--nc-color-surface` equals **its own** `--nc-neutral-bg-app`, not the page's | ✓ |
| A nested scope keeps its parent's palette when only its mode changes | ✓ |
| Drawer and scrim meet with **zero-area intersection** | ✓ |
| `elementFromPoint` inside the drawer never returns the scrim | ✓ |
| The drawer's `border-radius` is `0px`, ignoring the reader's corner preset | ✓ |
| Blackout paints pure black and leaves `--nc-color-surface` byte-identical | ✓ |
| The header fits on one line at 320 px | ✓ |
| No `box-shadow` on any element in the shell | ✓ |
| **Every header control is one box on one centre line**, whatever element it is | ✓ |
| **All thirteen accent swatches paint thirteen distinct colors** | ✓ |
| **One frame per example, and its width control resizes only that example** | ✓ |
| Every `Lineage` scene at 320/768/1024/1440 × en/pt-BR clears nine geometric rules | ✓ |
| The Rust and JS layout engines produce identical scenes for every blessed case | ✓ |
| A glance at a scene changes nothing; a press changes only the camera | ✓ |
| **The settings panel closes on a click outside it** | ✓ |
| **The default palette applies with the script blocked** | ✓ |
| **The language menu offers three answers, and remembers the one chosen** | ✓ |
| pt-BR orders the sidebar in Portuguese, not in English | ✓ |
| Every glass surface has **no filtered ancestor**, and a frosted panel samples | ✓ |
| All **fourteen scripts** render with a loaded face, none as tofu | ✓ |
| A Latin page fetches **none** of the eleven script faces | ✓ |
| A CJK page fetches a handful of slices, not a monolithic family | ✓ |
| The React bundle: **0 requests** on a page view, **1** on the first toggle | ✓ |
| React's `ThemeScope` stamps the same attributes Dioxus server-rendered | ✓ |
| Each border sits one step from **its own** surface, measured in canvas pixels | ✓ |
| 4 patterns × 2 modes × blackout on/off all clear **4.5:1** against real glyphs | ✓ |
| No layout shift across mode, glass, radius, pattern, aurora and blackout | ✓ |
| A stored palette links with `blocking="render"`, so nothing flashes | ✓ |
| Reduced motion leaves `getAnimations()` empty even with every effect on | ✓ |
| With JavaScript off the page is whole and follows the OS dark preference | ✓ |
| Widening past `lg` releases drawer, scrim and scroll lock with no script | ✓ |
| Switching back from React **unmounts** it rather than hiding it | ✓ |

**The count is deliberately not written down.** It was, and it was wrong within
a stage. `pnpm run test:browser` prints it.

## Bugs this site found, and what each cost

All of them were silent, which is why they are written down.

**A script hook that no element carried.** `shell.js` bound to
`[data-nd-settings]`, which nothing emitted: the attribute was hand-written on a
hand-built `<details>`, and when the panel became the generated `Disclosure` it
went with it. `querySelector` returned `null`, `?.` swallowed it, and three
things stopped at once — all thirteen accent swatches painted the page's accent,
their stylesheets were never fetched, and the panel would not close. **A script
hook is a prop now**: `Disclosure.name`, like `Choice.name` and the search
field's own `name` already were. `Search.name` went further and made it
**required**, because the optional version shipped its own version of this
defect: `id = format!("nd-search-{}", name.unwrap_or_default())` gave four
unnamed fields on one page the id `nd-search-` and four `<label for>` pointing
at whichever the browser resolved first.

**A `<select>` that submitted the same value twice.** `creatable` suppressed
its *add* offer by comparing `option.textContent`, and `create()` wrote the term
into `value` **and** `textContent` — so wherever the two differed, and the
shipped example `SelectOption { value: "ochre-balanced", label: "Ochre" }` is
exactly that, typing the value found no label to match, offered to create, and
appended a second option carrying a value the control already had. One
single-valued select, `palette=ochre-balanced` twice. Nothing failed, because
nothing in this repository had ever read a `FormData`: every test asserted the
DOM. Four now read the form.

**A `required` control the script had hidden made the form unsubmittable.** The
browser has to focus an invalid control to anchor its bubble; `native.hidden` is
`display: none`, so it cannot, and it gives up — Chrome logs *"An invalid form
control with name=… is not focusable"* and the submit silently does nothing,
Firefox blocks it with nothing said at all. `Select { multiple, required }` had
shipped in the examples the whole time and was never seen, because **no example
on this site was inside a `<form>`**. There is one now, on `Select`'s own page,
and the browser's own `validationMessage` goes into the component's own error
slot with focus on the control the reader can see.

**A reset neither repainted nor cleaned up.** `form.reset()` fires one event on
the form and none on the controls inside it, so the enhanced control went on
showing an answer the select no longer held; and a script-created option's
`defaultSelected` is `false`, so reset deselected it and **left it in the list
for ever** — one more phantom suggestion on every reset. Both are `Select`'s,
and `TagPicker` would have inherited both.

**`/` opened the search palette out from under a typeahead.** `isTyping` knew
`input`, `textarea`, `select` and `contenteditable`; a focused `role="treeitem"`
is none of those, and the hotkey listener is registered before the roving one —
so a reader typing a node's name to jump to it would have been thrown into the
palette. A component that reads printable keys says so with `data-nd-typeahead`.

**A scrollable element carried a radius, in the one state nobody looked at.**
`TagPicker`'s native `<select multiple>` is a four-row scrolling listbox and is
`hidden` the moment the script enhances the control — so every sweep asking that
question saw nothing, because every sweep runs with a script. Half of what a
no-script page shows is a control some enhanced page hides. `material.spec.js`
asks it without one now.

**A tree with no indentation.** Every label on every level sat at exactly the
same x. A parent's icon pushed its own name right by one step, and one step is
precisely what the nesting had indented the child by — so the two cancelled and
the hierarchy was invisible. It did not look wrong; it looked like a flat list
that happened to have chevrons. A row with no drawing reserves the slot now, in
a tree that has any.

**A row of avatars that widened the page.** Five faces at 320 pixels overhung
their stage by eighteen and pushed the whole document sideways, which is a
defect on every other component on it. They scroll instead — wrapping would
stack the second row the wrong way round against the first.

**A guard that matched `rem` inside `remove_label`.** `index.rs` asserted that
no entry in the palette looks like a measurement by testing `name.contains("rem")`,
and the first prop in the repository to contain those three letters failed a
test about the spacing scale. It asks for a measurement now: digits, then a
unit, then the end of a word. The same class of defect as
`the_examples_reach_every_prop` finding `expanded:` inside `icon_expanded:` and
reading the attribute above the wrong one.

**`data-nd-` twice.** The variant grid wrote `data-nd-{attribute}` where the
attribute already *was* `data-nd-size`, so every page shipped
`data-nd-data-nd-size`. Fourteen component pages rendered every variant of every
enum prop identically, and each looked like a component whose variants did
nothing rather than like a page with a typo in it.

**Three props that were accepted, documented, and ignored.** `Heading.level`
rendered `<h2>` whatever it said; `List.kind` rendered `<ul>` while its own
documentation said it changed the element; `CodeBlock.numbered` stamped an
attribute nothing could read. A prop now declares the `elements` it switches
between, in the spec, which reaches both implementations and this site at once —
and `numbered` was **deleted**, because no stylesheet can number the lines of a
text node and a prop that cannot work is worse than a missing one. It came back
with the component that could implement it: `CodeBlock` takes the code as a text
prop now, splits it into one element per line, and numbers them with a counter —
which the deletion note predicted, in the sentence saying that re-adding it
"needs a `code` text prop first, and that is a different component".

**A bool default that only one language honoured.** The generator emitted
`#[props(default)]` for every bool, which is `false` whatever the spec declares,
while React read the same spec and wrote `marker ?? true`. `Disclosure`'s
chevron was never drawn and every code block shipped `data-nd-copy="false"` —
drift produced by the generator, which is why neither implementation could be
reviewed into catching it.

**English on every Portuguese component page.** A component's guidance, every
prop's documentation, its keyboard contract and its accessibility notes live in
`spec/components/<name>.toml`, where they also become the doc comment — and the
site printed those same strings on the page in every locale. They are now
*derived* into the catalogue by `component::derived_messages`, so the
cross-locale check demands a Portuguese answer for each one and English is still
written exactly once.

**`hidden` hid nothing.** The browser's `[hidden] { display: none }` is a
*user-agent* rule, so any author rule outranks it. Every `hidden` in the shell
was inert. One `[hidden] { display: none !important }` fixes it, and it is the
only `!important` in the stylesheet.

**Every border inside a themed component was the page's color.** The shell
declared `--nd-edge: <hairline> solid var(--nc-color-border)` at `:root` — a
`var()` resolves where it is *declared*. The `frozen_color_indirection` gate now
catches it; the shell names its border color at each point of use, and the
repetition is the point.

**Glass was never applied on a first visit.** Every glass block was a bare
attribute selector, so until a reader changed the setting nothing matched,
`--noctua-glass-blur` was undefined, and `backdrop-filter: blur(var(…))` was
invalid at computed-value time — which is `none`. The default level is now also
declared at `:where(:root)`, and a test asserts it.

**A scrollable element carrying a radius, again.** `.nd-codeblock` had
`overflow-x: auto` and `border-radius` on one rule, so a classic scrollbar
painted square through the rounded corner the moment corners were set to
`circular`. It survived because no page the suite looked at had a code block
wide enough to scroll. The radius is the `<pre>`; the scrolling is the `<code>`
inside it.

**A test that asserted the bug.** `a_list_changes_its_element_with_its_kind`
checked for `data-nd-kind="ordered"` — which the component did emit, on a
`<ul>`. It was named after the right property and checked the attribute the
defect left behind.

**An exemption keyed on a translated word.** The tofu check exempted CJK with a
regex over the script's English name, so it evaporated the moment that name
became a message and the suite called a correctly rendered Chinese line tofu.
Whether a script is ideographic is now declared in the table that describes it.

**Three pages deleted, and one kept out of the wreckage.** `/effects`,
`/colors-and-themes` and `/spacing` are gone. Deleting the second would have
taken the only rendered `ThemeScope` on the site with it — and with that, a
dozen browser specs covering the invariant the whole theme layer rests on. What
survived is `/theme-scope/`: it is a component, the sidebar is a list of
components, and that page was always its documentation wearing another name.

**A test that could pass on an empty reading.** The pseudo-locale test scans the
rendered page for unbracketed text, and its scanner lost itself in the first
inline `<script>`: `i < asked.length` reads as an opening tag and the search for
its `>` runs past `</script>`. It examined one run per page and passed. The
floor — *this must examine more than five hundred runs* — is what caught it, and
is the reason the assertion is still worth anything.

**A source gate that had stopped checking.** `block_comment_state` treated any
`/*` as opening a block comment, including one inside a `//` line comment. One
comment in this repository's own sources said `spec/components/*.toml`, and from
that line on **every gate skipped the rest of the file** — silently, for a whole
stage. It surfaced only because deleting that comment made two real findings
appear out of nowhere. A `//` now ends the line for the tracker, and a string is
not scanned for comment markers either.

**A test that asserted a consequence rather than the property.** The sidebar's
ordering test compared the English order against the Portuguese one and required
them to differ — true only while some page happened to sort differently in the
two. Deleting two foundations left a page set that sorts identically in both,
and the test failed while the sort was perfectly correct. It now asserts what
the sort promises: within a group, the collated titles never go backwards, in
every locale.

17. **A rule that said a scope must paint its own background, above a rule that
    painted none.** The comment had said "a scope MUST paint its own
    background — a light island inside a dark page with no background of its
    own is unreadable" since the scope layer was written; the declaration was
    never there. So the demonstration page's `mode: Light` scope drew light-mode
    ink straight onto the dark page and was very nearly invisible, while the
    nested `mode: Dark` one looked fine and hid the fact that anything was wrong.
    One declaration, and the page's whole subject became visible.

18. **Three classes on real markup with no CSS at all.** `.nd-frame-widths`,
    `.nd-frame-impls` and the component's own `.nd-choice-group` styled nothing,
    so the frame's controls were unstyled boxes in a flex row. Beside them,
    `.nd-frame-controls` — which is where `margin-inline-start: auto` lived —
    was carried by no markup at all, so "the controls are on the right" had been
    written down and never once been true. Alignment is `ToolbarGroup { align }`
    now, in the contract, where a test can reach it.

19. **Twenty-three buttons with no accessible name.** The derived variant grid
    renders the component's *element* with the `specimen` icon inside it, so for
    a component that `needs_name` it shipped a nameless control per variant —
    six of them on `IconButton`'s page before this change, twenty-three on
    `Button`'s. The grid names each specimen after the caption beside it now,
    which is the one string that cannot drift from what the reader sees. The
    browser suite's all-pages loop had covered `heading/` and no page whose
    specimen is a control.

20. **A reset that restored two things out of three.** Switching an example to
    React and pressing Reset put the render and the radio back and left the
    `<figcaption>` showing React source under a Dioxus render — the one place
    `[data-nd-impl-source]` was never touched. Beside it, `requests.set(el, {})`
    meant "inherit the page" rather than "go back to what you were built with",
    so the same press threw away the demonstration's authored
    `azure-vivid / light / full / round`.

21. **`const defaults = {…}` in sixteen shipped React components, read by
    none.** The generator emitted it for every component that had a default and
    nothing ever referenced it: the generic path writes `x ?? "value"` from the
    same spec field and a custom body writes its own. Dead code, and worse than
    dead — a second place one default was written, with nothing asserting the
    two agreed.

22. **Four cycling controls that never showed what they were cycling.** The
    header's mode control is three radios *because* a cycling button has to show
    its current state to be usable, and that argument is written out at length
    in `shell.rs`. Two hundred lines below it, the scoped-theme page carried
    four buttons with one static glyph each. They are `Segmented` groups now —
    and making them show their state exposed the next bug, which is that they
    had been reporting the page's values rather than the scope's.

23. **A compound button that could not wrap, and a heading that could not
    break.** `/qps/` at 320px: the compound example was 410px of text in a 199px
    control, because the detail line inherited `white-space: nowrap` from the
    button; and `ExampleFrame` — twelve characters, one word, 3rem — made the
    whole document scroll sideways. Both are now caught for every page, locale
    and width by `tests/browser/responsive.spec.js`, which is the sentence
    "open `/qps/` at 320 px as well" turned into a check that runs.

24. **A quarter of the stylesheet styled nothing.** Forty-one `.nd-*` classes
    with no markup anywhere — mostly the wreckage of `/effects`,
    `/colors-and-themes` and `/spacing`, which were deleted without their CSS.
    Among them `.nd-specimen`, whose separator rule never applied because the
    typography families wore `.nd-variant-item`, borrowed from the frame's
    variant grid, instead.

25. **A scope that could be put on and not taken off.** The frame's appearance
    toggle removed two attributes where `scopeAttributes` stamps six. The four
    left behind were not inert: `dist/scope.css` matches
    `[data-palette]:not(:root)`, so the stage went on re-resolving the entire
    semantic contract at itself — and with `data-theme` gone it resolved against
    the *system* preference rather than the page. On a dark machine showing a
    light page the stage was stuck dark for ever, and every further press did
    nothing. `resolveScopes` had the same hole from the other side: it treated
    the resolver's `null` — *this scope changes nothing* — as "leave it alone"
    rather than as "take it all off", so the reset leaked the same four.

    Both are one loop now, over `scopeAttributeNames`, which the theme model
    generates beside `scopeAttributes` so the stamping and the un-stamping
    cannot disagree about what a scope is made of.

26. **A scope that changed every colour except the one behind them.** The stage
    resolved the contract correctly and painted no background, so the examples'
    ink and borders flipped while the page's surface showed through underneath.
    This is the third time this exact defect has been found here — `.nd-scope`
    carried a comment demanding a background it did not paint — and it is worth
    the third entry because the failure is always the same shape: re-resolving
    the contract changes what the colours *mean* and paints nothing.

    A scoped stage paints `var(--nc-color-surface)` at the point of use, and
    only while it is scoped: an opaque stage at rest would hide the page's
    background pattern inside every frame on the site.

    **The test that missed it read the variable.** It asserted that
    `--nc-color-surface` at the stage differed from the page's — which was true
    the whole time, because resolution was never the broken half. It reads the
    painted pixel now, and presses the toggle three times, because one press
    proved nothing about the second.

27. **A frame that could not be isolated from a frame around it.** The frame's
    own page demonstrates one by putting one inside another, and that is where
    every "there is only ever one frame" assumption came out at once.

    The appearance toggle inverted `<html>`'s mode — the same thing as the mode
    in force only while no frame is nested. Inside an inverted outer frame the
    inner one asked for the opposite of the *page*, which was the mode it
    already had, so the resolver correctly called it a no-op and stamped
    nothing: the inner frame could not invert itself at all. It reads
    `settingAt(stage, "mode")` now — what is in force *where the element sits*,
    walking every scope above it.

    And every handler addressed `frame.querySelectorAll(…)`, so the outer frame
    owned ten width radios, two stages, two readouts and two resets. Pressing
    the outer control marked the inner control's matching stop while resizing
    only the outer stage, leaving the inner one claiming a width it was not.
    One helper — the nearest `[data-nd-frame]` decides — and a frame addresses
    its own parts. It is the same test `exampleScope` already made for the React
    mount, applied everywhere it was missing.

28. **Two ways to change one setting, and neither knew about the other.** The
    header carried a three-icon mode control; the panel behind the gear carried
    a mode row and nine more. A reader had two controls for one value, and the
    header spent a third of its 320px budget on the copy.

    There is one theme menu now, behind a palette rather than a gear, and the
    example frame opens the same component to theme its own stage. What decides where a value lands
    is where the control *sits*: inside a scope it themes that scope, inside a
    frame it themes that frame's stage, otherwise the document.

    The panel was rebuilt with it. It had been eleven `Segmented` groups in one
    flat scroll, every row the same weight, with a single `<hr>` that separated
    the scopable settings from the page-only ones and said so nowhere, and the
    reset at the bottom of all of it. It is sectioned now; each row is a name
    beside its control rather than a name above a full-width block; the three
    booleans are `Switch`es rather than radio groups of "Off" and "On"; and the
    reset is in the panel's header where it can be reached without scrolling
    past everything it undoes.

    **Three things the rebuild found.** The frame's thirteen accent swatches
    all painted the reader's own accent, because the sheets that define them are
    fetched when *the settings panel* opens and the frame's menu is not it. The
    toolbar's roving focus walked into the menu's thirty controls, because a
    `<details>` in a bar is one stop — its summary — and nothing said so. And
    the quick toggle reported itself pressed when a reader chose `system` in a
    frame on a page that was already following the system, because it asked
    "has a mode been requested" rather than "is this showing something other
    than what it inherits".

29. **I shipped that menu without opening it in a browser, and five defects
    went with it.** Every one was a measurement away. This is the entry for
    the measurements.

    **The thirteen accent swatches each overlapped the next by 2.93px.** The
    row asked for `repeat(auto-fill, minmax(var(--noctua-space-6), 1fr))` — a
    24px minimum — under an item that `[data-nd-hide-label]` gives a *fixed*
    `var(--noctua-space-8)`. Tracks came out 25.07px, every 32px chip overhung
    its own by 7px, and twelve borders doubled. The rule is gone rather than
    corrected: the group is `spread`, and thirteen chips of one width wrap into
    a grid on their own — a flex item cannot overhang a sibling.

    **The frame's menu was a band inside the frame, and a frame is not tall
    enough to hold it.** 457px of panel covering every visible example, and
    `overflow: clip` cut off three of its eight rows on a 387px frame and all
    eight on a 172px one. It was built inline on a guess I did not check —
    that a floating panel would need `position: fixed` and would be clipped
    anyway. The first half is true; the second is not. `overflow` clips only
    descendants whose containing block is inside it, and `.nd-frame` declares
    no `transform`, `filter` or `contain`. A probe appended to it paints and
    hit-tests at viewport `x=40` while the frame starts at `x=288`.

    **Two settings read as one, because the panel's own gaps were inverted.**
    Rows sat 12px apart while a row's name sat 16px from its own control — the
    parts of one setting further apart than two settings. `Corners` wrapped
    onto a second line 4px below its first and bled straight into `Glass`. The
    order is now 4px inside a control, 16px within a row, 24px between rows,
    24px + a rule + 24px between sections; and the panel went to 28rem, which
    stops `Corners` wrapping and buys all of that for no extra height.

    Two more the sweep across 4 widths × 3 locales × 2 modes found, neither of
    which showed at the width I had been looking at:

    **A row's name ran across its control in the pseudo-locale.** The name
    column is a fixed `space-24`; `⟦Saatuuraatiioon⟧` is 138px with no space to
    wrap at. `qps` exists to find exactly this and it only finds it if
    something asks. `overflow-wrap: anywhere` breaks the word instead.

    **A panel cannot outrank its own ancestor's stacking context.** The bar is
    `sticky` with a `z-index`, so it *is* a stacking context and the panel's
    `layer-popover` resolves inside it — the whole thing paints at the bar's
    `layer-raised`. On the page where one frame sits inside another, the inner
    frame's bar is `raised` too and later in the document, so it covered the
    outer menu's first row. The bar carries the rank while its menu is open,
    because raising something inside a context it cannot leave does nothing.

    And one regression of my own, caught by looking: the band it replaced was
    opaque, and I made the floating version `.noctua-glass`, which is 78%
    surface. A fifth of a stage the reader had just turned dark read through a
    panel that was still light. It is opaque again, which is the trade
    `.nd-frame-bar` already documents two elements away.

    **What the suite asks now**, because none of it asked anything about where
    a control ended up: no control overlaps the one beside it; no name is wider
    than the box holding it, in any of the three locales; every row of a frame's
    open menu hit-tests to itself, at 768px in `pt-BR` where the nested frame
    is; and a group is further from its neighbour than from its own parts. Each
    was confirmed by restoring the defect and watching it fail.

30. **A frame is a boundary, and it was not one — in both directions.** On the
    one page where an `ExampleFrame` sits inside another, the inner frame drove
    its parent and wore its parent's appearance.

    **Outwards.** `themedBy` asked for the nearest *scope* and only then for the
    nearest *frame*. With the outer stage themed, that scope is nearer to the
    inner frame's toolbar than the inner frame is — so the nested example's own
    menu changed the frame around it. One query for both, and whichever is
    nearer decides, which is the rule as it was always stated: where the control
    sits.

    **Inwards, and this was the harder half.** A stage that has been themed
    carries attributes; one that has not carries none — and carrying none is
    exactly how a subtree inherits everything above it. The two are the same
    markup. Three things had to change and each was invisible on its own:

    * A frame's request now DEFAULTS TO THE PAGE on every scopable setting, so
      the resolver stamps it whenever that differs from what it would otherwise
      inherit — and returns null, costing nothing, when it does not.
    * Every frame is registered with the resolver at load, themed or not. One it
      never visits is one it cannot seal, and an unthemed stage was never
      visited.
    * `system` is stamped as the ABSENCE of `data-theme`, which is also how a
      subtree says "whatever is above me". A sealed frame states what the machine
      is showing instead of omitting it.

    **The frame AND its stage**, because they answer to different things. The
    stage shows what the reader chose for it; the bar, the border and the label
    answer to the page — "a control that changes appearance along with the thing
    it controls takes away the reader's anchor", which was written for a frame
    sitting on the page and quietly stopped being true for one sitting inside
    another's dark stage.

    **It is generic, which was the actual request.** The seal is *where
    inheritance starts*, not a list of settings to block, and the page's theme is
    built by looping `scopableSettings` — now exported from the generated model
    rather than hand-copied into the shell twice, which is how a setting added to
    `spec/theme.toml` was previously isolated by nobody. The test walks the same
    generated list, so a new setting is covered the day it exists.

    Also here: 32px of nothing above the panel's title. `.nd-heading` carries
    `margin-block-start` — the gap between a heading and the text ABOVE it,
    which is prose rhythm and meaningless where there is nothing above — and it
    beat the panel's `margin: 0` on source order alone, one class each.

31. **The glass was untestable, so nobody could tell it was working.** Six
    tokens tune the material — blur, opacity, saturate, brightness, edge, sheen
    — and the only surfaces that used it were the header, the drawer and the
    settings panel, all of which sit over a flat page. A frosted rectangle over
    a flat colour is a tint. There was nothing to judge.

    An `ExampleFrame` now has a control that arranges the three things judging
    it needs at once: **colour with hard edges** behind the stage (twelve flat
    shapes, because blur is only visible where there is an edge to lose),
    **blank space and a scrollbar** so the examples can be dragged across them,
    and **the material on everything inside that declares it**.

    **Nine components declare it**, which is the other half: `Button`,
    `Segmented`, `SegmentedItem`, `Toolbar`, `Search`, `Disclosure`,
    `Switch`, `CodeBlock`, `Blockquote` — every one that paints a fill a reader
    could see through. Still ONE recipe: the generated `.noctua-glass` selector
    gained `[data-nd-glass="true"]` rather than a second `backdrop-filter`,
    which the source gate would have refused anyway.

    **Four things this took that were not obvious.**

    A component's fill outranks the recipe. `.nd-button[data-nd-variant="solid"]
    [data-nd-tone="danger"]:hover` is (0,4,0) against the recipe's (0,1,0), so
    each control restates the mix with ITS OWN colour, one attribute higher
    than the rule it replaces — a glass danger button is red frosted glass.
    Written at each point of use and never held in a custom property, because
    `--x: var(--nc-color-danger)` freezes to the palette of whatever declares
    it, which `frozen_color_indirection` fails the build on.

    Eight of the nine have hand-written bodies, so the generic emitter does not
    write their attributes. Each needed the line, and `Button` needed it twice
    — it renders an `<a>` as well as a `<button>`. The Switch's went on the
    TRACK: the button's own box paints nothing, so frosting it would frost
    nothing, and the knob has to stay solid because its position is the state.

    The colour came out **on top of the text**. A positioned element paints
    above every static one in the same stacking context whatever the source
    order says, and the examples are static. `z-index: var(--noctua-layer-base)`
    on the field puts it back underneath.

    Leaving the test **un-frosted the examples that were authored frosted** —
    `glass: true` is the prop's own documentation, and setting everything to
    `false` on the way out quietly edited the example a reader came to look at.
    Each control remembers what it was.

    **Three more the tests passed straight through, found by opening the nested
    example and looking at it — the case where an `ExampleFrame` is the thing
    being dragged across the field.** Every one of them is a rule written for a
    frame sitting on a PAGE, and the rig is the one place a frame is not.

    A frame's bar is `position: sticky` at `--nd-header-h`, and sticky sticks to
    the nearest SCROLL CONTAINER — inside the rig that is the rig, not the page.
    So the nested frame's bar left its own frame and floated a third of the way
    down the field while the frame scrolled out from under it. It does not stick
    in there: the frame it belongs to is a few hundred pixels tall and entirely
    on screen, which is the case sticky was for and this is not.

    A stage paints nothing until something themes it, so the page's background
    pattern reads through every example on the site. Over a field of hard-edged
    colour that made the nested frame a hole: the shapes ran straight through
    its border and it stopped reading as a component at all. Inside the rig it
    takes the page's own surface — which is also the only way to see what
    frosting something *inside* it does.

    And the blank space was a fixed step, which is a guess about how tall the
    content is. At `space-64` a short frame's examples were pushed off the
    bottom; at `space-32` a frame holding ONE example came to 376px inside a
    384px stage and **had no scrollbar at all** — the one thing the rig exists
    to provide. Half the stage each side makes the blank alone exactly one stage
    tall, so any content overflows it whatever the frame holds, and the first
    example lands in the middle where a reader looks for it. The gutter is
    `stable`, because a rig that promises a scrollbar should not wait to be
    touched before showing one.

    The earlier version of this, before those: the blank space was `space-64` — 256px in a
    384px stage — so on a frame whose content is short, which is every frame on
    the ExampleFrame page, pressing the control gave a blank field with a
    toolbar at the end of it and the examples pushed off the bottom. A third of
    the stage instead of two thirds. The assertion that now guards it bounds the
    blank from BOTH sides: generous enough to drag an example clear of the
    colour, and never most of the stage.

    Also here: the "Where it comes from" section is gone from every component
    page. Four file paths, and the reader who wants them is reading the
    repository rather than the site.

32. **The glass on a control was wrong in the way its own contract says it must
    not be.** `spec/components/button.toml` documents the prop as *"the tone
    stays in the ink and the edge — a glass button is a pane, whatever it is
    for"*, and the CSS did the opposite: it tinted the pane with the tone and
    left the ink alone. Both symptoms follow from that one inversion.

    A **neutral outline button went grey**. `Cancel` has no fill at all —
    `background: none`, `--nc-color-fg` ink, a strong border — and the rule
    matched `[data-nd-variant][data-nd-tone="neutral"]` for EVERY variant and
    painted `--nc-color-subtle`, the solid grey a neutral button is *filled*
    with. Measured, the pane went from `253,253,253` to `171,170,169`.

    A **solid button stopped being readable**. Its ink is `--nc-color-on-accent`
    — white, solved against a *solid* accent fill — and a frosted fill lets a
    fifth of the backdrop through and lifts the pane under it.

    The pane is the surface now, for every control and every tone, and the tone
    moved to the border. The ink is `--nc-color-fg` rather than the tone's own
    colour, and that took a measurement to settle: `--nc-color-accent` as ink
    against a surface is **3.17:1**. That is a property of the palette, not of
    the glass — a tone's solid step is solved to be read *against*, not read
    *with*, and the contract has no "accent text on a surface" step. `outline`
    and `link` live with it on an opaque page; a pane that also lets the
    backdrop through cannot. Every enabled frosted control now measures 11.6:1
    or better, from a screenshot, over the colour field.

    **The rig never really scrolled, and it took the arithmetic to see it.** The
    blank was `padding` on the scroller — and a whole stage of blank each side
    is two stages of padding on a box one stage tall. With `box-sizing:
    border-box` a box cannot be shorter than its own padding, so it became 768px
    inside a 384px stage and the stage *clipped* it: most of the blank was
    unreachable and the scrollbar covered 90px of travel where there should have
    been 500. It looked plausible in a screenshot, which is why it survived two
    rounds of me looking at screenshots of it. The blank is two flex spacers
    now, which is content rather than padding, and the scroller is exactly as
    tall as the stage.

    Also: a frame inside the rig is **frosted**, frame and bar, rather than the
    opaque surface it got last round — a component being dragged across the
    field is the thing under test. Its own controls stay unfrosted; they belong
    to the inner frame, and frosting them inside a frosted pane is glass
    sampling glass. The colour field is **inset** from the frame's edges, more
    on the block axis, so an example crossing it goes blank → colour → blank.
    And there are **twenty-four shapes** instead of twelve, half of them small:
    a 192px block reads the same at `subtle` as at `full` because the blur
    radius is smaller than the shape and only its edge moves, while a 16px dot
    is the size the radius acts on.

    **And then the pane went too far the other way.** With every control frosted
    to the plain surface, `Save` and `Cancel` became the same object — a primary
    button that loses its colour has lost the one thing it was for. So the tint
    is back, for the two variants that were FILLED to begin with (`solid` and
    `subtle`) and for nothing else, which is the half that turned `Cancel` grey.

    What it is tinted with took a second measurement. The tone's `-bg` step is
    the obvious candidate — it is what `subtle` is filled with, and the system
    already pairs it with `--nc-color-fg` — and in this palette it computes to
    **0.975 lightness at zero chroma**: a tint nobody can see, which left `Save`
    8 units of blue away from `Cancel`. The pane is mixed from the tone's SOLID
    step against the surface instead, at a named `--nd-tint`, so it is visibly
    the tone in both modes and the page's ink still reads on it. Measured:
    `204,223,244` against `252,252,252`, at 10.1:1 and 13.4:1.

    Two more from looking at it wide: every shape was anchored to one edge of
    the field or the other, so past about 1400px they separated into two
    clusters with a blank corridor between them and a control parked in the
    middle had nothing behind it — six are anchored to the centre now. And the
    colour's side margins were as wide as its top and bottom ones, which is
    width the field could have used: the travel is vertical, so the inline
    gutter is a quarter of the block one.

    **And frosting the frame broke the menu inside it.** `backdrop-filter` on an
    element makes it the containing block for every `position: fixed`
    DESCENDANT — and the nested frame's own theme menu is one. Stamping the
    material on the frame and on its bar re-anchored that panel to the frame,
    which is `overflow: clip`, so it opened where nothing could see it.
    Measured: `offsetParent` went from `null` to the frame's toolbar and the
    panel's box from `y=72` to `y=1852`.

    The material goes on a LAYER now — `.nd-frame-glass`, a first child that
    covers the frame and carries the attribute, with the bar giving up its own
    surface so the frost reads through it. The header has had exactly this fix
    since it was built, and for exactly this reason: `.nd-header-glass` exists
    so the header carries no filter and its own panel can stay fixed. The test
    asserts the layer is frosted, the frame and the bar are **not**, and the
    nested frame's menu still opens on screen with the rig running.

    **`tests/browser/pixels.js`** is `luminance`, `contrast`, `extremes` and a
    new `pixelAt`, lifted out of `readability.spec.js` — which had them to
    itself while the only readability claim in the repository was about the
    background pattern under prose. `extremes` cannot answer "what colour is
    this surface": the lightest pixel in a frosted control is as often an
    antialiased edge as it is the pane. A named point can, and that is the
    assertion that catches the grey `Cancel`.

33. **The toolbar spent half its width naming units, and the rig did not own
    the setting it tests.** Four changes, and three of them are about a control
    telling the truth about itself.

    **The width stops stack.** `320px 30rem 48rem 64rem Largura total` was five
    labels that each spelled their own unit, and it took **58% of the bar**. The
    unit goes under the number now — `SegmentedItem` gained a `detail` prop with
    the same contract `Button.detail` already had, *part of the accessible name*
    — and the same five stops take **23%** at 1440px and 37% at 768px, measured.
    Every one is still one click away, which was the thing worth protecting: a
    menu would have been narrower still and is the wrong trade for a control a
    reader flips back and forth. The alternative to a prop was `children` with
    `hide_label`, which forces the icon-only box and is the wrong shape for a
    stack.

    **The glass test's icon** is Lucide `image` rather than `layers` — a picture
    in a frame, because putting one behind the examples is what the control
    does. `layers.svg` is gone with its entry; the asset check runs both ways.

    **The rig turns the glass on and holds it there.** A control that says "look
    at the glass" over a stage whose glass is `off` does nothing, and the reader
    could switch it off underneath the rig at any moment. Starting the test sets
    the material to the model's own default if it is off, remembers what the
    reader had, and puts it back on the way out. `Off` is marked
    `aria-disabled` — refused rather than removed, because `SegmentedItem`
    documents that exact difference: *"so a reader can find out it exists and is
    refused instead of meeting a gap where an option used to be"*. The one place
    a value is spelled in the script is asserted against the generated list, so
    a rename in the spec fails loudly rather than turning the guard off.

    **And it says why.** A note under the Glass row, rendered only in a frame's
    menu and shown only while that frame's rig runs — a control marked disabled
    with no reason given is a control that looks broken. Two `>` in the selector
    so a frame nested inside a testing frame does not inherit an explanation for
    somebody else's rig.

    Two things the measuring caught: the first version of the width assertion
    counted a second line that was *present* rather than *rendered*, and passed
    with `display: none` on it; and the share of the bar is only meaningful
    while the bar is one line — below 1024px it wraps and the group has a line
    to itself by design, so the sweep asserts it only where it means something.

34. **A correction first: I said this machine had no network, and it has one.**
    Every Lucide glyph should be fetched at the pinned tag, and I had drawn
    `image.svg` from memory. Checked all fourteen against `v0.265.0`: thirteen
    byte-identical, that one an older variant of the same idea. It is gone this
    round anyway — replaced by `flask-conical`, **fetched** — and the rule is in
    `AGENTS.md` now, where `spec/icons.toml` had already written it down.

    **The width control is one component.** Five stops that each spelled their
    own unit plus a separate readout took 58% of the toolbar; the unit is under
    the number, the last stop is `Máx`, and the measured width is that stop's
    own second line — the one stop whose real width is not already stated by its
    label. **24% at 1440px**, measured, and every stop still one click.
    `Segmented` gained a `selection` prop for it: `quiet` marks the value in
    force with the surface and a stronger edge rather than with the accent,
    because a group in a bar beside four other controls should not be the
    loudest thing on it.

    **Three regressions, all mine, all from the same blind spot.** A positioned
    element paints above a static sibling, and I keep meeting it from a new
    angle. The frost moved onto `.nd-frame-glass` last round and the nested
    frame's bar was `static` — so the layer covered it and **the header
    vanished**. `relative` fixed that and broke the next thing: `inset-block-start:
    var(--nd-header-h)`, written for the sticky case, applies to `relative` as a
    shove, and the bar landed 65px down **on its own stage**. And once the page
    frosted its own frames, the same rule bit a third time at the top level:
    the stage was only positioned inside a rig, so **every frame's examples went
    under their own frost** and washed out.

    **Two real defects the tests found while I was fixing the tests.** At 768px
    the nested frame's menu hung from a bar narrower than the panel and started
    at **x = -86** — off the left of the window. And a frame low on the page left
    `100vh - anchor` of room, so its menu opened as a sliver at the bottom with
    every row of six below the fold. Panels are clamped to the window now and
    flip above their control when there is more room up there, which is the
    ordinary popover answer to both.

    **The page frosts the frames on it**, exactly as a rig frosts the frame in
    it, whenever `glass` is not `off`. The note under the Glass row is a
    `Message`, so the value it refuses is a code chip rather than a word with
    backticks around it.

    **The bar stopped travelling with the page, and the frame got a surface.**
    Sticky under the header kept the controls of a several-screens-tall frame in
    reach, and it cost three defects to do it — sticking to a rig instead of the
    page, being shoved onto its own stage by the offset it needed for sticking,
    and being painted over by the frost after being made static to stop the
    first two. It stays at the top of its own frame now, `relative` rather than
    `static` because it still has to paint above the glass layer and still needs
    a `z-index` while its own menu is open.

    The frame had no background at all — a hole with a border, with the page's
    pattern running straight through the examples. Its surface is
    `--nc-color-surface` at the point of use (never `--noctua-page-background`,
    which is declared at `:root` and freezes to the root's palette) and it lives
    on the LAYER rather than on the frame: a `backdrop-filter` samples what is
    painted behind the element carrying it, so a fill on the frame would be
    behind the layer and the frost would sample the frame's own colour. One
    element is the frame's surface either way — flat when the glass is off,
    frosted when it is on.

    And the ExampleFrame page lost its essay: four headings, a four-item list
    and three paragraphs, replaced by one. The controls carry names and the
    frame below is the demonstration; a page that explains an apparatus a reader
    can simply press is a page nobody reads twice.

35. **The page background was a boolean, and it is a choice of six.** `blackout`
    was on or off — pure black, or the palette's own dark. It is `backdrop` now:
    `plain`, `cool`, `black`, `tinted`, `soft`, `softer`. Blackout was the name
    of one VALUE, not of the choice, and keeping it would have left five of six
    options as settings of a thing called blackout.

    **None of it needed a change to `noctua-colors`.** Every backdrop asked for
    was already solved there, per palette, in both modes — `--nc-color-surface`,
    `-surface-cool`, `-accent-bg`, `-surface-subtle`, `-surface-raised`. Five of
    the six rules therefore carry no mode condition at all: the token already
    knows what it means in each. Measured, for `blue-vivid` in dark: `17,17,17`
    · `15,17,20` · `0,0,0` · `19,24,31` · `24,23,23` · `32,31,31`.

    **And light defaults to paper.** It was `#fdfdfd`, the lightest step of the
    neutral ramp, and it is `#fff` now — a second colour literal beside `#000`,
    earning its place the same way: no ramp token is pure white, only the
    document background reads it, and `--nc-color-surface` is untouched, so not
    one of the 48,441 pairs the colour system measured moves. There is a test
    that says exactly that — the page is `255,255,255` and the contract is not.

    `black` is the one value with no light meaning, so it is the only one that
    needs both halves of the dark condition written out; in light it falls back
    to `plain`, which is the pure white it would have wanted anyway.

    **What the rename cost, and what caught it.** `shell.js` imported
    `blackoutValues` by name from the generated module, so the whole script
    failed to load and **47 browser specs went red at once** — which is the
    generated model doing its job: a setting renamed in the spec cannot be
    half-renamed. And the frame's own surface was `--nc-color-surface`, which
    stopped being the page the moment the page became a choice; it reads
    `--noctua-page-background` now, for the same reason its bar keeps the page's
    theme.

    The readability sweep went from 16 cases to 48 — every pattern × every mode ×
    every backdrop — because a pattern's peak ink over a lifted dark page is a
    contrast pair nobody had measured before this.

36. **Ten pages that were one page.** `Em` was a page. `Strong` was a page. A
    reader asking which mark a keystroke takes opened three of them, and each
    held one paragraph, one frame and a two-row table. The ten marks that go
    inside a sentence — `Abbr` `Code` `Del` `Em` `Ins` `Kbd` `Mark` `Strong`
    `Text` `Time` — are one reference now, at `/inline-text/`. Thirty-five pages
    became twenty-six.

    Nothing about the components changed: same contracts, same packages, same
    examples. Only where they are documented moved.

    **Which ones moved is in their contracts, not in a list beside the site.**
    `[meta] page = "inline-text"` on each of the ten; `site::pages` groups by it
    and **panics on a value no page declares**, because a component naming a
    collection that does not exist would otherwise be documented nowhere at all
    — on no page of its own, and on no collection. Confirmed by pointing `Em` at
    `"nowhere"` and watching `check` go red. An eleventh mark documents itself
    with one line.

    `ComponentPage`'s body is `ComponentBody { nested }` now, used by both kinds
    of page. `nested` moves every heading down one, so on a collection the
    component's own name is the `<h2>` and its blocks are `<h3>` — otherwise the
    outline is a list of headings with no tree in it.

    Two things the change turned up. A sidebar test asserted the component block
    was sorted by comparing it with a plain `.sort()`, which is codepoint order
    and agreed only while every title in the block was an ASCII identifier;
    `Texto em linha` is a sentence, so the assertion collates now, like the
    sidebar itself. And an anchor jump landed **under the sticky header** —
    `#kbd` scrolled the section to y=0 and the header sat on its name. One
    `scroll-margin-block-start`, and a test that measures the heading against
    the header's own bottom edge.

37. **A frame owns what is inside it — and entry 35 above is now wrong in one
    sentence.** Entry 35 ends by saying the frame's own surface "reads
    `--noctua-page-background` now". That property does not exist any more, and
    the reason it had to go is the whole of this entry. This is a log, so that
    sentence stays where it is; this is its successor.

    **The report.** Page on light with glass. On the one page with a frame
    inside a frame, set the OUTER frame to dark with glass off. The inner frame
    stayed light, with a frosted header. Reported more than once.

    **Why it kept coming back: it was the documented behaviour.** `AGENTS.md`
    said *"AN EXAMPLE FRAME IS SEALED FROM EVERYTHING BUT THE PAGE,
    GENERICALLY"*; `shell.js`'s `requestAt` overwrote any nested frame's request
    with the page's whole theme; and two browser tests were named after the
    wrong rule — one of them literally *"a frame inherits the page, never a
    frame around it"*. Every previous fix patched the script and was put back by
    the paragraph and the tests. Inverting all three together was the only
    change that could hold.

    The rule now: **a setting a frame states owns its whole subtree; a setting
    nobody stated comes from outside.** Reset clears the request rather than
    stating the page. `tests/browser/frame-isolation.spec.js` is named after the
    claim so it cannot get lost again, and it records the two traps it has
    already fallen into — never `stage.getAttribute(…)` (a frame that stated
    nothing carries nothing, so the assertion passes with the bug present *and*
    absent), and ask the pixels or ask what is in force.

    **Four correctness fixes the seal had been hiding**, each a regression the
    moment the seal came off. `inheritedAt` found its parent by the STAMPED
    attribute, so what a scope inherited depended on which ancestors a pass
    happened to have visited — theme the inner frame first and it took the page.
    `themeScope`'s accent branch read the PAGE's palette, so choosing `teal` in a
    frame that inherits `rose-muted` moved two axes for one click. `resolveScopes`
    ensured the stylesheet for a palette a sparse request does not name.
    `requestFrom` listed five setting names by hand and fed the React mount, so a
    frame themed with anything else handed React a different request than Dioxus
    had rendered — on the one page that exists to demonstrate that the two agree.

    **All eleven settings are scopable now, not seven.** The backdrop, the
    pattern, the aurora and the grain were `scope = false`, and the reason was
    structural rather than a judgement: each put its payload on a DESCENDANT
    (`:root[data-…] .noctua-fx { background-image: … }`), and two bare rules of
    equal specificity are settled by SOURCE ORDER rather than by proximity —
    `spec/theme.toml`'s note about `syntax` had been saying so all along. `none`
    and `off` were the *absence* of a rule, so a subtree could not cancel what
    was above it.

    Each declares a custom property on the element carrying its attribute now,
    and one rule reads all four:

    ```css
    :root,
    [data-noctua-scope] {
      background-color: var(--noctua-backdrop);
      background-image: var(--noctua-grain-image, none),
                        var(--noctua-glow-image, none),
                        var(--noctua-pattern-image, none);
    }
    ```

    **No new element**, and that is the argument for it: a background is behind
    content by definition, so there is no `z-index`, no `isolation`, and none of
    the stacking-order class of bug that cost the old effects layer three
    regressions. `docs-site/src/shell.rs`'s `Effects` component and its two
    children are gone, along with their CSS — the page paints through the same
    rule a frame does, and `ThemeScope { pattern }` works for any consumer of
    the package rather than only inside this site.

    Two things it cost, both accepted and both recorded in place: the grain's
    opacity is baked into the generated SVG (a background layer has no opacity
    of its own, and `opacity` on the scope would fade the content), and the
    grain no longer breathes — one element carries one animation, and the
    aurora's drift is the one that reads as motion. A still grain is exactly the
    static end-state reduced motion already shipped.

    **Two frozen inks, unfrozen.** `--noctua-pattern-ink` and
    `--noctua-glow-ink` held `color-mix(in oklab, var(--nc-color-fg) 4%, …)` at
    `:root` — `frozen_color_indirection` in its textbook form, passing only
    because `dist/` is in the gate's `SKIP` list. Harmless with one page-level
    layer; the moment a frame paints its own, the ink is the PAGE's foreground,
    so a light frame on a dark page draws near-white on near-white and the
    feature silently does nothing. What survives as a token is the dimensionless
    percentage; the `color-mix` moved to the point of use, which is the trade
    `glass()` already makes.

    **Three defects the work turned up that nobody had reported.**

    *A scope kept the page's ink.* `dist/scope.css` re-declares 1,767
    `--nc-color-*` names so a subtree resolves them against its own palette —
    and does nothing for the properties that CONSUME them. `body { color:
    var(--nc-color-fg) }` resolves at `<body>`, and every descendant inherits
    the computed value, so a dark scope on a light page drew near-black text on
    a near-black surface: about 1.2:1. It went unreported because every
    component that sets its own colour looked right; only prose that inherited
    was wrong. One zero-specificity rule at the end of `scope.css`.

    *A frame's saturation control did nothing at all.* `apply` has handled
    `accent` and `saturation` together since it was written — both recompute the
    palette, because the palette is what reaches the DOM. `themeScope` handled
    only the accent, so choosing `vivid` in a frame's menu stored a value no
    attribute is stamped from and no rule reads. The row moved, the chosen state
    moved, and the frame did not change. It lied silently because the row beside
    it worked.

    *The documented prop table was four props short and a test said otherwise.*
    `the_documented_props_are_the_scopable_settings` compared one hand-written
    list against another hand-written list, so the two agreed with each other
    and neither had to agree with `spec/theme.toml`. It reads
    `SCOPABLE_SETTINGS` from the generated model now. The same shape of fix went
    into `scope_attributes`, whose body listed five settings by hand while
    everything around it came from `scopable`; it is generated, with the field
    set following the spec.

    **A test that passed with its own defect restored.** The scope-ink guard was
    first written the way the rest of `readability.spec.js` is written — clip to
    the words, read the extremes of the screenshot. It passed with the fix taken
    back out, because a probe laid over a live page catches whatever is painted
    in that rectangle rather than its own glyphs; the header's furniture was the
    light end. It names the two colours and paints them into a 1×1 canvas
    itself now. **Every new guard in this change was confirmed by restoring the
    defect it names**, and this is why.

    **The aurora finally has a measured bound.** The pattern's 4% has been
    measured since it was written; the aurora's 18% never was, and it was the
    one that needed it — its lobes are 60%×50% *of the box*, which on a page is
    two washes in the margins and inside a 320px scope is a wash over the
    content. Measured at 320px, in both modes, against a real glyph: it clears
    4.5:1, so the ceiling stands where it was.

    **The menu split.** The four moved out of *The page* and into a new
    *Background* section shown in both reaches; `syntax` is alone in the
    page-only one, and it stays there for the structural reason its own comment
    gives. The test that guarded that section was asserting four names, one of
    which — `"blackout"` — had been renamed to `backdrop` entry 35 ago, so it
    was a selector matching nothing. It reads the model now, and asserts both
    halves: every scopable setting is offered on a frame, and no other is.

38. **A scene in three dimensions, and the four defects it took to get one.**

    **What it is.** `Lineage` and `LineageFigure`: figures and the
    relationships between them, placed in real space and drawn out of elements
    rather than on a canvas. It is on this list because four of its defects
    were found by looking at it rather than by reading it, and because two of
    them are the kind that look like something else entirely.

    **`translate3d(…) none` is not a transform list.** `none` is a value for
    the whole `transform` property; inside a list it is a syntax error, and a
    syntax error in `transform` drops the **whole declaration**. Every
    horizontal connection lost its transform and piled up at the scene's
    origin, while every vertical one — which carries a real `rotateZ` — was
    correct. It read as a routing bug in the layout engine, and the engine was
    right: the blessed cases said so. It took a computed style to see.
    `lineage.spec.js` asserts a horizontal run has a transform at all.

    **Depth was drawn toward the reader.** The engine counts planes outward
    from zero; CSS counts `+z` toward the camera. So the second plane was
    magnified by `d / (d − z)`, the eighth would have been past the camera
    altogether, and a raked scene of **two** planes was enough that
    `Page.captureScreenshot` timed out at thirty seconds and the tab never
    recovered. The whole documentation page looked like a compositor limit and
    was a sign error. The guard asserts a deeper plane's matrix has a negative
    `z`.

    **A figure's connections announced `u0`.** A union is a mark, so a person's
    only *direct* connection is to the mark between them and their partner — and
    a visually hidden link named after one says nothing to anybody. It was
    caught by `every_sentence_on_every_page_is_translated`, which noticed a text
    run that had never been through the catalogue, in a component whose ties
    exist precisely so relationships are not prose. A mark with no name of its
    own is walked through now, and direction is a rank comparison rather than a
    bond direction, because a bond points from a parent to a union and from that
    union to a child.

    **A filter of nothing but spaces matched nothing**, while the Rust test that
    claimed to cover it passed — on a double space in its own haystack, produced
    by a node with no metadata and no terms. The JavaScript half found it, which
    is what a second implementation held to the same file is for.

    **The variant grid, and what a specimen is for.** Without one, the grid
    renders the component's root and a sentence — which for this component is an
    empty field five cells tall, twenty-five of them on one page, each with its
    own perspective and its own 3D context. Two pieces and the run between them
    show `size`, `tilt`, `thickness`, `fog` and `glass` honestly and cost seven
    elements. `content-visibility: auto` on the scene does the rest: thirty-eight
    scenes on a page, one of them on screen.

    **What ships, and what does not.** `dist/lineage.js` is the first behaviour
    this repository ships — a navigation component whose consumer cannot
    navigate is a control that lies — with three.js vendored beside it and
    fetched on the first interaction. The component **styles** still live in
    `docs-site/assets/shell.css` like every other component's, so a consumer
    outside this repository gets a themed, unstyled, static scene: the §9.4 gap
    is unchanged and is now one file short of absurd.

39. **Everything entry 38 got wrong about looking at it.** The developer read
    the page and found seven defects in an afternoon, and every one of them was
    something a screenshot would have shown.

    **The frame grew and shrank as the pointer crossed it.** Two causes, and
    both were mine. `content-visibility: auto` on the scene's wrapper applies
    size containment while the element is skipped, and the wrapper's width came
    from its content — so every scene was **zero wide** until it scrolled into
    view and then snapped to full width. And `CSS3DRenderer` builds a DOM of its
    own and sizes it with `setSize`; dropped into a grid container that centres
    what is in it, that sized element became the thing the field measured itself
    against. The containment moved to the field, whose height is stated in CSS
    and whose width comes from its parent, so there is nothing left for it to
    collapse; and the renderer's DOM is gone — what is used now is the part that
    matters, **the camera matrix, composed exactly as `CSS3DRenderer` composes
    one**, written onto the scene that is already there.

    **The camera rendered the scene upside down.** `CSS3DRenderer` negates `y`
    twice — once in the camera matrix, because CSS counts it downward, and again
    in every object's own matrix. Composing only the first is a scene that reads
    bottom to top, which is what the first version of this shipped.

    **The scenes opened tilted, and nobody asked them to.** The resting camera
    is `level` now. Depth is still there — a figure is a box, the planes are
    apart and step sideways as they recede — and the first drag shows it.

    **A figure was a card with a second card behind it**, which read as exactly
    that. It is a box now: a back and four walls, thicker, and the walls are
    drawn only once there is an angle to see them from. That last clause is not
    an optimisation detail — four faces per figure on a page of thirteen scenes
    is what stopped the compositor answering.

    **A connection's label had its own line through it.** The label sits half a
    slab in front of the run now.

    **The variant grid showed identical pictures side by side.** A specimen is
    one piece of static markup for every value of every prop, so `tight` and
    `roomy` were the same picture twice — a control that lies about its own
    effect. The sample is keyed off the attribute the grid is varying, and it is
    **flat**: thirty-nine perspective contexts for a still picture is thirty-nine
    3D contexts nobody asked for. The camera questions it cannot answer —
    `view`, `tilt`, `fog` — are answered by worked examples side by side.

    **And the examples were thin.** There are seventeen now, one for every
    feature, with portraits at every figure rather than initials — placeholders
    and nobody's likeness, attributed beside them.

40. **The width defect, named correctly on the third attempt.** Entry 39 says
    the frame grew and shrank because of `content-visibility` and because
    `CSS3DRenderer` inserted a sized element. Both were true and neither was the
    cause. This is its successor.

    **Every example on this site sits in a flex container that shrink-wraps its
    items.** `.nd-example-stage` is `flex-direction: column` with
    `align-items: start`, so a child with `width: auto` is sized by its own
    max-content — and for a scene that is the whole drawing. Two consequences,
    and the developer saw both: a family wider than the page made **the page**
    wider than the window rather than scrolling inside its own window, and
    anything that shrank the drawing shrank the box with it. Putting
    `content-visibility: auto` anywhere inside that box shrank the drawing to
    nothing, which is how a scene came to be **seventy-five pixels wide** until
    a pointer went near it.

    The fix is two declarations and neither is the one tried twice before:
    `width: 100%` with `min-width: 0`, so the box fills what it is given and may
    shrink below its content; and `contain: inline-size`, so the drawing
    contributes nothing to any intrinsic width and no ancestor is ever sized by
    it. The variant grid's samples are excluded, because their track has no
    width of its own to inherit and containment would collapse them.

    **`content-visibility` is gone entirely**, and the compositor is fine
    without it now that the walls of a figure are drawn only when there is an
    angle to see them and the variant samples are flat.

    `lineage.spec.js` measures every example's field and requires them all to be
    the same width — which is what nobody was checking while three different
    explanations were being written down.

41. **A picture that rearranged itself because somebody looked at it.** The
    reported defect, in the developer's words: hovering an example moved the
    whole thing down, and scrollbars that should not have existed disappeared.
    Both halves were true and they had one cause between them.

    **The runtime replaced the scrollbars with a camera on `pointerenter`**, and
    it did that by moving the scene: `data-nd-camera="three"` switched the field
    to `overflow: hidden` and the scene to `position: absolute` at the middle of
    it. So the layout before the handover and the layout after it were two
    different layouts, and a scene dropped **428 pixels** the moment a pointer
    crossed the box. Measured, not guessed.

    **And the scrollbars were wrong before that.** Every scene on the page
    carried a vertical one for exactly forty pixels — the half-card the drawing
    hangs over its own edge, plus the field's padding. Twenty-eight of fifty-two
    scenes were scrollable at rest.

    The answer is that the scene is **fitted** rather than scrolled. The cell
    comes down until the whole drawing is inside its window in both directions,
    with no script anywhere near it: `min(--nd-step, 100cqw / (w + 1),
    (window × step − air) / (h + 1))`, which divides a length by a NUMBER and so
    is ordinary `calc`. The window is a fixed size that nothing in it can
    change, the scene is out of the flow in **both** cameras, and the stylesheet
    composes the resting camera exactly the way `CSS3DRenderer` composes any
    other — so three.js arriving writes a different transform onto a scene that
    is already where it belongs. A guard renders every piece of every scene,
    mounts every camera, and requires nothing to move by more than a pixel.

    Three things fell out of that, each its own defect:

    - **`100cqw` inside a container's own declarations answers for the container
      above it.** Written on the field it was the window's width, so the widest
      example kept its full cell and hung a card over each edge. The container
      is `.nd-lineage`; the fit is computed on the field.
    - **A registered property is readable and an ordinary one is not.**
      `getComputedStyle(root).getPropertyValue("--nd-pitch")` on a tilted scene
      answers `var(--noctua-scene-tilt)` — the source text — so the camera read
      zero for every tilted scene and drew it flat. `@property` gives the four
      the script and the stylesheet share a type.
    - **Both camera angles are negated**, and that was checked rather than
      reasoned into place: the composed matrix was compared with the
      stylesheet's own at three unrelated pairs of angles, and one combination
      of the four reproduces it.

42. **Everything in a scene is measured in cells now**, because the fit made
    that necessary. A card that stayed 5.5rem tall in a cell the fit had brought
    down to 5rem overlapped the row below it, and a name that stayed 16px in a
    card two thirds the size was given **zero pixels** by the grid — one example
    drew a portrait above a date with nothing between them. So `card`, `mark`,
    `slab`, `face`, `name`, `meta`, `pad`, `gap` and `perspective` are fractions
    of a cell, and the card is a minimum with a ceiling rather than a fixed
    height: at least a card tall, at most a cell less the width of a connection,
    because a box that fills its whole cell is a box drawn over the bracket that
    reaches it.

43. **The rest of what a reader found, one example at a time.**

    - **Four scenes drew a dotted line into an empty box.** They used a graph
      with a `slot` in it and never filled the slot. Nothing on the page has an
      unfilled slot now, and a probe over every scene says so.
    - **A filter took every connection out of the scene.** A union point has no
      name to search for, so `matches("Doe")` was false for it, so every bond
      through it had an unmatched end — and in a genealogy every bond goes
      through a union. An unlabelled mark now answers when either end does,
      which is the same walk-through `ties` already made.
    - **Every connection that crossed a plane in the deep view stopped short.**
      Two defects at once: the deep view set two cells between planes while the
      router counts a run's length in cells, and a `z` run was rotated toward
      the camera rather than away. The isotropy at the top of the section is the
      contract the router computes against; the deep view does not get to break
      it.
    - **The walls of a figure waited for a reader to turn the camera**, which
      left the resting view — the one everybody sees — drawing flat cards with a
      rectangle behind them. A scene has a real perspective: a figure away from
      the middle shows a wall with the camera straight on. They are always drawn
      now, and shaded rather than outlined, because a line where two faces meet
      is the second card this was accused of being.
    - **Four more pairs in the variant grid were one picture under two names** —
      `resting` and `raked`, `thin` and `slab`, `bond` and `figure`, `tall` and
      `full`. `lineage.spec.js` now fingerprints every sample in every group and
      fails on a repeat.
    - **The page scrolled sideways at 1024 pixels** — on this page, on the
      tables page and on the typography page. A grid track's automatic minimum
      is the min-content of what is in it, so `1fr` was a floor and not a
      ceiling. `responsive.spec.js` stopped at 768 and resumed at 1440, which is
      the one gap it lives in; 1024 is on the list now.

44. **The second reading, and the fourteen things in it.** A developer went
    through `Lineage` again and reported what was still wrong. Every item below
    is one of theirs, and each is written where it was caused rather than where
    it was seen.

    **The mouse was not driving anything.** A drag selected the text in every
    card it crossed and finished by opening the browser's context menu, and the
    right button did nothing else at all. Both are the platform being helpful
    about a gesture that is not the platform's: `contextmenu` is refused on the
    field, `pointerdown` prevents the selection, and the right button pans —
    joining shift and the middle button, which already did.

    **A touch screen could pinch and nothing else**, because the browser was
    taking every drag over a scene as a scroll. One finger orbits now, two
    fingers pan and zoom and twist at the same time — they are measured
    together, so a sloppy pinch still does all three — and a double tap fits.
    `touch-action: none` is what hands the gestures over, and it is written only
    once the runtime exists, so with the script blocked the page still scrolls
    through every scene on it.

    **Zooming resized the frame around the scene.** `--nd-cell` carried the zoom
    and the window was measured in it. There are two now: `--nd-fit` is what the
    box is measured in and `--nd-cell` is what the drawing is drawn at.

    **A figure's body had four hard corners** against a rounded front. Each wall
    takes the corner the card has.

    **A connection's label erased the connection.** Centred on the run with an
    opaque surface behind it, a label left a word with a stub of rule either
    side and nothing between them. It sits above the line now and its background
    is a `color-mix` with `transparent`.

    **The thing a reader pressed to pick a connection was not the connection.**
    The runs are inside the `<label>` now — `display: contents`, so nothing
    moves — and a picked line keeps its own colour and gains a band of that
    colour travelling along it. It used to turn accent, which reads as a
    different KIND of connection rather than the same one singled out.

    **A dimmed figure had a line drawn across its face.** `opacity` on the card
    made the card see-through and what showed through was the connection behind
    it. The surface stays opaque and the writing on it fades.

    **A union point sat a third of the way along its own line.** It is placed
    like anything else on its row, and where it landed depended on what its
    partners had to make room for underneath. A mark joining exactly two pieces
    on its own row is moved to the midpoint of the two — in cells, because the
    middle of two half-cells is a quarter and a quarter is a number both engines
    print identically.

    **A bracket cleared the row above it by twelve pixels.** The channel is
    eleven twentieths of the way down rather than ten, written as integers over
    twenty for the reason every other number in that file is an integer.

    **`isolate` left connections running to the edge of the drawing.** The
    filter took figures out silently, so a family with a filter on it looked
    like a family with a mistake in it. The engine emits one **cut marker** per
    removed group now — a dashed ring carrying how many people are behind it —
    standing at the place of the member that touched what stayed, with the
    connection to it drawn.

    **Four pairs in the variant grid were one picture under two names**, and the
    two that were not were distorted: `spacing` faked its difference with
    `scale: 0.8 1`, which stretched every letter in the sample. The specimen is
    a parent and TWO children now, and each variant moves the drawing rather
    than squashing it — the children move apart and the bracket between them
    grows by exactly as much as its ends did.

    **In the deep view a whole person was hidden inside their cousin's
    partner.** The view kept the flat layout's `x` and pushed each branch back
    in `z`, so a branch already standing to the right also got the sideways
    step. Each plane begins where the plane in front of it ended now, and
    nothing in the scene is behind anything else.

    **Every example was a genealogy missing half its parents.** A child hanging
    off one parent is a claim rather than a gap. Every graph on the page gives
    every child two, unknown where unknown, and there are two new scenes for the
    cases a tree of couples cannot state at all: a child with two biological
    parents and two who raised them, and a donor beside the parent who carried
    the child and the partner who is neither.

    **The server was drawn as a member of the family.** It is a control: it
    stands in its own cluster beside the scene, the branch it brings in carries
    its own tone, and pressing it a second time puts that branch away.

    **The Christmas example was four people not wearing hats and an owl.** It is
    its own scene now — three people, six figures, one occasion, and everybody
    in a hat.

45. **One `ExampleFrame` per example, on every page**, which reverses entry
    §9.10's "one frame per page, holding every example on it". A frame is a
    viewport: the question it answers is *does THIS survive a narrow column*,
    and answering it for thirty examples at once answers it for none of them. A
    reader who wanted one example at 320 pixels had to take the other
    twenty-nine there with it. Three browser specs assumed "the frame" meant
    something and were rewritten to say which one they mean — the first, or the
    one with a frame inside it, or the one with the most frostable controls in
    it, each found by description rather than by position.

46. **An owl, in WebGL, which is documentation and not a component.** The origin
    figure on the `Lineage` page is a modelled, lit owl whose head and eyes turn
    toward the pointer, damped, with a small idle drift and none of it under
    reduced motion. It is the strongest available demonstration of the one thing
    the component's contract claims about a figure's interior: that it is
    anybody's.

    **It cost a second vendored bundle and it does not ship.** `three-webgl.js`
    is 481 KB — the renderer, four geometries, a material and two lights — and
    it is copied to the documentation site and **not** into `dist/`. A consumer
    of `Lineage` never fetches a byte of it. It arrives on the first frame the
    owl is on screen, and never if a reader does not scroll to it.

    **Its colours are painted and then read.** The tokens are `oklch()`, and
    three.js's colour parser does not know that space — it answers white for
    every one of them, which is how the first owl appeared: a white bird on a
    white card. A 1×1 canvas is the recipe the rest of this repository already
    uses for the same question.

47. **The audit, and the reason it exists.** A reader listed ten more defects in
    one message and finished with the sentence that matters: *these are basic,
    obvious UI problems, and they will keep happening unless the way you work
    changes.* They were right. Every one of the ten was a MEASUREMENT — two
    boxes intersecting, a gap of zero, a line ending on nothing — and none of it
    was being measured. Screenshots were being looked at instead, a few of them,
    at one width.

    `tests/browser/lineage-audit.spec.js` walks **every scene on the page, at
    320 / 768 / 1024 / 1440, in both locales**, and fails on six things: two
    boxes overlapping, two figures with no air between them, a connection whose
    end reaches nothing, a label written over a face, a box clipping its own
    content, and anything outside its window. It names the scene, the two things
    involved, and how far it missed by — *`deep`: a connection ends 33 pixels
    short of `dan`* — because two page coordinates tell whoever reads the
    failure nothing about which picture to open.

    Run against the build that was reported, it found **37 findings**, and they
    were the reader's ten by name. Each of the six rules has since been
    confirmed by restoring the defect it names: a figure as wide as its cell
    brings back 20, a run drawn half its length brings back 258.

    **Two of the rules had to be taught what a drawing is.** A bracket's stem
    meets the middle of its channel rather than the end of it — a T rather than
    a corner — and reading that as a loose end reported every elbow on the page.
    And a run travelling in depth is foreshortened, so its bounding box is the
    projection of a line rather than the line; bonds with a `z` run are excluded
    by name, and the exclusion has to look at every run rather than the visible
    ones, because a run pointing at the camera is nought pixels wide and was
    being filtered out before it could disqualify its own bond.

48. **What the audit found, and what each of them was.**

    **A figure was as wide as its cell.** Two siblings are one cell apart, so
    every pair of neighbours in every scene touched exactly — reported three
    times over as `comfortable` being wrong, as `flat` being wrong, and as
    "everything is glued together". A figure is 0.84 of a cell now. **`spacing`
    is deleted** with it: it was three gaps, two of which drew figures touching
    or overlapping, and no gap could have fixed a width.

    **The slab had square corners on a round card.** Four flat walls cannot
    follow a rounded rectangle. It is five copies of the card's own silhouette
    stepped back through the slab instead — rounded from every angle, no corner
    for a wall to miss, and the same five elements.

    **A label was written over a face.** Hung above its run, a label 26 pixels
    tall reached 11 pixels into the card above it; the channel between two rows
    is 23 pixels of air, so "above the line" and "clear of the row" were not
    both available. It is centred on the run, and the line reads through it
    because the background is a `color-mix` with `transparent`. A label on a
    UNION goes half a cell down into the channel: a union runs along the row, a
    word is wider than the gap between two cards on it, and there is nowhere
    else for it to be.

    **A card could grow to within a hairline of the row below**, which left no
    room for the bracket, let alone the label on it. Four fifths of a cell is a
    card's; the last fifth is the connection's.

    **A tooltip was behind the scene.** It had no depth of its own, so every
    figure with a slab was in front of it. It comes a whole cell forward.

    **A union dot was drawn on somebody's face.** Centring a mark between its
    partners is skipped when the midpoint would land inside a figure.

    **Crossings were counted for descent bonds between adjacent rows and nothing
    else**, so a union along a row and a bond spanning two were invisible to the
    thing whose job is to remove them — a reader sent a picture of exactly that.
    Every bond is a segment over every row it passes now, and an
    adjacent-transposition pass tries the swaps a median sweep cannot reach.

    **An origin was the ancestor of the people its descendants married.**
    Everybody parentless arrived under it. Refusing every married figure is the
    opposite mistake — two founders who married each other are both married, and
    refusing both leaves the origin joined to nobody and standing on their row.
    In declaration order, and a figure whose partner has already been taken is
    left to their partner: one line, to one person.

    **A cut marker rescued a connection whose other end was also gone**, which
    drew a line from a marker off into empty space. It rescues one only when the
    far end is still shown.

    **`span=4` is four cells, not two**, which is why the milestone was four
    cells wide and shrank its whole scene to a thumbnail with its text cut off.

    **An unknown parent said `—`.** It says *biological father unknown* now, in
    both locales, in a dashed muted card two cells wide — `kind=unknown` is what
    makes the room, because a sentence does not fit in a name's worth of it.

    **`Text` and `Avatar` inside a figure carried the type scale**, which is
    right everywhere except inside a scene that has been fitted: at 16 pixels in
    a 94-pixel figure "Eve Marsh" wrapped and the figure clipped its own second
    line. Anything standing in a scene is measured in cells.

    **The owl looked away from the cursor** — one sign — **and had no name**.

49. **One `ExampleFrame` per example** [45] made three browser specs ambiguous:
    they said "the frame" when a page had one. Each now says which one it means
    by description rather than by position — the first, the one with a frame
    inside it, the one with the most frostable controls in it — so an example
    added tomorrow cannot break them by shifting an index.

50. **Why three rounds of fixes each shipped new defects.** Every length in a
    scene — type, portraits, paddings, line weights — was a fraction of one
    cell, and the fit shrank the cell until the scene fit its window. So every
    fix was a tuning of fractions against whichever scenes were looked at, and
    the next scene picked a different cell and broke a different fraction:
    microscopic cards on the wide scenes, truncated words on the narrow ones,
    the same ten defects reworded each round. The redo changed the model
    instead of the fractions. **Type is never smaller than readable**: the cell
    is a fixed length, the across-pitch may compress to 0.85 of it and no
    further, and text and portraits are real lengths that layout never scales —
    only the `size` prop does. A scene wider than the floor allows **pans**:
    the platform's own scrolling with no script, the camera with one. The
    camera mounts on an act (a press, a key, focus) and never on a glance,
    and its first movement folds the reader's scroll position into the pan, so
    engaging it moves nothing.

51. **The 27-cell ratchet.** On the milestone scene, each build drew the family
    a little further from the kinless note — twenty-six cells away by the
    fourth. Two passes conspired: centring counted a partner on the SAME row as
    kin, so a figure jumped onto its own union mark; and the separation that
    restores minimum gaps resolved overlaps by pushing right only. Each round
    of centring therefore moved the row's mean rightward, and nothing ever
    moved it back. Partners count as kin only across rows now, and the spread
    restores the row's mean, so it resolves overlaps without transporting the
    row. Neither half was visible alone: the jump was legal centring, the push
    was legal separating, and the walk existed only in their loop.

52. **The two engines had quietly diverged.** `layout.js` is a line-for-line
    transcription of `layout.rs`, and the blessed cases assert the Rust half
    only — so the JS half had kept an older crossing counter (descent bonds
    between adjacent rows and nothing else) and had never received the
    transposition pass at all. Every React render of every scene was laid out
    by a different algorithm than the one being tested. Both halves are now
    held to `spec/lineage/cases.toml` — never to each other, because comparing
    the twins passes whenever both are wrong in the same way — with the JS half
    run over every blessed case by `tests/browser/lineage-layout.spec.js`. It
    is the §9.3 rule — *the spec is the single source* — applied to the one
    artifact that is hand-transcribed rather than generated.

53. **A card three pixels too short for its own name.** The figure card is a
    fixed height, sized to its worst content — and the sum of portrait, a
    two-line name, a date and the paddings came to three pixels more than the
    card. Flexbox resolved the deficit by squeezing the name box to 33 pixels
    against two 17.25-pixel lines, and `-webkit-line-clamp` answered a box
    shorter than its clamp by ellipsising the FIRST line: "Gramps Doe" rendered
    as "Gramps…" over "Doe", which a reader had reported on two other names in
    an earlier round. The card is 7rem now, with the arithmetic in the token's
    own comment, and the eyes pass over every example is what caught it —
    the audit measures type size, not type completeness.

54. **A partner with no other ties was stranded by the centring.** "Biological
    mother unknown" stood eight cells from her own partner with the union point
    floating in the empty middle: a figure whose every connection is a same-row
    union has no cross-row kin, so the centring pass — which moves a figure
    toward its kin — never moved her at all, and she stayed wherever the
    packing had left her while her union mark walked toward the children. Such
    a figure is now pulled flush against its nearest partner after the rounds,
    inside whatever room its row neighbours leave. One pass, no feedback: a
    figure with no cross-row bonds is nobody's kin, so moving it moves nothing
    else — which is what [51] requires of any pass that moves things.

55. **`isolate` re-lays the scene, and that is a decided exception.** "A filter
    marks, never moves" survives for `highlight`, which is the mode for a
    reader who must not lose their place. But `isolate` REMOVES people, and
    keeping the removed figures' geometry drew four people's empty space with a
    cut marker floating at the far end of a page-wide line. The subgraph that
    answers the filter is laid out fresh; each removed connected group becomes
    one dashed ring carrying the count of the people it stands for, re-aimed
    bonds keeping their kinds, so the ring stands exactly where the graph put
    the people it replaces — beside a union if they married in, under a
    bracket if they descended.

56. **The eyes pass, as a deliverable.** After the audit went green, every one
    of the fifty-two scenes on the `Lineage` page was screenshotted and looked
    at —
    1440 and 320 in English, the text-bearing scenes again in Portuguese — and
    the verdicts written down rather than summarised as "reviewed". It found
    [53], [54], and a christmas figure whose only text was "wearing a paper
    hat" with no name saying whose head the hat was on. Three things it
    accepted are recorded as accepted, not absorbed: a scene wider than the
    squeeze floor shows its left end first and pans; the deep view's planes
    overlap in projection, which is what planes stacked in depth look like
    through a fogged camera; and a label whose free stretch is shorter than a
    cell sits just below its line instead of just above, because a short
    stretch has a union mark and two card edges crowding the air above it.

57. **A click squeezed the scene to a strip two lines tall.** Two defects in
    one report. A click is never perfectly still — the mouse settles by a
    pixel or two as the button goes down — and the drag handler orbited on any
    movement, so a click in the air beside a figure was a one-pixel orbit and
    the camera engaged. And engaging collapses the field: the space leaves the
    flow (`position: absolute`), and a grid container whose only child is
    absolute is as tall as its padding. Both halves are fixed where each
    belongs: the camera moves only once the pointer leaves a four-pixel
    `STILL` circle — the frame that crosses it moves nothing, so the wander is
    never folded into the first turn, for mouse and finger alike — and the
    engaged field states the height the space was giving it, in `--nd-step`
    rather than the zoomed pitch, because the window must hold still while the
    scene zooms inside it. The glance spec's tap now wanders two pixels the
    way a real click does, and asserts the field's box across engagement; each
    half was confirmed by restoring its defect in the served build and
    watching its own assertion fail. **The proof run also caught the proof**:
    the first restoration "passed" against a `dist/lineage.js` the docs build
    had not regenerated — `cargo xtask docs` rebuilds the site, `cargo xtask
    build` rebuilds `dist/` — so the dead zone being proven was not in the
    bundle being served. Build, then docs, then proofs.

58. **The pan wore a permanent scrollbar.** A scene wider than the squeeze
    floor pans by design — but the field dressed as a scroll container to do
    it, and every wide scene sat over a grey bar until the camera's
    `overflow: clip` took it away, which the reader reported outright. The
    field hides its own bar now (`scrollbar-width: none`): wheel, trackpad,
    swipe and the focused arrow keys all still pan it, and the box no longer
    changes by a gutter's height when the camera engages. The pan spec asserts
    the promise both ways — the computed `scrollbar-width`, and that borders
    account for every pixel between `offsetHeight` and `clientHeight` —
    and restoring the bar in the served build failed it with an answer that
    taught something: the bar came back as `thin`, because `dist/theme.css`
    already thins every scrollbar on the page, and a thin grey bar under a
    drawing is still a grey bar under a drawing.

59. **Zooming out re-laid the drawing instead of shrinking the picture.** The
    camera's zoom was multiplied into the grid's pitches, so `--nd-cellx` and
    `--nd-celly` shrank while the cards' own lengths — fixed by doctrine —
    did not: narrow cards under full-size type, and the rows drawn into each
    other until the figures overlapped, which the reader sent a picture of.
    Worse in the engaged camera, where the distance already carries the zoom
    (`lens / zoom`), so the layout shrank AND the projection magnified: zoom
    squared across, zoom down. A camera zoom is a magnification of the
    finished picture — everything shrinks together, like stepping back from
    it — so the zoom now lives in exactly two places, one per camera: a
    `scale3d` in the space's resting transform, sitting with the rotations it
    commutes with, and the camera distance three.js already had. The pitches
    never see it. A pan stays a promise in screen pixels in both halves — the
    stylesheet keeps its pan translate outside its scale, and the projection
    divides its look-at target by the zoom. The spec zooms a scene out by
    keyboard and holds two ratios across it: a card against its own height,
    and the row pitch against the cards standing on it — each confirmed by
    restoring its own half of the defect in the served build.

60. **The children centred under a phantom, and the row air was nine pixels.**
    Three reader requests, one drawing. The stem into a card's top was nine
    pixels because one token pitched both axes: the air between two rows —
    where the brackets, their labels and the stems all live — was the cell
    minus the card, and the card had grown. The down-pitch is its own token
    now (`scene.row`, 10rem to the cell's 9). And the children of a union sat
    two cells right of the point descending into them: the crossing passes
    have no opinion about a mark ALONG a row — a same-row bond crosses
    nothing they count — so the union mark stayed where declaration order
    left it, after both partners. The children centred under its laid-out
    position, off past the second partner, while the dot itself was DRAWN
    back on the stretch between the two; everything agreed with everything
    except the picture. `seat_marks` now seats every union mark between its
    partners in the ordering — the mark's key is the mean of its partners'
    orders, compared as a fraction so the integers stay integers — and the
    drop, the dot and the bracket's middle are one x: Cara under Ada, Dan
    under Eve, in the reader's own words. **Re-blessing was also not what it
    claimed**: the `bless` tool PRINTS the recomputed cases and the command
    in use piped its output to `tail`, so a "re-bless" had been a no-op and
    the previous round's engine change survived only because it happened not
    to move any blessed case. The output goes through a filter into the file
    now, and the check ran green against a file the engine actually wrote.

61. **The buttons traded places, the wheel became the zoom, and the wheel was
    already broken.** Three reader asks and one find. The left button pans
    now — moving the picture is the gesture a reader wants first and most, so
    it sits on the button that needs no knowledge — and turning moved to the
    right and middle buttons and shift, where panning had been. The wheel
    zooms an ENGAGED scene and only an engaged one: before the reader has
    taken the camera it still belongs to the page, because taking every wheel
    event would strand a reader who scrolled onto a scene on top of it, over
    every scene on the page. Proving that boundary found it already broken
    from the other side: `overscroll-behavior: contain` — there to keep a
    horizontal pan from becoming the browser's back-forward swipe — was
    declared for both axes on an element that only scrolls horizontally, so
    the vertical wheel hit a scroll container that could not move, chaining
    was forbidden, and the page under the pointer had quietly stopped
    scrolling over every scene. It contains `x` alone now. The unknown-parent
    sentence also breaks in two even lines ("Pai biológico /
    desconhecido") — a `16ch` cap makes it wrap, `text-wrap: balance` keeps
    the halves even in both locales, and the first cap tried taught why `ch`
    is not a character: "mother unknown" is fourteen characters, wider than
    fourteen `ch`, and at `14ch` the clamp ellipsised a third line.

62. **A union line drove through the sister-in-law.** In the filter example
    the row read Ada · union · Syb · Eve: the ordering had left Syb standing
    between the union point and Eve, so the line from Ada to her wife ran
    straight through Eve's sister — which reads as a different marriage. The
    reader stated the rule this breaks as an absolute, and it is one now, at
    both levels. **The audit**: a line never passes through a card that is
    not its destination — measured as a run whose penetration of a card is
    deep while BOTH its ends are outside it, because every legitimate stem is
    drawn to its own piece's centre and so always has one end inside; the
    first draft measured penetration alone and reported every leg of every
    bracket in the book. Marks are exempt (a union point sits on its own
    line), diagonals are not measured (their box is not the line). Run
    against the shipped build it found exactly the reported defect —
    `highlighted: a union line drives 105 pixels through syb` — and nothing
    else. **The engine**: `flank_marks`, after `seat_marks`, closes the row
    around every union mark — the nearest partner on each side is pulled
    flush against it, and whoever stood between slides outside. `pull_partners`
    had refused exactly this move because it never worsens the crossing
    count on its watch, and a rule stated as absolute outranks a count:
    crossings are recounted after and reported as whatever they then are.

63. **The owl looked up at a pointer under her beak.** The horizontal gaze had
    been inverted, was flipped, and was PROVEN — the stamp test moved the
    pointer hard to each side and read `data-nd-owl-look` back. The vertical
    was flipped "to match" in the same edit and proven by nothing, and its
    reasoning was wrong the same way the horizontal's had been: positive
    `rotation.x` rotates `+z` toward `−y`, which tips a camera-facing head
    DOWN, so the screen-down pointer axis carries straight through and the
    "matching" negation pointed her the other way. A reader photographed her
    looking at the ceiling. The stamp now carries both axes ("right down",
    "left up"), a spec drives the pointer to two corners and trusts nothing
    but the render's own rotation written back, and the fix was also
    confirmed by eye at four times scale: pointer below, pupils at the floor
    of the eyes; pointer above, face tilted back. The lesson is the one this
    file keeps relearning: a sign fixed by argument is a sign that will be
    fixed again — measure both, or one of them is a guess wearing the other's
    proof.

64. **A turned scene drew the legs over the faces, and the deep fog was
    transparency.** Every run is drawn to its piece's centre and the card
    covers the overlap — but run and face were COPLANAR, so the cover held
    only at rest: turn the camera and the depth sort drew the stem over the
    portrait it entered, which a reader photographed. Every run now lies two
    pixels behind the faces' plane: parallel planes never cross, so the card
    wins at every angle, and the projected offset at the resting tilt is
    under half a pixel. The same photograph named the second half — in the
    deep view the fog was `opacity` on the card, so every deeper figure was
    see-through and its own centre-drawn legs read straight through its
    face. A line may fade by transparency; a card may not: the card's fog is
    an opaque VEIL of the page's own surface laid over the content, so a
    deep figure pales into the distance and still hides what passes behind
    it. Two specs hold the two signs — a run's computed `matrix3d` carries
    `z ≤ −2`, and no visible deep card computes an opacity under one — and
    each failed when its own defect was restored in the served build.

65. **The unknown parent out-shouted the parents.** The card that exists to
    say "nobody knows" was the biggest, darkest thing in its scene: two full
    cells wide and a full figure tall around two lines of text, washed with
    eight parts ink in a hundred. It is quieter than a person now, in every
    dimension a card has — a cell and a half wide (the sentence breaks in two
    even lines, so half the extra cell carries it), as tall as its own
    sentence plus breathing room, and washed at four parts in a hundred. The
    extrusion body followed the card for free, because the body is `inset: 0`
    in a piece that is exactly as tall as its card — the mechanism the fix
    leaned on rather than fought.

66. **The camera was caged, and the fog is deleted.** Flying into a deep
    scene hit invisible bars: the zoom was clamped between 0.15× and 4× of
    the fit. A reader asked for travel that is free in every direction and
    every sense, and it is — the zoom is multiplicative, so it approaches
    zero and infinity without arriving, and the one bound left is a
    thousandth, because `apply` writes the zoom to three decimals and
    anything smaller serialises as `0`: `scale3d(0)` does not draw a scene
    very far away, it erases it. At a thousandth the whole drawing is
    already smaller than two pixels; double-tap, `0` and "fit the whole
    scene" remain the way home from anywhere. Proven by flight: 36,000×
    into the scene, camera between the planes, and out to the floor. The
    same message deleted `fog` — the reader said the effect made no sense,
    and the prop went the way `numbered` and `spacing` went, whole: the
    contract entry, both locales' documentation, the token, the attribute in
    both emitters, the CSS, and the example that set it. The see-through
    spec stays, renamed, so a future depth cue cannot bring the
    transparency back with it.

67. **Zooming in pressed against an invisible wall.** Freeing the zoom [66]
    freed the number, not the space: `distance = lens / zoom` approaches the
    front plane for ever without arriving, so the front plane magnified
    without bound while the deeper planes — whose scale converges to
    `lens / depth` — stopped growing, and a reader pressing toward Fay and
    Dan hit glass. "Não parece ser um ambiente realmente 3D", and it was
    not. Past 4× the wheel now FLIES: each further notch moves the camera
    itself forward along its own view direction, without limit, through the
    scene and out the other side; zooming back retraces the flight to where
    it began and only then resumes shrinking the picture, and fit remains
    the way home from anywhere. What the flight puts behind the camera is
    hidden frame by frame — CSS draws no frustum, and an element behind the
    eye is not skipped but smeared across the screen mirrored and enormous —
    with each piece, run and label measured against the camera's plane, at
    computed values rather than inline ones, because a figure standing at a
    named point carries `--nd-x: var(--nd-at-ada-x)` and only the computed
    value has done the substitution. Proven in flight: one piece behind
    after the first leg, the whole scene behind after the third, an empty
    room beyond it, and forty notches back out to the whole tree with
    nothing left hidden.

68. **Every joint was a butt joint.** Two runs meeting at a corner each
    stopped exactly at the path point — so each fell half the line's own
    thickness short of the other's far edge, and every elbow had a notch and
    every T a step. Nobody sees three halves of a pixel at rest; the flight
    [67] magnifies a corner twentyfold and the pipe was visibly broken,
    which the reader photographed. Every run is now half a line longer at
    each end — a cap, `translateX(-line/2)` and `+line` of width in the
    run's own frame — so every corner and every T closes into a
    geometrically exact square at any magnification. At the far ends the
    cap vanishes into the cards the runs already enter, and the audit's
    penetration and loose-end tolerances never notice a line's own
    half-thickness.

69. **`line: Beam` — a connection with a section.** A reader asked for lines
    with real volume for the three-dimensional views, and for both formats
    to be shown. The prop is an enum beside `route` — `ribbon` (the flat
    strip, the default and all a flat view can show) and `beam`, which
    closes every run into a square tube with the same anatomy as a figure's
    body at line scale: the run is the front face, three children fold the
    back and the walls, and `background: inherit` means a dashed or coloured
    run dashes and colours all four faces without a second set of rules. The
    caps every run wears [68] close the corners in three dimensions too.
    Both formats are exemplified twice: a paired example (`Beam, Raked` ·
    `Ribbon`) and the variant grid — where the first attempt taught two
    lessons at once. The grid failed "no two values draw one picture",
    because the SPECIMEN carries no beam faces: the sample is spec HTML, not
    a component render, so the faces had to be added there as inert
    elements the beam mode folds out. And the first sample fix — a yaw so
    the section would read — put four audit findings on the board, because
    a turned box is outside the flat arithmetic the audit measures with.
    The line samples pitch hard (sixty degrees, pitch only) instead: a
    ribbon foreshortens to a sliver, a beam keeps its folded wall, and
    every rule keeps its geometry.

70. **Volume is the resting state, and glass was a word.** Three reader
    asks. `thickness` rests at `slab` now — the component is three-
    dimensional, so its resting look is — and `line` lost its static
    default: unset, it FOLLOWS `thickness`, beams under figures with volume,
    ribbons in a deliberately flat scene (`thickness: none`). The mechanism
    is the `Tag.pressed` precedent: an enum with no spec default is an
    `Option`, and the absence of the prop is itself an answer, resolved
    once in each emitter so the attribute a consumer reads always carries a
    real value. And `glass: true` — on the scene and on a figure — had
    shipped changing NOTHING a reader could see: both attributes were
    emitted faithfully and styled by nothing, because the theme's one glass
    recipe lost its `background` to the opaque surfaces shell.css declares
    later. The surfaces yield to the frost now (the filter stays the
    theme's — a second `backdrop-filter` is a source-gate finding), and a
    frosted card hides its extrusion body, because a pane of glass with a
    grey box glued behind it reads as murk, not material.

71. **Lineage's glass is deleted.** One message after the frost finally
    became visible [70], the reader removed the option outright, and it went
    the way `fog` went [66], whole: both contract entries (`Lineage.glass`
    and `LineageFigure.glass`), both locales' documentation, the attribute
    stamps in all four emitter halves, the frost-and-body CSS from [70], and
    the two examples that set it — the aviary and the round figure, whose
    captions no longer claim it. The system's glass material itself is
    untouched; what is gone is this component's doorway to it.

72. **The wild scene: a photographed owl, no boxes, shy lines, solid type.**
    One request, four features, three of them contract entries. `shape=bare`
    on a graph figure finally DOES something — the grammar always accepted
    it and the piece always emitted it, and no CSS consumed it; now it melts
    the card (no border, no surface, no body) while the transparent card
    element stays as the anchor the runs and the audit measure against.
    `reach: shy` trims every figure-bound run end by 0.35 of a pitch, in
    both engines, held by a blessed case whose numbers show the trim to the
    digit (0.85 = 0.5 + 0.35) and by the parity spec — which failed first,
    because `dist/lineage.cases.json` had not learnt the `reach` field and
    the JS half was quietly running untrimmed; the audit's landing rule
    widens by exactly the trim for shy scenes rather than skipping them.
    `lettering: solid` extrudes every name and date: five aria-hidden plies
    behind the real text, turning with the scene — with a base rule keeping
    plies inert everywhere else, because the variant SPECIMEN carries three
    per name and shown flat they would print every name six times. The
    variant grid caught both new props drawing one picture per pair and
    got honest samples: specimen-declared `data-nd-shy-tip` ends for the
    trim, the hard sixty-degree pitch for the plies. **The owl itself is the
    one thing not here yet**: every photoreal scan worth shipping sits
    behind an account wall automation must not pass, and the one anonymous
    mirror found carried no license at all — so `data-nd-owl="real"`, the
    GLTFLoader bundle (three-webgl.js grew to 558 KB), the centring, the
    gaze rig and the graceful flat-portrait fallback are all in place, and
    `assets/vendor/owl/ATTRIBUTION.md` says exactly what file to drop in
    and which two Sketchfab taxidermy scans to take it from.

73. **The scan landed, and she flies.** The developer supplied the model by
    hand — a rigged, Blender-exported glTF with embedded textures, no Draco,
    and one skeletal animation — and the nest plays it: an `AnimationMixer`
    advances the clip by real elapsed time (clamped, so a background tab does
    not leap), loops it, and holds a pose rather than a glitch under reduced
    motion. Two findings on the way in, both the machinery doing its job.
    The first fixed scalar left her a speck in her own sky — the scale is
    computed from the camera's own frustum now, wingspan to 92% of the
    frame, whatever the nest measures — and her nest is a sky, not a coin:
    wider than tall, canvas sized to the whole box instead of the inscribed
    square. And the audit caught her wings CLIPPED by her own anchor —
    "figure at noctua clips 46 pixels of its own content", eight times over
    — because a flying owl is wider than a one-cell piece: the piece grew a
    span and a bare figure no longer clips at all, since a figure with no
    box has nothing to clip to. The provenance closed the next
    message: "Owl" by kenchoo on Sketchfab (skfb.ly/pwnos), **CC BY-NC 4.0**
    — and NonCommercial is a standing constraint, recorded in the
    attribution in as many words: the owl lives in the documentation site
    only, never in `dist/`, and the day anything documented here is sold
    she is the first thing that has to go.

74. **The owl was a sticker.** Turn the scene and she went edge-on: the
    canvas is a flat plane standing in a 3D space, and a plane seen from the
    side is a paper cutout, which the reader photographed. The classic
    answer, both halves: the nest is a BILLBOARD now — it counter-rotates
    the scene's camera (inverse composition, yaw then pitch, from the same
    `--nd-yaw`/`--nd-pitch` the whole scene draws with) so the window always
    faces the reader — and the renderer turns the MODEL inside by the same
    angles, directly and with no ease, because a lagging model would wobble
    against an instant counter-rotation. What shows in the ever-facing
    window is therefore the owl's actual side: orbit the wild scene and she
    banks past in profile. Both owls got the treatment, and the gaze stamp
    moved from the bird's total rotation to the GAZE alone — the scene's
    resting tilt would otherwise read as her looking down before anybody
    moved anything.

75. **The wingtips were shaved, the bird was soft, and neither fix may know
    it is an owl.** Three reports against the scanned model, closed with the
    reader's own constraint: the nest must show ANY `.glb` perfectly, not
    mould itself to this one. The clipping had two layers. The visible one —
    a fit margin so tight the pose envelope lay on the frame — hid the real
    one: the envelope was wrong, because `Box3.expandByObject` reads
    geometry bounds and a skinned mesh's geometry is its BIND POSE — the
    bones deform it on the GPU, so every sampled pose measured the same
    rest box and the upstroke left the frame anyway. The bounds are now
    taken BY VERTEX with the skinning applied (`getVertexPosition`), the
    union over twenty-four sampled poses of the first clip — a walk cycle
    that leaves its own rest box entirely is measured all the same — and
    with the envelope exact, a 0.95 margin is safe and she flies larger.
    The softness was a pixel-ratio cap of two and unasked-for texture
    sampling: the cap rose to three and every material map takes the
    hardware's best anisotropy, whatever maps a model brings. The audit
    closed the loop once more — the taller nest clipped nine pixels against
    the figure's fixed card height, so a bare figure's height is its
    content's now, a box a bare figure never had.

76. **The model stage, redone — because the first version was owl-shaped
    against an explicit instruction.** The reader had said the feature must
    not mould itself to the owl, and it did anyway: the loader lived in
    `owl.js`, was dispatched by `data-nd-owl="real"`, sized by an
    owl-specific rule, its file hardcoded. The reader called it dishonest,
    and the record here agrees rather than softens it. The redo is
    `model.js`: `[data-nd-model="<path>"]` makes any element a viewer for
    any binary glTF — the FILE is the input, five optional
    `data-nd-model-*` parameters (fit, clip, gaze, fov, ratio) are the
    knobs, and the defaults are chosen to carry most models unconfigured,
    because this module is the seed of an upload feature that will be handed
    arbitrary files. Everything genuinely general moved in whole — skinned
    per-vertex pose-envelope bounds, frustum fit at the element's own
    aspect, max anisotropy on whatever maps a model brings, real-time
    clip playback, the billboard bearing inside a `Lineage` scene, the
    graceful `[data-nd-model-flat]` fallback — and `owl.js` went back to
    being only the modelled owl. A spec now holds the instruction itself:
    the stage carries no owl attribute, the served `model.js` contains the
    string "owl" nowhere, the example sets not one parameter, and the model
    still hatches. The audit's clip rule learnt one honest refinement on
    the way: a box whose overflow is VISIBLE cuts nothing, and
    `scrollHeight` reports overhang whether or not anything is lost.

77. **The specificity sweep.** After the model stage, the reader asked for
    the same audit over every Lineage/LineageFigure layer: nothing may
    depend on anything specific. Three passes — engines/emitters/runtime,
    the whole lineage stylesheet block, specs/i18n/tests/site plumbing —
    found the CODE clean of example content and the seams dirty. What was
    redone: **the contract now names every behavior-carrying word** — the
    `kind` vocabulary that places (`union|partner|marriage|donor` share a
    rank, `link|lateral|note` rank nothing, all else descends), the words
    that only dress, the synthesized `origin` and `elided` with the
    reserved `-elided` id suffix, the bond attributes `dash`/`route`/`note`
    the list had omitted, and `data-nd-piece` in `[state]`, which runtime
    and stylesheet both key on. **The one magic word that resized geometry
    died**: `kind=unknown` no longer widens a figure — `span` learnt halves
    (`span=1.5`, the model's own unit) and the examples say their width
    outright. **Two contract sentences were corrected** to what the engine
    does: multi-row bonds are not "always taut" (unions go around, rank
    gaps elbow through), and a figure's `action` is not the runtime's hook
    (`swaps` is; a swap switch now renders `aria-pressed="false"` from the
    start). **The component stopped consulting the docs**: the container
    gate inverted (samples opt out in their own section), the specimen's
    elided rules moved home, and the specimen's two pieces of stagecraft
    got names that say so — `data-nd-sample-tip`, `data-nd-sample-branch`
    — plus five plies to match the real emission and a comment naming the
    fakes. **The contract prose lost its cast**: no more "Doe family
    server", no Ada/Bob/Eve in the a11y notes, no "doe finds Doe".
    **Tests moved home**: the owl-gaze and model-stage specs now live in
    `examples.spec.js` — they test what the site builds around the
    component and never ships — and the camera-zoom spec picks any two
    figures on different rows instead of two names from the demo family.

78. **Two regressions of the sweep's own, and the trim became the author's.**
    The reader's screenshot showed all three. The bare figure's border came
    back because the sweep's `tone=neutral` rule — added so neutral would
    "do something" — sat after the bare rule and `tone` DEFAULTS to neutral,
    so it repainted a border onto every figure on the site, including the
    ones whose shape had deliberately melted it. The rule is deleted and
    the lesson written where it stood: neutral IS the resting border, and a
    rule for the default value is a rule on everything. The wings clipped
    again because the pose envelope samples the clip at intervals and the
    flap's extreme lay between samples — 64 samples now, and the fitted
    envelope wears 3% of air, because a pose between samples can exceed the
    union by a feather and a feather at the frame's edge is a cut wing. And
    the line ending behind the name exposed the fixed trim's blind spot: a
    tall bare figure's centre is far above its text, and 0.35 cells from
    the centre is still inside the words. `reach=<cells>` on a PIECE is the
    knob — the same digits-and-a-dot grammar as `span`, halves legal, the
    piece's own distance winning over the scene mode, working in `touch`
    scenes too, stamped as `data-nd-reach` for consumers and for the audit,
    whose landing rule now gives each solid its own tolerance — and finding
    a slotted figure's piece taught it one more mechanism: a figure at an
    anchor is not nested in its piece, it names it. Blessed case extended:
    one figure carries `reach=0.5` against the scene's 0.35, and a reach as
    long as the run itself is refused rather than let a line vanish.

79. **The knob was too coarse and the sampler too even.** `reach` stored
    half-cells like everything structural, so the example's `reach=0.8`
    ROUNDED to a whole cell and the line stood a wilderness below the name
    — a distance a reader tunes against a word's descenders needs tenths,
    and tenths of a cell it now stores (the structural grid keeps its
    halves; this is a distance, not a position). And the pose envelope
    sampled the clip a fixed 64 times whatever its length, so a fast
    wingbeat inside a long clip could fall between samples — it samples by
    TIME now, thirty per second of clip, bounded [48, 240], and the
    downstroke's tips clear the frame with air across the whole cycle.

80. **The fit was honest only at rest.** The envelope was fitted to the
    frame axis by axis — width to width, height to height — but the stage
    is a billboard: the scene's orbit and the gaze turn the MODEL, to any
    yaw at all, and a wingspan fitted facing front swings past the frame
    the moment it points anywhere else. The reader photographed the cut
    wing at an orbited angle, the third wing report, each a different
    cause. The one bound that does not care which way the model faces is
    the pose envelope's bounding SPHERE, and the fit is now the exact
    sphere-in-frustum condition — a frustum plane at `tan α = half·fit/d`
    clears a centred sphere of radius `s·r` precisely when `s·r ≤ d·sin α`
    — so no orbit, no gaze, and no pose can reach the edge, perspective
    included. The exactness matters: the first sphere fit corrected for
    nearness with the corner point (`d − s·r`) and gave up a fifth of the
    size the tangent plane allows. The honest cost stays: a sphere is
    larger than any single pose, so the resting model stands smaller than
    an axis-fitted one — that is the price of turning freely, paid in the
    default so every model is safe and an author who pins a stage outside
    any scene can raise `fit` toward 1. The spec watches ninety frames of
    flight at a hard orbit and reads a two-pixel ring at the canvas edge;
    with the axis fit restored it fails, with the sphere it reads nothing.

81. **The sphere was the right guarantee and the wrong size.** Fitting the
    whole envelope's sphere inside the element made the model SMALL — the
    reader had asked for larger than even the axis fit showed, and the
    shrunken picture also PIXELATED under the scene's zoom, because fewer
    canvas pixels carried it and the zoom is a magnification of the canvas
    as a picture. The two dials are now separate. The world scale keeps the
    exact sphere-in-frustum maximum — the no-cut guarantee, untouched. The
    SIZE comes from the canvas: the element's box is a window, not a cage,
    and the canvas overdraws it (`--nd-model-grow`, stamped for the
    stylesheet, the stage's `overflow: hidden` gone — it was a cut waiting
    to happen) by exactly the factor that shows the performance at `fit` of
    the element's own box, `fit` now `1.15` and legitimately past one, so
    the model stands LARGER than the axis fit ever showed while its turned
    extremes land on transparent overdraw. Measured from layout boxes, not
    client rects — a transformed rect would scale the buffer by the zoom a
    second time — because the resolution now follows the zoom once, on
    purpose: the canvas re-renders at the magnification it is shown at, up
    to `ratio` (now a resolution cap, default 4), checked on a dead band so
    a wheel's worth of notches costs one reallocation. Three specs hold the
    round: the edge ring stays empty at a hard orbit, a wingbeat must
    exceed the element's own box, and closing the camera in must add
    pixels; the middle one and the last both fail against the sphere-only
    build, restored and photographed failing.

82. **The double, and the figure that takes anything.** The resting
    presentation stays the default for every model; `data-nd-model-size` is
    the new dial on the stage — a multiplier on the fitted default, `2` in
    the new queen example — and it moves only the presentation: the world
    scale keeps the sphere-in-frustum guarantee and the overdraw grows
    proportionally (its bound scales with the ask). The queen example also
    puts a DESCRIPTION under the name, and that needed no feature at all:
    the figure's children are anything, in order, so the description is one
    more `Text` handed over — the contract already said so. What the double
    exposed was the field's edge: a canvas taller than the rows painted
    past the scene, over the page above it. The stage now measures its own
    poke and asks the scene to BREATHE by that much — stamped on the
    component's ROOT, not the field, because air inside the field shifts
    the space within its own tilted projection and the last row left
    through the bottom while the first was being rescued; measured in
    accumulating rounds, because the resting tilt eases in AFTER the mount
    (a single measure took 6px of air and the tilt then raised the true
    poke past 160) and a residual written over an earlier round's total
    thrashed; stopped at the reader's first gesture, because a measure
    taken mid-orbit would ask for swung air the padding never gives back.
    Two specs hold the round — the doubled overdraw with the canvas inside
    the root, failed against the sizeless build, and the description
    standing below the name — and the sentence itself is a message in both
    locales, with the owl's CC BY-NC credit carried into the new note.

83. **The air was outside the window.** The reader photographed the queen
    with a dead band above and the model cut to a sliver — engaged, orbited
    back to flat. The engaged camera CLIPS at the field (`overflow: clip`),
    and [82]'s air was worn on the ROOT, outside that clip: at rest the
    wings painted over the root's padding and looked held; one gesture
    later the window cut them at its edge and the padding stood empty
    above. The air now goes INTO the drawing — the grid origin
    (`--nd-oy`) moves down by it and the space grows by it, so the rows
    and the window move together and the field, the very box the camera
    clips to, holds the whole canvas in every mode; the runtime needs no
    teaching because it measures the space and the field live. That made
    the drawing taller than the DEFAULT window: `height` defaults to
    `tall`, a cap of five pitches, and the family slid below the fold —
    the contract's own words ("the size of the window, never the size of
    the graph") name the fix, `height: Full` on the example, a scene whose
    point is one large figure showing all of itself. The breathe spec
    gained the photographed pose: engage, orbit back to flat, and the
    engaged window must still hold the canvas — restored to the [82]
    build, it fails; here it holds.

84. **The page was a wall, and now it is told.** The reader called /lineage
    confused: notes up to 431 characters, twenty-nine frames in declaration
    order reading as repetition (`PAIR` under six scenes, `SLOTS` under
    three examples), and `LineageFigure` — the half a consumer touches
    first — standing behind the scene's whole reference. The examples are
    CONSOLIDATED: Lineage 23 entries to 17 (adoption+donor, partners+
    spanning, thickness+line, picked+camera, and aviary+placed+four each
    became one multi-scene frame, the mechanism the route pair already
    used), LineageFigure 6 to 4 (link+server became one "what a figure
    element is" frame; figure-note died outright — `marked` already showed
    the same card at the same mark). Every prop token still appears in the
    captions, which `the_examples_reach_every_prop` now actually holds
    (`height` had silently gone missing from every caption). The page is a
    HAND-BUILT body (`LineagePage`, the ThemeScope/Icons precedent):
    lead → jump nav → the scene → THE FIGURE while the first scene is
    still on screen → themed sections (bonds and unions · the space ·
    filter, pick, camera · scenes that change · placement said outright ·
    models in the scene) → all fourteen derived variant grids → both
    contracts at the end. The prose was cut to one claim per note (the
    two model notes keep the CC BY-NC owl credit), both guidances lost
    their build archaeology, and the lead, the section summary and the
    guidance no longer say the same sentence three times in two inches —
    the summaries are simply not printed on this page. The wild and queen
    graphs now share their roost through one macro, so the two can never
    drift apart. Deliberately NOT done: compacting the shared variant-grid
    CSS — the gain was centimetres and the risk crossed every component
    page. Earlier the same day the user had decided Lineage and
    LineageFigure stay TWO components (the merge routes all degrade
    something doctrinal: no-script placement, or compile-checked prop
    parity, or a one-use spec concept); this page is the answer to the
    part of that complaint that was real — they now READ as one system.
    Orphan keys swept in both locales (`married`, `here`, and every
    merged-away note).

85. **The toy owl retired.** The reader pointed at the round figure with
    the modelled owl — spheres and cones from `owl.js`, the site's first
    attempt at a bird — and asked for it gone. The real scanned model on
    the stage (`model.js`) superseded it long ago, so the example did not
    fall alone: `owl.js`, its copy step in `main.rs`, its `shell.js`
    import, the `.nd-owl` rules, the `ORIGIN` graph it alone used and the
    gaze spec all left with it. What its caption carried moved rather than
    died: the figure's `parallax` now leans the basic example's accent
    card (the scene under it declares `parallax: true`), `label` sits on
    the link example's caption where the code always set it, and the
    link-and-swap frame split back into its two claims so the figure
    keeps four written examples, which the example-count test demands.
    The founders scene — the same round origin, worn by a portrait
    instead of a toy — already shows everything the dead example showed.

86. **`DateInput`, the user's pick under §5.7 — and the system's first free
    text boxes.** One date as the three numbers a reader already knows,
    feature-complete per §9.12. The decisive calls, each
    against a house precedent: the root is its own `<fieldset>` (TagPicker's
    reason — the error slot the script fills must be derived by the
    component that carries the hook); the reader's field order is DETECTED
    (`Intl.DateTimeFormat` over the page's language then the UA's, §9.9's
    own route) and applied by MOVING the boxes in the document before
    anything has focus — never CSS `order`, so eyes, Tab, screen reader and
    the posted pairs walk one row; a DECLARED order is served that way at
    SSR and never second-guessed; no combobox anywhere — the suggestions
    hang off one script-built button, quick picks are inert `Time` children
    the script clones into a focused listbox, and the calendar is a month
    grid drawn from the locale's own formatter (the one vocabulary that is
    not a prop, because month names are the locale's); no auto-advance,
    stepping on the arrows instead, paste filling all
    three by the order in force; validation speaks only the caller's
    sentences (`{field}`/`{min}`/`{max}` holes, bounds worded by Intl,
    never ISO), on submit and on leaving a complete field — and a
    caller-rendered error is never the script's to clear. With no script:
    a question, three labelled boxes, three posted values, the browser's
    own `required`, and not one ARIA attribute promising what nothing
    keeps. Bugs the build caught on the way: the emitter served declared
    orders day-first (fixed by laying the items out by `resolved` in both
    halves); the calendar's month heading collided with the month box's id;
    rebuilding the grid under focus fired a null-relatedTarget `focusout`
    that closed the panel (the handler now judges where focus LANDS, one
    frame later); and a three-box form with no submit control never submits
    implicitly (the example gained its Continue button). 19 specs of its
    own — FormData posting, order congruence, leap years, worded bounds,
    the script-blocked honesty block, locale sweep — and the page enrolled
    in every generic sweep. Suite 786; the calendar icon vendored from
    Lucide at the pinned tag.

87. **The Continue button breathed.** It sat flush against the fieldset —
    the birth example's form was the one form on the site not wearing
    `.nd-example-form`, the column every other example form breathes with.
    Classed; a spec now measures the gap between the field and its submit
    control, and with the class removed it fails.

88. **The table deleted, and rebuilt around a column that is a real thing.**
    A developer asked for screenshots of `Table` and the first one showed the
    defect: at 420 pixels with `layout: Stack`, the caption *The spacing scale*
    wrapped into three lines inside a sixty-pixel column. Measured on the built
    page — `tableWidth: 354 · captionWidth: 60 · tableDisplay: block ·
    captionDisplay: table-caption`. The cause is the whole strategy of the
    stacked layout: `display: block` on the `<table>` leaves a `<caption>` a
    `table-caption` box with no table to be the caption *of*, so it shrinks to
    its own `min-content`. The parts survived because each restates its ARIA
    role in the markup; the caption was the one part nobody had restated a
    display for.

    The developer's answer was to delete the component and build a new one. The
    limitation underneath it was that **a column was not a thing**: width,
    alignment and sortability were declared on every heading cell and re-derived
    by a script from cell positions, which is how `10` sorted before `9` and how
    a heading spanning two columns sorted the wrong one. `TableColumn` is now
    one real `<colgroup>` per column, declared before the rows, and it is where
    the width, the comparator, the filter, the resize, the hiding and the
    pinning all live. `role="grid"` is **derived** from what those columns and
    rows allow, so it cannot be set on a table with nothing to drive; the narrow
    layout is a **container query on the scroller**, so a props table inside a
    narrow example frame stacks on a wide window; and the behaviour ships in
    `dist/table.js` rather than living in `shell.js`.

    Four defects the rebuild found in itself, each fixed where it was caused:
    clearing a sort left the rows where the sort had put them, because the
    document order was read after the first move rather than stamped before it;
    a filter that matched nothing left the previous total standing under an
    empty table, which is the exact lie `aggregate` exists to prevent, so a sum
    of nothing is now zero; the columns menu borrowed `.nd-panel` — the site
    header's sheet, whose material is applied in the markup — and drew as a
    transparent box with the table showing through; and a row that draws its own
    choice cell shifted every `<colgroup>` one column to the left, so a `fit`
    width meant for a code squeezed the column beside it into a letter per line.
    The last one is the interesting one: the caller should declare a leading
    `TableColumn` for the control, and the runtime inserts the missing groups
    where they did not, because the failure is silent and reads as a broken
    width rather than as a missing declaration. 25 specs of its own, and
    `pagination.spec.js` exists because the one pagination test was living in
    the table's file.

89. **The table's examples were indefensible, and the audit found more than was
    reported.** A developer read the screenshots and said the examples were
    bad: props shown with identical results, props too similar to tell apart,
    and data that was the design system's own spacing scale rather than
    anything from the world. Every complaint was right, and an audit added
    three nobody had seen.

    **Five tables rendered byte-identically** apart from which rows they
    listed — `TableSection { kind: Body }` emitted character-for-character what
    a bare `TableSection` emitted in the other twenty-nine examples, and
    `TableHeaderCell { scope: Row }` was applied by the shared row macro
    everywhere, so the example claiming to show it was a plain table. One frame
    promised `align`, `wrap` and `tone` and delivered **none**: `nowrap` on the
    word *Step* in a `fit` column cannot wrap, and **`data-nd-tone` on a cell or
    a heading had no rule in the stylesheet at all** — emitted into the markup
    of every cell and read by nothing. **Thirty of thirty** examples used the
    spacing scale under one caption, and three of them were wrong:
    `--noctua-space-5` does not exist, `--noctua-space-section` is an alias
    rather than a step, and the note claimed *every step* above a table showing
    three of thirteen. The page opened with the whole of `Pagination` because a
    collection renders in **filename byte order** and *p* precedes *t*; the same
    accident put `TableCell` before `TableColumn`.

    What the rebuild changed, beyond the examples: `tone` became a **name**
    rather than a list of four, from a `dist/tones.css` generated out of the
    colour system's 349 families — the component layer had been using nine.
    `Meter` and `Progress` arrived as the two real elements a bar in a cell
    wants, and `Info` as the small `i` that opens on a press. The per-column
    filter boxes became one search behind a magnifying glass; a column may
    declare thresholds and three tones so a **status comes from the value**; a
    `<tfoot>` may carry subtotal, tax and total at three weights; `<tbody>`
    groups fold; `more_src` fetches the next rows as `<tr>` markup; and the
    export menu writes CSV, JSON, the clipboard and the printer.

    Defects the rebuild found in itself, each caught by looking rather than by a
    test: the empty row showed above a table that had rows, because it was only
    ever hidden by a search; a currency total came back as a bare `6,820` under
    a column of `R$ 4.900,00`, so an aggregate now keeps the shape **and the
    separators** the server rendered and replaces only the digits; a `fit`
    column wrapped *3 Mar* onto two lines, which is narrower than the content
    allows and not what the word means; a group's heading sat in the first
    column with the rest of the row blank, because a lone cell in a row takes
    one column and not the width; a group's subtotal counted its own heading and
    itself; and the `<colgroup>` never carried the threshold attributes at all,
    because `TableColumn` is an authored body and the new props were added to
    the contract alone.

    Two that cost the most to find. **A `ResizeObserver` that wrote back what it
    measured**: `paintPins` set a custom property from a `getBoundingClientRect`
    every time it ran, the observer saw the style change, and the pair never
    settled — a core spent on every page with a table, and every element on it
    permanently "unstable" to anything waiting for one. And **a table left to
    shrink to its content inside a flex column** became a box under the
    breakpoint, so the container query — the thing that makes the narrow layout
    right inside a narrow frame — stacked a table nobody asked to stack and took
    its heading row and every sort control with it. The first showed up as a
    click that timed out; the second as a sort control that was there in the DOM
    and could not be pressed.

90. **A second reading of the table, and the seven things in it.** The developer
    went through the page again and reported what was still wrong. Every item
    below is one of theirs.

    **Hiding a column took its neighbour with it.** A hidden column was hidden
    twice — `visibility: collapse` on the `<colgroup>` *and* the `hidden`
    attribute on every cell — and automatic table layout miscounted the pair:
    hiding *Status* also took *Amount*, hiding *Due* also took *Status*. The
    cells are the mechanism now and the attribute is only the state a test
    reads.

    **Resizing destroyed the table.** A `<col>` width used to switch it to
    `table-layout: fixed`, where a column with no declared width takes an equal
    share of what is left — so the first drag threw every other column to the
    same width, values slid out from under their headings and printed through
    each other. Under automatic layout a width is a **preference**: the dragged
    column grows, the rest share the remainder, and nothing is ever narrower
    than its own content.

    **Every value sat one column right of the words naming it**, wherever a row
    drew its own leading cell. A choice control and an expand control are cells
    the row adds; the heading row adds neither unless the caller remembered, and
    forgetting is silent. The runtime inserts the missing heading, the same
    repair it already made for the missing `<colgroup>`. A browser test now
    walks **every table on the page** and checks that each heading starts where
    its own cell starts — the class of defect a screenshot shows and nothing
    else does.

    **The variant grids were tables of bare digits, and three of them were
    blank.** The specimens are raw markup, so they had been written as numbers
    to get past the translation sweep — but `.nd-identifier` was always the
    right answer, and a build number and a duration are identifiers. `<colgroup>`
    specimens showed nothing at all because a column with no rows has nothing to
    paint; the wrapper gives them a row, and the column takes a background so
    the extent each width asks for is visible.

    Three CSS facts had to be found before a specimen could size itself:
    `container-type: inline-size` **is** `contain: inline-size`, so the
    scroller's intrinsic width was zero and every attempt to let the item grow
    did nothing; a scroll container contributes almost nothing to its parent's
    intrinsic width; and `.nd-table`'s `inline-size: 100%` contributes nothing
    either. All three are disabled inside a variant stage.

    **Props whose values look identical no longer get a grid.** `variants =
    false` on a comparator, a band, a scope, a total a script computes, a
    `width` that means *fills its container* and a `height` that caps a box —
    none of them can be drawn in a still picture of one component, and a grid of
    identical pictures under different captions reads as a component whose props
    do nothing. And where two props wanted opposite things from one specimen —
    `align` needs a value short enough to have room to move in, `wrap` needs one
    long enough to break — a prop may now carry its own.

    **The paging example is gone.** It sat in a stack that squeezed it, and its
    links went to fragments that did nothing, so it demonstrated a control that
    did not work. `Pagination` has a page of its own; the table page keeps the
    sort example that was buried beside it.

91. **A third reading, and the one that named a rule rather than a bug.** The
    developer's sharpest note this round was not about a defect at all:
    `TableRow.emphasis` had the values `subtotal` and `total`, **which puts
    accounting inside a component that has nothing to do with accounting**. A
    table of servers has no subtotal, and a caller who wanted to close a group
    of them had nothing to reach for. They are `subtle` and `strong` now —
    weights, not meanings: *where does this section end* and *which line is the
    answer*. That is a subtotal and a total in a ledger, a per-region count
    above a fleet-wide one on a deployments board, a baseline above the run
    being compared to it. The dedicated example was rebuilt on builds and
    durations with no money in it, to prove the point rather than assert it. On
    the same instruction every other enum in the family was re-read: `align`,
    `wrap`, `rules`, `density`, `width`, `height`, `layout`, `select`, `scope`,
    `sort_as`, `aggregate`, `aggregate_over` and `kind` are all presentational
    or structural, and none of them names a domain.

    **`numeric` did nothing, on the whole site, at every width.** The stacked
    layout flattens every cell to `text-align: start`, and that selector is one
    class and two attributes deep — so it outranked `[data-nd-align]`
    everywhere, and the un-stacking container query restored `display`,
    `padding` and `position` but never alignment. It stayed invisible because a
    heading is a flex column that ends its own items: the *words* moved to the
    end and the *values under them* did not, so it read as a column that had
    come loose from its name rather than as an alignment that was never applied.
    A browser test now sweeps every aligned cell on the page at two widths.

    **A two-level heading was read as a row two columns short.** Both the
    runtime's repair from entry 90 and the test that guards it counted *cells*
    where they meant *columns*, so one heading spanning two columns beside one
    spanning two rows looked like a head missing a cell: a blank `<th>` was
    inserted, and every word moved one column right of the values it named.
    Both count `colSpan` now. The example itself was also wrong in a way no test
    can see — *Result* spanned the environment and the duration, which are not
    two halves of a result. A spanning heading has to be **true** or it teaches
    a reader to ignore the top level; it is *Duration* over *Waiting* and
    *Running*.

    **And two more identical-picture props, both from the same cause as the
    hidden column: one thing declared twice.** Every column heading was
    `white-space: nowrap` in the stylesheet *and* `wrap` by default in the
    contract, so `wrap` and `nowrap` drew one picture twice; the default moved
    into the contract, where a reader can see it. And `truncate` acts on
    `.nd-table-value` — which the specimens did not have, so it wrapped like
    `wrap`. Specimen cells are ruled on all four sides now, because `start` and
    `end` are the same picture until the reader can see where the column stops.

92. **The aurora is gone, and the dots can be seen.** Two instructions, and the
    first one took more out of the system than it put in.

    `glow` was two soft radial washes of the accent behind the page. It was the
    only ambient effect that **moved**, and the only one whose geometry was a
    share of its box rather than a tile — 60%×50% lobes, which are margins on a
    page and a wash over the content inside a 320px scope. That cost it a
    contrast ceiling of its own (`[effect.alpha] glow`), a blur token, a
    `drift` duration that had to be kept out of `[motion.duration]` because an
    infinite animation collapsed to `0ms` restarts every frame rather than
    stopping, a `@keyframes`, a `prefers-reduced-motion` stop, a probe test
    built specially to measure it at the narrowest screen, and a background
    layer in the one rule that paints every page and every scope. All of it
    went. **Nothing a page is painted with moves any more**, and the emitter
    test that used to check the drift was not driven by a collapsing token now
    asserts the stronger thing: no `animation:` and no `@keyframes` anywhere in
    the appearance sheet, so the next person to animate a background has to
    come back and answer the reduced-motion question on purpose.

    The dots were invisible. Not faint — **invisible**: at the shared 4%
    ceiling, on a light page, the specimen crop is a blank white rectangle. The
    ceiling is per-pixel, and that is fair to `mesh`, `checks` and `diagonals`,
    which cover most of their tile; `dots` covers about a fortieth of its own,
    in specks a few pixels across with nothing touching them, so the same number
    buys a fortieth of the texture. `spec/tokens.toml` declares a second ceiling
    — `dots = 10` — and the dot grew from 9% of its tile to 14%.

    **And the test that was supposed to hold those ceilings was measuring
    furniture.** It clipped a 320×200 rectangle out of a body screenshot, but an
    element screenshot ignores `clip` and returns the whole element — so it was
    reading the entire page, reporting a baseline spread of 68/255 on what
    should have been bare background and then a *negative* contribution for
    every pattern, which passes an upper bound without ever seeing the ink. It
    takes a page screenshot over a region of real bare page now, asserts the
    baseline is zero as proof the region is bare, and holds each pattern to its
    own declared ceiling. Measured: dots 22, mesh 16–18, checks and diagonals 9,
    against 255.

93. **A specimen was not the component, and half the site's props were
    documented by a picture of nothing.** The developer reported four grids on
    `/button/` — `tone`, `justify`, `submit`, and the `submit`/`name`/`value`
    example — where every value drew the same thing. None of the causes were in
    `Button`.

    **A specimen wrote exactly one attribute.** Every tone in this system is
    painted compound — `.nd-button[data-nd-variant="solid"][data-nd-tone="danger"]`
    — so a specimen carrying `data-nd-tone` and nothing else matched no rule at
    all and fell through to the browser's own grey button, four times. A
    specimen now carries **every enum prop at its declared default** and then the
    prop under test. Bools are deliberately excluded: `pressed`, `expanded` and
    `current` are absent when false *on purpose*, and a specimen writing
    `aria-pressed="false"` would state the opposite of the contract it
    illustrates.

    **A void element cannot hold its own picture**, and half the form controls
    are one. `<input class="nd-text-input">figure</input>` is not what the parser
    does with that — the figure lands *beside* the input, so every rule shaped
    `.nd-text-input[data-nd-size="small"] .nd-text-input-box` matched nothing and
    six controls documented `size` with three identical boxes. `<meter>` and
    `<progress>` are the same story for a different reason: they are replaced,
    and an engine that supports them never renders the fallback content. Both
    kinds become a `<span>` carrying the same class and attributes.

    **`variants = false` was enforced in one of three call sites.** Three places
    render a variant grid — a component page, a collection page and the scene —
    and only the collection page asked. So the declaration worked on `/tables/`
    and did nothing on `/button/`, where `submit` went on drawing three
    identical buttons after the contract had said not to. One `fn drawn(prop)`
    now answers it for all three.

    **A prop's own specimen lost to the component's icon.** `Button.justify`
    declared a specimen and got the `check` glyph anyway, because the icon was
    resolved from `component.specimen` before the prop's was consulted — and one
    glyph has nothing to be pushed apart from, so `start` and `between` were one
    picture.

    **A variant stage had killed the table.** Disabling `container-type` (entry
    90) also disabled `@container nd-table (min-width: 30rem)`, so every table
    specimen was in the *stacked* form — where cells are blocks and rules between
    rows are removed. `density` and `rules` drew one picture per value because in
    that form neither does anything.

    The guard is `tests/browser/variants.spec.js`, which asks
    `lineage.spec.js`'s question — *no two values of one prop draw the same
    picture* — on **every** component page. It found **57 pairs across 18
    pages**. Its fingerprint had to learn two things on the way: `text-align`,
    padding and eleven more properties, without which four genuinely different
    values of `TableCell.align` reported as one; and the platform's own
    pseudo-elements, since `<meter>`, `<progress>` and a range `<input>` are
    painted almost entirely into `::-webkit-*` parts that `getComputedStyle(el)`
    cannot see — the test was blind exactly where the drawing is.

    What survived the repairs took `variants = false`, seventeen props whose
    values a lone specimen genuinely cannot draw: a segment order a script
    lays out, an input `kind` that is the same empty box five times, a
    presentation the script chooses, a carousel with no slides, a paging control
    with no pages, sizes that live on the surrounding `Field`. Each is
    documented by its page's real examples and by the prop table, which is
    strictly more than a row of identical pictures was.

    On the page itself: the `submit` example is a real form now — type in the
    field, and `reset` empties it, `submit` sends it, `button` waits for a
    script — `action` moved into the `menu` example, which is the button a
    script actually has to find, and `icon_end`'s caption said `ChevronDown`
    while drawing `ExternalLink`.

94. **The menu opened nothing, so the menu was built.** The developer pressed
    the button in the `menu` example and nothing happened. It had been that way
    since `Button` shipped: `menu: true` wrote `aria-haspopup="menu"`, drew a
    chevron and **announced a menu to a screen reader**, while `aria-controls`
    pointed at a hidden paragraph and nothing in the system could open it. There
    was no component for *a button that opens something living elsewhere in the
    document* — `Disclosure` is `<details>`, whose panel is its own child, and
    its guidance sent readers to `IconButton`, deleted months ago.

    `Menu` and `MenuItem` are that component, and `dist/menu.js` is the tenth
    shipped runtime file. It is `info.js`'s shape almost line for line — a
    control, a floating panel, `anchor.js` for the seventh time — plus the
    keyboard a menu owes and an ordinary button does not: Down and Up wrap
    through the items on a roving `tabindex`, Home and End reach the ends, a
    printable key jumps and a run of them spells a word, Enter and Space run an
    item and close, Escape closes and returns the focus, Tab leaves and closes.
    One menu is open at a time.

    Three decisions worth the words. **`aria-disabled`, not `disabled`, on an
    item** — the opposite of `Button`, and deliberately: a disabled `<button>`
    leaves the tab order, which in a menu means the arrows skip it, and an item
    that is skipped is an action a reader concludes does not exist. **The
    keyboard opens onto an item and the pointer does not** — a reader who
    pressed a key is walking, and one who clicked is reading. **`Menu.name` is
    the id**, not a derivation of it, because the caller writes `controls` on one
    component and `name` on the other and a derivation they cannot see is one
    they will get wrong.

    Four things it turned up on the way. `aria-expanded="false"` **is true on a
    menu button** — a control that opens something opens something whether or not
    it is open right now, so the absence of the attribute is the lie and
    `Button`'s otherwise-correct *write it only when true* rule needed the
    exception. The docs site's own hamburger held `.nd-menu`, a class that now
    belongs to a component, and would have taken a floating panel's rules. A
    menu specimen in a variant grid is `position: fixed` with nothing to anchor
    to, so three of them stacked in the corner of the window until the stage
    un-floated them. And `MenuItem { icon: "x" }` drew **nothing at all**: the
    file is `x.svg` and the icon's own name is `close`, and a `text` icon prop
    ends in `IconName::parse`, which answers `None` in silence — there is a test
    now that reads every icon an example names.

    **And the page's prose was read whole, in both languages.** One sentence was
    false — `example.button.pressed` ended *"this system ships no runtime at
    all"* while nine files shipped — and the rest were rewritten to one rule:
    **one sentence, one claim, every claim checkable from the page it is on.**
    Out went an aside about the pseudo-locale being in the test matrix, a note
    opening with *"It leaves the tab order too"* whose *it* and *too* pointed at
    nothing, a *"preset-driven bevel would be a rectangle"* that took three
    readings, and a `menu` doc promising a menu nothing could open.

95. **One page shape, a model at the top of each, and the furniture cut in
    half.** The developer's report was four sentences: the components carry
    visual errors, the pages disagree with each other, the texts are long and
    badly written, and nothing opens with a model of what it documents.

    **The numbers, measured on the built site at 1120px before the change.**
    `/tables/` was **56,919px** — sixty-three screens — and `/inline-text/`
    spent **2,961px on example toolbars against 2,105px of example**, so the
    apparatus was 141% of everything it framed; on `/button/`, 92%. Twenty-two
    identical six-control bars were the first thing a reader saw on a page
    about buttons. Guidance blocks ran to a median of 1,176 characters, a11y
    notes 909, and one of them shouted `IF IT NAVIGATES, GIVE IT href` in the
    middle of a paragraph.

    **There was no page shape.** `ComponentBody` had two arms that were the
    same hundred lines with every heading pushed down one; `CollectionPage`
    stacked whole references end to end; `LineagePage` told its own story; six
    foundation pages were each their own thing. A reader who learned where the
    props table lived on `/button/` learned nothing about `/tables/`. That is
    one `DocPage` now, and the two arms are a three-line `Depth` enum: a head
    that opens with a complete working example, an index of what is on the
    page, and **When to use it · Examples · Every variant · Reference** in that
    order on all fifty-six.

    **Fifty overviews.** One per page, keyed by page rather than by component,
    because `/tables/` opens with one whole table built from all six of its
    parts and that belongs to no single contract. The examples below each
    isolate one prop, which is the right shape for an example and the wrong
    shape for an introduction — they answer questions a reader cannot have yet.

    **Four folds, and one of them is why `Disclosure` grew a prop.** Props,
    States, Keyboard and Accessibility ship closed, each saying how much is
    behind it. Closed, a section's title is the only thing left of it, and a
    `<span>` for a title is a section that has vanished from the outline a
    screen reader navigates by — so `Disclosure { heading }` puts a real
    `h2`/`h3`/`h4` inside the `<summary>`, where the HTML spec allows exactly
    one. The counts go INSIDE the translated message through `fill`, never
    joined to the end of it, so a language that puts the number first can.

    **After: `/tables/` 1,873px, `/inline-text/` 1,794px, `/button/` 6,824px**,
    with the example toolbars down 45% and the prose on `/tables/` down from
    23,615 characters to 7,077.

    **And the components were audited page by page, by eye and by measurement.**
    The dominant finding was one root cause with fifteen faces: a specimen for
    any component that needs CHILDREN rendered the pangram placeholder, so
    `Segmented`'s six grids were sixteen copies of one line of text and
    `Toolbar`, `ToolbarGroup`, `List`, `Truncate` and `Prose` each drew one
    picture under several names. They declare a `specimen` now. Where a still
    picture genuinely cannot show a difference — `ToolbarGroup.align` is
    `margin-inline-start: auto` and needs siblings to push against,
    `Info.placement` is about a note that is not open, `Heading.level` is
    document structure — the contract says `variants = false` and says why.

    Six more, each with the test that would have caught it. **`--nd-tone-on`
    named the fill twice** for `neutral` and `muted`, so every component
    painting a solid tone drew its text in the colour of its own background: a
    neutral badge's count measured **1.00:1** against the pill it was written
    on, in both modes, on every page that used one. **`<textarea>` is a
    replaced element** — its content is the VALUE — so a specimen's markup was
    rendered to the reader as prose. **The icon gallery had collapsed to one
    column** inside a flex parent that shrink-wraps. **A vertical carousel
    pointed left and right.** **`Info`'s marker, the number steppers and the
    table's column choices were 16, 16 and 13 pixels** against a 24px floor.
    And **an example wider than its frame was cut off with nothing to drag**,
    which at 320px lost two thirds of every line of the typography specimen.

    The audit's own sweep is `tests/browser/audit.spec.js`: one page shape,
    every jump landing, every fold keeping its heading, and nothing clipped,
    overlapping, overflowing or under the touch floor at three widths. It has
    to be taught this system's deliberate techniques by name — the
    visually-hidden recipe, a transparent control over a drawn one, a specimen
    that is a picture rather than a control — because unfiltered it reported
    seven hundred findings on one page and not one of them was a defect.

96. **The prose was rewritten again, and this time the rule is a test.** The
    developer's verdict on the previous pass was one word, and they gave the
    target as a before and after they wrote themselves:

    | | |
    |---|---|
    | **Rejected** | `One choice out of many, or one thing a reader agrees to, at a size a thumb can find.` |
    | **Wanted** | `Let users select one or more options by using the checkboxes component.` |

    **The previous pass had diagnosed the problem as length.** It cut 1,059
    contract fields and 245 example notes by 60 to 65 per cent and kept the
    voice, which is why it did not help. Measured across the catalogue, the
    voice was 35 similes, 101 fields where a component *announced* or *refused*
    or *remembered*, 319 of 1,059 fields carrying an em dash, and project
    history addressed to a reader who never saw it.

    **So the rule stopped being a paragraph of instructions.**
    `crates/noctua-design-spec/src/prose.rs` runs under `cargo xtask check`
    over every contract field and both catalogues: no em dash, no semicolon, no
    sentence over twenty-five words, no shouted capitals outside an acronym
    list, a banned-phrase list per language, and a ceiling per kind of string.
    A written rule loses to a house voice; a failing test does not.

    **The rule that decides it came from the developer's own example.** What
    they rejected passes every other check on that page — no em dash, no shout,
    no banned phrase, eighty-four characters, one sentence. It is still wrong,
    because it is a noun phrase. So a summary may not open on an article or a
    quantifier, which leaves a verb. Both of their sentences are calibration
    tests: `the_register_the_developer_asked_for_passes` and
    `the_register_the_developer_rejected_does_not`.

    Nine agents rewrote 1,554 English strings and their Portuguese twins
    against that checker. It went from **2,517 violations to zero**. Four of
    its own rules were wrong and were found by the people using it: `for ever`
    matched inside `for every`, a backticked `` `tones::ALL` `` was read as
    shouting when it is an identifier, a page body was being measured against
    the ceiling meant for a one-line note, and a heuristic that called anything
    ending in a full stop a sentence was holding a paragraph of demonstration
    text to that same ceiling. The note list is read from
    `docs-site/src/examples.rs` now, which is the file that decides it.

    Two defects fixed on the way: `page.intro.body` was the literal string
    `Lorem ipsum`, shipping in the source locale, and `component.slider.summary`
    read *where where it sits*, faithfully translated as *onde onde*. And a
    summary is written twice — in the contract and in the catalogue — with
    nothing deriving one from the other, so `ListItem` had already drifted;
    `a_summary_says_the_same_thing_in_both_places` is why it cannot again.

97. **A name that was never a source, cited twenty-nine times.** The developer
    read the last entry and corrected it: GOV.UK had been mentioned once, in a
    prompt, as an example of the kind of thing to aim for. Nothing here was
    taken from them.

    It appeared in prop documentation a reader sees, in stylesheet comments, in
    contract files that generate the Rust doc comments and the React types, in
    three browser specs, in `NOCTUA.md`, in the workspace manual and in this
    log. Every one of them read as attribution, and attribution is a claim
    about a relationship. Making that claim on somebody else's behalf, for a
    relationship that does not exist, is worse than saying nothing.

    **The facts stayed and the name went.** A checkbox is forty pixels with a
    forty-four pixel target because WCAG 2.5.8 asks for twenty-four at AA and
    forty-four at AAA; that is a measurement and it is still written down. The
    error summary still lists everything wrong at the top of a form, and still
    takes the focus. What is gone is the sentence saying whose idea it was.

    `xtask/src/gates.rs::cited_sources` stops it coming back, and it is the one
    gate here that reads the RAW line. Every other rule is about code and reads
    a line with its comments taken off; most of these twenty-nine were comments,
    so a check that could not see a comment could not see the problem. It reaches
    source rather than prose files, which is stated where it is written.

    It is not about vendored work. `assets/icons/ATTRIBUTION.md`,
    `assets/vendor/`, the fonts and the palette carry real licences for real
    drawings and real code, and those are obligations rather than compliments.

98. **The Portuguese was read, and it was not a style problem.** The developer
    caught one word in conversation: *gate* had been translated as **portão**,
    which in Portuguese is a garden gate. That prompted a dedicated reading of
    all 2,297 Portuguese strings against their English source, which is the one
    thing no check here measures.

    **The measurable half came first.** Nine agents had written that Portuguese
    in one pass, each choosing its own vocabulary: *reader* came out as
    **leitor** 137 times, **quem lê** 73 and **a pessoa** 5; *tab stop* split
    evenly between **parada de tabulação** and **parada de tab**; *default*
    stayed in English 8 times. The glossary was settled before anybody edited,
    or the reading would have produced a tenth vocabulary.

    **What four readers found was meaning, not style.**

    - `component.pagination.prop.summary.doc` **said the opposite of its
      source**: the English says a reader is told the set did NOT change when a
      page turns, and the negation had gone.
    - `example.link.lead` and `example.link.text` are concatenated at render,
      and produced **"O resto está em a documentação"** on the page.
    - **`composer` was `compositor`** in eleven `TagPicker` strings, which is
      somebody who writes music.
    - **`sumário`** for an error summary is a table of contents; **`recuo`** for
      a font fallback is a paragraph indent, and it stated something false on
      the typography page; **`régua`** for a drawn rule is a measuring ruler;
      **`lavagem`** for a colour wash is laundry; **`linhas de comando`** for a
      company's reporting lines is the CLI; **`Embrulhe`** for *wrap* is what
      you do to a present.
    - **`settings.pattern.label` was `Padrão`**, so the appearance menu read
      *Padrão · Nenhum · Pontos · Malha* — which a Brazilian reads as
      *Default: None*, and the glossary reserves *padrão* for *default*.
    - **Missing negative concord**, which is ungrammatical: *envia nada*,
      *diz nada* rather than *não envia nada*.
    - `Tax (VAT 23%)` had become **IVA**, a European tax, on a table of reais.

    **One of them was this session's own work.** The Checkbox summary had been
    written as *Permita selecionar uma ou mais opções* — a calque, because
    *permitir* wants *permitir que alguém faça*. All 22 summaries opened four
    different ways and now open one: `Deixe a pessoa <infinitivo>`.

    **Two collisions were settled catalogue-wide.** `cursor` meant the slider's
    grip in fourteen strings and the text caret in three, with *Move o cursor
    para o começo* appearing under both; the grip is **alça** now, rewritten
    string by string because the gender changes with it. `região viva` for a
    live region became **região dinâmica**.

    A mechanical scan for known calques and false friends was run first and came
    back with nothing but substring false positives. That is the finding: this
    class of defect is not detectable by pattern, and the register checker,
    which passes, cannot see any of it.

99. **The controls came back out, onto the frame's trailing edge.** Entry 95
    folded every example's control bar behind a gear, because `/button/` was
    spending 1,416px on twenty-two permanent bars against 1,543px of example.
    The developer rejected it: pressing a gear to reveal a horizontal header is
    worse than the header was. What they asked for is a compact strip that is
    **always visible, down the right side of the component**.

    The room was there and was being wasted vertically. Measured at 1120px on
    `/button/`, the frame is 900px wide and the widest example ends 487px in —
    so every frame had between 413 and 815px of empty space beside it, and was
    paying for its controls in height, which is the scarce direction. `.nd-frame`
    is a two-column grid now, and the strip is an ordinary **static grid item**:
    `shell.spec.js`'s floating-surface predicate only considers `fixed`,
    `sticky` and `absolute`, so a rail in normal flow needed no allow-list entry
    and no justification paragraph.

    **Frames got shorter, not taller.** On `/button/`, shortest 221 → **213**,
    median 284 → **213**, tallest 368 → **270**. The rail is 96px wide and 212px
    tall, so it fits inside every frame on the page without setting the height.

    **Five stops cannot stack.** At 30px each the width control alone is 150px,
    and with four icons under it the rail reaches 278px — taller than half the
    frames it has to fit in. So the five stops became one `Button { menu }` that
    opens a `Menu`, and the trigger names the width in force. `menu.toml`'s own
    guidance — *a menu is actions, not values* — holds here: pressing *320px*
    resizes this stage, and the stage's width is the state rather than something
    the menu keeps. No item carries a tick. The measured readout moved with it,
    from a segmented detail line onto the `full` item's own, which is where the
    choice is made.

    `MenuItem` gained `value`, and the generator was not enough: `MenuItem` has
    an authored body, so `data-nd-value` had to be written by hand into both
    Dioxus arms and the React shared props. The contract generated the
    documentation; the DOM was unchanged until it was edited. **A prop on a
    custom component is two edits, and the build reports neither.**

    **The axis is not fixed, so the attribute could not be either.** Below the
    `sm` step the rail lies down into a row under the stage, which is what keeps
    a 320px stage at full width. `Toolbar` writes `aria-orientation` and
    `shell.js` reads it to decide which arrows walk the strip — so a value baked
    into the markup is wrong at one of the two widths, and announcing an axis a
    reader cannot walk is the thing this system refuses everywhere else. The
    markup ships `horizontal`, which is the ARIA default and is true below the
    step; `shell.js` upgrades it to `vertical` when the stylesheet stands the
    rail up, reading the breakpoint **off the token layer** rather than writing
    `30rem` a second time. With no script the layout is the narrow one and the
    attribute already matches it. Measured: at 320px `horizontal`/row, rail
    275×91 below the stage; at 1120px `vertical`/column, rail 96×212 beside it;
    no horizontal overflow at either.

    **The lying-down strip wraps, and a gap written to stop it is dead CSS.**
    At 320 the five controls need 303px against 286 of room: English fits on one
    row at 51px tall, Portuguese does not, because *Redefinir* and *Máx* are
    45px longer than *Reset* and *Max*, so it takes two rows and 93. Tightening
    the gap looked like it worked and had not run at all —
    `[data-nd-density="compact"]` on `.nd-toolbar` beats `.nd-frame-bar` on
    specificity, and the English row had been fitting on its own the whole time.
    **A declaration that loses on specificity is indistinguishable from one that
    works, if the thing you measure would have passed anyway.** The rule was
    removed rather than fought; wrapping is what keeps a control from being
    clipped at the width where clipping is likeliest.

    **Two tests had never run.** Both skip hidden elements — `shell.spec.js`'s
    floating-surface sweep on `offsetParent !== null`, `audit.spec.js`'s 320px
    sweep on `display: none` — and the bar lived inside a closed `<details>`.
    So the width stops, the theme summary and the glass, mode and reset buttons
    had never once been measured against 24×24, clipped text, or overflowing
    `.nd-main`. Making the rail always visible turned both on for the first
    time. **A test that skips what is hidden is a test that a fold silently
    switches off**, and entry 95 switched off two without noticing.

    Two more specs were asserting the old geometry rather than the claim.
    `frame.spec.js` measured the width control as a share of the bar's *width*,
    which is false by construction on a column and is now a share of its height;
    `glass.spec.js:562` asserted the bar was flush *above* the stage and now
    asserts it is beside it. `menu.spec.js` broke for a better reason: "the
    first menu on the page" started finding the frame's own width menu, so it
    scopes to `.nd-example-stage` — **a spec that says *the first* is a spec
    that will be wrong the day the page gains another one.**

100. **The rail was ragged, and two rules that looked applied had never run.**
    The developer looked at it and said the icons were against the left edge
    instead of centred like the controls that carry words, and that the strip
    could be smaller. Both were one cause: `.nd-toolbar[aria-orientation=
    "vertical"]` sets `align-items: stretch`, and a rule written against
    `.nd-frame-bar` alone is one class against a class **and** an attribute, so
    it loses. Every control was handed the rail's full width; the two with text
    filled it and looked centred, the four glyphs did not and sat hard against
    the leading edge.

    **The same trap had already eaten a `gap`** in the entry above, and the fix
    there was to delete the rule. That was the wrong lesson. The rule was right
    and the selector was weak — `.nd-toolbar.nd-frame-bar` is two classes and
    wins, and the stylesheet now settles the rail's axis, its spacing and its
    alignment in one place at both widths, with `shell.js` writing only the
    `aria-orientation` that says which axis is drawn.

    While fixing it, **two edits landed in the wrong rule entirely**: a
    scripted `replace` with a count of one matched the first
    `padding: var(--noctua-space-2)` in the file rather than the rail's, and
    moved a floating panel's padding instead. It was found by reading the
    **built** stylesheet for the value that should have changed and finding it
    unchanged. *Measure the property you edited, in the artefact that ships.*

    **`Reset` got a glyph, which is what made the strip compact.** It kept its
    word for one stage because `spec/icons.toml` declared nothing meaning *put
    it back*, and the file refuses to give an existing drawing a second meaning.
    `rotate-ccw` was vendored for it — an arrow that comes back round to where
    it started, which no other icon in the set is using. That removed the only
    79px control from a column of 32px ones.

    | measured on `/button/` | before the rail | fold replaced | now |
    |---|---|---|---|
    | rail, standing | — | 96 x 213 | **86 x 187** |
    | strip at 320, English | — | 51 | **43** |
    | strip at 320, Portuguese | — | 93 | **43** |
    | shortest frame | 221 | 213 | **189** |
    | median frame | 284 | 213 | **189** |

    **The rail cannot go below the width of its one text control**, and that
    control cannot become a glyph: `spec/components/menu.toml` refuses a menu
    whose items carry state, so the width in force is read back on the trigger
    rather than ticked in the list. Its label box has a floor, because `Max` and
    `320px` are 12px apart and without one the rail grew as the reader chose a
    stop — moving the stage's column while they were measuring it.

    Three tests were added, all written as **geometry rather than as
    properties**: every control's centre line is the rail's, the rail is a
    column with a gap and padding at or under one step, and naming any stop
    leaves the rail exactly as wide. Asserting `align-items` would have passed
    against a rule that never ran.

101. **The live number came out of the width menu.** The `Max` stop carried a
    second line showing the stage's measured width, written in by `shell.js` on
    every resize — the last remnant of the `Segmented` this control used to be,
    where the number sat beside the stops rather than inside one. The developer
    asked for it gone, and it should not have survived the move: a menu is a
    list of things a reader can choose, and a number that changes while nobody
    touches it is a readout wearing an option's clothes. It is the same
    objection `spec/components/menu.toml` already makes to an item carrying
    state, which is why no stop has a tick.

    Three things went with it, and that is the measure of how much was being
    spent on one line of text: `measure()`, the `ResizeObserver` watching every
    stage on the page, and the start-up pass that filled the readouts before the
    first paint. Nothing else called any of them. The `full` item no longer
    passes `detail` at all, so the empty `<span>` the script wrote into is not
    rendered either — `.nd-menu-item-detail` now appears zero times on the page,
    which is what the test asserts.

    **The width in force is still readable**, on the trigger, which is where the
    entry above says it has to be. What is gone is the number nobody asked for,
    on the one stop that could not name itself.

102. **The call got a header, and the padding had to move to give it one.** Every
    frame opens with the call that produced what is inside it, and the developer
    said it ran into the example: *isso acaba se misturando ao conteúdo*. It
    did. The caption and the example sat in one flex column, on one surface,
    twelve pixels apart, with nothing between them. It is a band now — flush
    with the stage's edges, on the same `surface-subtle` `.nd-frame-bar`
    carries, with a hairline under it, so the header and the rail read as one
    layer of chrome wrapped around the example.

    **No Rust, no contract, no regeneration.** `<figcaption class=
    "nd-example-source"`> was already the right element in the right place; what
    it lacked was a treatment. 506 of the site's 509 frames hold exactly one
    example, so styling the caption puts a header at the top of the frame for
    all but three of them. The three that hold several — `/spacing/`,
    `/typography/`, `/theme-scope/` — get one band per example, which is the
    only honest answer: a frame demonstrating four different things has no
    single call to put at its top.

    **The band could not be pulled out with a negative margin.**
    `.nd-frame-stage` is `overflow-x: auto`, and content overflowing a
    scroller's START edge is clipped and unreachable — the band would have come
    out cut off on the left at every width. So the stage's `padding` moved down
    one level onto `.nd-example`'s parts. That works because `.nd-glass-scroll`
    is `display: contents` at rest, which already makes each example a direct
    flex item of the stage.

    **And that is where it broke something.** Under the glass rig
    `.nd-glass-scroll` becomes a real box and carried `padding-inline` of its
    own, so the examples were suddenly inset 32px instead of 16 — enough to move
    the controls across the colour field. `glass.spec.js` caught it as *a
    frosted white control must stay white*, reporting a blue pane. The fix is
    that the scroller's inline padding goes to zero and the examples carry it
    there too, which restores the rig's geometry exactly.

    **The test it broke was passing on alignment, not on its claim.** Its
    subject is the control's OWN paint — a rule that filled an outline button
    because it matched every variant of the neutral tone. What shows THROUGH a
    frosted pane is a different question, and the rig exists precisely because
    the answer changes as an example travels across the colour. The control had
    simply been sitting in the blank; nine coloured shapes overlap it now, and
    any change to the height of anything above it would have done the same. It
    hides the colour field before sampling, so the frost stays on, the colour
    goes out, and the only thing left that can move the pane is the paint.
    **A pixel test whose result depends on where something happens to land is a
    test that reports the next unrelated layout change as a colour defect.**

    Five checks were added to `example.spec.js`, all geometry: the band reaches
    both edges of its stage on a one-example page and a four-example one; it is
    separated by a border and not by a shadow, and paints a surface the stage
    does not; the example begins below the band rather than overlapping it —
    the developer's complaint, stated as a measurement; the median frame on
    `/button/` stays at 189px and the rail still starts at the frame's top; and
    at 320px in all three locales the band still spans the stage with no
    sideways scroll.

    The background assertion names no colour. `expect(bg).not.toBe("rgba(0, 0,
    0, 0)")` was the obvious way to write "not see-through" and the source gate
    refused it on sight — correctly, since a colour literal in a test is the
    test asserting the stylesheet against itself. It compares the band's
    background to the stage's instead.

    **Then the band showed what the pill had been hiding.** The developer read
    it back: every call opened with what looked like a space before its first
    character, out of line with its own second line. `.nd-code-inline` is a pill
    — a tint with `0 var(--noctua-space-1)` around it — which is right for a
    token name inside a sentence. The `subtle` tone already drops the tint, so
    on the band the only thing left of the pill was its padding, and under
    `pre-wrap` **only the first line is inside it**. Four pixels, on every call
    on the site, invisible for as long as the caption sat on the page's own
    surface with nothing to line up against. The band gave it something.

    It also went a step down in size, to `--noctua-size-caption` — which is the
    size the scale keeps for a `<figcaption>`, and this is one. `Code` sets
    `small` everywhere, and at that size the header was competing with the
    example it names.

    The test for it measures **the rendered text**, not the box: the box was
    never wrong. It walks the caption a character at a time with a `Range`,
    takes the first painted glyph on each line, and asserts the unindented lines
    all begin on the same column — which is the band's padding. Reading a
    computed `padding-inline` would have passed both before and after.

103. **Twelve components, and four repairs to the ground they stand on.** The
    catalogue went from 72 to 86: Dialog, Divider, Icon, Image, Label,
    MessageBar, OtpField, Popover, Rating, Tabs (with `Tab` and `TabPanel`),
    Toast and Tooltip. The browser suite went from 1,234 to 1,532.

    **The repairs came first**, because three of them touch every file a new
    component also touches.

    *Every variant ships folded.* The examples above it already show the
    variants a reader chooses between, in use. The grid is the exhaustive check,
    which is what somebody comparing two values comes for and what everybody
    else scrolls past. `Depth::section_fold()` was added rather than reusing
    `folded()`: the grid is a peer of *Examples* and *Reference*, so folding it
    must not move it down the outline. **It also switched a test back on** —
    `audit.spec.js` measured every stage for emptiness and everything inside a
    shut `<details>` measures 0x0, so the sweep had to learn that a closed fold
    is not an empty stage.

    *`Select`'s variant grid drew the browser's widget, not this component.* It
    was the only form control whose specimen was a live `<select>` — focusable,
    openable, and painted in the platform's own chrome, because the rules that
    give a select its arrow are scoped to a `.nd-select-field` wrapper a
    specimen cannot carry. It also stretched to 30rem, since `.nd-select` is the
    FIELD. It is an inert `-figure` now, like every other input. And the figure
    had a second defect nobody had reported: **one drawn box for all three
    sizes**, on four components at once. The three heights the contracts claim
    are drawn now, written once so three inputs cannot disagree about `large`.

    *`TextInput` is `Input`.* The rename reached the contract, the class, every
    `data-nd-*` hook, both locales, the page slug, the authored bodies, the
    runtime, the stylesheet and the specs. The generated Dioxus, React and
    TypeScript files had to be **deleted by hand** — the build never removes
    files. Its examples were rewritten because the developer was right that they
    were wrong: three fields carried the label *Website*, one of them a
    telephone box, a reference number was labelled *Website* and refused by a
    mail server, a disabled password shipped a nameless reveal button, and two
    captions named props their bodies never set. Six examples now ask six
    questions.

    *`NumberInput` had ten defects*, and the one that explains the rest is that
    its refusal sentences were written into a slot that did not exist. The test
    that should have caught it is titled *"a value out of range is refused with
    a sentence"* and **never asserted a sentence**. Also repaired: no `pattern`,
    so nothing was enforced without a script while a comment claimed otherwise;
    steppers that were visible and inert with no script; `readonly` ignored by
    both the buttons and the handler; `role="spinbutton"` with no `aria-valuenow`
    until touched; grouping that only appeared on focus; focus rounding `7.5` to
    `8` before anything was typed; and `inputmode` disagreeing with its own
    contract.

    **What the twelve had to answer.** `Icon` was promoted rather than
    duplicated: declaring `spec/components/icon.toml` beside the existing
    emitter would have put two `Icon`s in one crate, with the explicit
    re-export silently shadowing the generated one — a divergence with no
    compile error. It moved into the pipeline instead, and the built markup was
    diffed across every English page to prove it renders identically. `Label`
    became the one label and `Field` composes it, which took `.nd-label` over
    from a site class that meant something adjacent. `Tabs` is its own component
    because `Segmented` chooses a value and tabs reveal a panel, and
    `segmented.toml` had been pointing at a component that did not exist.
    `Divider` had to answer a deletion: `.nd-rule` was removed from this site on
    the argument that one unexplained line between equal blocks says nothing.

    **Three defects the work found in what already shipped.** `message-bar.js`
    selected `[data-nd-dismissible="true"]`, which Dialog, Popover and Toast all
    carry, so it stamped them enhanced and the three real enhancers silently
    skipped every one — dialogs opened nothing and no timer ever started, with
    nothing logged. `.nd-example-end` was inert, because its parent aligns to the
    start and the element was only as wide as its own control. And
    `spec/icons.toml` declared three sizes that reached one doc comment and had
    **already drifted** from the stylesheet.

    Four new floating surfaces meant four new entries in `shell.spec.js`'s
    allow-list, each named and justified there before it could ship — which is
    what that list is for. `Toast` is the first entry on it that is visible with
    nothing pressed.

104. **All 86 components audited, because two of them had been wrong the same
    way.** The developer read the `TextInput` and `NumberInput` repairs and drew
    the right inference: those two were written the way the other 84 were, so
    the defects were a sampling result rather than two accidents. Six read-only
    agents audited disjoint slices in parallel, hunting seven classes each taken
    from a defect already confirmed here. Five sequential passes repaired what
    they found. The suite went from 1,532 to 1,653.

    **The worst of it was that the React half did not run.** `table.js` and
    `lineage.js` had not parsed for an unknown length of time — `export` cannot
    be a binding identifier, and `const filter` cannot redeclare the parameter
    `filter` — while `cargo xtask check` was green throughout. `lineage_figure.js`
    and `color_picker.js` parsed and threw on first render, naming variables
    nothing declared. `select.js` and `date_input.js` rendered `Field` without
    importing it. **Six of the package's two published implementations were
    broken and every test passed**, because the parity tests checked the prop
    surface and nothing checked that the output was JavaScript.

    Three guards answer that now, all of them in Rust with no Node: a lexer and
    structural reader that every emitted `.js` must survive, a check that every
    declared default reaches **both** languages, and a check that no React body
    names an identifier it does not have. Each was verified by reintroducing the
    defect it was written for. Replayed against the pre-fix files, they report
    all four originals.

    **The second family was the control that lies.** Every `Switch` on
    `/switch/` was inert — the handler routed unconditionally into the theme
    model and returned early for any name not in it, while the `Segmented`
    branch three lines below had exactly the fallback it lacked, under the
    comment *"A control on a page about controls has to work."* `Menu`'s button
    shipped visible and announced with nothing able to open it. `Toolbar`
    claimed a roving tab stop it did not emit. `Tree.form` promised a hidden
    checkbox that no body rendered. `Carousel` needed a script to show more than
    one slide while its guidance promised otherwise. `Toast` was a panel laid
    over the page with no way out. Where the code was right and the sentence was
    wrong, the sentence was corrected and the reason recorded beside it — twice,
    for `Tree` and `Toolbar`, because a parent cannot write on children it
    cannot see.

    **The third family was the declaration that reaches nothing.** A build-time
    guard reading the emitted source found **31** props declaring a `data-nd-*`
    attribute no body writes, against the 18 the audit had named — and one of
    the opposite kind: `TableRow.expandable` was read by the runtime to decide
    `role="grid"` and written by neither body, so every table of expandable rows
    had been a plain table.

    **`aria-sort` was on 403 header cells** for 13 sortable columns, 214 of them
    on row headers naming a record, while `shell.js` asserted the opposite as
    settled fact three lines from the code that did it. `aria-required` did not
    exist anywhere in the repository, so an enhanced `Select { required }` never
    told anybody the answer was compulsory.

    **And the tests.** Twenty were vacuous. The floating-surface sweep admitted
    `position: fixed` and then filtered on `offsetParent !== null`, which is
    *always* null for a fixed element — ten of its fifteen allow-list entries,
    including all four surfaces the list had just been extended for, were
    unreachable by it. `pagination.spec.js` asserted the absence of
    `[data-nd-pagination-step="previous"]`, **an attribute the component has
    never emitted**; both assertions were vacuously true and would have passed
    with both links rendered. Four more passed on an empty selector with no
    floor. And `audit.spec.js`'s whole-site sweep never looked inside a fold:
    measured after the repair, it sees **702** stages with the folds opened
    against **341** as shipped, so half the site had been outside the clipped,
    overflowing and small-target checks at every width.

    **The Portuguese needed reading, again.** `de zero a cinco` for a scale that
    starts at one, `no ponteiro` for *on hover*, `fora do trabalho` for *clear of
    the work*, a lock turned into a submission, four dropped clauses, the only
    *tu* form in 2,753 keys, and two collisions of the `padrão` shape: `dica`
    naming both the `hint` prop and the `Tooltip` component, and *in the reader's
    language* rendering two ways sixty times. Two guards followed — one English
    sentence to one Portuguese sentence, restricted to prose keys because labels
    legitimately disagree, and an eight-term glossary checked one way.
    `padrão`, `obrigatório` and `ponteiro` were deliberately left out, each with
    two honest English sources.

    **What the exercise says about the suite.** Every defect above sat under a
    green run. The repairs are worth less than the guards, and the guards are the
    reason to write the whole thing down: a parity test that checks a prop
    surface says nothing about the body, a sweep with no floor says nothing about
    an empty page, a test that skips what is hidden says nothing about a fold,
    and a title is not an assertion.


105. **Every example is data, and React renders all of them.**

    The components keep `NOCTUA.md §9.3` by construction, because both halves are
    generated from one spec. The examples did not: all 499 were Rust closures
    returning `rsx!`, which only Dioxus can run, so the React package this system
    ships was rendered nowhere on the site that documents it. Entry 104 is what
    that cost — six React modules broken, two of them not even parsing, under a
    green suite.

    **An example is a tree now.** `docs-site/src/tree.rs` holds the shape,
    derived from a census of all 499 before anything moved: no hooks, no event
    handlers, no branching, no closures in any body, and three dynamic inputs
    between them. Six node shapes and nine value shapes cover it.
    `docs-site/src/generated/interpreter.rs` turns a tree back into the real
    typed components, generated from `spec/components/` because that is the only
    place that knows every prop's kind, default and whether it is required. The
    prop type surface is closed: `Option<String>` 389, `String` 202, `bool` 182,
    `Element` 59, enum 133, `Option<enum>` 5, `IconName` 14, and no fourth thing.

    **The net was tested before it was trusted.**
    `every_tree_draws_what_its_closure_drew` renders both halves with
    `dioxus_ssr` and compares them byte for byte in all three locales — a message
    key transcribed as the wrong key is identical in English often enough to
    matter, and never in Portuguese. It was run against three deliberate lies
    first: a wrong enum value, a wrong key, a flipped bool. All three failed it.
    Final state: **1,497 renderings, no difference.**

    **A gate caught itself.**
    `every_shape_the_ir_declares_is_one_a_tree_uses` shipped with two of the ten
    value shapes missing from its own list, so it passed while proving nothing
    about them — the vacuous-assertion class entry 104 was written about, one
    entry later. Fixed, and it then condemned `Value::Fill`: predicted by the
    census, needed by no example. Deleted.

    **Weight, measured rather than estimated.** The first switch on any page
    costs 85.3 KB gzip for all 83 components, against 62.1 KB for the one
    hardcoded example it replaces. `CodeBlock` and `Lineage` are separate chunks,
    fetched only by the two pages that name them, because the syntax grammars and
    the layout engine are 107 KB nobody else should pay for. Every tree on the
    site is 7.8 KB, in one file beside the search index and for the reason
    written there. A build assertion over the esbuild metafile holds the 1.2 MB
    message catalogue out of the bundle for ever. Nothing is fetched on a page
    view, and that is asserted rather than believed.

    **What the migration found in the examples.** About twenty defects, none
    repaired here. The dominant class is a caption naming a prop its body never
    sets — `TableColumn.width: Wide`, `TableRow.selected`, `hide_label` on both
    `Radio` and `Checkbox`, `Field.kind: Group` in a hero that calls no `Field`.
    Two are worse: a `TreeItem` labelled with the Typography page title submits
    the value `tag`, and the `tables` hero draws invoice `INV-0046` as paid for
    R$ 4.900,00 while every other table on that page draws it as refunded for
    R$ 320,00.

    **The closures are gone.** They were kept beside the trees for exactly as
    long as it took to prove the transcription, and then `examples.rs` went from
    **13,047 lines to 2,536**: the caption and the note, and nothing else. Two
    descriptions of one example, only one of which ships, is the drift this
    repository keeps finding.

    The deletion carries its own proof. The site was built before and after, and
    all **505 files are byte-identical** — the page had already been rendering
    from the trees, so removing what it no longer read could not change it. Two
    tests that read `examples.rs` as text, looking for `icon: "…"` and
    `tone: "…"`, would have gone on passing while scanning a file with no icon
    and no tone in it. They are tree walks now, which cannot go quiet that way.

    **Three defects the tests found afterwards.** The frame's switch shipped as a
    two-stop `Segmented`, and a rail is a **column**: it cost 70px of height on
    every frame, against a rail that exists to spend the width no frame was
    using. An icon toggle costs one row, like the four under it. A per-frame
    press wrote the page-wide preference, so that frame's own Reset re-drew it in
    React instead of putting it back. And the React `ThemeScope` was briefly
    rendered nowhere, because it comes from `spec/theme.toml` rather than
    `spec/components/` and the generated interpreter had no arm for it —
    `tree::theme_scope` is that arm, and its sample markup is byte-identical to
    what the hand-written component drew.

106. **The switch stopped being the theme's accent, and now it is again.** The
    knob is one colour in both states, which is what makes it read as one object
    rather than as two of different sizes. Yesterday's pass paid for that by
    moving the TRACK away from the knob: the on state became
    `color-mix(in oklch, var(--nc-color-accent) 74%, var(--nc-color-surface))`,
    a percentage fitted until every one of the 78 palette-and-appearance pairs
    cleared `WCAG 1.4.11`'s 3:1.

    It cleared it, and it was wrong. Three quarters of a theme's accent over its
    surface is a colour **no palette in this system contains**, so the one
    control a reader flips to see the accent was the one control not showing it.
    Every other filled control — a solid `Button`, a checked `Checkbox`, a
    pressed `Tag` — paints `var(--nc-color-accent)` and nothing else.

    The floor is met the way the colour system always meant it to be. The switch
    is now the two solid tones the `Button` already ships: off is
    `subtle` / `on-subtle`, on is `accent` / `on-accent`, hover one step of each.
    An `on-` colour carrying against its own solid is the palette's own
    guarantee rather than a sum this repository did, and measured across all 78
    pairs the worst case is **3.13:1** with the bare accent. The browser agrees
    with the arithmetic to the same palette and the same two decimals, over 156
    painted-pixel measurements, at one knob size of 16x16 throughout.

    **The test now reads the stylesheet instead of restating it.** The old one
    carried `const ACCENT: f64 = 0.74` beside the rule that mixed 74%, so it
    passed while guarding something already broken — a contrast sum cannot see
    a wrong colour, only a wrong ratio. `every_switch_track_carries_its_knob`
    pulls the five colours out of `shell.css`, demands each be a whole
    `var(--nc-color-…)`, and resolves it through the vendor's `contexts.css` to
    the palette token behind it. A `color-mix` fails at the first step.

    One defect fell out of it. The frosted knob's override named
    `[aria-checked="true"]`, leaving the off state to take the solid rule's ink
    — survivable only while that ink was `fg` anyway. The moment the solid knob
    became white, a frosted switch that was off had a white knob on a white
    frost. Measured at **1.07:1**, and the browser suite asserts both states now
    because the frost is a composite the arithmetic cannot reach.
