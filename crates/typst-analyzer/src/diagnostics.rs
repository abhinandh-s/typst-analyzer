use anyhow::{anyhow, Error};
use tower_lsp::lsp_types::{
    Diagnostic, DiagnosticRelatedInformation, DiagnosticSeverity, Url,
};
use typst_syntax::SyntaxKind;

use crate::backend::Backend;
use crate::symbols::{find_missing_items, range_to_location, range_to_lsp_range, Symbol};
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
        Ok(vec![first.to_owned()])
    }

    pub fn missing_label_error(&self, uri: Url) -> Result<Vec<Diagnostic>, Error> {
        let mut diagnostic_item = Vec::new();
        let mut symbol_vec = Vec::new();
        let mut ast_labels = Vec::new();
        let mut ast_references = Vec::new();
        let binding = self.ast_map.get(&uri.to_string());
        if let Some(ast) = &binding {
            let source = ast.value();
            for node in ast.root().children() {
                if node.kind() == SyntaxKind::Label {
                    // slice out the range of node from ast_map source
                    if let Some(range) = &ast.value().range(node.span()) {
                        let ctx = source
                            .get(range.to_owned())
                            .ok_or(anyhow!("Failed to get ctx of ast from range"))?;
                        let _loc = range_to_location(uri.clone(), source, range)?;
                        let label = ctx
                            .strip_prefix("<")
                            .ok_or(anyhow!("failed to strip symbols"))?
                            .strip_suffix(">")
                            .ok_or(anyhow!("failed to strip symbols"))?;
                        ast_labels.push(label);
                    }
                }

                if node.kind() == SyntaxKind::Ref {
                    // slice out the range of node from ast_map source
                    if let Some(range) = &ast.value().range(node.span()) {
                        let ctx = source
                            .get(range.to_owned())
                            .ok_or(anyhow!("Failed to get ctx of ast from range"))?;
                        let loc = range_to_location(uri.clone(), source, range)?;
                        let reference = ctx
                            .strip_prefix("@")
                            .ok_or(anyhow!("failed to strip symbols"))?;
                        let symbol = Symbol {
                            name: reference.to_owned(),
                            location: loc.clone(),
                            symbol_type: SyntaxKind::Ref,
                        };
                        symbol_vec.push(symbol.clone());
                        ast_references.push(reference);
                    }
                }
            }

            let missing = find_missing_items(&ast_references, &ast_labels);
            for s in &symbol_vec {
                for i in &missing {
                    if s.name == *i {
                        diagnostic_item.push(Diagnostic {
                            range: s.location.range,
                            severity: Some(DiagnosticSeverity::ERROR),
                            source: Some("typst-analyzer".to_owned()),
                            message: "reference is missing label".to_owned(),
                            ..Default::default()
                        });
                    }
                }
            }
        }
        Ok(diagnostic_item)
    }
}
