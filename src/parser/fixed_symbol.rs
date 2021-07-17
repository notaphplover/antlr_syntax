use crate::parser::fixed_production::FixedProduction;

pub struct FixedSymbol<TLex, TSyntax> {
    pub fixed_production: FixedProduction<TLex, TSyntax>,
    pub symbol_to_derive: TSyntax,
}

impl<TLex, TSyntax> FixedSymbol<TLex, TSyntax> {
    pub fn new(
        fixed_production: FixedProduction<TLex, TSyntax>,
        symbol_to_derive: TSyntax,
    ) -> Self {
        Self {
            fixed_production,
            symbol_to_derive,
        }
    }
}
