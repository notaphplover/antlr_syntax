use crate::parser::failed_production::FailedProduction;
use crate::parser::fix_result::FixResult;
use crate::token::token::Token;

/// Syntax error solver
///
/// Fixes syntax errors of a RecursiveDescentParser
///
/// A fix attempt is the last resource used in order to parse an input.
/// It's safe to make these assumptions:
///
/// 1. Every ancestor of the node has a single child
/// 2. Given the production A → BCD and C → E
///
///    Any attempt to fix C → E results in an attempt to fix A → BCD if the attempt to fix C → E
///    is not successful
///
pub trait SyntaxErrorSolver<TLex, TSyntax> {
    /// Called when a symbol has a single candidate production.
    ///
    /// It's safe to make these assumptions:
    ///
    /// 1. The production could not be parsed.
    /// 2. At least a production's symbol could not be fixed.
    ///
    /// Any attempt to fix a production symbol productions should be useless at this point.
    ///
    /// The solver may consider failing at this point in order to fix the parent production.
    fn fix_failed_production(
        &self,
        tokens: &Vec<Token<TLex, TSyntax>>,
        tokens_position: usize,
        failed_production: &FailedProduction<TLex, TSyntax>,
    ) -> Option<FixResult<TLex, TSyntax>>;

    /// Called when a symbol has multiple candidate productions
    ///
    /// It's safe to make these assumptions:
    ///
    /// 1. No production could be parser.
    /// 2. No fix attempt has been done, an attempt to fix a production symbol productions may be
    ///    worth.
    ///
    /// The solver may consider failing at this point in order to fix the parent production.
    fn fix_failed_productions(
        &self,
        tokens: &Vec<Token<TLex, TSyntax>>,
        tokens_position: usize,
        failed_productions: &Vec<FailedProduction<TLex, TSyntax>>,
    ) -> Option<FixResult<TLex, TSyntax>>;
}
