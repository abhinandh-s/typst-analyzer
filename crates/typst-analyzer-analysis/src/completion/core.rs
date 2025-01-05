//! # core
//!
//! The main component of this module is the `TypCmpItem` struct, which encapsulates properties
//! of a completion item, such as its label, type, documentation, and insertion behavior. The
//! `convert` function in its implementation converts these items into LSP-compatible `CompletionItem`s.

use tower_lsp::lsp_types::{
    CompletionItem, CompletionItemKind, CompletionItemLabelDetails, Documentation,
    InsertTextFormat, MarkupContent, MarkupKind,
};

use super::code::FuncMaker;
use super::snippets::SnippetMaker;

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
pub struct TypCmpItem {
    pub label: String,
    pub label_details: String,
    pub kind: CompletionItemKind,
    pub documentation: String,
    pub insert_text: String,
}

impl From<SnippetMaker> for TypCmpItem {
    fn from(value: SnippetMaker) -> Self {
        TypCmpItem {
            label: value.label,
            label_details: "snippet".to_owned(),
            kind: CompletionItemKind::SNIPPET,
            documentation: value.documentation,
            insert_text: value.insert_text,
        }
    }
}

impl From<FuncMaker> for TypCmpItem {
    fn from(value: FuncMaker) -> Self {
        TypCmpItem {
            label: value.label,
            label_details: "code".to_owned(),
            kind: CompletionItemKind::FUNCTION,
            documentation: value.documentation,
            insert_text: value.insert_text,
        }
    }
}

pub trait ToTypCmpItem {
    fn to_typ_cmp_item(self) -> Vec<TypCmpItem>;
}

// Implement the trait for Vec<SnippetMaker>
impl ToTypCmpItem for Vec<SnippetMaker> {
    fn to_typ_cmp_item(self) -> Vec<TypCmpItem> {
        self.into_iter().map(TypCmpItem::from).collect()
    }
}

// Implement the trait for Vec<FuncMaker>
impl ToTypCmpItem for Vec<FuncMaker> {
    fn to_typ_cmp_item(self) -> Vec<TypCmpItem> {
        self.into_iter().map(TypCmpItem::from).collect()
    }
}

impl TypCmpItem {
    pub fn new(
        label: String,
        label_details: String,
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
    /// use typst_analyzer_analysis::completion::core::TypCmpItem;
    /// let typ_items = vec![
    ///     TypCmpItem {
    ///         label: "bold".to_owned(),
    ///         label_details: "text formatting".to_string(),
    ///         kind: CompletionItemKind::SNIPPET,
    ///         documentation: "Make text bold using `*bold*`.".to_owned(),
    ///         insert_text: "*${1:Text}*".to_owned(),
    ///     }
    /// ];
    /// let lsp_items = TypCmpItem::convert(typ_items);
    /// assert_eq!(lsp_items.len(), 1);
    /// ```
    pub fn convert(items: Vec<TypCmpItem>) -> Vec<CompletionItem> {
        items
            .into_iter()
            .map(|item| CompletionItem {
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
            })
            .collect()
    }
}
