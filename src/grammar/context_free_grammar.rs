use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::iter::FromIterator;

use crate::grammar::context_free_grammar_production::ContextFreeGrammarProduction;

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

    pub fn is_non_terminal(&self, symbol: &T) -> bool {
        self.non_terminal_symbols_set.contains(symbol)
    }

    pub fn is_terminal(&self, symbol: &T) -> bool {
        self.terminal_symbols_set.contains(symbol)
    }
}

impl<T: Clone + Eq + Hash> ContextFreeGrammar<T> {
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
}
