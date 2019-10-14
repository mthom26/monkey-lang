use rustyline::{
    self,
    error::ReadlineError
};

mod lexer;
use lexer::{lexer};
mod parser;
use parser::parse;

fn main() {
    let mut rl = rustyline::Editor::<()>::new();

    loop {
        match rl.readline(">> ") {
            Ok(line) => {
                let mut tokens = lexer(line.as_bytes());
                let ast = parse(&mut tokens);
                println!("{:?}", ast);
            },
            Err(ReadlineError::Interrupted) => break, // 'Ctrl-c' pressed
            Err(_) => println!("No Input")
        }
    }
}
