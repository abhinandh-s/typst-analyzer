use crate::backend::{position_to_offset, Backend};
use tower_lsp::lsp_types::*;
use typst_analyzer_analysis::completion::generate_completions;
use typst_analyzer_analysis::node::node_walker;
use typst_syntax::SyntaxKind;

pub trait TypstCompletion {
    fn handle_completions(&self, doc_pos: TextDocumentPositionParams) -> Vec<CompletionItem>;
}

impl TypstCompletion for Backend {
    fn handle_completions(&self, doc_pos: TextDocumentPositionParams) -> Vec<CompletionItem> {
        let uri: String = doc_pos.text_document.uri.to_string();
        if let Some(text) = self.doc_map.get(&uri) {
            if let Some(position) = position_to_offset(&text, doc_pos.position) {
                if let Some(ast_map_ctx) = self.ast_map.get(&uri) {
                    let syntax_kind: Vec<SyntaxKind> = node_walker(position, &ast_map_ctx);
                    return generate_completions(syntax_kind);
                }
            }
        }
        Vec::new()
    }
}
