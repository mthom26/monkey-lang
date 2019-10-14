use crate::parser::{Expression, Statement};

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
}
