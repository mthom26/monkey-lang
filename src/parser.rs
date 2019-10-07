use crate::lexer::Token;

#[derive(Debug, PartialEq)]
pub enum Statement {
    Let { name: String, value: () }
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
            },
            _ => ()
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
    // 
    let value = parse_expression(tokens, pos);

    let statement = Statement::Let {
        name, value
    };

    (statement, pos)
}

fn parse_expression(tokens: &Vec<Token>, start_pos: usize) -> () {
    ()
}

#[cfg(test)]
mod tests {
    use crate::{
        lexer::lexer,
        parser::{parse, Statement}
    };

    #[test]
    fn parse_basic_let_statement() {
        // TODO After implementing expression parsing check that '8' evaluates correctly
        let input = "let var_name = 8;";

        let tokens = lexer(input.as_bytes());
        let statements = parse(&tokens);

        let expected = vec![
            Statement::Let{ name: "var_name".to_owned(), value: () }
        ];

        assert_eq!(expected, statements);
    }
}
