mod lexer;
mod parser;

use std::env::args;
use std::iter::Peekable;

struct Command {
    name: String,
    description: String,
    command_fn: fn(String)
}

fn build_commands() -> Vec<Command> {
    vec![
        Command {
            name: "list-morphemes".to_string(),
            description: "Runs only the Lexer and lists all Morphemes in the input".to_string(),
            command_fn: list_morphemes
        },
        Command {
            name: "naive".to_string(),
            description: "Runs the Naive Recursive Descent Parser (G2) and calculates the result".to_string(),
            command_fn: |x| run_parser(parser::naive::expression, x)
        },
        Command {
            name: "iterative".to_string(),
            description: "Runs the Iterative Parser (G3) and calculates the result".to_string(),
            command_fn: |x| run_parser(parser::iterative::expression, x)
        },
        Command {
            name: "split".to_string(),
            description: "Runs the Split Parser (G4) and calculates the result".to_string(),
            command_fn: |x| run_parser(parser::split::expression, x)
        },
    ]
}

fn main() {
    let commands = build_commands();

    match args().nth(1) {
        Some(command_str) => {
            let command = commands.iter().find(|x| x.name == command_str);

            match command {
                Some(c) => {
                    let input = args().skip(2).collect::<Vec<String>>().join(" ");
                    return (c.command_fn)(input);
                },
                None => {
                    println!("Unknown command '{}'\n", command_str);
                    print_usage(&commands);
                }
            }
        },
        None => print_usage(&commands)
    }
}

fn print_usage(commands: &Vec<Command>) {
    println!("Usage: calculator COMMAND INPUT\n");

    println!("Where COMMAND is one of the available commands listed below");
    println!("and INPUT is a mathematical expression.\n");
    
    println!("Available commands:\n");
    commands.iter().for_each(|c| println!("{:25} - {}", c.name, c.description));
}


fn list_morphemes(input: String) {
    let lexer = lexer::Lexer::from_str(&input);
    let morphemes = lexer.morphemes();

    println!("Command Line Input: '{}'\n", input);

    println!("Morphemes");
    println!("=========");

    morphemes.for_each(|m| println!("{:?}", m));
}

fn run_parser(parser: fn(&mut Peekable<lexer::Morphemes>) -> Result<f64, parser::Error>, input: String) {
    let lexer = lexer::Lexer::from_str(&input);
    let morphemes = lexer.morphemes();

    let result = parser(&mut morphemes.peekable());
    match result {
        Ok(v) => println!("{}", v),
        Err(e) => println!("Error {:?}", e)
    }
}
