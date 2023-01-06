use bitd::lexer::roll_dice;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = args[1..].join(" ");

    let result = roll_dice(&input);
    dbg!("{}", result);
}
