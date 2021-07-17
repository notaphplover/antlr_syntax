use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::iter::FromIterator;

use crate::grammar::context_free_grammar::ContextFreeGrammar;
use crate::grammar::context_free_grammar_production::ContextFreeGrammarProduction;
use crate::grammar::first_follow_symbols::FirstFollowSymbols;

pub struct RecursiveDescentParserTransitions<T> {
    table: HashMap<T, HashMap<T, Vec<ContextFreeGrammarProduction<T>>>>,
}

impl<T: Eq + Hash> RecursiveDescentParserTransitions<T> {
    pub fn get_productions(
        &self,
        symbol_to_derive: &T,
        first_symbol: &T,
    ) -> Option<&Vec<ContextFreeGrammarProduction<T>>> {
        let derivations_map_option = self.table.get(symbol_to_derive);

        match derivations_map_option {
            Some(derivations_map) => derivations_map.get(first_symbol),
            None => None,
        }
    }
}

impl<T: Clone + Eq + Hash> RecursiveDescentParserTransitions<T> {
    pub fn from(
        grammar: &ContextFreeGrammar<T>,
        first_follow_symbols: &FirstFollowSymbols<T>,
    ) -> RecursiveDescentParserTransitions<T> {
        RecursiveDescentParserTransitions::inner_from(grammar, first_follow_symbols)
    }

    fn inner_from(
        grammar: &ContextFreeGrammar<T>,
        first_follow_symbols: &FirstFollowSymbols<T>,
    ) -> RecursiveDescentParserTransitions<T> {
        let non_terminal_symbols = grammar.get_non_terminal_symbols();
        let terminal_symbols = grammar.get_terminal_symbols();
        let mut table = Self::inner_from_initial_table(&non_terminal_symbols, &terminal_symbols);

        Self::inner_from_process_productions(
            grammar,
            first_follow_symbols,
            &mut table,
            &non_terminal_symbols,
        );

        Self { table }
    }

    /*
     * Implementation notes:
     *
     * This function determines the terminal symbols a to set the production A → α
     * to M[A; a]
     *
     * From Compilers - Principles, Techniques, and Tools:
     *
     * -----------------------------------------------------------------------------
     *
     * Algorithm 4.31 : Construction of a predictive parsing table.
     *
     * INPUT: Grammar G.
     * OUTPUT: Parsing table M.
     * METHOD: For each production A → α of the grammar, do the following:
     *
     *     1. For each terminal a in FIRST(α), add A → α to M[A; a].
     *     2. If ε is in FIRST(α), then for each terminal b in FOLLOW(A), add A → α
     *        to M[A; b]. If ε is in FIRST(α) and $ is in FOLLOW(A), add A → α to
     *        M[A; $] as well.
     *
     * -----------------------------------------------------------------------------
     *
     * Unfortunately we don't compute FIRST(α). Fortunately, $ is handled as an
     * additional terminal symbol in the grammar. Keeping this in mind, there's a
     * way to determine all the terminal symbols a in which M[A; a] = A → α
     *
     * -----------------------------------------------------------------------------
     *
     * Alternative algorithm:
     *
     * INPUT: Grammar G.
     * OUTPUT: Parsing table M.
     * METHOD: For each production A → Bα of the grammar, do the following:
     *
     *     1. For each terminal a in FIRST(B), add A → α to M[A; a].
     *     2.1. If B is terminal and B is ε, then for each terminal b in FOLLOW(A),
     *          add A → α to M[A; b]
     *     2.2. If B is non terminal and ε is in FIRST(B), then for each terminal b
     *          in FOLLOW(B), add A → α to M[A; b].
     */
    fn inner_from_get_production_first_symbols(
        grammar: &ContextFreeGrammar<T>,
        production: &ContextFreeGrammarProduction<T>,
        first_follow_symbols: &FirstFollowSymbols<T>,
    ) -> HashSet<T> {
        let production_first_symbols: HashSet<T>;

        let symbol = production.output.get(0).unwrap();

        let symbol_first_symbols = first_follow_symbols.get_first_symbols(&symbol).unwrap();

        if symbol_first_symbols.contains(grammar.get_epsilon_symbol()) {
            production_first_symbols =
                Self::inner_from_get_production_first_symbols_on_epsilon_first(
                    grammar,
                    production,
                    first_follow_symbols,
                    symbol,
                    symbol_first_symbols,
                );
        } else {
            production_first_symbols =
                Self::inner_from_get_production_first_symbols_on_non_epsilon_first(
                    symbol_first_symbols,
                );
        }

        production_first_symbols
    }

    fn inner_from_get_production_first_symbols_on_epsilon_first(
        grammar: &ContextFreeGrammar<T>,
        production: &ContextFreeGrammarProduction<T>,
        first_follow_symbols: &FirstFollowSymbols<T>,
        symbol: &T,
        symbol_first_symbols: &HashSet<T>,
    ) -> HashSet<T> {
        let mut production_first_symbols = HashSet::new();

        symbol_first_symbols
            .iter()
            .filter(|symbol| grammar.get_epsilon_symbol().ne(*symbol))
            .for_each(|symbol| {
                production_first_symbols.insert(symbol.clone());
            });

        let symbol_follow_symbols: &HashSet<T>;

        if grammar.get_epsilon_symbol().eq(symbol) {
            symbol_follow_symbols = first_follow_symbols
                .get_follow_symbols(&production.input)
                .unwrap();
        } else {
            symbol_follow_symbols = first_follow_symbols.get_follow_symbols(&symbol).unwrap();
        }

        symbol_follow_symbols.iter().for_each(|symbol| {
            production_first_symbols.insert(symbol.clone());
        });

        production_first_symbols
    }

    fn inner_from_get_production_first_symbols_on_non_epsilon_first(
        symbol_first_symbols: &HashSet<T>,
    ) -> HashSet<T> {
        let mut production_first_symbols = HashSet::new();

        symbol_first_symbols.iter().for_each(|symbol| {
            production_first_symbols.insert(symbol.clone());
        });

        production_first_symbols
    }

    fn inner_from_initial_table(
        non_terminal_symbols: &Vec<T>,
        terminal_symbols: &Vec<T>,
    ) -> HashMap<T, HashMap<T, Vec<ContextFreeGrammarProduction<T>>>> {
        HashMap::from_iter(non_terminal_symbols.iter().map(|symbol| {
            (
                symbol.clone(),
                HashMap::from_iter(
                    terminal_symbols
                        .iter()
                        .map(|symbol| (symbol.clone(), vec![])),
                ),
            )
        }))
    }

    fn inner_from_process_productions(
        grammar: &ContextFreeGrammar<T>,
        first_follow_symbols: &FirstFollowSymbols<T>,
        table: &mut HashMap<T, HashMap<T, Vec<ContextFreeGrammarProduction<T>>>>,
        non_terminal_symbols: &Vec<T>,
    ) {
        for non_terminal_symbol in non_terminal_symbols {
            let symbol_productions = grammar.get_productions(non_terminal_symbol).unwrap();

            let symbol_ref_table = table.get_mut(non_terminal_symbol).unwrap();

            for symbol_production in symbol_productions {
                let production_first_symbols = Self::inner_from_get_production_first_symbols(
                    grammar,
                    symbol_production,
                    first_follow_symbols,
                );

                for production_first_symbol in &production_first_symbols {
                    let symbol_symbol_productions =
                        symbol_ref_table.get_mut(production_first_symbol).unwrap();

                    symbol_symbol_productions.push((*symbol_production).clone());
                }
            }
        }
    }
}
