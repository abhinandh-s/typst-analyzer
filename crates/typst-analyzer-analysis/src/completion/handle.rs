use tower_lsp::lsp_types::*;
use typst_syntax::SyntaxKind;

use super::{code, markup};

pub fn generate_completions(
    context: Vec<SyntaxKind>,
) -> Result<Vec<CompletionItem>, anyhow::Error> {
    // Generate completion candidates based on the context
    let mut completions: Vec<CompletionItem> = vec![];
    for kind in context {
        // Check all possible patterns and add relevant completions
        if kind == SyntaxKind::FuncCall {
            completions.append(&mut markup::items());
        }

        if kind == SyntaxKind::LineComment {
            completions.append(&mut typ_comments_cmp());
        }

        if kind == SyntaxKind::BlockComment {
            completions.append(&mut typ_comments_cmp());
        }

        if kind == SyntaxKind::Markup {
            completions.append(&mut markup::items());
            completions.append(&mut markup::constructors());
            completions.append(&mut markup::typ_image_cmp()?);
            completions.append(&mut code::constructors());
        }

        if kind == SyntaxKind::Equation {
            completions.append(&mut markup::items());
        }
    }
    Ok(completions)
}
// Generate completion items based on the context (node type)
fn typ_comments_cmp() -> Vec<CompletionItem> {
    let mut items = Vec::new();

    let comment_ctx = vec![
        ("TODO: ", "todo", "Task comment"),
        ("NOTE: ", "note", "Task comment"),
        ("FIX: ", "fix", "Task comment"),
        ("BUG: ", "bug", "Task comment"),
        ("TEST: ", "test", "Task comment"),
    ];
    // Add more specific completions based on the node kind
    for (insert_text, label, detail) in comment_ctx {
        items.push(CompletionItem {
            label: label.to_owned(),
            kind: Some(CompletionItemKind::TEXT),
            detail: Some(detail.to_owned()),
            insert_text: Some(insert_text.to_owned()),
            ..Default::default()
        });
    }
    items
}
