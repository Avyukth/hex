//! Fonts for Hex's Rune windows and offscreen proofs.
//!
//! Rune shapes complex scripts with HarfBuzz semantics, but it can only shape
//! a script when a face with those glyphs is in its collection. Hex bundles
//! Google's Noto Sans faces for every script used by Indic-Transcribe's 25
//! languages, so history, live text, and vocabulary render in native script
//! on macOS, Windows, and Linux without depending on the host's fonts.
//!
//! The faces from `rune-theme-opencode` stay first and keep their names, so
//! Latin text is unchanged. Each Noto face is appended as a fallback family;
//! parley selects the first family in the CSS list that actually has a glyph.
//! The OFL licence for every bundled face is beside it in `assets/fonts`.

use rune_theme_opencode as theme;

/// Bytes registered with every `TextSystem`. The theme's Inter and IBM Plex
/// Mono faces come first; the Noto faces follow.
// The first three entries are `theme::FONTS` (Inter, Plex Mono regular and
// medium). Indexing a const slice keeps this a plain `&'static [&[u8]]`.
pub const FONTS: &[&[u8]] = &[
    theme::FONTS[0],
    theme::FONTS[1],
    theme::FONTS[2],
    include_bytes!("../../assets/fonts/NotoSansDevanagari[wdth,wght].ttf"),
    include_bytes!("../../assets/fonts/NotoSansBengali[wdth,wght].ttf"),
    include_bytes!("../../assets/fonts/NotoSansTamil[wdth,wght].ttf"),
    include_bytes!("../../assets/fonts/NotoSansTelugu[wdth,wght].ttf"),
    include_bytes!("../../assets/fonts/NotoSansKannada[wdth,wght].ttf"),
    include_bytes!("../../assets/fonts/NotoSansMalayalam[wdth,wght].ttf"),
    include_bytes!("../../assets/fonts/NotoSansGujarati[wdth,wght].ttf"),
    include_bytes!("../../assets/fonts/NotoSansGurmukhi[wdth,wght].ttf"),
    include_bytes!("../../assets/fonts/NotoSansOriya[wdth,wght].ttf"),
    include_bytes!("../../assets/fonts/NotoSansMeeteiMayek[wght].ttf"),
    include_bytes!("../../assets/fonts/NotoSansOlChiki[wght].ttf"),
];

/// `(sans, mono)` CSS family lists. `Family::Sans` styles resolve through
/// this stack, so no pane needs per-script detection.
pub const FAMILIES: (&str, &str) = (
    "Inter, Noto Sans Devanagari, Noto Sans Bengali, Noto Sans Tamil, \
     Noto Sans Telugu, Noto Sans Kannada, Noto Sans Malayalam, \
     Noto Sans Gujarati, Noto Sans Gurmukhi, Noto Sans Oriya, \
     Noto Sans Meetei Mayek, Noto Sans Ol Chiki",
    "IBM Plex Mono",
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bundled_faces_cover_every_noto_family_in_the_sans_stack() {
        assert!(FONTS.len() > theme::FONTS.len());
        let (sans, mono) = FAMILIES;
        assert_eq!(mono, "IBM Plex Mono");
        assert!(sans.starts_with("Inter, "));
        // Every Noto family named in the CSS stack has bundled bytes. The
        // order follows FONTS after the three theme faces.
        for name in [
            "Devanagari",
            "Bengali",
            "Tamil",
            "Telugu",
            "Kannada",
            "Malayalam",
            "Gujarati",
            "Gurmukhi",
            "Oriya",
            "Meetei Mayek",
            "Ol Chiki",
        ] {
            let family = format!("Noto Sans {name}");
            assert!(
                sans.contains(&family),
                "{family} is missing from the sans stack"
            );
        }
        assert_eq!(FONTS.len(), theme::FONTS.len() + 11);
    }
}
