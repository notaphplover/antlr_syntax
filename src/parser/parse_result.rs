use crate::ast::abstract_syntax_tree::AbstractSyntaxTree;
use crate::parser::failed_symbol::FailedSymbol;
use crate::parser::fixed_symbol::FixedSymbol;
use crate::token::token::Token;

pub enum ParseResult<TLex, TSyntax> {
    Ok(AbstractSyntaxTree<Token<TLex, TSyntax>>),
    Err(FailedSymbol<TLex, TSyntax>),
    Fix(FixedSymbol<TLex, TSyntax>),
}
