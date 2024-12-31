use std::collections::VecDeque;

use typst_syntax::{LinkedNode, SyntaxKind, SyntaxNode};

/// Walks down the AST from current cursor position and Returns a VecDeque of SyntaxKind.
/// Must provide markup in vector in all cases since thas is the root.
pub fn kind_walker(cursor: usize, ast: &SyntaxNode) -> VecDeque<SyntaxKind> {
    let linked_root = LinkedNode::new(ast);
    // Find the LinkedNode at the cursor position
    let current_node = linked_root.leaf_at(cursor, typst_syntax::Side::Before);
    // lets get markup too and with kind give completions if the node contains markup we will
    // provide normal static cmp.
    let mut nodes: VecDeque<SyntaxKind> = VecDeque::new();
    if let Some(node) = current_node {
        nodes.push_front(node.clone().kind());
        // Loop to find the parent and its parents
        let mut parent = node.parent();
        while let Some(p) = parent {
            nodes.push_front(p.clone().kind());
            parent = p.parent();
        }
    }
    nodes
}

/// Walks down the AST from current cursor position and Returns a VecDeque of SyntaxKind.
/// Must provide markup in vector in all cases since thas is the root.
pub fn node_walker(cursor: usize, ast: &SyntaxNode) -> VecDeque<LinkedNode> {
    let linked_root = LinkedNode::new(ast);
    // Find the LinkedNode at the cursor position
    let current_node = linked_root.leaf_at(cursor, typst_syntax::Side::Before);
    // lets get markup too and with kind give completions if the node contains markup we will
    // provide normal static cmp.
    let mut nodes: VecDeque<LinkedNode> = VecDeque::new();
    if let Some(node) = current_node {
        nodes.push_front(node.clone());
        // Loop to find the parent and its parents
        let mut parent = node.parent();
        while let Some(p) = parent {
            nodes.push_front(p.clone());
            parent = p.parent();
        }
    }
    nodes
}
