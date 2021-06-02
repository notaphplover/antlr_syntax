pub struct ContextFreeGrammarProduction<TType> {
    pub input: TType,
    pub output: Vec<TType>,
}

impl<T: PartialEq<T>> ContextFreeGrammarProduction<T> {
    pub fn new(input: T, output: Vec<T>) -> Self {
        ContextFreeGrammarProduction { input, output }
    }
}

impl<TType: Clone + PartialEq> Clone for ContextFreeGrammarProduction<TType> {
    fn clone(&self) -> Self {
        Self::new(self.input.clone(), self.output.clone())
    }
}
