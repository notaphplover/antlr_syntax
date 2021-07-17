use std::borrow::Borrow;
use std::cell::{Ref, RefCell, RefMut};
use std::collections::hash_map::RandomState;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::iter::{FromIterator, Map};
use std::slice::Iter;

use crate::grammar::context_free_grammar::ContextFreeGrammar;
use crate::grammar::context_free_grammar_production::ContextFreeGrammarProduction;

pub type SymbolsMap<T> = HashMap<T, HashSet<T>>;

pub struct FirstFollowSymbols<T> {
    first_symbols: SymbolsMap<T>,
    follow_symbols: SymbolsMap<T>,
}

impl<T> FirstFollowSymbols<T> {
    fn converge<M, F>(model: &M, callback: F) -> ()
    where
        F: Fn(&M) -> bool,
    {
        while callback(model) {}
    }
}

impl<T: Eq + Hash> FirstFollowSymbols<T> {
    pub fn new(first_symbols: SymbolsMap<T>, follow_symbols: SymbolsMap<T>) -> Self {
        FirstFollowSymbols {
            first_symbols,
            follow_symbols,
        }
    }

    pub fn get_first_symbols<'a>(&'a self, symbol: &'a T) -> Option<&'a HashSet<T>> {
        FirstFollowSymbols::get_symbols_from_hash_map(&self.first_symbols, symbol)
    }

    pub fn get_follow_symbols<'a>(&'a self, symbol: &'a T) -> Option<&'a HashSet<T>> {
        FirstFollowSymbols::get_symbols_from_hash_map(&self.follow_symbols, symbol)
    }

    fn get_symbols_from_hash_map<'a>(
        hash_map: &'a SymbolsMap<T>,
        key: &T,
    ) -> Option<&'a HashSet<T>> {
        hash_map.get(key)
    }

    fn ref_map_to_symbols_map(symbols_ref_map: HashMap<T, RefCell<HashSet<T>>>) -> SymbolsMap<T> {
        HashMap::from_iter(
            symbols_ref_map
                .into_iter()
                .map(|(symbol, symbols_ref)| (symbol, symbols_ref.take())),
        )
    }
}

impl<T: Clone + Eq + Hash> FirstFollowSymbols<T> {
    pub fn from(grammar: &ContextFreeGrammar<T>) -> Self {
        let first_symbols = Self::inner_get_first_symbols(grammar);
        let follow_symbols = Self::inner_get_follow_symbols(grammar, &first_symbols);

        Self::new(first_symbols, follow_symbols)
    }

    fn inner_get_first_symbols(grammar: &ContextFreeGrammar<T>) -> SymbolsMap<T> {
        let non_terminal_symbols = grammar.get_non_terminal_symbols();
        let terminal_symbols = grammar.get_terminal_symbols();

        let first_symbols_map: HashMap<T, RefCell<HashSet<T>>> =
            Self::inner_get_first_symbols_initial_map(&non_terminal_symbols, &terminal_symbols);

        Self::converge(
            &first_symbols_map,
            |model: &HashMap<T, RefCell<HashSet<T>>>| -> bool {
                let mut updated_at_iter: bool = false;

                let productions = non_terminal_symbols
                    .iter()
                    .map(|symbol| (symbol, grammar.get_productions(symbol).unwrap()));

                for (symbol, tuple_productions) in productions {
                    let mut tuple_symbol_first_symbols: RefMut<HashSet<T>> =
                        first_symbols_map.get(symbol).unwrap().borrow_mut();

                    for production in tuple_productions {
                        updated_at_iter |= Self::inner_get_first_symbols_process_production(
                            grammar,
                            model,
                            &mut tuple_symbol_first_symbols,
                            &production,
                        );
                    }
                }

                updated_at_iter
            },
        );

        Self::ref_map_to_symbols_map(first_symbols_map)
    }

    fn inner_get_first_symbols_initial_map(
        non_terminal_symbols: &Vec<T>,
        terminal_symbols: &Vec<T>,
    ) -> HashMap<T, RefCell<HashSet<T>>> {
        let non_terminal_symbols_iterator =
            Self::inner_get_first_symbols_non_terminal_map(&non_terminal_symbols);
        let terminal_symbols_iterator =
            Self::inner_get_first_symbols_terminal_map(&terminal_symbols);

        HashMap::from_iter(non_terminal_symbols_iterator.chain(terminal_symbols_iterator))
    }

    fn inner_get_first_symbols_non_terminal_map(
        symbols: &Vec<T>,
    ) -> Map<Iter<'_, T>, fn(&T) -> (T, RefCell<HashSet<T, RandomState>>)> {
        symbols
            .iter()
            .map(|symbol| (symbol.clone(), RefCell::new(HashSet::new())))
    }

    fn inner_get_first_symbols_process_production(
        grammar: &ContextFreeGrammar<T>,
        first_symbols_map: &HashMap<T, RefCell<HashSet<T>>>,
        tuple_symbol_first_symbols: &mut RefMut<HashSet<T>>,
        production: &ContextFreeGrammarProduction<T>,
    ) -> bool {
        let mut first_symbols_updated: bool = false;
        let mut output_iterator_option: Option<Iter<T>> = Some(production.output.iter());

        while output_iterator_option.is_some() {
            let production_symbol_option = output_iterator_option.as_mut().unwrap().next();

            match production_symbol_option {
                Some(production_symbol) => {
                    first_symbols_updated |=
                        Self::inner_get_first_symbols_process_production_symbol(
                            grammar,
                            first_symbols_map,
                            tuple_symbol_first_symbols,
                            &mut output_iterator_option,
                            &production.input,
                            production_symbol,
                        );
                }
                None => {
                    first_symbols_updated |=
                        tuple_symbol_first_symbols.insert(grammar.get_epsilon_symbol().clone());
                    break;
                }
            }
        }

        first_symbols_updated
    }

    fn inner_get_first_symbols_process_production_symbol(
        grammar: &ContextFreeGrammar<T>,
        first_symbols_map: &HashMap<T, RefCell<HashSet<T>>>,
        tuple_symbol_first_symbols: &mut RefMut<HashSet<T>>,
        output_iterator_option: &mut Option<Iter<T>>,
        production_input: &T,
        production_symbol: &T,
    ) -> bool {
        let mut first_symbols_updated = false;

        if production_symbol.eq(production_input) {
            if !tuple_symbol_first_symbols.contains(grammar.get_epsilon_symbol()) {
                *output_iterator_option = None;
            }
        } else {
            if grammar.is_non_terminal(production_symbol) {
                first_symbols_updated |=
                    Self::inner_get_first_symbols_process_production_non_terminal_symbol(
                        grammar,
                        first_symbols_map,
                        tuple_symbol_first_symbols,
                        output_iterator_option,
                        production_symbol,
                    );
            } else {
                first_symbols_updated |=
                    tuple_symbol_first_symbols.insert(production_symbol.clone());

                *output_iterator_option = None
            }
        }

        first_symbols_updated
    }

    fn inner_get_first_symbols_process_production_non_terminal_symbol(
        grammar: &ContextFreeGrammar<T>,
        first_symbols_map: &HashMap<T, RefCell<HashSet<T>>>,
        tuple_symbol_first_symbols: &mut RefMut<HashSet<T>>,
        output_iterator_option: &mut Option<Iter<T>>,
        production_symbol: &T,
    ) -> bool {
        let mut first_symbols_updated = false;

        let symbol_first_symbols: Ref<HashSet<T>> =
            first_symbols_map.get(production_symbol).unwrap().borrow();

        if symbol_first_symbols.contains(grammar.get_epsilon_symbol()) {
            first_symbols_updated |= Self::insert_many(
                tuple_symbol_first_symbols,
                &mut symbol_first_symbols
                    .iter()
                    .filter(|symbol| grammar.get_epsilon_symbol().ne(*symbol)),
            );
        } else {
            first_symbols_updated |=
                Self::insert_many(tuple_symbol_first_symbols, &mut symbol_first_symbols.iter());

            *output_iterator_option = None
        }

        first_symbols_updated
    }

    fn inner_get_first_symbols_terminal_map(
        symbols: &Vec<T>,
    ) -> Map<Iter<'_, T>, fn(&T) -> (T, RefCell<HashSet<T, RandomState>>)> {
        symbols.iter().map(|symbol| {
            let mut hash_set = HashSet::new();
            hash_set.insert(symbol.clone());

            (symbol.clone(), RefCell::new(hash_set))
        })
    }

    fn inner_get_follow_symbols(
        grammar: &ContextFreeGrammar<T>,
        first_symbols_map: &SymbolsMap<T>,
    ) -> SymbolsMap<T> {
        let non_terminal_symbols = grammar.get_non_terminal_symbols();

        let follow_symbols_map: HashMap<T, RefCell<HashSet<T>>> = HashMap::from_iter(
            non_terminal_symbols
                .iter()
                .map(|symbol| (symbol.clone(), RefCell::new(HashSet::new()))),
        );

        Self::converge(&follow_symbols_map, |model| {
            let mut updated_at_iter: bool = false;

            let productions = non_terminal_symbols
                .iter()
                .map(|symbol| (symbol, grammar.get_productions(symbol).unwrap()));

            for (_, tuple_productions) in productions {
                for production in tuple_productions {
                    updated_at_iter |= Self::get_follow_symbols_process_production(
                        grammar,
                        first_symbols_map,
                        model,
                        production,
                    );
                }
            }

            updated_at_iter
        });

        Self::ref_map_to_symbols_map(follow_symbols_map)
    }

    fn get_follow_symbols_process_production_last_epsilon_chain(
        grammar: &ContextFreeGrammar<T>,
        first_symbols_map: &SymbolsMap<T>,
        follow_symbols_map: &HashMap<T, RefCell<HashSet<T>>>,
        production: &ContextFreeGrammarProduction<T>,
    ) -> bool {
        let mut follow_symbols_updated: bool = false;

        let mut production_output_index: usize = production.output.len() - 1;

        loop {
            let output_symbol: &T = production.output.get(production_output_index).unwrap();

            if output_symbol.ne(&production.input) && grammar.is_non_terminal(output_symbol) {
                let input_follow_symbols =
                    follow_symbols_map.get(&production.input).unwrap().borrow();
                let mut symbol_follow_symbols_ref =
                    follow_symbols_map.get(output_symbol).unwrap().borrow_mut();

                input_follow_symbols.iter().for_each(|symbol| {
                    follow_symbols_updated |= symbol_follow_symbols_ref.insert(symbol.clone());
                });
            }

            let output_symbol_first_symbols: &HashSet<T> =
                first_symbols_map.get(output_symbol).unwrap().borrow();

            let output_symbol_first_symbols_contains_epsilon: bool =
                output_symbol_first_symbols.contains(grammar.get_epsilon_symbol());

            if production_output_index == 0 || !output_symbol_first_symbols_contains_epsilon {
                break;
            } else {
                production_output_index -= 1;
            }
        }

        follow_symbols_updated
    }

    fn get_follow_symbols_process_production(
        grammar: &ContextFreeGrammar<T>,
        first_symbols_map: &SymbolsMap<T>,
        follow_symbols_map: &HashMap<T, RefCell<HashSet<T>>>,
        production: &ContextFreeGrammarProduction<T>,
    ) -> bool {
        let mut follow_symbols_updated: bool =
            Self::get_follow_symbols_process_production_last_epsilon_chain(
                grammar,
                first_symbols_map,
                follow_symbols_map,
                &production,
            );

        let last_production_output_index: usize = production.output.len() - 1;

        if last_production_output_index > 0 {
            let mut production_output_index: usize = last_production_output_index - 1;
            let mut first_indexes: Vec<usize> = vec![];

            loop {
                follow_symbols_updated |= Self::get_follow_symbols_process_production_symbol(
                    grammar,
                    first_symbols_map,
                    follow_symbols_map,
                    production,
                    production_output_index,
                    &mut first_indexes,
                );

                if production_output_index == 0 {
                    break;
                } else {
                    production_output_index -= 1;
                }
            }
        }

        follow_symbols_updated
    }

    fn get_follow_symbols_process_production_symbol(
        grammar: &ContextFreeGrammar<T>,
        first_symbols_map: &SymbolsMap<T>,
        follow_symbols_map: &HashMap<T, RefCell<HashSet<T>>>,
        production: &ContextFreeGrammarProduction<T>,
        production_output_index: usize,
        first_indexes: &mut Vec<usize>,
    ) -> bool {
        let follow_symbols_updated: bool;

        let current_symbol: &T = production.output.get(production_output_index).unwrap();

        if grammar.is_non_terminal(current_symbol) {
            follow_symbols_updated =
                Self::get_follow_symbols_process_production_non_terminal_symbol(
                    grammar,
                    first_symbols_map,
                    follow_symbols_map,
                    production,
                    production_output_index,
                    first_indexes,
                    current_symbol,
                );
        } else {
            follow_symbols_updated = false;

            first_indexes.clear();
        }

        follow_symbols_updated
    }

    fn get_follow_symbols_process_production_non_terminal_symbol(
        grammar: &ContextFreeGrammar<T>,
        first_symbols_map: &SymbolsMap<T>,
        follow_symbols_map: &HashMap<T, RefCell<HashSet<T>>>,
        production: &ContextFreeGrammarProduction<T>,
        production_output_index: usize,
        first_indexes: &mut Vec<usize>,
        current_symbol: &T,
    ) -> bool {
        let mut follow_symbols_updated: bool = false;

        let next_symbol_index: usize = production_output_index + 1;
        let next_symbol = production.output.get(next_symbol_index).unwrap();

        if current_symbol.ne(next_symbol) {
            let next_symbol_first_symbols = first_symbols_map.get(next_symbol).unwrap();

            if !next_symbol_first_symbols.contains(grammar.get_epsilon_symbol()) {
                first_indexes.clear();
            }

            first_indexes.push(next_symbol_index);

            let mut current_symbol_follow_symbols =
                follow_symbols_map.get(current_symbol).unwrap().borrow_mut();

            follow_symbols_updated |= Self::update_follow_symbols_lambda_chain(
                grammar,
                first_symbols_map,
                production,
                first_indexes,
                &mut current_symbol_follow_symbols,
            );
        }

        follow_symbols_updated
    }

    fn insert_many(
        hash_set_ref: &mut RefMut<HashSet<T>>,
        iterator: &mut dyn Iterator<Item = &T>,
    ) -> bool {
        let mut item_inserted: bool = false;

        iterator.for_each(|symbol_first_symbol| {
            item_inserted |= hash_set_ref.insert(symbol_first_symbol.clone());
        });

        item_inserted
    }

    fn update_follow_symbols_lambda_chain(
        grammar: &ContextFreeGrammar<T>,
        first_symbols_map: &SymbolsMap<T>,
        production: &ContextFreeGrammarProduction<T>,
        first_indexes: &Vec<usize>,
        current_symbol_follow_symbols: &mut HashSet<T>,
    ) -> bool {
        let mut follow_symbols_updated: bool = false;

        first_indexes.iter().for_each(|index| {
            let symbol: &T = production.output.get(*index).unwrap();

            first_symbols_map
                .get(symbol)
                .unwrap()
                .iter()
                .filter(|symbol| (grammar.get_epsilon_symbol()).ne(*symbol))
                .for_each(|symbol| {
                    follow_symbols_updated |= current_symbol_follow_symbols.insert(symbol.clone());
                });
        });

        follow_symbols_updated
    }
}
