use egui::text::LayoutJob;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

pub struct SyntaxHighlighter {
    pub ps: SyntaxSet,
    pub ts: ThemeSet,
}

impl SyntaxHighlighter {
    pub fn new() -> Self {
        Self {
            ps: SyntaxSet::load_defaults_newlines(),
            ts: ThemeSet::load_defaults(),
        }
    }

    pub fn highlight(&self, text: &str, language: &str, theme: &str) -> LayoutJob {
        let syntax = self.ps.find_syntax_by_name(language)
            .or_else(|| self.ps.find_syntax_by_extension(language))
            .unwrap_or_else(|| self.ps.find_syntax_plain_text());
        
        let mut h = HighlightLines::new(syntax, &self.ts.themes[theme]);
        let mut job = LayoutJob::default();

        for line in LinesWithEndings::from(text) {
            let ranges: Vec<(Style, &str)> = h.highlight_line(line, &self.ps).unwrap();
            for (style, range) in ranges {
                let color = egui::Color32::from_rgb(
                    style.foreground.r,
                    style.foreground.g,
                    style.foreground.b,
                );
                job.append(
                    range,
                    0.0,
                    egui::TextFormat {
                        font_id: egui::FontId::monospace(14.0),
                        color,
                        ..Default::default()
                    },
                );
            }
        }
        job
    }
}
