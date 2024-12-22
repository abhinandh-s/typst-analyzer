#![allow(dead_code)]
use std::collections::HashMap;

use tower_lsp::lsp_types::{
    CompletionItem, CompletionItemKind, Documentation, InsertTextFormat, MarkupContent, MarkupKind,
};

// use self::markup;

pub mod handle;
pub(crate) mod markup;

// FIX: update
fn table() -> HashMap<String, String> {
    let mut tables = HashMap::new();

    // Generate tables from 1x1 to 7x7 (NxM)
    for n in 1..=7 {
        // Number of rows
        for m in 1..=7 {
            // Number of columns
            let key = format!("table{}x{}", n, m); // Key format: tableNxM

            // Generate table header: Column1 | Column2 | ... | ColumnM
            let header = (1..=m)
                .map(|i| format!("Column{}", i))
                .collect::<Vec<_>>()
                .join(" | ");

            // Generate separator line: --------------- | --------------- | ... | ---------------
            let separator = (0..m)
                .map(|_| "---------------")
                .collect::<Vec<_>>()
                .join(" | ");

            // Generate rows: Item1.1 | Item2.1 | ... | ItemN.1 for N rows
            let mut rows = Vec::new();
            for i in 1..=n {
                let row = (1..=m)
                    .map(|j| format!("Item{}.{}", i, j))
                    .collect::<Vec<_>>()
                    .join(" | ");
                rows.push(row);
            }

            // Join the header, separator, and rows to form the table in markdown format
            let value = format!(
                "| {} |\n| {} |\n| {} |",
                header,
                separator,
                rows.join(" |\n| ")
            );

            tables.insert(key, value);
        }
    }
    tables
}

// FIX: update
fn provide_tables() -> Vec<CompletionItem> {
    let table = table();
    let mut table_items = Vec::new();
    for (key, val) in table {
        let num: usize = key[1..].parse().unwrap_or(0);
        table_items.push(CompletionItem {
            label: key.to_owned(),
            label_details: Some(tower_lsp::lsp_types::CompletionItemLabelDetails {
                detail: Some("Table".to_owned()),
                description: None,
            }),
            kind: Some(CompletionItemKind::CONSTANT),
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
    table_items
}

// FIX: update
fn snippets() -> Vec<CompletionItem> {
    vec![CompletionItem {
        label: "maths".to_owned(),
        label_details: Some(tower_lsp::lsp_types::CompletionItemLabelDetails {
            detail: Some("details from label_details".to_owned()),
            description: None,
        }),
        kind: Some(CompletionItemKind::CONSTANT),
        detail: Some(
            "# Details from CompletionItem
                "
            .to_owned(),
        ),
        documentation: Some(tower_lsp::lsp_types::Documentation::MarkupContent(
            MarkupContent {
                kind: tower_lsp::lsp_types::MarkupKind::Markdown,
                value: "documentation for the function".to_owned(),
            },
        )),
        insert_text: Some("this will be placed".to_owned()),
        insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
        ..Default::default()
    }]
}

// FIX: update
fn provide_markdown_links() -> Vec<CompletionItem> {
    vec![CompletionItem {
        label: "link".to_owned(),
        label_details: Some(tower_lsp::lsp_types::CompletionItemLabelDetails {
            detail: Some("Markdown Link".to_owned()),
            description: None,
        }),
        kind: Some(CompletionItemKind::VARIABLE),
        documentation: Some(Documentation::MarkupContent(MarkupContent {
            kind: MarkupKind::Markdown,
            value: "Insert Markdown link: [LINK TEXT](URL)".to_string(),
        })),
        insert_text: Some(format!("[{}]({})", "${1:Link Text}", "${2:URL}")),
        insert_text_format: Some(InsertTextFormat::SNIPPET),
        ..Default::default()
    }]
}
// FIX: update
fn provide_images() -> Vec<CompletionItem> {
    vec![CompletionItem {
        label: "image".to_owned(),
        label_details: Some(tower_lsp::lsp_types::CompletionItemLabelDetails {
            detail: Some("Image".to_owned()),
            description: None,
        }),
        kind: Some(CompletionItemKind::FUNCTION),
        documentation: Some(Documentation::MarkupContent(MarkupContent {
            kind: MarkupKind::Markdown,
            value: "Insert Image: ![alt text](path/to/image.png)".to_string(),
        })),
        insert_text: Some(format!("![{}]({})", "${1:Alt Text}", "${2:PATH}")),
        insert_text_format: Some(InsertTextFormat::SNIPPET),
        ..Default::default()
    }]
}
