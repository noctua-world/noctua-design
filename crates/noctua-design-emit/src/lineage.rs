//! The `Lineage` layout, for the JavaScript package, and the cases both halves
//! are held to.
//!
//! | Output | For |
//! |---|---|
//! | `packages/tokens/lineage.js` | the React implementation, which lays a scene out in a browser |
//! | `packages/tokens/lineage.d.ts` | the same, typed |
//! | `dist/lineage.cases.json` | the blessed expectation **both** engines are checked against |
//!
//! # Nothing here is computed
//!
//! The algorithm is `crates/noctua-design-lineage/src/layout.rs` and its twin
//! `layout.js`, hand-written side by side. This module copies the second into
//! the JavaScript package verbatim, exactly as `grammar` does for the scanner.
//! Generating an algorithm from a specification would mean inventing a language
//! to describe it in, and that language would need a compiler nobody asked for.
//!
//! # Why the cases go to `dist/` rather than into a test
//!
//! There are two engines and they have to agree. Comparing them **to each
//! other** passes whenever they are both wrong in the same way, which is the
//! failure that actually happens. So the expectation is a file: a Rust test
//! asserts against it, a Playwright test asserts against it, and a
//! disagreement names which half moved.

use std::fmt::Write as _;

use noctua_design_spec::lineage::Cases;

use crate::{CommentStyle, EmittedFile, header_from};

/// Where the cases live, as written in generated headers.
pub const LINEAGE_SPEC: &str = "spec/lineage/cases.toml";
/// Where the algorithm lives, as written in generated headers.
pub const LINEAGE_SOURCE: &str = "crates/noctua-design-lineage/src/layout.js";
/// Where the runtime lives, likewise.
pub const LINEAGE_RUNTIME: &str = "crates/noctua-design-lineage/src/runtime.js";

/// Emits every `Lineage` artifact.
///
/// `layout` is the contents of `crates/noctua-design-lineage/src/layout.js`,
/// read from disk by `xtask` the way the icon drawings and the scanner are — it
/// is vendored input to this emitter, not something this emitter can invent.
#[must_use]
pub fn emit(
    cases: &Cases,
    layout: &str,
    runtime: &str,
    three: &str,
    licence: &str,
) -> Vec<EmittedFile> {
    vec![
        EmittedFile::new("packages/tokens/lineage.js", js(layout)),
        EmittedFile::new("packages/tokens/lineage.d.ts", js_types()),
        EmittedFile::new("dist/lineage.cases.json", blessed(cases)),
        // THE RUNTIME SHIPS, and that is a decision rather than an oversight: a
        // navigation component whose consumer cannot navigate is a control that
        // lies. It sits beside `dist/bootstrap.js`, which this system already
        // ships, and it is fetched on demand rather than on load.
        EmittedFile::new("dist/lineage.js", runtime_js(runtime)),
        // Vendored, not installed, for the reason `assets/vendor/three/`
        // records: CI runs `cargo xtask check` with no Node at all.
        EmittedFile::new("dist/vendor/three-css3d.js", three.to_owned()),
        EmittedFile::new("dist/vendor/LICENSE-three-MIT.txt", licence.to_owned()),
    ]
}

/// The runtime, with a header saying where it really lives.
fn runtime_js(runtime: &str) -> String {
    let mut out = header_from(CommentStyle::Block, LINEAGE_RUNTIME);
    out.push_str(
        "\n// COPIED, NOT GENERATED, like the layout beside it. It imports\n\
         // ./vendor/three-css3d.js on the first interaction and never before.\n\n",
    );
    out.push_str(runtime);
    out
}

/// The transcription, with a header saying where it really lives.
fn js(layout: &str) -> String {
    let mut out = header_from(CommentStyle::Block, LINEAGE_SOURCE);
    out.push_str(
        "\n// COPIED, NOT GENERATED. The file above is hand-written beside its Rust\n\
         // twin, `layout.rs`, and the two are changed in one commit. Both are checked\n\
         // against dist/lineage.cases.json, never against each other.\n\n",
    );
    out.push_str(layout);
    out
}

/// The types, written by hand because the module is.
fn js_types() -> String {
    let mut out = header_from(CommentStyle::Block, LINEAGE_SOURCE);
    out.push_str(
        "\n/** What a piece of the graph is. */\n\
         export type LineagePiece = \"figure\" | \"mark\";\n\n\
         /** What a bond does to the ranks of the two things it joins. */\n\
         export type LineageShape = \"descent\" | \"union\" | \"lateral\";\n\n\
         /** Which way a run of connection travels. `free` carries a `basis`. */\n\
         export type LineageAxis = \"x\" | \"y\" | \"z\" | \"free\";\n\n\
         /** One declared piece. */\n\
         export interface LineageNode {\n\
         \x20 id: string;\n\
         \x20 piece: LineagePiece;\n\
         \x20 label: string;\n\
         \x20 meta: string;\n\
         \x20 terms: string;\n\
         \x20 image: string;\n\
         \x20 kind: string;\n\
         \x20 tone: string;\n\
         \x20 shape: string;\n\
         \x20 set: string;\n\
         \x20 href: string;\n\
         \x20 cluster: string;\n\
         \x20 rank: number | null;\n\
         \x20 /** In half-cells, so a mark can be half a figure without a fraction. */\n\
         \x20 span: number;\n\
         \x20 slot: boolean;\n\
         \x20 origin: boolean;\n\
         }\n\n\
         /** One declared connection. */\n\
         export interface LineageBond {\n\
         \x20 from: string;\n\
         \x20 to: string;\n\
         \x20 kind: string;\n\
         \x20 shape: LineageShape;\n\
         \x20 label: string;\n\
         \x20 note: string;\n\
         \x20 tone: string;\n\
         \x20 dash: string;\n\
         \x20 route: string;\n\
         \x20 set: string;\n\
         }\n\n\
         /** Everything a graph declared. */\n\
         export interface LineageGraph {\n\
         \x20 nodes: LineageNode[];\n\
         \x20 bonds: LineageBond[];\n\
         \x20 skipped: number;\n\
         }\n\n\
         /** One piece, placed. Coordinates are in cells. */\n\
         export interface LineagePlace {\n\
         \x20 node: number;\n\
         \x20 x: number;\n\
         \x20 y: number;\n\
         \x20 z: number;\n\
         \x20 rank: number;\n\
         \x20 cluster: number;\n\
         \x20 lane: number;\n\
         }\n\n\
         /** One straight run of one connection. */\n\
         export interface LineageSegment {\n\
         \x20 bond: number;\n\
         \x20 x: number;\n\
         \x20 y: number;\n\
         \x20 z: number;\n\
         \x20 len: number;\n\
         \x20 axis: LineageAxis;\n\
         \x20 /** Column-major 3×3. The identity unless `axis` is `free`. */\n\
         \x20 basis: number[];\n\
         }\n\n\
         /** A whole graph, placed. */\n\
         export interface LineageScene {\n\
         \x20 graph: LineageGraph;\n\
         \x20 places: LineagePlace[];\n\
         \x20 segments: LineageSegment[];\n\
         \x20 width: number;\n\
         \x20 height: number;\n\
         \x20 depth: number;\n\
         \x20 crossings: number;\n\
         \x20 skipped: number;\n\
         \x20 broken: number;\n\
         }\n\n\
         /** How the scene is arranged. */\n\
         export interface LineageOptions {\n\
         \x20 view?: \"flat\" | \"deep\";\n\
         \x20 route?: \"elbow\" | \"taut\";\n\
         \x20 reach?: \"touch\" | \"shy\";\n\
         }\n\n\
         export function parse(source: string): LineageGraph;\n\
         export function place(graph: LineageGraph, options?: LineageOptions): LineageScene;\n\
         export function scene(source: string, options?: LineageOptions): LineageScene;\n\
         export function matches(node: LineageNode, filter: string): boolean;\n\
         export function fold(text: string): string;\n\
         export function shapeOf(kind: string): LineageShape;\n",
    );
    out
}

/// The one expectation both engines are checked against.
fn blessed(cases: &Cases) -> String {
    let mut out = String::from("{\n");
    let _ = writeln!(
        out,
        "  \"_generated\": \"Generated by noctua-design — do not edit. Source: {LINEAGE_SPEC}. \
         Regenerate: {}.\",",
        crate::REGENERATE
    );
    out.push_str("  \"cases\": [\n");
    for (index, case) in cases.case.iter().enumerate() {
        let comma = if index + 1 == cases.case.len() {
            ""
        } else {
            ","
        };
        let _ = write!(
            out,
            "    {{\n      \"name\": \"{}\",\n      \"graph\": \"{}\",\n      \
             \"view\": \"{}\",\n      \"route\": \"{}\",\n      \"reach\": \"{}\",\n      \
             \"places\": [{}],\n      \"segments\": [{}],\n      \"runs\": {},\n      \
             \"crossings\": {},\n      \"skipped\": {},\n      \"broken\": {}\n    }}{comma}\n",
            json(&case.name),
            json(&case.graph),
            json(&case.view),
            json(&case.route),
            json(&case.reach),
            list(&case.places),
            list(&case.segments),
            case.runs,
            case.crossings,
            case.skipped,
            case.broken,
        );
    }
    out.push_str("  ]\n}\n");
    out
}

/// A JSON array body of quoted strings.
fn list(items: &[String]) -> String {
    items
        .iter()
        .map(|item| format!("\"{}\"", json(item)))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Escapes a string for JSON.
fn json(text: &str) -> String {
    let mut out = String::new();
    for c in text.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            '\r' => out.push_str("\\r"),
            other if (other as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", other as u32);
            }
            other => out.push(other),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shipped() -> Cases {
        let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../spec/lineage/cases.toml");
        noctua_design_spec::lineage::load(root).expect("the shipped cases")
    }

    #[test]
    fn the_transcription_is_copied_and_not_rewritten() {
        // A generated header, and then the file, byte for byte. An emitter that
        // reformatted it would make the two halves impossible to diff.
        let layout = "export function scene() {}\n";
        let emitted = js(layout);
        assert!(emitted.ends_with(layout), "the transcription was rewritten");
        assert!(emitted.contains("do not edit"));
    }

    #[test]
    fn every_case_reaches_the_blessed_file() {
        let cases = shipped();
        let json = blessed(&cases);
        for case in &cases.case {
            assert!(
                json.contains(&case.name),
                "`{}` is not in dist/lineage.cases.json",
                case.name
            );
        }
        assert_eq!(
            json.matches("\"places\"").count(),
            cases.case.len(),
            "a case reached the file without its placements"
        );
    }

    #[test]
    fn a_newline_in_a_graph_survives_the_journey() {
        // The graphs are multi-line, and a JSON encoder that dropped a newline
        // would hand the JavaScript half a different graph than the Rust half
        // read — which is exactly the disagreement this file exists to prevent.
        let cases = shipped();
        let json = blessed(&cases);
        assert!(json.contains("figure ada\\nfigure bob") || json.contains("\\n"));
        assert!(!json.contains("\n      \"graph\": \"figure ada\nfigure"));
    }
}
