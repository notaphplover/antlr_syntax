pub struct ContextFreeGrammarProduction<TType> {
    pub input: TType,
    pub output: Vec<TType>,
}

impl<T: PartialEq<T>> ContextFreeGrammarProduction<T> {
    pub fn new(input: T, output: Vec<T>) -> Self {
        ContextFreeGrammarProduction { input, output }
    }
}
