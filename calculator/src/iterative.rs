// Note that copy-pasted code is intentional in order to compare the different solutions as they evolve

use std::iter::Peekable;
use super::lexer::{ MorphemeContent, Morphemes, SymbolType };
use super::parser_error::Error;
use super::parser_utils::{ eat_one, peek_symbol };

#[cfg(test)]
use super::lexer::Morpheme;

pub fn expression(morphemes: &mut Peekable<Morphemes>) -> Result<f64, Error> {
    let mut accu = term(morphemes)?;

    loop {
        match peek_symbol(morphemes) {
            Some(SymbolType::Add) => {
                eat_one(morphemes);
                accu += term(morphemes)?;
            },
            Some(SymbolType::Subtract) => {
                eat_one(morphemes);
                accu -= term(morphemes)?;
            },
            _ => return Ok(accu)
        }
    }
}

pub fn term(morphemes: &mut Peekable<Morphemes>) -> Result<f64, Error> {
    let mut accu = factor(morphemes)?;

    loop {
        match peek_symbol(morphemes) {
            Some(SymbolType::Multiply) => {
                eat_one(morphemes);
                accu *= factor(morphemes)?;
            },
            Some(SymbolType::Divide) => {
                eat_one(morphemes);
                accu /= factor(morphemes)?;
            },
            _ => return Ok(accu)
        }
    }
}

pub fn factor(morphemes: &mut Peekable<Morphemes>) -> Result<f64, Error> {
    match morphemes.next() {
        Some(morpheme) => match morpheme.content {
            MorphemeContent::Number{ value } => Ok(value),
            MorphemeContent::Symbol{ symbol_type } => match symbol_type {
                SymbolType::RoundOpeningBrace => Ok(expr_factor(morphemes)?),
                _ => Err(Error::UnexpectedMorpheme{morpheme: morpheme})
            },
            _ => Err(Error::UnexpectedMorpheme{morpheme: morpheme})
        },
        None => Err(Error::UnexpectedEOF)
    }
}

fn expr_factor(morphemes: &mut Peekable<Morphemes>) -> Result<f64, Error> {
    let result = expression(morphemes)?;

    match morphemes.next() {
        Some(morpheme) => match morpheme.content {
            MorphemeContent::Symbol{symbol_type: SymbolType::RoundClosingBrace} => Ok(result),
            _ => Err(Error::UnexpectedMorpheme{morpheme: morpheme})
        },
        None => Err(Error::UnexpectedEOF)
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

        ("12/2*3", Ok(18.0)),

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
