# Clawpad

A professional, multi-tab Rust-based text editor built with `eframe` and `ropey`.

## Features

- **Multi-Tab Support**: Edit multiple files simultaneously with a responsive tab system.
- **Syntax Highlighting**: Built-in support for Rust, Python, and Markdown using `syntect`.
- **Markdown Preview**: Side-by-side live preview for Markdown files.
- **Persistent Settings**: Customize your experience with `settings.json` (font size, theme, etc.).
- **File Explorer**: Integrated sidebar for easy navigation.
- **Robust Text Engine**: Powered by `ropey` for efficient handling of large files.
- **Professional UI**: Status bar with line/char counts, language detection, and more.
- **Minimap**: High-level view of your code for quick navigation.
- **Advanced Search**: Workspace-wide search (ripgrep-style) with Ctrl+Shift+F.
- **Glassmorphism**: Elegant transparent UI with adjustable transparency.
- **Distraction-free Mode**: Focus on your code by hiding all UI elements (F11).
- **Multi-Cursor (Basic)**: Support for multiple cursors and selection occurrences (Ctrl+D).

## Installation

Ensure you have Rust installed.

```bash
cargo build --release
```

## Usage

Run the editor:

```bash
cargo run
```

### Shortcuts
- **F11**: Toggle Distraction-free Mode.
- **Ctrl+F**: Toggle Global Search Panel.
- **Ctrl+D**: Select next occurrence (Multi-cursor).
- **File > New Tab**: Create a new document.
- **File > Open**: Open an existing file.
- **File > Save**: Save the current document.
- **View > Show Sidebar**: Toggle the file explorer.
- **View > Markdown Preview**: Toggle the Markdown preview (only for `.md` files).

## Configuration

Settings are stored in `settings.json`:

```json
{
  "font_size": 14.0,
  "font_family": "monospace",
  "theme_dark": true,
  "transparency": 0.9
}
```
