use crate::parser::fixed_production::FixedProduction;

pub struct FixResult<TLex, TSyntax> {
    pub final_token_position: usize,
    pub production: FixedProduction<TLex, TSyntax>,
}
