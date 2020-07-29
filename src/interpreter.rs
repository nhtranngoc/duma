use crate::token::Token;
use crate::token::TokenType::*;

#[derive(Clone)]
pub struct Interpreter {
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

    fn get_next_token(&mut self) -> Option<Token> {
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

            ' ' => {
                self.position += 1;
                return self.get_next_token();
            }

            '+' => {
                self.position += 1;
                Some(Token {
                    token_type: Plus
                })
            }

            '-' => {
                self.position += 1;
                Some(Token {
                    token_type: Minus,
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

    pub fn expr(&mut self) -> i32 {
        // expr -> INTEGER PLUS INTEGER
        // set current token to the first token taken from the input
        self.current_token = self.get_next_token();

        let mut left = 0;
        let mut right = 0;
        let mut operator = "unknown";

        // We expect the current token to be a single-digit integer
        let token = self.clone().current_token.unwrap();
        if let Integer(value) = token.token_type {
            left = value;
            self.eat(token);
        }

        // We expect the current token to be a '+' token
        let token  = self.clone().current_token.unwrap();
        if token.token_type == Plus {
            operator = "plus";
            self.eat(token);
        } else if token.token_type == Minus {
            operator = "minus";
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

        match operator.as_ref() {
            "plus" => left + right,
            "minus" => left - right,
            _ => panic!("Unknown operator!")
        }
    }
}