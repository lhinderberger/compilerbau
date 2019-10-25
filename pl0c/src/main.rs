mod lexer;

fn main() {
    println!("Hello, world!");

    lexer::Lexer::from_str("asdf").next_morpheme(); // To stop compiler warnings about "unused functions" until there is a CLI
}
