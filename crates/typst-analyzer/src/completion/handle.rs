use tower_lsp::lsp_types::*;
use typst_syntax::{Source, SyntaxKind, SyntaxNode};
use crate::backend::Backend;

pub trait TypstCompletion {
    fn get_completion_items_from_typst(
        &self,
        doc_pos: TextDocumentPositionParams,
    ) -> Vec<CompletionItem>;
}

impl TypstCompletion for Backend {
    fn get_completion_items_from_typst(
        &self,
        doc_pos: TextDocumentPositionParams,
    ) -> Vec<CompletionItem> {
        let uri = doc_pos.text_document.uri.to_string();
        let position = doc_pos.position;
        let mut items = Vec::new();
        eprintln!("Completion request for {:?}", uri);

        if let Some(source) = self.sources.get(&uri) {
            eprintln!("Source found for {:?}", uri);

            // Convert the position to an offset
            if let Some(offset) = self.position_to_offset(source.text(), position) {
                eprintln!("Offset for position {:?}: {:?}", position, offset);

                // Find the node at the given position
                if let Some(node) = find_node_at_position(&source, offset) {
                    eprintln!("Node found at offset {:?}: {:?}", offset, node.kind());

                    // Get contextual completion items based on the node and its context
                    items = get_contextual_completion_items(&source, &node, offset);
                } else {
                    eprintln!("No node found at the given offset");
                }
            } else {
                eprintln!("Invalid position, could not convert to offset");
            }
        } else {
            eprintln!("No source found for the given URI");
        }

        items
    }
}

// Generate completion items based on the context (node type)
fn get_contextual_completion_items(source: &Source, node: &SyntaxNode, offset: usize) -> Vec<CompletionItem> {
    let mut items = Vec::new();

    // Check sibling nodes to understand the broader context
    if let Some(parent) = find_parent_node(source, node, offset) {
        match parent.kind() {
            SyntaxKind::Markup | SyntaxKind::Math | SyntaxKind::Code => {
                items.append(&mut get_markup_math_code_completions());
            }
            _ => {
                items.append(&mut get_generic_completions());
            }
        }
    } else {
        items.append(&mut get_generic_completions());
    }
    
    let comment_ctx = vec![
        ("TODO: ", "todo", "Task comment"),
        ("NOTE: ", "note", "Task comment"),
        ("FIX: ", "fix", "Task comment"),
        ("BUG: ", "bug", "Task comment"),
        ("TEST: ", "test", "Task comment"),
    ];
    // Add more specific completions based on the node kind
    match node.kind() {
        SyntaxKind::Dollar => {
            items.append(&mut get_math_completions());
        }
        // loop though the comment context
        SyntaxKind::BlockComment => {
            for (insert_text, label, detail) in comment_ctx {
                items.push(CompletionItem {
                    label: label.to_owned(),
                    kind: Some(CompletionItemKind::TEXT),
                    detail: Some(detail.to_owned()),
                    insert_text: Some(insert_text.to_owned()),
                    ..Default::default()
                });
            }
        }
        _ => {}
    }

    items
}

// Helper function to get completions for markup, math, and code contexts
fn get_markup_math_code_completions() -> Vec<CompletionItem> {
    let mut items = Vec::new();

    let markup_items = vec![
        ("Paragraph break", "parbreak"),
        ("Strong emphasis", "strong"),
        ("Emphasis", "emph"),
        ("Raw text", "raw"),
        ("Link", "link"),
        ("Label", "label"),
        ("Reference", "ref"),
        ("Heading", "heading"),
        ("Bullet list", "list"),
        ("Numbered list", "enum"),
        ("Term list", "terms"),
        ("Math", "Math"),
        ("Line break", "linebreak"),
        ("Smart quote", "smartquote"),
        ("Symbol shorthand", "Symbols"),
        ("Code expression", "Scripting"),
        ("Character escape", "Below"),
        ("Comment", "Below"),
    ];

    for (detail, item) in markup_items {
        items.push(CompletionItem {
            label: item.to_owned(),
            kind: Some(CompletionItemKind::TEXT),
            detail: Some(detail.to_owned()),
            insert_text: Some(item.to_owned()),
            ..Default::default()
        });
    }

    items
}

// Helper function to get math-specific completions
fn get_math_completions() -> Vec<CompletionItem> {
    let mut items = Vec::new();

    let math_items = vec![
        ("Fraction", r"\frac{}{}"),
        ("Square Root", r"\sqrt{}"),
        ("Summation", r"\sum_{}^{}"),
        ("Integral", r"\int_{}^{}"),
        ("Subscript", r"_{ }"),
        ("Superscript", r"^{ }"),
    ];

    for (detail, item) in math_items {
        items.push(CompletionItem {
            label: item.to_owned(),
            kind: Some(CompletionItemKind::TEXT),
            detail: Some(detail.to_owned()),
            insert_text: Some(item.to_owned()),
            ..Default::default()
        });
    }

    items
}

// Helper function to get generic completions
fn get_generic_completions() -> Vec<CompletionItem> {
    let mut items = Vec::new();

    let keywords = vec![
        "import",
        "include",
        "set",
        "show",
        "if",
        "else",
        "for",
        "while",
        "parbreak",
        "strong",
        "emph",
        "raw",
        "link",
        "label",
        "ref",
        "heading",
        "list",
        "enum",
        "terms",
        "Math",
        "linebreak",
        "smartquote",
        "Symbols",
        "Scripting",
        "Below",
    ];

    for keyword in keywords {
        items.push(CompletionItem {
            label: keyword.to_owned(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some("Typst keyword".to_owned()),
            insert_text: Some(keyword.to_owned()),
            ..Default::default()
        });
    }

    items
}

// Helper function to find the node at a given position in the AST
fn find_node_at_position(source: &Source, offset: usize) -> Option<SyntaxNode> {
    // Recursive function to traverse the syntax tree
    fn traverse(node: &SyntaxNode, offset: usize, source: &Source) -> Option<SyntaxNode> {
        if let Some(range) = source.range(node.span()) {
            if range.start <= offset && offset < range.end {
                for child in node.children() {
                    if let Some(found) = traverse(child, offset, source) {
                        return Some(found);
                    }
                }
                return Some(node.clone());
            }
        }
        None
    }
    traverse(source.root(), offset, source)
}

// Helper function to find the parent node of a given node
fn find_parent_node(source: &Source, node: &SyntaxNode, offset: usize) -> Option<SyntaxNode> {
    // Traverse the tree from the root to find the parent node
    fn traverse_parent(root: &SyntaxNode, target: &SyntaxNode, source: &Source) -> Option<SyntaxNode> {
        for child in root.children() {
            if *child == *target {
                return Some(root.clone());
            }
            if let Some(parent) = traverse_parent(child, target, source) {
                return Some(parent);
            }
        }
        None
    }
    traverse_parent(source.root(), node, source)
}
