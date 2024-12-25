use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind, InsertTextFormat};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypCmpItem<'a> {
    pub label: String,
    pub label_details: &'a str,
    pub kind: CompletionItemKind,
    pub documentation: &'a str,
    pub insert_text: String,
}

impl TypCmpItem<'_> {
    pub fn get_cmp(items: Vec<TypCmpItem>) -> Vec<CompletionItem> {
        let mut cmpitem: Vec<CompletionItem> = Vec::new();
        for item in items {
            let cmp: CompletionItem = CompletionItem {
                label: item.label.to_owned(),
                label_details: Some(tower_lsp::lsp_types::CompletionItemLabelDetails {
                    detail: Some(item.label_details.to_owned()),
                    description: None,
                }),
                kind: Some(item.kind),
                documentation: Some(tower_lsp::lsp_types::Documentation::String(
                    item.documentation.to_owned(),
                )),
                insert_text: Some(item.insert_text),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..CompletionItem::default()
            };
            cmpitem.push(cmp);
        }
        cmpitem
    }
}
