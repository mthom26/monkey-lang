use crate::parser::{Expression, Operator, Prefix, Statement};

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
        Expression::Infix { left, op, right } => match op {
            // Integer operations
            Operator::PLUS => match (eval_expression(*left), eval_expression(*right)) {
                (Object::Int(l_val), Object::Int(r_val)) => Object::Int(l_val + r_val),
                _ => panic!("'+' operator only valid on integers"),
            },
            Operator::MINUS => match (eval_expression(*left), eval_expression(*right)) {
                (Object::Int(l_val), Object::Int(r_val)) => Object::Int(l_val - r_val),
                _ => panic!("'-' operator only valid on integers"),
            },
            Operator::MULTIPLY => match (eval_expression(*left), eval_expression(*right)) {
                (Object::Int(l_val), Object::Int(r_val)) => Object::Int(l_val * r_val),
                _ => panic!("'*' operator only valid on integers"),
            },
            Operator::DIVIDE => match (eval_expression(*left), eval_expression(*right)) {
                (Object::Int(l_val), Object::Int(r_val)) => Object::Int(l_val / r_val),
                _ => panic!("'/' operator only valid on integers"),
            },
            // Comparison operations
            Operator::EQUAL => match (eval_expression(*left), eval_expression(*right)) {
                (Object::Int(l_val), Object::Int(r_val)) => Object::Boolean(l_val == r_val),
                (Object::Boolean(l_val), Object::Boolean(r_val)) => Object::Boolean(l_val == r_val),
                _ => panic!("Problem in Infix equality check"),
            },
            Operator::NEQUAL => match (eval_expression(*left), eval_expression(*right)) {
                (Object::Int(l_val), Object::Int(r_val)) => Object::Boolean(l_val != r_val),
                (Object::Boolean(l_val), Object::Boolean(r_val)) => Object::Boolean(l_val != r_val),
                _ => panic!("Problem in Infix not equality check"),
            },
            Operator::GREATER => match (eval_expression(*left), eval_expression(*right)) {
                (Object::Int(l_val), Object::Int(r_val)) => Object::Boolean(l_val > r_val),
                _ => panic!("Problem in Infix greater than check"),
            },
            Operator::LESS => match (eval_expression(*left), eval_expression(*right)) {
                (Object::Int(l_val), Object::Int(r_val)) => Object::Boolean(l_val < r_val),
                _ => panic!("Problem in Infix less than check"),
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

    #[test]
    fn test_infixes() {
        let input = "2 + 3";
        let expected = Object::Int(5);
        assert_eq!(expected, evaluated(input));

        let input = "2 - 3";
        let expected = Object::Int(-1);
        assert_eq!(expected, evaluated(input));

        let input = "2 * 3";
        let expected = Object::Int(6);
        assert_eq!(expected, evaluated(input));

        let input = "9 / 3";
        let expected = Object::Int(3);
        assert_eq!(expected, evaluated(input));

        let input = "9 == 3";
        let expected = Object::Boolean(false);
        assert_eq!(expected, evaluated(input));

        let input = "9 == 9";
        let expected = Object::Boolean(true);
        assert_eq!(expected, evaluated(input));

        let input = "true == false";
        let expected = Object::Boolean(false);
        assert_eq!(expected, evaluated(input));

        let input = "3 != 6";
        let expected = Object::Boolean(true);
        assert_eq!(expected, evaluated(input));

        let input = "true != false";
        let expected = Object::Boolean(true);
        assert_eq!(expected, evaluated(input));

        let input = "3 > 4";
        let expected = Object::Boolean(false);
        assert_eq!(expected, evaluated(input));

        let input = "3 < 4";
        let expected = Object::Boolean(true);
        assert_eq!(expected, evaluated(input));
    }
}
