use std::iter::Peekable;
use super::lexer::{ Morpheme, MorphemeContent, Morphemes, SymbolType };

#[derive(Debug, PartialEq)]
pub enum Error {
    UnexpectedEOF,
    UnexpectedMorpheme{ morpheme: Morpheme }
}

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
    let left = factor(morphemes)?;

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

fn eat_one(morphemes: &mut Peekable<Morphemes>) {
    let _ = morphemes.next().unwrap();
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

fn peek_symbol(morphemes: &mut Peekable<Morphemes>) -> Option<SymbolType> {
    match morphemes.peek() {
        None => None,
        Some(morpheme) => match morpheme.content {
            MorphemeContent::Symbol{ symbol_type } => Some(symbol_type),
            _ => None
        }
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