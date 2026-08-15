//! Every case in `spec/lineage/cases.toml`, laid out and checked.
//!
//! The other half of this — `layout.js` against `dist/lineage.cases.json` —
//! is `tests/browser/lineage-layout.spec.js`. Neither half is ever compared to
//! the other; the module documentation on `noctua_design_spec::lineage` argues
//! why, and it is the reason this file exists rather than a test that renders
//! the same graph twice.
//!
//! To regenerate the expectations after a deliberate change:
//!
//! ```sh
//! cargo test -p noctua-design-lineage bless -- --ignored --nocapture
//! ```

use noctua_design_lineage::{Options, Route, Scene, View, scene};
use noctua_design_spec::lineage::{Case, load};

/// Where the cases live, relative to this crate.
const CASES: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../spec/lineage/cases.toml");

/// Three decimals, which is the precision the two implementations promise one
/// another. Anything finer would make a language's float formatting part of the
/// contract; anything coarser would let a real disagreement through.
fn round(value: f64) -> String {
    let text = format!("{value:.3}");
    if text == "-0.000" {
        "0.000".to_owned()
    } else {
        text
    }
}

/// The same number, written the way a person would rather read it.
fn tidy(value: f64) -> String {
    let text = round(value);
    let trimmed = text.trim_end_matches('0').trim_end_matches('.');
    if trimmed.is_empty() {
        "0".to_owned()
    } else {
        trimmed.to_owned()
    }
}

fn options(case: &Case) -> Options {
    Options {
        view: if case.view == "deep" {
            View::Deep
        } else {
            View::Flat
        },
        route: if case.route == "taut" {
            Route::Taut
        } else {
            Route::Elbow
        },
        // The blessed cases exercise the layout, and `isolate` is a pruning
        // pass in front of it — its own browser test covers it.
        isolate: false,
        shy: case.reach == "shy",
    }
}

/// `<id> <x> <y> <z>` for every piece, in declaration order.
fn places(scene: &Scene) -> Vec<String> {
    scene
        .places
        .iter()
        .map(|place| {
            let id = &scene.graph.nodes[place.node].id;
            format!("{id} {} {} {}", tidy(place.x), tidy(place.y), tidy(place.z))
        })
        .collect()
}

/// `<bond> <x> <y> <z> <len> <axis>` for every run.
fn segments(scene: &Scene) -> Vec<String> {
    scene
        .segments
        .iter()
        .map(|run| {
            let axis = format!("{:?}", run.axis).to_lowercase();
            // The rotation rides along on a free run and only on a free run:
            // it is the one piece of floating-point arithmetic in the engine,
            // so it is the piece that most needs pinning across two languages,
            // and it is the identity everywhere else.
            let basis = if axis == "free" {
                let numbers: Vec<String> = run.basis.iter().map(|value| tidy(*value)).collect();
                format!(" {}", numbers.join(" "))
            } else {
                String::new()
            };
            format!(
                "{} {} {} {} {} {axis}{basis}",
                run.bond,
                tidy(run.x),
                tidy(run.y),
                tidy(run.z),
                tidy(run.len),
            )
        })
        .collect()
}

/// A row with every number re-rounded, so two spellings of one number compare
/// equal.
fn normalise(row: &str) -> String {
    row.split_whitespace()
        .map(|field| {
            field
                .parse::<f64>()
                .map_or_else(|_| field.to_owned(), round)
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[test]
fn every_case_is_placed_exactly_where_it_says() {
    let cases = load(CASES).expect("the shipped cases");
    assert!(
        cases.case.len() >= 10,
        "a conformance set this small is not one"
    );
    for case in &cases.case {
        let scene = scene(&case.graph, options(case));
        let found = places(&scene);
        let wanted: Vec<String> = case.places.iter().map(|row| normalise(row)).collect();
        let found_rows: Vec<String> = found.iter().map(|row| normalise(row)).collect();
        assert_eq!(found_rows, wanted, "`{}` was placed differently", case.name);
        assert_eq!(
            scene.segments.len(),
            case.runs,
            "`{}` drew a different number of runs",
            case.name
        );
        assert_eq!(scene.crossings, case.crossings, "`{}` crossings", case.name);
        assert_eq!(scene.skipped, case.skipped, "`{}` skipped", case.name);
        assert_eq!(scene.broken, case.broken, "`{}` broken", case.name);
        if !case.segments.is_empty() {
            let wanted: Vec<String> = case.segments.iter().map(|row| normalise(row)).collect();
            let found: Vec<String> = segments(&scene).iter().map(|row| normalise(row)).collect();
            assert_eq!(found, wanted, "`{}` was routed differently", case.name);
        }
    }
}

#[test]
fn the_same_graph_twice_is_the_same_scene_twice() {
    // PURITY, asserted rather than asserted about. A layout that consulted a
    // clock, a locale or a random number would still pass every case above on
    // the run that blessed them.
    let cases = load(CASES).expect("the shipped cases");
    for case in &cases.case {
        let once = places(&scene(&case.graph, options(case)));
        let twice = places(&scene(&case.graph, options(case)));
        assert_eq!(
            once, twice,
            "`{}` is not a function of its input",
            case.name
        );
    }
}

#[test]
fn nothing_hangs_on_a_graph_written_to_break_it() {
    // TERMINATION. None of these can be allowed to loop, and a test that hangs
    // is a test suite that hangs.
    let hostile = [
        "bond a b\nbond b a\nfigure a\nfigure b",
        &"figure a\n".repeat(200),
        &(0..80)
            .map(|index| format!("figure f{index}\nbond f{index} f{}", index + 1))
            .collect::<Vec<_>>()
            .join("\n"),
        "figure a rank=999\nfigure b rank=-999\nbond a b",
        "mark m\nbond m m\nfigure a\nbond a m kind=union\nbond m a kind=union",
    ];
    for graph in hostile {
        let scene = scene(graph, Options::default());
        assert!(
            scene.places.len() <= 201,
            "a graph produced more pieces than it declared"
        );
    }
}

#[test]
#[ignore = "a tool, not a check: prints spec/lineage/cases.toml with every expectation recomputed"]
fn bless() {
    let cases = load(CASES).expect("the shipped cases");
    let header = std::fs::read_to_string(CASES).expect("the shipped cases");
    let comment: Vec<&str> = header
        .lines()
        .take_while(|line| line.starts_with('#') || line.trim().is_empty())
        .collect();
    println!("{}", comment.join("\n"));
    for case in &cases.case {
        let scene = scene(&case.graph, options(case));
        println!("[[case]]");
        println!("name = {:?}", case.name);
        println!("note = {:?}", case.note);
        for (key, value) in [
            ("view", &case.view),
            ("route", &case.route),
            ("reach", &case.reach),
        ] {
            if !value.is_empty() {
                println!("{key} = {value:?}");
            }
        }
        // The leading newline is what TOML trims back off, so the file this
        // prints parses to exactly the graph it was given.
        println!("graph = \"\"\"\n{}\"\"\"", case.graph);
        println!("places = [");
        for row in places(&scene) {
            println!("  {row:?},");
        }
        println!("]");
        if !case.segments.is_empty() {
            println!("segments = [");
            for row in segments(&scene) {
                println!("  {row:?},");
            }
            println!("]");
        }
        println!("runs = {}", scene.segments.len());
        println!("crossings = {}", scene.crossings);
        if scene.skipped > 0 {
            println!("skipped = {}", scene.skipped);
        }
        if scene.broken > 0 {
            println!("broken = {}", scene.broken);
        }
        println!();
    }
}
