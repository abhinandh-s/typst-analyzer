use crate::backend::{position_to_offset, Backend};
use tower_lsp::lsp_types::*;
use typst_syntax::{LinkedNode, Source, Span, SyntaxKind, SyntaxNode};

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
            // Convert the position to an offset
            if let Some(offset) = position_to_offset(source.text(), position) {
                // Find the node at the given position
                if let Some(node) = find_node_at_position(&source, offset) {
                    eprintln!("Node found at offset {:?}: {:?}", offset, node.kind());
                    // Get contextual completion items based on the node and its context
                    items = get_contextual_completion_items(&source, &node, offset);
                }
            }
        } else {
            eprintln!("No source found for the given URI");
        }
        items
    }
}

// Generate completion items based on the context (node type)
fn get_contextual_completion_items(
    _source: &Source,
    node: &SyntaxNode,
    _offset: usize,
) -> Vec<CompletionItem> {
    let mut items = Vec::new();

    // Check sibling nodes to understand the broader context
    if let Some(parent) = find_parent_node(node, node.span()) {
        eprintln!("Parent node found: {:?}", parent.kind());
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

/// Finds the parent node of a given `Span` in a syntax tree starting from the provided `root`.
///
/// # Arguments
/// * `root` - The root of the syntax tree.
/// * `target_span` - The span for which to find the parent node.
///
/// # Returns
/// An `Option<LinkedNode>` containing the parent node if found, or `None` otherwise.
pub fn find_parent_node(root: &SyntaxNode, target_span: Span) -> Option<LinkedNode<'_>> {
    // Initialize traversal at the root
    let mut current_node = LinkedNode::new(root);

    // Traverse the tree upwards to find the parent node containing the target span
    while let Some(parent) = current_node.parent() {
        // Get the range of the current node
        let current_range = current_node.range();

        // Convert target_span into a range
        let target_range = target_span.number()..(target_span.number() + 1);

        // Check if the target range is fully contained within the current range
        if current_range.start <= target_range.start as usize
            && current_range.end >= target_range.end as usize
        {
            eprintln!("Found parent node with kind: {:?}", parent.kind());
            return Some(current_node);
        }

        // Move to the parent node
        current_node = parent.clone();
    }

    // If no parent node is found that contains the target span, return None
    eprintln!("No parent node found for span: {:?}", target_span);
    None
}
