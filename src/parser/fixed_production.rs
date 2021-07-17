use crate::parser::fixed_production_part::FixedProductionPart;

pub struct FixedProduction<TLex, TSyntax> {
    pub fixed_parts: Vec<FixedProductionPart<TLex, TSyntax>>,
}

impl<TLex, TSyntax> FixedProduction<TLex, TSyntax> {
    pub fn new(fixed_parts: Vec<FixedProductionPart<TLex, TSyntax>>) -> Self {
        Self { fixed_parts }
    }
}
