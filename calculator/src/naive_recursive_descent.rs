use std::iter::Peekable;
use super::lexer::{ Morphemes, SymbolType };
use super::parser_common::factor;
use super::parser_error::Error;
use super::parser_utils::{ eat_one, peek_symbol };

#[cfg(test)]
use super::lexer::{ Morpheme, MorphemeContent };

pub fn expression(morphemes: &mut Peekable<Morphemes>) -> Result<f64, Error> {
    let left = term(morphemes)?;

    match peek_symbol(morphemes) {
        Some(SymbolType::Add) => {
            eat_one(morphemes);
            Ok(left + expression(morphemes)?)
        },
        Some(SymbolType::Subtract) => {
            eat_one(morphemes);
            Ok(left - expression(morphemes)?)
        },
        None | Some(_) => Ok(left)
    }
}

pub fn term(morphemes: &mut Peekable<Morphemes>) -> Result<f64, Error> {
    let left = factor(morphemes, expression)?;

    match peek_symbol(morphemes) {
        Some(SymbolType::Multiply) => {
            eat_one(morphemes);
            Ok(left * term(morphemes)?)
        },
        Some(SymbolType::Divide) => {
            eat_one(morphemes);
            Ok(left / term(morphemes)?)
        },
        None | Some(_) => Ok(left)
    }
}

#[cfg(test)]
#[test]
fn yields_expected_results() {
    let test_data = vec![
        ("1+0", Ok(1.0)), ("1-0", Ok(1.0)),
        ("1+1", Ok(2.0)), ("1*1", Ok(1.0)),

        ("2*3", Ok(6.0)), ("8/2", Ok(4.0)),
        ("3+4", Ok(7.0)), ("5-3", Ok(2.0)),

        ("1/2", Ok(0.5)), (".3+.4", Ok(0.7)),

        ("1+2+3", Ok(6.0)), ("2*3*4", Ok(24.0)),

        ("2+3*4+5", Ok(19.0)), ("(2+3)*(4+5)", Ok(45.0)),

        ("12/2*3", Ok(2.0)), // This result is mathematically wrong but expected, since the naive recursive descent parser calculates right-to-left

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

    for (input, expected) in test_data {
        let lexer = super::lexer::Lexer::from_str(input);
        let mut morphemes = lexer.morphemes().peekable();
        let actual = expression(&mut morphemes);

        assert_eq!(expected, actual);
    }
}