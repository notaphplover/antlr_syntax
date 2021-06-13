use crate::ast::abstract_syntax_tree::AbstractSyntaxTree;
use crate::parser::failed_state::FailedState;
use crate::token::token::Token;

pub type ParseResult<TLex, TSyntax> = Result<
    AbstractSyntaxTree<Token<TLex, TSyntax>>,
    FailedState<TLex, TSyntax>,
>;
