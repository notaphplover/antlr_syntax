use crate::parser::failed_production::FailedProduction;

pub struct FailedSymbol<TLex, TSyntax> {
    pub failed_productions: Vec<FailedProduction<TLex, TSyntax>>,
    pub symbol_to_derive: TSyntax,
}

impl<TLex, TSyntax> FailedSymbol<TLex, TSyntax> {
    pub fn new(
        failed_productions: Vec<FailedProduction<TLex, TSyntax>>,
        symbol_to_derive: TSyntax,
    ) -> Self {
        Self {
            failed_productions,
            symbol_to_derive,
        }
    }
}
