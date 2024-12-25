use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind};

use super::generic::TypCmpItem;

pub fn constructors() -> Vec<CompletionItem> {
    let mut items = Vec::new();
    // vec of tuples with the constructor name, the label details and insert text
    let constructor: Vec<(&str, &str, String)> = vec![(
        "A line from one point to another.",
        "line",
        "#line(length: ${1:100}%, stroke: (paint: rgb(\"#757575\"), thickness: 0.1pt))"
            .to_owned(),
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
    TypCmpItem::get_cmp(items)
}
