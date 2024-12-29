#![allow(dead_code)]

use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind};

use super::generic::TypCmpItem;

pub fn constructors() -> Vec<CompletionItem> {
    let mut items = Vec::new();
    // vec of tuples with the constructor name, the label details and insert text
    let constructor: Vec<(&str, &str, String)> = vec![
        (
            "A line from one point to another.",
            "line",
            "#line(length: ${1:100}%, stroke: (paint: rgb(\"#757575\"), thickness: 0.1pt))"
                .to_owned(),
        ),
        (
            r#"
# A footnote.

Includes additional remarks and references on the same page with footnotes. A footnote will insert a superscript number that links to the note at the bottom of the page. Notes are numbered sequentially throughout your document and can break across multiple pages.

To customize the appearance of the entry in the footnote listing, see [footnote.entry](https://typst.app/docs/reference/model/footnote/#definitions-entry). The footnote itself is realized as a normal superscript, so you can use a set rule on the [super](https://typst.app/docs/reference/text/super/) function to customize it. You can also apply a show rule to customize only the footnote marker (superscript number) in the running text.
"#,
            "footnote",
            "#footnote[${1:footnote}]".to_owned(),
        ),
    ];
    for ctx in constructor {
        let item = TypCmpItem {
            label: ctx.1.to_owned(),
            label_details: "code",
            kind: CompletionItemKind::FUNCTION,
            documentation: ctx.0,
            insert_text: ctx.2.to_owned(),
        };
        items.push(item);
    }
    TypCmpItem::get_cmp(items)
}

#[derive(Debug)]
struct CodeModeItems<'a> {
    label: String,
    label_details: &'a str,
    kind: CompletionItemKind,
    documentation: &'a str,
    insert_text: String,
}
impl<'a> CodeModeItems<'a> {
    pub(crate) fn new(
        label: String,
        label_details: &'a str,
        kind: CompletionItemKind,
        documentation: &'a str,
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
    ///         documentation: "Make text bold using `*bold*`.",
    ///         insert_text: "*${1:Text}*".to_owned(),
    ///     }
    /// ];
    /// let lsp_items = TypCmpItem::get_cmp(typ_items);
    /// assert_eq!(lsp_items.len(), 1);
    /// ```
    pub(crate) fn get_cmp(items: Vec<CodeModeItems>) -> Vec<TypCmpItem> {
        let mut cmpitem: Vec<TypCmpItem> = Vec::new();
        for item in items {
            let cmp: TypCmpItem<'_> = TypCmpItem {
                label: item.label,
                label_details: "code",
                kind: CompletionItemKind::FUNCTION,
                documentation: item.label_details,
                insert_text: item.insert_text,
            };
            cmpitem.push(cmp);
        }
        cmpitem
    }
}
fn code_mode_ctx() -> Result<Vec<CompletionItem>, anyhow::Error> {
    let mut items = Vec::new();

    let _n = TypCmpItem::new(
        "line".to_owned(),
        "A line from one point to another.",
        CompletionItemKind::FUNCTION,
        "A line from one point to another.",
        "#line(length: ${1:100}%, stroke: (paint: rgb(\"#757575\"), thickness: 0.1pt))".to_owned(),
    );

    // vec of tuples with the constructor name, the label details and insert text
    let constructor: Vec<(&str, &str, String)> = vec![(
        "A line from one point to another.",
        "line",
        "#line(length: ${1:100}%, stroke: (paint: rgb(\"#757575\"), thickness: 0.1pt))".to_owned(),
    )];
    for ctx in constructor {
        let item = TypCmpItem {
            label: ctx.1.to_owned(),
            label_details: "code",
            kind: CompletionItemKind::FUNCTION,
            documentation: ctx.0,
            insert_text: ctx.2.to_owned(),
        };
        items.push(item);
    }
    Ok(TypCmpItem::get_cmp(items))
}
