// TODO: analyze the AST of the document to provide context-sensitive suggestions.

use tower_lsp::lsp_types::{
    CompletionItem, CompletionItemKind, Documentation, InsertTextFormat, MarkupContent, MarkupKind,
};

use super::markup;

pub fn handle_completions() -> Vec<CompletionItem> {
    let mut cmp = Vec::new();
    // cmp.extend(snippets());
    // cmp.extend(provide_tables());
    // cmp.extend(provide_markdown_links());
    // cmp.extend(provide_images());
    cmp.extend(markup::bold());
    cmp.extend(markup::emphasis());
    cmp.extend(markup::raw_text());
    cmp.extend(markup::label());
    cmp.extend(markup::reference());
    cmp.extend(markup::headers());
    cmp.extend(markup::single_line_comment());
    cmp.extend(markup::multi_line_comment());
    cmp.extend(markup::bullet_list());
    cmp
}

#[derive(Debug)]
pub struct CmpItems<'a> {
    pub label: &'a str,
    pub label_details: &'a str,
    pub kind: CompletionItemKind,
    pub documentation: &'a str,
    pub insert_text: String,
}

impl CmpItems<'_> {
    pub fn provide_cmp_items(items: CmpItems) -> Vec<CompletionItem> {
        vec![CompletionItem {
            label: items.label.to_owned(),
            label_details: Some(tower_lsp::lsp_types::CompletionItemLabelDetails {
                detail: Some(items.label_details.to_owned()),
                description: None,
            }),
            kind: Some(items.kind),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: items.documentation.to_string(),
            })),
            insert_text: Some(items.insert_text),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        }]
    }
}
