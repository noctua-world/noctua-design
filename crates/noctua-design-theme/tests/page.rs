//! The page's colour, as a native host reads it.

use noctua_design_theme::{Backdrop, Mode, Palette};

/// A light page is paper whatever the palette, `black` is black only in dark,
/// and the system preference has no colour until the host resolves it.
#[test]
fn a_page_has_a_colour_in_each_appearance() {
    for palette in Palette::ALL {
        assert_eq!(
            palette.page(Mode::Light, Backdrop::Plain),
            Some([255, 255, 255])
        );
        assert_eq!(
            palette.page(Mode::Light, Backdrop::Black),
            Some([255, 255, 255])
        );
        assert_eq!(palette.page(Mode::Dark, Backdrop::Black), Some([0, 0, 0]));
        assert_eq!(palette.page(Mode::System, Backdrop::Plain), None);
        let [r, g, b] = palette
            .page(Mode::Dark, Backdrop::Plain)
            .expect("an appearance");
        assert!(
            u16::from(r) + u16::from(g) + u16::from(b) < 200,
            "{} paints a dark page light",
            palette.as_str()
        );
    }
}

/// The table is not one row written for every palette: two accents tint
/// their dark pages differently.
#[test]
fn two_palettes_paint_two_tinted_pages() {
    assert_ne!(
        Palette::BlueBalanced.page(Mode::Dark, Backdrop::Tinted),
        Palette::AmberVivid.page(Mode::Dark, Backdrop::Tinted)
    );
}
