use std::cell::{RefCell, RefMut, Ref};
use std::collections::{HashMap, HashSet};
use std::iter::{FromIterator, Map};
use std::hash::Hash;

use crate::grammar::context_free_grammar_production::ContextFreeGrammarProduction;
use crate::grammar::first_follow_symbols::{FirstFollowSymbols, SymbolsMap};
use std::slice::Iter;
use std::borrow::Borrow;

pub struct ContextFreeGrammar<T> {
    epsilon_symbol: T,
    initial_symbol: T,
    non_terminal_symbols_set: HashSet<T>,
    productions: HashMap<T, Vec<ContextFreeGrammarProduction<T>>>,
    terminal_symbols_set: HashSet<T>,
}

impl<T> ContextFreeGrammar<T> {
    pub fn get_epsilon_symbol(&self) -> &T {
        &self.epsilon_symbol
    }

    pub fn get_initial_symbol(&self) -> &T {
        &self.initial_symbol
    }

    fn converge<M, F>(model: &M, callback: F) -> ()
        where
            F: Fn(&M, &mut bool) -> (),
    {
        let mut updated_at_iter: bool = false;

        loop {
            callback(model, &mut updated_at_iter);

            if updated_at_iter {
                updated_at_iter = false;
            } else {
                break;
            }
        }
    }
}

impl<T: Clone> ContextFreeGrammar<T> {
    pub fn get_non_terminal_symbols(&self) -> Vec<T> {
        self.non_terminal_symbols_set
            .iter()
            .map(|symbol| symbol.clone())
            .collect()
    }

    pub fn get_terminal_symbols(&self) -> Vec<T> {
        self.terminal_symbols_set
            .iter()
            .map(|symbol| symbol.clone())
            .collect()
    }
}

impl<T: Eq + Hash> ContextFreeGrammar<T> {
    pub fn get_productions(&self, symbol: &T) -> Option<&Vec<ContextFreeGrammarProduction<T>>> {
        self.productions.get(symbol)
    }
}

impl<T: Clone + Eq + Hash> ContextFreeGrammar<T> {
    pub fn get_first_follow_symbols(&self) -> FirstFollowSymbols<T> {
        let first_symbols_map: SymbolsMap<T> = self.get_first_symbols();
        let follow_symbols_map: SymbolsMap<T> = self.get_follow_symbols(&first_symbols_map);

        FirstFollowSymbols::new(first_symbols_map, follow_symbols_map)
    }

    pub fn new(
        epsilon_symbol: T,
        initial_symbol: T,
        productions: Vec<ContextFreeGrammarProduction<T>>,
    ) -> Self {
        let productions_hash_map: HashMap<T, Vec<ContextFreeGrammarProduction<T>>> =
            Self::new_process_productions_map(productions);

        Self::new_check_productions_map(&epsilon_symbol, &productions_hash_map);

        let non_terminal_symbols: Vec<T> = productions_hash_map.keys().map(|key| key.clone()).collect();
        let non_terminal_symbols_set: HashSet<T> = HashSet::from_iter(non_terminal_symbols.into_iter());
        let terminal_symbols_set: HashSet<T> = ContextFreeGrammar::build_terminal_symbols_set(
            &non_terminal_symbols_set,
            &productions_hash_map,
        );

        ContextFreeGrammar {
            epsilon_symbol,
            initial_symbol,
            non_terminal_symbols_set,
            productions: productions_hash_map,
            terminal_symbols_set,
        }
    }

    fn build_terminal_symbols_set(
        non_terminal_symbols_set: &HashSet<T>,
        productions_hash_map: &HashMap<T, Vec<ContextFreeGrammarProduction<T>>>,
    ) -> HashSet<T> {
        let mut terminal_symbols_set: HashSet<T> = HashSet::new();

        productions_hash_map.values().for_each(|productions| {
            productions.iter().for_each(|production| {
                production.output.iter().for_each(|symbol| {
                    if !non_terminal_symbols_set.contains(symbol) {
                        terminal_symbols_set.insert(symbol.clone());
                    }
                });
            });
        });

        terminal_symbols_set
    }

    fn get_first_symbols(&self) -> SymbolsMap<T> {
        let non_terminal_symbols_iterator =
            self.get_first_symbols_non_terminal_map();
        let terminal_symbols_iterator = self.get_first_symbols_terminal_map();

        let first_symbols_map: HashMap<T, RefCell<HashSet<T>>> = HashMap::from_iter(
            non_terminal_symbols_iterator.chain(terminal_symbols_iterator),
        );

        ContextFreeGrammar::<T>::converge(&first_symbols_map, |model: &HashMap<T, RefCell<HashSet<T>>>, updated_at_iter: &mut bool| {
            for (symbol, tuple_productions) in &self.productions {
                let mut tuple_symbol_first_symbols: RefMut<HashSet<T>> =
                    first_symbols_map.get(symbol).unwrap().borrow_mut();

                for production in tuple_productions {
                    *updated_at_iter |= self.get_first_symbols_process_production(
                        model,
                        &mut tuple_symbol_first_symbols,
                        &production,
                    );
                }
            }
        });

        Self::to_symbols_map(first_symbols_map)
    }

    fn get_first_symbols_non_terminal_map(&self)
        -> Map<std::collections::hash_set::Iter<T>, fn(&T) -> (T, RefCell<HashSet<T>>)> {
        (&self.non_terminal_symbols_set)
            .into_iter()
            .map(|symbol| (symbol.clone(), RefCell::new(HashSet::new())))
    }

    fn get_first_symbols_process_production(
        &self,
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
                    first_symbols_updated |= self.get_first_symbols_process_production_symbol(
                        first_symbols_map,
                        tuple_symbol_first_symbols,
                        &mut output_iterator_option,
                        &production.input,
                        production_symbol,
                    );
                },
                None => {
                    first_symbols_updated |= tuple_symbol_first_symbols.insert(
                        self.epsilon_symbol.clone()
                    );
                    break;
                }
            }
        }

        first_symbols_updated
    }

    fn get_first_symbols_process_production_non_terminal_symbol(
        &self,
        first_symbols_map: &HashMap<T, RefCell<HashSet<T>>>,
        tuple_symbol_first_symbols: &mut RefMut<HashSet<T>>,
        output_iterator_option: &mut Option<Iter<T>>,
        production_symbol: &T,
    ) -> bool {
        let mut first_symbols_updated = false;

        let symbol_first_symbols: Ref<HashSet<T>> =
            first_symbols_map.get(production_symbol).unwrap().borrow();

        if symbol_first_symbols.contains(&self.epsilon_symbol) {
            first_symbols_updated |= ContextFreeGrammar::insert_many(
                tuple_symbol_first_symbols,
                &mut symbol_first_symbols
                    .iter()
                    .filter(|symbol| self.epsilon_symbol.ne(*symbol)),
            );
        } else {
            first_symbols_updated |= ContextFreeGrammar::insert_many(
                tuple_symbol_first_symbols,
                &mut symbol_first_symbols.iter(),
            );

            *output_iterator_option = None
        }

        first_symbols_updated
    }

    fn get_first_symbols_process_production_symbol(
        &self,
       first_symbols_map: &HashMap<T, RefCell<HashSet<T>>>,
       tuple_symbol_first_symbols: &mut RefMut<HashSet<T>>,
       output_iterator_option: &mut Option<Iter<T>>,
       production_input: &T,
       production_symbol: &T,
    ) -> bool {
        let mut first_symbols_updated = false;

        if production_symbol.eq(production_input) {
            if !tuple_symbol_first_symbols.contains(&self.epsilon_symbol) {
                *output_iterator_option = None;
            }
        } else {
            if (&self.non_terminal_symbols_set).contains(production_symbol) {
                first_symbols_updated |= self.get_first_symbols_process_production_non_terminal_symbol(
                    first_symbols_map,
                    tuple_symbol_first_symbols,
                    output_iterator_option,
                    production_symbol,
                );
            } else {
                first_symbols_updated |= tuple_symbol_first_symbols.insert(production_symbol.clone());

                *output_iterator_option = None
            }
        }

        first_symbols_updated
    }

    fn get_first_symbols_terminal_map(&self)
        -> Map<std::collections::hash_set::Iter<T>, fn(&T) -> (T, RefCell<HashSet<T>>)> {
        (&self.terminal_symbols_set)
            .into_iter()
            .map(
                |symbol| {
                    let mut hash_set = HashSet::new();
                    hash_set.insert(symbol.clone());

                    (symbol.clone(), RefCell::new(hash_set))
                },
            )
    }

    fn get_follow_symbols(&self, first_symbols_map: &SymbolsMap<T>) -> SymbolsMap<T> {
        let follow_symbols_map: HashMap<T, RefCell<HashSet<T>>> = HashMap::from_iter(
            self.get_non_terminal_symbols()
                .into_iter()
                .map(|symbol| (symbol, RefCell::new(HashSet::new()))),
        );

        ContextFreeGrammar::<T>::converge(&follow_symbols_map, |model, updated_at_iter| {
            for (_, tuple_productions) in &self.productions {
                for production in tuple_productions {
                    *updated_at_iter |= self.get_follow_symbols_process_production(
                        first_symbols_map,
                        model,
                        production,
                    );
                }
            }
        });

        Self::to_symbols_map(follow_symbols_map)
    }

    fn get_follow_symbols_process_production(
       &self,
       first_symbols_map: &SymbolsMap<T>,
       follow_symbols_map: &HashMap<T, RefCell<HashSet<T>>>,
       production: &ContextFreeGrammarProduction<T>,
    ) -> bool {
        let mut follow_symbols_updated: bool = self.get_follow_symbols_process_production_last_epsilon_chain(
            first_symbols_map,
            follow_symbols_map,
            &production,
        );

        let last_production_output_index: usize = production.output.len() - 1;

        if last_production_output_index > 0 {
            let mut production_output_index: usize = last_production_output_index - 1;
            let mut first_indexes: Vec<usize> = vec![];

            loop {
                follow_symbols_updated |= self.get_follow_symbols_process_production_symbol(
                    first_symbols_map,
                    follow_symbols_map,
                    production,
                    production_output_index,
                    &mut first_indexes,
                );

                if production_output_index == 0 {
                    break
                } else {
                    production_output_index -= 1;
                }
            }
        }

        follow_symbols_updated
    }

    fn get_follow_symbols_process_production_last_epsilon_chain(
        &self,
        first_symbols_map: &SymbolsMap<T>,
        follow_symbols_map: &HashMap<T, RefCell<HashSet<T>>>,
        production: &ContextFreeGrammarProduction<T>,
    ) -> bool {
        let mut follow_symbols_updated: bool = false;

        let mut production_output_index: usize = production.output.len() - 1;

        loop {
            let output_symbol: &T = production.output.get(production_output_index).unwrap();

            if output_symbol.ne(&production.input) && self.non_terminal_symbols_set.contains(output_symbol) {
                let input_follow_symbols = follow_symbols_map.get(&production.input).unwrap().borrow();
                let mut symbol_follow_symbols_ref = follow_symbols_map.get(output_symbol).unwrap().borrow_mut();

                input_follow_symbols.iter().for_each(|symbol| {
                    follow_symbols_updated |= symbol_follow_symbols_ref.insert(symbol.clone());
                });
            }

            let output_symbol_first_symbols: &HashSet<T> = first_symbols_map
                .get(output_symbol).unwrap().borrow();

            let output_symbol_first_symbols_contains_epsilon: bool =
                output_symbol_first_symbols.contains(&self.epsilon_symbol);

            if production_output_index == 0 || !output_symbol_first_symbols_contains_epsilon {
                break;
            } else {
                production_output_index -= 1;
            }
        }

        follow_symbols_updated
    }

    fn get_follow_symbols_process_production_symbol(
        &self,
        first_symbols_map: &SymbolsMap<T>,
        follow_symbols_map: &HashMap<T, RefCell<HashSet<T>>>,
        production: &ContextFreeGrammarProduction<T>,
        production_output_index: usize,
        first_indexes: &mut Vec<usize>,
     ) -> bool {
        let follow_symbols_updated: bool;

        let current_symbol: &T = production.output.get(production_output_index).unwrap();

        if self.non_terminal_symbols_set.contains(current_symbol) {
            follow_symbols_updated = self.get_follow_symbols_process_production_non_terminal_symbol(
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
        &self,
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

            if !next_symbol_first_symbols.contains(self.get_epsilon_symbol()) {
                first_indexes.clear();
            }

            first_indexes.push(next_symbol_index);

            let mut current_symbol_follow_symbols = follow_symbols_map
                .get(current_symbol)
                .unwrap()
                .borrow_mut();

            follow_symbols_updated |= self.update_follow_symbols_lambda_chain(
                first_symbols_map, production, first_indexes, &mut current_symbol_follow_symbols,
            );
        }

        follow_symbols_updated
    }

    fn insert_many(hash_set_ref: &mut RefMut<HashSet<T>>, iterator: &mut dyn Iterator<Item=&T>) -> bool {
        let mut item_inserted: bool = false;

        iterator.for_each(|symbol_first_symbol| {
            item_inserted |= hash_set_ref.insert(symbol_first_symbol.clone());
        });

        item_inserted
    }

    fn new_check_productions_map(
        epsilon_symbol: &T,
        productions_map: &HashMap<T, Vec<ContextFreeGrammarProduction<T>>>,
    ) -> () {
        for (symbol, productions) in productions_map {
            if symbol.eq(epsilon_symbol) {
                panic!("Expected epsilon symbol not to generate any symbols!");
            }

            for production in productions {
                if production.output.len() == 0 {
                    panic!("Expected production to have at least one symbol");
                } else {
                    if production.output.len() > 1 && production.output.contains(epsilon_symbol) {
                        panic!("Expected epsilon production not to have additional symbol");
                    }
                }
            }
        }
    }

    fn new_process_productions_map(productions: Vec<ContextFreeGrammarProduction<T>>) -> HashMap<T, Vec<ContextFreeGrammarProduction<T>>> {
        let mut productions_hash_map: HashMap<T, Vec<ContextFreeGrammarProduction<T>>> =
            HashMap::new();

        for production in productions {
            let current_productions: Option<&mut Vec<ContextFreeGrammarProduction<T>>> =
                productions_hash_map.get_mut(&production.input);

            match current_productions {
                None => {
                    productions_hash_map.insert(production.input.clone(), vec![production]);
                }
                Some(productions) => productions.push(production),
            }
        }

        productions_hash_map
    }

    fn to_symbols_map(symbols_ref_map: HashMap<T, RefCell<HashSet<T>>>) -> SymbolsMap<T> {
        HashMap::from_iter(
            symbols_ref_map.into_iter().map(
                |(symbol, symbols_ref)|
                    (symbol, symbols_ref.take())
            )
        )
    }

    fn update_follow_symbols_lambda_chain(
        &self,
        first_symbols_map: &SymbolsMap<T>,
        production: &ContextFreeGrammarProduction<T>,
        first_indexes: &Vec<usize>,
        current_symbol_follow_symbols: &mut HashSet<T>,
    ) -> bool {
        let mut follow_symbols_updated: bool = false;
        
        first_indexes.iter().for_each(|index| {
            let symbol: &T = production.output.get(*index).unwrap();

            first_symbols_map.get(symbol)
                .unwrap()
                .iter()
                .filter(|symbol| (&self.epsilon_symbol).ne(*symbol))
                .for_each(|symbol| {
                    follow_symbols_updated |= current_symbol_follow_symbols.insert(symbol.clone());
                });
        });

        follow_symbols_updated
    }
}
