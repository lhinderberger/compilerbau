use std::iter::Peekable;
use super::lexer::{ Morphemes, SymbolType };
use super::parser_common::factor;
use super::parser_error::Error;
use super::parser_utils::{ eat_one, peek_symbol };

#[cfg(test)]
use super::parser_tests::{ yields_expected_results_for_basic_expressions, calculates_left_to_right };

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
    let mut accu = factor(morphemes, expression)?;

    loop {
        match peek_symbol(morphemes) {
            Some(SymbolType::Multiply) => {
                eat_one(morphemes);
                accu *= factor(morphemes, expression)?;
            },
            Some(SymbolType::Divide) => {
                eat_one(morphemes);
                accu /= factor(morphemes, expression)?;
            },
            _ => return Ok(accu)
        }
    }
}


#[cfg(test)]
#[test]
fn yields_expected_results() {
    yields_expected_results_for_basic_expressions(expression);
    calculates_left_to_right(expression);
}
