use std::fmt;
use structopt::StructOpt;

// struct for Cli argument(s)
#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

// Token types
// 
//  EOF (end-of-file) token is used to indicate that
//  there is no more input left for lexical analysis
#[derive(Clone, Debug, PartialEq)]
enum  TokenType {
    Integer(i32),
    Plus,
    EOF
}

use TokenType::*;

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = match self {
            Integer(value) => format!("Integer, {}", value),
            Plus => "Plus".into(),
            EOF => "EOF".into()
        };
        write!(f, "{}", output)
    }
}

#[derive(Clone, Debug)]
struct Token {
    token_type: TokenType,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Token({})", self.token_type)
    }
}

#[derive(Clone)]
struct Interpreter {
    text: String,
    position: usize,
    current_token: Option<Token>,
}

impl Interpreter {
    pub fn new(text: String) -> Self {
        Interpreter {
            // Client string input, e.g. "3+5"
            text,
            // position is an index into self.text
            position: 0,
            // current token instance
            current_token: None,
        }
    }

    pub fn get_next_token(&mut self) -> Option<Token> {
        // Lexical analyzer (also known as scanner or tokenizer)
        // 
        // This method is responsible for breaking a sentence apart into tokens. One token at a time.

        // If self.position index past the end of the self.text ?
        // If so, then return EOF token because there is no more input left to convert into tokens
        if self.position > self.text.len() -1 {
            return Some(Token {token_type: EOF})
        }

        // Get a character at the position self.position and decide what token to create based on the single character.
        let current_char = self.text.as_bytes()[self.position] as char;
        match current_char {
            char if char.is_digit(10) => {
                self.position += 1;
                Some(Token {
                    token_type: Integer(current_char.to_digit(10).unwrap() as i32),
                })
            }

            char if char == '+' => {
                self.position += 1;
                Some(Token {
                    token_type: Plus
                })
            }

            _ => panic!(format!("Invalid token found: {}", current_char))
        }
    }

    fn eat(&mut self, token: Token) {
        // Compare the current token type with the passed token type
        // and if the match then "eat" the current token and
        // assign the next token to the self.current_token
        let current_token = self.clone().current_token.unwrap();
        if current_token.token_type == token.token_type {
            self.current_token = self.get_next_token();
        } else {
            panic!("Token error: eat!")
        }
    }

    fn expr(&mut self) -> i32 {
        // expr -> INTEGER PLUS INTEGER
        // set current token to the first token taken from the input
        self.current_token = self.get_next_token();

        let mut left = 0;
        let mut right = 0;

        // We expect the current token to be a single-digit integer
        let token = self.clone().current_token.unwrap();
        if let Integer(value) = token.token_type {
            left = value;
            self.eat(token);
        }

        // We expect the current token to be a '+' token
        let token  = self.clone().current_token.unwrap();
        if token.token_type == Plus {
            self.eat(token);
        }

        // We expect the current token to be a single-digit integer
        let token = self.clone().current_token.unwrap();
        if let Integer(value) = token.token_type {
            right = value;
            self.eat(token);
        }

        // After the above call the self.current_token is set to EOF token

        // At this point INTEGER PLUS INTEGER sequence of tokens has been successfully found and the method
        // can just return the result of adding two integers, thus effectively interpreting client input

        left + right
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();

    let content = std::fs::read_to_string(&args.path)?;

    let mut interpreter = Interpreter::new(content.into());
    let result = interpreter.expr();

    println!("{}", result);

    Ok(())
}
