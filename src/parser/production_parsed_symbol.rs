use crate::ast::abstract_syntax_node::AbstractSyntaxNode;
use crate::parser::fixed_symbol::FixedSymbol;
use crate::token::token::Token;

pub enum ProductionParsedSymbol<TLex, TSyntax> {
    Ok(AbstractSyntaxNode<Token<TLex, TSyntax>>),
    Fix(FixedSymbol<TLex, TSyntax>),
}
