use std::collections::VecDeque;

use crate::backend::{position_to_offset, Backend};
use tower_lsp::lsp_types::{CompletionItem, TextDocumentPositionParams};
use typst_analyzer_analysis::completion::generate_completions;
use typst_analyzer_analysis::node::node_walker;
use typst_syntax::SyntaxKind;

pub(crate) trait TypstCompletion {
    fn handle_completions(
        &self,
        params: TextDocumentPositionParams,
    ) -> Result<Vec<CompletionItem>, anyhow::Error>;
}

impl TypstCompletion for Backend {
    fn handle_completions(
        &self,
        params: TextDocumentPositionParams,
    ) -> Result<Vec<CompletionItem>, anyhow::Error> {
        let uri: String = params.text_document.uri.to_string();
        if let Some(text) = self.doc_map.get(&uri) {
            if let Some(position) = position_to_offset(&text, params.position) {
                if let Some(ast_map_ctx) = self.ast_map.get(&uri) {
                    let syntaxnode = &ast_map_ctx.value().text();
                    let parsed = typst_syntax::parse(syntaxnode);
                    let syntax_kind: VecDeque<SyntaxKind> = node_walker(position, &parsed);
                    return generate_completions(syntax_kind.into());
                }
            }
        }
        Ok(Vec::new())
    }
}
