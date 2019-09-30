#[derive(Debug, PartialEq)]
pub enum Token {
    ILLEGAL,            // Illegal token
    EOF,                // End of file
    IDENT(String),      // Identifier
    INT(isize),         // Integer
    STRING(String),     // String

    //Operators
    ASSIGN,             // '='
    PLUS,               // '+'
    MINUS,              // '-'
    LT,                 // '<' Less than
    GT,                 // '>' Greater than
    EQ,                 // '==' Equal to
    NEQ,                // '!=' Not equal to
    BANG,               // '!'

    // Delimiters
    COMMA,              // ','
    SEMICOLON,          // ';'
    LPAREN,             // '('
    RPAREN,             // ')'
    LBRACE,             // '{'
    RBRACE,             // '}'

    // Keywords
    FN,                 // Function            
    LET,
    IF,
    ELSE,
    RETURN,
    TRUE,
    FALSE
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
            ch if is_letter(ch) => {
                let (new_pos, token) = read_letters(pos, input);
                tokens.push(token);
                pos = new_pos;
            },
            b'{' => tokens.push(Token::LBRACE),
            b'}' => tokens.push(Token::RBRACE),
            b'(' => tokens.push(Token::LPAREN),
            b')' => tokens.push(Token::RPAREN),
            b';' => tokens.push(Token::SEMICOLON),
            b',' => tokens.push(Token::COMMA),
            b'+' => tokens.push(Token::PLUS),
            b'-' => tokens.push(Token::MINUS),
            b'=' => {
                match peek_next_char(pos, input) {
                    b'=' => { tokens.push(Token::EQ); pos += 1; },
                    0 => tokens.push(Token::EOF),
                    _ => tokens.push(Token::ASSIGN)
                }
            },
            b'!' => {
                match peek_next_char(pos, input) {
                    b'=' => { tokens.push(Token::NEQ); pos += 1; },
                    0 => tokens.push(Token::EOF),
                    _ => tokens.push(Token::BANG)
                }
            },
            b'>' => tokens.push(Token::GT),
            b'<' => tokens.push(Token::LT),
            b' ' => (), // Ignore whitespace
            _ => tokens.push(Token::ILLEGAL),
        }
        pos += 1;
    }

    tokens
}

// Currently number digits can't be used in identifiers
// May change this later
fn is_letter(ch: u8) -> bool {
    b'a' <= ch && ch <= b'z' ||
    b'A' <= ch && ch <= b'Z' ||
    b'_' == ch
}

fn read_letters(start_pos: usize, input: &[u8]) -> (usize, Token) {
    let mut pos = start_pos;
    let mut identifier = Vec::new();
    // Add next character to identifier until next character is not a letter
    while pos < input.len() && is_letter(input[pos]) {
        identifier.push(input[pos]);
        pos += 1;
    }

    let token = is_keyword(&identifier);
    (pos - 1, token)
}

fn is_keyword(chars: &[u8]) -> Token {
    match chars {
        [b'l', b'e', b't'] => Token::LET,
        [b'f', b'n'] => Token::FN,
        [b'i', b'f'] => Token::IF,
        [b'e', b'l', b's', b'e'] => Token::ELSE,
        [b'r', b'e', b't', b'u', b'r', b'n'] => Token::RETURN,
        [b't', b'r', b'u', b'e'] => Token::TRUE,
        [b'f', b'a', b'l', b's', b'e'] => Token::FALSE,
        chars => Token::IDENT(String::from_utf8_lossy(chars).to_string())
    }
}

// Peek at the next character in input
fn peek_next_char(start_pos: usize, input: &[u8]) -> u8 {
    if start_pos >= input.len() {
        // We must be at the EOF, outer loop should catch this so it
        // might not be needed here...
        return 0;
    }
    input[start_pos + 1]
}

#[cfg(test)]
mod tests {
    
    use crate::lexer::{lexer, Token, is_letter};

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

    #[test]
    fn test_is_letter() {
        let input = b'a';
        assert!(is_letter(input));
        let input = b'A';
        assert!(is_letter(input));
        let input = b'z';
        assert!(is_letter(input));
        let input = b'Z';
        assert!(is_letter(input));
        let input = b'f';
        assert!(is_letter(input));
        let input = b'U';
        assert!(is_letter(input));
        let input = b'_';
        assert!(is_letter(input));

        let input = b'&';
        assert!(!is_letter(input));
    }

    #[test]
    fn lex_ignore_whitespace() {
        let input = "}let  hello     ){  ; ";

        let tokens = lexer(input.as_bytes());
        let expected = vec![
            Token::RBRACE,
            Token::LET,
            Token::IDENT("hello".to_owned()),
            Token::RPAREN,
            Token::LBRACE,
            Token::SEMICOLON,
            Token::EOF,
        ];

        assert_eq!(expected, tokens);
    }

    #[test]
    fn test_double_character_tokens() {
        let input = "= == !=;";

        let tokens = lexer(input.as_bytes());
        let expected = vec![
            Token::ASSIGN,
            Token::EQ,
            Token::NEQ,
            Token::SEMICOLON,
            Token::EOF,
        ];

        assert_eq!(expected, tokens);
    }

    #[test]
    fn test_keywords() {
        let input = "let x = true; if hello == false { fn y() }";

        let tokens = lexer(input.as_bytes());
        let expected = vec![
            Token::LET,
            Token::IDENT("x".to_owned()),
            Token::ASSIGN,
            Token::TRUE,
            Token::SEMICOLON,
            Token::IF,
            Token::IDENT("hello".to_owned()),
            Token::EQ,
            Token::FALSE,
            Token::LBRACE,
            Token::FN,
            Token::IDENT("y".to_owned()),
            Token::LPAREN,
            Token::RPAREN,
            Token::RBRACE,
            Token::EOF,
        ];

        assert_eq!(expected, tokens);
    }
}
