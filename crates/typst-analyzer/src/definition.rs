use std::collections::VecDeque;

use tower_lsp::lsp_types::{
    GotoDefinitionParams, GotoDefinitionResponse, Location, Position, Range,
};
use typst_analyzer_analysis::node::node_walker;
use typst_syntax::SyntaxKind;

use crate::backend::{position_to_offset, Backend};
use crate::error_ctx::TypError;
use crate::typ_logger;

pub(crate) trait HandleDefinitions {
    fn provide_definitions(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<GotoDefinitionResponse, anyhow::Error>;
}

impl HandleDefinitions for Backend {
    fn provide_definitions(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<GotoDefinitionResponse, anyhow::Error> {
        let uri = params.text_document_position_params.text_document.uri;
        if let Some(text) = self.doc_map.get(&uri.to_string()) {
            if let Some(position) =
                position_to_offset(&text, params.text_document_position_params.position)
            {
                if let Some(ast_map_ctx) = self.ast_map.get(&uri.to_string()) {
                    let syntaxnode = &ast_map_ctx.value().text();
                    let parsed = typst_syntax::parse(syntaxnode);
                    // node_walker will walk throug the AST map from cursor position and return
                    // VecDeque as [Markup, Ref, RefMarker] if cursor is in
                    // a RefMarker ie, reference.
                    let syntax_kind: VecDeque<SyntaxKind> = node_walker(position, &parsed);
                    typ_logger!("syntax_kind: {:?}", syntax_kind);
                    let refmarker = syntax_kind.back().ok_or(TypError::NonCriticalError(
                        "Failed to get SyntaxKind from node_walker",
                    ))?;
                    // refmarker = RefMarker
                    typ_logger!("refmarker: {:?}", refmarker);
                    // dummy return
                    return Ok(GotoDefinitionResponse::Scalar(Location {
                        uri,
                        range: Range {
                            start: Position {
                                line: 1,
                                character: 4,
                            },
                            end: Position {
                                line: 1,
                                character: 4,
                            },
                        },
                    }));
                }
            }
        }
        Err(TypError::Invalid.into())
    }
}
