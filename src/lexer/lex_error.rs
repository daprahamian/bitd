use std::fmt;

#[derive(Debug)]
pub struct LexError {
    message: String,
}

impl LexError {
    pub fn new(message: &str) -> LexError {
        LexError {
            message: message.to_string(),
        }
    }

    pub fn dice_expression(expr: &str) -> LexError {
        LexError {
            message: format!("Could not lex Dice Expression \"{}\"", expr),
        }
    }

    pub fn parse_number(expr: &str) -> LexError {
        LexError {
            message: format!("Failed to parse \"{}\" into number", expr),
        }
    }
}

impl std::fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
