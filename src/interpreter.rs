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

        // Get a character at the position self.position and decide what token to crconsumee based on the single character.
        let mut current_char = self.text.as_bytes()[self.position] as char;
        match current_char {
            char if char.is_digit(10) => {
                let mut digits = String::new();

                while current_char.is_digit(10) {
                    if self.position == self.text.len() - 1 {
                        digits.push(current_char);
                        break;
                    }
                    digits.push(current_char);
                    self.position += 1;
                    current_char = self.text.as_bytes()[self.position] as char;
                }

                Some(Token {
                    token_type: Integer(digits.parse::<i32>().unwrap()),
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

    fn get_current_token(&self) -> Token {
        return self.clone().current_token.unwrap();
    }

    fn consume(&mut self, token: Token) {
        // Compare the current token type with the passed token type
        // and if the match then "consume" the current token and
        // assign the next token to the self.current_token
        let current_token = self.get_current_token();
        if current_token.token_type == token.token_type {
            self.current_token = self.get_next_token();
        } else {
            panic!("Token error: consume!")
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
        let mut token = self.get_current_token();
        if let Integer(value) = token.token_type {
            left = value;
            self.consume(token);
        }

        // We expect the current token to be a '+' token
        token  = self.get_current_token();
        if token.token_type == Plus {
            operator = "plus";
            self.consume(token);
        } else if token.token_type == Minus {
            operator = "minus";
            self.consume(token);
        }

        // We expect the current token to be a single-digit integer
        let token = self.get_current_token();
        if let Integer(value) = token.token_type {
            right = value;
            self.consume(token);
        }

        // After the above call the self.current_token is set to EOF token

        // At this point INTEGER PLUS INTEGER sequence of tokens has been successfully found and the method
        // can just return the result of adding two integers, thus effectively interpreting client input

        match operator {
            "plus" => left + right,
            "minus" => left - right,
            _ => panic!("Unknown operator!")
        }
    }
}