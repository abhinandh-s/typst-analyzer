use tower_lsp::lsp_types::*;
use typst_syntax::{LinkedNode, SyntaxKind};

use crate::typ_logger;

use super::fonts::get_fonts;
use super::{code, markup, snippets};

pub fn generate_completions(
    context: Vec<LinkedNode>,
) -> Result<Vec<CompletionItem>, anyhow::Error> {
    // Generate completion candidates based on the context
    let mut completions: Vec<CompletionItem> = vec![];
    for node in context {
        // Check all possible patterns and add relevant completions
        //
        // A FuncCall can contain Parameters there args and actual markup content
        // but it cat also contain other FuncCall
        //
        // #emph(    <- FuncCall
        //   text(      <- FuncCall and Args
        //      fallback: true     <- Named
        //   )[     <- ContentBlock
        //      With a function call.   <- Markup
        //   ],
        // )
        if node.kind() == SyntaxKind::FuncCall {
            typ_logger!("inside FuncCall, FuncCall: {:?}", node);
            for child in node.children() {
                //     // typ_logger!("inside text, text: {:?}", child);
                if child.text() == "text" && child.kind() == SyntaxKind::Ident {
                    completions.append(&mut typ_text_func_cmp());
                }
                //     if child.kind() == SyntaxKind::Args {
                //         typ_logger!("inside args, args: {:?}", child);
                //     }
            }
        }

        if node.kind() == SyntaxKind::LineComment {
            completions.append(&mut collect_comments_cmp());
        }

        if node.kind() == SyntaxKind::BlockComment {
            completions.append(&mut collect_comments_cmp());
            completions.append(&mut collect_fonts_cmp());
        }
        if node.kind() == SyntaxKind::Bool {
            completions.append(&mut collect_comments_cmp());
        }
        if node.kind() == SyntaxKind::Markup {}
    }
    completions.append(&mut snippets::collect());
    if let Ok(mut markup_cmp) = markup::collect() {
        completions.append(&mut markup_cmp)
    }
    completions.append(&mut code::collect());
    Ok(completions)
}
// Generate completion items based on the context (node type)
fn collect_comments_cmp() -> Vec<CompletionItem> {
    let comment_ctx = vec![
        ("TODO: ", "todo", "Task comment"),
        ("NOTE: ", "note", "Task comment"),
        ("FIX: ", "fix", "Task comment"),
        ("BUG: ", "bug", "Task comment"),
        ("TEST: ", "test", "Task comment"),
    ];

    comment_ctx
        .into_iter()
        .map(|(insert_text, label, detail)| CompletionItem {
            label: label.to_owned(),
            kind: Some(CompletionItemKind::TEXT),
            detail: Some(detail.to_owned()),
            insert_text: Some(insert_text.to_owned()),
            ..Default::default()
        })
        .collect()
}

/// # font [ str or array ]
///
/// User can provide additional fonts by uploading .ttf or .otf files into project.
/// They should be discovered automatically.
/// The priority is: project fonts > server fonts.
///
/// ```typst
/// #set text(font: "PT Sans")
/// This is sans-serif.
///
/// #set text(font: (
///   "Inria Serif",
///   "Noto Sans Arabic",
/// ))
/// ```
fn collect_fonts_cmp() -> Vec<CompletionItem> {
    let mut items = Vec::new();
    // Add more specific completions based on the node kind
    if let Ok(Some(comment_ctx)) = get_fonts() {
        for label in comment_ctx {
            let _insert_text = label.families;
            items.push(CompletionItem {
                label: "label".to_owned(),
                kind: Some(CompletionItemKind::TEXT),
                detail: Some("detail".to_owned()),
                insert_text: Some("label".to_owned()),
                ..Default::default()
            });
        }
    }
    items
}

// use crate::OkSome;

// Generate completion items based on the context (node type)
/// text(
/// content,
/// str,
/// ) -> content
fn typ_text_func_cmp() -> Vec<CompletionItem> {
    let mut items = Vec::new();

    let comment_ctx = vec![
        (
            "font",
            "font: ${1:str or array},",
            "# Parameters\nfont: str or array",
        ),
        (
            "fallback",
            "fallback: ${1:bool},",
            "# Parameters\nfallback: bool",
        ),
        // style: str,
        // weight: intstr,
        // stretch: ratio,
        // size: length,
        // fill: colorgradientpattern,
        // stroke: nonelengthcolorgradientstrokepatterndictionary,
        // tracking: length,
        // spacing: relative,
        // cjk-latin-spacing: noneauto,
        // baseline: length,
        // overhang: bool,
        // top-edge: lengthstr,
        // bottom-edge: lengthstr,
        // lang: str,
        // region: nonestr,
        // script: autostr,
        // dir: autodirection,
        // hyphenate: autobool,
        // costs: dictionary,
        // kerning: bool,
        // alternates: bool,
        // stylistic-set: noneintarray,
        // ligatures: bool,
        // discretionary-ligatures: bool,
        // historical-ligatures: bool,
        // number-type: autostr,
        // number-width: autostr,
        // slashed-zero: bool,
        // fractions: bool,
        // features: arraydictionary,
    ];
    // Add more specific completions based on the node kind
    for (label, insert_text, detail) in comment_ctx {
        items.push(CompletionItem {
            label: label.to_owned(),
            kind: Some(CompletionItemKind::PROPERTY),
            detail: None,
            insert_text: Some(insert_text.to_owned()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            label_details: Some(CompletionItemLabelDetails {
                detail: Some("Code".to_owned()),
                description: None,
            }),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: detail.to_owned(),
            })),
            ..Default::default()
        });
    }
    items
}
