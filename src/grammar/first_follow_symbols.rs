use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub type SymbolsMap<T> = HashMap<T, HashSet<T>>;

pub struct FirstFollowSymbols<T> {
    first_symbols: SymbolsMap<T>,
    follow_symbols: SymbolsMap<T>,
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
}