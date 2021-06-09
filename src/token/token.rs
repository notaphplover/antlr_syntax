pub struct Token<TLex, TSyntax> {
    pub lex: Option<TLex>,
    pub t_type: TSyntax,
}

impl<TLex, TSyntax> Token<TLex, TSyntax> {
    pub fn new(lex: Option<TLex>, t_type: TSyntax) -> Self {
        Self { lex, t_type }
    }
}

impl<TLex: Clone, TSyntax: Clone> Clone for Token<TLex, TSyntax> {
    fn clone(&self) -> Self {
        Self::new(self.lex.clone(), self.t_type.clone())
    }
}
