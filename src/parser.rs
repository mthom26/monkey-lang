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
    Infix { left: Box<Expression>, op: Operator, right: Box<Expression> },
    If { condition: Box<Expression>, consequence: Vec<Statement>, alternative: Vec<Statement>},
}

#[derive(Debug, PartialEq)]
enum Operator {
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
                statements.push(statement);
            },
            Token::RETURN => {
                tokens.pop_front(); // Discard RETURN Token
                let statement = parse_return(tokens);
                statements.push(statement);
            },
            Token::RBRACE => break, // We must be at end of a block so break
            _ => {
                let exp = parse_expression(tokens, Precedence::LOWEST);
                statements.push(Statement::ExpressionStatement(exp));
            }
        }

        match tokens.pop_front() {
            Some(Token::SEMICOLON) => (),
            _ => panic!("Expected SEMICOLON Token in parse loop.")
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
        _ => panic!("Parse error in let statement. Expected Identifier.")
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
            println!("CON");
            assert_eq!(Token::LBRACE, tokens.pop_front().unwrap());
            let consequence = parse(tokens);
            assert_eq!(Token::RBRACE, tokens.pop_front().unwrap());
            
            let alternative = match &tokens[0] {
                Token::ELSE => {
                    tokens.pop_front();
                    assert_eq!(Token::LBRACE, tokens.pop_front().unwrap());
                    println!("ALT");
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
        _ => panic!("Unexpected token in _parse_expression")
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
        parser::{parse, Statement, Expression, Operator}
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
        let input = "2 + 5 + 8;";

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
        let input = "2 + (5 + 8);";

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
        let input = "1 + 2 * 3;";

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
        let input = "if (7) { 1 + 3; } else { 8; };";

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
}
