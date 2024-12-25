/*!
# Markup

Paragraph break Blank line parbreak
Strong emphasis *strong* strong
Emphasis _emphasis_ emph
Raw text `print(1)` raw
Link https://typst.app/ link
Label <intro> label
Reference @intro ref
TODO: Heading = Heading heading
TODO: Bullet list - item list
TODO: Numbered list + item enum
TODO: Term list / Term: description terms
TODO: Math $x^2$ Math
TODO: Line break \ linebreak
TODO: Smart quote 'single' or "double" smartquote
TODO: Symbol shorthand ~, --- Symbols
TODO: Code expression #rect(width: 1cm) Scripting
TODO: Character escape Tweet at us \#ad Below
TODO: Comment /* block */, // line Below
*/

use std::collections::HashMap;

use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind, InsertTextFormat, MarkupContent};

use super::generic::TypCmpItem;

pub fn items() -> Vec<CompletionItem> {
    let mut items = Vec::new();
    items.append(&mut headers());
    items.append(&mut single_line_comment());
    items.append(&mut multi_line_comment());
    items.append(&mut bold());
    items.append(&mut emphasis());
    items.append(&mut raw_text());
    items.append(&mut label());
    items.append(&mut reference());
    items
}

pub fn constructors() -> Vec<CompletionItem> {
    let mut items = Vec::new();
    // vec of tuples with the constructor name, the label details and insert text
    let constructor: Vec<(&str, &str, String)> = vec![
        ("Paragraph break", "parbreak", "parbreak()\n".to_owned()),
        ("Strong emphasis", "strong", "*${1:Text}*".to_owned()),
        ("Strong emphasis", "bold", "*${1:Text}*".to_owned()),
        ("Emphasis", "emph", "_${1:Text}_".to_owned()),
        ("Raw text", "raw", "`${1:Text}`".to_owned()),
        ("Link", "link", "https://${1:URL}/".to_owned()),
        ("Label", "label", "<${1:Text}>".to_owned()),
        ("Reference", "ref", "@${1:Text}".to_owned()),
        ("Heading", "heading", "${1:Text} ".to_owned()),
        ("Bullet list", "list", "- ${1:Text}".to_owned()),
        ("Numbered list", "enum", "+ ${1:Text}".to_owned()),
        ("Term list", "terms", "/ ${1:Term}: ${2:Description}".to_owned()),
        ("Math", "Math", "${1:Math}".to_owned()),
        ("Line break", "linebreak", "\\".to_owned()),
        ("Smart quote", "smartquote", "'${1:Text}' or \"${2:Text}\"".to_owned()),
        ("Symbol shorthand", "Symbols", "~, ---".to_owned()),
        ("Code expression", "Scripting", "#${1:Code}".to_owned()),
        ("Character escape", "Below", "\\#${1:Text}".to_owned()),
        ("Comment", "Below", "/* ${1:Text} */, // ${2:Text}".to_owned()),
    ];
    for ctx in constructor {
        let item = TypCmpItem {
            label: ctx.1,
            label_details: "markup",
            kind: CompletionItemKind::CONSTRUCTOR,
            documentation: ctx.0,
            insert_text: ctx.2.to_owned(),
        };
        items.push(item);
    } 
    TypCmpItem::provide_cmp_items(items)
}

fn bold() -> Vec<CompletionItem> {
    let item = TypCmpItem {
        label: "bold",
        label_details: "markup",
        kind: CompletionItemKind::CONSTRUCTOR,
        documentation: "Strong emphasis",
        insert_text: format!("*{}*", "${1:Text}"),
    };
    TypCmpItem::provide_cmp_item(item)
}

fn emphasis() -> Vec<CompletionItem> {
    let item = TypCmpItem {
        label: "emphasis",
        label_details: "markup",
        kind: CompletionItemKind::CONSTRUCTOR,
        documentation: "Emphasis",
        insert_text: format!("_{}_", "${1:Text}"),
    };
    TypCmpItem::provide_cmp_item(item)
}

fn raw_text() -> Vec<CompletionItem> {
    let item = TypCmpItem {
        label: "raw_text",
        label_details: "markup",
        kind: CompletionItemKind::CONSTRUCTOR,
        documentation: "Raw text",
        insert_text: format!("`{}`", "${1:Text}"),
    };
    TypCmpItem::provide_cmp_item(item)
}

fn label() -> Vec<CompletionItem> {
    let item = TypCmpItem {
        label: "label",
        label_details: "markup",
        kind: CompletionItemKind::CONSTRUCTOR,
        documentation: "Label",
        insert_text: format!("<{}>", "${1:Text}"),
    };
    TypCmpItem::provide_cmp_item(item)
}

fn reference() -> Vec<CompletionItem> {
    let item = TypCmpItem {
        label: "reference",
        label_details: "markup",
        kind: CompletionItemKind::CONSTRUCTOR,
        documentation: "Reference",
        insert_text: format!("@{}", "${1:Text}"),
    };
    TypCmpItem::provide_cmp_item(item)
}

fn single_line_comment() -> Vec<CompletionItem> {
    let item = TypCmpItem {
        label: "comment",
        label_details: "code",
        kind: CompletionItemKind::FUNCTION,
        documentation: "Single line comment",
        insert_text: format!("// {}", "${1:Text}"),
    };
    TypCmpItem::provide_cmp_item(item)
}

fn multi_line_comment() -> Vec<CompletionItem> {
    let item = TypCmpItem {
        label: "comments",
        label_details: "code",
        kind: CompletionItemKind::FUNCTION,
        documentation: "Multi line comment",
        insert_text: format!("/*\n {}\n*/", "${1:Text}"),
    };
    TypCmpItem::provide_cmp_item(item)
}

fn get_headers() -> HashMap<String, String> {
    let mut cmp_item = HashMap::new();
    let mut item = "=".to_owned();

    // Use a fresh `label` for each iteration
    for num in 1..7 {
        let label = format!("h{}", num); // Create the label as "h<num>"
        cmp_item.insert(label, item.clone());
        item.push('='); // Add an additional '#' to the item for the next iteration
    }
    cmp_item
}

#[test]
fn header_test() {
    let mut expected = HashMap::new();
    expected.insert("h1".to_owned(), "=".to_owned());
    expected.insert("h2".to_owned(), "==".to_owned());
    expected.insert("h3".to_owned(), "===".to_owned());
    expected.insert("h4".to_owned(), "====".to_owned());
    expected.insert("h5".to_owned(), "=====".to_owned());
    expected.insert("h6".to_owned(), "======".to_owned());

    let result = get_headers();

    assert_eq!(
        result, expected,
        "The generated HashMap does not match the expected output."
    );
}

fn headers() -> Vec<CompletionItem> {
    let header = get_headers();
    let mut header_items = Vec::new();
    for (key, val) in header {
        let num: usize = key[1..].parse().unwrap_or(0);
        header_items.push(CompletionItem {
            label: key.to_owned(),
            label_details: Some(tower_lsp::lsp_types::CompletionItemLabelDetails {
                detail: Some("Header".to_owned()),
                description: None,
            }),
            kind: Some(CompletionItemKind::CONSTRUCTOR),
            documentation: Some(tower_lsp::lsp_types::Documentation::MarkupContent(
                MarkupContent {
                    kind: tower_lsp::lsp_types::MarkupKind::Markdown,
                    value: format!("level {} heading", num),
                },
            )),
            insert_text: Some(format!("{} ", val)),
            insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
            ..Default::default()
        });
    }
    header_items
}
