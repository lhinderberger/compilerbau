// Common parser rules

use std::iter::Peekable;
use super::lexer::{ MorphemeContent, Morphemes, SymbolType };
use super::parser_error::Error;

// Rule for parsing Factors, which is identical for the naive, iterative and split parsers
// It takes in the usual peekable morpheme iterator as well as a pointer to the function for parsing expressions
// (for the recursion that occurs when parsing a braced exception)
pub fn factor(morphemes: &mut Peekable<Morphemes>, expression_fn: fn(&mut Peekable<Morphemes>) -> Result<f64, Error>) -> Result<f64, Error> {
    match morphemes.next() {
        Some(morpheme) => match morpheme.content {
            MorphemeContent::Number{ value } => Ok(value),
            MorphemeContent::Symbol{ symbol_type } => match symbol_type {
                SymbolType::RoundOpeningBrace => Ok(expr_factor(morphemes, expression_fn)?),
                _ => Err(Error::UnexpectedMorpheme{morpheme: morpheme})
            },
            _ => Err(Error::UnexpectedMorpheme{morpheme: morpheme})
        },
        None => Err(Error::UnexpectedEOF)
    }
}

fn expr_factor(morphemes: &mut Peekable<Morphemes>, expression_fn: fn(&mut Peekable<Morphemes>) -> Result<f64, Error>) -> Result<f64, Error> {
    let result = expression_fn(morphemes)?;

    match morphemes.next() {
        Some(morpheme) => match morpheme.content {
            MorphemeContent::Symbol{symbol_type: SymbolType::RoundClosingBrace} => Ok(result),
            _ => Err(Error::UnexpectedMorpheme{morpheme: morpheme})
        },
        None => Err(Error::UnexpectedEOF)
    }
}