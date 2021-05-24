use crate::ast::abstract_syntax_node::AbstractSyntaxNode;

pub struct AbstractSyntaxTree<TToken> {
    pub root: AbstractSyntaxNode<TToken>,
}

impl<TToken> AbstractSyntaxTree<TToken> {
    pub fn new(root: AbstractSyntaxNode<TToken>) -> AbstractSyntaxTree<TToken> {
        AbstractSyntaxTree { root }
    }
}
