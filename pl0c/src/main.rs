//TODO: Add Integration tests

#[macro_use]
extern crate lazy_static;

mod lexer;
mod parser;
mod semantics;

use std::env::args;
use std::fs::File;
use std::io::{ Read, Write, stdin, stdout };


const STDIN_FILENAME : &'static str = "stdin";
const STDOUT_FILENAME: &'static str = "stdout";


struct Command {
    name: String,
    description: String,
    command_fn: fn(&mut dyn Read, &mut dyn Write)
}

fn build_commands() -> Vec<Command> {
    vec![
        Command {
            name: "list-morphemes".to_string(),
            description: "Runs only the Lexer and lists all Morphemes in the input".to_string(),
            command_fn: list_morphemes
        },
        Command {
            name: "verify-syntax".to_string(),
            description: "Runs the parser without semantics, just to verify the syntax".to_string(),
            command_fn: verify_syntax
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
                    let input_filename = args().nth(2).unwrap_or(STDIN_FILENAME.to_string());
                    let output_filename = args().nth(3).unwrap_or(STDOUT_FILENAME.to_string());

                    let mut input: Box<dyn Read> = match input_filename.as_ref() {
                        STDIN_FILENAME => Box::new(stdin()),
                        _ => Box::new(File::open(input_filename).unwrap())
                    };

                    let mut output: Box<dyn Write> = match output_filename.as_ref() {
                        STDOUT_FILENAME => Box::new(stdout()),
                        _ => Box::new(File::create(output_filename).unwrap())
                    };

                    return (c.command_fn)(input.as_mut(), output.as_mut());
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
    println!("Usage: pl0c COMMAND [INPUT_FILENAME] [OUTPUT_FILENAME]\n");

    println!("Where COMMAND is one of the available commands listed below,");
    println!("INPUT_FILENAME is the name of the PL0 source file to read (stdin if omitted),");
    println!("and OUTPUT_FILENAME is the name of the file to write the output to (stdout if omitted).\n");
    
    println!("Available commands:\n");
    commands.iter().for_each(|c| println!("{:15} - {}", c.name, c.description));
}


fn list_morphemes(input: &mut dyn Read, output: &mut dyn Write) {
    let sourcecode = read_all(input);

    let lexer = lexer::Lexer::from_str(&sourcecode);
    let morphemes = lexer.morphemes();

    morphemes.for_each(|m| writeln!(output, "{:?}", m).unwrap());
}

fn run_parser<T: parser::VertexTookObserver>(input: &mut dyn Read, semantics: &mut T) -> Result<(),parser::Error> {
    let sourcecode = read_all(input);

    let lexer = lexer::Lexer::from_str(&sourcecode);
    let mut parser = parser::Parser::new(lexer.morphemes().peekable(), semantics);

    parser.parse()
}

fn verify_syntax(input: &mut dyn Read, output: &mut dyn Write) {
    let result = run_parser(input, &mut semantics::NopSemantics{});
    writeln!(output, "{:?}", result).unwrap();
}

fn read_all(input: &mut dyn Read) -> String {
    let mut buffer = String::new();
    input.read_to_string(&mut buffer).unwrap();

    buffer
}
