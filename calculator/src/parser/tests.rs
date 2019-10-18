use std::iter::Peekable;

use super::Error;
use super::super::lexer::{ Lexer, Morpheme, MorphemeContent, Morphemes, SymbolType };

pub fn yields_expected_results_for_basic_expressions(expression_fn: fn(&mut Peekable<Morphemes>) -> Result<f64, Error>) {
    let test_data = vec![
        ("1+0", Ok(1.0)), ("1-0", Ok(1.0)),
        ("1+1", Ok(2.0)), ("1*1", Ok(1.0)),

        ("2*3", Ok(6.0)), ("8/2", Ok(4.0)),
        ("3+4", Ok(7.0)), ("5-3", Ok(2.0)),

        ("1/2", Ok(0.5)), (".3+.4", Ok(0.7)),

        ("1+2+3", Ok(6.0)), ("2*3*4", Ok(24.0)),

        ("2+3*4+5", Ok(19.0)), ("(2+3)*(4+5)", Ok(45.0)),

        ("1+", Err(Error::UnexpectedEOF)),
        ("2*3*", Err(Error::UnexpectedEOF)),
        ("(1+1", Err(Error::UnexpectedEOF)),

        ("1++", Err(Error::UnexpectedMorpheme{
            morpheme: Morpheme{
                offset: 2,
                length: 1,
                content: MorphemeContent::Symbol{ symbol_type: SymbolType::Add }
            }
        })),
        ("(1+*", Err(Error::UnexpectedMorpheme{
            morpheme: Morpheme{
                offset: 3,
                length: 1,
                content: MorphemeContent::Symbol{ symbol_type: SymbolType::Multiply }
            }
        }))
    ];

    run_tests(test_data, expression_fn);
}

pub fn calculates_right_to_left(expression_fn: fn(&mut Peekable<Morphemes>) -> Result<f64, Error>) {
    // These results are mathematically wrong but expected, since the parser calculates right-to-left
    let test_data = vec![
        ("12/2*3", Ok(2.0)),
        ("12/3*2", Ok(2.0)),
    ];

    run_tests(test_data, expression_fn);
}

pub fn calculates_left_to_right(expression_fn: fn(&mut Peekable<Morphemes>) -> Result<f64, Error>) {
    let test_data = vec![
        ("12/2*3", Ok(18.0)),
        ("12/3*2", Ok(8.0)),
    ];

    run_tests(test_data, expression_fn);
}

fn run_tests(test_data: Vec<(&'static str, Result<f64, Error>)>, expression_fn: fn(&mut Peekable<Morphemes>) -> Result<f64, Error>) {
    for (input, expected) in test_data {
        let lexer = Lexer::from_str(input);
        let mut morphemes = lexer.morphemes().peekable();
        let actual = expression_fn(&mut morphemes);

        assert_eq!(expected, actual);
    }
}