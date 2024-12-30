use crate::prelude::*;
use crate::symbols::range_to_lsp_range;
use tower_lsp::lsp_types::{InlayHint, InlayHintLabel, Position, Url};
use typst_syntax::SyntaxKind;

use crate::backend::Backend;

#[derive(Debug, Clone)]
struct HintMaker<'a> {
    label: &'a str,
    position: Position,
}

impl Backend {
    pub(crate) fn provide_hints(&self, uri: Url) -> Result<Vec<InlayHint>, Error> {
        // Check for unclosed delimiters
        let mut hints = Vec::new();
        if let Ok(mut inlay_hints) = self.inlay_hints(uri.clone()) {
            hints.append(&mut inlay_hints);
        }
        Ok(hints)
    }

    pub fn inlay_hints(&self, uri: Url) -> Result<Vec<InlayHint>, anyhow::Error> {
        let mut hints = Vec::new();
        let mut inlayhints = Vec::new();
        let binding = self.ast_map.get(&uri.to_string());

        if let Some(ast) = &binding {
            let source = ast.value();
            for node in ast.root().children() {
                if node.kind() == SyntaxKind::Linebreak {
                    // slice out the range of node from ast_map source
                    if let Some(range) = &source.range(node.span()) {
                        let loc = range_to_lsp_range(source, range)?;

                        hints.push(HintMaker {
                            label: "linebreak",
                            position: Position {
                                line: loc.end.line,
                                character: loc.end.character,
                            },
                        });
                    }
                }
                if node.kind() == SyntaxKind::Label {
                    if let Some(range) = &source.range(node.span()) {
                        let loc = range_to_lsp_range(source, range)?;

                        hints.push(HintMaker {
                            label: "label",
                            position: Position {
                                line: loc.start.line,
                                character: loc.start.character + 1,
                            },
                        });
                    }
                }
                if node.kind() == SyntaxKind::Ref {
                    if let Some(range) = &source.range(node.span()) {
                        let loc = range_to_lsp_range(source, range)?;

                        hints.push(HintMaker {
                            label: "reference",
                            position: Position {
                                line: loc.start.line,
                                character: loc.start.character + 1,
                            },
                        });
                    }
                }
            }
        }
        for hint in hints {
            inlayhints.push(InlayHint {
                position: hint.position,
                label: InlayHintLabel::String(hint.label.to_owned()),
                kind: Some(tower_lsp::lsp_types::InlayHintKind::TYPE),
                text_edits: None,
                tooltip: None,
                padding_left: Some(true),
                padding_right: Some(true),
                data: None,
            });
        }
        Ok(inlayhints)
    }
}
