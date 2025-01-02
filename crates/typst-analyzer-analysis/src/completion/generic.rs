//! # generic
//!
//! The main component of this module is the `TypCmpItem` struct, which encapsulates properties
//! of a completion item, such as its label, type, documentation, and insertion behavior. The
//! `get_cmp` function in its implementation converts these items into LSP-compatible `CompletionItem`s.

use tower_lsp::lsp_types::{
    CompletionItem, CompletionItemKind, CompletionItemLabelDetails, Documentation,
    InsertTextFormat, MarkupContent, MarkupKind,
};

/// Represents a Typst-specific completion item.
///
/// This struct defines the properties of a Typst completion item, such as its label,
/// type, documentation, and insert text. It is used to generate LSP completion items
/// for use in an editor.
///
/// # Fields
/// - `label`: The main text shown in the completion list.
/// - `label_details`: Additional information about the label, typically its category or context.
/// - `kind`: The kind of completion item, such as a constructor, function, or snippet.
/// - `documentation`: A brief description or explanation of the completion item.
/// - `insert_text`: The text to insert into the editor when the item is selected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypCmpItem<'a> {
    pub label: String,
    pub label_details: &'a str,
    pub kind: CompletionItemKind,
    pub documentation: String,
    pub insert_text: String,
}

impl<'a> TypCmpItem<'a> {
    pub fn new(
        label: String,
        label_details: &'a str,
        kind: CompletionItemKind,
        documentation: String,
        insert_text: String,
    ) -> Self {
        Self {
            label,
            label_details,
            kind,
            documentation,
            insert_text,
        }
    }

    /// Converts a list of `TypCmpItem` into LSP-compatible `CompletionItem`s.
    ///
    /// This function takes a vector of `TypCmpItem` and maps each item into a
    /// `CompletionItem` that adheres to the LSP specification. This is useful
    /// for providing intelligent completions in an editor environment.
    ///
    /// # Parameters
    /// - `items`: A vector of `TypCmpItem` structs to be converted.
    ///
    /// # Returns
    /// A vector of `CompletionItem` structs, ready for use in the LSP.
    ///
    /// # Example
    /// ```
    /// use tower_lsp::lsp_types::CompletionItemKind;
    /// use typst_analyzer_analysis::completion::generic::TypCmpItem;
    /// let typ_items = vec![
    ///     TypCmpItem {
    ///         label: "bold".to_owned(),
    ///         label_details: "text formatting",
    ///         kind: CompletionItemKind::SNIPPET,
    ///         documentation: "Make text bold using `*bold*`.".to_owned(),
    ///         insert_text: "*${1:Text}*".to_owned(),
    ///     }
    /// ];
    /// let lsp_items = TypCmpItem::get_cmp(typ_items);
    /// assert_eq!(lsp_items.len(), 1);
    /// ```
    pub fn get_cmp(items: Vec<TypCmpItem>) -> Vec<CompletionItem> {
        let mut cmpitem: Vec<CompletionItem> = Vec::new();
        for item in items {
            let cmp: CompletionItem = CompletionItem {
                label: item.label.to_owned(),
                label_details: Some(CompletionItemLabelDetails {
                    detail: Some(item.label_details.to_owned()),
                    description: None,
                }),
                kind: Some(item.kind),
                documentation: Some(Documentation::MarkupContent(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: item.documentation.to_owned(),
                })),
                insert_text: Some(item.insert_text),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..CompletionItem::default()
            };
            cmpitem.push(cmp);
        }
        cmpitem
    }
}
