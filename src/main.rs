#[derive(Debug, PartialEq)]
pub enum Token {
    ILLEGAL,
    EOF,
    IDENT(String),
    INT(isize),
    STRING(String),

    // Operators
    ASSIGN,
    PLUS,

    // Delimiters
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // Keywords
    FN,
    LET,
}

fn main() {
    
}

pub fn lexer(input: &[u8]) -> Vec<Token> {
    let mut pos = 0;
    let mut tokens = Vec::new();

    loop {
        if pos >= input.len() {
            tokens.push(Token::EOF);
            break;
        }
        match input[pos] {
            b'{' => tokens.push(Token::LBRACE),
            b'}' => tokens.push(Token::RBRACE),
            b'(' => tokens.push(Token::LPAREN),
            b')' => tokens.push(Token::RPAREN),
            b';' => tokens.push(Token::SEMICOLON),
            b',' => tokens.push(Token::COMMA),
            _ => tokens.push(Token::ILLEGAL),
        }
        pos += 1;
    }

    tokens
}

#[cfg(test)]
mod tests {
    use crate::{lexer, Token};

    #[test]
    fn lex_tokens() {
        let input = "{}();,";

        let tokens = lexer(input.as_bytes());
        let expected = vec![
            Token::LBRACE,
            Token::RBRACE,
            Token::LPAREN,
            Token::RPAREN,
            Token::SEMICOLON,
            Token::COMMA,
            Token::EOF,
        ];

        assert_eq!(expected, tokens);
    }
}
