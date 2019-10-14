use std::collections::VecDeque;

#[rustfmt::skip]
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
    ASTERISK,           // '*'
    SLASH,              // '/'

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
    FALSE,
}

pub fn lexer(input: &[u8]) -> VecDeque<Token> {
    let mut pos = 0;
    let mut tokens = VecDeque::new();

    loop {
        if pos >= input.len() {
            tokens.push_back(Token::EOF);
            break;
        }
        match input[pos] {
            ch if is_letter(ch) => {
                let (new_pos, token) = read_letters(pos, input);
                tokens.push_back(token);
                pos = new_pos;
            }
            ch if is_digit(ch) => {
                let (new_pos, token) = read_digits(pos, input);
                tokens.push_back(token);
                pos = new_pos;
            }
            b'{' => tokens.push_back(Token::LBRACE),
            b'}' => tokens.push_back(Token::RBRACE),
            b'(' => tokens.push_back(Token::LPAREN),
            b')' => tokens.push_back(Token::RPAREN),
            b';' => tokens.push_back(Token::SEMICOLON),
            b',' => tokens.push_back(Token::COMMA),
            b'+' => tokens.push_back(Token::PLUS),
            b'-' => tokens.push_back(Token::MINUS),
            b'=' => match peek_next_char(pos, input) {
                b'=' => {
                    tokens.push_back(Token::EQ);
                    pos += 1;
                }
                0 => tokens.push_back(Token::EOF),
                _ => tokens.push_back(Token::ASSIGN),
            },
            b'!' => match peek_next_char(pos, input) {
                b'=' => {
                    tokens.push_back(Token::NEQ);
                    pos += 1;
                }
                0 => tokens.push_back(Token::EOF),
                _ => tokens.push_back(Token::BANG),
            },
            b'>' => tokens.push_back(Token::GT),
            b'<' => tokens.push_back(Token::LT),
            b'*' => tokens.push_back(Token::ASTERISK),
            b'/' => tokens.push_back(Token::SLASH),
            39 => {
                // 39 is the ascii int for the ' character
                let (new_pos, token) = read_string(pos, input);
                tokens.push_back(token);
                pos = new_pos;
            }
            b' ' | b'\n' | b'\r' | b'\t' => (), // Ignore whitespace
            _ => tokens.push_back(Token::ILLEGAL),
        }
        pos += 1;
    }

    tokens
}

// Currently number digits can't be used in identifiers
// May change this later
#[rustfmt::skip]
fn is_letter(ch: u8) -> bool {
    b'a' <= ch && ch <= b'z' ||
    b'A' <= ch && ch <= b'Z' ||
    b'_' == ch
}

fn is_digit(ch: u8) -> bool {
    b'0' <= ch && ch <= b'9'
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

fn read_digits(start_pos: usize, input: &[u8]) -> (usize, Token) {
    let mut pos = start_pos;
    let mut identifier = Vec::new();
    // Add next character to identifier until next character is not a letter
    while pos < input.len() && is_digit(input[pos]) {
        identifier.push(input[pos]);
        pos += 1;
    }

    let num: isize = String::from_utf8_lossy(&identifier)
        .to_string()
        .parse()
        .unwrap();

    let token = Token::INT(num);
    (pos - 1, token)
}

fn read_string(start_pos: usize, input: &[u8]) -> (usize, Token) {
    let mut pos = start_pos + 1;
    let mut value = Vec::new();

    while input[pos] != 39 {
        value.push(input[pos]);
        pos += 1;
    }

    let token = Token::STRING(String::from_utf8_lossy(&value).to_string());
    (pos, token)
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
        chars => Token::IDENT(String::from_utf8_lossy(chars).to_string()),
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
    use crate::lexer::{is_letter, lexer, Token};
    use std::collections::VecDeque;

    #[test]
    fn lex_tokens() {
        let input = "{}();,";

        let tokens = lexer(input.as_bytes());
        let expected = VecDeque::from(vec![
            Token::LBRACE,
            Token::RBRACE,
            Token::LPAREN,
            Token::RPAREN,
            Token::SEMICOLON,
            Token::COMMA,
            Token::EOF,
        ]);

        assert_eq!(expected, tokens);
    }

    #[test]
    fn lex_digits() {
        let input = "let x = 67;   hello    num3ber  9";

        let tokens = lexer(input.as_bytes());
        let expected = VecDeque::from(vec![
            Token::LET,
            Token::IDENT("x".to_owned()),
            Token::ASSIGN,
            Token::INT(67),
            Token::SEMICOLON,
            Token::IDENT("hello".to_owned()),
            Token::IDENT("num".to_owned()),
            Token::INT(3),
            Token::IDENT("ber".to_owned()),
            Token::INT(9),
            Token::EOF,
        ]);

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
        let expected = VecDeque::from(vec![
            Token::RBRACE,
            Token::LET,
            Token::IDENT("hello".to_owned()),
            Token::RPAREN,
            Token::LBRACE,
            Token::SEMICOLON,
            Token::EOF,
        ]);

        assert_eq!(expected, tokens);
    }

    #[test]
    fn ignore_all_whitespace() {
        let input = "let x = 4;
        let y =     30;
        let    z = {    hello };";

        let tokens = lexer(input.as_bytes());
        let expected = VecDeque::from(vec![
            Token::LET,
            Token::IDENT("x".to_owned()),
            Token::ASSIGN,
            Token::INT(4),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("y".to_owned()),
            Token::ASSIGN,
            Token::INT(30),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("z".to_owned()),
            Token::ASSIGN,
            Token::LBRACE,
            Token::IDENT("hello".to_owned()),
            Token::RBRACE,
            Token::SEMICOLON,
            Token::EOF,
        ]);

        assert_eq!(expected, tokens);
    }

    #[test]
    fn test_double_character_tokens() {
        let input = "= == !=;";

        let tokens = lexer(input.as_bytes());
        let expected = VecDeque::from(vec![
            Token::ASSIGN,
            Token::EQ,
            Token::NEQ,
            Token::SEMICOLON,
            Token::EOF,
        ]);

        assert_eq!(expected, tokens);
    }

    #[test]
    fn test_keywords() {
        let input = "let x = true; if hello == false { fn y() }";

        let tokens = lexer(input.as_bytes());
        let expected = VecDeque::from(vec![
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
        ]);

        assert_eq!(expected, tokens);
    }

    #[test]
    fn test_string() {
        let input = "let name = 'jimmy * 123';";

        let tokens = lexer(input.as_bytes());
        let expected = VecDeque::from(vec![
            Token::LET,
            Token::IDENT("name".to_owned()),
            Token::ASSIGN,
            Token::STRING("jimmy * 123".to_owned()),
            Token::SEMICOLON,
            Token::EOF,
        ]);

        assert_eq!(expected, tokens);
    }
}
