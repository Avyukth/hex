# Indic Text Rendering

Hex bundles Google's Noto Sans faces so text it draws itself renders in
native Indic scripts on macOS, Windows, and Linux, regardless of the fonts
the host has installed.

```ts
Transcript text (History, live text, vocabulary, meetings)
└── rendered by Rune with the sans font stack
    ├── Inter (theme) — Latin, unchanged
    └── Noto Sans {Devanagari, Bengali, Tamil, Telugu, Kannada, Malayalam,
        Gujarati, Gurmukhi, Oriya, Meetei Mayek, Ol Chiki} — script fallback
```

The bundled faces are under `assets/fonts/`, each with its OFL licence. They
are appended to the theme's Inter and IBM Plex Mono bytes in
`src/fonts.rs` and named in the sans CSS family list, so parley's
shaper (HarfBuzz semantics) selects the first face that has each glyph. No
pane detects scripts or changes font by hand. The same constants drive the
window and every offscreen proof/golden renderer.

Coverage corresponds to the Indic scripts used by the Indic-Transcribe Canary
model: Devanagari (Hindi, Marathi, Sanskrit, Nepali, Bhojpuri, Bhili, Bodo,
Dogri, Konkani, Maithili, and Devanagari Kashmiri/Sindhi), Bengali (Bangla,
Assamese), Meetei Mayek (Manipuri), Ol Chiki (Santali), Tamil, Telugu, Kannada,
Malayalam, Gujarati, Gurmukhi (Punjabi), and Oriya (Odia). Perso-Arabic script
faces (for Urdu and Perso-Arabic Sindhi/Kashmiri) are not bundled yet.

Copying a transcript puts plain UTF-8 on the clipboard: the destination app
chooses its own fonts, so Hex's bundles do not travel with the text and are
not needed for pasted output.

Checks: `fonts::tests::bundled_faces_cover_every_noto_family_in_the_sans_stack`,
the `rune_ui` golden suite (unchanged images — Latin fallback is untouched),
and a headless `--scene mac-settings-vocabulary` render with native-script
terms.
