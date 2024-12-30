use std::collections::{HashMap, VecDeque};

use tower_lsp::lsp_types::{
    GotoDefinitionParams, GotoDefinitionResponse, Location, Position, Range, Url,
};
use typst_analyzer_analysis::node::node_walker;
use typst_syntax::SyntaxKind;

use crate::backend::{position_to_offset, Backend};
use crate::error_ctx::TypError;
use crate::prelude::*;
use crate::symbols::range_to_location;

pub(crate) trait HandleDefinitions {
    fn provide_definitions(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<GotoDefinitionResponse, Error>;
}

impl HandleDefinitions for Backend {
    fn provide_definitions(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<GotoDefinitionResponse, Error> {
        let uri = params.text_document_position_params.text_document.uri;
        let pos = params.text_document_position_params.position;
        
        while let Ok(pos) = self.definitions(uri.clone()) {

        }

        // let line_start = 
        // dummy return
        Ok(GotoDefinitionResponse::Scalar(Location {
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
        }))
    }
}

#[derive(Debug)]
pub struct DefinitionsMaker {
    pub range: Range,
    pub response: GotoDefinitionResponse,
}

impl Backend {
    pub fn definitions(&self, uri: Url) -> Result<Vec<DefinitionsMaker>, anyhow::Error> {
        let mut definitions = Vec::new();
        let mut ast_labels = HashMap::new();
        let mut ast_references = HashMap::new();
        let binding = self.ast_map.get(&uri.to_string());

        if let Some(ast) = &binding {
            let source = ast.value();
            for node in ast.root().children() {
                if node.kind() == SyntaxKind::Label {
                    // slice out the range of node from ast_map source
                    if let Some(range) = &source.range(node.span()) {
                        let loc = range_to_location(uri.clone(), source, range)?;
                        let ctx = source
                            .get(range.to_owned())
                            .ok_or(anyhow!("Failed to get ctx of ast from range"))?;
                        let label = ctx
                            .strip_prefix("<")
                            .ok_or(anyhow!("failed to strip symbols"))?
                            .strip_suffix(">")
                            .ok_or(anyhow!("failed to strip symbols"))?;
                        ast_labels.insert(label, loc);
                    }
                }
                if node.kind() == SyntaxKind::Ref {
                    // slice out the range of node from ast_map source
                    if let Some(range) = &source.range(node.span()) {
                        let loc = range_to_location(uri.clone(), source, range)?;
                        let ctx = source
                            .get(range.to_owned())
                            .ok_or(anyhow!("Failed to get ctx of ast from range"))?;
                        let reference = ctx
                            .strip_prefix("@")
                            .ok_or(anyhow!("failed to strip symbols"))?;
                        ast_references.insert(reference, loc);
                    }
                }
            }
        }
        for (k, v) in ast_references {
            if let Some(loc) = ast_labels.get(k) {
                // definitions.push(GotoDefinitionResponse::Scalar(loc.to_owned()));
                definitions.push(DefinitionsMaker {
                    range: v.range,
                    response: GotoDefinitionResponse::Scalar(loc.to_owned()),
                });
            }
        }
        Ok(definitions)
    }
}
