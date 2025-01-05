use std::collections::{hash_map, VecDeque};
use std::env::current_dir;

use anyhow::{anyhow, Error};
use tower_lsp::lsp_types::{Diagnostic, DiagnosticRelatedInformation, DiagnosticSeverity, Url};
use typst_analyzer_analysis::node::print_all_descendants;
use typst_syntax::{LinkedNode, SyntaxKind};

use crate::backend::Backend;
use crate::symbols::{find_missing_items, range_to_location, range_to_lsp_range, Symbol};
use crate::typ_logger;

impl Backend {
    pub(crate) fn provide_diagnostics(&self, uri: Url) -> Result<Vec<Diagnostic>, Error> {
        // Check for unclosed delimiters
        let mut diagnostics = Vec::new(); // check_unclosed_delimiters(&doc);
        if let Ok(mut missing_labels) = self.missing_label_error(uri.clone()) {
            diagnostics.append(&mut missing_labels);
        }
        // -- TEST:
        if let Ok(mut missing_path_error) = self.missing_path_error(uri.clone()) {
            diagnostics.append(&mut missing_path_error);
        }
        if let Ok(mut syntax_err) = self.syntax_error(uri.clone()) {
            diagnostics.append(&mut syntax_err);
        }
        Ok(diagnostics)
    }

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
            for symbol in &symbol_vec {
                if missing.contains(&symbol.name.as_str()) {
                    diagnostic_item.push(Diagnostic {
                        range: symbol.location.range,
                        severity: Some(DiagnosticSeverity::ERROR),
                        source: Some("typst-analyzer".to_owned()),
                        message: "reference is missing label".to_owned(),
                        ..Default::default()
                    });
                }
            }
        }
        Ok(diagnostic_item)
    }

    pub fn missing_path_error(&self, uri: Url) -> Result<Vec<Diagnostic>, Error> {
         let uri_one: String = uri.to_string();
        if let Some(text) = self.ast_map.get(&uri_one) {
        let s = text.value();
            let a = s.root();
            let l = LinkedNode::new(a);
            //    if let Some(position) = position_to_offset(&text, params.position) {
        //        if let Some(ast_map_ctx) = self.ast_map.get(&uri) {
        //            let syntaxnode = &ast_map_ctx.value().text();
        //            let parsed = typst_syntax::parse(syntaxnode);
        //            let linked_node: VecDeque<LinkedNode> = node_walker(position, &parsed);
        //            print_all_descendants(node);
        //        }
        //    }
        }

        let mut imports = String::new();
        let mut diagnostic_item = Vec::new();
        let mut imports_map: hash_map::HashMap<String, Symbol> = hash_map::HashMap::new();

        if let Some(ast) = &self.ast_map.get(&uri.to_string()) {
            let source = ast.value();
            for node in ast.root().children() {
                if node.kind() == SyntaxKind::ModuleImport {
                    // slice out the range of node from ast_map source
                    if let Some(range) = &source.range(node.span()) {
                        if let Some(ctx) = source.get(range.to_owned()) {
                            let loc = range_to_location(uri.clone(), source, range)?;
                            if let Some((_, rhs)) = ctx.split_once("\"") {
                                if let Some((lhs, _)) = rhs.rsplit_once("\"") {
                                    imports.push_str(lhs);
                                    imports_map.insert(
                                        lhs.to_owned(),
                                        Symbol {
                                            name: imports.clone(),
                                            location: loc.clone(),
                                            symbol_type: SyntaxKind::ModuleImport,
                                        },
                                    );
                                }
                            }
                        }
                    }
                }
            }

            imports_map.into_iter().for_each(|(file, symbol)| {
                if let Ok(c_dir) = current_dir() {
                    if !c_dir.join(file.clone()).exists() {
                        diagnostic_item.push(Diagnostic {
                            range: symbol.location.range,
                            severity: Some(DiagnosticSeverity::ERROR),
                            source: Some("typst-analyzer".to_owned()),
                            message: format!("file not found \"{}\"", file),
                            ..Default::default()
                        });
                    }
                }
            });
        }
        Ok(diagnostic_item)
    }
}
