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
    Infix { left: Box<Expression>, op: Operator, right: Box<Expression> }
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

pub fn parse(tokens: &Vec<Token>) -> Vec<Statement> {
    let mut pos = 0;
    let mut statements: Vec<Statement> = Vec::new();

    loop {
        match tokens[pos] {
            Token::EOF => break,
            Token::LET => {
                let (statement, new_pos) = parse_let(tokens, pos);
                statements.push(statement);
                pos = new_pos;
            },
            Token::RETURN => {
                let (statement, new_pos) = parse_return(tokens, pos);
                statements.push(statement);
                pos = new_pos;
            },
            _ => {
                let (exp, new_pos) = parse_expression(tokens, pos, Precedence::LOWEST);
                statements.push(Statement::ExpressionStatement(exp));
                pos = new_pos;
            }
        }
        pos += 1;
    }

    statements
}

fn parse_let(tokens: &Vec<Token>, start_pos: usize) -> (Statement, usize) {
    let mut pos = start_pos + 1;
    // println!("Tokens: {:?}", tokens);
    let name = match &tokens[pos] {
        Token::IDENT(name) => name.clone(),
        _ => panic!("Parse error in let statement. Expected Identifier.") 
    };
    pos += 1;

    match &tokens[pos] {
        Token::ASSIGN => (),
        _ => panic!("Parse error in let statement. Expected equality assignment.") 
    }
    pos += 1;

    let (value, pos) = parse_expression(tokens, pos, Precedence::LOWEST);

    let statement = Statement::Let {
        name, value
    };

    (statement, pos)
}

fn parse_return(tokens: &Vec<Token>, start_pos: usize) -> (Statement, usize) {
    let mut pos = start_pos + 1;

    let (value, pos) = parse_expression(tokens, pos, Precedence::LOWEST);

    let statement = Statement::Return {
        value
    };

    (statement, pos)
}

fn parse_expression(
    tokens: &Vec<Token>,
    start_pos: usize,
    precedence: Precedence
) -> (Expression, usize) {
    let mut pos = start_pos;
    // println!("parse_expression: {:?}, {}", tokens[pos], pos);
    let mut left_exp = match tokens[pos] {
        Token::INT(val) => Expression::Int(val),
        Token::LPAREN => {
            let (exp, new_pos) = parse_expression(tokens, pos + 1, Precedence::LOWEST);
            pos = new_pos;
            exp
        },
        _ => panic!("Derp!")
    };
    pos += 1;

    // println!("{}\n", tokens[pos] != Token::EOF);

    while precedence < tokens[pos].precedence() {
        let (new_exp, new_pos) = parse_infix(tokens, pos, left_exp);
        left_exp = new_exp;
        pos = new_pos;
    }

    (left_exp, pos)
}

#[allow(dead_code)]
fn parse_infix(
    tokens: &Vec<Token>,
    start_pos: usize,
    left: Expression
) -> (Expression, usize) {
    let mut pos = start_pos;
    // println!("parse_infix: {:?}, {}", tokens[pos], pos);
    let op = match tokens[pos] {
        Token::PLUS => Operator::PLUS,
        Token::MINUS => Operator::MINUS,
        Token::ASTERISK => Operator::MULTIPLY,
        Token::SLASH => Operator::DIVIDE,
        Token::EQ => Operator::EQUAL,
        Token::NEQ => Operator::NEQUAL,
        Token::GT => Operator::GREATER,
        Token::LT => Operator::LESS,
        _ => panic!("Parse Infix called on invalid Token.")
    };
    pos += 1;

    let (right_exp, new_pos) = parse_expression(tokens, pos, tokens[pos - 1].precedence());
    pos = new_pos;

    let exp = Expression::Infix {
        left: Box::new(left),
        op,
        right: Box::new(right_exp)
    };

    (exp, pos)
}

#[cfg(test)]
mod tests {
    use crate::{
        lexer::lexer,
        parser::{parse, Statement, Expression, Operator}
    };

    #[test]
    fn parse_basic_let_statement() {
        // TODO After implementing expression parsing check that '8' evaluates correctly
        let input = "let var_name = 8;";

        let tokens = lexer(input.as_bytes());
        let statements = parse(&tokens);

        let expected = vec![
            Statement::Let{ name: "var_name".to_owned(), value: Expression::Int(8) }
        ];

        assert_eq!(expected, statements);
    }

    #[test]
    fn parse_basic_return_statement() {
        // TODO After implementing expression parsing check that '5' evaluates correctly
        let input = "return 5;";

        let tokens = lexer(input.as_bytes());
        let statements = parse(&tokens);

        let expected = vec![
            Statement::Return{ value: Expression::Int(5) }
        ];

        assert_eq!(expected, statements);
    }

    #[test]
    fn parse_basic_expression() {
        let input = "2 + 5 + 8;";

        let tokens = lexer(input.as_bytes());
        let statements = parse(&tokens);

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

        let tokens = lexer(input.as_bytes());
        let statements = parse(&tokens);

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

        let tokens = lexer(input.as_bytes());
        let statements = parse(&tokens);

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
}
