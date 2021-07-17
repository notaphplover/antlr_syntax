#[cfg(test)]
mod test {

    mod new {
        use crate::grammar::context_free_grammar::ContextFreeGrammar;
        use crate::grammar::context_free_grammar_production::ContextFreeGrammarProduction;

        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
        enum TerminalTokenTypeTest {
            Eof,
            Id,
        }

        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
        enum SyntaxTokenTest {
            Epsilon,
            Module,
            Terminal(TerminalTokenTypeTest),
        }

        #[test]
        fn it_creates_a_new_instance() -> () {
            let grammar_productions: Vec<ContextFreeGrammarProduction<SyntaxTokenTest>> =
                vec![ContextFreeGrammarProduction::new(
                    SyntaxTokenTest::Module,
                    vec![
                        SyntaxTokenTest::Terminal(TerminalTokenTypeTest::Id),
                        SyntaxTokenTest::Terminal(TerminalTokenTypeTest::Eof),
                    ],
                )];

            ContextFreeGrammar::new(
                SyntaxTokenTest::Epsilon,
                SyntaxTokenTest::Module,
                grammar_productions,
            );
        }

        #[test]
        #[should_panic]
        fn it_panics_if_production_input_is_epsilon() {
            let grammar_productions: Vec<ContextFreeGrammarProduction<SyntaxTokenTest>> =
                vec![ContextFreeGrammarProduction::new(
                    SyntaxTokenTest::Epsilon,
                    vec![SyntaxTokenTest::Terminal(TerminalTokenTypeTest::Id)],
                )];

            ContextFreeGrammar::new(
                SyntaxTokenTest::Epsilon,
                SyntaxTokenTest::Module,
                grammar_productions,
            );
        }

        #[test]
        #[should_panic]
        fn it_panics_if_production_output_has_no_symbols() -> () {
            let grammar_productions: Vec<ContextFreeGrammarProduction<SyntaxTokenTest>> =
                vec![ContextFreeGrammarProduction::new(
                    SyntaxTokenTest::Module,
                    vec![],
                )];

            ContextFreeGrammar::new(
                SyntaxTokenTest::Epsilon,
                SyntaxTokenTest::Module,
                grammar_productions,
            );
        }

        #[test]
        #[should_panic]
        fn it_panics_if_production_output_has_epsilon_and_any_other_symbol() -> () {
            let grammar_productions: Vec<ContextFreeGrammarProduction<SyntaxTokenTest>> =
                vec![ContextFreeGrammarProduction::new(
                    SyntaxTokenTest::Module,
                    vec![
                        SyntaxTokenTest::Terminal(TerminalTokenTypeTest::Id),
                        SyntaxTokenTest::Epsilon,
                    ],
                )];

            ContextFreeGrammar::new(
                SyntaxTokenTest::Epsilon,
                SyntaxTokenTest::Module,
                grammar_productions,
            );
        }
    }
}
