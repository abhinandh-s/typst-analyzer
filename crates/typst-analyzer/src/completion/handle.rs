use tower_lsp::lsp_types::*;
use typst_syntax::{Source, SyntaxKind, SyntaxNode};

use crate::backend::Backend;

pub trait TypstCompletion {
    fn get_completion_items_from_typst(
        &self,
        doc_pos: TextDocumentPositionParams,
    ) -> Vec<CompletionItem>;
    fn find_node_at_position(&self, source: &Source, offset: usize) -> Option<SyntaxNode>;
}

impl TypstCompletion for Backend {
    // Generate completion items based on the AST node context
    fn get_completion_items_from_typst(
        &self,
        doc_pos: TextDocumentPositionParams,
    ) -> Vec<CompletionItem> {
        let uri = doc_pos.text_document.uri.to_string();
        let position = doc_pos.position;
        let mut items = Vec::new();

        if let Some(source) = self.sources.get(&uri) {
            // Convert the position to an offset
            if let Some(offset) = self.position_to_offset(source.text(), position) {
                // Find the node at the given position
                if let Some(node) = self.find_node_at_position(&source, offset) {
                    // Determine the context and generate completion items based on the node
                    match node.kind() {
                        SyntaxKind::Markup => {}
                        SyntaxKind::Ident => {
                            // Add completion for Typst identifiers
                            items.push(CompletionItem {
                                label: "identifier".to_owned(),
                                kind: Some(CompletionItemKind::VARIABLE),
                                detail: Some("Typst identifier".to_owned()),
                                insert_text: Some("identifier".to_owned()),
                                ..Default::default()
                            });
                        }
                        SyntaxKind::FuncCall => {
                            // Add completion for Typst function calls
                            items.push(CompletionItem {
                                label: "function".to_owned(),
                                kind: Some(CompletionItemKind::FUNCTION),
                                detail: Some("Typst function call".to_owned()),
                                insert_text: Some("function()".to_owned()),
                                ..Default::default()
                            });
                        }
                        _ => {
                            // Add generic Typst keywords
                            let keywords = vec![
                                "import", "include", "set", "show", "if", "else", "for",
                                "while",
                                // Add other Typst keywords here...
                            ];

                            for keyword in keywords {
                                items.push(CompletionItem {
                                    label: keyword.to_owned(),
                                    kind: Some(CompletionItemKind::KEYWORD),
                                    detail: Some("Typst keyword".to_owned()),
                                    insert_text: Some(keyword.to_owned()),
                                    ..Default::default()
                                });
                            }
                        }
                    }
                }
            }
        }

        items
    }
    // Helper function to find the node at a given position in the AST
    fn find_node_at_position(&self, source: &Source, offset: usize) -> Option<SyntaxNode> {
        // Recursive function to traverse the syntax tree
        fn traverse(node: &SyntaxNode, offset: usize, source: &Source) -> Option<SyntaxNode> {
            if let Some(range) = source.range(node.span()) {
                if range.start <= offset && offset < range.end {
                    for child in node.children() {
                        if let Some(found) = traverse(child, offset, source) {
                            return Some(found);
                        }
                    }
                    return Some(node.clone());
                }
            }
            None
        }

        traverse(source.root(), offset, source)
    }
}

//// TODO: analyze the AST of the document to provide context-sensitive suggestions.
//
//use tower_lsp::lsp_types::{
//    CompletionItem, CompletionItemKind, Documentation, InsertTextFormat, MarkupContent, MarkupKind,
//};
//
//use super::markup;
//
//pub fn handle_completions() -> Vec<CompletionItem> {
//    let mut cmp = Vec::new();
//    cmp.extend(markup::bold());
//    cmp.extend(markup::emphasis());
//    cmp.extend(markup::raw_text());
//    cmp.extend(markup::label());
//    cmp.extend(markup::reference());
//    cmp.extend(markup::headers());
//    cmp.extend(markup::single_line_comment());
//    cmp.extend(markup::multi_line_comment());
//    cmp.extend(markup::bullet_list());
//    cmp
//}
//
//#[derive(Debug)]
//pub struct CmpItems<'a> {
//    pub label: &'a str,
//    pub label_details: &'a str,
//    pub kind: CompletionItemKind,
//    pub documentation: &'a str,
//    pub insert_text: String,
//}
//
//impl CmpItems<'_> {
//    pub fn provide_cmp_items(items: CmpItems) -> Vec<CompletionItem> {
//        vec![CompletionItem {
//            label: items.label.to_owned(),
//            label_details: Some(tower_lsp::lsp_types::CompletionItemLabelDetails {
//                detail: Some(items.label_details.to_owned()),
//                description: None,
//            }),
//            kind: Some(items.kind),
//            documentation: Some(Documentation::MarkupContent(MarkupContent {
//                kind: MarkupKind::Markdown,
//                value: items.documentation.to_owned(),
//            })),
//            insert_text: Some(items.insert_text),
//            insert_text_format: Some(InsertTextFormat::SNIPPET),
//            ..Default::default()
//        }]
//    }
//}
