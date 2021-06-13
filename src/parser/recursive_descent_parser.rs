use std::hash::Hash;

use crate::ast::abstract_syntax_node::AbstractSyntaxNode;
use crate::ast::abstract_syntax_tree::AbstractSyntaxTree;
use crate::grammar::context_free_grammar::ContextFreeGrammar;
use crate::grammar::first_follow_symbols::FirstFollowSymbols;
use crate::parser::failed_production::FailedProduction;
use crate::parser::failed_state::FailedState;
use crate::parser::parse_result::ParseResult;
use crate::parser::recursive_descent_parser_transitions::RecursiveDescentParserTransitions;
use crate::token::token::Token;

type ParseProductionResult<TLex, TSyntax> = Result<
    AbstractSyntaxNode<Token<TLex, TSyntax>>,
    FailedProduction<TLex, TSyntax>,
>;

type ParseSymbolResult<'a, TLex, TSyntax> = Result<
    ParsingState<'a, TLex, TSyntax, std::vec::IntoIter<&'a Vec<TSyntax>>>,
    FailedState<TLex, TSyntax>,
>;

struct ParsingState<'a, TLex, TSyntax: 'a, TIter: Iterator<Item=&'a Vec<TSyntax>>> {
    initial_token_position: usize,
    final_token_position: usize,
    prod_iter_option: Option<TIter>,
    node: AbstractSyntaxNode<Token<TLex, TSyntax>>
}

impl<'a, TLex, TSyntax, TIter: Iterator<Item=&'a Vec<TSyntax>>> ParsingState<'a, TLex, TSyntax, TIter> {
    pub fn new(
        initial_token_position: usize,
        final_token_position: usize,
        prod_iter_option: Option<TIter>,
        node: AbstractSyntaxNode<Token<TLex, TSyntax>>,
    ) -> Self {
        ParsingState { initial_token_position, final_token_position, prod_iter_option, node }
    }
}

pub struct RecursiveDescentParser<'a, TSyntax> {
    grammar: &'a ContextFreeGrammar<TSyntax>,
    transitions: RecursiveDescentParserTransitions<TSyntax>,
}

impl<'a, TSyntax: Clone + Eq + Hash> RecursiveDescentParser<'a, TSyntax> {
    pub fn from_grammar(grammar: &'a ContextFreeGrammar<TSyntax>) -> Self {
        let first_follow_symbols = FirstFollowSymbols::from(grammar);

        Self::from_grammar_and_first_follow_symbols(grammar, &first_follow_symbols)
    }

    pub fn from_grammar_and_first_follow_symbols(
        grammar: &'a ContextFreeGrammar<TSyntax>,
        first_follow_symbols: &FirstFollowSymbols<TSyntax>,
    ) -> Self {
        Self {
            grammar,
            transitions: RecursiveDescentParserTransitions::from(grammar, first_follow_symbols),
        }
    }

    pub fn parse_from_tokens<TLex: Clone + 'a, TIter: Iterator<Item=Token<TLex, TSyntax>>>(
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

        let first_token_productions
            = self.inner_get_token_productions(symbol_to_derive, &first_token.t_type);

        let first_token_productions_iter = first_token_productions.into_iter();

        let state_result = self.inner_parse_from_tokens(
            symbol_to_derive,
            &tokens_vector,
            token_position,
            first_token_productions_iter,
        );

        state_result.map(|state| AbstractSyntaxTree::new(state.node))
    }

    fn build_token_failed_state<TLex: Clone>(
        production_symbol: &TSyntax,
    ) -> ParseSymbolResult<'a, TLex, TSyntax> {
        let failed_state = FailedState::new(vec![], production_symbol.clone());
        Err(failed_state)
    }

    fn inner_get_token_productions(
        &self,
        symbol_to_derive: &TSyntax,
        first_token: &TSyntax,
    ) -> Vec<&Vec<TSyntax>> {
        self.transitions
            .get_productions(symbol_to_derive, first_token)
            .map(
                |productions| -> Vec<&Vec<TSyntax>> {
                    productions
                        .iter()
                        .map(|production| &production.output).collect()
                }
            )
            .unwrap_or(vec![])
    }

    fn inner_parse_from_tokens<TLex: Clone>(
        &self,
        symbol_to_derive: &TSyntax,
        tokens: &Vec<Token<TLex, TSyntax>>,
        tokens_position: usize,
        mut production_outputs: std::vec::IntoIter<&'a Vec<TSyntax>>,
    ) -> ParseSymbolResult<'a, TLex, TSyntax> {
        let mut failed_productions: Vec<FailedProduction<TLex, TSyntax>> = vec![];

        for production_output in &mut production_outputs {
            let mut current_token_position = tokens_position;
            let node_option = self.inner_parse_from_tokens_production(
                symbol_to_derive,
                tokens,
                &mut current_token_position,
                production_output,
            );

            match node_option {
                Ok(node) => {
                    return Ok(ParsingState::new(
                        tokens_position,
                        current_token_position,
                        Some(production_outputs),
                        node,
                    ));
                },
                Err(failed_production) => {
                    failed_productions.push(failed_production);
                }
            }
        }

        Err(FailedState::new(
            failed_productions,
            symbol_to_derive.clone(),
        ))
    }

    fn inner_parse_from_tokens_production<TLex: Clone>(
        &self,
        symbol_to_derive: &TSyntax,
        tokens: &Vec<Token<TLex, TSyntax>>,
        current_token_position: &mut usize,
        production_output: &'a Vec<TSyntax>,
    ) -> ParseProductionResult<TLex, TSyntax> {
        let mut states: Vec<ParsingState<'_, TLex, TSyntax, std::vec::IntoIter<&Vec<TSyntax>>>> = Vec::new();

        while states.len() < production_output.len() {
            let production_symbol = production_output.get(states.len()).unwrap();
            let state_option = self.inner_parse_from_tokens_production_symbol(
                production_symbol,
                tokens,
                *current_token_position,
            );

            match state_option {
                Ok(state) => {
                    *current_token_position = state.final_token_position;
                    states.push(state);
                }
                Err(failed_state) => {
                    match self.inner_parse_pop_states(&mut states, tokens) {
                        Some(production_parsing_states) => {
                            return Self::inner_parse_from_tokens_production_build_failed_state(
                                failed_state,
                                production_output,
                                production_parsing_states,
                            );
                        },
                        None => {
                            *current_token_position = states.get(
                                states.len() - 1
                            ).unwrap().final_token_position;
                        }
                    }
                }
            }
        }

        Self::inner_parse_from_tokens_production_build_node(
            symbol_to_derive,
            states,
        )
    }

    fn inner_parse_from_tokens_production_build_failed_state<'b, TLex>(
        failed_state: FailedState<TLex, TSyntax>,
        production_output: &Vec<TSyntax>,
        production_parsing_states: Vec<
            ParsingState<'b, TLex, TSyntax, std::vec::IntoIter<&'b Vec<TSyntax>>>
        >,
    ) -> ParseProductionResult<TLex, TSyntax> {
        let mut pending_symbols: Vec<FailedState<TLex, TSyntax>> = vec![failed_state];

        let child_nodes: Vec<AbstractSyntaxNode<Token<TLex, TSyntax>>> =
            production_parsing_states.into_iter().map(|state| state.node).collect();

        for i in child_nodes.len() + 1 .. production_output.len() {
            pending_symbols.push(
                FailedState::new(
                    vec![],
                    production_output.get(i).unwrap().clone(),
                ),
            );
        }
        
        let failing_state: FailedProduction<TLex, TSyntax> = FailedProduction::new(
            child_nodes,
            pending_symbols,
        );

        Err(failing_state)
    }

    fn inner_parse_from_tokens_production_build_node<'b, TLex>(
        symbol_to_derive: &TSyntax,
        states: Vec<ParsingState<'b, TLex, TSyntax, std::vec::IntoIter<&'b Vec<TSyntax>>>>
    ) -> ParseProductionResult<TLex, TSyntax> {
        let child_nodes: Vec<AbstractSyntaxNode<Token<TLex, TSyntax>>> =
            states.into_iter().map(|state| state.node).collect();

        let node: AbstractSyntaxNode<Token<TLex, TSyntax>>
            = AbstractSyntaxNode::new(
            child_nodes,
            Token::new(None, symbol_to_derive.clone()),
        );

        Ok(node)
    }

    fn inner_parse_from_tokens_production_non_terminal<TLex: Clone>(
        &self,
        production_symbol: &TSyntax,
        tokens: &Vec<Token<TLex, TSyntax>>,
        token_position: usize,
    ) -> ParseSymbolResult<TLex, TSyntax> {
        let current_token_symbol = tokens.get(token_position).unwrap();

        let token_productions = self.inner_get_token_productions(
            production_symbol,
            &current_token_symbol.t_type,
        );

        let token_productions_iter = token_productions.into_iter();

        self.inner_parse_from_tokens(
            production_symbol,
            tokens,
            token_position,
            token_productions_iter,
        )
    }

    fn inner_parse_terminal_symbol<TLex: Clone>(
        &self,
        production_symbol: &TSyntax,
        tokens: &Vec<Token<TLex, TSyntax>>,
        token_position: usize,
    ) -> ParseSymbolResult<'a, TLex, TSyntax> {
        if self.grammar.get_epsilon_symbol().eq(production_symbol) {
            let state: ParsingState<'a, TLex, TSyntax, std::vec::IntoIter<&'a Vec<TSyntax>>>
                = ParsingState::new(
                token_position,
                token_position,
                None,
                AbstractSyntaxNode::new(vec![], Token::new(None, production_symbol.clone())),
            );

            Ok(state)
        } else {
            Self::inner_parse_non_epsilon_terminal_symbol(
                production_symbol,
                tokens,
                token_position,
            )
        }
    }

    fn inner_parse_non_epsilon_terminal_symbol<TLex: Clone>(
        production_symbol: &TSyntax,
        tokens: &Vec<Token<TLex, TSyntax>>,
        token_position: usize,
    ) -> ParseSymbolResult<'a, TLex, TSyntax> {
        let current_token_symbol_option = tokens.get(token_position);

        match current_token_symbol_option {
            Some(current_token_symbol) => {
                if production_symbol.eq(&current_token_symbol.t_type) {
                    let state: ParsingState<'a, TLex, TSyntax, std::vec::IntoIter<&'a Vec<TSyntax>>>
                        = ParsingState::new(
                        token_position,
                        token_position + 1,
                        None,
                        AbstractSyntaxNode::new(vec![], current_token_symbol.clone())
                    );
    
                    Ok(state)
                } else {
                    Self::build_token_failed_state(production_symbol)
                }
            },
            None => {
                Self::build_token_failed_state(production_symbol)
            }
        }
    }

    fn inner_parse_from_tokens_production_symbol<TLex: Clone>(
        &self,
        production_symbol: &TSyntax,
        tokens: &Vec<Token<TLex, TSyntax>>,
        token_position: usize,
    ) -> ParseSymbolResult<TLex, TSyntax> {
        if self.grammar.is_non_terminal(production_symbol) {
            self.inner_parse_from_tokens_production_non_terminal(
                production_symbol,
                tokens,
                token_position,
            )
        } else {
            self.inner_parse_terminal_symbol(
                production_symbol,
                tokens,
                token_position,
            )
        }
    }

    fn inner_parse_pop_states<TLex: Clone>(
        &self,
        states: &mut Vec<ParsingState<'a, TLex, TSyntax, std::vec::IntoIter<&'a Vec<TSyntax>>>>,
        tokens: &Vec<Token<TLex, TSyntax>>,
    ) -> Option<Vec<ParsingState<'a, TLex, TSyntax, std::vec::IntoIter<&'a Vec<TSyntax>>>>> {
        let mut reversed_failed_states: Vec<ParsingState<'a, TLex, TSyntax, std::vec::IntoIter<&'a Vec<TSyntax>>>> = vec![];
        let mut states_pop_success: bool = false;

        while states.len() > 0 && !states_pop_success {
            let last_state = states.pop().unwrap();

            if last_state.prod_iter_option.is_some() {
                let productions_iterator = last_state.prod_iter_option.unwrap();

                let state_option = self.inner_parse_from_tokens(
                    &last_state.node.token.t_type,
                    tokens,
                    last_state.initial_token_position,
                    productions_iterator,
                );

                match state_option {
                    Ok(state) => {
                        states.push(state);
                        states_pop_success = true;
                    }
                    _ => { }
                }
            } else {
                reversed_failed_states.push(last_state);
            }
        }

        if states_pop_success {
            None
        } else {
            reversed_failed_states.reverse();
            Some(reversed_failed_states)
        }
    }
}

impl<'a, T: Clone> RecursiveDescentParser<'a, T> {
    fn iterator_to_vec<TElem, TIter: Iterator<Item=TElem>>(iterator: TIter) -> Vec<TElem> {
        let mut vector: Vec<TElem> = vec![];

        iterator.for_each(|symbol| { vector.push(symbol) });

        vector
    }
}
