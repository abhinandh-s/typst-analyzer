use typst_syntax::{LinkedNode, SyntaxKind, SyntaxNode};

/// Walks down the AST from current cursor position and Returns a vector of SyntaxKind.
/// Must provide markup in vector in all cases since thas is the root.
pub fn node_walker(pos: usize, ast: &SyntaxNode) -> Vec<SyntaxKind> {
    let linked_root = LinkedNode::new(ast);
    // Find the LinkedNode at the cursor position
    let current_node = linked_root.leaf_at(pos, typst_syntax::Side::Before);
    // lets get markup too and with kind give completions if the node contains markup we will
    // provide normal static cmp.
    let mut nodes: Vec<SyntaxKind> = Vec::new();
    if let Some(node) = current_node {
        nodes.push(node.clone().kind());
        // Loop to find the parent and its parents
        let mut parent = node.parent();
        while let Some(p) = parent {
            nodes.push(p.clone().kind());
            parent = p.parent();
        }
    }
    nodes
}
