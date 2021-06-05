#[cfg(test)]
mod test {
    mod parse_from_tokens {
        use crate::grammar::context_free_grammar_production::ContextFreeGrammarProduction;
        use crate::grammar::context_free_grammar::ContextFreeGrammar;
        use crate::parser::recursive_descent_parser::RecursiveDescentParser;
        use crate::ast::abstract_syntax_tree::AbstractSyntaxTree;
        use crate::ast::abstract_syntax_node::AbstractSyntaxNode;

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

            let ast =
                recursive_descent_parser
                    .parse_from_tokens(vec![].into_iter());

            assert!(ast.is_some());
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
                    .parse_from_tokens(vec![&DummySyntaxTokenTest::Eof].into_iter())
                    .unwrap();

            let expected_asn = AbstractSyntaxNode::new(
                vec![
                    AbstractSyntaxNode::new(
                        vec![],
                        DummySyntaxTokenTest::Eof
                    ),
                ],
                DummySyntaxTokenTest::S
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
                    .parse_from_tokens(vec![&DummySyntaxTokenTest::Eof].into_iter())
                    .unwrap();

            let expected_asn = AbstractSyntaxNode::new(
                vec![
                    AbstractSyntaxNode::new(
                        vec![
                            AbstractSyntaxNode::new(
                                vec![],
                                DummySyntaxTokenTest::Epsilon
                            ),
                        ],
                        DummySyntaxTokenTest::Empty
                    ),
                    AbstractSyntaxNode::new(
                        vec![],
                        DummySyntaxTokenTest::Eof
                    ),
                ],
                DummySyntaxTokenTest::S
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
                            &DummySyntaxTokenTest::CommonTerminal,
                            &DummySyntaxTokenTest::ATerminal,
                            &DummySyntaxTokenTest::Eof,
                        ].into_iter()
                    )
                    .unwrap();

            let a_expected_asn = AbstractSyntaxNode::new(
                vec![
                    AbstractSyntaxNode::new(
                        vec![
                            AbstractSyntaxNode::new(
                                vec![],
                                DummySyntaxTokenTest::CommonTerminal
                            ),
                            AbstractSyntaxNode::new(
                                vec![],
                                DummySyntaxTokenTest::ATerminal
                            ),
                        ],
                        DummySyntaxTokenTest::A
                    ),
                    AbstractSyntaxNode::new(
                        vec![],
                        DummySyntaxTokenTest::Eof
                    ),
                ],
                DummySyntaxTokenTest::S
            );

            let a_expected_ast = AbstractSyntaxTree::new(a_expected_asn);

            let b_ast =
                recursive_descent_parser
                    .parse_from_tokens(
                        vec![
                            &DummySyntaxTokenTest::CommonTerminal,
                            &DummySyntaxTokenTest::BTerminal,
                            &DummySyntaxTokenTest::Eof,
                        ].into_iter()
                    )
                    .unwrap();

            let b_expected_asn = AbstractSyntaxNode::new(
                vec![
                    AbstractSyntaxNode::new(
                        vec![
                            AbstractSyntaxNode::new(
                                vec![],
                                DummySyntaxTokenTest::CommonTerminal
                            ),
                            AbstractSyntaxNode::new(
                                vec![],
                                DummySyntaxTokenTest::BTerminal
                            ),
                        ],
                        DummySyntaxTokenTest::B
                    ),
                    AbstractSyntaxNode::new(
                        vec![],
                        DummySyntaxTokenTest::Eof
                    ),
                ],
                DummySyntaxTokenTest::S
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
                            &DummySyntaxTokenTest::ATerminal,
                            &DummySyntaxTokenTest::BTerminal,
                            &DummySyntaxTokenTest::CTerminal,
                            &DummySyntaxTokenTest::Eof,
                        ].into_iter()
                    )
                    .unwrap();

            let abc_expected_asn = AbstractSyntaxNode::new(
                vec![
                    AbstractSyntaxNode::new(
                        vec![
                            AbstractSyntaxNode::new(
                                vec![],
                                DummySyntaxTokenTest::ATerminal
                            ),
                            AbstractSyntaxNode::new(
                                vec![],
                                DummySyntaxTokenTest::BTerminal
                            ),
                        ],
                        DummySyntaxTokenTest::A
                    ),
                    AbstractSyntaxNode::new(
                        vec![
                            AbstractSyntaxNode::new(
                                vec![],
                                DummySyntaxTokenTest::CTerminal
                            ),
                        ],
                        DummySyntaxTokenTest::B
                    ),
                    AbstractSyntaxNode::new(
                        vec![],
                        DummySyntaxTokenTest::Eof
                    ),
                ],
                DummySyntaxTokenTest::S
            );

            let abc_expected_ast = AbstractSyntaxTree::new(abc_expected_asn);

            let abcd_ast =
                recursive_descent_parser
                    .parse_from_tokens(
                        vec![
                            &DummySyntaxTokenTest::ATerminal,
                            &DummySyntaxTokenTest::BTerminal,
                            &DummySyntaxTokenTest::CTerminal,
                            &DummySyntaxTokenTest::DTerminal,
                            &DummySyntaxTokenTest::Eof,
                        ].into_iter()
                    )
                    .unwrap();

            let abcd_expected_asn = AbstractSyntaxNode::new(
                vec![
                    AbstractSyntaxNode::new(
                        vec![
                            AbstractSyntaxNode::new(
                                vec![],
                                DummySyntaxTokenTest::ATerminal
                            ),
                        ],
                        DummySyntaxTokenTest::A
                    ),
                    AbstractSyntaxNode::new(
                        vec![
                            AbstractSyntaxNode::new(
                                vec![],
                                DummySyntaxTokenTest::BTerminal
                            ),
                            AbstractSyntaxNode::new(
                                vec![],
                                DummySyntaxTokenTest::CTerminal
                            ),
                            AbstractSyntaxNode::new(
                                vec![],
                                DummySyntaxTokenTest::DTerminal
                            ),
                        ],
                        DummySyntaxTokenTest::B
                    ),
                    AbstractSyntaxNode::new(
                        vec![],
                        DummySyntaxTokenTest::Eof
                    ),
                ],
                DummySyntaxTokenTest::S
            );

            let abcd_expected_ast = AbstractSyntaxTree::new(abcd_expected_asn);

            assert!(ast_equals(&abc_ast, &abc_expected_ast));
            assert!(ast_equals(&abcd_ast, &abcd_expected_ast));
        }
    }
}