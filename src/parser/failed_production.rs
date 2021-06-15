use crate::ast::abstract_syntax_node::AbstractSyntaxNode;
use crate::parser::failed_state::FailedState;
use crate::token::token::Token;

pub struct FailedProduction<TLex, TSyntax> {
    pub failed_symbol: FailedState<TLex, TSyntax>,
    pub parsed_symbols: Vec<AbstractSyntaxNode<Token<TLex, TSyntax>>>,
    pub pending_symbols: Vec<TSyntax>,
}

impl<TLex, TSyntax> FailedProduction<TLex, TSyntax> {
    pub fn new(
        failed_symbol: FailedState<TLex, TSyntax>,
        parsed_symbols: Vec<AbstractSyntaxNode<Token<TLex, TSyntax>>>,
        pending_symbols: Vec<TSyntax>,
    ) -> Self {
        Self { failed_symbol, parsed_symbols, pending_symbols }
    }
}
