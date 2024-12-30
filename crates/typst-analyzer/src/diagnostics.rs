use anyhow::{anyhow, Error};
use tower_lsp::lsp_types::{Diagnostic, DiagnosticRelatedInformation, DiagnosticSeverity, Url};

use crate::backend::Backend;
use crate::symbols::{range_to_location, range_to_lsp_range};
use crate::typ_logger;

impl Backend {
    pub(crate) fn syntax_error(&self, uri: Url) -> Result<Vec<Diagnostic>, Error> {
        let mut diagnostics: Vec<Diagnostic> = Vec::new();
        if let Some(ast) = &self.ast_map.get(&uri.to_string()) {
            for node in ast.root().children() {
                if node.erroneous() {
                    let syntax_err = node.errors();
                    for err in syntax_err {
                        let span = err.span;
                        if let Some(range) = &ast.value().range(span) {
                            let source = ast.value();
                            let msg = err.message;
                            let hints = err.hints;
                            let mut related_info = Vec::new();
                            for hint in hints.clone() {
                                related_info.push(DiagnosticRelatedInformation {
                                    location: range_to_location(uri.clone(), source, range)?,
                                    message: hint.to_string(),
                                })
                            }
                            typ_logger!("hints: {:#?}", hints); // i havn't seen any hint yet :(
                            diagnostics.push(Diagnostic {
                                range: range_to_lsp_range(source, range)?,
                                severity: Some(DiagnosticSeverity::ERROR),
                                source: Some("typst-analyzer".to_owned()),
                                message: msg.to_string(),
                                related_information: Some(related_info),
                                ..Default::default()
                            });
                        }
                    }
                }
            }
        }
        // cuz in some cases the top most [first] error will splash diagnostics error all across the document with
        // false errors dues to the top most error
        let first = diagnostics
            .first()
            .ok_or(anyhow!("failed to collect frist item."))?;
        Ok(vec![first.clone()])
    }
}
