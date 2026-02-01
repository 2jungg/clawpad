mod document;
mod settings;
mod syntax;

use document::Document;
use settings::Settings;
use syntax::SyntaxHighlighter;

use eframe::egui;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

fn main() -> eframe::Result {
    env_logger::init();
    let settings = Settings::load();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1400.0, 900.0])
            .with_title("Clawpad Professional")
            .with_transparent(true),
        ..Default::default()
    };
    eframe::run_native(
        "Clawpad",
        options,
        Box::new(|cc| {
            if settings.theme_dark {
                cc.egui_ctx.set_visuals(egui::Visuals::dark());
            } else {
                cc.egui_ctx.set_visuals(egui::Visuals::light());
            }
            Ok(Box::new(ClawpadApp::new(cc, settings)))
        }),
    )
}

struct ClawpadApp {
    documents: Vec<Document>,
    active_index: usize,
    settings: Settings,
    highlighter: SyntaxHighlighter,
    
    // UI State
    show_sidebar: bool,
    show_preview: bool,
    show_minimap: bool,
    distraction_free: bool,
    sidebar_files: Vec<PathBuf>,
    
    // Search State
    show_search: bool,
    search_query: String,
    search_results: Vec<SearchResult>,
    
    // Markdown Preview Cache
    md_cache: egui_commonmark::CommonMarkCache,
}

struct SearchResult {
    path: PathBuf,
    line_number: usize,
    line_content: String,
}

impl ClawpadApp {
    fn new(_cc: &eframe::CreationContext<'_>, settings: Settings) -> Self {
        let mut app = Self {
            documents: vec![Document::untitled()],
            active_index: 0,
            settings,
            highlighter: SyntaxHighlighter::new(),
            show_sidebar: true,
            show_preview: false,
            show_minimap: true,
            distraction_free: false,
            sidebar_files: Vec::new(),
            show_search: false,
            search_query: String::new(),
            search_results: Vec::new(),
            md_cache: egui_commonmark::CommonMarkCache::default(),
        };
        app.refresh_sidebar();
        app
    }

    fn refresh_sidebar(&mut self) {
        let mut files = Vec::new();
        for entry in WalkDir::new(".").max_depth(2).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if name.starts_with('.') || name == "settings.json" {
                        continue;
                    }
                }
                if path.to_string_lossy().contains("target") {
                    continue;
                }
                files.push(path.to_path_buf());
            }
        }
        self.sidebar_files = files;
    }

    fn active_doc(&self) -> &Document {
        &self.documents[self.active_index]
    }

    fn active_doc_mut(&mut self) -> &mut Document {
        &mut self.documents[self.active_index]
    }

    fn open_file(&mut self, path: PathBuf) {
        // Check if already open
        if let Some(index) = self.documents.iter().position(|d| d.file_path.as_ref() == Some(&path)) {
            self.active_index = index;
            return;
        }

        if let Ok(content) = fs::read_to_string(&path) {
            let doc = Document::new(content, Some(path));
            self.documents.push(doc);
            self.active_index = self.documents.len() - 1;
        }
    }

    fn save_current(&mut self) {
        let doc = self.active_doc_mut();
        if let Some(path) = &doc.file_path {
            if fs::write(path, &doc.content_cache).is_ok() {
                doc.is_dirty = false;
            }
        } else {
            self.save_current_as();
        }
    }

    fn save_current_as(&mut self) {
        if let Some(path) = rfd::FileDialog::new().save_file() {
            let doc = self.active_doc_mut();
            if fs::write(&path, &doc.content_cache).is_ok() {
                doc.file_path = Some(path.clone());
                doc.is_dirty = false;
                // Update language
                doc.language = match path.extension().and_then(|e| e.to_str()) {
                    Some("rs") => "Rust",
                    Some("py") => "Python",
                    Some("md") => "Markdown",
                    _ => "Plain Text",
                }.to_string();
                self.refresh_sidebar();
            }
        }
    }
}

impl eframe::App for ClawpadApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Apply transparency to visuals
        let mut visuals = ctx.style().visuals.clone();
        visuals.panel_fill = visuals.panel_fill.linear_multiply(self.settings.transparency);
        visuals.window_fill = visuals.window_fill.linear_multiply(self.settings.transparency);
        ctx.set_visuals(visuals);

        if !self.distraction_free {
            self.draw_top_panel(ctx);
            self.draw_status_bar(ctx);
            self.draw_sidebar(ctx);
            if self.show_search {
                self.draw_search_panel(ctx);
            }
        }
        self.draw_central_panel(ctx);

        // Shortcuts
        if ctx.input(|i| i.key_pressed(egui::Key::F11)) {
            self.distraction_free = !self.distraction_free;
        }
        if ctx.input(|i| i.modifiers.command && i.key_pressed(egui::Key::F)) {
            self.show_search = !self.show_search;
        }
    }
}

impl ClawpadApp {
    fn draw_top_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New Tab").clicked() {
                        self.documents.push(Document::untitled());
                        self.active_index = self.documents.len() - 1;
                        ui.close_menu();
                    }
                    if ui.button("Open...").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            self.open_file(path);
                        }
                        ui.close_menu();
                    }
                    if ui.button("Save").clicked() {
                        self.save_current();
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Exit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                ui.menu_button("View", |ui| {
                    ui.checkbox(&mut self.show_sidebar, "Show Sidebar");
                    ui.checkbox(&mut self.show_preview, "Show Markdown Preview");
                    ui.checkbox(&mut self.show_minimap, "Show Minimap");
                    ui.checkbox(&mut self.show_search, "Show Search Panel");
                    if ui.checkbox(&mut self.distraction_free, "Distraction-free Mode (F11)").clicked() {
                        ui.close_menu();
                    }
                });

                ui.menu_button("Settings", |ui| {
                    if ui.add(egui::Slider::new(&mut self.settings.font_size, 8.0..=32.0).text("Font Size")).changed() {
                        let _ = self.settings.save();
                    }
                    if ui.add(egui::Slider::new(&mut self.settings.transparency, 0.1..=1.0).text("Transparency")).changed() {
                        let _ = self.settings.save();
                    }
                    if ui.checkbox(&mut self.settings.theme_dark, "Dark Theme").changed() {
                        if self.settings.theme_dark {
                            ctx.set_visuals(egui::Visuals::dark());
                        } else {
                            ctx.set_visuals(egui::Visuals::light());
                        }
                        let _ = self.settings.save();
                    }
                });
            });
        });

        egui::TopBottomPanel::top("tab_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let mut close_tab = None;
                for (i, doc) in self.documents.iter().enumerate() {
                    let name = if doc.is_dirty {
                        format!("* {}", doc.name())
                    } else {
                        doc.name()
                    };
                    
                    let is_active = i == self.active_index;
                    let response = ui.selectable_label(is_active, name);
                    if response.clicked() {
                        self.active_index = i;
                    }
                    if response.middle_clicked() {
                        close_tab = Some(i);
                    }
                }

                if let Some(i) = close_tab {
                    if self.documents.len() > 1 {
                        self.documents.remove(i);
                        if self.active_index >= self.documents.len() {
                            self.active_index = self.documents.len().saturating_sub(1);
                        }
                    }
                }
            });
        });
    }

    fn draw_status_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let doc = self.active_doc();
                ui.label(format!("Language: {}", doc.language));
                ui.separator();
                ui.label(format!("Lines: {}", doc.rope.len_lines()));
                ui.separator();
                ui.label(format!("Chars: {}", doc.rope.len_chars()));
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("Font: {} {}", self.settings.font_family, self.settings.font_size));
                });
            });
        });
    }

    fn draw_sidebar(&mut self, ctx: &egui::Context) {
        if !self.show_sidebar {
            return;
        }

        egui::SidePanel::left("sidebar")
            .resizable(true)
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.heading("Explorer");
                ui.separator();
                
                egui::ScrollArea::vertical().show(ui, |ui| {
                    let files = self.sidebar_files.clone();
                    for path in files {
                        let name = path.file_name().unwrap_or_default().to_string_lossy();
                        if ui.selectable_label(false, name).clicked() {
                            self.open_file(path);
                        }
                    }
                });
                
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    if ui.button("Refresh").clicked() {
                        self.refresh_sidebar();
                    }
                });
            });
    }

    fn draw_search_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::right("search_panel")
            .resizable(true)
            .default_width(300.0)
            .show(ctx, |ui| {
                ui.heading("Global Search");
                ui.horizontal(|ui| {
                    let response = ui.text_edit_singleline(&mut self.search_query);
                    if response.changed() || ui.button("Search").clicked() {
                        self.perform_search();
                    }
                });
                ui.separator();

                egui::ScrollArea::vertical().show(ui, |ui| {
                    let mut to_open = None;
                    for result in &self.search_results {
                        let file_name = result.path.file_name().unwrap_or_default().to_string_lossy();
                        if ui.selectable_label(false, format!("{}:{} - {}", file_name, result.line_number, result.line_content)).clicked() {
                            to_open = Some(result.path.clone());
                        }
                    }
                    if let Some(path) = to_open {
                        self.open_file(path);
                    }
                });
            });
    }

    fn perform_search(&mut self) {
        self.search_results.clear();
        if self.search_query.len() < 2 { return; }

        for entry in WalkDir::new(".").max_depth(3).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() {
                if path.to_string_lossy().contains("target") || path.to_string_lossy().contains(".git") {
                    continue;
                }
                if let Ok(content) = fs::read_to_string(path) {
                    for (i, line) in content.lines().enumerate() {
                        if line.contains(&self.search_query) {
                            self.search_results.push(SearchResult {
                                path: path.to_path_buf(),
                                line_number: i + 1,
                                line_content: line.trim().to_string(),
                            });
                        }
                        if self.search_results.len() > 100 { break; }
                    }
                }
            }
        }
    }

    fn draw_central_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(egui::Color32::TRANSPARENT))
            .show(ctx, |ui| {
            if self.show_preview && self.active_doc().language == "Markdown" {
                ui.columns(2, |columns| {
                    self.draw_editor_with_minimap(&mut columns[0]);
                    self.draw_preview(&mut columns[1]);
                });
            } else {
                self.draw_editor_with_minimap(ui);
            }
        });
    }

    fn draw_editor_with_minimap(&mut self, ui: &mut egui::Ui) {
        if self.show_minimap {
            ui.horizontal(|ui| {
                let editor_width = ui.available_width() - 100.0;
                let height = ui.available_height();
                
                let editor_rect = egui::Rect::from_min_size(ui.cursor().min, egui::vec2(editor_width, height));
                ui.allocate_new_ui(egui::UiBuilder::new().max_rect(editor_rect), |ui| {
                    self.draw_editor(ui);
                });
                
                let minimap_rect = egui::Rect::from_min_size(ui.cursor().min + egui::vec2(editor_width + 5.0, 0.0), egui::vec2(95.0, height));
                ui.allocate_new_ui(egui::UiBuilder::new().max_rect(minimap_rect), |ui| {
                    self.draw_minimap(ui);
                });
            });
        } else {
            self.draw_editor(ui);
        }
    }

    fn draw_minimap(&mut self, ui: &mut egui::Ui) {
        let doc = self.active_doc();
        let content = doc.content_cache.clone();
        
        egui::ScrollArea::vertical()
            .id_salt("minimap_scroll")
            .show(ui, |ui| {
                ui.disable(); // Read-only
                let font_id = egui::FontId::monospace(2.0);
                ui.add(
                    egui::TextEdit::multiline(&mut content.as_str())
                        .font(font_id)
                        .frame(false)
                        .desired_width(f32::INFINITY),
                );
            });
    }

    fn draw_editor(&mut self, ui: &mut egui::Ui) {
        let font_size = self.settings.font_size;
        let font_family = self.settings.font_family.clone();
        let theme = if self.settings.theme_dark { "base16-ocean.dark" } else { "base16-ocean.light" }.to_string();
        let highlighter = &self.highlighter;
        
        let active_index = self.active_index;
        let doc = &mut self.documents[active_index];
        let language = doc.language.clone();
        
        let font_id = if font_family == "monospace" {
            egui::FontId::monospace(font_size)
        } else {
            egui::FontId::new(font_size, egui::FontFamily::Name(font_family.into()))
        };

        // Handle Ctrl+D for next occurrence
        if ui.input(|i| i.modifiers.command && i.key_pressed(egui::Key::D)) {
            if let Some(mut state) = egui::TextEdit::load_state(ui.ctx(), egui::Id::new("editor")) {
                let content = &doc.content_cache;
                if let Some(range) = state.cursor.char_range() {
                    let start = range.primary.index.min(range.secondary.index);
                    let end = range.primary.index.max(range.secondary.index);
                    if start != end {
                        let selected = &content[start..end];
                        if let Some(pos) = content[end..].find(selected) {
                            let new_start = end + pos;
                            let new_end = new_start + selected.len();
                            state.cursor.set_char_range(Some(egui::text::CCursorRange::two(
                                egui::text::CCursor::new(new_start),
                                egui::text::CCursor::new(new_end)
                            )));
                            state.store(ui.ctx(), egui::Id::new("editor"));
                        }
                    }
                }
            }
        }

        egui::ScrollArea::both() // Enable horizontal scroll too
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
                    let mut layout_job = highlighter.highlight(string, &language, &theme);
                    layout_job.wrap.max_width = wrap_width;
                    for section in &mut layout_job.sections {
                        section.format.font_id = font_id.clone();
                    }
                    ui.fonts(|f| f.layout_job(layout_job))
                };

                let text_edit = egui::TextEdit::multiline(&mut doc.content_cache)
                    .id(egui::Id::new("editor"))
                    .font(font_id.clone())
                    .lock_focus(true)
                    .desired_width(f32::INFINITY)
                    .desired_rows(50) // Increase default rows
                    .frame(false)
                    .layouter(&mut layouter);
                
                let output = ui.add_sized(ui.available_size(), text_edit);

                if output.changed() {
                    doc.sync_from_cache();
                }
            });
    }

    fn draw_preview(&mut self, ui: &mut egui::Ui) {
        let content = self.active_doc().content_cache.clone();
        egui::ScrollArea::vertical().show(ui, |ui| {
            egui_commonmark::CommonMarkViewer::new()
                .show(ui, &mut self.md_cache, &content);
        });
    }
}
