use crate::evaluator::Environment;
use crate::parser::{Expression, Operator, Prefix, Statement};

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Null,
    Int(isize),
    Boolean(bool),
    String(String),
    Return(Box<Object>),
    Function {
        parameters: Vec<String>,
        body: Vec<Statement>,
    },
}

pub fn eval_block(ast: Vec<Statement>, env: &mut Environment) -> Object {
    let mut result = Object::Null;

    for statement in ast {
        match statement {
            Statement::ExpressionStatement(exp) => {
                result = eval_expression(exp, env);
            }
            Statement::Return { value } => {
                result = Object::Return(Box::new(eval_expression(value, env)));
            }
            Statement::Let { name, value } => {
                let new_value = eval_expression(value, env);
                env.set(name, new_value.clone());
                result = new_value
            }
        }

        match result {
            Object::Return(_) => break,
            _ => (),
        }
    }

    result
}

pub fn eval(ast: Vec<Statement>, env: &mut Environment) -> Object {
    let result = eval_block(ast, env);

    // If final result is a Return unwrap it...
    match result {
        Object::Return(val) => *val,
        _ => result,
    }
}

fn eval_expression(exp: Expression, env: &mut Environment) -> Object {
    match exp {
        Expression::Int(val) => Object::Int(val),
        Expression::Boolean(val) => Object::Boolean(val),
        Expression::String(val) => Object::String(val),
        Expression::Prefix { prefix, value } => match prefix {
            Prefix::BANG => match eval_expression(*value, env) {
                Object::Boolean(val) => Object::Boolean(!val),
                _ => panic!("'!' operator only valid for boolean types"),
            },
            Prefix::MINUS => match eval_expression(*value, env) {
                Object::Int(val) => Object::Int(-val),
                _ => panic!("'-' operator only valid for integer types"),
            },
        },
        Expression::Infix { left, op, right } => match op {
            // Integer operations
            Operator::PLUS => match (eval_expression(*left, env), eval_expression(*right, env)) {
                (Object::Int(l_val), Object::Int(r_val)) => Object::Int(l_val + r_val),
                _ => panic!("'+' operator only valid on integers"),
            },
            Operator::MINUS => match (eval_expression(*left, env), eval_expression(*right, env)) {
                (Object::Int(l_val), Object::Int(r_val)) => Object::Int(l_val - r_val),
                _ => panic!("'-' operator only valid on integers"),
            },
            Operator::MULTIPLY => match (eval_expression(*left, env), eval_expression(*right, env))
            {
                (Object::Int(l_val), Object::Int(r_val)) => Object::Int(l_val * r_val),
                _ => panic!("'*' operator only valid on integers"),
            },
            Operator::DIVIDE => match (eval_expression(*left, env), eval_expression(*right, env)) {
                (Object::Int(l_val), Object::Int(r_val)) => Object::Int(l_val / r_val),
                _ => panic!("'/' operator only valid on integers"),
            },
            // Comparison operations
            Operator::EQUAL => match (eval_expression(*left, env), eval_expression(*right, env)) {
                (Object::Int(l_val), Object::Int(r_val)) => Object::Boolean(l_val == r_val),
                (Object::Boolean(l_val), Object::Boolean(r_val)) => Object::Boolean(l_val == r_val),
                _ => panic!("Problem in Infix equality check"),
            },
            Operator::NEQUAL => match (eval_expression(*left, env), eval_expression(*right, env)) {
                (Object::Int(l_val), Object::Int(r_val)) => Object::Boolean(l_val != r_val),
                (Object::Boolean(l_val), Object::Boolean(r_val)) => Object::Boolean(l_val != r_val),
                _ => panic!("Problem in Infix not equality check"),
            },
            Operator::GREATER => {
                match (eval_expression(*left, env), eval_expression(*right, env)) {
                    (Object::Int(l_val), Object::Int(r_val)) => Object::Boolean(l_val > r_val),
                    _ => panic!("Problem in Infix greater than check"),
                }
            }
            Operator::LESS => match (eval_expression(*left, env), eval_expression(*right, env)) {
                (Object::Int(l_val), Object::Int(r_val)) => Object::Boolean(l_val < r_val),
                _ => panic!("Problem in Infix less than check"),
            },
        },
        Expression::If {
            condition,
            consequence,
            alternative,
        } => match eval_expression(*condition, env) {
            Object::Boolean(true) => eval_block(consequence, env),
            Object::Boolean(false) => {
                if alternative.len() == 0 {
                    return Object::Null;
                }
                eval_block(alternative, env)
            }
            _ => panic!("If conditional must evaluate to a boolean"),
        },
        Expression::Ident(name) => env.get(&name),
        Expression::FnLiteral { parameters, body } => Object::Function { parameters, body },
        Expression::FnCall { function, args } => {
            let (parameters, body) = match *function {
                Expression::Ident(name) => match env.get(&name) {
                    Object::Function { parameters, body } => (parameters, body),
                    _ => panic!("Attempted to call non-function"),
                },
                Expression::FnLiteral { parameters, body } => (parameters, body),
                _ => panic!("Error calling function"),
            };

            assert_eq!(parameters.len(), args.len());

            let mut func_env = Environment::new();
            for (paramater, arg) in parameters.into_iter().zip(args.into_iter()) {
                func_env.set(paramater, eval_expression(arg, env));
            }

            eval(body, &mut func_env)
        }
        _ => panic!("Unexpected Expression in eval_expression"),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        evaluator::{eval, Environment, Object},
        lexer::lexer,
        parser::{parse, Expression, Statement},
    };

    // Convenience function to lex, parse and eval an input
    fn evaluated(input: &str) -> Object {
        let mut tokens = lexer(input.as_bytes());
        let statements = parse(&mut tokens);
        let mut env = Environment::new();
        eval(statements, &mut env)
    }

    #[test]
    fn test_expression_eval() {
        let input = "5";
        let expected = Object::Int(5);
        assert_eq!(expected, evaluated(input));

        let input = "false";
        let expected = Object::Boolean(false);
        assert_eq!(expected, evaluated(input));

        let input = "'hello'";
        let expected = Object::String("hello".to_owned());
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

    #[test]
    fn test_if_conditionals() {
        let input = "if(true) { 1 }";
        let expected = Object::Int(1);
        assert_eq!(expected, evaluated(input));

        let input = "if(false) { 1 } else { 2 }";
        let expected = Object::Int(2);
        assert_eq!(expected, evaluated(input));

        let input = "if(2 > 1) { true } else { false }";
        let expected = Object::Boolean(true);
        assert_eq!(expected, evaluated(input));

        let input = "if(2 < 1) { true } else { false }";
        let expected = Object::Boolean(false);
        assert_eq!(expected, evaluated(input));

        let input = "if(2 - 3 + 29 == 4 * 7) { 1 } else { 2 }";
        let expected = Object::Int(1);
        assert_eq!(expected, evaluated(input));
    }

    #[test]
    fn test_return() {
        let input = "return 2; 7";
        let expected = Object::Int(2);
        assert_eq!(expected, evaluated(input));

        let input = "if(true) {
            if(true) {
                return 2;
            }
            return 1;
        }";
        let expected = Object::Int(2);
        assert_eq!(expected, evaluated(input));
    }

    #[test]
    fn test_env() {
        let input = "let a = 3; return a;";
        let expected = Object::Int(3);
        assert_eq!(expected, evaluated(input));
    }

    #[test]
    fn test_fn_literals() {
        let input = "fn() { return 1; }";
        let expected = Object::Function {
            parameters: vec![],
            body: vec![Statement::Return {
                value: Expression::Int(1),
            }],
        };
        assert_eq!(expected, evaluated(input));

        let input = "fn(a, b) { return true; }";
        let expected = Object::Function {
            parameters: vec!["a".to_owned(), "b".to_owned()],
            body: vec![Statement::Return {
                value: Expression::Boolean(true),
            }],
        };
        assert_eq!(expected, evaluated(input));
    }

    #[test]
    fn test_fn_calls() {
        let input = "let ret = fn(x) { return x; }; ret(5)";
        let expected = Object::Int(5);
        assert_eq!(expected, evaluated(input));

        let input = "let ret = fn(x, y) { return x + y; }; ret(5, 9)";
        let expected = Object::Int(14);
        assert_eq!(expected, evaluated(input));
    }
}
