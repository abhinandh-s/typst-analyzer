#![allow(dead_code)]

use anyhow::anyhow;
use tower_lsp::lsp_types::{DidChangeTextDocumentParams, Location, Position, Range, Url};
use typst_syntax::{Source, SyntaxKind, SyntaxNode};

use crate::backend::Backend;
use crate::typ_logger;

#[derive(Debug)]
pub struct Symbol {
    pub name: String,        // Symbol's name (e.g., function name)
    pub location: Location,  // Where the symbol is in the document
    pub symbol_type: String, // Type of symbol (e.g., "function", "variable")
}

pub(crate) trait SymbolTable {
    fn extract_node_ctx(
        &self,
        uri: Url,
        node: &SyntaxNode,
    ) -> Result<Option<(String, Location)>, anyhow::Error>;
    fn populate_symbol_table(
        &self,
        params: DidChangeTextDocumentParams,
    ) -> Result<String, anyhow::Error>;
}

impl SymbolTable for Backend {
    fn extract_node_ctx(
        &self,
        uri: Url,
        node: &SyntaxNode,
    ) -> Result<Option<(String, Location)>, anyhow::Error> {
        let doc = self
            .doc_map
            .get(&uri.to_string())
            .ok_or(anyhow!("Failed to get document ctx"))?;

        if let Some(_text) = self.doc_map.get(&uri.to_string()) {
            //  typ_logger!("document ctx for symbol_table: {:#?}", &text);
        }
        let span = node.span();
       // typ_logger!("span = {:?}, span_id = {:?}, node = {:?}, node_text = {:?}, node_kind = {:?}",
       //     &span,
       //     &span.id(),
       //     &node,
       //     &node.text(),
       //     &node.kind());
        // -- BUG: i am getting span = Span(1), span_id = None which means the node: &SyntaxNode i provided
        //         is not pointing into any source file
        let file_id_re = span.id().ok_or(anyhow!(
            "Failed to get document file id: span = {:?}, span_id = {:?}, node = {:?}, node_text = {:?}, node_kind = {:?}",
            &span,
            &span.id(),
            &node,
            &node.text(),
            &node.kind()
        ));
        match file_id_re {
            Ok(file_id) => {
                let source = Source::new(file_id, doc.value().to_owned());
                let range = source
                    .range(span)
                    .ok_or(anyhow!("Failed to get range for symbol_table"))?;
                let ctx = source
                    .get(range.clone())
                    .ok_or(anyhow!("Failed to get ctx of ast from range"))?;
                // typ_logger!("{:#?}", &ctx);
                let (starting_line, starting_col) = (
                    source
                        .byte_to_line(range.start)
                        .ok_or(anyhow!("Failed to get Location of symbol",))?,
                    source
                        .byte_to_column(range.start)
                        .ok_or(anyhow!("Failed to get Location of symbol",))?,
                );
                let (ending_line, ending_col) = (
                    source
                        .byte_to_line(range.end)
                        .ok_or(anyhow!("Failed to get Location of symbol",))?,
                    source
                        .byte_to_column(range.end)
                        .ok_or(anyhow!("Failed to get Location of symbol",))?,
                );
                let loc = Location {
                    uri,
                    range: Range {
                        start: Position {
                            line: starting_line as u32,
                            character: starting_col as u32,
                        },
                        end: Position {
                            line: ending_line as u32,
                            character: ending_col as u32,
                        },
                    },
                };
                Ok(Some((ctx.to_owned(), loc)))
            }
            Err(_) => Ok(None),
        }
    }
    fn populate_symbol_table(
        &self,
        params: DidChangeTextDocumentParams,
    ) -> Result<String, anyhow::Error> {
        for enrty_asr in &self.ast_map {
            typ_logger!(
                "AST Debugging: URI = {:?}, AST root span = {:?}, AST root span id = {:?}",
                enrty_asr.key(),
                enrty_asr.value().span(),
                enrty_asr.value().span().id()
            );
            for i in enrty_asr.value().children() {
                typ_logger!("children in ast_map: {:?}, span: {:?}, span_id: {:?}", i, i.span(), i.span().id());
            }
        }

        if let Some(ast) = &self.ast_map.get(&params.text_document.uri.to_string()) {
           // typ_logger!(
           //     "AST Debugging: URI = {:?}, AST root span = {:?}",
           //     params.text_document.uri,
           //     ast.span()
           // );
            for node in ast.children() {
                // typ_logger!("inside for loop: node = {:?}", &node);
                if let Some(node_ctx) =
                    self.extract_node_ctx(params.text_document.uri.clone(), node)?
                {
                    //     typ_logger!("inside for loop: node_ctx = {:#?}", &node_ctx);
                    //   typ_logger!("node in ast childrens: {:?}", node);
                    if node.kind() == SyntaxKind::FuncCall {
                        //       typ_logger!("inside node kind = SyntaxNode::FuncCall");
                        let items = node_ctx;
                        let symbol = Symbol {
                            name: items.0,
                            location: items.1,
                            symbol_type: "function".to_owned(),
                        };
                        self.symbol_table.insert(symbol.name.clone(), symbol);
                    //     typ_logger!("symbol_table: {:#?}", self.symbol_table);
                    } else {
                        //   typ_logger!("not inside node kind = SyntaxNode::FuncCall");
                    }
                }
            }
        }

       // typ_logger!("symbol_table: started");
        // get AST of whole document

        // typ_logger!("document ctx for ast_map: {:#?}", &ast);

        // for i in ast.children() {
        //     typ_logger!(
        //         "node = {:?}, node_kind = {:?}, node_text = {:?}, span = {:?}",
        //         i,
        //         i.kind(),
        //         i.text(),
        //         i.span()
        //     );
        // }

        Ok("this".to_owned())
    }
}
