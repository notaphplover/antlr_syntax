use crate::ast::abstract_syntax_node::AbstractSyntaxNode;
use crate::parser::fix_gap::FixGap;
use crate::parser::fixed_symbol::FixedSymbol;
use crate::token::token::Token;

pub enum FixedProductionPart<TLex, TSyntax> {
    Ok(AbstractSyntaxNode<Token<TLex, TSyntax>>),
    Fixed(FixedSymbol<TLex, TSyntax>),
    Gap(FixGap<TLex, TSyntax>),
}
