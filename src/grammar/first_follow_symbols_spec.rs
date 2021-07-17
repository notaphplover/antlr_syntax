#[cfg(test)]
mod test {

    mod get_first_follow_symbols {
        use crate::grammar::context_free_grammar::ContextFreeGrammar;
        use crate::grammar::context_free_grammar_production::ContextFreeGrammarProduction;
        use crate::grammar::first_follow_symbols::FirstFollowSymbols;

        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
        enum TerminalTokenTypeTest {
            Eof,
            Eos,
            Id,
        }

        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
        enum SyntaxTokenTest {
            Epsilon,
            Expression,
            Module,
            Terminal(TerminalTokenTypeTest),
        }

        #[test]
        fn it_returns_first_symbols() -> () {
            let grammar_productions: Vec<ContextFreeGrammarProduction<SyntaxTokenTest>> =
                vec![ContextFreeGrammarProduction::new(
                    SyntaxTokenTest::Module,
                    vec![
                        SyntaxTokenTest::Terminal(TerminalTokenTypeTest::Id),
                        SyntaxTokenTest::Terminal(TerminalTokenTypeTest::Eof),
                    ],
                )];

            let grammar: ContextFreeGrammar<SyntaxTokenTest> = ContextFreeGrammar::new(
                SyntaxTokenTest::Epsilon,
                SyntaxTokenTest::Module,
                grammar_productions,
            );

            let first_follow_symbols: FirstFollowSymbols<SyntaxTokenTest> =
                FirstFollowSymbols::from(&grammar);

            let module_first_symbols = first_follow_symbols
                .get_first_symbols(&SyntaxTokenTest::Module)
                .unwrap();

            assert_eq!(module_first_symbols.len(), 1);
            assert!(module_first_symbols
                .contains(&SyntaxTokenTest::Terminal(TerminalTokenTypeTest::Id)));
        }

        #[test]
        fn it_returns_first_symbols_terminal() -> () {
            let grammar_productions: Vec<ContextFreeGrammarProduction<SyntaxTokenTest>> =
                vec![ContextFreeGrammarProduction::new(
                    SyntaxTokenTest::Module,
                    vec![
                        SyntaxTokenTest::Terminal(TerminalTokenTypeTest::Id),
                        SyntaxTokenTest::Terminal(TerminalTokenTypeTest::Eof),
                    ],
                )];

            let grammar: ContextFreeGrammar<SyntaxTokenTest> = ContextFreeGrammar::new(
                SyntaxTokenTest::Epsilon,
                SyntaxTokenTest::Module,
                grammar_productions,
            );

            let first_follow_symbols: FirstFollowSymbols<SyntaxTokenTest> =
                FirstFollowSymbols::from(&grammar);

            let id_first_symbols = first_follow_symbols
                .get_first_symbols(&SyntaxTokenTest::Terminal(TerminalTokenTypeTest::Id))
                .unwrap();

            assert_eq!(id_first_symbols.len(), 1);
            assert!(
                id_first_symbols.contains(&SyntaxTokenTest::Terminal(TerminalTokenTypeTest::Id))
            );
        }

        #[test]
        fn it_returns_first_symbols_non_terminal() -> () {
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

            let module_first_symbols = first_follow_symbols
                .get_first_symbols(&SyntaxTokenTest::Module)
                .unwrap();

            assert_eq!(module_first_symbols.len(), 1);
            assert!(module_first_symbols
                .contains(&SyntaxTokenTest::Terminal(TerminalTokenTypeTest::Id)));
        }

        #[test]
        fn it_returns_first_symbols_non_terminal_epsilon() -> () {
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
                ContextFreeGrammarProduction::new(
                    SyntaxTokenTest::Expression,
                    vec![SyntaxTokenTest::Epsilon],
                ),
            ];

            let grammar: ContextFreeGrammar<SyntaxTokenTest> = ContextFreeGrammar::new(
                SyntaxTokenTest::Epsilon,
                SyntaxTokenTest::Module,
                grammar_productions,
            );

            let first_follow_symbols: FirstFollowSymbols<SyntaxTokenTest> =
                FirstFollowSymbols::from(&grammar);

            let module_first_symbols = first_follow_symbols
                .get_first_symbols(&SyntaxTokenTest::Module)
                .unwrap();

            assert_eq!(module_first_symbols.len(), 2);
            assert!(module_first_symbols
                .contains(&SyntaxTokenTest::Terminal(TerminalTokenTypeTest::Id)));
            assert!(module_first_symbols
                .contains(&SyntaxTokenTest::Terminal(TerminalTokenTypeTest::Eof)));
        }

        #[test]
        fn it_returns_first_epsilon_symbols_non_terminal_epsilon() -> () {
            let grammar_productions: Vec<ContextFreeGrammarProduction<SyntaxTokenTest>> = vec![
                ContextFreeGrammarProduction::new(
                    SyntaxTokenTest::Module,
                    vec![SyntaxTokenTest::Expression],
                ),
                ContextFreeGrammarProduction::new(
                    SyntaxTokenTest::Expression,
                    vec![
                        SyntaxTokenTest::Terminal(TerminalTokenTypeTest::Id),
                        SyntaxTokenTest::Terminal(TerminalTokenTypeTest::Eos),
                    ],
                ),
                ContextFreeGrammarProduction::new(
                    SyntaxTokenTest::Expression,
                    vec![SyntaxTokenTest::Epsilon],
                ),
            ];

            let grammar: ContextFreeGrammar<SyntaxTokenTest> = ContextFreeGrammar::new(
                SyntaxTokenTest::Epsilon,
                SyntaxTokenTest::Module,
                grammar_productions,
            );

            let first_follow_symbols: FirstFollowSymbols<SyntaxTokenTest> =
                FirstFollowSymbols::from(&grammar);

            let module_first_symbols = first_follow_symbols
                .get_first_symbols(&SyntaxTokenTest::Module)
                .unwrap();

            assert_eq!(module_first_symbols.len(), 2);
            assert!(module_first_symbols.contains(&SyntaxTokenTest::Epsilon));
            assert!(module_first_symbols
                .contains(&SyntaxTokenTest::Terminal(TerminalTokenTypeTest::Id)));
        }

        #[test]
        fn it_returns_no_additional_first_symbols_on_infinite_recursion() -> () {
            let grammar_productions: Vec<ContextFreeGrammarProduction<SyntaxTokenTest>> =
                vec![ContextFreeGrammarProduction::new(
                    SyntaxTokenTest::Module,
                    vec![SyntaxTokenTest::Module],
                )];

            let grammar: ContextFreeGrammar<SyntaxTokenTest> = ContextFreeGrammar::new(
                SyntaxTokenTest::Epsilon,
                SyntaxTokenTest::Module,
                grammar_productions,
            );

            let first_follow_symbols: FirstFollowSymbols<SyntaxTokenTest> =
                FirstFollowSymbols::from(&grammar);

            let module_first_symbols = first_follow_symbols
                .get_first_symbols(&SyntaxTokenTest::Module)
                .unwrap();

            assert_eq!(module_first_symbols.len(), 0);
        }

        #[test]
        fn it_returns_next_symbol_first_symbols_on_infinite_recursion_if_first_symbol_contains_epsilon(
        ) -> () {
            let grammar_productions: Vec<ContextFreeGrammarProduction<SyntaxTokenTest>> = vec![
                ContextFreeGrammarProduction::new(
                    SyntaxTokenTest::Module,
                    vec![
                        SyntaxTokenTest::Module,
                        SyntaxTokenTest::Terminal(TerminalTokenTypeTest::Eof),
                    ],
                ),
                ContextFreeGrammarProduction::new(
                    SyntaxTokenTest::Module,
                    vec![SyntaxTokenTest::Epsilon],
                ),
            ];

            let grammar: ContextFreeGrammar<SyntaxTokenTest> = ContextFreeGrammar::new(
                SyntaxTokenTest::Epsilon,
                SyntaxTokenTest::Module,
                grammar_productions,
            );

            let first_follow_symbols: FirstFollowSymbols<SyntaxTokenTest> =
                FirstFollowSymbols::from(&grammar);

            let module_first_symbols = first_follow_symbols
                .get_first_symbols(&SyntaxTokenTest::Module)
                .unwrap();

            assert_eq!(module_first_symbols.len(), 2);
            assert!(module_first_symbols.contains(&SyntaxTokenTest::Epsilon));
            assert!(module_first_symbols
                .contains(&SyntaxTokenTest::Terminal(TerminalTokenTypeTest::Eof)));
        }

        #[test]
        fn it_returns_follow_symbols() -> () {
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

            let expression_follow_symbols = first_follow_symbols
                .get_follow_symbols(&SyntaxTokenTest::Expression)
                .unwrap();

            assert_eq!(expression_follow_symbols.len(), 1);
            assert!(expression_follow_symbols
                .contains(&SyntaxTokenTest::Terminal(TerminalTokenTypeTest::Eof)));
        }

        #[test]
        fn it_returns_follow_symbols_non_terminal() -> () {
            let grammar_productions: Vec<ContextFreeGrammarProduction<SyntaxTokenTest>> = vec![
                ContextFreeGrammarProduction::new(
                    SyntaxTokenTest::Module,
                    vec![
                        SyntaxTokenTest::Expression,
                        SyntaxTokenTest::Module,
                        SyntaxTokenTest::Terminal(TerminalTokenTypeTest::Eof),
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

            let expression_follow_symbols = first_follow_symbols
                .get_follow_symbols(&SyntaxTokenTest::Expression)
                .unwrap();

            assert_eq!(expression_follow_symbols.len(), 1);
            assert!(expression_follow_symbols
                .contains(&SyntaxTokenTest::Terminal(TerminalTokenTypeTest::Id)));
        }

        #[test]
        fn it_returns_follow_symbols_non_terminal_last() -> () {
            #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
            enum DummyTerminalTokenTypeTest {
                A,
                Na,
                Ns,
                S,
            }

            #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
            enum DummySyntaxTokenTest {
                Epsilon,
                A,
                B,
                C,
                S,
                Terminal(DummyTerminalTokenTypeTest),
            }

            let grammar_productions: Vec<ContextFreeGrammarProduction<DummySyntaxTokenTest>> = vec![
                ContextFreeGrammarProduction::new(
                    DummySyntaxTokenTest::S,
                    vec![DummySyntaxTokenTest::A],
                ),
                ContextFreeGrammarProduction::new(
                    DummySyntaxTokenTest::B,
                    vec![
                        DummySyntaxTokenTest::A,
                        DummySyntaxTokenTest::Terminal(DummyTerminalTokenTypeTest::Na),
                    ],
                ),
                ContextFreeGrammarProduction::new(
                    DummySyntaxTokenTest::C,
                    vec![
                        DummySyntaxTokenTest::S,
                        DummySyntaxTokenTest::Terminal(DummyTerminalTokenTypeTest::Ns),
                    ],
                ),
                ContextFreeGrammarProduction::new(
                    DummySyntaxTokenTest::A,
                    vec![DummySyntaxTokenTest::Terminal(
                        DummyTerminalTokenTypeTest::A,
                    )],
                ),
                ContextFreeGrammarProduction::new(
                    DummySyntaxTokenTest::S,
                    vec![DummySyntaxTokenTest::Terminal(
                        DummyTerminalTokenTypeTest::S,
                    )],
                ),
            ];

            let grammar: ContextFreeGrammar<DummySyntaxTokenTest> = ContextFreeGrammar::new(
                DummySyntaxTokenTest::Epsilon,
                DummySyntaxTokenTest::S,
                grammar_productions,
            );

            let first_follow_symbols: FirstFollowSymbols<DummySyntaxTokenTest> =
                FirstFollowSymbols::from(&grammar);

            let a_follow_symbols = first_follow_symbols
                .get_follow_symbols(&DummySyntaxTokenTest::A)
                .unwrap();

            assert_eq!(a_follow_symbols.len(), 2);
            assert!(a_follow_symbols.contains(&DummySyntaxTokenTest::Terminal(
                DummyTerminalTokenTypeTest::Na
            )));
            assert!(a_follow_symbols.contains(&DummySyntaxTokenTest::Terminal(
                DummyTerminalTokenTypeTest::Ns
            )));

            let s_follow_symbols = first_follow_symbols
                .get_follow_symbols(&DummySyntaxTokenTest::S)
                .unwrap();

            assert_eq!(s_follow_symbols.len(), 1);
            assert!(s_follow_symbols.contains(&DummySyntaxTokenTest::Terminal(
                DummyTerminalTokenTypeTest::Ns
            )));
        }

        #[test]
        fn it_returns_follow_symbols_non_terminal_last_epsilon_chain() -> () {
            #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
            enum DummyTerminalTokenTypeTest {
                S,
            }

            #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
            enum DummySyntaxTokenTest {
                Epsilon,
                A,
                B,
                R,
                S,
                Terminal(DummyTerminalTokenTypeTest),
            }

            let grammar_productions: Vec<ContextFreeGrammarProduction<DummySyntaxTokenTest>> = vec![
                ContextFreeGrammarProduction::new(
                    DummySyntaxTokenTest::S,
                    vec![DummySyntaxTokenTest::A, DummySyntaxTokenTest::B],
                ),
                ContextFreeGrammarProduction::new(
                    DummySyntaxTokenTest::A,
                    vec![DummySyntaxTokenTest::Epsilon],
                ),
                ContextFreeGrammarProduction::new(
                    DummySyntaxTokenTest::B,
                    vec![DummySyntaxTokenTest::Epsilon],
                ),
                ContextFreeGrammarProduction::new(
                    DummySyntaxTokenTest::R,
                    vec![
                        DummySyntaxTokenTest::S,
                        DummySyntaxTokenTest::Terminal(DummyTerminalTokenTypeTest::S),
                    ],
                ),
            ];

            let grammar: ContextFreeGrammar<DummySyntaxTokenTest> = ContextFreeGrammar::new(
                DummySyntaxTokenTest::Epsilon,
                DummySyntaxTokenTest::R,
                grammar_productions,
            );

            let first_follow_symbols: FirstFollowSymbols<DummySyntaxTokenTest> =
                FirstFollowSymbols::from(&grammar);

            let a_follow_symbols = first_follow_symbols
                .get_follow_symbols(&DummySyntaxTokenTest::A)
                .unwrap();

            assert_eq!(a_follow_symbols.len(), 1);
            assert!(a_follow_symbols.contains(&DummySyntaxTokenTest::Terminal(
                DummyTerminalTokenTypeTest::S
            )));
        }

        #[test]
        fn it_returns_follow_symbols_non_terminal_epsilon_chain() -> () {
            #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
            enum DummyTerminalTokenTypeTest {
                A,
                B,
                C,
            }

            #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
            enum DummySyntaxTokenTest {
                Epsilon,
                A,
                B,
                C,
                S,
                Terminal(DummyTerminalTokenTypeTest),
            }

            let grammar_productions: Vec<ContextFreeGrammarProduction<DummySyntaxTokenTest>> = vec![
                ContextFreeGrammarProduction::new(
                    DummySyntaxTokenTest::S,
                    vec![
                        DummySyntaxTokenTest::A,
                        DummySyntaxTokenTest::B,
                        DummySyntaxTokenTest::C,
                    ],
                ),
                ContextFreeGrammarProduction::new(
                    DummySyntaxTokenTest::A,
                    vec![DummySyntaxTokenTest::Terminal(
                        DummyTerminalTokenTypeTest::A,
                    )],
                ),
                ContextFreeGrammarProduction::new(
                    DummySyntaxTokenTest::B,
                    vec![DummySyntaxTokenTest::Epsilon],
                ),
                ContextFreeGrammarProduction::new(
                    DummySyntaxTokenTest::B,
                    vec![DummySyntaxTokenTest::Terminal(
                        DummyTerminalTokenTypeTest::B,
                    )],
                ),
                ContextFreeGrammarProduction::new(
                    DummySyntaxTokenTest::C,
                    vec![DummySyntaxTokenTest::Terminal(
                        DummyTerminalTokenTypeTest::C,
                    )],
                ),
            ];

            let grammar: ContextFreeGrammar<DummySyntaxTokenTest> = ContextFreeGrammar::new(
                DummySyntaxTokenTest::Epsilon,
                DummySyntaxTokenTest::S,
                grammar_productions,
            );

            let first_follow_symbols: FirstFollowSymbols<DummySyntaxTokenTest> =
                FirstFollowSymbols::from(&grammar);

            let a_follow_symbols = first_follow_symbols
                .get_follow_symbols(&DummySyntaxTokenTest::A)
                .unwrap();

            assert_eq!(a_follow_symbols.len(), 2);
            assert!(a_follow_symbols.contains(&DummySyntaxTokenTest::Terminal(
                DummyTerminalTokenTypeTest::B
            )));
            assert!(a_follow_symbols.contains(&DummySyntaxTokenTest::Terminal(
                DummyTerminalTokenTypeTest::C
            )));
        }
    }
}
