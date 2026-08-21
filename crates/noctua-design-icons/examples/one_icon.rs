//! One icon, drawn into a real linked binary.
//!
//! # This exists to be measured, not to be run
//!
//! The whole icon layer is arranged around one claim: an application that draws
//! one icon pays for one icon, however many the system has. Every drawing is an
//! associated `const`, and a `const` is materialised at its use sites and at no
//! others — so the claim follows from the language rather than from an
//! optimisation, and it holds at every optimisation level.
//!
//! A claim nothing measures is a claim that lapses. `cargo xtask check` builds
//! this in release and reads the bytes back: `menu`'s geometry has to be in
//! there, and several drawings nothing here names have to be absent. The first
//! half is what stops the test passing because the needle was misspelled.
//!
//! It prints, rather than doing nothing, so that the reference cannot be
//! optimised away as dead — which would make the measurement true for a reason
//! that has nothing to do with the design.

use noctua_design_icons::IconDrawing;

fn main() {
    let icon = IconDrawing::MENU;
    println!("{} {} {}", icon.name, icon.view_box, icon.body);
}
