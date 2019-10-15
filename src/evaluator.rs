use crate::parser::{Expression, Prefix, Statement};

#[derive(Debug, PartialEq)]
pub enum Object {
    Null,
    Int(isize),
    Boolean(bool),
}

pub fn eval(ast: Vec<Statement>) -> Object {
    let mut result = Object::Null;

    for statement in ast {
        match statement {
            Statement::ExpressionStatement(exp) => {
                result = eval_expression(exp);
            }
            _ => (),
        }
    }

    result
}

fn eval_expression(exp: Expression) -> Object {
    match exp {
        Expression::Int(val) => Object::Int(val),
        Expression::Boolean(val) => Object::Boolean(val),
        Expression::Prefix { prefix, value } => match prefix {
            Prefix::BANG => match eval_expression(*value) {
                Object::Boolean(val) => Object::Boolean(!val),
                _ => panic!("'!' operator only valid for boolean types"),
            },
            Prefix::MINUS => match eval_expression(*value) {
                Object::Int(val) => Object::Int(-val),
                _ => panic!("'-' operator only valid for integer types"),
            },
        },
        _ => panic!("Unexpected Expression in eval_expression"),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        evaluator::{eval, Object},
        lexer::lexer,
        parser::parse,
    };

    // Convenience function to lex, parse and eval an input
    fn evaluated(input: &str) -> Object {
        let mut tokens = lexer(input.as_bytes());
        let statements = parse(&mut tokens);
        eval(statements)
    }

    #[test]
    fn test_expression_eval() {
        let input = "5";
        let expected = Object::Int(5);
        assert_eq!(expected, evaluated(input));

        let input = "false";
        let expected = Object::Boolean(false);
        assert_eq!(expected, evaluated(input));
    }

    #[test]
    fn test_prefixes() {
        let input = "!true";
        let expected = Object::Boolean(false);
        assert_eq!(expected, evaluated(input));

        let input = "!!false";
        let expected = Object::Boolean(false);
        assert_eq!(expected, evaluated(input));

        let input = "-27";
        let expected = Object::Int(-27);
        assert_eq!(expected, evaluated(input));
    }
}
