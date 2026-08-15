//! Four passes: rank, group, order, place. Then the connections are routed.
//!
//! # Rank — who is above whom
//!
//! Longest path over the descent-shaped bonds, with an explicit `rank=` acting
//! as a floor rather than an override, so relaxation only ever pushes a piece
//! **down** and therefore terminates.
//!
//! A union is a *soft* constraint: partners sit on one row where they can. The
//! rule is one sentence, and the user's hardest case is why it is worded this
//! way — *the shallower partner is pushed down to meet the deeper one, unless
//! it is an ancestor of the deeper one.* A grandparent married to their own
//! grandchild is an ancestor, so nothing moves and the bond simply spans two
//! rows; a spouse who married in is not, so they move up beside their partner.
//! Pushing an ancestor down would push their descendants down with them, and
//! the grandchild among them — forever.
//!
//! # Group — several trees in one space, and what depth is for
//!
//! Pieces that reach one another belong to one **cluster**, and clusters are
//! set well apart along `x`. Inside a cluster, every piece below the second row
//! belongs to the **branch** it descends from — one child of the founding
//! couple, one department of the company.
//!
//! That is the whole difference between the two views. Flat lays the branches
//! out along `x`, which is how a family tree has always been drawn. Deep stacks
//! them along `z`, so **a branch is a plane**: a generation forty figures wide
//! becomes four planes of ten receding into the scene, and the connections
//! between the trunk and each branch are what travel in depth. One rule, one
//! code path, and a shape a reader can name.
//!
//! # Order — the crossings
//!
//! The median heuristic, swept down and up, then adjacent transposition, then a
//! pass that pulls partners together where doing so costs no crossing. The best
//! ordering seen is kept, measured by [`Scene::crossings`] — which is reported
//! rather than hidden, because "the ordering reduces crossings" is a claim, and
//! an unmeasured claim stops being true without anybody noticing.
//!
//! # Place — integers all the way
//!
//! Every coordinate is an `i32` count of **half-cells** until the moment it is
//! handed out, when it is halved. A half-cell exists so a mark can be half the
//! width of a figure without introducing a fraction, and integers exist so this
//! file and `layout.js` cannot drift apart by an ulp.
//!
//! # Long connections are drawn taut, and there are no dummy rows
//!
//! A bond that spans more than one row — a union across generations, an
//! explicit `rank=` gap — is drawn as one straight run instead of an elbow.
//! The alternative is a dummy piece on every row it crosses, which is the
//! textbook answer and buys a tidier line for a whole class of ordering and
//! routing bugs. A diagonal reads as *this connection is unusual*, which it is.

use crate::{Axis, Bond, Graph, Node, Options, Piece, Place, Route, Scene, Segment, Shape, View};

/// How many times the ordering is swept down and up.
///
/// Eight is where the crossing count stops improving on every case in
/// `spec/lineage/cases.toml`, and the sweep is bounded rather than run to a
/// fixed point because a fixed point is not guaranteed to exist.
const SWEEPS: usize = 8;

/// Half-cells in a cell.
const CELL: i32 = 2;

/// Ranks are relaxed at most this many times before the answer is taken as it
/// stands. Ranks only ever increase, so this bound is a safety net rather than
/// a policy: a graph would need this many chained pushes to reach it.
const RELAX: usize = 64;

/// Parses nothing and places everything. See the module documentation.
#[must_use]
#[allow(clippy::needless_pass_by_value)] // The graph is consumed into the Scene.
pub fn place(graph: Graph, options: Options) -> Scene {
    let mut skipped = graph.skipped;
    let mut graph = tidy(graph, &mut skipped);
    let mut work = Work::new(&graph);

    adopt(&mut graph, &mut work);
    let broken = work.rank_everything(&graph);
    work.group_everything(&graph);
    // Counted here, while a group is still a whole cluster. Once the deep view
    // has made a group mean a branch, a crossing between two branches is not a
    // crossing any more — they are on different planes — and reporting the
    // smaller number would be reporting a different question.
    let crossings = work.order_everything(&graph);
    work.place_everything(&graph, options);

    let places = work.places(&graph, options.view);
    let segments = route(&graph, &places, options);
    let (width, height, depth) = extent(&graph, &places);

    Scene {
        graph,
        places,
        segments,
        width,
        height,
        depth,
        crossings,
        skipped,
        broken,
    }
}

/// Drops what cannot be drawn: a second piece with an id already taken, and a
/// bond naming an id nobody declared. Both are counted, never silenced.
fn tidy(graph: Graph, skipped: &mut usize) -> Graph {
    let mut nodes: Vec<Node> = Vec::with_capacity(graph.nodes.len());
    for node in graph.nodes {
        if nodes.iter().any(|kept| kept.id == node.id) {
            *skipped += 1;
        } else {
            nodes.push(node);
        }
    }
    let mut bonds = Vec::with_capacity(graph.bonds.len());
    for bond in graph.bonds {
        let known = |id: &str| nodes.iter().any(|node| node.id == id);
        if known(&bond.from) && known(&bond.to) {
            bonds.push(bond);
        } else {
            *skipped += 1;
        }
    }
    Graph {
        nodes,
        bonds,
        skipped: graph.skipped,
    }
}

/// Everything the passes write down, indexed by node.
struct Work {
    rank: Vec<i32>,
    cluster: Vec<usize>,
    branch: Vec<usize>,
    /// Scratch for [`Self::separate`]: the row's positions before the spread,
    /// so the spread can restore the row's mean.
    kept_x: Vec<i32>,
    group: Vec<usize>,
    order: Vec<i32>,
    x: Vec<i32>,
    parents: Vec<Vec<usize>>,
    children: Vec<Vec<usize>>,
    partners: Vec<Vec<usize>>,
    /// How wide a group is, in half-cells, once it has been placed.
    spans: Vec<i32>,
}

impl Work {
    fn new(graph: &Graph) -> Self {
        let n = graph.nodes.len();
        Self {
            rank: vec![0; n],
            cluster: (0..n).collect(),
            branch: vec![0; n],
            kept_x: vec![0; n],
            group: vec![0; n],
            order: vec![0; n],
            x: vec![0; n],
            parents: vec![Vec::new(); n],
            children: vec![Vec::new(); n],
            partners: vec![Vec::new(); n],
            spans: Vec::new(),
        }
    }

    /// The index of an id, which every pass below can rely on existing because
    /// [`tidy`] has already dropped the bonds that named anything else.
    fn of(graph: &Graph, id: &str) -> usize {
        graph
            .nodes
            .iter()
            .position(|node| node.id == id)
            .unwrap_or(0)
    }
}

/// Hangs everything parentless from the origin of its own declared cluster.
///
/// This is what makes the top of the tree stay the top: an author adds forty
/// figures and never touches the origin, and every one of them that nobody
/// declared a parent for arrives underneath it.
fn adopt(graph: &mut Graph, work: &mut Work) {
    let origins: Vec<usize> = (0..graph.nodes.len())
        .filter(|&index| graph.nodes[index].origin)
        .collect();
    if origins.is_empty() {
        return;
    }
    let parented: Vec<bool> = (0..graph.nodes.len())
        .map(|index| {
            let id = &graph.nodes[index].id;
            graph
                .bonds
                .iter()
                .any(|bond| bond.shape == Shape::Descent && &bond.to == id)
        })
        .collect();

    // ONE OF A COUPLE, NOT BOTH.
    //
    // Everybody without a parent used to arrive under the origin, so a mythic
    // ancestor was drawn as the ancestor of the people her descendants married
    // — reported as *the owl must be joined to one member of the family, not to
    // the partner*. Refusing every married figure is the opposite mistake: two
    // founders who married each other are both married, refusing both leaves
    // the origin joined to nobody, and an origin with no children does not sit
    // above anybody — it lands on their row.
    //
    // So: declaration order, and a figure whose partner has already been taken
    // is left to their partner. One line, to one person, and the couple stays
    // under the origin where a reader expects it.
    for origin in origins {
        let mut taken: Vec<String> = Vec::new();
        let mut adopted: Vec<String> = Vec::new();
        for (index, node) in graph.nodes.iter().enumerate() {
            if index == origin
                || node.origin
                || parented[index]
                || node.piece != Piece::Figure
                || node.cluster != graph.nodes[origin].cluster
            {
                continue;
            }
            // THROUGH THE MARK, because that is where a marriage lives: a
            // couple is usually `ada—u0—eve`, so a direct-bond check saw no
            // partnership at all and the origin adopted both — the owl grew a
            // second line, to the person who married in.
            let partner_of = |id: &str, other: &str| -> bool {
                graph.bonds.iter().any(|bond| {
                    bond.shape == Shape::Union
                        && ((bond.from == id && bond.to == other)
                            || (bond.to == id && bond.from == other))
                })
            };
            let partnered = taken.iter().any(|held| {
                partner_of(&node.id, held)
                    || graph.nodes.iter().any(|mark| {
                        mark.piece == Piece::Mark
                            && partner_of(&node.id, &mark.id)
                            && partner_of(held, &mark.id)
                    })
            });
            if partnered {
                continue;
            }
            taken.push(node.id.clone());
            adopted.push(node.id.clone());
        }
        for id in adopted {
            graph.bonds.push(Bond {
                from: graph.nodes[origin].id.clone(),
                to: id,
                kind: "origin".to_owned(),
                shape: Shape::Descent,
                ..Bond::default()
            });
        }
    }
    let n = graph.nodes.len();
    work.parents = vec![Vec::new(); n];
    work.children = vec![Vec::new(); n];
}

// --- Rank -------------------------------------------------------------------

impl Work {
    /// Fills [`Work::rank`] and returns how many bonds were dropped to break a
    /// cycle.
    fn rank_everything(&mut self, graph: &Graph) -> usize {
        let mut edges: Vec<(usize, usize)> = Vec::new();
        let mut unions: Vec<(usize, usize)> = Vec::new();
        for bond in &graph.bonds {
            let from = Work::of(graph, &bond.from);
            let to = Work::of(graph, &bond.to);
            match bond.shape {
                Shape::Descent => edges.push((from, to)),
                Shape::Union => unions.push((from, to)),
                Shape::Lateral => {}
            }
        }
        let broken = break_cycles(graph.nodes.len(), &mut edges);

        for (index, node) in graph.nodes.iter().enumerate() {
            self.rank[index] = node.rank.unwrap_or(0);
        }
        relax(&mut self.rank, &edges);
        for &(a, b) in &unions {
            self.marry(a, b, &edges);
            self.partners[a].push(b);
            self.partners[b].push(a);
        }
        for (index, (parents, children)) in self
            .parents
            .iter_mut()
            .zip(self.children.iter_mut())
            .enumerate()
        {
            for &(from, to) in &edges {
                if to == index {
                    parents.push(from);
                }
                if from == index {
                    children.push(to);
                }
            }
        }
        let floor = self.rank.iter().copied().min().unwrap_or(0);
        for rank in &mut self.rank {
            *rank -= floor;
        }
        broken
    }

    /// Pulls the shallower partner down to the deeper one, unless it is an
    /// ancestor of it. The module documentation argues the exception.
    fn marry(&mut self, a: usize, b: usize, edges: &[(usize, usize)]) {
        let (shallow, deep) = if self.rank[a] <= self.rank[b] {
            (a, b)
        } else {
            (b, a)
        };
        if self.rank[shallow] == self.rank[deep] || reaches(edges, shallow, deep) {
            return;
        }
        self.rank[shallow] = self.rank[deep];
        relax(&mut self.rank, edges);
    }
}

/// Removes the back edges a depth-first walk finds, so the rest is acyclic.
///
/// A cycle is an author saying X descends from Y descends from X. Every figure
/// is kept; the bond that closed the loop is what goes, and the count reaches
/// the DOM so an author can see that it happened.
fn break_cycles(n: usize, edges: &mut Vec<(usize, usize)>) -> usize {
    let mut colour = vec![0_u8; n];
    let mut keep = vec![true; edges.len()];
    let mut broken = 0;
    for start in 0..n {
        if colour[start] != 0 {
            continue;
        }
        // An explicit stack, because a genealogy deep enough to overflow a
        // recursive walk is a graph somebody generated rather than wrote.
        let mut stack = vec![(start, 0_usize)];
        colour[start] = 1;
        while let Some(&mut (node, ref mut cursor)) = stack.last_mut() {
            let next = edges
                .iter()
                .enumerate()
                .skip(*cursor)
                .find(|&(_, &(from, _))| from == node);
            if let Some((at, &(_, to))) = next {
                *cursor = at + 1;
                if colour[to] == 1 {
                    keep[at] = false;
                    broken += 1;
                } else if colour[to] == 0 {
                    colour[to] = 1;
                    stack.push((to, 0));
                }
            } else {
                colour[node] = 2;
                stack.pop();
            }
        }
    }
    let mut index = 0;
    edges.retain(|_| {
        index += 1;
        keep[index - 1]
    });
    broken
}

/// Pushes every child below its parents, repeatedly. Ranks only increase.
fn relax(rank: &mut [i32], edges: &[(usize, usize)]) {
    for _ in 0..RELAX {
        let mut moved = false;
        for &(from, to) in edges {
            if rank[to] < rank[from] + 1 {
                rank[to] = rank[from] + 1;
                moved = true;
            }
        }
        if !moved {
            return;
        }
    }
}

/// Whether `from` reaches `to` by descent.
fn reaches(edges: &[(usize, usize)], from: usize, to: usize) -> bool {
    let mut seen = vec![from];
    let mut cursor = 0;
    while cursor < seen.len() {
        let node = seen[cursor];
        cursor += 1;
        for &(a, b) in edges {
            if a == node && b == to {
                return true;
            }
            if a == node && !seen.contains(&b) {
                seen.push(b);
            }
        }
    }
    false
}

// --- Group ------------------------------------------------------------------

impl Work {
    /// Clusters (what reaches what) and branches (what descends from what).
    fn group_everything(&mut self, graph: &Graph) {
        for bond in &graph.bonds {
            let a = Work::of(graph, &bond.from);
            let b = Work::of(graph, &bond.to);
            self.join(a, b);
        }
        for (index, node) in graph.nodes.iter().enumerate() {
            if node.cluster.is_empty() {
                continue;
            }
            if let Some(first) = graph
                .nodes
                .iter()
                .position(|other| other.cluster == node.cluster)
            {
                self.join(first, index);
            }
        }
        self.compact_clusters();
        self.find_branches();
    }

    /// Union-find without the find: the sets are tiny and the flat walk is the
    /// same in both languages, which is worth more here than an optimisation.
    fn join(&mut self, a: usize, b: usize) {
        let (from, to) = (self.cluster[a], self.cluster[b]);
        if from == to {
            return;
        }
        for slot in &mut self.cluster {
            if *slot == from {
                *slot = to;
            }
        }
    }

    /// Renumbers clusters by the order they were first mentioned, so the
    /// leftmost tree is the one declared first.
    fn compact_clusters(&mut self) {
        let mut seen: Vec<usize> = Vec::new();
        for index in 0..self.cluster.len() {
            let key = self.cluster[index];
            let at = seen.iter().position(|&s| s == key).unwrap_or_else(|| {
                seen.push(key);
                seen.len() - 1
            });
            self.cluster[index] = at;
        }
    }

    /// A branch is everything descending from one **child of the trunk**.
    /// Everything else is branch zero, which is the trunk itself.
    ///
    /// Seeded on a child rather than on a row, and that distinction is the
    /// whole of it: a spouse who married in stands on the second row too, and
    /// giving them a plane of their own would put a couple on two planes and
    /// their children on a third. So a seed is a figure on the second row **who
    /// has a parent on the first**, and everybody else — spouses, union marks,
    /// grandchildren — inherits from a parent, or failing that from a partner.
    fn find_branches(&mut self) {
        let clusters = self.cluster.iter().copied().max().map_or(0, |max| max + 1);
        let mut next = 1;
        for cluster in 0..clusters {
            let top = (0..self.rank.len())
                .filter(|&index| self.cluster[index] == cluster)
                .map(|index| self.rank[index])
                .min()
                .unwrap_or(0);
            for index in 0..self.rank.len() {
                let child_of_the_trunk = self.parents[index]
                    .iter()
                    .any(|&parent| self.rank[parent] == top);
                if self.cluster[index] == cluster
                    && self.rank[index] == top + 1
                    && child_of_the_trunk
                {
                    self.branch[index] = next;
                    next += 1;
                }
            }
        }
        // Then outward, taking the first parent's branch and falling back to
        // the first partner's. Both lists are in declaration order, so the
        // answer is the same in both languages every time.
        for _ in 0..RELAX {
            let mut moved = false;
            for index in 0..self.branch.len() {
                if self.branch[index] != 0 {
                    continue;
                }
                let inherited = self.parents[index]
                    .iter()
                    .chain(&self.partners[index])
                    .find(|&&other| self.branch[other] != 0)
                    .copied();
                if let Some(other) = inherited {
                    self.branch[index] = self.branch[other];
                    moved = true;
                }
            }
            if !moved {
                break;
            }
        }
    }
}

// --- Order ------------------------------------------------------------------

impl Work {
    /// Chooses the group each piece is laid out inside, then orders each row.
    /// Returns how many crossings the ordering it settled on leaves behind.
    fn order_everything(&mut self, graph: &Graph) -> usize {
        for index in 0..graph.nodes.len() {
            self.group[index] = self.cluster[index];
        }
        for index in 0..graph.nodes.len() {
            self.order[index] = i32::try_from(index).unwrap_or(i32::MAX);
        }
        let mut best = self.crossings(graph);
        let mut kept = self.order.clone();
        for sweep in 0..SWEEPS {
            self.sweep(graph, sweep % 2 == 0);
            let now = self.crossings(graph);
            if now < best {
                best = now;
                kept.clone_from(&self.order);
            }
        }
        self.order = kept;
        let best = self.exact(graph, best);
        let best = self.transpose(graph, best);
        self.pull_partners(graph, best);
        self.seat_marks(graph);
        self.flank_marks(graph);
        self.crossings(graph)
    }

    /// Closes the row around every union mark: its nearest partner on each
    /// side stands flush against it.
    ///
    /// [`Self::seat_marks`] puts the mark BETWEEN its partners; this puts the
    /// partners AGAINST the mark. Without it, whoever the ordering left
    /// standing between a mark and its partner had the union line drawn
    /// straight through their card — Eve's sister, between Ada's union and
    /// Eve, read as the person Ada had married. The reader's rule is absolute
    /// — a line never passes through a card that is not its destination — and
    /// it outranks the crossing count, which is measured after and reported
    /// as whatever it then is.
    fn flank_marks(&mut self, graph: &Graph) {
        let groups = self.group.iter().copied().max().map_or(0, |max| max + 1);
        for group in 0..groups {
            let mut ranks: Vec<i32> = (0..graph.nodes.len())
                .filter(|&index| self.group[index] == group)
                .map(|index| self.rank[index])
                .collect();
            ranks.sort_unstable();
            ranks.dedup();
            for rank in ranks {
                let mut sequence = self.row(graph, group, rank);
                let mut orders: Vec<i32> =
                    sequence.iter().map(|&index| self.order[index]).collect();
                orders.sort_unstable();
                let marks: Vec<usize> = sequence
                    .iter()
                    .copied()
                    .filter(|&index| graph.nodes[index].piece == Piece::Mark)
                    .collect();
                for mark in marks {
                    for side in [-1_i64, 1] {
                        let at = sequence
                            .iter()
                            .position(|&index| index == mark)
                            .expect("the mark is on its own row");
                        let nearest = self.partners[mark]
                            .iter()
                            .copied()
                            .filter(|&p| self.rank[p] == rank && self.group[p] == group)
                            .filter_map(|p| {
                                let pos = sequence.iter().position(|&index| index == p)?;
                                let offset = i64::try_from(pos).unwrap_or(0)
                                    - i64::try_from(at).unwrap_or(0);
                                (offset.signum() == side).then_some((offset.abs(), pos))
                            })
                            .min();
                        let Some((_, pos)) = nearest else { continue };
                        let partner = sequence.remove(pos);
                        let seat = sequence
                            .iter()
                            .position(|&index| index == mark)
                            .expect("still there");
                        let seat = if side < 0 { seat } else { seat + 1 };
                        sequence.insert(seat, partner);
                    }
                }
                for (slot, &index) in sequence.iter().enumerate() {
                    self.order[index] = orders[slot];
                }
            }
        }
    }

    /// Seats every union mark between its partners on its own row.
    ///
    /// The crossing passes have no opinion about a mark along a row — a
    /// same-row bond crosses nothing they count — so a mark stayed wherever
    /// declaration order left it, which for the family on the front page was
    /// AFTER both partners. Everything then agreed with everything else except
    /// the picture: the children centred under the mark's laid-out position,
    /// off past the second partner, while the dot itself was drawn back on the
    /// stretch between the two — a bracket two cells right of the point that
    /// descends into it. The mark's key is the mean of its same-row partners'
    /// orders, compared as a fraction so the integers stay integers, and the
    /// row's own order values are dealt back out in the new sequence.
    fn seat_marks(&mut self, graph: &Graph) {
        let groups = self.group.iter().copied().max().map_or(0, |max| max + 1);
        for group in 0..groups {
            let mut ranks: Vec<i32> = (0..graph.nodes.len())
                .filter(|&index| self.group[index] == group)
                .map(|index| self.rank[index])
                .collect();
            ranks.sort_unstable();
            ranks.dedup();
            for rank in ranks {
                let row = self.row(graph, group, rank);
                let mut keyed: Vec<(i64, i64, i32, usize)> = row
                    .iter()
                    .map(|&index| {
                        let same: Vec<i64> = self.partners[index]
                            .iter()
                            .filter(|&&other| {
                                self.rank[other] == rank && self.group[other] == group
                            })
                            .map(|&other| i64::from(self.order[other]))
                            .collect();
                        if graph.nodes[index].piece == Piece::Mark && !same.is_empty() {
                            let count = i64::try_from(same.len()).unwrap_or(1);
                            (same.iter().sum(), count, self.order[index], index)
                        } else {
                            (i64::from(self.order[index]), 1, self.order[index], index)
                        }
                    })
                    .collect();
                keyed.sort_by(|a, b| (a.0 * b.1).cmp(&(b.0 * a.1)).then(a.2.cmp(&b.2)));
                let mut orders: Vec<i32> = row.iter().map(|&index| self.order[index]).collect();
                orders.sort_unstable();
                for (slot, &(_, _, _, index)) in keyed.iter().enumerate() {
                    self.order[index] = orders[slot];
                }
            }
        }
    }

    /// One pass over every row, ordering each piece by the median position of
    /// its neighbours on the row it is being compared against.
    fn sweep(&mut self, graph: &Graph, downward: bool) {
        let mut ranks: Vec<i32> = self.rank.clone();
        ranks.sort_unstable();
        ranks.dedup();
        if !downward {
            ranks.reverse();
        }
        for rank in ranks {
            let mut row: Vec<usize> = (0..graph.nodes.len())
                .filter(|&index| self.rank[index] == rank)
                .collect();
            let keys: Vec<i32> = row
                .iter()
                .map(|&index| self.median(index, downward))
                .collect();
            let mut paired: Vec<(i32, i32, usize)> = row
                .iter()
                .zip(&keys)
                .map(|(&index, &key)| (key, self.order[index], index))
                .collect();
            paired.sort_by_key(|&(key, order, _)| (key, order));
            row.clear();
            for (position, &(_, _, index)) in paired.iter().enumerate() {
                self.order[index] = i32::try_from(position).unwrap_or(i32::MAX);
                row.push(index);
            }
        }
    }

    /// The median of the neighbours' positions, or the piece's own position
    /// when it has none — a piece with nothing to line up against stays put,
    /// which is what keeps a sweep from shuffling unrelated rows.
    fn median(&self, index: usize, downward: bool) -> i32 {
        let neighbours = if downward {
            &self.parents[index]
        } else {
            &self.children[index]
        };
        if neighbours.is_empty() {
            return self.order[index] * 2;
        }
        let mut positions: Vec<i32> = neighbours.iter().map(|&other| self.order[other]).collect();
        positions.sort_unstable();
        let middle = positions.len() / 2;
        if positions.len() % 2 == 1 {
            positions[middle] * 2
        } else {
            positions[middle - 1] + positions[middle]
        }
    }

    /// Moves partners next to one another where it costs nothing.
    ///
    /// A wedding drawn with the bride three columns from the groom is correct
    /// and unreadable, and no crossing count will ever object to it.
    fn pull_partners(&mut self, graph: &Graph, mut best: usize) {
        for _ in 0..SWEEPS {
            let mut moved = false;
            for a in 0..graph.nodes.len() {
                for &b in &self.partners[a].clone() {
                    if self.rank[a] != self.rank[b] || (self.order[a] - self.order[b]).abs() <= 1 {
                        continue;
                    }
                    let step = if self.order[a] < self.order[b] { 1 } else { -1 };
                    let target = self.order[a] + step;
                    let Some(other) = (0..graph.nodes.len()).find(|&index| {
                        self.rank[index] == self.rank[a] && self.order[index] == target
                    }) else {
                        continue;
                    };
                    self.order.swap(a, other);
                    let now = self.crossings(graph);
                    if now <= best {
                        best = now;
                        moved = true;
                    } else {
                        self.order.swap(a, other);
                    }
                }
            }
            if !moved {
                return;
            }
        }
    }

    /// How many pairs of connections cross, summed over every pair of adjacent
    /// rows. Counted as inversions, which is what a crossing between two rows
    /// is.
    fn crossings(&self, graph: &Graph) -> usize {
        // EVERY BOND, OVER EVERY ROW IT SPANS.
        //
        // This counted `Descent` bonds between ADJACENT rows and nothing else,
        // which made two whole kinds of crossing invisible to the thing whose
        // job is to remove them: a union drawn along a row, and any bond that
        // spans more than one — a grandparent married to a grandchild, a
        // figure with an explicit `rank`. A reader sent a picture of exactly
        // that: a union crossing two descent lines with nothing objecting.
        //
        // A bond is a segment now, cut into one piece per row it passes, and
        // two segments on the same row cross when their ends are inverted. A
        // union lies along one row, so both its ends are on it and it crosses
        // whatever passes between its partners — which is the case the picture
        // was of.
        let mut edges: Vec<(i32, i32, i32)> = Vec::new();
        for bond in &graph.bonds {
            let from = Work::of(graph, &bond.from);
            let to = Work::of(graph, &bond.to);
            if self.group[from] != self.group[to] {
                continue;
            }
            let (top, bottom) = if self.rank[from] <= self.rank[to] {
                (from, to)
            } else {
                (to, from)
            };
            let span = self.rank[bottom] - self.rank[top];
            if span == 0 {
                // Along one row: it occupies the whole span between its ends.
                edges.push((self.rank[top], self.order[top], self.order[bottom]));
                continue;
            }
            // Down: one segment per row it crosses, each interpolated so a bond
            // spanning three rows is three ordinary crossings rather than one
            // that nothing counts.
            for step in 0..span {
                let a = self.order[top] + (self.order[bottom] - self.order[top]) * step / span;
                let b =
                    self.order[top] + (self.order[bottom] - self.order[top]) * (step + 1) / span;
                edges.push((self.rank[top] + step, a, b));
            }
        }
        let mut total = 0;
        for (index, &(rank, upper, lower)) in edges.iter().enumerate() {
            for &(other_rank, other_upper, other_lower) in &edges[index + 1..] {
                if rank != other_rank {
                    continue;
                }
                let crosses = (upper < other_upper && lower > other_lower)
                    || (upper > other_upper && lower < other_lower);
                if crosses {
                    total += 1;
                }
            }
        }
        total
    }

    /// Tries every ordering of every small row, and keeps the best.
    ///
    /// The median sweep is a guess and the transposition pass only sees
    /// adjacent swaps; a reader sent a scene whose crossings a human removes
    /// by swapping two figures three places apart. The graphs this component
    /// draws are tiny, so for a row of up to six pieces the honest answer is
    /// to try all of them. Permutations are visited in lexicographic order
    /// and only a strict improvement is kept, so both engines land on the
    /// same drawing every time.
    fn exact(&mut self, graph: &Graph, mut best: usize) -> usize {
        if graph.nodes.len() > 24 {
            return best;
        }
        let groups = self.group.iter().copied().max().map_or(0, |max| max + 1);
        for _ in 0..2 {
            for group in 0..groups {
                let mut ranks: Vec<i32> = (0..graph.nodes.len())
                    .filter(|&index| self.group[index] == group)
                    .map(|index| self.rank[index])
                    .collect();
                ranks.sort_unstable();
                ranks.dedup();
                for rank in ranks {
                    let mut row: Vec<usize> = (0..graph.nodes.len())
                        .filter(|&index| self.group[index] == group && self.rank[index] == rank)
                        .collect();
                    row.sort_by_key(|&index| self.order[index]);
                    if row.len() < 2 || row.len() > 6 {
                        continue;
                    }
                    let values: Vec<i32> = row.iter().map(|&index| self.order[index]).collect();
                    let mut assignment: Vec<usize> = (0..row.len()).collect();
                    let mut kept: Vec<usize> = assignment.clone();
                    loop {
                        for (slot, &member) in assignment.iter().enumerate() {
                            self.order[row[member]] = values[slot];
                        }
                        let now = self.crossings(graph);
                        if now < best {
                            best = now;
                            kept.clone_from(&assignment);
                        }
                        if !next_permutation(&mut assignment) {
                            break;
                        }
                    }
                    for (slot, &member) in kept.iter().enumerate() {
                        self.order[row[member]] = values[slot];
                    }
                }
            }
        }
        best
    }

    /// Swaps every neighbouring pair that removes a crossing, until none does.
    ///
    /// The median sweep orders a row by where its neighbours are, which is a
    /// good guess and cannot see a swap that only pays off two rows later.
    /// This is the exhaustive half: try every adjacent pair, keep what helps.
    fn transpose(&mut self, graph: &Graph, mut best: usize) -> usize {
        for _ in 0..SWEEPS {
            let mut moved = false;
            let mut ranks: Vec<i32> = self.rank.clone();
            ranks.sort_unstable();
            ranks.dedup();
            for rank in ranks {
                let mut row: Vec<usize> = (0..graph.nodes.len())
                    .filter(|&index| self.rank[index] == rank)
                    .collect();
                row.sort_by_key(|&index| self.order[index]);
                for pair in 1..row.len() {
                    let (a, b) = (row[pair - 1], row[pair]);
                    self.order.swap(a, b);
                    let now = self.crossings(graph);
                    if now < best {
                        best = now;
                        row.swap(pair - 1, pair);
                        moved = true;
                    } else {
                        self.order.swap(a, b);
                    }
                }
            }
            if !moved {
                break;
            }
        }
        best
    }
}

// --- Place ------------------------------------------------------------------

impl Work {
    /// Turns an order into half-cell coordinates.
    fn place_everything(&mut self, graph: &Graph, options: Options) {
        // ONE GAP, AND IT IS A CELL. See `Options` for what was here.
        let gap = CELL;
        if options.view == View::Deep {
            for index in 0..graph.nodes.len() {
                self.group[index] = self.branch[index];
            }
        }
        self.pack_rows(graph, gap);
        for _ in 0..4 {
            self.centre(graph, true, gap);
            self.centre(graph, false, gap);
        }
        // A PARTNER WITH NO OTHER TIES STANDS BESIDE THE UNION. A figure whose
        // every connection is a same-row union has no cross-row kin, so the
        // centring above never moves it: it stays wherever the packing left it
        // while its union mark walks toward the children — which stretched
        // "Biological mother unknown" eight cells from her own partner, with
        // the union point floating in the empty middle. Such a figure is
        // pulled flush against its nearest partner, inside whatever room its
        // row neighbours leave. One pass, after the rounds: the figure is
        // nobody's kin (no cross-row bonds, by definition), so moving it
        // feeds nothing back.
        self.snuggle(graph, gap);
        // NOTHING LEFT OF ZERO. Centring pulls a row toward its kin and kin
        // can be to the left, so a figure could end up at a negative
        // coordinate — outside the scene's own box, which clips. A reader sent
        // a screenshot of half of Bob Doe. Each group is shifted so its
        // leftmost member is exactly at its own origin.
        let groups = self.group.iter().copied().max().map_or(0, |max| max + 1);
        for group in 0..groups {
            let least = (0..graph.nodes.len())
                .filter(|&index| self.group[index] == group)
                .map(|index| self.x[index])
                .min();
            if let Some(least) = least {
                if least != 0 {
                    for index in 0..graph.nodes.len() {
                        if self.group[index] == group {
                            self.x[index] -= least;
                        }
                    }
                }
            }
        }
        self.measure_groups(graph, gap);
    }

    /// Lays each row out left to right, in order, with a gap between pieces.
    fn pack_rows(&mut self, graph: &Graph, gap: i32) {
        let groups = self.group.iter().copied().max().map_or(0, |max| max + 1);
        for group in 0..groups {
            let mut ranks: Vec<i32> = (0..graph.nodes.len())
                .filter(|&index| self.group[index] == group)
                .map(|index| self.rank[index])
                .collect();
            ranks.sort_unstable();
            ranks.dedup();
            for rank in ranks {
                let mut cursor = 0;
                let mut previous: Option<usize> = None;
                for index in self.row(graph, group, rank) {
                    let air = previous.map_or(0, |left| air_between(graph, left, index, gap));
                    let half = graph.nodes[index].span / 2;
                    self.x[index] = cursor + air + half;
                    cursor = self.x[index] + half;
                    previous = Some(index);
                }
            }
        }
    }

    /// The pieces on one row of one group, in order.
    fn row(&self, graph: &Graph, group: usize, rank: i32) -> Vec<usize> {
        let mut row: Vec<usize> = (0..graph.nodes.len())
            .filter(|&index| self.group[index] == group && self.rank[index] == rank)
            .collect();
        row.sort_by_key(|&index| self.order[index]);
        row
    }

    /// Pulls each piece toward the middle of what it is joined to on the row
    /// above (or below), then restores the minimum separation.
    fn centre(&mut self, graph: &Graph, downward: bool, gap: i32) {
        let groups = self.group.iter().copied().max().map_or(0, |max| max + 1);
        for group in 0..groups {
            let mut ranks: Vec<i32> = (0..graph.nodes.len())
                .filter(|&index| self.group[index] == group)
                .map(|index| self.rank[index])
                .collect();
            ranks.sort_unstable();
            ranks.dedup();
            if !downward {
                ranks.reverse();
            }
            for rank in ranks {
                let row = self.row(graph, group, rank);
                for &index in &row {
                    let neighbours = if downward {
                        &self.parents[index]
                    } else {
                        &self.children[index]
                    };
                    // PARTNERS COUNT AS KIN ONLY ACROSS ROWS. A partner on
                    // ANOTHER row (a spanning union) is a real alignment
                    // target; a partner on the SAME row is what the ordering
                    // and the spacing already hold adjacent — and pulling
                    // toward one is an intra-row jump that, together with a
                    // spread that resolves overlaps rightward, walked a family
                    // twenty-six cells away from a milestone note, one round
                    // at a time.
                    let kin: Vec<i32> = neighbours
                        .iter()
                        .chain(
                            self.partners[index]
                                .iter()
                                .filter(|&&other| self.rank[other] != self.rank[index]),
                        )
                        .filter(|&&other| self.group[other] == group)
                        .map(|&other| self.x[other])
                        .collect();
                    if kin.is_empty() {
                        continue;
                    }
                    let low = kin.iter().copied().min().unwrap_or(0);
                    let high = kin.iter().copied().max().unwrap_or(0);
                    // Rounded down, and both halves round down, because the
                    // sum is never negative: rows start at zero.
                    self.x[index] = (low + high) / 2;
                }
                self.separate(graph, &row, gap);
            }
        }
    }

    /// Pulls a figure whose only ties are same-row unions flush against its
    /// nearest partner — see the call site for why nothing else ever moves it.
    fn snuggle(&mut self, graph: &Graph, gap: i32) {
        let groups = self.group.iter().copied().max().map_or(0, |max| max + 1);
        for group in 0..groups {
            let mut ranks: Vec<i32> = (0..graph.nodes.len())
                .filter(|&index| self.group[index] == group)
                .map(|index| self.rank[index])
                .collect();
            ranks.sort_unstable();
            ranks.dedup();
            for rank in ranks {
                let row = self.row(graph, group, rank);
                for slot in 0..row.len() {
                    let index = row[slot];
                    if graph.nodes[index].piece != Piece::Figure
                        || !self.parents[index].is_empty()
                        || !self.children[index].is_empty()
                        || self.partners[index].is_empty()
                        || self.partners[index]
                            .iter()
                            .any(|&other| self.rank[other] != rank || self.group[other] != group)
                    {
                        continue;
                    }
                    let near = self.partners[index]
                        .iter()
                        .copied()
                        .min_by_key(|&other| (self.x[other] - self.x[index]).abs())
                        .expect("checked non-empty");
                    let desired = if self.x[index] <= self.x[near] {
                        self.x[near]
                            - graph.nodes[near].span / 2
                            - air_between(graph, index, near, gap)
                            - graph.nodes[index].span / 2
                    } else {
                        self.x[near]
                            + graph.nodes[near].span / 2
                            + air_between(graph, near, index, gap)
                            + graph.nodes[index].span / 2
                    };
                    let mut x = desired;
                    if slot > 0 {
                        let prev = row[slot - 1];
                        x = x.max(
                            self.x[prev]
                                + graph.nodes[prev].span / 2
                                + air_between(graph, prev, index, gap)
                                + graph.nodes[index].span / 2,
                        );
                    }
                    if slot + 1 < row.len() {
                        let next = row[slot + 1];
                        x = x.min(
                            self.x[next]
                                - graph.nodes[next].span / 2
                                - air_between(graph, index, next, gap)
                                - graph.nodes[index].span / 2,
                        );
                    }
                    // Moving away from the partner is not snuggling.
                    if (x - self.x[near]).abs() < (self.x[index] - self.x[near]).abs() {
                        self.x[index] = x;
                    }
                }
            }
        }
    }

    /// Restores the order and the minimum gap after a row has been pulled
    /// about, left to right and then right to left so a row stays centred
    /// rather than drifting toward the left edge.
    fn separate(&mut self, graph: &Graph, row: &[usize], gap: i32) {
        for &index in row {
            self.kept_x[index] = self.x[index];
        }
        for pair in row.windows(2) {
            let (left, right) = (pair[0], pair[1]);
            let least = self.x[left]
                + graph.nodes[left].span / 2
                + air_between(graph, left, right, gap)
                + graph.nodes[right].span / 2;
            if self.x[right] < least {
                self.x[right] = least;
            }
        }
        for pair in row.windows(2).rev() {
            let (left, right) = (pair[0], pair[1]);
            let most = self.x[right]
                - graph.nodes[right].span / 2
                - air_between(graph, left, right, gap)
                - graph.nodes[left].span / 2;
            if self.x[left] > most {
                self.x[left] = most;
            }
        }
        // MEAN-PRESERVING, AND THAT IS THE FIX FOR A RATCHET. The two passes
        // above resolve overlaps mostly by pushing right, and the centring
        // that calls this jumps a figure onto its own union mark — so every
        // round moved a family a little further from anything without kin,
        // and four rounds moved it twenty-six cells from a milestone note.
        // Restoring the row's mean makes the spread symmetric: overlaps still
        // resolve, nothing drifts.
        let count = i32::try_from(row.len()).unwrap_or(1).max(1);
        let mut drift = 0;
        for &index in row {
            drift += self.x[index];
        }
        let mut before = 0;
        for &index in row {
            before += self.kept_x[index];
        }
        let shift = (drift - before).div_euclid(count);
        if shift != 0 {
            for &index in row {
                self.x[index] -= shift;
            }
        }
    }

    /// Pulls each group back against the origin, then measures how wide it is.
    ///
    /// The centring pass only ever pushes right — it moves a piece toward its
    /// kin and then restores the minimum gap — so after four rounds a scene
    /// that should begin at zero begins at twelve. Normalising here rather than
    /// inside the pass keeps the pass simple and means the number a reader
    /// sees, and the width a stylesheet reserves, are the same number.
    fn measure_groups(&mut self, graph: &Graph, gap: i32) {
        let groups = self.group.iter().copied().max().map_or(0, |max| max + 1);
        let mut left = vec![i32::MAX; groups];
        for index in 0..graph.nodes.len() {
            let edge = self.x[index] - graph.nodes[index].span / 2;
            let group = self.group[index];
            if edge < left[group] {
                left[group] = edge;
            }
        }
        for index in 0..graph.nodes.len() {
            let shift = left[self.group[index]];
            if shift != i32::MAX {
                self.x[index] -= shift;
            }
        }
        self.spans = vec![0; groups];
        for index in 0..graph.nodes.len() {
            let right = self.x[index] + graph.nodes[index].span / 2;
            let group = self.group[index];
            if right > self.spans[group] {
                self.spans[group] = right;
            }
        }
        // ONE gap between neighbouring groups. This was `gap * 3`, which put
        // three empty cells after every family and was reported as "the
        // distances are absurdly gigantic" — with a photograph of a milestone
        // note a whole page away from the family it annotates.
        for span in &mut self.spans {
            *span += gap;
        }
    }

    /// A mark sits in the middle of the line it interrupts.
    ///
    /// # Why it did not
    ///
    /// A union point is placed like anything else on its row: it takes half the
    /// air a figure would, and where it lands then depends on what its two
    /// partners had to make room for underneath them. On the first example on
    /// the documentation page that put the dot a third of the way along its own
    /// line rather than halfway, which is exactly the kind of wrongness a
    /// reader sees before they can say what it is.
    ///
    /// So a mark joining exactly two pieces on its own row is moved to the
    /// midpoint of the two, in CELLS rather than in half-cells: the middle of
    /// two half-cell positions is a quarter, and a quarter of a cell is a
    /// number both languages compute and print identically.
    ///
    /// Only that case. A mark with one neighbour is the end of a line and has
    /// no middle to sit in; a mark joining three has no midpoint at all.
    fn centre_marks(graph: &Graph, places: &mut [Place]) {
        // Read every move first, then make them. A mark centred between two
        // marks would otherwise depend on which was visited first, and the
        // whole crate's promise is that the same graph is the same scene.
        let mut moves: Vec<(usize, f64)> = Vec::new();
        for (index, place) in places.iter().enumerate() {
            if graph.nodes[place.node].piece != Piece::Mark {
                continue;
            }
            let id = &graph.nodes[place.node].id;
            let mut sides: Vec<usize> = Vec::new();
            for bond in &graph.bonds {
                let far = if &bond.from == id {
                    &bond.to
                } else if &bond.to == id {
                    &bond.from
                } else {
                    continue;
                };
                let other = places
                    .iter()
                    .position(|candidate| graph.nodes[candidate.node].id == *far);
                if let Some(other) = other {
                    if places[other].rank == place.rank && !sides.contains(&other) {
                        sides.push(other);
                    }
                }
            }
            if sides.len() == 2 {
                // allow-literal: `f64::midpoint` has no JavaScript spelling, and
                // `layout.js` is a transcription a reader checks line against line.
                #[allow(clippy::manual_midpoint)]
                let middle = (places[sides[0]].x + places[sides[1]].x) / 2.0;
                // AND NOT ONTO SOMEBODY. Two partners with a third person
                // standing between them put the midpoint inside that person's
                // card, and the union dot was drawn on their face — reported,
                // with a photograph. A mark that cannot be centred stays where
                // the layout put it, which is beside the line rather than on
                // top of a reader.
                let clear = places.iter().enumerate().all(|(other, place)| {
                    other == index
                        || place.rank != places[index].rank
                        || graph.nodes[place.node].piece == Piece::Mark
                        || (place.x - middle).abs() >= 0.75
                });
                if clear {
                    moves.push((index, middle));
                }
            }
        }
        for (index, x) in moves {
            places[index].x = x;
        }
    }

    /// The finished coordinates, in cells.
    ///
    /// This is the one place the two views differ, and they differ in a single
    /// axis: **flat sets the branches side by side and deep sets them behind
    /// one another.** Clusters are always side by side, because two unrelated
    /// families hidden behind each other would be a worse drawing in any view.
    fn places(&self, graph: &Graph, view: View) -> Vec<Place> {
        let mut places = self.laid(graph, view);
        Self::centre_marks(graph, &mut places);
        places
    }

    /// Where each branch starts, so that in the deep view every one of them
    /// begins at the same left edge.
    ///
    /// THE DEFECT THIS FIXES. The deep view kept the flat layout's `x` and
    /// pushed each branch back in `z`, so a branch already standing to the
    /// right ALSO got the sideways step — and two figures on different planes
    /// landed on exactly the same point. From straight on that is one card; at
    /// any angle it is a card with another one growing out of it. In the
    /// example on this page a whole person disappeared behind their cousin's
    /// partner.
    ///
    /// Each branch is measured and moved to zero. What separates them after
    /// that is the stagger alone, which is a number this file chooses rather
    /// than one the flat layout happened to leave behind.
    fn zeroes(&self, graph: &Graph) -> Vec<i32> {
        let lanes = usize::try_from(
            (0..graph.nodes.len())
                .map(|index| self.lane(index))
                .max()
                .unwrap_or(0),
        )
        .unwrap_or(0)
            + 1;
        let mut left = vec![i32::MAX; lanes];
        let mut right = vec![i32::MIN; lanes];
        for index in 0..graph.nodes.len() {
            let lane = usize::try_from(self.lane(index)).unwrap_or(0);
            left[lane] = left[lane].min(self.x[index]);
            right[lane] = right[lane].max(self.x[index]);
        }
        // EVERY PLANE STARTS AT THE SAME EDGE. Depth is the whole point of
        // the deep view — a wide generation becomes planes, not a wider row —
        // so the planes share their x space and the stagger below plus the
        // resting turn is what keeps each one's corner visible. Spreading
        // them out cumulatively was tried, and it rebuilt `flat` eighteen
        // cells wide with extra steps.
        let _ = right;
        let mut out = vec![0; lanes];
        for lane in 0..lanes {
            if left[lane] != i32::MAX {
                out[lane] = left[lane];
            }
        }
        out
    }

    /// Where the four passes left everything, before the marks are centred.
    fn laid(&self, graph: &Graph, view: View) -> Vec<Place> {
        let offsets = self.offsets(graph, view);
        let zeroes = self.zeroes(graph);
        (0..graph.nodes.len())
            .map(|index| {
                let deep = view == View::Deep;
                let key = if deep {
                    self.cluster[index]
                } else {
                    self.group[index]
                };
                // THE PLANES STAGGER SIDEWAYS AS THEY RECEDE, and that is what
                // makes the deep view readable from straight on. Stacked
                // exactly behind one another they are one plane with things
                // hidden behind it, and the camera turn that used to separate
                // them opened every scene at an angle nobody asked for.
                // TWO CELLS PER PLANE, NOT ONE. A branch one cell to the
                // side of the one in front of it is a branch whose cards
                // overlap the cards in front as soon as the camera moves off
                // dead centre — which is the only camera from which the deep
                // view is worth looking at.
                let zero = if deep {
                    zeroes[usize::try_from(self.lane(index)).unwrap_or(0)]
                } else {
                    0
                };
                Place {
                    node: index,
                    x: half(self.x[index] - zero + offsets[key]),
                    y: f64::from(self.rank[index]),
                    z: if deep {
                        f64::from(self.lane(index))
                    } else {
                        0.0
                    },
                    rank: self.rank[index],
                    cluster: self.cluster[index],
                    lane: self.lane(index),
                }
            })
            .collect()
    }

    /// Where each group starts along `x`, in half-cells.
    ///
    /// In the deep view a cluster is as wide as its widest branch rather than
    /// as wide as all of them, because in that view the branches are stacked
    /// rather than laid out.
    fn offsets(&self, graph: &Graph, view: View) -> Vec<i32> {
        let mut spans = self.spans.clone();
        if view == View::Deep {
            let clusters = self.cluster.iter().copied().max().map_or(0, |max| max + 1);
            let mut widest = vec![0; clusters];
            for index in 0..graph.nodes.len() {
                let cluster = self.cluster[index];
                let span = self.spans[self.group[index]];
                if span > widest[cluster] {
                    widest[cluster] = span;
                }
            }
            spans = widest;
        }
        let mut offsets = vec![0; spans.len()];
        let mut cursor = 0;
        for (group, span) in spans.iter().enumerate() {
            offsets[group] = cursor;
            cursor += span;
        }
        offsets
    }

    /// Which depth plane a piece sits on. The trunk is always the front one.
    fn lane(&self, index: usize) -> i32 {
        i32::try_from(self.branch[index]).unwrap_or(0)
    }
}

/// How much air belongs between two neighbours, in half-cells.
///
/// A mark takes half of it, because a mark is half the size of a figure and a
/// union that cannot sit between the two people it joins is a union drawn in
/// the wrong place. Measured: with one gap for everything, a couple's union
/// mark landed half a cell to the right of the midpoint of its own children,
/// and no amount of centring could reach it — the separation minimum was in the
/// way.
fn air_between(graph: &Graph, left: usize, right: usize, gap: i32) -> i32 {
    // A LABELLED LINE NEEDS LINE TO SIT ON. Two neighbours joined by a
    // same-row bond that carries a label get double the air, because the label
    // lives on the free stretch between their cards and a packed pair leaves
    // no stretch at all — the word "doador" was drawn on top of the donor.
    let joined = graph.bonds.iter().any(|bond| {
        !bond.label.is_empty()
            && ((bond.from == graph.nodes[left].id && bond.to == graph.nodes[right].id)
                || (bond.from == graph.nodes[right].id && bond.to == graph.nodes[left].id))
    });
    if joined {
        return gap * 2;
    }
    let small = graph.nodes[left].piece == Piece::Mark || graph.nodes[right].piece == Piece::Mark;
    if small { (gap + 1) / 2 } else { gap }
}

/// Half a count of half-cells, which is a count of cells.
fn half(value: i32) -> f64 {
    f64::from(value) / 2.0
}

/// How far the scene reaches, in cells, with the pieces' own width counted.
fn extent(graph: &Graph, places: &[Place]) -> (f64, f64, f64) {
    let mut width = 0.0_f64;
    let mut height = 0.0_f64;
    let mut depth = 0.0_f64;
    for place in places {
        let half_span = half(graph.nodes[place.node].span) / 2.0;
        width = width.max(place.x + half_span);
        height = height.max(place.y);
        depth = depth.max(place.z);
    }
    (width, height, depth)
}

// --- Route ------------------------------------------------------------------

/// Turns every bond into the runs that draw it.
fn route(graph: &Graph, places: &[Place], options: Options) -> Vec<Segment> {
    let lanes = lanes(graph, places);
    let mut segments = Vec::new();
    for (index, bond) in graph.bonds.iter().enumerate() {
        let from = &places[Work::of(graph, &bond.from)];
        let to = &places[Work::of(graph, &bond.to)];
        let taut = bond.shape == Shape::Lateral
            || bond.route == "taut"
            || (bond.route.is_empty() && options.route == Route::Taut);
        let offset = f64::from(lanes[index]) * 0.15;
        if taut {
            segments.push(straight(index, from, to));
        } else if bond.shape == Shape::Union {
            if from.rank == to.rank {
                segments.extend(across(index, from, to));
            } else {
                // A UNION ACROSS GENERATIONS GOES AROUND THE OUTSIDE.
                //
                // It used to be drawn straight, and a straight line from a
                // grandparent to a grandchild passes through everything
                // standing between them — the reported "horrendous, totally
                // messy" scene was mostly these. Around the block's edge it
                // crosses nothing: along its own channel, down the outside,
                // in along the destination's channel.
                segments.extend(around(index, from, to, places, offset));
            }
        } else {
            segments.extend(elbow(index, from, to, offset));
        }
    }
    shy_ends(graph, places, &mut segments, options.shy);
    segments
}

/// How much of a cell a shy connection leaves between its end and the figure.
const SHY: f64 = 0.35;

/// Trims every run end that lands on a FIGURE's own point — see `Options::shy`
/// and [`Node::reach`]: the piece's own `reach` wins over the scene's mode,
/// which is how one tall figure gets a longer stand-off in a `touch` scene.
fn shy_ends(graph: &Graph, places: &[Place], segments: &mut [Segment], shy: bool) {
    let points: Vec<(&Place, f64)> = graph
        .nodes
        .iter()
        .zip(places)
        .filter(|(node, _)| node.piece == Piece::Figure)
        .filter_map(|(node, place)| {
            let trim = if node.reach >= 0 {
                f64::from(node.reach) / 10.0
            } else if shy {
                SHY
            } else {
                return None;
            };
            (trim > 0.0).then_some((place, trim))
        })
        .collect();
    if points.is_empty() {
        return;
    }
    let at = |x: f64, y: f64, z: f64| {
        points
            .iter()
            .find(|(place, _)| {
                (place.x - x).abs() <= f64::EPSILON
                    && (place.y - y).abs() <= f64::EPSILON
                    && (place.z - z).abs() <= f64::EPSILON
            })
            .map(|(_, trim)| *trim)
    };
    for segment in segments {
        // The direction the run travels: an axis, or the basis' own first
        // column for a free one.
        let dir = match segment.axis {
            Axis::X => [1.0, 0.0, 0.0],
            Axis::Y => [0.0, 1.0, 0.0],
            Axis::Z => [0.0, 0.0, 1.0],
            Axis::Free => [segment.basis[0], segment.basis[1], segment.basis[2]],
        };
        // allow-literal: `mul_add` fuses its rounding and JavaScript cannot,
        // so the plain spelling is the one both engines share.
        #[allow(clippy::suboptimal_flops)]
        let far = (
            segment.len * dir[0] + segment.x,
            segment.len * dir[1] + segment.y,
            segment.len * dir[2] + segment.z,
        );
        // Never below a stub: a run trimmed to nothing is a loose end with
        // extra steps.
        if let Some(trim) = at(segment.x, segment.y, segment.z) {
            if segment.len > trim + 0.05 {
                segment.x += trim * dir[0];
                segment.y += trim * dir[1];
                segment.z += trim * dir[2];
                segment.len -= trim;
            }
        }
        if let Some(trim) = at(far.0, far.1, far.2) {
            if segment.len > trim + 0.05 {
                segment.len -= trim;
            }
        }
    }
}

/// Which lane each bond's channel takes, so brackets never overlap.
///
/// Two different parents' brackets can share the same channel — the strip of
/// air above one row — and where their x-ranges overlap, their lines used to
/// lie on top of one another for the shared stretch. A reader sent the
/// picture. Bonds are grouped by channel and by the upper end they hang from
/// (one family's bracket is one drawing and SHOULD overlay itself); families
/// whose intervals overlap take successive lanes, first-declared first.
fn lanes(graph: &Graph, places: &[Place]) -> Vec<i32> {
    // (channel row, upper id) -> interval over x, in declaration order.
    let mut families: Vec<(i32, String, f64, f64)> = Vec::new();
    let mut belongs: Vec<Option<usize>> = vec![None; graph.bonds.len()];
    for (index, bond) in graph.bonds.iter().enumerate() {
        if bond.shape != Shape::Descent {
            continue;
        }
        let from = &places[Work::of(graph, &bond.from)];
        let to = &places[Work::of(graph, &bond.to)];
        if from.rank == to.rank {
            continue;
        }
        let (upper, lower) = if from.rank <= to.rank {
            (from, to)
        } else {
            (to, from)
        };
        let upper_id = if from.rank <= to.rank {
            bond.from.clone()
        } else {
            bond.to.clone()
        };
        let key = (lower.rank, upper_id);
        let left = upper.x.min(lower.x);
        let right = upper.x.max(lower.x);
        if let Some(at) = families
            .iter()
            .position(|(rank, id, _, _)| *rank == key.0 && *id == key.1)
        {
            families[at].2 = families[at].2.min(left);
            families[at].3 = families[at].3.max(right);
            belongs[index] = Some(at);
        } else {
            families.push((key.0, key.1, left, right));
            belongs[index] = Some(families.len() - 1);
        }
    }
    // Greedy, in declaration order: the smallest lane no overlapping,
    // already-assigned family of the same channel is using.
    let mut lane_of: Vec<i32> = vec![0; families.len()];
    for at in 0..families.len() {
        let mut lane = 0;
        loop {
            let clash = (0..at).any(|other| {
                families[other].0 == families[at].0
                    && lane_of[other] == lane
                    && families[other].2 <= families[at].3
                    && families[at].2 <= families[other].3
            });
            if !clash {
                break;
            }
            lane += 1;
        }
        lane_of[at] = lane;
    }
    (0..graph.bonds.len())
        .map(|index| belongs[index].map_or(0, |at| lane_of[at]))
        .collect()
}

/// The outside elbow a spanning union takes: along the upper partner's own
/// channel, down past the block's edge, in along the lower partner's channel.
fn around(bond: usize, from: &Place, to: &Place, places: &[Place], offset: f64) -> Vec<Segment> {
    let (upper, lower) = if from.rank <= to.rank {
        (from, to)
    } else {
        (to, from)
    };
    // Half a cell past the widest thing in the same cluster.
    let edge = places
        .iter()
        .filter(|place| place.cluster == upper.cluster)
        .map(|place| place.x)
        .fold(0.0_f64, f64::max)
        + 0.75;
    let top = f64::from(upper.rank) + 0.55 - offset;
    let bottom = f64::from(lower.rank) - 0.45 + offset;
    vec![
        run(bond, upper.x, upper.y, upper.z, top - upper.y, Axis::Y),
        run(bond, upper.x, top, upper.z, edge - upper.x, Axis::X),
        run(bond, edge, top, upper.z, bottom - top, Axis::Y),
        run(
            bond,
            lower.x.min(edge),
            bottom,
            upper.z,
            edge - lower.x,
            Axis::X,
        ),
        run(bond, lower.x, bottom, lower.z, lower.y - bottom, Axis::Y),
    ]
}

/// Down, across, through, down: the bracket every family tree is drawn with.
fn elbow(bond: usize, from: &Place, to: &Place, offset: f64) -> Vec<Segment> {
    let (upper, lower) = if from.y <= to.y {
        (from, to)
    } else {
        (to, from)
    };
    // JUST ABOVE THE ROW IT DESCENDS INTO — 0.45 of a pitch above it — for
    // two reasons that both bind. The label sits ON TOP of this line (the
    // developer's explicit, twice-repeated instruction), so the room the label
    // needs is all above it; and a bond that spans more than one rank must
    // put its channel where no intermediate row is, which "just above the
    // destination" is and "the midpoint" is not.
    // The lane offset goes DOWN, toward the row the bracket enters: upward it
    // would eat the strip above the first line, which is where every label
    // lives.
    let channel = f64::from(lower.rank) - 0.45 + offset;
    // Straight down, so there is nothing for the bracket to bracket. Drawn as
    // one run rather than two abutting ones: the two render identically, and a
    // seam in the middle of a straight line is a hairline waiting to show up
    // under a transform.
    if (lower.x - upper.x).abs() <= f64::EPSILON && (lower.z - upper.z).abs() <= f64::EPSILON {
        return vec![run(
            bond,
            upper.x,
            upper.y,
            upper.z,
            lower.y - upper.y,
            Axis::Y,
        )];
    }
    let mut runs = vec![run(
        bond,
        upper.x,
        upper.y,
        upper.z,
        channel - upper.y,
        Axis::Y,
    )];
    if (lower.x - upper.x).abs() > f64::EPSILON {
        let left = upper.x.min(lower.x);
        runs.push(run(
            bond,
            left,
            channel,
            upper.z,
            (lower.x - upper.x).abs(),
            Axis::X,
        ));
    }
    if (lower.z - upper.z).abs() > f64::EPSILON {
        let near = upper.z.min(lower.z);
        runs.push(run(
            bond,
            lower.x,
            channel,
            near,
            (lower.z - upper.z).abs(),
            Axis::Z,
        ));
    }
    runs.push(run(
        bond,
        lower.x,
        channel,
        lower.z,
        lower.y - channel,
        Axis::Y,
    ));
    runs
}

/// One run along the row, for two partners who share it.
fn across(bond: usize, from: &Place, to: &Place) -> Vec<Segment> {
    let left = from.x.min(to.x);
    let length = (to.x - from.x).abs();
    let mut runs = vec![run(bond, left, from.y, from.z, length, Axis::X)];
    if (to.z - from.z).abs() > f64::EPSILON {
        let near = from.z.min(to.z);
        runs.push(run(
            bond,
            to.x,
            from.y,
            near,
            (to.z - from.z).abs(),
            Axis::Z,
        ));
    }
    runs
}

/// One straight run from end to end, carrying the rotation that turns its own
/// `+x` onto the direction it travels.
fn straight(bond: usize, from: &Place, to: &Place) -> Segment {
    let (dx, dy, dz) = (to.x - from.x, to.y - from.y, to.z - from.z);
    let len = (dx * dx + dy * dy + dz * dz).sqrt();
    let mut segment = run(bond, from.x, from.y, from.z, len, Axis::Free);
    segment.basis = basis(dx, dy, dz, len);
    segment
}

/// The next permutation in lexicographic order, or `false` at the last.
///
/// The textbook algorithm, written identically in both languages so the two
/// engines visit orderings in the same sequence and therefore keep the same
/// winner on a tie.
fn next_permutation(items: &mut [usize]) -> bool {
    if items.len() < 2 {
        return false;
    }
    let mut pivot = items.len() - 1;
    while pivot > 0 && items[pivot - 1] >= items[pivot] {
        pivot -= 1;
    }
    if pivot == 0 {
        return false;
    }
    let mut swap_with = items.len() - 1;
    while items[swap_with] <= items[pivot - 1] {
        swap_with -= 1;
    }
    items.swap(pivot - 1, swap_with);
    items[pivot..].reverse();
    true
}

/// One run.
fn run(bond: usize, x: f64, y: f64, z: f64, len: f64, axis: Axis) -> Segment {
    Segment {
        bond,
        x,
        y,
        z,
        len: len.max(0.0),
        axis,
        basis: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
    }
}

/// A rotation taking `+x` onto `(dx, dy, dz)`, column-major.
///
/// Built by Gram-Schmidt from the direction and a helper, so the only
/// transcendental involved is `sqrt` — exactly rounded in both languages. The
/// helper is the depth axis, which puts the run's face toward a reader looking
/// into the scene; a run travelling *along* the depth axis takes the vertical
/// instead, because a helper parallel to the direction spans nothing.
fn basis(dx: f64, dy: f64, dz: f64, len: f64) -> [f64; 9] {
    if len <= f64::EPSILON {
        return [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
    }
    let x = [dx / len, dy / len, dz / len];
    let helper = if x[2].abs() > 0.9 {
        [0.0, 1.0, 0.0]
    } else {
        [0.0, 0.0, 1.0]
    };
    let mut y = cross(helper, x);
    let scale = (y[0] * y[0] + y[1] * y[1] + y[2] * y[2]).sqrt();
    if scale <= f64::EPSILON {
        return [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
    }
    for value in &mut y {
        *value /= scale;
    }
    let z = cross(x, y);
    [x[0], x[1], x[2], y[0], y[1], y[2], z[0], z[1], z[2]]
}

/// The cross product, written out because importing one would be a dependency.
fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
