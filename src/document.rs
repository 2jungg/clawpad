use ropey::Rope;
use std::path::PathBuf;

pub struct Document {
    pub rope: Rope,
    pub file_path: Option<PathBuf>,
    pub is_dirty: bool,
    pub language: String,
    // Buffer for egui::TextEdit
    pub content_cache: String,
    pub cursors: Vec<usize>, // Byte offsets for extra cursors
}

impl Document {
    pub fn new(content: String, path: Option<PathBuf>) -> Self {
        let language = if let Some(p) = &path {
            match p.extension().and_then(|e| e.to_str()) {
                Some("rs") => "Rust",
                Some("py") => "Python",
                Some("md") => "Markdown",
                _ => "Plain Text",
            }
        } else {
            "Plain Text"
        }.to_string();

        Self {
            rope: Rope::from_str(&content),
            file_path: path,
            is_dirty: false,
            language,
            content_cache: content,
            cursors: Vec::new(),
        }
    }

    pub fn untitled() -> Self {
        Self::new(String::new(), None)
    }

    pub fn sync_from_cache(&mut self) {
        if self.rope != self.content_cache {
            self.rope = Rope::from_str(&self.content_cache);
            self.is_dirty = true;
        }
    }

    // Placeholder for future multi-selection expansion
}

    pub fn name(&self) -> String {
        self.file_path
            .as_ref()
            .and_then(|p| p.file_name())
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "Untitled".to_string())
    }
}
