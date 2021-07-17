#[cfg(test)]
mod test {
    mod from_grammar {
        use crate::grammar::context_free_grammar::ContextFreeGrammar;
        use crate::grammar::context_free_grammar_production::ContextFreeGrammarProduction;
        use crate::grammar::first_follow_symbols::FirstFollowSymbols;
        use crate::parser::recursive_descent_parser_transitions::RecursiveDescentParserTransitions;

        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
        enum TerminalTokenTypeTest {
            Eof,
            Eos,
            Eq,
            Id,
        }

        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
        enum SyntaxTokenTest {
            Epsilon,
            Expression,
            Module,
            Sentence,
            Terminal(TerminalTokenTypeTest),
        }

        fn equals<T: PartialEq>(
            first_production: &ContextFreeGrammarProduction<T>,
            second_production: &ContextFreeGrammarProduction<T>,
        ) -> bool {
            if first_production.input.ne(&second_production.input) {
                return false;
            }

            if first_production.output.len() != second_production.output.len() {
                return false;
            }

            for i in 0..first_production.output.len() {
                if first_production.output.get(i).unwrap()
                    != second_production.output.get(i).unwrap()
                {
                    return false;
                }
            }

            true
        }

        #[test]
        fn it_adds_first_symbols_productions() -> () {
            let grammar_productions: Vec<ContextFreeGrammarProduction<SyntaxTokenTest>> = vec![
                ContextFreeGrammarProduction::new(
                    SyntaxTokenTest::Module,
                    vec![
                        SyntaxTokenTest::Expression,
                        SyntaxTokenTest::Terminal(TerminalTokenTypeTest::Eof),
                    ],
                ),
                ContextFreeGrammarProduction::new(
                    SyntaxTokenTest::Expression,
                    vec![
                        SyntaxTokenTest::Terminal(TerminalTokenTypeTest::Id),
                        SyntaxTokenTest::Terminal(TerminalTokenTypeTest::Eos),
                    ],
                ),
            ];

            let grammar: ContextFreeGrammar<SyntaxTokenTest> = ContextFreeGrammar::new(
                SyntaxTokenTest::Epsilon,
                SyntaxTokenTest::Module,
                grammar_productions,
            );

            let first_follow_symbols: FirstFollowSymbols<SyntaxTokenTest> =
                FirstFollowSymbols::from(&grammar);

            let recursive_descent_parser_transitions: RecursiveDescentParserTransitions<
                SyntaxTokenTest,
            > = RecursiveDescentParserTransitions::from(&grammar, &first_follow_symbols);

            let module_id_productions = recursive_descent_parser_transitions
                .get_productions(
                    &SyntaxTokenTest::Module,
                    &SyntaxTokenTest::Terminal(TerminalTokenTypeTest::Id),
                )
                .unwrap();

            let first_module_id_production = module_id_productions.get(0).unwrap();
            let expected_first_module_id_production = ContextFreeGrammarProduction::new(
                SyntaxTokenTest::Module,
                vec![
                    SyntaxTokenTest::Expression,
                    SyntaxTokenTest::Terminal(TerminalTokenTypeTest::Eof),
                ],
            );

            assert_eq!(module_id_productions.len(), 1);
            assert!(equals(
                first_module_id_production,
                &expected_first_module_id_production
            ));
        }

        #[test]
        fn it_adds_follow_symbols_productions() -> () {
            let grammar_productions: Vec<ContextFreeGrammarProduction<SyntaxTokenTest>> = vec![
                ContextFreeGrammarProduction::new(
                    SyntaxTokenTest::Module,
                    vec![
                        SyntaxTokenTest::Sentence,
                        SyntaxTokenTest::Terminal(TerminalTokenTypeTest::Eof),
                    ],
                ),
                ContextFreeGrammarProduction::new(
                    SyntaxTokenTest::Sentence,
                    vec![SyntaxTokenTest::Epsilon],
                ),
                ContextFreeGrammarProduction::new(
                    SyntaxTokenTest::Sentence,
                    vec![
                        SyntaxTokenTest::Terminal(TerminalTokenTypeTest::Id),
                        SyntaxTokenTest::Terminal(TerminalTokenTypeTest::Eq),
                        SyntaxTokenTest::Expression,
                        SyntaxTokenTest::Terminal(TerminalTokenTypeTest::Eos),
                    ],
                ),
                ContextFreeGrammarProduction::new(
                    SyntaxTokenTest::Expression,
                    vec![SyntaxTokenTest::Terminal(TerminalTokenTypeTest::Id)],
                ),
            ];

            let grammar: ContextFreeGrammar<SyntaxTokenTest> = ContextFreeGrammar::new(
                SyntaxTokenTest::Epsilon,
                SyntaxTokenTest::Module,
                grammar_productions,
            );

            let first_follow_symbols: FirstFollowSymbols<SyntaxTokenTest> =
                FirstFollowSymbols::from(&grammar);

            let recursive_descent_parser_transitions: RecursiveDescentParserTransitions<
                SyntaxTokenTest,
            > = RecursiveDescentParserTransitions::from(&grammar, &first_follow_symbols);

            let module_eof_productions = recursive_descent_parser_transitions
                .get_productions(
                    &SyntaxTokenTest::Module,
                    &SyntaxTokenTest::Terminal(TerminalTokenTypeTest::Id),
                )
                .unwrap();

            let first_module_eof_production = module_eof_productions.get(0).unwrap();
            let expected_first_module_id_production = ContextFreeGrammarProduction::new(
                SyntaxTokenTest::Module,
                vec![
                    SyntaxTokenTest::Sentence,
                    SyntaxTokenTest::Terminal(TerminalTokenTypeTest::Eof),
                ],
            );

            assert_eq!(module_eof_productions.len(), 1);
            assert!(equals(
                first_module_eof_production,
                &expected_first_module_id_production
            ));
        }

        #[test]
        fn it_adds_once_on_first_follow_conflict() -> () {
            let grammar_productions: Vec<ContextFreeGrammarProduction<SyntaxTokenTest>> = vec![
                ContextFreeGrammarProduction::new(
                    SyntaxTokenTest::Module,
                    vec![
                        SyntaxTokenTest::Sentence,
                        SyntaxTokenTest::Sentence,
                        SyntaxTokenTest::Terminal(TerminalTokenTypeTest::Eof),
                    ],
                ),
                ContextFreeGrammarProduction::new(
                    SyntaxTokenTest::Sentence,
                    vec![SyntaxTokenTest::Epsilon],
                ),
                ContextFreeGrammarProduction::new(
                    SyntaxTokenTest::Sentence,
                    vec![
                        SyntaxTokenTest::Terminal(TerminalTokenTypeTest::Id),
                        SyntaxTokenTest::Terminal(TerminalTokenTypeTest::Eq),
                        SyntaxTokenTest::Expression,
                        SyntaxTokenTest::Terminal(TerminalTokenTypeTest::Eos),
                    ],
                ),
                ContextFreeGrammarProduction::new(
                    SyntaxTokenTest::Expression,
                    vec![SyntaxTokenTest::Terminal(TerminalTokenTypeTest::Id)],
                ),
            ];

            let grammar: ContextFreeGrammar<SyntaxTokenTest> = ContextFreeGrammar::new(
                SyntaxTokenTest::Epsilon,
                SyntaxTokenTest::Module,
                grammar_productions,
            );

            let first_follow_symbols: FirstFollowSymbols<SyntaxTokenTest> =
                FirstFollowSymbols::from(&grammar);

            let recursive_descent_parser_transitions: RecursiveDescentParserTransitions<
                SyntaxTokenTest,
            > = RecursiveDescentParserTransitions::from(&grammar, &first_follow_symbols);

            let module_id_productions = recursive_descent_parser_transitions
                .get_productions(
                    &SyntaxTokenTest::Module,
                    &SyntaxTokenTest::Terminal(TerminalTokenTypeTest::Id),
                )
                .unwrap();

            let first_module_id_production = module_id_productions.get(0).unwrap();
            let expected_first_module_id_production = ContextFreeGrammarProduction::new(
                SyntaxTokenTest::Module,
                vec![
                    SyntaxTokenTest::Sentence,
                    SyntaxTokenTest::Sentence,
                    SyntaxTokenTest::Terminal(TerminalTokenTypeTest::Eof),
                ],
            );

            assert_eq!(module_id_productions.len(), 1);
            assert!(equals(
                first_module_id_production,
                &expected_first_module_id_production
            ));
        }

        #[test]
        fn it_adds_on_non_terminal_symbol_with_first_set_is_epsilon() {
            #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
            enum DummySyntaxTokenTest {
                Empty,
                Eof,
                Epsilon,
                S,
            }

            let grammar_productions: Vec<ContextFreeGrammarProduction<DummySyntaxTokenTest>> = vec![
                ContextFreeGrammarProduction::new(
                    DummySyntaxTokenTest::S,
                    vec![DummySyntaxTokenTest::Empty, DummySyntaxTokenTest::Eof],
                ),
                ContextFreeGrammarProduction::new(
                    DummySyntaxTokenTest::Empty,
                    vec![DummySyntaxTokenTest::Epsilon],
                ),
            ];

            let grammar: ContextFreeGrammar<DummySyntaxTokenTest> = ContextFreeGrammar::new(
                DummySyntaxTokenTest::Epsilon,
                DummySyntaxTokenTest::S,
                grammar_productions,
            );

            let first_follow_symbols: FirstFollowSymbols<DummySyntaxTokenTest> =
                FirstFollowSymbols::from(&grammar);

            let recursive_descent_parser_transitions: RecursiveDescentParserTransitions<
                DummySyntaxTokenTest,
            > = RecursiveDescentParserTransitions::from(&grammar, &first_follow_symbols);

            let empty_eof_productions = recursive_descent_parser_transitions
                .get_productions(&DummySyntaxTokenTest::Empty, &DummySyntaxTokenTest::Eof)
                .unwrap();

            let first_empty_eof_production = empty_eof_productions.get(0).unwrap();
            let expected_first_empty_eof_production = ContextFreeGrammarProduction::new(
                DummySyntaxTokenTest::Empty,
                vec![DummySyntaxTokenTest::Epsilon],
            );

            assert_eq!(empty_eof_productions.len(), 1);
            assert!(equals(
                first_empty_eof_production,
                &expected_first_empty_eof_production,
            ));
        }
    }
}
