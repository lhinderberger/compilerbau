mod lexer;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = args[1..].join(" ");

    let lexer = lexer::Lexer::from_str(&input);


    println!("Command Line Input: '{}'\n", input);

    println!("Morphemes");
    println!("=========");

    lexer.morphemes().for_each(|m| println!("{:?}", m));
}
