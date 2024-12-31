use crate::prelude::*;
use tower_lsp::lsp_types::{DidChangeTextDocumentParams, Location, Position, Range, Url};
use typst_syntax::{Source, SyntaxKind};

use crate::backend::Backend;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Symbol {
    pub name: String,            // Symbol's name (e.g., function name)
    pub location: Location,      // Where the symbol is in the document
    pub symbol_type: SyntaxKind, // Type of symbol (e.g., "FunCall", "Ref")
}

pub(crate) trait SymbolTable {
    fn populate_symbol_table(
        &self,
        params: DidChangeTextDocumentParams,
    ) -> Result<Vec<Symbol>, Error>;
}

// the node kind is:
//
// FuncCall: 28 [Ident: "footnote", Args: 20 [ContentBlock: 20 [LeftBracket: "[", Markup: 18 [Text: "this is a footnote"], RightBracket: "]"]]]
impl SymbolTable for Backend {
    // this is a fallible function location is varying from the result we got.
    // need additional tesing since inaccurate maipping of symbols led messing up users document when we relay on on it
    fn populate_symbol_table(
        &self,
        params: DidChangeTextDocumentParams,
    ) -> Result<Vec<Symbol>, Error> {
        let mut symbol_vec = Vec::new();
        let mut ast_labels = Vec::new();
        let mut ast_references = Vec::new();
        let binding = self.ast_map.get(&params.text_document.uri.to_string());
        if let Some(ast) = &binding {
            let source = ast.value();
            for node in ast.root().children() {
                if node.kind() == SyntaxKind::Label {
                    // slice out the range of node from ast_map source
                    if let Some(range) = &ast.value().range(node.span()) {
                        let ctx = source
                            .get(range.to_owned())
                            .ok_or(anyhow!("Failed to get ctx of ast from range"))?;
                        let loc =
                            range_to_location(params.text_document.uri.clone(), source, range)?;
                        let label = ctx
                            .strip_prefix("<")
                            .ok_or(anyhow!("failed to strip symbols"))?
                            .strip_suffix(">")
                            .ok_or(anyhow!("failed to strip symbols"))?;
                        let symbol = Symbol {
                            name: label.to_owned(),
                            location: loc.clone(),
                            symbol_type: SyntaxKind::Label,
                        };
                        symbol_vec.push(symbol.clone());
                        ast_labels.push(label);
                        let symbol_table_re = self
                            .symbol_table
                            .insert(symbol.name.clone(), symbol)
                            .ok_or(anyhow!("failed to map symbols into symbol table\n name: {:?}\nLocation: {:?}", ctx, loc));
                        match symbol_table_re {
                            Ok(_symbol_table) => {
                                //     typ_logger!("symbol_table: {:#?}", symbol_table)
                            }
                            Err(_) => continue, // this must do the job
                        }
                    }
                }

                if node.kind() == SyntaxKind::Ref {
                    // slice out the range of node from ast_map source
                    if let Some(range) = &ast.value().range(node.span()) {
                        let ctx = source
                            .get(range.to_owned())
                            .ok_or(anyhow!("Failed to get ctx of ast from range"))?;
                        let loc =
                            range_to_location(params.text_document.uri.clone(), source, range)?;
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
                        let symbol_table_re = self
                            .symbol_table
                            .insert(symbol.name.clone(), symbol)
                            .ok_or(anyhow!("failed to map symbols into symbol table\n name: {:?}\nLocation: {:?}", ctx, loc));
                        match symbol_table_re {
                            Ok(_symbol_table) => {
                                //     typ_logger!("symbol_table: {:#?}", symbol_table)
                            }
                            Err(_) => continue, // this must do the job
                        }
                    }
                }
            }
        }

        let mut diagnostic_item = Vec::new();
        let missing = find_missing_items(&ast_labels, &ast_references);
        for s in &symbol_vec {
            for i in &missing {
                if s.name == *i {
                    diagnostic_item.push(s.clone());
                }
            }
        }

       // typ_logger!("symbol table: {:#?}", symbol_vec);
       // typ_logger!("labels: {:#?}", ast_labels);
       // typ_logger!("references: {:#?}", ast_references);
       // typ_logger!("missing: {:#?}", missing);
        Ok(diagnostic_item)
    }
}

impl Backend {}

pub(crate) fn find_missing_items<T: Eq + std::hash::Hash + Clone>(
    vec1: &[T],
    vec2: &[T],
) -> Vec<T> {
    // Convert vec2 to a HashSet for quick lookups
    let set2: std::collections::HashSet<_> = vec2.iter().cloned().collect();

    // Find elements in vec1 that are not in vec2
    vec1.iter()
        .filter(|item| !set2.contains(item))
        .cloned()
        .collect()
}

pub(crate) fn range_to_location(
    uri: Url,
    source: &Source,
    range: &core::ops::Range<usize>,
) -> Result<Location, Error> {
    let (starting_line, starting_char) = (
        source
            .byte_to_line(range.start)
            .ok_or(anyhow!("Failed to get Location from Source"))?,
        source
            .byte_to_column(range.start)
            .ok_or(anyhow!("Failed to get Location from Source"))?,
    );
    let (ending_line, ending_char) = (
        source
            .byte_to_line(range.end)
            .ok_or(anyhow!("Failed to get Location from Source"))?,
        source
            .byte_to_column(range.end)
            .ok_or(anyhow!("Failed to get Location from Source"))?,
    );
    Ok(Location {
        uri,
        range: Range {
            start: Position {
                line: starting_line as u32,
                character: starting_char as u32,
            },
            end: Position {
                line: ending_line as u32,
                character: ending_char as u32,
            },
        },
    })
}

//pub(crate) fn range_to_position(
//    range: &core::ops::Range<usize>,
//) -> Result<Position, anyhow::Error> {
//    Ok(Position {
//        line: todo!(),
//        character: todo!(),
//    })
//}
pub(crate) fn range_to_lsp_range(
    source: &Source,
    range: &core::ops::Range<usize>,
) -> Result<Range, Error> {
    let (starting_line, starting_char) = (
        source
            .byte_to_line(range.start)
            .ok_or(anyhow!("Failed to get Range from Source"))?,
        source
            .byte_to_column(range.start)
            .ok_or(anyhow!("Failed to get Range from Source"))?,
    );
    let (ending_line, ending_char) = (
        source
            .byte_to_line(range.end)
            .ok_or(anyhow!("Failed to get Range from Source"))?,
        source
            .byte_to_column(range.end)
            .ok_or(anyhow!("Failed to get Range from Source"))?,
    );
    Ok(Range {
        start: Position {
            line: starting_line as u32,
            character: starting_char as u32,
        },
        end: Position {
            line: ending_line as u32,
            character: ending_char as u32,
        },
    })
}
