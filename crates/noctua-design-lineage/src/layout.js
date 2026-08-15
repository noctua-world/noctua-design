// The layout, for the JavaScript package. Its twin is `layout.rs` and
// `parse.rs` beside it, and the three are one file.
//
// READ THEM TOGETHER OR NOT AT ALL.
//
// A `Lineage` is laid out where it renders: in Rust during pre-rendering, and
// in a browser for the React package. Neither can reach the other, and this
// system ships no runtime that could hold one copy — so the same algorithm has
// to exist twice, and the only way two implementations stay honest is to keep
// them in one directory and change them in one commit.
//
// This file is not generated from the Rust. It is a transcription, function for
// function and branch for branch, and `spec/lineage/cases.toml` — emitted as
// `dist/lineage.cases.json` — is the blessed expectation both are checked
// against, never against each other. Two halves that agree with one another and
// disagree with the specification is the failure that actually happens.
//
// THE THREE INVARIANTS, the same three `scan.js` carries.
//
// 1. Totality. Every line either parses or is counted in `skipped`. No input is
//    rejected and nothing is dropped without being counted.
// 2. Termination. Every loop is bounded by a constant or by the node count. A
//    cycle an author wrote by accident is broken and counted, never followed.
// 3. Purity. The output is a function of (graph, options) and nothing else. No
//    clock, no locale, no randomness — the last is not a preference: the two
//    halves must agree to the last digit.
//
// WHERE JAVASCRIPT WOULD DIVERGE, AND WHAT IS DONE ABOUT IT.
//
// * `Array.prototype.sort` is lexicographic by default. Every sort here passes
//   a comparator. A rank list sorted as strings would put 10 before 2 and the
//   drawing would be silently wrong in one language only.
// * Rust's integer division truncates toward zero. Every division that stands
//   in for one is wrapped in `Math.trunc`.
// * `parseInt` is lenient and `i32::from_str` is not. `whole()` below refuses
//   what Rust refuses, including a number too large to be an `i32`.
// * `toLowerCase()` is not length-preserving and `to_ascii_lowercase` is. The
//   fold is per code point, and it only touches ASCII and the Latin-1 letters.

/** The longest an identifier may be. See `parse.rs` for why there is a bound. */
const ID_LIMIT = 64;
/** How many times the ordering is swept down and up. */
const SWEEPS = 8;
/** Half-cells in a cell. */
const CELL = 2;
/** The bound on rank relaxation. Ranks only increase, so this is a safety net. */
const RELAX = 64;
/** The identity rotation, which every axis-aligned run carries. */
const IDENTITY = [1, 0, 0, 0, 1, 0, 0, 0, 1];

// --- The language -----------------------------------------------------------

/**
 * Reads a graph. Never fails.
 *
 * @param {string} source
 * @returns {{nodes: object[], bonds: object[], skipped: number}}
 */
export function parse(source) {
  const graph = { nodes: [], bonds: [], skipped: 0 };
  for (const line of source.split("\n")) {
    const tokens = tokenize(line);
    if (tokens.length === 0) continue;
    if (tokens[0] === "figure") statement(graph, tokens, "figure");
    else if (tokens[0] === "mark") statement(graph, tokens, "mark");
    else if (tokens[0] === "bond") bond(graph, tokens);
    else graph.skipped += 1;
  }
  return graph;
}

/** Splits a line into tokens, honouring double quotes. */
function tokenize(line) {
  const tokens = [];
  let token = "";
  let quoted = false;
  for (const ch of line) {
    if (quoted) {
      if (ch === '"') quoted = false;
      else token += ch;
    } else if (ch === '"') {
      quoted = true;
    } else if (/\s/u.test(ch)) {
      if (token !== "") {
        tokens.push(token);
        token = "";
      }
    } else if (ch === "#" && token === "") {
      break;
    } else {
      token += ch;
    }
  }
  if (token !== "") tokens.push(token);
  return tokens;
}

/** A blank piece, with the one field a derived default would get wrong. */
function blank(id, piece) {
  return {
    id,
    piece,
    label: "",
    meta: "",
    terms: "",
    image: "",
    kind: "",
    tone: "",
    shape: "",
    set: "",
    href: "",
    cluster: "",
    rank: null,
    span: piece === "mark" ? 1 : 2,
    // How far connections stop short of this piece — see `lib.rs`.
    reach: -1,
    slot: false,
    origin: false,
    // How many figures this stands in for, when it is a cut marker — see
    // `lib.rs`. Zero for everything an author wrote.
    elided: 0,
  };
}

/** `figure <id> …` and `mark <id> …`. */
function statement(graph, tokens, piece) {
  const id = identifier(tokens[1]);
  if (id === null) {
    graph.skipped += 1;
    return;
  }
  const node = blank(id, piece);
  for (const token of tokens.slice(2)) {
    if (!attribute(node, token)) graph.skipped += 1;
  }
  graph.nodes.push(node);
}

/** `bond <from> <to> …`. */
function bond(graph, tokens) {
  const from = identifier(tokens[1]);
  const to = identifier(tokens[2]);
  if (from === null || to === null || from === to) {
    graph.skipped += 1;
    return;
  }
  const bond = {
    from,
    to,
    kind: "",
    shape: "descent",
    label: "",
    note: "",
    tone: "",
    dash: "",
    route: "",
    set: "",
  };
  for (const token of tokens.slice(3)) {
    if (!bondAttribute(bond, token)) graph.skipped += 1;
  }
  bond.shape = shapeOf(bond.kind);
  graph.bonds.push(bond);
}

/** What a bond's `kind` does to the ranks of the two things it joins. */
export function shapeOf(kind) {
  // A donor joins a union the way a parent does — see `lib.rs`.
  if (kind === "union" || kind === "partner" || kind === "marriage" || kind === "donor") return "union";
  if (kind === "link" || kind === "lateral" || kind === "note") return "lateral";
  return "descent";
}

/** One `key=value` or one flag on a piece. `false` means nobody recognised it. */
function attribute(node, token) {
  if (token === "origin") {
    node.origin = true;
    return true;
  }
  if (token === "slot") {
    node.slot = true;
    return true;
  }
  const at = token.indexOf("=");
  if (at < 0) return false;
  const key = token.slice(0, at);
  const value = token.slice(at + 1);
  switch (key) {
    case "label":
      node.label = text(value);
      return true;
    case "meta":
      node.meta = text(value);
      return true;
    case "terms":
      node.terms = text(value);
      return true;
    case "kind":
      node.kind = word(value);
      return true;
    case "tone":
      node.tone = word(value);
      return true;
    case "shape":
      node.shape = word(value);
      return true;
    case "set":
      node.set = word(value);
      return true;
    case "cluster":
      node.cluster = word(value);
      return true;
    case "image":
      node.image = url(value);
      return true;
    case "href":
      node.href = url(value);
      return true;
    case "rank": {
      const rank = whole(value);
      node.rank = rank === null ? null : clamp(rank, -999, 999);
      return true;
    }
    case "reach": {
      // The same spellings `span` accepts — see `parse.rs`.
      const plainReach = /^[0-9]+(?:\.[0-9]*)?$/.test(value) || /^\.[0-9]+$/.test(value);
      if (plainReach) {
        const cells = Math.min(6, Math.max(0, Number.parseFloat(value) || 0));
        // Tenths, not half-cells — see `lib.rs`.
        node.reach = Math.round(cells * 10);
      }
      return true;
    }
    case "span": {
      // Halves are legal, and the two engines accept the same spellings:
      // digits and at most one dot — see `parse.rs`.
      const plain = /^[0-9]+(?:\.[0-9]*)?$/.test(value) || /^\.[0-9]+$/.test(value);
      const span = plain ? Number.parseFloat(value) : Number.NaN;
      const cells = Number.isFinite(span) ? Math.min(12, Math.max(0.5, span)) : 1;
      node.span = Math.round(cells * 2);
      return true;
    }
    default:
      return false;
  }
}

/** One `key=value` on a bond. `false` means nobody recognised it. */
function bondAttribute(bond, token) {
  const at = token.indexOf("=");
  if (at < 0) return false;
  const key = token.slice(0, at);
  const value = token.slice(at + 1);
  switch (key) {
    case "kind":
      bond.kind = word(value);
      return true;
    case "label":
      bond.label = text(value);
      return true;
    case "note":
      bond.note = text(value);
      return true;
    case "tone":
      bond.tone = word(value);
      return true;
    case "dash":
      bond.dash = word(value);
      return true;
    case "route":
      bond.route = word(value);
      return true;
    case "set":
      bond.set = word(value);
      return true;
    default:
      return false;
  }
}

/** An identifier, or `null`. */
function identifier(value) {
  if (value === undefined) return null;
  const ok = value.length > 0 && value.length <= ID_LIMIT && /^[A-Za-z0-9_-]+$/u.test(value);
  return ok ? value : null;
}

/** A word that reaches the DOM as an attribute value and a CSS selector. */
function word(value) {
  const ok = value.length > 0 && value.length <= ID_LIMIT && /^[A-Za-z0-9_:-]+$/u.test(value);
  return ok ? value : "";
}

/** Reader-facing text, bounded by length. Counted in code points, as Rust does. */
function text(value) {
  return [...value].slice(0, 160).join("");
}

/**
 * A URL, or the empty string.
 *
 * Relative, or `http`/`https` absolute. Everything else — `javascript:`,
 * `data:`, a scheme nobody has heard of — is dropped, because this string may
 * have come from a consumer's database and lands in an `href` or a `src`.
 */
function url(value) {
  const lower = asciiLower(value);
  const absolute = lower.startsWith("https://") || lower.startsWith("http://");
  const at = value.indexOf(":");
  const schemeish = at >= 0 && !value.slice(0, at).includes("/");
  return value === "" || (schemeish && !absolute) ? "" : value;
}

/** What Rust's `i32::from_str` accepts, and nothing else. */
function whole(value) {
  if (!/^[+-]?\d+$/u.test(value)) return null;
  const number = Number(value);
  return number >= -2147483648 && number <= 2147483647 ? number : null;
}

/** ASCII only, so it is length-preserving in both languages. */
function asciiLower(text) {
  let out = "";
  for (const ch of text) out += ch >= "A" && ch <= "Z" ? ch.toLowerCase() : ch;
  return out;
}

function clamp(value, low, high) {
  return Math.min(Math.max(value, low), high);
}

/**
 * The least of a list, or `fallback` when it is empty.
 *
 * Written out rather than `Math.min(...list)`: spreading a list of a few
 * thousand into a call blows the argument limit, and a scene that large should
 * draw badly rather than throw.
 */
function minOf(list, fallback) {
  let least = fallback;
  for (let index = 0; index < list.length; index += 1) {
    if (index === 0 || list[index] < least) least = list[index];
  }
  return least;
}

/** The greatest of a list, or `fallback` when it is empty. */
function maxOf(list, fallback) {
  let most = fallback;
  for (let index = 0; index < list.length; index += 1) {
    if (index === 0 || list[index] > most) most = list[index];
  }
  return most;
}

// --- Filtering --------------------------------------------------------------

/**
 * Lower-cases and strips the Latin-1 diacritics, one code point in, one out.
 *
 * Deliberately not `toLowerCase()`: that is not length-preserving, and the
 * matched positions are the only thing a highlight has.
 */
export function fold(text) {
  let out = "";
  for (const ch of text) {
    const code = ch.codePointAt(0);
    if ((code >= 0xc0 && code <= 0xc5) || (code >= 0xe0 && code <= 0xe5)) out += "a";
    else if (code === 0xc7 || code === 0xe7) out += "c";
    else if ((code >= 0xc8 && code <= 0xcb) || (code >= 0xe8 && code <= 0xeb)) out += "e";
    else if ((code >= 0xcc && code <= 0xcf) || (code >= 0xec && code <= 0xef)) out += "i";
    else if (code === 0xd1 || code === 0xf1) out += "n";
    else if (
      (code >= 0xd2 && code <= 0xd6) ||
      (code >= 0xf2 && code <= 0xf6) ||
      code === 0xd8 ||
      code === 0xf8
    )
      out += "o";
    else if ((code >= 0xd9 && code <= 0xdc) || (code >= 0xf9 && code <= 0xfc)) out += "u";
    else if (code === 0xdd || code === 0xfd || code === 0xff) out += "y";
    else out += ch >= "A" && ch <= "Z" ? ch.toLowerCase() : ch;
  }
  return out;
}

/**
 * Whether a piece answers a filter. An empty filter matches everything, and so
 * does a filter of nothing but spaces — see `lib.rs`, which needed this half to
 * notice that its own test was passing for the wrong reason.
 */
export function matches(node, filter) {
  const wanted = trim(fold(filter));
  if (wanted === "") return true;
  return fold(`${node.label} ${node.meta} ${node.terms}`).includes(wanted);
}

/** ASCII whitespace only: `trim()` and `str::trim` disagree about the rest. */
function trim(text) {
  const white = (ch) => ch === " " || ch === "\t" || ch === "\n" || ch === "\r" || ch === "\f";
  let start = 0;
  let end = text.length;
  while (start < end && white(text[start])) start += 1;
  while (end > start && white(text[end - 1])) end -= 1;
  return text.slice(start, end);
}

// --- The layout -------------------------------------------------------------

/** Parses a graph and places it, which is the whole module in one call. */
export function scene(source, options) {
  return place(parse(source), options);
}

/**
 * Places a parsed graph.
 *
 * @param {{nodes: object[], bonds: object[], skipped: number}} graph
 * @param {{view?: string, route?: string}} options
 */
export function place(graph, options) {
  const view = options && options.view === "deep" ? "deep" : "flat";
  const route = options && options.route === "taut" ? "taut" : "elbow";
  const shy = !!(options && options.shy);

  const counted = { skipped: graph.skipped };
  const tidied = tidy(graph, counted);
  const work = newWork(tidied);

  adopt(tidied, work);
  const broken = rankEverything(work, tidied);
  groupEverything(work, tidied);
  const crossings = orderEverything(work, tidied);
  placeEverything(work, tidied, view);

  const places = placesOf(work, tidied, view);
  const segments = routeAll(tidied, places, route);
  shyEnds(tidied, places, segments, shy);
  const size = extent(tidied, places);

  return {
    graph: tidied,
    places,
    segments,
    width: size[0],
    height: size[1],
    depth: size[2],
    crossings,
    skipped: counted.skipped,
    broken,
  };
}

/** Drops what cannot be drawn, and counts it. */
function tidy(graph, counted) {
  const nodes = [];
  for (const node of graph.nodes) {
    if (nodes.some((kept) => kept.id === node.id)) counted.skipped += 1;
    else nodes.push(node);
  }
  const bonds = [];
  const known = (id) => nodes.some((node) => node.id === id);
  for (const bond of graph.bonds) {
    if (known(bond.from) && known(bond.to)) bonds.push(bond);
    else counted.skipped += 1;
  }
  return { nodes, bonds, skipped: graph.skipped };
}

function newWork(graph) {
  const n = graph.nodes.length;
  const fill = (value) => Array.from({ length: n }, () => value);
  return {
    rank: fill(0),
    cluster: Array.from({ length: n }, (_, index) => index),
    branch: fill(0),
    group: fill(0),
    order: fill(0),
    x: fill(0),
    parents: Array.from({ length: n }, () => []),
    children: Array.from({ length: n }, () => []),
    partners: Array.from({ length: n }, () => []),
    spans: [],
  };
}

/** The index of an id. Every bond naming anything else is already gone. */
function indexOf(graph, id) {
  const at = graph.nodes.findIndex((node) => node.id === id);
  return at < 0 ? 0 : at;
}

/** Hangs everything parentless from the origin of its own declared cluster. */
function adopt(graph, work) {
  const origins = [];
  for (let index = 0; index < graph.nodes.length; index += 1) {
    if (graph.nodes[index].origin) origins.push(index);
  }
  if (origins.length === 0) return;
  const parented = graph.nodes.map((node) =>
    graph.bonds.some((bond) => bond.shape === "descent" && bond.to === node.id),
  );
  // One of a couple, not both — see `layout.rs`.
  for (const origin of origins) {
    const adopted = [];
    const taken = [];
    for (let index = 0; index < graph.nodes.length; index += 1) {
      const node = graph.nodes[index];
      if (
        index === origin ||
        node.origin ||
        parented[index] ||
        node.piece !== "figure" ||
        node.cluster !== graph.nodes[origin].cluster
      ) {
        continue;
      }
      // Through the mark — see `layout.rs`.
      const partnerOf = (id, other) =>
        graph.bonds.some(
          (bond) =>
            bond.shape === "union" &&
            ((bond.from === id && bond.to === other) || (bond.to === id && bond.from === other)),
        );
      const partnered = taken.some(
        (held) =>
          partnerOf(node.id, held) ||
          graph.nodes.some(
            (mark) =>
              mark.piece === "mark" && partnerOf(node.id, mark.id) && partnerOf(held, mark.id),
          ),
      );
      if (partnered) continue;
      taken.push(node.id);
      adopted.push(node.id);
    }
    for (const id of adopted) {
      graph.bonds.push({
        from: graph.nodes[origin].id,
        to: id,
        kind: "origin",
        shape: "descent",
        label: "",
        note: "",
        tone: "",
        dash: "",
        route: "",
        set: "",
      });
    }
  }
  const n = graph.nodes.length;
  work.parents = Array.from({ length: n }, () => []);
  work.children = Array.from({ length: n }, () => []);
}

// --- Rank -------------------------------------------------------------------

function rankEverything(work, graph) {
  let edges = [];
  const unions = [];
  for (const bond of graph.bonds) {
    const from = indexOf(graph, bond.from);
    const to = indexOf(graph, bond.to);
    if (bond.shape === "descent") edges.push([from, to]);
    else if (bond.shape === "union") unions.push([from, to]);
  }
  const cut = breakCycles(graph.nodes.length, edges);
  edges = cut.edges;

  for (let index = 0; index < graph.nodes.length; index += 1) {
    work.rank[index] = graph.nodes[index].rank === null ? 0 : graph.nodes[index].rank;
  }
  relax(work.rank, edges);
  for (const [a, b] of unions) {
    marry(work, a, b, edges);
    work.partners[a].push(b);
    work.partners[b].push(a);
  }
  for (let index = 0; index < graph.nodes.length; index += 1) {
    for (const [from, to] of edges) {
      if (to === index) work.parents[index].push(from);
      if (from === index) work.children[index].push(to);
    }
  }
  const floor = minOf(work.rank, 0);
  for (let index = 0; index < work.rank.length; index += 1) work.rank[index] -= floor;
  return cut.broken;
}

/** Pulls the shallower partner down, unless it is an ancestor of the deeper. */
function marry(work, a, b, edges) {
  const shallow = work.rank[a] <= work.rank[b] ? a : b;
  const deep = shallow === a ? b : a;
  if (work.rank[shallow] === work.rank[deep] || reaches(edges, shallow, deep)) return;
  work.rank[shallow] = work.rank[deep];
  relax(work.rank, edges);
}

/** Removes the back edges a depth-first walk finds, so the rest is acyclic. */
function breakCycles(n, edges) {
  const colour = Array.from({ length: n }, () => 0);
  const keep = edges.map(() => true);
  let broken = 0;
  for (let start = 0; start < n; start += 1) {
    if (colour[start] !== 0) continue;
    const stack = [[start, 0]];
    colour[start] = 1;
    while (stack.length > 0) {
      const top = stack[stack.length - 1];
      const node = top[0];
      let at = -1;
      for (let index = top[1]; index < edges.length; index += 1) {
        if (edges[index][0] === node) {
          at = index;
          break;
        }
      }
      if (at >= 0) {
        const to = edges[at][1];
        top[1] = at + 1;
        if (colour[to] === 1) {
          keep[at] = false;
          broken += 1;
        } else if (colour[to] === 0) {
          colour[to] = 1;
          stack.push([to, 0]);
        }
      } else {
        colour[node] = 2;
        stack.pop();
      }
    }
  }
  return { edges: edges.filter((_, index) => keep[index]), broken };
}

/** Pushes every child below its parents, repeatedly. Ranks only increase. */
function relax(rank, edges) {
  for (let round = 0; round < RELAX; round += 1) {
    let moved = false;
    for (const [from, to] of edges) {
      if (rank[to] < rank[from] + 1) {
        rank[to] = rank[from] + 1;
        moved = true;
      }
    }
    if (!moved) return;
  }
}

/** Whether `from` reaches `to` by descent. */
function reaches(edges, from, to) {
  const seen = [from];
  let cursor = 0;
  while (cursor < seen.length) {
    const node = seen[cursor];
    cursor += 1;
    for (const [a, b] of edges) {
      if (a === node && b === to) return true;
      if (a === node && !seen.includes(b)) seen.push(b);
    }
  }
  return false;
}

// --- Group ------------------------------------------------------------------

function groupEverything(work, graph) {
  for (const bond of graph.bonds) {
    join(work, indexOf(graph, bond.from), indexOf(graph, bond.to));
  }
  for (let index = 0; index < graph.nodes.length; index += 1) {
    const node = graph.nodes[index];
    if (node.cluster === "") continue;
    const first = graph.nodes.findIndex((other) => other.cluster === node.cluster);
    if (first >= 0) join(work, first, index);
  }
  compactClusters(work);
  findBranches(work);
}

function join(work, a, b) {
  const from = work.cluster[a];
  const to = work.cluster[b];
  if (from === to) return;
  for (let index = 0; index < work.cluster.length; index += 1) {
    if (work.cluster[index] === from) work.cluster[index] = to;
  }
}

function compactClusters(work) {
  const seen = [];
  for (let index = 0; index < work.cluster.length; index += 1) {
    const key = work.cluster[index];
    let at = seen.indexOf(key);
    if (at < 0) {
      seen.push(key);
      at = seen.length - 1;
    }
    work.cluster[index] = at;
  }
}

/** A branch is everything descending from one child of the trunk. */
function findBranches(work) {
  const clusters = maxOf(work.cluster, -1) + 1;
  let next = 1;
  for (let cluster = 0; cluster < clusters; cluster += 1) {
    const ranks = [];
    for (let index = 0; index < work.rank.length; index += 1) {
      if (work.cluster[index] === cluster) ranks.push(work.rank[index]);
    }
    const top = minOf(ranks, 0);
    for (let index = 0; index < work.rank.length; index += 1) {
      const childOfTheTrunk = work.parents[index].some((parent) => work.rank[parent] === top);
      if (work.cluster[index] === cluster && work.rank[index] === top + 1 && childOfTheTrunk) {
        work.branch[index] = next;
        next += 1;
      }
    }
  }
  for (let round = 0; round < RELAX; round += 1) {
    let moved = false;
    for (let index = 0; index < work.branch.length; index += 1) {
      if (work.branch[index] !== 0) continue;
      const kin = work.parents[index].concat(work.partners[index]);
      const other = kin.find((candidate) => work.branch[candidate] !== 0);
      if (other !== undefined) {
        work.branch[index] = work.branch[other];
        moved = true;
      }
    }
    if (!moved) break;
  }
}

// --- Order ------------------------------------------------------------------

function orderEverything(work, graph) {
  for (let index = 0; index < graph.nodes.length; index += 1) {
    work.group[index] = work.cluster[index];
    work.order[index] = index;
  }
  let best = countCrossings(work, graph);
  let kept = work.order.slice();
  for (let sweep = 0; sweep < SWEEPS; sweep += 1) {
    sweepOnce(work, graph, sweep % 2 === 0);
    const now = countCrossings(work, graph);
    if (now < best) {
      best = now;
      kept = work.order.slice();
    }
  }
  work.order = kept;
  best = exactOrder(work, graph, best);
  best = transpose(work, graph, best);
  pullPartners(work, graph, best);
  seatMarks(work, graph);
  flankMarks(work, graph);
  return countCrossings(work, graph);
}

/** Closes the row around every union mark — see `layout.rs`. */
function flankMarks(work, graph) {
  for (let group = 0; group < groupCount(work); group += 1) {
    for (const rank of ranksOfGroup(work, graph, group)) {
      const sequence = rowOf(work, graph, group, rank);
      const orders = sequence.map((index) => work.order[index]).sort((a, b) => a - b);
      const marks = sequence.filter((index) => graph.nodes[index].piece === "mark");
      for (const mark of marks) {
        for (const side of [-1, 1]) {
          const at = sequence.indexOf(mark);
          let nearest = null;
          for (const p of work.partners[mark]) {
            if (work.rank[p] !== rank || work.group[p] !== group) continue;
            const pos = sequence.indexOf(p);
            if (pos < 0) continue;
            const offset = pos - at;
            if (Math.sign(offset) !== side) continue;
            if (nearest === null || Math.abs(offset) < nearest[0]) {
              nearest = [Math.abs(offset), pos];
            }
          }
          if (nearest === null) continue;
          const partner = sequence.splice(nearest[1], 1)[0];
          const seat = sequence.indexOf(mark) + (side < 0 ? 0 : 1);
          sequence.splice(seat, 0, partner);
        }
      }
      for (let slot = 0; slot < sequence.length; slot += 1) {
        work.order[sequence[slot]] = orders[slot];
      }
    }
  }
}

/** Seats every union mark between its partners on its own row — see `layout.rs`. */
function seatMarks(work, graph) {
  for (let group = 0; group < groupCount(work); group += 1) {
    for (const rank of ranksOfGroup(work, graph, group)) {
      const row = rowOf(work, graph, group, rank);
      const keyed = row.map((index) => {
        const same = work.partners[index].filter(
          (other) => work.rank[other] === rank && work.group[other] === group,
        );
        if (graph.nodes[index].piece === "mark" && same.length > 0) {
          let sum = 0;
          for (const other of same) sum += work.order[other];
          return [sum, same.length, work.order[index], index];
        }
        return [work.order[index], 1, work.order[index], index];
      });
      keyed.sort((a, b) => a[0] * b[1] - b[0] * a[1] || a[2] - b[2]);
      const orders = row.map((index) => work.order[index]).sort((a, b) => a - b);
      for (let slot = 0; slot < keyed.length; slot += 1) {
        work.order[keyed[slot][3]] = orders[slot];
      }
    }
  }
}

/** Swaps every neighbouring pair that removes a crossing — see `layout.rs`. */
function transpose(work, graph, best) {
  for (let round = 0; round < SWEEPS; round += 1) {
    let moved = false;
    const ranks = [];
    for (let index = 0; index < graph.nodes.length; index += 1) {
      if (!ranks.includes(work.rank[index])) ranks.push(work.rank[index]);
    }
    ranks.sort((a, b) => a - b);
    for (const rank of ranks) {
      const row = [];
      for (let index = 0; index < graph.nodes.length; index += 1) {
        if (work.rank[index] === rank) row.push(index);
      }
      row.sort((a, b) => work.order[a] - work.order[b]);
      for (let pair = 1; pair < row.length; pair += 1) {
        const a = row[pair - 1];
        const b = row[pair];
        const held = work.order[a];
        work.order[a] = work.order[b];
        work.order[b] = held;
        const now = countCrossings(work, graph);
        if (now < best) {
          best = now;
          const swap_ = row[pair - 1];
          row[pair - 1] = row[pair];
          row[pair] = swap_;
          moved = true;
        } else {
          const back = work.order[a];
          work.order[a] = work.order[b];
          work.order[b] = back;
        }
      }
    }
    if (!moved) break;
  }
  return best;
}

/** Tries every ordering of every small row — see `layout.rs`. */
function exactOrder(work, graph, best) {
  if (graph.nodes.length > 24) return best;
  const groups = groupCount(work);
  for (let pass = 0; pass < 2; pass += 1) {
    for (let group = 0; group < groups; group += 1) {
      const ranks = [];
      for (let index = 0; index < graph.nodes.length; index += 1) {
        if (work.group[index] === group && !ranks.includes(work.rank[index])) {
          ranks.push(work.rank[index]);
        }
      }
      ranks.sort((a, b) => a - b);
      for (const rank of ranks) {
        const row = [];
        for (let index = 0; index < graph.nodes.length; index += 1) {
          if (work.group[index] === group && work.rank[index] === rank) row.push(index);
        }
        row.sort((a, b) => work.order[a] - work.order[b]);
        if (row.length < 2 || row.length > 6) continue;
        const values = row.map((index) => work.order[index]);
        let assignment = row.map((_, slot) => slot);
        let kept = assignment.slice();
        for (;;) {
          for (let slot = 0; slot < assignment.length; slot += 1) {
            work.order[row[assignment[slot]]] = values[slot];
          }
          const now = countCrossings(work, graph);
          if (now < best) {
            best = now;
            kept = assignment.slice();
          }
          if (!nextPermutation(assignment)) break;
        }
        for (let slot = 0; slot < kept.length; slot += 1) {
          work.order[row[kept[slot]]] = values[slot];
        }
      }
    }
  }
  return best;
}

/** The next permutation in lexicographic order — see `layout.rs`. */
function nextPermutation(items) {
  if (items.length < 2) return false;
  let pivot = items.length - 1;
  while (pivot > 0 && items[pivot - 1] >= items[pivot]) pivot -= 1;
  if (pivot === 0) return false;
  let swapWith = items.length - 1;
  while (items[swapWith] <= items[pivot - 1]) swapWith -= 1;
  const held = items[pivot - 1];
  items[pivot - 1] = items[swapWith];
  items[swapWith] = held;
  const tail = items.splice(pivot).reverse();
  for (const item of tail) items.push(item);
  return true;
}

function sweepOnce(work, graph, downward) {
  const ranks = uniqueRanks(work);
  if (!downward) ranks.reverse();
  for (const rank of ranks) {
    const row = [];
    for (let index = 0; index < graph.nodes.length; index += 1) {
      if (work.rank[index] === rank) row.push(index);
    }
    const paired = row.map((index) => [median(work, index, downward), work.order[index], index]);
    paired.sort((a, b) => a[0] - b[0] || a[1] - b[1]);
    for (let position = 0; position < paired.length; position += 1) {
      work.order[paired[position][2]] = position;
    }
  }
}

/** Every rank in the scene, ascending. The comparator is not optional. */
function uniqueRanks(work) {
  const ranks = work.rank.slice().sort((a, b) => a - b);
  return ranks.filter((rank, index) => index === 0 || rank !== ranks[index - 1]);
}

function median(work, index, downward) {
  const neighbours = downward ? work.parents[index] : work.children[index];
  if (neighbours.length === 0) return work.order[index] * 2;
  const positions = neighbours.map((other) => work.order[other]).sort((a, b) => a - b);
  const middle = Math.trunc(positions.length / 2);
  if (positions.length % 2 === 1) return positions[middle] * 2;
  return positions[middle - 1] + positions[middle];
}

/** Moves partners next to one another where it costs nothing. */
function pullPartners(work, graph, best) {
  for (let round = 0; round < SWEEPS; round += 1) {
    let moved = false;
    for (let a = 0; a < graph.nodes.length; a += 1) {
      for (const b of work.partners[a].slice()) {
        if (work.rank[a] !== work.rank[b] || Math.abs(work.order[a] - work.order[b]) <= 1) continue;
        const step = work.order[a] < work.order[b] ? 1 : -1;
        const target = work.order[a] + step;
        let other = -1;
        for (let index = 0; index < graph.nodes.length; index += 1) {
          if (work.rank[index] === work.rank[a] && work.order[index] === target) {
            other = index;
            break;
          }
        }
        if (other < 0) continue;
        swap(work.order, a, other);
        const now = countCrossings(work, graph);
        if (now <= best) {
          best = now;
          moved = true;
        } else {
          swap(work.order, a, other);
        }
      }
    }
    if (!moved) return;
  }
}

function swap(list, a, b) {
  const held = list[a];
  list[a] = list[b];
  list[b] = held;
}

/** How many pairs of connections cross, over every pair of adjacent rows. */
function countCrossings(work, graph) {
  // Every bond, over every row it spans — see `layout.rs`.
  const edges = [];
  for (const bond of graph.bonds) {
    const from = indexOf(graph, bond.from);
    const to = indexOf(graph, bond.to);
    if (work.group[from] !== work.group[to]) continue;
    const top = work.rank[from] <= work.rank[to] ? from : to;
    const bottom = top === from ? to : from;
    const span = work.rank[bottom] - work.rank[top];
    if (span === 0) {
      edges.push([work.rank[top], work.order[top], work.order[bottom]]);
      continue;
    }
    for (let step = 0; step < span; step += 1) {
      const a =
        work.order[top] + Math.trunc(((work.order[bottom] - work.order[top]) * step) / span);
      const b =
        work.order[top] +
        Math.trunc(((work.order[bottom] - work.order[top]) * (step + 1)) / span);
      edges.push([work.rank[top] + step, a, b]);
    }
  }
  let total = 0;
  for (let index = 0; index < edges.length; index += 1) {
    const [rank, upper, lower] = edges[index];
    for (let other = index + 1; other < edges.length; other += 1) {
      const [otherRank, otherUpper, otherLower] = edges[other];
      if (rank !== otherRank) continue;
      if (
        (upper < otherUpper && lower > otherLower) ||
        (upper > otherUpper && lower < otherLower)
      ) {
        total += 1;
      }
    }
  }
  return total;
}

// --- Place ------------------------------------------------------------------

function placeEverything(work, graph, view) {
  // One gap, and it is a cell — see `lib.rs` for what used to choose it.
  const gap = CELL;
  if (view === "deep") {
    for (let index = 0; index < graph.nodes.length; index += 1) work.group[index] = work.branch[index];
  }
  packRows(work, graph, gap);
  for (let round = 0; round < 4; round += 1) {
    centre(work, graph, true, gap);
    centre(work, graph, false, gap);
  }
  // A partner with no other ties stands beside the union — see `layout.rs`.
  snuggle(work, graph, gap);
  // Nothing left of zero — see `layout.rs` for the half of Bob Doe.
  const groups = groupCount(work);
  for (let group = 0; group < groups; group += 1) {
    let least = Number.MAX_SAFE_INTEGER;
    for (let index = 0; index < graph.nodes.length; index += 1) {
      if (work.group[index] === group && work.x[index] < least) least = work.x[index];
    }
    if (least !== Number.MAX_SAFE_INTEGER && least !== 0) {
      for (let index = 0; index < graph.nodes.length; index += 1) {
        if (work.group[index] === group) work.x[index] -= least;
      }
    }
  }
  measureGroups(work, graph, gap);
}

function groupCount(work) {
  return maxOf(work.group, -1) + 1;
}

function ranksOfGroup(work, graph, group) {
  const ranks = [];
  for (let index = 0; index < graph.nodes.length; index += 1) {
    if (work.group[index] === group) ranks.push(work.rank[index]);
  }
  ranks.sort((a, b) => a - b);
  return ranks.filter((rank, index) => index === 0 || rank !== ranks[index - 1]);
}

function packRows(work, graph, gap) {
  for (let group = 0; group < groupCount(work); group += 1) {
    for (const rank of ranksOfGroup(work, graph, group)) {
      let cursor = 0;
      let previous = -1;
      for (const index of rowOf(work, graph, group, rank)) {
        const air = previous < 0 ? 0 : airBetween(graph, previous, index, gap);
        const half = Math.trunc(graph.nodes[index].span / 2);
        work.x[index] = cursor + air + half;
        cursor = work.x[index] + half;
        previous = index;
      }
    }
  }
}

function rowOf(work, graph, group, rank) {
  const row = [];
  for (let index = 0; index < graph.nodes.length; index += 1) {
    if (work.group[index] === group && work.rank[index] === rank) row.push(index);
  }
  row.sort((a, b) => work.order[a] - work.order[b]);
  return row;
}

function centre(work, graph, downward, gap) {
  for (let group = 0; group < groupCount(work); group += 1) {
    const ranks = ranksOfGroup(work, graph, group);
    if (!downward) ranks.reverse();
    for (const rank of ranks) {
      const row = rowOf(work, graph, group, rank);
      for (const index of row) {
        const neighbours = downward ? work.parents[index] : work.children[index];
        // Partners count as kin only across rows — see `layout.rs`.
        const kin = neighbours
          .concat(work.partners[index].filter((other) => work.rank[other] !== work.rank[index]))
          .filter((other) => work.group[other] === group)
          .map((other) => work.x[other]);
        if (kin.length === 0) continue;
        const low = minOf(kin, 0);
        const high = maxOf(kin, 0);
        work.x[index] = Math.trunc((low + high) / 2);
      }
      separate(work, graph, row, gap);
    }
  }
}

/** Pulls a figure whose only ties are same-row unions flush against its
 * nearest partner — see `layout.rs` for why nothing else ever moves it. */
function snuggle(work, graph, gap) {
  for (let group = 0; group < groupCount(work); group += 1) {
    for (const rank of ranksOfGroup(work, graph, group)) {
      const row = rowOf(work, graph, group, rank);
      for (let slot = 0; slot < row.length; slot += 1) {
        const index = row[slot];
        if (
          graph.nodes[index].piece !== "figure" ||
          work.parents[index].length > 0 ||
          work.children[index].length > 0 ||
          work.partners[index].length === 0 ||
          work.partners[index].some(
            (other) => work.rank[other] !== rank || work.group[other] !== group,
          )
        ) {
          continue;
        }
        let near = work.partners[index][0];
        for (const other of work.partners[index]) {
          if (Math.abs(work.x[other] - work.x[index]) < Math.abs(work.x[near] - work.x[index])) {
            near = other;
          }
        }
        const desired =
          work.x[index] <= work.x[near]
            ? work.x[near] -
              Math.trunc(graph.nodes[near].span / 2) -
              airBetween(graph, index, near, gap) -
              Math.trunc(graph.nodes[index].span / 2)
            : work.x[near] +
              Math.trunc(graph.nodes[near].span / 2) +
              airBetween(graph, near, index, gap) +
              Math.trunc(graph.nodes[index].span / 2);
        let x = desired;
        if (slot > 0) {
          const prev = row[slot - 1];
          x = Math.max(
            x,
            work.x[prev] +
              Math.trunc(graph.nodes[prev].span / 2) +
              airBetween(graph, prev, index, gap) +
              Math.trunc(graph.nodes[index].span / 2),
          );
        }
        if (slot + 1 < row.length) {
          const next = row[slot + 1];
          x = Math.min(
            x,
            work.x[next] -
              Math.trunc(graph.nodes[next].span / 2) -
              airBetween(graph, index, next, gap) -
              Math.trunc(graph.nodes[index].span / 2),
          );
        }
        // Moving away from the partner is not snuggling.
        if (Math.abs(x - work.x[near]) < Math.abs(work.x[index] - work.x[near])) {
          work.x[index] = x;
        }
      }
    }
  }
}

function separate(work, graph, row, gap) {
  const before = row.map((index) => work.x[index]);
  for (let at = 0; at + 1 < row.length; at += 1) {
    const left = row[at];
    const right = row[at + 1];
    const least =
      work.x[left] +
      Math.trunc(graph.nodes[left].span / 2) +
      airBetween(graph, left, right, gap) +
      Math.trunc(graph.nodes[right].span / 2);
    if (work.x[right] < least) work.x[right] = least;
  }
  for (let at = row.length - 2; at >= 0; at -= 1) {
    const left = row[at];
    const right = row[at + 1];
    const most =
      work.x[right] -
      Math.trunc(graph.nodes[right].span / 2) -
      airBetween(graph, left, right, gap) -
      Math.trunc(graph.nodes[left].span / 2);
    if (work.x[left] > most) work.x[left] = most;
  }
  if (row.length === 0) return;
  // Mean-preserving — see `layout.rs` for the ratchet this stops.
  let drift = 0;
  let was = 0;
  for (let at = 0; at < row.length; at += 1) {
    drift += work.x[row[at]];
    was += before[at];
  }
  const shift = Math.floor((drift - was) / row.length);
  if (shift !== 0) {
    for (const index of row) work.x[index] -= shift;
  }
}

function measureGroups(work, graph, gap) {
  const groups = groupCount(work);
  const left = Array.from({ length: groups }, () => Number.MAX_SAFE_INTEGER);
  for (let index = 0; index < graph.nodes.length; index += 1) {
    const edge = work.x[index] - Math.trunc(graph.nodes[index].span / 2);
    if (edge < left[work.group[index]]) left[work.group[index]] = edge;
  }
  for (let index = 0; index < graph.nodes.length; index += 1) {
    const shift = left[work.group[index]];
    if (shift !== Number.MAX_SAFE_INTEGER) work.x[index] -= shift;
  }
  work.spans = Array.from({ length: groups }, () => 0);
  for (let index = 0; index < graph.nodes.length; index += 1) {
    const right = work.x[index] + Math.trunc(graph.nodes[index].span / 2);
    if (right > work.spans[work.group[index]]) work.spans[work.group[index]] = right;
  }
  // One gap between neighbouring groups — see `layout.rs`.
  for (let group = 0; group < groups; group += 1) work.spans[group] += gap;
}

/** Where each branch starts, so the deep view begins them all at one edge. */
function zeroesOf(work, graph) {
  const lanes = maxOf(work.branch, 0) + 1;
  const left = Array.from({ length: lanes }, () => Number.POSITIVE_INFINITY);
  const right = Array.from({ length: lanes }, () => Number.NEGATIVE_INFINITY);
  for (let index = 0; index < graph.nodes.length; index += 1) {
    const lane = work.branch[index];
    left[lane] = Math.min(left[lane], work.x[index]);
    right[lane] = Math.max(right[lane], work.x[index]);
  }
  // Every plane starts at the same edge — see `layout.rs`.
  const out = Array.from({ length: lanes }, () => 0);
  for (let lane = 0; lane < lanes; lane += 1) {
    if (Number.isFinite(left[lane])) out[lane] = left[lane];
  }
  return out;
}

function placesOf(work, graph, view) {
  const offsets = offsetsOf(work, graph, view);
  // Every branch starts at the same left edge — see `layout.rs`.
  const zeroes = zeroesOf(work, graph);
  const places = [];
  for (let index = 0; index < graph.nodes.length; index += 1) {
    const deep = view === "deep";
    const key = deep ? work.cluster[index] : work.group[index];
    // The planes stagger sideways as they recede — see `layout.rs`.
    // Two cells per plane, not one — see `layout.rs`.
    const zero = deep ? zeroes[work.branch[index]] : 0;
    places.push({
      node: index,
      x: half(work.x[index] - zero + offsets[key]),
      y: work.rank[index],
      z: deep ? work.branch[index] : 0,
      rank: work.rank[index],
      cluster: work.cluster[index],
      lane: work.branch[index],
    });
  }
  centreMarks(graph, places);
  return places;
}

/** A mark sits in the middle of the line it interrupts — see `layout.rs`. */
function centreMarks(graph, places) {
  const at = (id) => places.findIndex((place) => graph.nodes[place.node].id === id);
  for (let index = 0; index < places.length; index += 1) {
    if (graph.nodes[places[index].node].piece !== "mark") continue;
    const id = graph.nodes[places[index].node].id;
    const sides = [];
    for (const bond of graph.bonds) {
      const far = bond.from === id ? bond.to : bond.to === id ? bond.from : null;
      if (far === null) continue;
      const other = at(far);
      if (other >= 0 && places[other].rank === places[index].rank && !sides.includes(other)) {
        sides.push(other);
      }
    }
    if (sides.length !== 2) continue;
    const middle = (places[sides[0]].x + places[sides[1]].x) / 2;
    // And not onto somebody — see `layout.rs`.
    const clear = places.every(
      (place, other) =>
        other === index ||
        place.rank !== places[index].rank ||
        graph.nodes[place.node].piece === "mark" ||
        Math.abs(place.x - middle) >= 0.75,
    );
    if (clear) places[index].x = middle;
  }
}

function offsetsOf(work, graph, view) {
  let spans = work.spans.slice();
  if (view === "deep") {
    const clusters = maxOf(work.cluster, -1) + 1;
    const widest = Array.from({ length: clusters }, () => 0);
    for (let index = 0; index < graph.nodes.length; index += 1) {
      const span = work.spans[work.group[index]];
      if (span > widest[work.cluster[index]]) widest[work.cluster[index]] = span;
    }
    spans = widest;
  }
  const offsets = [];
  let cursor = 0;
  for (const span of spans) {
    offsets.push(cursor);
    cursor += span;
  }
  return offsets;
}

/** How much air belongs between two neighbours, in half-cells. */
function airBetween(graph, left, right, gap) {
  // A labelled line needs line to sit on — see `layout.rs`.
  const joined = graph.bonds.some(
    (bond) =>
      bond.label !== "" &&
      ((bond.from === graph.nodes[left].id && bond.to === graph.nodes[right].id) ||
        (bond.from === graph.nodes[right].id && bond.to === graph.nodes[left].id)),
  );
  if (joined) return gap * 2;
  const small = graph.nodes[left].piece === "mark" || graph.nodes[right].piece === "mark";
  return small ? Math.trunc((gap + 1) / 2) : gap;
}

function half(value) {
  return value / 2;
}

function extent(graph, places) {
  let width = 0;
  let height = 0;
  let depth = 0;
  for (const place of places) {
    const halfSpan = half(graph.nodes[place.node].span) / 2;
    width = Math.max(width, place.x + halfSpan);
    height = Math.max(height, place.y);
    depth = Math.max(depth, place.z);
  }
  return [width, height, depth];
}

// --- Route ------------------------------------------------------------------

/** How much of a cell a shy connection leaves — see `layout.rs`. */
const SHY = 0.35;

/** Trims every run end that lands on a figure's own point — see `layout.rs`. */
function shyEnds(graph, places, segments, shy) {
  const points = [];
  for (let index = 0; index < graph.nodes.length; index += 1) {
    const node = graph.nodes[index];
    if (node.piece !== "figure") continue;
    const trim = node.reach >= 0 ? node.reach / 10 : shy ? SHY : null;
    if (trim !== null && trim > 0) points.push([places[index], trim]);
  }
  if (points.length === 0) return;
  const at = (x, y, z) => {
    const hit = points.find(
      ([place]) =>
        Math.abs(place.x - x) <= Number.EPSILON &&
        Math.abs(place.y - y) <= Number.EPSILON &&
        Math.abs(place.z - z) <= Number.EPSILON,
    );
    return hit ? hit[1] : null;
  };
  for (const segment of segments) {
    const dir =
      segment.axis === "x"
        ? [1, 0, 0]
        : segment.axis === "y"
          ? [0, 1, 0]
          : segment.axis === "z"
            ? [0, 0, 1]
            : [segment.basis[0], segment.basis[1], segment.basis[2]];
    const far = [
      segment.len * dir[0] + segment.x,
      segment.len * dir[1] + segment.y,
      segment.len * dir[2] + segment.z,
    ];
    // Never below a stub — see `layout.rs`.
    const near = at(segment.x, segment.y, segment.z);
    if (near !== null && segment.len > near + 0.05) {
      segment.x += near * dir[0];
      segment.y += near * dir[1];
      segment.z += near * dir[2];
      segment.len -= near;
    }
    const tail = at(far[0], far[1], far[2]);
    if (tail !== null && segment.len > tail + 0.05) {
      segment.len -= tail;
    }
  }
}

function routeAll(graph, places, defaultRoute) {
  const laneOf = lanesOf(graph, places);
  const segments = [];
  for (let index = 0; index < graph.bonds.length; index += 1) {
    const bond = graph.bonds[index];
    const from = places[indexOf(graph, bond.from)];
    const to = places[indexOf(graph, bond.to)];
    const taut =
      bond.shape === "lateral" ||
      bond.route === "taut" ||
      (bond.route === "" && defaultRoute === "taut");
    const offset = laneOf[index] * 0.15;
    if (taut) segments.push(straight(index, from, to));
    else if (bond.shape === "union") {
      if (from.rank === to.rank) {
        for (const run of across(index, from, to)) segments.push(run);
      } else {
        // A union across generations goes around the outside — see `layout.rs`.
        for (const run of around(index, from, to, places, offset)) segments.push(run);
      }
    } else for (const run of elbow(index, from, to, offset)) segments.push(run);
  }
  return segments;
}

/** Which lane each bond's channel takes — see `layout.rs`. */
function lanesOf(graph, places) {
  const families = [];
  const belongs = Array.from({ length: graph.bonds.length }, () => null);
  for (let index = 0; index < graph.bonds.length; index += 1) {
    const bond = graph.bonds[index];
    if (bond.shape !== "descent") continue;
    const from = places[indexOf(graph, bond.from)];
    const to = places[indexOf(graph, bond.to)];
    if (from.rank === to.rank) continue;
    const upper = from.rank <= to.rank ? from : to;
    const lower = upper === from ? to : from;
    const upperId = from.rank <= to.rank ? bond.from : bond.to;
    const left = Math.min(upper.x, lower.x);
    const right = Math.max(upper.x, lower.x);
    const found = families.findIndex(
      (family) => family.rank === lower.rank && family.id === upperId,
    );
    if (found >= 0) {
      families[found].left = Math.min(families[found].left, left);
      families[found].right = Math.max(families[found].right, right);
      belongs[index] = found;
    } else {
      families.push({ rank: lower.rank, id: upperId, left, right });
      belongs[index] = families.length - 1;
    }
  }
  const laneOfFamily = Array.from({ length: families.length }, () => 0);
  for (let at = 0; at < families.length; at += 1) {
    let lane = 0;
    for (;;) {
      let clash = false;
      for (let other = 0; other < at; other += 1) {
        if (
          families[other].rank === families[at].rank &&
          laneOfFamily[other] === lane &&
          families[other].left <= families[at].right &&
          families[at].left <= families[other].right
        ) {
          clash = true;
          break;
        }
      }
      if (!clash) break;
      lane += 1;
    }
    laneOfFamily[at] = lane;
  }
  return belongs.map((at) => (at === null ? 0 : laneOfFamily[at]));
}

/** The outside elbow a spanning union takes — see `layout.rs`. */
function around(bond, from, to, places, offset) {
  const upper = from.rank <= to.rank ? from : to;
  const lower = upper === from ? to : from;
  let edge = 0;
  for (const place of places) {
    if (place.cluster === upper.cluster && place.x > edge) edge = place.x;
  }
  edge += 0.75;
  const top = upper.rank + 0.55 - offset;
  const bottom = lower.rank - 0.45 + offset;
  return [
    run(bond, upper.x, upper.y, upper.z, top - upper.y, "y"),
    run(bond, upper.x, top, upper.z, edge - upper.x, "x"),
    run(bond, edge, top, upper.z, bottom - top, "y"),
    run(bond, Math.min(lower.x, edge), bottom, upper.z, edge - lower.x, "x"),
    run(bond, lower.x, bottom, lower.z, lower.y - bottom, "y"),
  ];
}

/** Down, across, through, down: the bracket every family tree is drawn with. */
function elbow(bond, from, to, offset) {
  const upper = from.y <= to.y ? from : to;
  const lower = upper === from ? to : from;
  // Just above the row it descends into; the lane offset goes DOWN — see
  // `layout.rs`.
  const channel = lower.rank - 0.45 + offset;
  if (Math.abs(lower.x - upper.x) <= Number.EPSILON && Math.abs(lower.z - upper.z) <= Number.EPSILON) {
    return [run(bond, upper.x, upper.y, upper.z, lower.y - upper.y, "y")];
  }
  const runs = [run(bond, upper.x, upper.y, upper.z, channel - upper.y, "y")];
  if (Math.abs(lower.x - upper.x) > Number.EPSILON) {
    runs.push(
      run(bond, Math.min(upper.x, lower.x), channel, upper.z, Math.abs(lower.x - upper.x), "x"),
    );
  }
  if (Math.abs(lower.z - upper.z) > Number.EPSILON) {
    runs.push(
      run(bond, lower.x, channel, Math.min(upper.z, lower.z), Math.abs(lower.z - upper.z), "z"),
    );
  }
  runs.push(run(bond, lower.x, channel, lower.z, lower.y - channel, "y"));
  return runs;
}

/** One run along the row, for two partners who share it. */
function across(bond, from, to) {
  const runs = [
    run(bond, Math.min(from.x, to.x), from.y, from.z, Math.abs(to.x - from.x), "x"),
  ];
  if (Math.abs(to.z - from.z) > Number.EPSILON) {
    runs.push(run(bond, to.x, from.y, Math.min(from.z, to.z), Math.abs(to.z - from.z), "z"));
  }
  return runs;
}

/** One straight run, carrying the rotation that turns its own +x onto it. */
function straight(bond, from, to) {
  const dx = to.x - from.x;
  const dy = to.y - from.y;
  const dz = to.z - from.z;
  const len = Math.sqrt(dx * dx + dy * dy + dz * dz);
  const segment = run(bond, from.x, from.y, from.z, len, "free");
  segment.basis = basis(dx, dy, dz, len);
  return segment;
}

function run(bond, x, y, z, len, axis) {
  return { bond, x, y, z, len: Math.max(len, 0), axis, basis: IDENTITY.slice() };
}

/**
 * A rotation taking `+x` onto `(dx, dy, dz)`, column-major.
 *
 * Gram-Schmidt from the direction and a helper, so the only transcendental
 * involved is `sqrt` — exactly rounded here and in Rust. `atan2` would have
 * been the obvious way and is not required to agree between the two.
 */
function basis(dx, dy, dz, len) {
  if (len <= Number.EPSILON) return IDENTITY.slice();
  const x = [dx / len, dy / len, dz / len];
  const helper = Math.abs(x[2]) > 0.9 ? [0, 1, 0] : [0, 0, 1];
  const y = cross(helper, x);
  const scale = Math.sqrt(y[0] * y[0] + y[1] * y[1] + y[2] * y[2]);
  if (scale <= Number.EPSILON) return IDENTITY.slice();
  y[0] /= scale;
  y[1] /= scale;
  y[2] /= scale;
  const z = cross(x, y);
  return [x[0], x[1], x[2], y[0], y[1], y[2], z[0], z[1], z[2]];
}

function cross(a, b) {
  return [
    a[1] * b[2] - a[2] * b[1],
    a[2] * b[0] - a[0] * b[2],
    a[0] * b[1] - a[1] * b[0],
  ];
}

// --- Draw -------------------------------------------------------------------
//
// The twin of `draw.rs`. Everything a renderer needs, worked out once, so that
// the two component bodies — one in `rsx!` and one in `createElement` — are
// dumb walkers rather than two copies of a decision.

/**
 * Whether a word may be interpolated into a custom property name.
 *
 * A SECURITY CHECK, NOT A TIDINESS ONE. An occasion and a set are props, so
 * neither has been through the parser's identifier rule, and both end up in a
 * property name inside a `style` attribute. Fail closed.
 */
export function nameable(value) {
  return typeof value === "string" && value.length > 0 && value.length <= ID_LIMIT
    && /^[A-Za-z0-9_-]+$/u.test(value);
}

/** A `group:name` set, as the two words that make a property name. */
export function setParts(set) {
  if (typeof set !== "string") return null;
  const at = set.indexOf(":");
  if (at < 0) return null;
  const group = set.slice(0, at);
  const name = set.slice(at + 1);
  return nameable(group) && nameable(name) ? [group, name] : null;
}

/** The name a piece is drawn with: its label, or its id when it has none. */
export function shownName(node) {
  return node.label === "" ? node.id : node.label;
}

/** Which set of each group is showing. */
function chosen(graph, sets) {
  const active = [];
  const claim = (entry) => {
    const parts = setParts(entry);
    if (parts && !active.some(([group]) => group === parts[0])) active.push(parts);
  };
  for (const entry of sets.split(/\s+/u)) claim(entry);
  for (const node of graph.nodes) claim(node.set);
  for (const bond of graph.bonds) claim(bond.set);
  return active;
}

/** Whether a thing carrying this set is the one showing. */
function showing(set, active) {
  const parts = setParts(set);
  if (parts === null) return true;
  return active.some(([group, name]) => group === parts[0] && name === parts[1]);
}

/**
 * Everything one piece is joined to, in declaration order.
 *
 * It sees through an unlabelled mark — a link announcing "u0" says nothing —
 * and direction is a rank comparison rather than a bond direction, because a
 * bond points from a parent to a union and from that union to a child. See
 * `draw.rs`, which argues both.
 */
function tiesOf(laid, id) {
  const rankOf = (other) => {
    const place = laid.places.find((p) => laid.graph.nodes[p.node].id === other);
    return place === undefined ? null : place.rank;
  };
  const nodeOf = (other) => laid.graph.nodes.find((node) => node.id === other);
  const mine = rankOf(id) ?? 0;
  const otherEnd = (bond, from) => {
    if (bond.from === from) return bond.to;
    if (bond.to === from) return bond.from;
    return null;
  };

  const out = [];
  for (const bond of laid.graph.bonds) {
    const next = otherEnd(bond, id);
    if (next === null) continue;
    const through = nodeOf(next);
    let reached;
    if (through !== undefined && through.piece === "mark" && through.label === "") {
      reached = [];
      for (const onward of laid.graph.bonds) {
        const far = otherEnd(onward, next);
        if (far === null || far === id) continue;
        // Which of the two bonds describes the pair — see `draw.rs`.
        const plain = bond.shape === "union" && onward.shape !== "union";
        const kind = plain ? onward.kind : bond.kind;
        let note = plain ? onward.label : bond.label;
        if (note === "") note = plain ? bond.label : onward.label;
        reached.push([far, kind, note]);
      }
    } else {
      reached = [[next, bond.kind, bond.label]];
    }
    for (const [far, kind, note] of reached) {
      const node = nodeOf(far);
      if (node === undefined || out.some((tie) => tie.id === far)) continue;
      const rank = rankOf(far);
      out.push({ id: far, label: shownName(node), kind, note, upward: rank !== null && rank < mine });
    }
  }
  return out;
}

/**
 * The subgraph `isolate` draws — see `draw.rs`.
 *
 * @param {any} graph
 * @param {string} filter
 */
function prune(graph, filter) {
  const kept = graph.nodes.map((node) => answers(graph, node, filter));
  const at = (id) => graph.nodes.findIndex((node) => node.id === id);

  const seen = graph.nodes.map(() => false);
  const markers = [];
  const standIn = graph.nodes.map(() => null);
  for (let index = 0; index < graph.nodes.length; index += 1) {
    if (kept[index] || seen[index]) continue;
    const component = [index];
    seen[index] = true;
    for (let walk = 0; walk < component.length; walk += 1) {
      const here = graph.nodes[component[walk]].id;
      for (const bond of graph.bonds) {
        const far = bond.from === here ? bond.to : bond.to === here ? bond.from : null;
        if (far === null) continue;
        const other = at(far);
        if (other >= 0 && !kept[other] && !seen[other]) {
          seen[other] = true;
          component.push(other);
        }
      }
    }
    const count = component.filter(
      (member) => graph.nodes[member].piece === "figure",
    ).length;
    if (count === 0) continue;
    const marker = `${graph.nodes[component[0]].id}-elided`;
    markers.push({
      ...blank(marker, "figure"),
      kind: "elided",
      elided: count,
      cluster: graph.nodes[component[0]].cluster,
    });
    for (const member of component) standIn[member] = marker;
  }

  const nodes = graph.nodes.filter((_, index) => kept[index]);
  nodes.push(...markers);

  const bonds = [];
  for (const bond of graph.bonds) {
    const mapEnd = (id) => {
      const index = at(id);
      if (index < 0) return id;
      if (kept[index]) return id;
      return standIn[index];
    };
    const from = mapEnd(bond.from);
    const to = mapEnd(bond.to);
    if (from === null || to === null || from === to) continue;
    if (bonds.some((prior) => prior.from === from && prior.to === to && prior.kind === bond.kind)) {
      continue;
    }
    bonds.push({ ...bond, from, to });
  }

  return { nodes, bonds, skipped: graph.skipped };
}

/** The filter, trimmed the way `matches` trims it. */
function trimmedFilter(filter) {
  let start = 0;
  let stop = filter.length;
  const blank_ = (ch) => ch === " " || ch === "\t" || ch === "\n" || ch === "\r";
  while (start < stop && blank_(filter[start])) start += 1;
  while (stop > start && blank_(filter[stop - 1])) stop -= 1;
  return filter.slice(start, stop);
}

/**
 * Whether a piece answers the filter — see `draw.rs` for why a union point
 * answers when the people it joins do.
 *
 * @param {any} graph
 * @param {any} node
 * @param {string} filter
 */
function answers(graph, node, filter) {
  if (node.piece !== "mark" || node.label !== "") return matches(node, filter);
  return graph.bonds.some((bond) => {
    const far = bond.from === node.id ? bond.to : bond.to === node.id ? bond.from : null;
    if (far === null) return false;
    return graph.nodes.some((other) => other.id === far && matches(other, filter));
  });
}

/**
 * Reads a graph, places it, and works out everything a renderer needs.
 *
 * @param {string} source
 * @param {{view?: string, route?: string}} options
 * @param {string} filter
 * @param {string} sets
 */
export function draw(source, options, filter, sets) {
  // `isolate` removes, so the layout must not remember — see `draw.rs`.
  let parsed = parse(source);
  const wanted = filter || "";
  if (options && options.isolate && trimmedFilter(wanted) !== "") {
    parsed = prune(parsed, wanted);
  }
  const laid = place(parsed, options);
  const active = chosen(laid.graph, sets || "");

  const pieces = laid.places.map((place) => {
    const node = laid.graph.nodes[place.node];
    return {
      id: node.id,
      mark: node.piece === "mark",
      slot: node.slot,
      // The name as it is drawn — see `draw.rs` for why a figure falls back to
      // its id and a mark does not.
      label: node.piece === "mark" ? node.label : shownName(node),
      meta: node.meta,
      image: node.image,
      href: node.href,
      kind: node.kind,
      tone: node.tone,
      shape: node.shape,
      set: node.set,
      x: place.x,
      y: place.y,
      z: place.z,
      span: node.span / 2,
      reach: node.reach / 10,
      matched: node.elided > 0 || answers(laid.graph, node, wanted),
      shown: showing(node.set, active),
      ties: tiesOf(laid, node.id),
      elided: node.elided,
    };
  });
  const matched = pieces.filter(
    (piece) => piece.matched && !piece.mark && piece.elided === 0,
  ).length;

  const bonds = laid.graph.bonds.map((bond, index) => {
    const from = pieces.find((piece) => piece.id === bond.from);
    const to = pieces.find((piece) => piece.id === bond.to);
    const ends = (pick) => ((from === undefined ? 0 : pick(from)) + (to === undefined ? 0 : pick(to))) / 2;
    return {
      index,
      kind: bond.kind,
      tone: bond.tone,
      dash: bond.dash,
      set: bond.set,
      label: bond.label,
      note: bond.note,
      from: bond.from,
      to: bond.to,
      matched: (from === undefined || from.matched) && (to === undefined || to.matched),
      // Whether the label goes just below its line — see `draw.rs`.
      tagBelow:
        from !== undefined &&
        to !== undefined &&
        from.y === to.y &&
        Math.abs(from.x - to.x) - (from.span + to.span) / 2 < 1,
      // A connection is only shown when both its ends are — see `draw.rs`.
      shown:
        showing(bond.set, active) &&
        (from === undefined || from.shown) &&
        (to === undefined || to.shown),
      // On a row line the label centres on the free stretch — see `draw.rs`.
      tag: (() => {
        if (from !== undefined && to !== undefined && from.y === to.y) {
          const left = from.x <= to.x ? from : to;
          const right = left === from ? to : from;
          return [
            ((left.x + left.span / 2) + (right.x - right.span / 2)) / 2,
            from.y,
            from.z,
          ];
        }
        const channel = laid.segments.find((run) => run.bond === index && run.axis === "x");
        if (channel) return [channel.x + channel.len / 2, channel.y, channel.z];
        return [ends((p) => p.x), ends((p) => p.y), ends((p) => p.z)];
      })(),
      runs: laid.segments.filter((run) => run.bond === index),
    };
  });

  return {
    pieces,
    bonds,
    width: laid.width,
    height: laid.height,
    depth: laid.depth,
    crossings: laid.crossings,
    skipped: laid.skipped,
    broken: laid.broken,
    matched,
  };
}

/**
 * An attribute value, or nothing at all when the author said nothing.
 *
 * An attribute present with an empty value is an attribute a selector matches —
 * `[data-nd-tone]` would be every piece in the scene.
 */
export function some(value) {
  return value === "" || value === undefined || value === null ? undefined : value;
}
