use crate::lexer::lex_error::LexError;
use lazy_static::lazy_static;
use logos::{Lexer, Logos};
use regex::Regex;
use std::fmt::Debug;

pub mod lex_error;

#[derive(Debug)]
pub enum OperatorType {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
pub struct DiceExpressionData {
    dice_quantity: u64,
    dice_size: u64,
    modifiers: String,
}

fn parse_dice_expression(lex: &mut Lexer<Token>) -> Result<DiceExpressionData, LexError> {
    lazy_static! {
        static ref PATTERN: Regex = Regex::new(r"(\d+?)d(\d+)([A-Za-z][A-Za-z0-9]*)?").unwrap();
    };
    let slice = lex.slice();
    let m = PATTERN
        .captures(slice)
        .ok_or_else(|| LexError::dice_expression(slice))?;

    let dice_quantity: u64 = match m.get(1) {
        Some(txt) => txt
            .as_str()
            .parse::<u64>()
            .map_err(|_err| LexError::parse_number(txt.as_str()))?,
        None => 1,
    };
    let dice_size: u64 = match m.get(2) {
        Some(txt) => txt
            .as_str()
            .parse::<u64>()
            .map_err(|_err| LexError::parse_number(txt.as_str()))?,
        None => 1,
    };
    let modifiers: String = match m.get(3) {
        Some(txt) => txt.as_str().to_owned(),
        None => String::new(),
    };

    Ok(DiceExpressionData {
        dice_quantity,
        dice_size,
        modifiers,
    })
}

fn parse_dice_operator(lex: &mut Lexer<Token>) -> Result<OperatorType, LexError> {
    let op_type: OperatorType = match lex.slice() {
        "+" => OperatorType::Add,
        "-" => OperatorType::Subtract,
        "*" => OperatorType::Multiply,
        "/" => OperatorType::Divide,
        op => return Err(LexError::new(&format!("Unknown Operator \"{}\"", op))),
    };

    Ok(op_type)
}

#[derive(Logos, Debug)]
pub enum Token {
    #[regex(r"(\d+)?d\d+([A-Za-z][A-Za-z0-9]*)?", parse_dice_expression)]
    DiceExpression(DiceExpressionData),

    #[regex(r"\d+", |lex| lex.slice().parse())]
    Constant(u64),

    #[regex(r"\s+", logos::skip)]
    Whitespace,

    #[regex(r"[\+\-\*/]", parse_dice_operator)]
    Operator(OperatorType),

    #[error]
    Error,
}

pub fn lex(dice_expression: &str) -> Result<Vec<Token>, LexError> {
    dbg!(dice_expression);
    let lex = Token::lexer(dice_expression);
    let mut tokens: Vec<Token> = Vec::new();

    for token in lex {
        match token {
            Token::Error => {
                dbg!(token);
                return Err(LexError::new("Lexer Error Occurred"));
            }
            n => tokens.push(n),
        }
    }
    Ok(tokens)
}
