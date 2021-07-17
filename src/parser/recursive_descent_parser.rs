use std::hash::Hash;

use crate::ast::abstract_syntax_node::AbstractSyntaxNode;
use crate::ast::abstract_syntax_tree::AbstractSyntaxTree;
use crate::grammar::context_free_grammar::ContextFreeGrammar;
use crate::grammar::first_follow_symbols::FirstFollowSymbols;
use crate::parser::failed_production::FailedProduction;
use crate::parser::failed_symbol::FailedSymbol;
use crate::parser::fix_result::FixResult;
use crate::parser::fixed_production::FixedProduction;
use crate::parser::fixed_production_part::FixedProductionPart;
use crate::parser::fixed_symbol::FixedSymbol;
use crate::parser::parse_result::ParseResult;
use crate::parser::production_parsed_symbol::ProductionParsedSymbol;
use crate::parser::recursive_descent_parser_transitions::RecursiveDescentParserTransitions;
use crate::parser::syntax_error_solver::SyntaxErrorSolver;
use crate::token::token::Token;
use std::marker::PhantomData;

enum ParseSymbolResult<'a, TLex, TSyntax> {
    Ok(ParsingState<'a, TLex, TSyntax, std::vec::IntoIter<&'a Vec<TSyntax>>>),
    Err(FailedSymbol<TLex, TSyntax>),
    Fix(FixedState<TLex, TSyntax>),
}

enum ParseProductionResult<TLex, TSyntax> {
    Ok(AbstractSyntaxNode<Token<TLex, TSyntax>>),
    Err(FailedProduction<TLex, TSyntax>),
    Fix(FixedProduction<TLex, TSyntax>),
}

struct FixedState<TLex, TSyntax> {
    final_token_position: usize,
    fixed_symbol: FixedSymbol<TLex, TSyntax>,
}

impl<TLex, TSyntax> FixedState<TLex, TSyntax> {
    pub fn new(final_token_position: usize, fixed_symbol: FixedSymbol<TLex, TSyntax>) -> Self {
        Self {
            final_token_position,
            fixed_symbol,
        }
    }
}

struct ParsingState<'a, TLex, TSyntax: 'a, TIter: Iterator<Item = &'a Vec<TSyntax>>> {
    initial_token_position: usize,
    final_token_position: usize,
    prod_iter_option: Option<TIter>,
    node: AbstractSyntaxNode<Token<TLex, TSyntax>>,
}

impl<'a, TLex, TSyntax, TIter: Iterator<Item = &'a Vec<TSyntax>>>
    ParsingState<'a, TLex, TSyntax, TIter>
{
    pub fn new(
        initial_token_position: usize,
        final_token_position: usize,
        prod_iter_option: Option<TIter>,
        node: AbstractSyntaxNode<Token<TLex, TSyntax>>,
    ) -> Self {
        ParsingState {
            initial_token_position,
            final_token_position,
            prod_iter_option,
            node,
        }
    }
}

enum State<'a, TLex, TSyntax: 'a, TIter: Iterator<Item = &'a Vec<TSyntax>>> {
    Parsing(ParsingState<'a, TLex, TSyntax, TIter>),
    Fixed(FixedState<TLex, TSyntax>),
}

impl<'a, TLex, TSyntax: 'a, TIter: Iterator<Item = &'a Vec<TSyntax>>>
    State<'a, TLex, TSyntax, TIter>
{
    pub fn is_fixed(&self) -> bool {
        matches!(*self, Self::Fixed(_))
    }
}

pub struct RecursiveDescentParser<'a, TLex, TSyntax, TSolver: SyntaxErrorSolver<TLex, TSyntax>> {
    grammar: &'a ContextFreeGrammar<TSyntax>,
    marker: PhantomData<TLex>,
    syntax_error_solver: Option<TSolver>,
    transitions: RecursiveDescentParserTransitions<TSyntax>,
}

impl<
        'a,
        TLex: Clone + 'a,
        TSyntax: Clone + Eq + Hash,
        TSolver: SyntaxErrorSolver<TLex, TSyntax>,
    > RecursiveDescentParser<'a, TLex, TSyntax, TSolver>
{
    pub fn from_grammar(grammar: &'a ContextFreeGrammar<TSyntax>) -> Self {
        let first_follow_symbols = FirstFollowSymbols::from(grammar);

        Self::from_grammar_and_first_follow_symbols(grammar, &first_follow_symbols)
    }

    pub fn from_grammar_and_first_follow_symbols(
        grammar: &'a ContextFreeGrammar<TSyntax>,
        first_follow_symbols: &FirstFollowSymbols<TSyntax>,
    ) -> Self {
        Self::from(grammar, first_follow_symbols, None)
    }

    pub fn from_grammar_and_solver(
        grammar: &'a ContextFreeGrammar<TSyntax>,
        syntax_error_solver: TSolver,
    ) -> Self {
        let first_follow_symbols = FirstFollowSymbols::from(grammar);

        Self::from(grammar, &first_follow_symbols, Some(syntax_error_solver))
    }

    pub fn parse_from_tokens<TIter: Iterator<Item = Token<TLex, TSyntax>>>(
        &self,
        tokens_iterator: TIter,
    ) -> ParseResult<TLex, TSyntax> {
        let tokens_vector = Self::iterator_to_vec(tokens_iterator);

        if tokens_vector.len() == 0 {
            panic!("Expecting at least one token!")
        }

        let symbol_to_derive = self.grammar.get_initial_symbol();
        let token_position: usize = 0;
        let first_token = tokens_vector.get(token_position).unwrap();

        let first_token_productions =
            self.inner_get_token_productions(symbol_to_derive, &first_token.t_type);

        let first_token_productions_iter = first_token_productions.into_iter();

        let parse_symbol_result = self.inner_parse_from_tokens(
            symbol_to_derive,
            &tokens_vector,
            token_position,
            first_token_productions_iter,
            true,
        );

        Self::parse_symbol_result_to_parse_result(parse_symbol_result)
    }

    fn from(
        grammar: &'a ContextFreeGrammar<TSyntax>,
        first_follow_symbols: &FirstFollowSymbols<TSyntax>,
        syntax_error_solver: Option<TSolver>,
    ) -> Self {
        Self {
            grammar,
            marker: PhantomData,
            syntax_error_solver,
            transitions: RecursiveDescentParserTransitions::from(grammar, first_follow_symbols),
        }
    }

    fn build_token_failed_symbol(
        production_symbol: &TSyntax,
    ) -> ParseSymbolResult<'a, TLex, TSyntax> {
        let failed_symbol = FailedSymbol::new(vec![], production_symbol.clone());
        ParseSymbolResult::Err(failed_symbol)
    }

    fn inner_get_token_productions(
        &self,
        symbol_to_derive: &TSyntax,
        first_token: &TSyntax,
    ) -> Vec<&Vec<TSyntax>> {
        self.transitions
            .get_productions(symbol_to_derive, first_token)
            .map(|productions| -> Vec<&Vec<TSyntax>> {
                productions
                    .iter()
                    .map(|production| &production.output)
                    .collect()
            })
            .unwrap_or(vec![])
    }

    fn inner_parse_from_tokens(
        &self,
        symbol_to_derive: &TSyntax,
        tokens: &Vec<Token<TLex, TSyntax>>,
        tokens_position: usize,
        production_outputs: std::vec::IntoIter<&'a Vec<TSyntax>>,
        is_single_path: bool,
    ) -> ParseSymbolResult<'a, TLex, TSyntax> {
        let child_is_single_path = Self::is_child_single_path(is_single_path, &production_outputs);
        let parse_productions_result = self.inner_parse_from_tokens_try_parse_productions(
            symbol_to_derive,
            tokens,
            tokens_position,
            production_outputs,
            is_single_path,
        );

        match parse_productions_result {
            Ok(parse_symbol_result) => parse_symbol_result,
            Err(failed_productions) => {
                let fix_option = self.inner_parse_from_tokens_try_fix_productions(
                    tokens,
                    tokens_position,
                    is_single_path,
                    child_is_single_path,
                    &failed_productions,
                );

                Self::inner_parse_from_tokens_fix_option_to_parse_symbol_result(
                    failed_productions,
                    fix_option,
                    symbol_to_derive,
                )
            }
        }
    }

    fn inner_parse_from_tokens_fix_option_to_parse_symbol_result(
        failed_productions: Vec<FailedProduction<TLex, TSyntax>>,
        fix_option: Option<FixResult<TLex, TSyntax>>,
        symbol_to_derive: &TSyntax,
    ) -> ParseSymbolResult<'a, TLex, TSyntax> {
        match fix_option {
            Some(fix_result) => {
                let fixed_symbol: FixedSymbol<TLex, TSyntax> =
                    FixedSymbol::new(fix_result.production, symbol_to_derive.clone());
                let fixed_state: FixedState<TLex, TSyntax> =
                    FixedState::new(fix_result.final_token_position, fixed_symbol);

                ParseSymbolResult::Fix(fixed_state)
            }
            None => ParseSymbolResult::Err(FailedSymbol::new(
                failed_productions,
                symbol_to_derive.clone(),
            )),
        }
    }

    fn inner_parse_from_tokens_try_fix_productions(
        &self,
        tokens: &Vec<Token<TLex, TSyntax>>,
        tokens_position: usize,
        is_single_path: bool,
        child_is_single_path: bool,
        failed_productions: &Vec<FailedProduction<TLex, TSyntax>>,
    ) -> Option<FixResult<TLex, TSyntax>> {
        if self.syntax_error_solver.is_some() && is_single_path {
            let syntax_error_solver = self.syntax_error_solver.as_ref().unwrap();

            if child_is_single_path {
                let failed_production = failed_productions.get(0).unwrap();

                syntax_error_solver.fix_failed_production(
                    &tokens,
                    tokens_position,
                    failed_production,
                )
            } else {
                syntax_error_solver.fix_failed_productions(
                    &tokens,
                    tokens_position,
                    failed_productions,
                )
            }
        } else {
            None
        }
    }

    fn inner_parse_from_tokens_try_parse_productions(
        &self,
        symbol_to_derive: &TSyntax,
        tokens: &Vec<Token<TLex, TSyntax>>,
        tokens_position: usize,
        mut production_outputs: std::vec::IntoIter<&'a Vec<TSyntax>>,
        is_single_path: bool,
    ) -> Result<ParseSymbolResult<'a, TLex, TSyntax>, Vec<FailedProduction<TLex, TSyntax>>> {
        let mut failed_productions: Vec<FailedProduction<TLex, TSyntax>> = vec![];
        let child_is_single_path = Self::is_child_single_path(is_single_path, &production_outputs);

        for production_output in &mut production_outputs {
            let mut current_token_position = tokens_position;
            let parse_production_result = self.inner_parse_from_tokens_production(
                symbol_to_derive,
                tokens,
                &mut current_token_position,
                production_output,
                child_is_single_path,
            );

            match parse_production_result {
                ParseProductionResult::Ok(node) => {
                    return Ok(ParseSymbolResult::Ok(ParsingState::new(
                        tokens_position,
                        current_token_position,
                        Some(production_outputs),
                        node,
                    )));
                }
                ParseProductionResult::Err(failed_production) => {
                    failed_productions.push(failed_production);
                }
                ParseProductionResult::Fix(fixed_production) => {
                    let fixed_symbol = FixedSymbol::new(fixed_production, symbol_to_derive.clone());
                    let fixed_state = FixedState::new(current_token_position, fixed_symbol);

                    return Ok(ParseSymbolResult::Fix(fixed_state));
                }
            }
        }

        Err(failed_productions)
    }

    fn inner_parse_from_tokens_production(
        &self,
        symbol_to_derive: &TSyntax,
        tokens: &Vec<Token<TLex, TSyntax>>,
        current_token_position: &mut usize,
        production_output: &'a Vec<TSyntax>,
        is_single_path: bool,
    ) -> ParseProductionResult<TLex, TSyntax> {
        let mut states: Vec<State<'_, TLex, TSyntax, std::vec::IntoIter<&Vec<TSyntax>>>> =
            Vec::new();

        while states.len() < production_output.len() {
            let production_symbol = production_output.get(states.len()).unwrap();
            let state_option = self.inner_parse_from_tokens_production_symbol(
                production_symbol,
                tokens,
                *current_token_position,
                is_single_path,
            );

            match state_option {
                ParseSymbolResult::Ok(parsing_state) => {
                    *current_token_position = parsing_state.final_token_position;
                    states.push(State::Parsing(parsing_state));
                }
                ParseSymbolResult::Err(failed_symbol) => {
                    match self.inner_parse_pop_states(&mut states, tokens, is_single_path) {
                        Some(production_parsing_states) => {
                            return Self::inner_parse_from_tokens_production_build_failed_symbol(
                                failed_symbol,
                                production_output,
                                production_parsing_states,
                            );
                        }
                        None => {
                            *current_token_position = match states.get(states.len() - 1).unwrap() {
                                State::Parsing(parsing_state) => parsing_state.final_token_position,
                                State::Fixed(fixed_state) => fixed_state.final_token_position,
                            }
                        }
                    }
                }
                ParseSymbolResult::Fix(fixed_state) => {
                    *current_token_position = fixed_state.final_token_position;
                    states.push(State::Fixed(fixed_state));
                }
            }
        }

        Self::inner_parse_from_tokens_production_build_node(symbol_to_derive, states)
    }

    fn inner_parse_from_tokens_production_build_failed_symbol<'b>(
        failed_symbol: FailedSymbol<TLex, TSyntax>,
        production_output: &Vec<TSyntax>,
        production_states: Vec<State<'b, TLex, TSyntax, std::vec::IntoIter<&'b Vec<TSyntax>>>>,
    ) -> ParseProductionResult<TLex, TSyntax> {
        let mut pending_symbols: Vec<TSyntax> = vec![];

        let child_nodes: Vec<ProductionParsedSymbol<TLex, TSyntax>> = production_states
            .into_iter()
            .map(|state| match state {
                State::Parsing(parsing_state) => ProductionParsedSymbol::Ok(parsing_state.node),
                State::Fixed(fixed_state) => ProductionParsedSymbol::Fix(fixed_state.fixed_symbol),
            })
            .collect();

        for i in child_nodes.len() + 1..production_output.len() {
            pending_symbols.push(production_output.get(i).unwrap().clone());
        }

        let failing_state: FailedProduction<TLex, TSyntax> =
            FailedProduction::new(failed_symbol, child_nodes, pending_symbols);

        ParseProductionResult::Err(failing_state)
    }

    fn inner_parse_from_tokens_production_build_node<'b>(
        symbol_to_derive: &TSyntax,
        production_states: Vec<State<'b, TLex, TSyntax, std::vec::IntoIter<&'b Vec<TSyntax>>>>,
    ) -> ParseProductionResult<TLex, TSyntax> {
        let has_fixed_states: bool = production_states
            .iter()
            .any(|state| -> bool { state.is_fixed() });

        if has_fixed_states {
            let fixed_production_parts: Vec<FixedProductionPart<TLex, TSyntax>> = production_states
                .into_iter()
                .map(|production_state| match production_state {
                    State::Fixed(fixed_state) => {
                        FixedProductionPart::Fixed(fixed_state.fixed_symbol)
                    }
                    State::Parsing(parsing_state) => FixedProductionPart::Ok(parsing_state.node),
                })
                .collect();

            let fixed_production: FixedProduction<TLex, TSyntax> =
                FixedProduction::new(fixed_production_parts);

            ParseProductionResult::Fix(fixed_production)
        } else {
            let child_nodes: Vec<AbstractSyntaxNode<Token<TLex, TSyntax>>> = production_states
                .into_iter()
                .map(|state| match state {
                    State::Parsing(parsing_state) => parsing_state.node,
                    _ => panic!("Unexpected state!"),
                })
                .collect();

            let node: AbstractSyntaxNode<Token<TLex, TSyntax>> =
                AbstractSyntaxNode::new(child_nodes, Token::new(None, symbol_to_derive.clone()));

            ParseProductionResult::Ok(node)
        }
    }

    fn inner_parse_from_tokens_production_non_terminal(
        &self,
        production_symbol: &TSyntax,
        tokens: &Vec<Token<TLex, TSyntax>>,
        token_position: usize,
        is_single_path: bool,
    ) -> ParseSymbolResult<TLex, TSyntax> {
        let current_token_symbol = tokens.get(token_position).unwrap();

        let token_productions =
            self.inner_get_token_productions(production_symbol, &current_token_symbol.t_type);

        let token_productions_iter = token_productions.into_iter();

        self.inner_parse_from_tokens(
            production_symbol,
            tokens,
            token_position,
            token_productions_iter,
            is_single_path,
        )
    }

    fn inner_parse_terminal_symbol(
        &self,
        production_symbol: &TSyntax,
        tokens: &Vec<Token<TLex, TSyntax>>,
        token_position: usize,
    ) -> ParseSymbolResult<'a, TLex, TSyntax> {
        if self.grammar.get_epsilon_symbol().eq(production_symbol) {
            let state: ParsingState<'a, TLex, TSyntax, std::vec::IntoIter<&'a Vec<TSyntax>>> =
                ParsingState::new(
                    token_position,
                    token_position,
                    None,
                    AbstractSyntaxNode::new(vec![], Token::new(None, production_symbol.clone())),
                );

            ParseSymbolResult::Ok(state)
        } else {
            Self::inner_parse_non_epsilon_terminal_symbol(production_symbol, tokens, token_position)
        }
    }

    fn inner_parse_non_epsilon_terminal_symbol(
        production_symbol: &TSyntax,
        tokens: &Vec<Token<TLex, TSyntax>>,
        token_position: usize,
    ) -> ParseSymbolResult<'a, TLex, TSyntax> {
        let current_token_symbol_option = tokens.get(token_position);

        match current_token_symbol_option {
            Some(current_token_symbol) => {
                if production_symbol.eq(&current_token_symbol.t_type) {
                    let state: ParsingState<
                        'a,
                        TLex,
                        TSyntax,
                        std::vec::IntoIter<&'a Vec<TSyntax>>,
                    > = ParsingState::new(
                        token_position,
                        token_position + 1,
                        None,
                        AbstractSyntaxNode::new(vec![], current_token_symbol.clone()),
                    );

                    ParseSymbolResult::Ok(state)
                } else {
                    Self::build_token_failed_symbol(production_symbol)
                }
            }
            None => Self::build_token_failed_symbol(production_symbol),
        }
    }

    fn inner_parse_from_tokens_production_symbol(
        &self,
        production_symbol: &TSyntax,
        tokens: &Vec<Token<TLex, TSyntax>>,
        token_position: usize,
        is_single_path: bool,
    ) -> ParseSymbolResult<TLex, TSyntax> {
        if self.grammar.is_non_terminal(production_symbol) {
            self.inner_parse_from_tokens_production_non_terminal(
                production_symbol,
                tokens,
                token_position,
                is_single_path,
            )
        } else {
            self.inner_parse_terminal_symbol(production_symbol, tokens, token_position)
        }
    }

    fn inner_parse_pop_states(
        &self,
        states: &mut Vec<State<'a, TLex, TSyntax, std::vec::IntoIter<&'a Vec<TSyntax>>>>,
        tokens: &Vec<Token<TLex, TSyntax>>,
        is_single_path: bool,
    ) -> Option<Vec<State<'a, TLex, TSyntax, std::vec::IntoIter<&'a Vec<TSyntax>>>>> {
        let mut reversed_failed_symbols: Vec<
            State<'a, TLex, TSyntax, std::vec::IntoIter<&'a Vec<TSyntax>>>,
        > = vec![];
        let mut states_pop_success: bool = false;

        while states.len() > 0 && !states_pop_success {
            let last_state = states.pop().unwrap();

            match last_state {
                State::Fixed(fixed_state) => {
                    reversed_failed_symbols.push(State::Fixed(fixed_state));
                }
                State::Parsing(parsing_state) => {
                    if parsing_state.prod_iter_option.is_some() {
                        let productions_iterator = parsing_state.prod_iter_option.unwrap();

                        let state_option = self.inner_parse_from_tokens(
                            &parsing_state.node.token.t_type,
                            tokens,
                            parsing_state.initial_token_position,
                            productions_iterator,
                            is_single_path,
                        );

                        match state_option {
                            ParseSymbolResult::Ok(state) => {
                                states.push(State::Parsing(state));
                                states_pop_success = true;
                            }
                            _ => {}
                        }
                    } else {
                        reversed_failed_symbols.push(State::Parsing(parsing_state));
                    }
                }
            }
        }

        if states_pop_success {
            None
        } else {
            reversed_failed_symbols.reverse();
            Some(reversed_failed_symbols)
        }
    }

    fn is_child_single_path(
        is_single_path: bool,
        production_outputs: &std::vec::IntoIter<&'a Vec<TSyntax>>,
    ) -> bool {
        is_single_path && production_outputs.len() == 1
    }
}

impl<'a, TLex, TSyntax: Clone, TSolver: SyntaxErrorSolver<TLex, TSyntax>>
    RecursiveDescentParser<'a, TLex, TSyntax, TSolver>
{
    fn iterator_to_vec<TElem, TIter: Iterator<Item = TElem>>(iterator: TIter) -> Vec<TElem> {
        let mut vector: Vec<TElem> = vec![];

        iterator.for_each(|symbol| vector.push(symbol));

        vector
    }
}

impl<'a, TLex, TSyntax, TSolver: SyntaxErrorSolver<TLex, TSyntax>>
    RecursiveDescentParser<'a, TLex, TSyntax, TSolver>
{
    fn parse_symbol_result_to_parse_result(
        parse_symbol_result: ParseSymbolResult<TLex, TSyntax>,
    ) -> ParseResult<TLex, TSyntax> {
        match parse_symbol_result {
            ParseSymbolResult::Ok(parsing_state) => {
                ParseResult::Ok(AbstractSyntaxTree::new(parsing_state.node))
            }
            ParseSymbolResult::Err(failed_symbol) => ParseResult::Err(failed_symbol),
            ParseSymbolResult::Fix(fixing_state) => ParseResult::Fix(fixing_state.fixed_symbol),
        }
    }
}
