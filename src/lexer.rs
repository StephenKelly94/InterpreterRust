#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal,
    Eof,

    Ident(String),
    Int(String),
    GreaterThen,

    Assign,
    Plus,

    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    Function,
    Let,
}

pub struct Lexer {
    input: Vec<u8>,
    position: usize,
    read_position: usize,
    char: u8,
}

impl Lexer {
    pub fn new(input: Vec<u8>) -> Lexer {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            char: 0,
        };
        lexer.read_char();
        lexer
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.char = 0;
        } else {
            self.char = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        let token = match self.char {
            b'=' => Token::Assign,
            b';' => Token::Semicolon,
            b'(' => Token::LParen,
            b')' => Token::RParen,
            b',' => Token::Comma,
            b'+' => Token::Plus,
            b'{' => Token::LBrace,
            b'}' => Token::RBrace,
            _ => Token::Eof
        };
        self.read_char();
        token
    }
}


#[test]
fn test_next_token() {
    let input = "=+(),;";

    let answer_tokens: Vec<Token> = vec![
        Token::Assign,
        Token::Plus,
        Token::LParen,
        Token::RParen,
        Token::Comma,
        Token::Semicolon,
    ];

    let mut lexer = Lexer::new(input.as_bytes().to_vec());
    for token in answer_tokens {
        let next_token = lexer.next_token();
        assert_eq!(next_token, token);
    }
}
