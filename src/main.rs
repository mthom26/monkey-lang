use rustyline::{
    self,
    error::ReadlineError
};

mod lexer;
use lexer::{lexer};

fn main() {
    let mut rl = rustyline::Editor::<()>::new();

    loop {
        match rl.readline(">> ") {
            Ok(line) => {
                let tokens = lexer(line.as_bytes());
                println!("{:?}", tokens);
            },
            Err(ReadlineError::Interrupted) => break, // 'Ctrl-c' pressed
            Err(_) => println!("No Input")
        }
    }
}
