use tower_lsp::lsp_types::CompletionItem;

use super::core::{ToTypCmpItem, TypCmpItem};

/// finds missing arguments from function call so we can returns a list of missing completions
pub fn find_missing_args() {}

pub fn collect() -> Vec<CompletionItem> {
    code_mode_ctx()
}

#[derive(Debug)]
pub(crate) struct FuncMaker {
    pub label: String,
    pub documentation: String,
    pub insert_text: String,
}

fn code_mode_ctx() -> Vec<CompletionItem> {
    let items = vec![
        FuncMaker {
        label: "line".to_owned(),
        documentation: "A line from one point to another.".to_owned(),
        insert_text:
            "#line(length: ${1:100}%, stroke: (paint: rgb(\"#757575\"), thickness: 0.1pt))"
                .to_owned(),
    },
        FuncMaker {
        label: "footnote".to_owned(),
        documentation: r#"
# A footnote.

Includes additional remarks and references on the same page with footnotes. A footnote will insert a superscript number that links to the note at the bottom of the page. Notes are numbered sequentially throughout your document and can break across multiple pages.

To customize the appearance of the entry in the footnote listing, see [footnote.entry](https://typst.app/docs/reference/model/footnote/#definitions-entry). The footnote itself is realized as a normal superscript, so you can use a set rule on the [super](https://typst.app/docs/reference/text/super/) function to customize it. You can also apply a show rule to customize only the footnote marker (superscript number) in the running text.
"#.to_owned(),
        insert_text:
            "#footnote[${1:footnote}]"
                .to_owned(),
    }
    ];
    TypCmpItem::convert(items.to_typ_cmp_item())
}
