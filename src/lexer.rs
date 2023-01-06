use regex::Regex;
use std::fmt;
use std::fmt::Debug;

#[derive(Debug)]
struct PositionInformation {}

#[derive(Debug)]
pub enum OperatorType {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
pub enum TokenType {
    DiceExpression,
    Constant,
    Whitespace,
    Newline,
    Operator(OperatorType),
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    raw_text: String,
    pos: PositionInformation,
}

impl Token {
    pub fn new(token_type: TokenType, raw_text: &str) -> Token {
        Token {
            token_type,
            raw_text: raw_text.to_string(),
            pos: PositionInformation {},
        }
    }
}

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
}

impl std::fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub fn roll_dice(dice_expression: &str) -> Result<Vec<Token>, LexError> {
    let cpy = dice_expression.to_string();
    let mut text = &cpy[..];
    // let mut line: u32 = 0;
    // let mut col: u32 = 0;
    let mut tokens: Vec<Token> = Vec::new();

    let DICE_EXPRESSION: Regex = Regex::new(r"^(\d+)?d\d+([A-Za-z][A-Za-z0-9]*)?").unwrap();
    let CONSTANT: Regex = Regex::new(r"^\d+").unwrap();
    let OPERATOR: Regex = Regex::new(r"^[\+\-\*/]").unwrap();
    let WHITESPACE: Regex = Regex::new(r"^\s+").unwrap();
    let UNKNOWN: Regex = Regex::new(r"\S+").unwrap();

    loop {
        if text.len() == 0 {
            break;
        }
        if let Some(m) = DICE_EXPRESSION.find(&text) {
            let str = m.as_str().to_string();
            text = &text[str.len()..];
            tokens.push(Token::new(TokenType::DiceExpression, &str));
        } else if let Some(m) = CONSTANT.find(&text) {
            let str = m.as_str().to_string();
            text = &text[str.len()..];
            tokens.push(Token::new(TokenType::Constant, &str));
        } else if let Some(m) = OPERATOR.find(&text) {
            let str = m.as_str().to_string();
            text = &text[str.len()..];
            let op_type = match &str[..] {
                "+" => OperatorType::Add,
                "-" => OperatorType::Subtract,
                "*" => OperatorType::Multiply,
                "/" => OperatorType::Divide,
                _ => return Err(LexError::new(&format!("Unknown Operator \"{}\"", &str))),
            };
            tokens.push(Token::new(TokenType::Operator(op_type), &str));
        } else if let Some(m) = WHITESPACE.find(&text) {
            let str = m.as_str().to_string();
            text = &text[str.len()..];
            tokens.push(Token::new(TokenType::Whitespace, &str));
        } else if let Some(m) = UNKNOWN.find(&text) {
            let message = format!("Unknown token \"{}\"", m.as_str());
            return Err(LexError::new(&message));
        } else {
            panic!("How did you get here?")
        };
    }
    Ok(tokens)
}
