use rustyline::{self, error::ReadlineError};

mod lexer;
use lexer::lexer;
mod parser;
use parser::parse;
mod evaluator;
use evaluator::{eval, Environment, Object};
mod compiler;

fn main() {
    let mut rl = rustyline::Editor::<()>::new();
    let mut env = Environment::new();

    loop {
        match rl.readline(">> ") {
            Ok(line) => {
                let mut tokens = lexer(line.as_bytes());
                let ast = parse(&mut tokens);
                let evaluated = eval(ast, &mut env);

                match evaluated {
                    Object::Int(val) => println!("{}", val),
                    Object::Boolean(val) => println!("{}", val),
                    Object::Null => println!("Null"),
                    _ => println!("Evaluation Error"),
                }
            }
            Err(ReadlineError::Interrupted) => break, // 'Ctrl-c' pressed
            Err(_) => println!("No Input"),
        }
    }
}
