use std::hash::Hash;

use crate::grammar::context_free_grammar::ContextFreeGrammar;
use crate::grammar::first_follow_symbols::FirstFollowSymbols;
use crate::parser::recursive_descent_parser_transitions::RecursiveDescentParserTransitions;
use crate::ast::abstract_syntax_node::AbstractSyntaxNode;
use crate::ast::abstract_syntax_tree::AbstractSyntaxTree;

struct RecursiveDescentParsingState<'a, T: 'a, TIter: Iterator<Item=&'a Vec<T>>> {
    initial_token_position: usize,
    final_token_position: usize,
    prod_iter_option: Option<TIter>,
    node: AbstractSyntaxNode<T>
}

impl<'a, T: 'a, TIter: Iterator<Item=&'a Vec<T>>> RecursiveDescentParsingState<'a, T, TIter> {
    pub fn new(
        initial_token_position: usize,
        final_token_position: usize,
        prod_iter_option: Option<TIter>,
        node: AbstractSyntaxNode<T>,
    ) -> Self {
        Self { initial_token_position, final_token_position, prod_iter_option, node }
    }
}

pub struct RecursiveDescentParser<'a, T> {
    grammar: &'a ContextFreeGrammar<T>,
    transitions: RecursiveDescentParserTransitions<T>,
}

impl<'a, T: Clone + Eq + Hash> RecursiveDescentParser<'a, T> {
    pub fn from_grammar(grammar: &'a ContextFreeGrammar<T>) -> Self {
        let first_follow_symbols = FirstFollowSymbols::from(grammar);

        Self::from_grammar_and_first_follow_symbols(grammar, &first_follow_symbols)
    }

    pub fn from_grammar_and_first_follow_symbols(
        grammar: &'a ContextFreeGrammar<T>,
        first_follow_symbols: &FirstFollowSymbols<T>,
    ) -> Self {
        Self {
            grammar,
            transitions: RecursiveDescentParserTransitions::from(grammar, first_follow_symbols),
        }
    }

    pub fn parse_from_tokens<TIter: Iterator<Item=&'a T>>(&self, tokens_iterator: TIter) -> Option<AbstractSyntaxTree<T>> {
        let tokens_vector = Self::iterator_to_vec(tokens_iterator);

        if tokens_vector.len() == 0 {
            panic!("Expecting at least one token!")
        }

        let symbol_to_derive = self.grammar.get_initial_symbol();
        let token_position: usize = 0;
        let first_token = *tokens_vector.get(token_position).unwrap();

        let first_token_productions
            = self.inner_get_token_productions(symbol_to_derive, first_token);

        let first_token_productions_iter = first_token_productions.into_iter();

        let state = self.inner_parse_from_tokens(
            symbol_to_derive,
            &tokens_vector,
            token_position,
            first_token_productions_iter,
        );

        state.map(
            |state|
                AbstractSyntaxTree::new(state.node)
        )
    }

    fn inner_get_token_productions(&self, symbol_to_derive: &T, first_token: &T) -> Vec<&Vec<T>> {
        self.transitions
            .get_productions(symbol_to_derive, first_token)
            .map(
                |productions| -> Vec<&Vec<T>> {
                    productions
                        .iter()
                        .map(|production| &production.output).collect()
                }
            )
            .unwrap_or(vec![])
    }

    fn inner_parse_from_tokens(
        &self,
        symbol_to_derive: &T,
        tokens: &Vec<&'a T>,
        tokens_position: usize,
        mut production_outputs: std::vec::IntoIter<&'a Vec<T>>,
    ) -> Option<RecursiveDescentParsingState<'a, T, std::vec::IntoIter<&'a Vec<T>>>> {
        let mut current_token_position = tokens_position;

        for production_output in &mut production_outputs {
            let node_option = self.inner_parse_from_tokens_production(
                symbol_to_derive,
                tokens,
                &mut current_token_position,
                production_output,
            );

            match node_option {
                Some(node) => {
                    let state = RecursiveDescentParsingState::new(
                        tokens_position,
                        current_token_position,
                        Some(production_outputs),
                        node,
                    );
    
                    return Some(state);
                },
                _ => {}
            }
        }

        None
    }

    fn inner_parse_from_tokens_production(
        &self,
        symbol_to_derive: &T,
        tokens: &Vec<&'a T>,
        current_token_position: &mut usize,
        production_output: &'a Vec<T>,
    ) -> Option<AbstractSyntaxNode<T>> {
        let mut states: Vec<RecursiveDescentParsingState<'_, T, std::vec::IntoIter<&Vec<T>>>> = Vec::new();

        while states.len() < production_output.len() {
            let production_symbol = production_output.get(states.len()).unwrap();
            let state_option = self.inner_parse_from_tokens_production_symbol(
                production_symbol,
                tokens,
                *current_token_position,
            );

            match state_option {
                Some(state) => {
                    *current_token_position = state.final_token_position;
                    states.push(state);
                }
                _ => {
                    if self.inner_parse_pop_states(&mut states, tokens) {
                        *current_token_position = states.get(states.len() - 1).unwrap().final_token_position
                    } else {
                        break;
                    }
                }
            }
        }

        let child_nodes: Vec<AbstractSyntaxNode<T>> =
            states.into_iter().map(|state| state.node).collect();

        Self::inner_parse_from_tokens_production_build_state(
            symbol_to_derive,
            production_output,
            child_nodes,
        )
    }

    fn inner_parse_from_tokens_production_build_state(
        symbol_to_derive: &T,
        production_output: &'a Vec<T>,
        child_nodes: Vec<AbstractSyntaxNode<T>>,
    ) -> Option<AbstractSyntaxNode<T>> {
        if child_nodes.len() == production_output.len() {
            let node: AbstractSyntaxNode<T>
                = AbstractSyntaxNode::new(
                child_nodes,
                symbol_to_derive.clone(),
            );

            Some(node)
        } else {
            None
        }
    }

    fn inner_parse_from_tokens_production_non_terminal(
        &self,
        production_symbol: &T,
        tokens: &Vec<&'a T>,
        token_position: usize,
    ) -> Option<RecursiveDescentParsingState<'_, T, std::vec::IntoIter<&'_ Vec<T>>>> {
        let current_token_symbol = *tokens.get(token_position).unwrap();

        let token_productions = self.inner_get_token_productions(
            production_symbol,
            current_token_symbol,
        );

        let token_productions_iter = token_productions.into_iter();

        self.inner_parse_from_tokens(
            production_symbol,
            tokens,
            token_position,
            token_productions_iter,
        )
    }

    fn inner_parse_from_tokens_production_terminal(
        &self,
        production_symbol: &T,
        tokens: &Vec<&'a T>,
        token_position: usize,
    ) -> Option<RecursiveDescentParsingState<'a, T, std::vec::IntoIter<&'a Vec<T>>>> {
        if self.grammar.get_epsilon_symbol().eq(production_symbol) {
            let state: RecursiveDescentParsingState<'a, T, std::vec::IntoIter<&'a Vec<T>>>
                = RecursiveDescentParsingState::new(
                token_position,
                token_position,
                None,
                AbstractSyntaxNode::new(vec![], production_symbol.clone()),
            );

            Some(state)
        } else {
            let current_token_symbol = *tokens.get(token_position).unwrap();

            if production_symbol.eq(current_token_symbol) {
                let state: RecursiveDescentParsingState<'a, T, std::vec::IntoIter<&'a Vec<T>>>
                    = RecursiveDescentParsingState::new(
                    token_position,
                    token_position + 1,
                    None,
                    AbstractSyntaxNode::new(vec![], production_symbol.clone()),
                );

                Some(state)
            } else {
                None
            }
        }
    }

    fn inner_parse_from_tokens_production_symbol(
        &self,
        production_symbol: &T,
        tokens: &Vec<&'a T>,
        token_position: usize,
    ) -> Option<RecursiveDescentParsingState<'_, T, std::vec::IntoIter<&'_ Vec<T>>>> {
        if self.grammar.is_non_terminal(production_symbol) {
            self.inner_parse_from_tokens_production_non_terminal(
                production_symbol,
                tokens,
                token_position,
            )
        } else {
            self.inner_parse_from_tokens_production_terminal(
                production_symbol,
                tokens,
                token_position,
            )
        }
    }

    fn inner_parse_pop_states(
        &self,
        states: &mut Vec<RecursiveDescentParsingState<'a, T, std::vec::IntoIter<&'a Vec<T>>>>,
        tokens: &Vec<&'a T>,
    ) -> bool {
        let mut states_pop_success: bool = false;

        while states.len() > 0 && !states_pop_success {
            let last_state = states.pop().unwrap();

            match last_state.prod_iter_option {
                Some(productions_iterator) => {
                    let state_option = self.inner_parse_from_tokens(
                        &last_state.node.token,
                        tokens,
                        last_state.initial_token_position,
                        productions_iterator,
                    );

                    match state_option {
                        Some(state) => {
                            states.push(state);
                            states_pop_success = true;
                        }
                        _ => {  }
                    }
                }
                _ => { }
            }
        }

        states_pop_success
    }
}

impl<'a, T: Clone> RecursiveDescentParser<'a, T> {
    fn iterator_to_vec<TIter: Iterator<Item=&'a T>>(iterator: TIter) -> Vec<&'a T> {
        let mut vector: Vec<&'a T> = vec![];

        iterator.for_each(|symbol| { vector.push(symbol) });

        vector
    }
}
