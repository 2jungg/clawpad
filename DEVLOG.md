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

## [0.3.1] - 2024-05-24

### Fixed
- **Syntax Error**: Resolved a mismatched bracket in `document.rs`.
- **Transparency**: Improved transparency implementation using `linear_multiply` on panel and window backgrounds.
- **UI Scaling**: Increased default window size to 1400x900 and improved editor area allocation.
- **Editor Experience**: Enabled horizontal scrolling and increased default row count for the text area.

## [0.3.2] - 2024-05-24

### Fixed
- **Transparency Logic**: Re-implemented visuals to use `with_alpha` and explicit frame fills for the central panel.
- **Real-time Updates**: Added `ctx.request_repaint()` to ensure transparency and font changes reflect instantly while sliding.
- **UI Layout**: Improved space allocation for the editor to fill the entire window and properly handle horizontal expansion.
- **Compiler Warnings**: Added `#[allow(dead_code)]` to internal fields intended for future features.

## [0.3.3] - 2024-05-24

### Fixed
- **Build Error**: Fixed compilation error by replacing non-existent `with_alpha` method with `gamma_multiply` for transparency handling.
- **Stability**: Verified build on server environment.
