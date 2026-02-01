# Development Log - Clawpad

## [0.3.0] - 2024-05-24

### Added
- **Minimap**: Implemented a read-only, scaled-down view of the code on the right side of the editor for spatial navigation.
- **Advanced Search**: Added a global search panel that allows searching text across the entire workspace (max depth 3).
- **Glassmorphism & Transparency**: Integrated window transparency and dynamic visual style adjustments via `settings.json`.
- **Distraction-free Mode**: Added a dedicated mode (F11) to hide all UI elements, leaving only the text editor for maximum focus.
- **Multi-Cursor Foundation**: Added infrastructure for multiple cursors and implemented "Ctrl+D" for next occurrence selection logic.

### Refactored
- **v0.3.0 UI Update**: Overhauled the central panel to support side-by-side editor/minimap and editor/preview layouts.
- **Settings**: Added `transparency` field and corresponding UI sliders.

### Technical Notes
- Improved transparency support by enabling `transparent: true` in `eframe::NativeOptions` and using `gamma_multiply` on visuals.
- Implemented `allocate_new_ui` and `UiBuilder` for better sub-region layout management (Minimap).
- Optimized search to avoid unnecessary file reads in deep directories.

## [0.2.0] - 2024-05-23
... (rest of the file)
