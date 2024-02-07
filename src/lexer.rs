use std::fmt::Display;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Token {
    Add,
    Subtract,
    Multiply,
    Divide,
    Number(i32),
    Eol,
    Unrecognized,
    //Assignemnt
    //I am currently unsure of how to detect/handle this
    //Use identifiers and store them in variables just like how you store user variables
    //Unrecognized,
    //Sin, Cos, Tan, etc
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Token::Add => write!(f, "+"),
            Token::Subtract => write!(f, "-"),
            Token::Multiply => write!(f, "*"),
            Token::Divide => write!(f, "/"),
            Token::Number(num) => write!(f, "{}", num),
            Token::Eol => write!(f, " EOL"),
            Token::Unrecognized => write!(f, "UNRECOGNIZED TOKEN"),
        };
    }
}

// Just floor division for now
pub struct Lexer {
    position: usize,
    read_position: usize,
    ch: u8,
    input: Vec<u8>,
}

#[allow(dead_code)]
impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            position: 0,
            read_position: 0,
            ch: 0,
            input: input.into_bytes(),
        };

        lexer.read_char();

        return lexer;
        //Do i parse
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        //println!("ch = {}", self.ch as char);

        let token = match self.ch {
            b'+' => Token::Add,
            b'-' => Token::Subtract,
            b'*' => Token::Multiply,
            b'/' => Token::Divide,
            b'0'..=b'9' => Token::Number(self.read_int()),
            0 => Token::Eol,
            _ => return Token::Unrecognized,
        };

        self.read_char();

        return token;
    }

    //Return result later in the future of future ness
    fn read_int(&mut self) -> i32 {
        let pos = self.position;
 
        while self.ch.is_ascii_digit() && self.peek().is_ascii_whitespace() {
            self.read_char();
        }

        //Convert range pos..self.position of Vec<u8> to int
        return String::from_utf8_lossy(&self.input[pos..=self.position]).parse().unwrap();
    }

    fn peek(&self) -> u8 {
        if self.read_position >= self.input.len() {
            return 0;
        }

        return self.input[self.read_position];
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_position];
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }
}
