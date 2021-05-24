pub struct AbstractSyntaxNode<TToken> {
    pub child_nodes: Vec<AbstractSyntaxNode<TToken>>,
    pub token: TToken,
}

impl<TToken> AbstractSyntaxNode<TToken> {
    pub fn new(
        child_nodes: Vec<AbstractSyntaxNode<TToken>>,
        token: TToken,
    ) -> AbstractSyntaxNode<TToken> {
        AbstractSyntaxNode { child_nodes, token }
    }
}
