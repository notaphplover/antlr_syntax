#[cfg(test)]
mod test {
    mod parse_from_tokens {
        use crate::ast::abstract_syntax_tree::AbstractSyntaxTree;
        use crate::ast::abstract_syntax_node::AbstractSyntaxNode;
        use crate::grammar::context_free_grammar_production::ContextFreeGrammarProduction;
        use crate::grammar::context_free_grammar::ContextFreeGrammar;
        use crate::parser::failed_production::FailedProduction;
        use crate::parser::failed_state::FailedState;
        use crate::parser::recursive_descent_parser::RecursiveDescentParser;
        use crate::token::token::Token;

        fn ast_equals<T: PartialEq>(
            first_ast: &AbstractSyntaxTree<T>,
            second_ast: &AbstractSyntaxTree<T>,
        ) -> bool {
            asn_equals(&first_ast.root, &second_ast.root)
        }

        fn asn_equals<T: PartialEq>(
            first_asn: &AbstractSyntaxNode<T>,
            second_asn: &AbstractSyntaxNode<T>,
        ) -> bool {
            if first_asn.token.ne(&second_asn.token) {
                return false;
            }

            if first_asn.child_nodes.len().ne(&second_asn.child_nodes.len()) {
                return false;
            }

            for i in 0.. first_asn.child_nodes.len() {
                if !asn_equals(
                    first_asn.child_nodes.get(i).unwrap(),
                    second_asn.child_nodes.get(i).unwrap(),
                ) {
                    return false;
                }
            }

            true
        }

        fn failed_states_equals<TLex: PartialEq, TSyntax: PartialEq>(
            first_failed_state: &FailedState<TLex, TSyntax>,
            second_failed_state: &FailedState<TLex, TSyntax>,
        ) -> bool {
            if first_failed_state.symbol_to_derive.ne(&second_failed_state.symbol_to_derive) {
                return false;
            }

            if first_failed_state.failed_productions.len() != second_failed_state.failed_productions.len() {
                return false;
            }

            for i in 0.. first_failed_state.failed_productions.len() {
                if !failed_productions_equals(
                    first_failed_state.failed_productions.get(i).unwrap(),
                    second_failed_state.failed_productions.get(i).unwrap(),
                ) {
                    return false;
                }
            }

            true
        }

        fn failed_productions_equals<TLex: PartialEq, TSyntax: PartialEq>(
            first_failed_production: &FailedProduction<TLex, TSyntax>,
            second_failed_production: &FailedProduction<TLex, TSyntax>,
        ) -> bool {
            if first_failed_production.parsed_symbols.len() != second_failed_production.parsed_symbols.len() {
                return false;
            }

            if first_failed_production.pending_symbols.len() != second_failed_production.pending_symbols.len() {
                return false;
            }

            for i in 0..first_failed_production.parsed_symbols.len() {
                if !asn_equals(
                    first_failed_production.parsed_symbols.get(i).unwrap(),
                    second_failed_production.parsed_symbols.get(i).unwrap(),
                ) {
                    return false;
                }
            }

            if !failed_states_equals(
                &first_failed_production.failed_symbol,
                &second_failed_production.failed_symbol
            ) {
                return false;
            }

            for i in 0..first_failed_production.pending_symbols.len() {
                if first_failed_production
                    .pending_symbols
                    .get(i)
                    .unwrap()
                    .ne(
                        second_failed_production
                            .pending_symbols
                            .get(i)
                            .unwrap()
                    ) {
                    return false;
                }
            }

            true
        }

        #[allow(unused_must_use)]
        #[test]
        #[should_panic]
        fn it_does_not_parse_empty_input() -> () {
            #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
            enum DummySyntaxTokenTest {
                Epsilon,
                S,
            }

            let grammar_productions: Vec<ContextFreeGrammarProduction<DummySyntaxTokenTest>> = vec![
                ContextFreeGrammarProduction::new(DummySyntaxTokenTest::S, vec![
                    DummySyntaxTokenTest::Epsilon,
                ]),
            ];

            let grammar: ContextFreeGrammar<DummySyntaxTokenTest> = ContextFreeGrammar::new(
                DummySyntaxTokenTest::Epsilon,
                DummySyntaxTokenTest::S,
                grammar_productions,
            );

            let recursive_descent_parser: RecursiveDescentParser<DummySyntaxTokenTest>
                = RecursiveDescentParser::from_grammar(&grammar);

            recursive_descent_parser
                .parse_from_tokens::<u64, std::vec::IntoIter<Token<u64, DummySyntaxTokenTest>>>(
                    vec![].into_iter(),
                );
        }

        #[test]
        fn it_parses_terminal_production() -> () {
            #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
            enum DummySyntaxTokenTest {
                Eof,
                Epsilon,
                S,
            }

            let grammar_productions: Vec<ContextFreeGrammarProduction<DummySyntaxTokenTest>> = vec![
                ContextFreeGrammarProduction::new(DummySyntaxTokenTest::S, vec![
                    DummySyntaxTokenTest::Eof,
                ]),
            ];

            let grammar: ContextFreeGrammar<DummySyntaxTokenTest> = ContextFreeGrammar::new(
                DummySyntaxTokenTest::Epsilon,
                DummySyntaxTokenTest::S,
                grammar_productions,
            );

            let recursive_descent_parser: RecursiveDescentParser<DummySyntaxTokenTest>
                = RecursiveDescentParser::from_grammar(&grammar);

            let ast =
                recursive_descent_parser
                    .parse_from_tokens(vec![
                        Token::new(Some(0u64), DummySyntaxTokenTest::Eof),
                    ].into_iter())
                    .ok().unwrap();

            let expected_asn = AbstractSyntaxNode::new(
                vec![
                    AbstractSyntaxNode::new(
                        vec![],
                        Token::new(Some(0u64), DummySyntaxTokenTest::Eof),
                    ),
                ],
                Token::new(None, DummySyntaxTokenTest::S),
            );

            let expected_ast = AbstractSyntaxTree::new(expected_asn);

            assert!(ast_equals(&ast, &expected_ast));
        }

        #[test]
        fn it_parses_epsilon_production() -> () {
            #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
            enum DummySyntaxTokenTest {
                Empty,
                Eof,
                Epsilon,
                S,
            }

            let grammar_productions: Vec<ContextFreeGrammarProduction<DummySyntaxTokenTest>> = vec![
                ContextFreeGrammarProduction::new(DummySyntaxTokenTest::S, vec![
                    DummySyntaxTokenTest::Empty,
                    DummySyntaxTokenTest::Eof,
                ]),
                ContextFreeGrammarProduction::new(DummySyntaxTokenTest::Empty, vec![
                    DummySyntaxTokenTest::Epsilon,
                ]),
            ];

            let grammar: ContextFreeGrammar<DummySyntaxTokenTest> = ContextFreeGrammar::new(
                DummySyntaxTokenTest::Epsilon,
                DummySyntaxTokenTest::S,
                grammar_productions,
            );

            let recursive_descent_parser: RecursiveDescentParser<DummySyntaxTokenTest>
                = RecursiveDescentParser::from_grammar(&grammar);

            let ast =
                recursive_descent_parser
                    .parse_from_tokens(vec![
                        Token::new(Some(0u64), DummySyntaxTokenTest::Eof)
                    ].into_iter())
                    .ok().unwrap();

            let expected_asn = AbstractSyntaxNode::new(
                vec![
                    AbstractSyntaxNode::new(
                        vec![
                            AbstractSyntaxNode::new(
                                vec![],
                                Token::new(None, DummySyntaxTokenTest::Epsilon)
                            ),
                        ],
                        Token::new(None, DummySyntaxTokenTest::Empty)
                    ),
                    AbstractSyntaxNode::new(
                        vec![],
                        Token::new(Some(0u64), DummySyntaxTokenTest::Eof)
                    ),
                ],
                Token::new(None, DummySyntaxTokenTest::S)
            );

            let expected_ast = AbstractSyntaxTree::new(expected_asn);

            assert!(ast_equals(&ast, &expected_ast));
        }

        #[test]
        fn it_parses_with_first_first_conflict() -> () {
            #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
            enum DummySyntaxTokenTest {
                A,
                ATerminal,
                B,
                BTerminal,
                CommonTerminal,
                Eof,
                Epsilon,
                S,
            }

            let grammar_productions: Vec<ContextFreeGrammarProduction<DummySyntaxTokenTest>> = vec![
                ContextFreeGrammarProduction::new(DummySyntaxTokenTest::S, vec![
                    DummySyntaxTokenTest::A,
                    DummySyntaxTokenTest::Eof,
                ]),
                ContextFreeGrammarProduction::new(DummySyntaxTokenTest::S, vec![
                    DummySyntaxTokenTest::B,
                    DummySyntaxTokenTest::Eof,
                ]),
                ContextFreeGrammarProduction::new(DummySyntaxTokenTest::A, vec![
                    DummySyntaxTokenTest::CommonTerminal,
                    DummySyntaxTokenTest::ATerminal,
                ]),
                ContextFreeGrammarProduction::new(DummySyntaxTokenTest::B, vec![
                    DummySyntaxTokenTest::CommonTerminal,
                    DummySyntaxTokenTest::BTerminal,
                ]),
            ];

            let grammar: ContextFreeGrammar<DummySyntaxTokenTest> = ContextFreeGrammar::new(
                DummySyntaxTokenTest::Epsilon,
                DummySyntaxTokenTest::S,
                grammar_productions,
            );

            let recursive_descent_parser: RecursiveDescentParser<DummySyntaxTokenTest>
                = RecursiveDescentParser::from_grammar(&grammar);

            let a_ast =
                recursive_descent_parser
                    .parse_from_tokens(
                        vec![
                            Token::new(Some(0u64), DummySyntaxTokenTest::CommonTerminal),
                            Token::new(Some(0u64), DummySyntaxTokenTest::ATerminal),
                            Token::new(Some(0u64), DummySyntaxTokenTest::Eof),
                        ].into_iter()
                    )
                    .ok().unwrap();

            let a_expected_asn = AbstractSyntaxNode::new(
                vec![
                    AbstractSyntaxNode::new(
                        vec![
                            AbstractSyntaxNode::new(
                                vec![],
                                Token::new(Some(0u64), DummySyntaxTokenTest::CommonTerminal),
                            ),
                            AbstractSyntaxNode::new(
                                vec![],
                                Token::new(Some(0u64), DummySyntaxTokenTest::ATerminal),
                            ),
                        ],
                        Token::new(None, DummySyntaxTokenTest::A),
                    ),
                    AbstractSyntaxNode::new(
                        vec![],
                        Token::new(Some(0u64), DummySyntaxTokenTest::Eof),
                    ),
                ],
                Token::new(None, DummySyntaxTokenTest::S)
            );

            let a_expected_ast = AbstractSyntaxTree::new(a_expected_asn);

            let b_ast =
                recursive_descent_parser
                    .parse_from_tokens(
                        vec![
                            Token::new(Some(0u64), DummySyntaxTokenTest::CommonTerminal),
                            Token::new(Some(0u64), DummySyntaxTokenTest::BTerminal),
                            Token::new(Some(0u64), DummySyntaxTokenTest::Eof),
                        ].into_iter()
                    )
                    .ok().unwrap();

            let b_expected_asn = AbstractSyntaxNode::new(
                vec![
                    AbstractSyntaxNode::new(
                        vec![
                            AbstractSyntaxNode::new(
                                vec![],
                                Token::new(Some(0u64), DummySyntaxTokenTest::CommonTerminal)
                            ),
                            AbstractSyntaxNode::new(
                                vec![],
                                Token::new(None, DummySyntaxTokenTest::BTerminal)
                            ),
                        ],
                        Token::new(None, DummySyntaxTokenTest::B)
                    ),
                    AbstractSyntaxNode::new(
                        vec![],
                        Token::new(Some(0u64), DummySyntaxTokenTest::Eof)
                    ),
                ],
                Token::new(None, DummySyntaxTokenTest::S)
            );

            let b_expected_ast = AbstractSyntaxTree::new(b_expected_asn);

            assert!(ast_equals(&a_ast, &a_expected_ast));
            assert!(ast_equals(&b_ast, &b_expected_ast));
        }

        #[test]
        fn it_parses_with_production_backtracking() -> () {
            #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
            enum DummySyntaxTokenTest {
                A,
                ATerminal,
                B,
                BTerminal,
                CTerminal,
                DTerminal,
                Eof,
                Epsilon,
                S,
            }

            let grammar_productions: Vec<ContextFreeGrammarProduction<DummySyntaxTokenTest>> = vec![
                ContextFreeGrammarProduction::new(DummySyntaxTokenTest::S, vec![
                    DummySyntaxTokenTest::A,
                    DummySyntaxTokenTest::B,
                    DummySyntaxTokenTest::Eof,
                ]),
                ContextFreeGrammarProduction::new(DummySyntaxTokenTest::A, vec![
                    DummySyntaxTokenTest::ATerminal,
                    DummySyntaxTokenTest::BTerminal,
                ]),
                ContextFreeGrammarProduction::new(DummySyntaxTokenTest::A, vec![
                    DummySyntaxTokenTest::ATerminal,
                ]),
                ContextFreeGrammarProduction::new(DummySyntaxTokenTest::B, vec![
                    DummySyntaxTokenTest::BTerminal,
                    DummySyntaxTokenTest::CTerminal,
                    DummySyntaxTokenTest::DTerminal,
                ]),
                ContextFreeGrammarProduction::new(DummySyntaxTokenTest::B, vec![
                    DummySyntaxTokenTest::CTerminal,
                ]),
            ];

            let grammar: ContextFreeGrammar<DummySyntaxTokenTest> = ContextFreeGrammar::new(
                DummySyntaxTokenTest::Epsilon,
                DummySyntaxTokenTest::S,
                grammar_productions,
            );

            let recursive_descent_parser: RecursiveDescentParser<DummySyntaxTokenTest>
                = RecursiveDescentParser::from_grammar(&grammar);

            let abc_ast =
                recursive_descent_parser
                    .parse_from_tokens(
                        vec![
                            Token::new(Some(0u64), DummySyntaxTokenTest::ATerminal),
                            Token::new(Some(0u64), DummySyntaxTokenTest::BTerminal),
                            Token::new(Some(0u64), DummySyntaxTokenTest::CTerminal),
                            Token::new(Some(0u64), DummySyntaxTokenTest::Eof),
                        ].into_iter()
                    )
                    .ok().unwrap();

            let abc_expected_asn = AbstractSyntaxNode::new(
                vec![
                    AbstractSyntaxNode::new(
                        vec![
                            AbstractSyntaxNode::new(
                                vec![],
                                Token::new(Some(0u64), DummySyntaxTokenTest::ATerminal),
                            ),
                            AbstractSyntaxNode::new(
                                vec![],
                                Token::new(Some(0u64), DummySyntaxTokenTest::BTerminal),
                            ),
                        ],
                        Token::new(None, DummySyntaxTokenTest::A),
                    ),
                    AbstractSyntaxNode::new(
                        vec![
                            AbstractSyntaxNode::new(
                                vec![],
                                Token::new(Some(0u64), DummySyntaxTokenTest::CTerminal),
                            ),
                        ],
                        Token::new(None, DummySyntaxTokenTest::B)
                    ),
                    AbstractSyntaxNode::new(
                        vec![],
                        Token::new(None, DummySyntaxTokenTest::Eof)
                    ),
                ],
                Token::new(None, DummySyntaxTokenTest::S)
            );

            let abc_expected_ast = AbstractSyntaxTree::new(abc_expected_asn);

            let abcd_ast =
                recursive_descent_parser
                    .parse_from_tokens(
                        vec![
                            Token::new(Some(0u64), DummySyntaxTokenTest::ATerminal),
                            Token::new(Some(0u64), DummySyntaxTokenTest::BTerminal),
                            Token::new(Some(0u64), DummySyntaxTokenTest::CTerminal),
                            Token::new(Some(0u64), DummySyntaxTokenTest::DTerminal),
                            Token::new(Some(0u64), DummySyntaxTokenTest::Eof),
                        ].into_iter()
                    )
                    .ok().unwrap();

            let abcd_expected_asn = AbstractSyntaxNode::new(
                vec![
                    AbstractSyntaxNode::new(
                        vec![
                            AbstractSyntaxNode::new(
                                vec![],
                                Token::new(Some(0u64), DummySyntaxTokenTest::ATerminal)
                            ),
                        ],
                        Token::new(None, DummySyntaxTokenTest::A)
                    ),
                    AbstractSyntaxNode::new(
                        vec![
                            AbstractSyntaxNode::new(
                                vec![],
                                Token::new(Some(0u64), DummySyntaxTokenTest::BTerminal)
                            ),
                            AbstractSyntaxNode::new(
                                vec![],
                                Token::new(Some(0u64), DummySyntaxTokenTest::CTerminal)
                            ),
                            AbstractSyntaxNode::new(
                                vec![],
                                Token::new(Some(0u64), DummySyntaxTokenTest::DTerminal)
                            ),
                        ],
                        Token::new(None, DummySyntaxTokenTest::B)
                    ),
                    AbstractSyntaxNode::new(
                        vec![],
                        Token::new(Some(0u64), DummySyntaxTokenTest::Eof)
                    ),
                ],
                Token::new(None, DummySyntaxTokenTest::S)
            );

            let abcd_expected_ast = AbstractSyntaxTree::new(abcd_expected_asn);

            assert!(ast_equals(&abc_ast, &abc_expected_ast));
            assert!(ast_equals(&abcd_ast, &abcd_expected_ast));
        }

        #[test]
        fn it_fails_on_unexpected_symbol_when_no_production_is_found() -> () {
            #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
            enum DummySyntaxTokenTest {
                ATerminal,
                BTerminal,
                Epsilon,
                S,
            }

            let grammar_productions: Vec<ContextFreeGrammarProduction<DummySyntaxTokenTest>> = vec![
                ContextFreeGrammarProduction::new(DummySyntaxTokenTest::S, vec![
                    DummySyntaxTokenTest::ATerminal,
                ]),
            ];

            let grammar: ContextFreeGrammar<DummySyntaxTokenTest> = ContextFreeGrammar::new(
                DummySyntaxTokenTest::Epsilon,
                DummySyntaxTokenTest::S,
                grammar_productions,
            );

            let recursive_descent_parser: RecursiveDescentParser<DummySyntaxTokenTest>
                = RecursiveDescentParser::from_grammar(&grammar);

            let failed_state =
                recursive_descent_parser
                    .parse_from_tokens(
                        vec![
                            Token::new(Some(0u64), DummySyntaxTokenTest::BTerminal),
                        ].into_iter()
                    )
                    .err().unwrap();

            let expected_failed_state = <FailedState<u64, DummySyntaxTokenTest>>::new(
                vec![],
                DummySyntaxTokenTest::S,
            );

            assert!(failed_states_equals(&failed_state, &expected_failed_state));
        }

        #[test]
        fn it_fails_on_unexpected_symbol_when_a_production_is_found() -> () {
            #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
            enum DummySyntaxTokenTest {
                ATerminal,
                BTerminal,
                Epsilon,
                S,
            }

            let grammar_productions: Vec<ContextFreeGrammarProduction<DummySyntaxTokenTest>> = vec![
                ContextFreeGrammarProduction::new(DummySyntaxTokenTest::S, vec![
                    DummySyntaxTokenTest::ATerminal,
                    DummySyntaxTokenTest::ATerminal,
                ]),
            ];

            let grammar: ContextFreeGrammar<DummySyntaxTokenTest> = ContextFreeGrammar::new(
                DummySyntaxTokenTest::Epsilon,
                DummySyntaxTokenTest::S,
                grammar_productions,
            );

            let recursive_descent_parser: RecursiveDescentParser<DummySyntaxTokenTest>
                = RecursiveDescentParser::from_grammar(&grammar);

            let failed_state =
                recursive_descent_parser
                    .parse_from_tokens(
                        vec![
                            Token::new(Some(0u64), DummySyntaxTokenTest::ATerminal),
                            Token::new(Some(0u64), DummySyntaxTokenTest::BTerminal),
                        ].into_iter()
                    )
                    .err().unwrap();

            let expected_failed_state = <FailedState<u64, DummySyntaxTokenTest>>::new(
                vec![
                    FailedProduction::new(
                        FailedState::new(vec![], DummySyntaxTokenTest::ATerminal),
                        vec![
                            AbstractSyntaxNode::new(
                                vec![],
                                Token::new(Some(0u64), DummySyntaxTokenTest::ATerminal),
                            ),
                        ], 
                        vec![],
                    ),
                ],
                DummySyntaxTokenTest::S,
            );

            assert!(failed_states_equals(&failed_state, &expected_failed_state));
        }

        #[test]
        fn it_fails_on_unexpected_symbol_when_a_production_is_found_with_parsed_symbols_ordered() -> () {
            #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
            enum DummySyntaxTokenTest {
                ATerminal,
                BTerminal,
                Epsilon,
                S,
            }

            let grammar_productions: Vec<ContextFreeGrammarProduction<DummySyntaxTokenTest>> = vec![
                ContextFreeGrammarProduction::new(DummySyntaxTokenTest::S, vec![
                    DummySyntaxTokenTest::ATerminal,
                    DummySyntaxTokenTest::BTerminal,
                    DummySyntaxTokenTest::ATerminal,
                ]),
            ];

            let grammar: ContextFreeGrammar<DummySyntaxTokenTest> = ContextFreeGrammar::new(
                DummySyntaxTokenTest::Epsilon,
                DummySyntaxTokenTest::S,
                grammar_productions,
            );

            let recursive_descent_parser: RecursiveDescentParser<DummySyntaxTokenTest>
                = RecursiveDescentParser::from_grammar(&grammar);

            let failed_state =
                recursive_descent_parser
                    .parse_from_tokens(
                        vec![
                            Token::new(Some(0u64), DummySyntaxTokenTest::ATerminal),
                            Token::new(Some(0u64), DummySyntaxTokenTest::BTerminal),
                        ].into_iter()
                    )
                    .err().unwrap();

            let expected_failed_state = <FailedState<u64, DummySyntaxTokenTest>>::new(
                vec![
                    FailedProduction::new(
                        FailedState::new(vec![], DummySyntaxTokenTest::ATerminal),
                        vec![
                            AbstractSyntaxNode::new(
                                vec![],
                                Token::new(Some(0u64), DummySyntaxTokenTest::ATerminal),
                            ),
                            AbstractSyntaxNode::new(
                                vec![],
                                Token::new(Some(0u64), DummySyntaxTokenTest::BTerminal),
                            ),
                        ], 
                        vec![],
                    ),
                ],
                DummySyntaxTokenTest::S,
            );

            assert!(failed_states_equals(&failed_state, &expected_failed_state));
        }

        #[test]
        fn it_fails_on_unexpected_symbol_when_multiple_productions_are_found() -> () {
            #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
            enum DummySyntaxTokenTest {
                ATerminal,
                BTerminal,
                Epsilon,
                S,
            }

            let grammar_productions: Vec<ContextFreeGrammarProduction<DummySyntaxTokenTest>> = vec![
                ContextFreeGrammarProduction::new(DummySyntaxTokenTest::S, vec![
                    DummySyntaxTokenTest::ATerminal,
                    DummySyntaxTokenTest::ATerminal,
                ]),
                ContextFreeGrammarProduction::new(DummySyntaxTokenTest::S, vec![
                    DummySyntaxTokenTest::ATerminal,
                    DummySyntaxTokenTest::ATerminal,
                    DummySyntaxTokenTest::ATerminal,
                ]),
            ];

            let grammar: ContextFreeGrammar<DummySyntaxTokenTest> = ContextFreeGrammar::new(
                DummySyntaxTokenTest::Epsilon,
                DummySyntaxTokenTest::S,
                grammar_productions,
            );

            let recursive_descent_parser: RecursiveDescentParser<DummySyntaxTokenTest>
                = RecursiveDescentParser::from_grammar(&grammar);

            let failed_state =
                recursive_descent_parser
                    .parse_from_tokens(
                        vec![
                            Token::new(Some(0u64), DummySyntaxTokenTest::ATerminal),
                            Token::new(Some(0u64), DummySyntaxTokenTest::BTerminal),
                        ].into_iter()
                    )
                    .err().unwrap();

            let expected_failed_state = <FailedState<u64, DummySyntaxTokenTest>>::new(
                vec![
                    FailedProduction::new(
                        FailedState::new(vec![], DummySyntaxTokenTest::ATerminal),
                        vec![
                            AbstractSyntaxNode::new(
                                vec![],
                                Token::new(Some(0u64), DummySyntaxTokenTest::ATerminal),
                            ),
                        ], 
                        vec![],
                    ),
                    FailedProduction::new(
                        FailedState::new(vec![], DummySyntaxTokenTest::ATerminal),
                        vec![
                            AbstractSyntaxNode::new(
                                vec![],
                                Token::new(Some(0u64), DummySyntaxTokenTest::ATerminal),
                            ),
                        ], 
                        vec![DummySyntaxTokenTest::ATerminal],
                    ),
                ],
                DummySyntaxTokenTest::S,
            );

            assert!(failed_states_equals(&failed_state, &expected_failed_state));
        }
    }
}
