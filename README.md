# Snippy

Rust cross-platform screenshot utility inspired by SnipLite.

## Current status

M0 prototype: capture the primary display, preview it, and save PNG files.

## Roadmap

- M1: multi-display selection, DPI handling, clipboard, global shortcuts
- M2: annotation tools and undo/redo
- M3: system tray and always-on-top pin windows
- M4: QR/barcode decoding and offline OCR
- M5: optional translation providers and signed releases

See PLAN.md for scope, architecture, and milestone acceptance criteria.

## Build

Install stable Rust, then run:

    cargo test
    cargo run

## Design

The UI uses eframe and egui. Screenshots use the screenshots crate. Image output uses image. Settings use directories and serde_json. Recognition and translation will be replaceable backends.

Screenshots stay local by default. Translation will send text only after an explicit user action.

## License

MIT.
