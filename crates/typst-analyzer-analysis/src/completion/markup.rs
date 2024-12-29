//! # Markup Module
//!
//! This module provides completion items for markup-related constructs in Typst.
//! The functions within this module generate completion items for various markup
//! elements such as headings, lists, inline formatting, and more. These completion
//! items can be used by a Language Server Protocol (LSP) implementation to assist
//! users in writing Typst documents.
//!
//! ## Supported Markup Items
//! - Paragraph break Blank line parbreak
//! - Strong emphasis *strong* strong
//! - Emphasis _emphasis_ emph
//! - Raw text `print(1)` raw
//! - Link https://typst.app/ link
//! - Label <intro> label
//! - Reference @intro ref
//! - Bullet list - item list
//! - Numbered list + item enum
//! - Term list / Term: description terms
//! - Line break \ linebreak
//! - Smart quote 'single' or "double" smartquote
//! - Code expression #rect(width: 1cm) Scripting
//! - Comment /* block */, // line Below
//! - Heading = Heading heading
//! - Math $x^2$ Math
//!   TODO: Can we do anything about this
//!     Symbol shorthand ~, --- Symbols
//!     Character escape Tweet at us \#ad Below
//!     image

use std::collections::HashMap;

use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind, InsertTextFormat, MarkupContent};

use crate::{get_images, typ_logger};

use super::generic::TypCmpItem;

pub fn cmp_items() -> Vec<CompletionItem> {
    let mut items = Vec::new();
    items.append(&mut headers());
    items.append(&mut constructors());
    items
}

fn constructors() -> Vec<CompletionItem> {
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
        ("Bullet list", "bullet-list", "- ${1:Item}".to_owned()),
        ("Numbered list", "numbered-list", "+ ${1:Item}".to_owned()),
        (
            "Term list",
            "terms",
            "/ ${1:Term}: ${2:Description}".to_owned(),
        ),
        ("Line break", "linebreak", "\\".to_owned()),
        ("Smart quote", "smartquote", "'${1:Text}'".to_owned()),
        ("Code expression", "Scripting", "#${1:Code}".to_owned()),
        ("Comment", "line-comment", "// ${1:Text}".to_owned()),
        ("Comment", "block-comment", "/*\n ${1:Text} \n*/".to_owned()),
        ("Maths Block", "maths", "\\$ ${1:Text} \\$".to_owned()),
    ];
    for ctx in constructor {
        let item = TypCmpItem {
            label: ctx.1.to_owned(),
            label_details: "markup",
            kind: CompletionItemKind::CONSTRUCTOR,
            documentation: ctx.0,
            insert_text: ctx.2.to_owned(),
        };
        items.push(item);
    }
    TypCmpItem::get_cmp(items)
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

pub fn typ_image_cmp() -> Result<Vec<CompletionItem>, anyhow::Error> {
    let mut items = Vec::new();
    let images = get_images()?;
    typ_logger!("image: {:#?}", images);
    for item in images {
        let image = item.to_string_lossy().to_string();
        let item = TypCmpItem {
            label: "image".to_owned(),
            label_details: "markup",
            kind: CompletionItemKind::FILE,
            documentation: "Image",
            insert_text: format!("#image(\"{}\", width: 100%)", image),
        };
        typ_logger!("image: {}", image);
        items.push(item);
    }
    Ok(TypCmpItem::get_cmp(items))
}
