#[cfg(test)]
mod test {
    use crate::parser::failed_production::FailedProduction;
    use crate::parser::fix_result::FixResult;
    use crate::parser::syntax_error_solver::SyntaxErrorSolver;
    use crate::token::token::Token;
    use std::marker::PhantomData;

    struct SyntaxErrorSolverMock<TLex, TSyntax> {
        lex_marker: PhantomData<TLex>,
        syntax_marker: PhantomData<TSyntax>,
    }

    impl<TLex, TSyntax> SyntaxErrorSolver<TLex, TSyntax> for SyntaxErrorSolverMock<TLex, TSyntax> {
        fn fix_failed_production(
            &self,
            _tokens: &Vec<Token<TLex, TSyntax>>,
            _tokens_position: usize,
            _failed_production: &FailedProduction<TLex, TSyntax>,
        ) -> Option<FixResult<TLex, TSyntax>> {
            None
        }

        fn fix_failed_productions(
            &self,
            _tokens: &Vec<Token<TLex, TSyntax>>,
            _tokens_position: usize,
            _failed_productions: &Vec<FailedProduction<TLex, TSyntax>>,
        ) -> Option<FixResult<TLex, TSyntax>> {
            None
        }
    }

    mod parse_from_tokens {
        use crate::ast::abstract_syntax_node::AbstractSyntaxNode;
        use crate::ast::abstract_syntax_tree::AbstractSyntaxTree;
        use crate::grammar::context_free_grammar::ContextFreeGrammar;
        use crate::grammar::context_free_grammar_production::ContextFreeGrammarProduction;
        use crate::parser::failed_production::FailedProduction;
        use crate::parser::failed_symbol::FailedSymbol;
        use crate::parser::fix_gap::FixGap;
        use crate::parser::fixed_production::FixedProduction;
        use crate::parser::fixed_production_part::FixedProductionPart;
        use crate::parser::fixed_symbol::FixedSymbol;
        use crate::parser::parse_result::ParseResult;
        use crate::parser::production_parsed_symbol::ProductionParsedSymbol;
        use crate::parser::recursive_descent_parser::RecursiveDescentParser;
        use crate::parser::recursive_descent_parser_spec::test::SyntaxErrorSolverMock;
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

            if first_asn
                .child_nodes
                .len()
                .ne(&second_asn.child_nodes.len())
            {
                return false;
            }

            for i in 0..first_asn.child_nodes.len() {
                if !asn_equals(
                    first_asn.child_nodes.get(i).unwrap(),
                    second_asn.child_nodes.get(i).unwrap(),
                ) {
                    return false;
                }
            }

            true
        }

        fn failed_symbols_equals<TLex: PartialEq, TSyntax: PartialEq>(
            first_failed_symbol: &FailedSymbol<TLex, TSyntax>,
            second_failed_symbol: &FailedSymbol<TLex, TSyntax>,
        ) -> bool {
            if first_failed_symbol
                .symbol_to_derive
                .ne(&second_failed_symbol.symbol_to_derive)
            {
                return false;
            }

            if first_failed_symbol.failed_productions.len()
                != second_failed_symbol.failed_productions.len()
            {
                return false;
            }

            for i in 0..first_failed_symbol.failed_productions.len() {
                if !failed_productions_equals(
                    first_failed_symbol.failed_productions.get(i).unwrap(),
                    second_failed_symbol.failed_productions.get(i).unwrap(),
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
            if first_failed_production.parsed_symbols.len()
                != second_failed_production.parsed_symbols.len()
            {
                return false;
            }

            if first_failed_production.pending_symbols.len()
                != second_failed_production.pending_symbols.len()
            {
                return false;
            }

            for i in 0..first_failed_production.parsed_symbols.len() {
                if !production_parsed_symbol_equals(
                    first_failed_production.parsed_symbols.get(i).unwrap(),
                    second_failed_production.parsed_symbols.get(i).unwrap(),
                ) {
                    return false;
                }
            }

            if !failed_symbols_equals(
                &first_failed_production.failed_symbol,
                &second_failed_production.failed_symbol,
            ) {
                return false;
            }

            for i in 0..first_failed_production.pending_symbols.len() {
                if first_failed_production
                    .pending_symbols
                    .get(i)
                    .unwrap()
                    .ne(second_failed_production.pending_symbols.get(i).unwrap())
                {
                    return false;
                }
            }

            true
        }

        fn fix_gap_equals<TLex: PartialEq, TSyntax: PartialEq>(
            first_fix_gap: &FixGap<TLex, TSyntax>,
            second_fix_gap: &FixGap<TLex, TSyntax>,
        ) -> bool {
            if first_fix_gap.skipped_symbols.len() != second_fix_gap.skipped_symbols.len() {
                return false;
            }

            for i in 0..first_fix_gap.skipped_symbols.len() {
                let skipped_symbol_equals: bool = first_fix_gap.skipped_symbols.get(i).unwrap()
                    == second_fix_gap.skipped_symbols.get(i).unwrap();

                if !skipped_symbol_equals {
                    return false;
                }
            }

            if first_fix_gap.skipped_tokens.len() != second_fix_gap.skipped_tokens.len() {
                return false;
            }

            for i in 0..first_fix_gap.skipped_tokens.len() {
                let skipped_token_equals: bool = first_fix_gap.skipped_tokens.get(i).unwrap()
                    == second_fix_gap.skipped_tokens.get(i).unwrap();

                if !skipped_token_equals {
                    return false;
                }
            }

            true
        }

        fn fixed_production_equals<TLex: PartialEq, TSyntax: PartialEq>(
            first_fixed_production: &FixedProduction<TLex, TSyntax>,
            second_fixed_production: &FixedProduction<TLex, TSyntax>,
        ) -> bool {
            if first_fixed_production.fixed_parts.len() != second_fixed_production.fixed_parts.len()
            {
                return false;
            }

            for i in 0..first_fixed_production.fixed_parts.len() {
                if !fixed_production_part_equals(
                    first_fixed_production.fixed_parts.get(i).unwrap(),
                    second_fixed_production.fixed_parts.get(i).unwrap(),
                ) {
                    return false;
                }
            }

            true
        }

        fn fixed_production_part_equals<TLex: PartialEq, TSyntax: PartialEq>(
            first_fixed_production_part: &FixedProductionPart<TLex, TSyntax>,
            second_fixed_production_part: &FixedProductionPart<TLex, TSyntax>,
        ) -> bool {
            match (first_fixed_production_part, second_fixed_production_part) {
                (FixedProductionPart::Ok(first_asn), FixedProductionPart::Ok(second_asn)) => {
                    asn_equals(first_asn, second_asn)
                }
                (
                    FixedProductionPart::Fixed(first_fixed_symbol),
                    FixedProductionPart::Fixed(second_fixed_symbol),
                ) => fixed_symbol_equals(first_fixed_symbol, second_fixed_symbol),
                (
                    FixedProductionPart::Gap(first_fix_gap),
                    FixedProductionPart::Gap(second_fix_gap),
                ) => fix_gap_equals(first_fix_gap, second_fix_gap),
                _ => false,
            }
        }

        fn fixed_symbol_equals<TLex: PartialEq, TSyntax: PartialEq>(
            first_fixed_symbol: &FixedSymbol<TLex, TSyntax>,
            second_fixed_symbol: &FixedSymbol<TLex, TSyntax>,
        ) -> bool {
            if first_fixed_symbol
                .symbol_to_derive
                .ne(&second_fixed_symbol.symbol_to_derive)
            {
                return false;
            }

            fixed_production_equals(
                &first_fixed_symbol.fixed_production,
                &second_fixed_symbol.fixed_production,
            )
        }

        fn parse_result_equals<TLex: PartialEq, TSyntax: PartialEq>(
            first_parse_result: &ParseResult<TLex, TSyntax>,
            second_parse_result: &ParseResult<TLex, TSyntax>,
        ) -> bool {
            match (first_parse_result, second_parse_result) {
                (ParseResult::Ok(first_ast), ParseResult::Ok(second_ast)) => {
                    ast_equals(first_ast, second_ast)
                }
                (ParseResult::Fix(first_fixed_symbol), ParseResult::Fix(second_fixed_symbol)) => {
                    fixed_symbol_equals(first_fixed_symbol, second_fixed_symbol)
                }
                (ParseResult::Err(first_failed_symbol), ParseResult::Err(second_failed_symbol)) => {
                    failed_symbols_equals(first_failed_symbol, second_failed_symbol)
                }
                _ => false,
            }
        }

        fn production_parsed_symbol_equals<TLex: PartialEq, TSyntax: PartialEq>(
            first_production_parsed_symbol: &ProductionParsedSymbol<TLex, TSyntax>,
            second_production_parsed_symbol: &ProductionParsedSymbol<TLex, TSyntax>,
        ) -> bool {
            match (
                first_production_parsed_symbol,
                second_production_parsed_symbol,
            ) {
                (
                    ProductionParsedSymbol::Fix(first_fixed_symbol),
                    ProductionParsedSymbol::Fix(second_fixed_symbol),
                ) => fixed_symbol_equals(first_fixed_symbol, second_fixed_symbol),
                (ProductionParsedSymbol::Ok(first_asn), ProductionParsedSymbol::Ok(second_asn)) => {
                    asn_equals(first_asn, second_asn)
                }
                _ => false,
            }
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

            let grammar_productions: Vec<ContextFreeGrammarProduction<DummySyntaxTokenTest>> =
                vec![ContextFreeGrammarProduction::new(
                    DummySyntaxTokenTest::S,
                    vec![DummySyntaxTokenTest::Epsilon],
                )];

            let grammar: ContextFreeGrammar<DummySyntaxTokenTest> = ContextFreeGrammar::new(
                DummySyntaxTokenTest::Epsilon,
                DummySyntaxTokenTest::S,
                grammar_productions,
            );

            let recursive_descent_parser: RecursiveDescentParser<
                u64,
                DummySyntaxTokenTest,
                SyntaxErrorSolverMock<u64, DummySyntaxTokenTest>,
            > = RecursiveDescentParser::from_grammar(&grammar);

            recursive_descent_parser
                .parse_from_tokens::<std::vec::IntoIter<Token<u64, DummySyntaxTokenTest>>>(
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

            let grammar_productions: Vec<ContextFreeGrammarProduction<DummySyntaxTokenTest>> =
                vec![ContextFreeGrammarProduction::new(
                    DummySyntaxTokenTest::S,
                    vec![DummySyntaxTokenTest::Eof],
                )];

            let grammar: ContextFreeGrammar<DummySyntaxTokenTest> = ContextFreeGrammar::new(
                DummySyntaxTokenTest::Epsilon,
                DummySyntaxTokenTest::S,
                grammar_productions,
            );

            let recursive_descent_parser: RecursiveDescentParser<
                u64,
                DummySyntaxTokenTest,
                SyntaxErrorSolverMock<u64, DummySyntaxTokenTest>,
            > = RecursiveDescentParser::from_grammar(&grammar);

            let parse_result = recursive_descent_parser.parse_from_tokens(
                vec![Token::new(Some(0u64), DummySyntaxTokenTest::Eof)].into_iter(),
            );

            let expected_asn = AbstractSyntaxNode::new(
                vec![AbstractSyntaxNode::new(
                    vec![],
                    Token::new(Some(0u64), DummySyntaxTokenTest::Eof),
                )],
                Token::new(None, DummySyntaxTokenTest::S),
            );

            let expected_ast = AbstractSyntaxTree::new(expected_asn);

            let expected_parse_result = ParseResult::Ok(expected_ast);

            assert!(parse_result_equals(&parse_result, &expected_parse_result));
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

            let recursive_descent_parser: RecursiveDescentParser<
                u64,
                DummySyntaxTokenTest,
                SyntaxErrorSolverMock<u64, DummySyntaxTokenTest>,
            > = RecursiveDescentParser::from_grammar(&grammar);

            let parse_result = recursive_descent_parser.parse_from_tokens(
                vec![Token::new(Some(0u64), DummySyntaxTokenTest::Eof)].into_iter(),
            );

            let expected_asn = AbstractSyntaxNode::new(
                vec![
                    AbstractSyntaxNode::new(
                        vec![AbstractSyntaxNode::new(
                            vec![],
                            Token::new(None, DummySyntaxTokenTest::Epsilon),
                        )],
                        Token::new(None, DummySyntaxTokenTest::Empty),
                    ),
                    AbstractSyntaxNode::new(
                        vec![],
                        Token::new(Some(0u64), DummySyntaxTokenTest::Eof),
                    ),
                ],
                Token::new(None, DummySyntaxTokenTest::S),
            );

            let expected_ast = AbstractSyntaxTree::new(expected_asn);

            let expected_parse_result = ParseResult::Ok(expected_ast);

            assert!(parse_result_equals(&parse_result, &expected_parse_result,));
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
                ContextFreeGrammarProduction::new(
                    DummySyntaxTokenTest::S,
                    vec![DummySyntaxTokenTest::A, DummySyntaxTokenTest::Eof],
                ),
                ContextFreeGrammarProduction::new(
                    DummySyntaxTokenTest::S,
                    vec![DummySyntaxTokenTest::B, DummySyntaxTokenTest::Eof],
                ),
                ContextFreeGrammarProduction::new(
                    DummySyntaxTokenTest::A,
                    vec![
                        DummySyntaxTokenTest::CommonTerminal,
                        DummySyntaxTokenTest::ATerminal,
                    ],
                ),
                ContextFreeGrammarProduction::new(
                    DummySyntaxTokenTest::B,
                    vec![
                        DummySyntaxTokenTest::CommonTerminal,
                        DummySyntaxTokenTest::BTerminal,
                    ],
                ),
            ];

            let grammar: ContextFreeGrammar<DummySyntaxTokenTest> = ContextFreeGrammar::new(
                DummySyntaxTokenTest::Epsilon,
                DummySyntaxTokenTest::S,
                grammar_productions,
            );

            let recursive_descent_parser: RecursiveDescentParser<
                u64,
                DummySyntaxTokenTest,
                SyntaxErrorSolverMock<u64, DummySyntaxTokenTest>,
            > = RecursiveDescentParser::from_grammar(&grammar);

            let a_parse_result = recursive_descent_parser.parse_from_tokens(
                vec![
                    Token::new(Some(0u64), DummySyntaxTokenTest::CommonTerminal),
                    Token::new(Some(0u64), DummySyntaxTokenTest::ATerminal),
                    Token::new(Some(0u64), DummySyntaxTokenTest::Eof),
                ]
                .into_iter(),
            );

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
                Token::new(None, DummySyntaxTokenTest::S),
            );

            let a_expected_ast = AbstractSyntaxTree::new(a_expected_asn);

            let a_expected_parse_result = ParseResult::Ok(a_expected_ast);

            let b_parse_result = recursive_descent_parser.parse_from_tokens(
                vec![
                    Token::new(Some(0u64), DummySyntaxTokenTest::CommonTerminal),
                    Token::new(Some(0u64), DummySyntaxTokenTest::BTerminal),
                    Token::new(Some(0u64), DummySyntaxTokenTest::Eof),
                ]
                .into_iter(),
            );

            let b_expected_asn = AbstractSyntaxNode::new(
                vec![
                    AbstractSyntaxNode::new(
                        vec![
                            AbstractSyntaxNode::new(
                                vec![],
                                Token::new(Some(0u64), DummySyntaxTokenTest::CommonTerminal),
                            ),
                            AbstractSyntaxNode::new(
                                vec![],
                                Token::new(None, DummySyntaxTokenTest::BTerminal),
                            ),
                        ],
                        Token::new(None, DummySyntaxTokenTest::B),
                    ),
                    AbstractSyntaxNode::new(
                        vec![],
                        Token::new(Some(0u64), DummySyntaxTokenTest::Eof),
                    ),
                ],
                Token::new(None, DummySyntaxTokenTest::S),
            );

            let b_expected_ast = AbstractSyntaxTree::new(b_expected_asn);

            let b_expected_parse_result = ParseResult::Ok(b_expected_ast);

            assert!(parse_result_equals(
                &a_parse_result,
                &a_expected_parse_result,
            ));

            assert!(parse_result_equals(
                &b_parse_result,
                &b_expected_parse_result,
            ));
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
                ContextFreeGrammarProduction::new(
                    DummySyntaxTokenTest::S,
                    vec![
                        DummySyntaxTokenTest::A,
                        DummySyntaxTokenTest::B,
                        DummySyntaxTokenTest::Eof,
                    ],
                ),
                ContextFreeGrammarProduction::new(
                    DummySyntaxTokenTest::A,
                    vec![
                        DummySyntaxTokenTest::ATerminal,
                        DummySyntaxTokenTest::BTerminal,
                    ],
                ),
                ContextFreeGrammarProduction::new(
                    DummySyntaxTokenTest::A,
                    vec![DummySyntaxTokenTest::ATerminal],
                ),
                ContextFreeGrammarProduction::new(
                    DummySyntaxTokenTest::B,
                    vec![
                        DummySyntaxTokenTest::BTerminal,
                        DummySyntaxTokenTest::CTerminal,
                        DummySyntaxTokenTest::DTerminal,
                    ],
                ),
                ContextFreeGrammarProduction::new(
                    DummySyntaxTokenTest::B,
                    vec![DummySyntaxTokenTest::CTerminal],
                ),
            ];

            let grammar: ContextFreeGrammar<DummySyntaxTokenTest> = ContextFreeGrammar::new(
                DummySyntaxTokenTest::Epsilon,
                DummySyntaxTokenTest::S,
                grammar_productions,
            );

            let recursive_descent_parser: RecursiveDescentParser<
                u64,
                DummySyntaxTokenTest,
                SyntaxErrorSolverMock<u64, DummySyntaxTokenTest>,
            > = RecursiveDescentParser::from_grammar(&grammar);

            let abc_parse_result = recursive_descent_parser.parse_from_tokens(
                vec![
                    Token::new(Some(0u64), DummySyntaxTokenTest::ATerminal),
                    Token::new(Some(0u64), DummySyntaxTokenTest::BTerminal),
                    Token::new(Some(0u64), DummySyntaxTokenTest::CTerminal),
                    Token::new(Some(0u64), DummySyntaxTokenTest::Eof),
                ]
                .into_iter(),
            );

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
                        vec![AbstractSyntaxNode::new(
                            vec![],
                            Token::new(Some(0u64), DummySyntaxTokenTest::CTerminal),
                        )],
                        Token::new(None, DummySyntaxTokenTest::B),
                    ),
                    AbstractSyntaxNode::new(vec![], Token::new(None, DummySyntaxTokenTest::Eof)),
                ],
                Token::new(None, DummySyntaxTokenTest::S),
            );

            let abc_expected_ast = AbstractSyntaxTree::new(abc_expected_asn);

            let abc_expected_parse_result = ParseResult::Ok(abc_expected_ast);

            let abcd_parse_result = recursive_descent_parser.parse_from_tokens(
                vec![
                    Token::new(Some(0u64), DummySyntaxTokenTest::ATerminal),
                    Token::new(Some(0u64), DummySyntaxTokenTest::BTerminal),
                    Token::new(Some(0u64), DummySyntaxTokenTest::CTerminal),
                    Token::new(Some(0u64), DummySyntaxTokenTest::DTerminal),
                    Token::new(Some(0u64), DummySyntaxTokenTest::Eof),
                ]
                .into_iter(),
            );

            let abcd_expected_asn = AbstractSyntaxNode::new(
                vec![
                    AbstractSyntaxNode::new(
                        vec![AbstractSyntaxNode::new(
                            vec![],
                            Token::new(Some(0u64), DummySyntaxTokenTest::ATerminal),
                        )],
                        Token::new(None, DummySyntaxTokenTest::A),
                    ),
                    AbstractSyntaxNode::new(
                        vec![
                            AbstractSyntaxNode::new(
                                vec![],
                                Token::new(Some(0u64), DummySyntaxTokenTest::BTerminal),
                            ),
                            AbstractSyntaxNode::new(
                                vec![],
                                Token::new(Some(0u64), DummySyntaxTokenTest::CTerminal),
                            ),
                            AbstractSyntaxNode::new(
                                vec![],
                                Token::new(Some(0u64), DummySyntaxTokenTest::DTerminal),
                            ),
                        ],
                        Token::new(None, DummySyntaxTokenTest::B),
                    ),
                    AbstractSyntaxNode::new(
                        vec![],
                        Token::new(Some(0u64), DummySyntaxTokenTest::Eof),
                    ),
                ],
                Token::new(None, DummySyntaxTokenTest::S),
            );

            let abcd_expected_ast = AbstractSyntaxTree::new(abcd_expected_asn);

            let abcd_expected_parse_result = ParseResult::Ok(abcd_expected_ast);

            assert!(parse_result_equals(
                &abc_parse_result,
                &abc_expected_parse_result,
            ));
            assert!(parse_result_equals(
                &abcd_parse_result,
                &abcd_expected_parse_result,
            ));
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

            let grammar_productions: Vec<ContextFreeGrammarProduction<DummySyntaxTokenTest>> =
                vec![ContextFreeGrammarProduction::new(
                    DummySyntaxTokenTest::S,
                    vec![DummySyntaxTokenTest::ATerminal],
                )];

            let grammar: ContextFreeGrammar<DummySyntaxTokenTest> = ContextFreeGrammar::new(
                DummySyntaxTokenTest::Epsilon,
                DummySyntaxTokenTest::S,
                grammar_productions,
            );

            let recursive_descent_parser: RecursiveDescentParser<
                u64,
                DummySyntaxTokenTest,
                SyntaxErrorSolverMock<u64, DummySyntaxTokenTest>,
            > = RecursiveDescentParser::from_grammar(&grammar);

            let parse_result = recursive_descent_parser.parse_from_tokens(
                vec![Token::new(Some(0u64), DummySyntaxTokenTest::BTerminal)].into_iter(),
            );

            let expected_failed_symbol =
                <FailedSymbol<u64, DummySyntaxTokenTest>>::new(vec![], DummySyntaxTokenTest::S);

            let expected_parse_result = ParseResult::Err(expected_failed_symbol);

            assert!(parse_result_equals(&parse_result, &expected_parse_result,));
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

            let grammar_productions: Vec<ContextFreeGrammarProduction<DummySyntaxTokenTest>> =
                vec![ContextFreeGrammarProduction::new(
                    DummySyntaxTokenTest::S,
                    vec![
                        DummySyntaxTokenTest::ATerminal,
                        DummySyntaxTokenTest::ATerminal,
                    ],
                )];

            let grammar: ContextFreeGrammar<DummySyntaxTokenTest> = ContextFreeGrammar::new(
                DummySyntaxTokenTest::Epsilon,
                DummySyntaxTokenTest::S,
                grammar_productions,
            );

            let recursive_descent_parser: RecursiveDescentParser<
                u64,
                DummySyntaxTokenTest,
                SyntaxErrorSolverMock<u64, DummySyntaxTokenTest>,
            > = RecursiveDescentParser::from_grammar(&grammar);

            let parse_result = recursive_descent_parser.parse_from_tokens(
                vec![
                    Token::new(Some(0u64), DummySyntaxTokenTest::ATerminal),
                    Token::new(Some(0u64), DummySyntaxTokenTest::BTerminal),
                ]
                .into_iter(),
            );

            let expected_failed_symbol = <FailedSymbol<u64, DummySyntaxTokenTest>>::new(
                vec![FailedProduction::new(
                    FailedSymbol::new(vec![], DummySyntaxTokenTest::ATerminal),
                    vec![ProductionParsedSymbol::Ok(AbstractSyntaxNode::new(
                        vec![],
                        Token::new(Some(0u64), DummySyntaxTokenTest::ATerminal),
                    ))],
                    vec![],
                )],
                DummySyntaxTokenTest::S,
            );

            let expected_parse_result = ParseResult::Err(expected_failed_symbol);

            assert!(parse_result_equals(&parse_result, &expected_parse_result,));
        }

        #[test]
        fn it_fails_on_unexpected_symbol_when_a_production_is_found_with_parsed_symbols_ordered(
        ) -> () {
            #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
            enum DummySyntaxTokenTest {
                ATerminal,
                BTerminal,
                Epsilon,
                S,
            }

            let grammar_productions: Vec<ContextFreeGrammarProduction<DummySyntaxTokenTest>> =
                vec![ContextFreeGrammarProduction::new(
                    DummySyntaxTokenTest::S,
                    vec![
                        DummySyntaxTokenTest::ATerminal,
                        DummySyntaxTokenTest::BTerminal,
                        DummySyntaxTokenTest::ATerminal,
                    ],
                )];

            let grammar: ContextFreeGrammar<DummySyntaxTokenTest> = ContextFreeGrammar::new(
                DummySyntaxTokenTest::Epsilon,
                DummySyntaxTokenTest::S,
                grammar_productions,
            );

            let recursive_descent_parser: RecursiveDescentParser<
                u64,
                DummySyntaxTokenTest,
                SyntaxErrorSolverMock<u64, DummySyntaxTokenTest>,
            > = RecursiveDescentParser::from_grammar(&grammar);

            let parse_result = recursive_descent_parser.parse_from_tokens(
                vec![
                    Token::new(Some(0u64), DummySyntaxTokenTest::ATerminal),
                    Token::new(Some(0u64), DummySyntaxTokenTest::BTerminal),
                ]
                .into_iter(),
            );

            let expected_failed_symbol = <FailedSymbol<u64, DummySyntaxTokenTest>>::new(
                vec![FailedProduction::new(
                    FailedSymbol::new(vec![], DummySyntaxTokenTest::ATerminal),
                    vec![
                        ProductionParsedSymbol::Ok(AbstractSyntaxNode::new(
                            vec![],
                            Token::new(Some(0u64), DummySyntaxTokenTest::ATerminal),
                        )),
                        ProductionParsedSymbol::Ok(AbstractSyntaxNode::new(
                            vec![],
                            Token::new(Some(0u64), DummySyntaxTokenTest::BTerminal),
                        )),
                    ],
                    vec![],
                )],
                DummySyntaxTokenTest::S,
            );

            let expected_parse_result = ParseResult::Err(expected_failed_symbol);

            assert!(parse_result_equals(&parse_result, &expected_parse_result,));
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
                ContextFreeGrammarProduction::new(
                    DummySyntaxTokenTest::S,
                    vec![
                        DummySyntaxTokenTest::ATerminal,
                        DummySyntaxTokenTest::ATerminal,
                    ],
                ),
                ContextFreeGrammarProduction::new(
                    DummySyntaxTokenTest::S,
                    vec![
                        DummySyntaxTokenTest::ATerminal,
                        DummySyntaxTokenTest::ATerminal,
                        DummySyntaxTokenTest::ATerminal,
                    ],
                ),
            ];

            let grammar: ContextFreeGrammar<DummySyntaxTokenTest> = ContextFreeGrammar::new(
                DummySyntaxTokenTest::Epsilon,
                DummySyntaxTokenTest::S,
                grammar_productions,
            );

            let recursive_descent_parser: RecursiveDescentParser<
                u64,
                DummySyntaxTokenTest,
                SyntaxErrorSolverMock<u64, DummySyntaxTokenTest>,
            > = RecursiveDescentParser::from_grammar(&grammar);

            let parse_result = recursive_descent_parser.parse_from_tokens(
                vec![
                    Token::new(Some(0u64), DummySyntaxTokenTest::ATerminal),
                    Token::new(Some(0u64), DummySyntaxTokenTest::BTerminal),
                ]
                .into_iter(),
            );

            let expected_failed_symbol = <FailedSymbol<u64, DummySyntaxTokenTest>>::new(
                vec![
                    FailedProduction::new(
                        FailedSymbol::new(vec![], DummySyntaxTokenTest::ATerminal),
                        vec![ProductionParsedSymbol::Ok(AbstractSyntaxNode::new(
                            vec![],
                            Token::new(Some(0u64), DummySyntaxTokenTest::ATerminal),
                        ))],
                        vec![],
                    ),
                    FailedProduction::new(
                        FailedSymbol::new(vec![], DummySyntaxTokenTest::ATerminal),
                        vec![ProductionParsedSymbol::Ok(AbstractSyntaxNode::new(
                            vec![],
                            Token::new(Some(0u64), DummySyntaxTokenTest::ATerminal),
                        ))],
                        vec![DummySyntaxTokenTest::ATerminal],
                    ),
                ],
                DummySyntaxTokenTest::S,
            );

            let expected_parse_result = ParseResult::Err(expected_failed_symbol);

            assert!(parse_result_equals(&parse_result, &expected_parse_result,));
        }
    }
}
