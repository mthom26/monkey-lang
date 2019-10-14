use std::collections::VecDeque;
use crate::lexer::Token;

#[derive(Debug, PartialEq)]
pub enum Statement {
    Let { name: String, value: Expression },
    Return { value: Expression },
    ExpressionStatement(Expression)
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Int(isize),
    Boolean(bool),
    Ident(String),
    String(String),
    Infix { left: Box<Expression>, op: Operator, right: Box<Expression> },
    Prefix { prefix: Prefix, value: Box<Expression> },
    If { condition: Box<Expression>, consequence: Vec<Statement>, alternative: Vec<Statement>},
    FnLiteral { parameters: Vec<String>, body: Vec<Statement> },
    FnCall { function: Box<Expression>, args: Vec<Expression> },
}

#[derive(Debug, PartialEq)]
pub enum Prefix {
    BANG,
    MINUS
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    PLUS,
    MINUS,
    MULTIPLY,
    DIVIDE,
    GREATER,
    LESS,
    EQUAL,
    NEQUAL
}

#[derive(PartialOrd, PartialEq)]
enum Precedence {
    LOWEST,
    EQUALS,         // ==
    LESSGREATER,    // < or >
    SUM,            // + or -
    PRODUCT,        // * or /
    PREFIX          // -x
}

impl Token {
    fn precedence(&self) -> Precedence {
        match self {
            Token::PLUS => Precedence::SUM,
            Token::MINUS => Precedence::SUM,
            Token::LT => Precedence::LESSGREATER,
            Token::GT => Precedence::LESSGREATER,
            Token::EQ => Precedence::EQUALS,
            Token::NEQ => Precedence::EQUALS,
            Token::ASTERISK => Precedence::PRODUCT,
            Token::SLASH => Precedence::PRODUCT,
            _ => Precedence::LOWEST
        }
    }
}

pub fn parse(tokens: &mut VecDeque<Token>) -> Vec<Statement> {
    let mut statements: Vec<Statement> = Vec::new();

    loop {
        match &tokens[0] {
            Token::EOF => break,
            Token::LET => {
                tokens.pop_front(); // Discard LET Token
                let statement = parse_let(tokens);
                assert_eq!(Token::SEMICOLON, tokens.pop_front().unwrap());
                statements.push(statement);
            },
            Token::RETURN => {
                tokens.pop_front(); // Discard RETURN Token
                let statement = parse_return(tokens);
                assert_eq!(Token::SEMICOLON, tokens.pop_front().unwrap());
                statements.push(statement);
            },
            Token::RBRACE => break, // We must be at end of a block so break
            _ => {
                let exp = parse_expression(tokens, Precedence::LOWEST);
                statements.push(Statement::ExpressionStatement(exp));
            }
        }
    }

    statements
}

fn parse_let(tokens: &mut VecDeque<Token>) -> Statement {
    let name = match tokens.pop_front() {
        Some(Token::IDENT(name)) => name.clone(),
        _ => panic!("Parse error in let statement. Expected Identifier.")
    };

    match tokens.pop_front() {
        Some(Token::ASSIGN) => (),
        _ => panic!("Parse error in let statement. Expected ASSIGN Token.")
    };

    let value = parse_expression(tokens, Precedence::LOWEST);

    Statement::Let {
        name, value
    }
}

fn parse_return(tokens: &mut VecDeque<Token>) -> Statement {
    let value = parse_expression(tokens, Precedence::LOWEST);

    Statement::Return {
        value
    }
}

fn parse_expression(tokens: &mut VecDeque<Token>, precedence: Precedence) -> Expression {
    // println!("parse_expression: {:?}", &tokens[0]);
    let mut left_exp = match tokens.pop_front() {
        Some(Token::INT(val)) => Expression::Int(val),
        Some(Token::TRUE) => Expression::Boolean(true),
        Some(Token::FALSE) => Expression::Boolean(false),
        Some(Token::STRING(val)) => Expression::String(val),
        Some(Token::IDENT(name)) => {
            if tokens[0] == Token::LPAREN { // Ident followed by LPAREN is a function call
                assert_eq!(Token::LPAREN, tokens.pop_front().unwrap());
                let mut args = vec![];

                loop {
                    match tokens[0] {
                        Token::RPAREN => break,
                        _ => {
                            let arg = parse_expression(tokens, Precedence::LOWEST);
                            args.push(arg);
                        }
                    }

                    match tokens.pop_front().unwrap() {
                        Token::RPAREN => break,
                        Token::COMMA => continue,
                        _ => panic!("Unexpected error when parsing function call.")
                    }
                }

                Expression::FnCall {
                    function: Box::new(Expression::Ident(name)),
                    args
                }
            } else {
                Expression::Ident(name)
            }
        },
        Some(Token::LPAREN) => {
            let exp = parse_expression(tokens, Precedence::LOWEST);
            match tokens.pop_front() {
                Some(Token::RPAREN) => (),
                _ => panic!("Expected RPAREN Token.")
            }
            exp
        },
        Some(Token::IF) => {
            assert_eq!(Token::LPAREN, tokens.pop_front().unwrap());
            let condition = parse_expression(tokens, Precedence::LOWEST);
            assert_eq!(Token::RPAREN, tokens.pop_front().unwrap());

            assert_eq!(Token::LBRACE, tokens.pop_front().unwrap());
            let consequence = parse(tokens);
            assert_eq!(Token::RBRACE, tokens.pop_front().unwrap());
            
            let alternative = match &tokens[0] {
                Token::ELSE => {
                    tokens.pop_front();
                    assert_eq!(Token::LBRACE, tokens.pop_front().unwrap());
                    let alternative = parse(tokens);
                    assert_eq!(Token::RBRACE, tokens.pop_front().unwrap());
                    alternative
                },
                _ => Vec::new()
            };

            Expression::If {
                condition: Box::new(condition),
                consequence,
                alternative
            }
        },
        Some(Token::MINUS) => Expression::Prefix {
            prefix: Prefix::MINUS,
            value: Box::new(parse_expression(tokens, Precedence::PREFIX))
        },
        Some(Token::BANG) => Expression::Prefix {
            prefix: Prefix::BANG,
            value: Box::new(parse_expression(tokens, Precedence::PREFIX))
        },
        Some(Token::FN) => {
            assert_eq!(Token::LPAREN, tokens.pop_front().unwrap());
            let mut parameters = vec![];

            loop {
                match tokens.pop_front().unwrap() {
                    Token::IDENT(val) => {
                        parameters.push(val);
                        match tokens.pop_front().unwrap() {
                            Token::COMMA => continue,
                            Token::RPAREN => break,
                            _ => panic!("Unexpected Token in Function Literal.")
                        };
                    },
                    Token::RPAREN => break,
                    _ => panic!("Unexpected Token in Function Literal.")
                }
            }

            assert_eq!(Token::LBRACE, tokens.pop_front().unwrap());
            let body = parse(tokens);
            assert_eq!(Token::RBRACE, tokens.pop_front().unwrap());

            Expression::FnLiteral {
                parameters,
                body
            }
        },
        _ => panic!("Unexpected token in parse_expression")
    };

    let mut next_token = &tokens[0];
    while precedence < next_token.precedence() {
        left_exp = parse_infix(tokens, left_exp);
        next_token = &tokens[0];
    }

    left_exp
}

fn parse_infix(tokens: &mut VecDeque<Token>, left: Expression) -> Expression {
    // println!("parse_infix: {:?}", &tokens[0]);
    let (op, precedence) = match tokens.pop_front() {
        Some(Token::MINUS) => (Operator::MINUS, Token::MINUS.precedence()),
        Some(Token::PLUS) => (Operator::PLUS, Token::PLUS.precedence()),
        Some(Token::ASTERISK) => (Operator::MULTIPLY, Token::ASTERISK.precedence()),
        Some(Token::SLASH) => (Operator::DIVIDE, Token::SLASH.precedence()),
        Some(Token::EQ) => (Operator::EQUAL, Token::EQ.precedence()),
        Some(Token::NEQ) => (Operator::NEQUAL, Token::NEQ.precedence()),
        Some(Token::GT) => (Operator::GREATER, Token::GT.precedence()),
        Some(Token::LT) => (Operator::LESS, Token::LT.precedence()),
        _ => panic!("Parse Infix called on invalid Token.")
    };

    let right_exp = parse_expression(tokens, precedence);

    Expression::Infix {
        left: Box::new(left),
        op,
        right: Box::new(right_exp)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        lexer::lexer,
        parser::{parse, Statement, Expression, Operator, Prefix}
    };

    #[test]
    fn parse_basic_let_statement() {
        let input = "let var_name = 8;";

        let mut tokens = lexer(input.as_bytes());
        let statements = parse(&mut tokens);

        let expected = vec![
            Statement::Let{ name: "var_name".to_owned(), value: Expression::Int(8) }
        ];

        assert_eq!(expected, statements);
    }

    #[test]
    fn parse_basic_return_statement() {
        let input = "return 5;";

        let mut tokens = lexer(input.as_bytes());
        let statements = parse(&mut tokens);

        let expected = vec![
            Statement::Return{ value: Expression::Int(5) }
        ];

        assert_eq!(expected, statements);
    }

    #[test]
    fn parse_basic_expression() {
        let input = "2 + 5 + 8";

        let mut tokens = lexer(input.as_bytes());
        let statements = parse(&mut tokens);

        let expected = vec![
            Statement::ExpressionStatement(Expression::Infix {
                left: Box::new(Expression::Infix {
                    left: Box::new(Expression::Int(2)),
                    op: Operator::PLUS,
                    right: Box::new(Expression::Int(5))
                }),
                op: Operator::PLUS,
                right: Box::new(Expression::Int(8))
            })
        ];

        assert_eq!(expected, statements);
    }

    #[test]
    fn parse_parenthesised_expression() {
        let input = "2 + (5 + 8)";

        let mut tokens = lexer(input.as_bytes());
        let statements = parse(&mut tokens);

        let expected = vec![
            Statement::ExpressionStatement(Expression::Infix {
                left: Box::new(Expression::Int(2)),
                op: Operator::PLUS,
                right: Box::new(Expression::Infix {
                    left: Box::new(Expression::Int(5)),
                    op: Operator::PLUS,
                    right: Box::new(Expression::Int(8))
                })
            })
        ];

        assert_eq!(expected, statements);
    }

    #[test]
    fn parse_operators() {
        let input = "1 + 2 * 3";

        let mut tokens = lexer(input.as_bytes());
        let statements = parse(&mut tokens);

        let expected = vec![
            Statement::ExpressionStatement(Expression::Infix {
                left: Box::new(Expression::Int(1)),
                op: Operator::PLUS,
                right: Box::new(Expression::Infix {
                    left: Box::new(Expression::Int(2)),
                    op: Operator::MULTIPLY,
                    right: Box::new(Expression::Int(3))
                })
            })
        ];

        assert_eq!(expected, statements);
    }

    #[test]
    fn test_if_statement() {
        let input = "if (7) { 1 + 3 } else { 8 }";

        let mut tokens = lexer(input.as_bytes());
        let statements = parse(&mut tokens);

        let expected = vec![
            Statement::ExpressionStatement(Expression::If {
                condition: Box::new(Expression::Int(7)),
                consequence: vec![
                    Statement::ExpressionStatement(Expression::Infix {
                        left: Box::new(Expression::Int(1)),
                        op: Operator::PLUS,
                        right: Box::new(Expression::Int(3))
                    })
                ],
                alternative: vec![
                    Statement::ExpressionStatement(Expression::Int(8))
                ]
            })
        ];

        assert_eq!(expected, statements);
    }

    #[test]
    fn test_prefixes() {
        let input = "let a = -33;
        let b = !true;
        let c = -1 + 2 + 3;";

        let mut tokens = lexer(input.as_bytes());
        let statements = parse(&mut tokens);

        let expected = vec![
            Statement::Let {
                name: "a".to_owned(),
                value: Expression::Prefix {
                    prefix: Prefix::MINUS,
                    value: Box::new(Expression::Int(33))
                }
            },
            Statement::Let {
                name: "b".to_owned(),
                value: Expression::Prefix {
                    prefix: Prefix::BANG,
                    value: Box::new(Expression::Boolean(true))
                }
            },
            Statement::Let {
                name: "c".to_owned(),
                value: Expression::Infix {
                    left: Box::new(Expression::Infix {
                        left: Box::new(Expression::Prefix {
                            prefix: Prefix::MINUS,
                            value: Box::new(Expression::Int(1))
                        }),
                        op: Operator::PLUS,
                        right: Box::new(Expression::Int(2))
                    }),
                    op: Operator::PLUS,
                    right: Box::new(Expression::Int(3))
                }
            },
        ];

        assert_eq!(expected, statements);
    }

    #[test]
    fn test_fn_literal() {
        let input = "fn(a, b) {
            return 23;
        }";

        let mut tokens = lexer(input.as_bytes());
        let statements = parse(&mut tokens);

        let expected  = vec![
            Statement::ExpressionStatement(Expression::FnLiteral {
                parameters: vec!["a".to_owned(), "b".to_owned()],
                body: vec![
                    Statement::Return {
                        value: Expression::Int(23)
                    }
                ]
            })
        ];

        assert_eq!(expected, statements);
    }

    #[test]
    fn test_function_call() {
        let input = "add(2, 7)";

        let mut tokens = lexer(input.as_bytes());
        let statements = parse(&mut tokens);

        let expected = vec![
            Statement::ExpressionStatement(Expression::FnCall {
                function: Box::new(Expression::Ident("add".to_owned())),
                args: vec![
                    Expression::Int(2),
                    Expression::Int(7)
                ]
            })
        ];

        assert_eq!(expected, statements);
    }

    #[test]
    fn test_mock_program() {
        let input = "let x = 7;
        let hello = true;
        let name = 'spyro';

        if(hello) {
            let y = x;
            11
        } else {
            2 + 3 * 9
        }
        
        return 1;";

        let mut tokens = lexer(input.as_bytes());
        let statements = parse(&mut tokens);

        let expected = vec![
            Statement::Let {
                name: "x".to_owned(),
                value: Expression::Int(7)
            },
            Statement::Let {
                name: "hello".to_owned(),
                value: Expression::Boolean(true)
            },
            Statement::Let {
                name: "name".to_owned(),
                value: Expression::String("spyro".to_owned())
            },
            Statement::ExpressionStatement(Expression::If {
                condition: Box::new(Expression::Ident("hello".to_owned())),
                consequence: vec![
                    Statement::Let {
                        name: "y".to_owned(),
                        value: Expression::Ident("x".to_owned())
                    },
                    Statement::ExpressionStatement(Expression::Int(11))
                ],
                alternative: vec![
                    Statement::ExpressionStatement(Expression::Infix {
                        left: Box::new(Expression::Int(2)),
                        op: Operator::PLUS,
                        right: Box::new(Expression::Infix {
                            left: Box::new(Expression::Int(3)),
                            op: Operator::MULTIPLY,
                            right: Box::new(Expression::Int(9))
                        })
                    })
                ]
            }),
            Statement::Return {
                value: Expression::Int(1)
            }
        ];

        assert_eq!(expected, statements);
    }
}
