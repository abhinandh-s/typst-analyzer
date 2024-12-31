use tower_lsp::lsp_types::{Position, Range, TextEdit, Url};

use crate::backend::Backend;
use crate::prelude::OkSome;

impl Backend {
    pub fn handle_formatting(&self, uri: Url) -> OkSome<Vec<TextEdit>> {
        let mut textedit = Vec::new();
        if let Some(ctx) = self.format_text_document(uri.clone()) {
            if let Some(text_document) = self.doc_map.get(&uri.to_string()) {
                let lines: Vec<&str> = text_document.lines().collect();
                let start = Position {
                    line: 0,
                    character: 0,
                };
                let end = if let Some(last_line) = lines.last() {
                    Position {
                        line: (lines.len() - 1) as u32,
                        character: last_line.chars().count() as u32,
                    }
                } else {
                    start // If the document is empty, the range is just the start position
                };
                let range = Range { start, end };
                textedit.push(TextEdit {
                    range,
                    new_text: ctx,
                });
            }
        }
        Ok(Some(textedit))
    }

    pub fn format_text_document(&self, uri: Url) -> Option<String> {
        let binding = self.ast_map.get(&uri.to_string());

        let config = typstyle_core::Config::default();
        let formatter = typstyle_core::Typstyle::new(config);

        if let Some(ast) = &binding {
            let source = ast.value();
            if let Ok(formatted) = formatter.format_source(source) {
                return Some(formatted);
            }
        }
        None
    }
}
