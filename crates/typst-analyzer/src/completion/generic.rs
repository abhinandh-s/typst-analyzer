use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind, InsertTextFormat};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypCmpItem<'a> {
   pub label: &'a str,
   pub label_details: &'a str,
   pub kind: CompletionItemKind,
   pub documentation: &'a str,
   pub insert_text: String,
}

impl TypCmpItem<'_> {
   pub fn provide_cmp_items(item: TypCmpItem) -> Vec<CompletionItem> {
       vec![CompletionItem {
           label: item.label.to_owned(),
           kind: Some(item.kind),
           detail: Some(item.label_details.to_owned()),
           documentation: Some(tower_lsp::lsp_types::Documentation::String(item.documentation.to_owned())),
           insert_text: Some(item.insert_text),
           insert_text_format: Some(InsertTextFormat::SNIPPET),
           ..CompletionItem::default()
       }]
   }
}
