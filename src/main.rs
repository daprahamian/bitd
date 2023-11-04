use bitd::lexer;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = args[1..].join(" ");

    let result = lexer::lex(&input);
    dbg!("{}", result);
}
