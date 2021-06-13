use crate::ast::abstract_syntax_node::AbstractSyntaxNode;
use crate::parser::failed_state::FailedState;
use crate::token::token::Token;

pub struct FailedProduction<TLex, TSyntax> {
    pub parsed_symbols: Vec<AbstractSyntaxNode<Token<TLex, TSyntax>>>,
    pub pending_symbols: Vec<FailedState<TLex, TSyntax>>,
}

impl<TLex, TSyntax> FailedProduction<TLex, TSyntax> {
    pub fn new(
        parsed_symbols: Vec<AbstractSyntaxNode<Token<TLex, TSyntax>>>,
        pending_symbols: Vec<FailedState<TLex, TSyntax>>,
    ) -> Self {
        Self { parsed_symbols, pending_symbols }
    }
}
