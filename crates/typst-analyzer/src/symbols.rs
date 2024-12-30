use anyhow::{anyhow, Error};
use tower_lsp::lsp_types::{DidChangeTextDocumentParams, Location, Position, Range, Url};
use typst_syntax::{Source, SyntaxKind};

use crate::backend::Backend;
use crate::typ_logger;

#[derive(Debug)]
pub struct Symbol {
    pub name: String,        // Symbol's name (e.g., function name)
    pub location: Location,  // Where the symbol is in the document
    pub symbol_type: String, // Type of symbol (e.g., "function", "variable")
}

pub(crate) trait SymbolTable {
    fn populate_symbol_table(
        &self,
        params: DidChangeTextDocumentParams,
    ) -> Result<String, Error>;
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
    ) -> Result<String, Error> {
        if let Some(ast) = &self.ast_map.get(&params.text_document.uri.to_string()) {
            let source = ast.value();
            for node in ast.root().children() {
                if node.kind() == SyntaxKind::FuncCall {
                    // slice out the range of node from ast_map source
                    if let Some(range) = &ast.value().range(node.span()) {
                        let ctx = source
                            .get(range.to_owned())
                            .ok_or(anyhow!("Failed to get ctx of ast from range"))?;
                        let loc =
                            range_to_location(params.text_document.uri.clone(), source, range)?;
                        typ_logger!("Location {:?}", loc);
                        let symbol = Symbol {
                            name: ctx.to_owned(),
                            location: loc.clone(),
                            symbol_type: SyntaxKind::FuncCall.name().to_owned(),
                        };
                        let symbol_table_re = self
                            .symbol_table
                            .insert(symbol.name.clone(), symbol)
                            .ok_or(anyhow!("failed to map symbols into symbol table\n name: {:?}\nLocation: {:?}", ctx, loc));
                        match symbol_table_re {
                            Ok(symbol_table) => {
                                typ_logger!("symbol_table: {:#?}", symbol_table)
                            }
                            Err(_) => continue, // this must do the job
                        }
                    }
                }
                if node.kind() == SyntaxKind::Ref {
                    // typ_logger!(
                    //     "Node kind != SyntaxNode::FuncCall, kind = {:?}",
                    //     &node.kind(),
                    // );
                }
            }
        }
        Ok("dummy_text".to_owned())
    }
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
