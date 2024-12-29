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
