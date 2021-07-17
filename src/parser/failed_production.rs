use crate::parser::failed_symbol::FailedSymbol;
use crate::parser::production_parsed_symbol::ProductionParsedSymbol;

pub struct FailedProduction<TLex, TSyntax> {
    pub failed_symbol: FailedSymbol<TLex, TSyntax>,
    pub parsed_symbols: Vec<ProductionParsedSymbol<TLex, TSyntax>>,
    pub pending_symbols: Vec<TSyntax>,
}

impl<TLex, TSyntax> FailedProduction<TLex, TSyntax> {
    pub fn new(
        failed_symbol: FailedSymbol<TLex, TSyntax>,
        parsed_symbols: Vec<ProductionParsedSymbol<TLex, TSyntax>>,
        pending_symbols: Vec<TSyntax>,
    ) -> Self {
        Self {
            failed_symbol,
            parsed_symbols,
            pending_symbols,
        }
    }
}
