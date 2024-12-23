//use std::collections::HashMap;
//
//use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind, InsertTextFormat, MarkupContent};
//
//use super::handle::CmpItems;
//
//pub fn bold() -> Vec<CompletionItem> {
//    let item = CmpItems {
//        label: "bold",
//        label_details: "markup",
//        kind: CompletionItemKind::CONSTRUCTOR,
//        documentation: "Strong emphasis",
//        insert_text: format!("*{}*", "${1:Text}"),
//    };
//    CmpItems::provide_cmp_items(item)
//}
//
//pub fn emphasis() -> Vec<CompletionItem> {
//    let item = CmpItems {
//        label: "emphasis",
//        label_details: "markup",
//        kind: CompletionItemKind::CONSTRUCTOR,
//        documentation: "Emphasis",
//        insert_text: format!("_{}_", "${1:Text}"),
//    };
//    CmpItems::provide_cmp_items(item)
//}
//
//pub fn raw_text() -> Vec<CompletionItem> {
//    let item = CmpItems {
//        label: "raw_text",
//        label_details: "markup",
//        kind: CompletionItemKind::CONSTRUCTOR,
//        documentation: "Raw text",
//        insert_text: format!("`{}`", "${1:Text}"),
//    };
//    CmpItems::provide_cmp_items(item)
//}
//
//pub fn label() -> Vec<CompletionItem> {
//    let item = CmpItems {
//        label: "label",
//        label_details: "markup",
//        kind: CompletionItemKind::CONSTRUCTOR,
//        documentation: "Label",
//        insert_text: format!("<{}>", "${1:Text}"),
//    };
//    CmpItems::provide_cmp_items(item)
//}
//
//pub fn reference() -> Vec<CompletionItem> {
//    let item = CmpItems {
//        label: "reference",
//        label_details: "markup",
//        kind: CompletionItemKind::CONSTRUCTOR,
//        documentation: "Reference",
//        insert_text: format!("@{}", "${1:Text}"),
//    };
//    CmpItems::provide_cmp_items(item)
//}
//
//pub fn single_line_comment() -> Vec<CompletionItem> {
//    let item = CmpItems {
//        label: "comment",
//        label_details: "code",
//        kind: CompletionItemKind::FUNCTION,
//        documentation: "Single line comment",
//        insert_text: format!("// {}", "${1:Text}"),
//    };
//    CmpItems::provide_cmp_items(item)
//}
//
//pub fn multi_line_comment() -> Vec<CompletionItem> {
//    let item = CmpItems {
//        label: "comments",
//        label_details: "code",
//        kind: CompletionItemKind::FUNCTION,
//        documentation: "Multi line comment",
//        insert_text: format!("/*\n {}\n*/", "${1:Text}"),
//    };
//    CmpItems::provide_cmp_items(item)
//}
//
//fn get_headers() -> HashMap<String, String> {
//    let mut cmp_item = HashMap::new();
//    let mut item = "=".to_owned();
//
//    // Use a fresh `label` for each iteration
//    for num in 1..7 {
//        let label = format!("h{}", num); // Create the label as "h<num>"
//        cmp_item.insert(label, item.clone());
//        item.push('='); // Add an additional '#' to the item for the next iteration
//    }
//    cmp_item
//}
//
//#[test]
//fn header_test() {
//    let mut expected = HashMap::new();
//    expected.insert("h1".to_owned(), "=".to_owned());
//    expected.insert("h2".to_owned(), "==".to_owned());
//    expected.insert("h3".to_owned(), "===".to_owned());
//    expected.insert("h4".to_owned(), "====".to_owned());
//    expected.insert("h5".to_owned(), "=====".to_owned());
//    expected.insert("h6".to_owned(), "======".to_owned());
//
//    let result = get_headers();
//
//    assert_eq!(
//        result, expected,
//        "The generated HashMap does not match the expected output."
//    );
//}
//
//pub fn headers() -> Vec<CompletionItem> {
//    let header = get_headers();
//    let mut header_items = Vec::new();
//    for (key, val) in header {
//        let num: usize = key[1..].parse().unwrap_or(0);
//        header_items.push(CompletionItem {
//            label: key.to_owned(),
//            label_details: Some(tower_lsp::lsp_types::CompletionItemLabelDetails {
//                detail: Some("Header".to_owned()),
//                description: None,
//            }),
//            kind: Some(CompletionItemKind::CONSTRUCTOR),
//            documentation: Some(tower_lsp::lsp_types::Documentation::MarkupContent(
//                MarkupContent {
//                    kind: tower_lsp::lsp_types::MarkupKind::Markdown,
//                    value: format!("level {} heading", num),
//                },
//            )),
//            insert_text: Some(format!("{} ", val)),
//            insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
//            ..Default::default()
//        });
//    }
//    header_items
//}
//
//// FIX: every thing below
//fn get_lists(symbol: &str) -> HashMap<String, String> {
//    let mut cmp_item = HashMap::new();
//    let mut item = symbol.to_owned();
//
//    // Use a fresh `label` for each iteration
//    for num in 1..7 {
//        let label = format!("bullet_lists_{}x", num); // Create the label as "h<num>"
//        cmp_item.insert(label, item.clone());
//        item.push_str(format!("\n{}", symbol).as_str()); // Add an additional '#' to the item for the next iteration
//    }
//    cmp_item
//}
//
//pub fn bullet_list() -> Vec<CompletionItem> {
//    let mut blist_items = Vec::new();
//    let mut symbols = HashMap::new();
//    symbols.insert(["bullet_list", "Bullet list"], "-");
//    symbols.insert(["numbered_list", "Numbered list"], "+");
//    for (desc, sym) in symbols {
//        let header = get_lists(sym);
//        let description = desc[1];
//        for (key, val) in header {
//            let num: usize = key[1..].parse().unwrap_or(0);
//            let mut index = 0_u8;
//            index += 1;
//            let snip_num = String::from("$") + "{" + index.to_string().as_str() + ":Text}";
//            blist_items.push(CompletionItem {
//                label: desc[0].to_owned() + "_" + index.to_string().as_str(),
//                label_details: Some(tower_lsp::lsp_types::CompletionItemLabelDetails {
//                    detail: Some("markup".to_owned()),
//                    description: None,
//                }),
//                kind: Some(CompletionItemKind::CONSTRUCTOR),
//                documentation: Some(tower_lsp::lsp_types::Documentation::MarkupContent(
//                    MarkupContent {
//                        kind: tower_lsp::lsp_types::MarkupKind::Markdown,
//                        value: format!(
//                            "{} {}",
//                            num,
//                            description.to_owned() + "_" + index.to_string().as_str()
//                        ),
//                    },
//                )),
//                insert_text: Some(format!("{} {}", val, snip_num)),
//                insert_text_format: Some(InsertTextFormat::SNIPPET),
//                ..Default::default()
//            });
//        }
//    }
//    blist_items
//}
