use std::hash::Hash;

use crate::grammar::context_free_grammar::ContextFreeGrammar;
use crate::grammar::first_follow_symbols::FirstFollowSymbols;
use crate::parser::recursive_descent_parser_transitions::RecursiveDescentParserTransitions;
use crate::ast::abstract_syntax_node::AbstractSyntaxNode;
use crate::ast::abstract_syntax_tree::AbstractSyntaxTree;
use crate::token::token::Token;

struct RecursiveDescentParsingState<'a, TLex, TSyntax: 'a, TIter: Iterator<Item=&'a Vec<TSyntax>>> {
    initial_token_position: usize,
    final_token_position: usize,
    prod_iter_option: Option<TIter>,
    node: AbstractSyntaxNode<Token<TLex, TSyntax>>
}

impl<'a, TLex, TSyntax, TIter: Iterator<Item=&'a Vec<TSyntax>>> RecursiveDescentParsingState<'a, TLex, TSyntax, TIter> {
    pub fn new(
        initial_token_position: usize,
        final_token_position: usize,
        prod_iter_option: Option<TIter>,
        node: AbstractSyntaxNode<Token<TLex, TSyntax>>,
    ) -> Self {
        Self { initial_token_position, final_token_position, prod_iter_option, node }
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
    ) -> Option<AbstractSyntaxTree<Token<TLex, TSyntax>>> {
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
    ) -> Option<RecursiveDescentParsingState<'a, TLex, TSyntax, std::vec::IntoIter<&'a Vec<TSyntax>>>> {
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

    fn inner_parse_from_tokens_production<TLex: Clone>(
        &self,
        symbol_to_derive: &TSyntax,
        tokens: &Vec<Token<TLex, TSyntax>>,
        current_token_position: &mut usize,
        production_output: &'a Vec<TSyntax>,
    ) -> Option<AbstractSyntaxNode<Token<TLex, TSyntax>>> {
        let mut states: Vec<RecursiveDescentParsingState<'_, TLex, TSyntax, std::vec::IntoIter<&Vec<TSyntax>>>> = Vec::new();

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
                        *current_token_position = states.get(
                            states.len() - 1
                        ).unwrap().final_token_position
                    } else {
                        break;
                    }
                }
            }
        }

        let child_nodes: Vec<AbstractSyntaxNode<Token<TLex, TSyntax>>> =
            states.into_iter().map(|state| state.node).collect();

        Self::inner_parse_from_tokens_production_build_state(
            symbol_to_derive,
            production_output,
            child_nodes,
        )
    }

    fn inner_parse_from_tokens_production_build_state<TLex>(
        symbol_to_derive: &TSyntax,
        production_output: &'a Vec<TSyntax>,
        child_nodes: Vec<AbstractSyntaxNode<Token<TLex, TSyntax>>>,
    ) -> Option<AbstractSyntaxNode<Token<TLex, TSyntax>>> {
        if child_nodes.len() == production_output.len() {
            let node: AbstractSyntaxNode<Token<TLex, TSyntax>>
                = AbstractSyntaxNode::new(
                child_nodes,
                Token::new(None, symbol_to_derive.clone()),
            );

            Some(node)
        } else {
            None
        }
    }

    fn inner_parse_from_tokens_production_non_terminal<TLex: Clone>(
        &self,
        production_symbol: &TSyntax,
        tokens: &Vec<Token<TLex, TSyntax>>,
        token_position: usize,
    ) -> Option<RecursiveDescentParsingState<'_, TLex, TSyntax, std::vec::IntoIter<&'_ Vec<TSyntax>>>> {
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

    fn inner_parse_from_tokens_production_terminal<TLex: Clone>(
        &self,
        production_symbol: &TSyntax,
        tokens: &Vec<Token<TLex, TSyntax>>,
        token_position: usize,
    ) -> Option<RecursiveDescentParsingState<'a, TLex, TSyntax, std::vec::IntoIter<&'a Vec<TSyntax>>>> {
        if self.grammar.get_epsilon_symbol().eq(production_symbol) {
            let state: RecursiveDescentParsingState<'a, TLex, TSyntax, std::vec::IntoIter<&'a Vec<TSyntax>>>
                = RecursiveDescentParsingState::new(
                token_position,
                token_position,
                None,
                AbstractSyntaxNode::new(vec![], Token::new(None, production_symbol.clone())),
            );

            Some(state)
        } else {
            let current_token_symbol = tokens.get(token_position).unwrap();

            if production_symbol.eq(&current_token_symbol.t_type) {
                let state: RecursiveDescentParsingState<'a, TLex, TSyntax, std::vec::IntoIter<&'a Vec<TSyntax>>>
                    = RecursiveDescentParsingState::new(
                    token_position,
                    token_position + 1,
                    None,
                    AbstractSyntaxNode::new(vec![], current_token_symbol.clone())
                );

                Some(state)
            } else {
                None
            }
        }
    }

    fn inner_parse_from_tokens_production_symbol<TLex: Clone>(
        &self,
        production_symbol: &TSyntax,
        tokens: &Vec<Token<TLex, TSyntax>>,
        token_position: usize,
    ) -> Option<RecursiveDescentParsingState<'_, TLex, TSyntax, std::vec::IntoIter<&'_ Vec<TSyntax>>>> {
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

    fn inner_parse_pop_states<TLex: Clone>(
        &self,
        states: &mut Vec<RecursiveDescentParsingState<'a, TLex, TSyntax, std::vec::IntoIter<&'a Vec<TSyntax>>>>,
        tokens: &Vec<Token<TLex, TSyntax>>,
    ) -> bool {
        let mut states_pop_success: bool = false;

        while states.len() > 0 && !states_pop_success {
            let last_state = states.pop().unwrap();

            match last_state.prod_iter_option {
                Some(productions_iterator) => {
                    let state_option = self.inner_parse_from_tokens(
                        &last_state.node.token.t_type,
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
    fn iterator_to_vec<TElem, TIter: Iterator<Item=TElem>>(iterator: TIter) -> Vec<TElem> {
        let mut vector: Vec<TElem> = vec![];

        iterator.for_each(|symbol| { vector.push(symbol) });

        vector
    }
}
