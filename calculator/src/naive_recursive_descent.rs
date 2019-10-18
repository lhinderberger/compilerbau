use std::iter::Peekable;
use super::lexer::{ Morphemes, SymbolType };
use super::parser_common::factor;
use super::parser_error::Error;
use super::parser_utils::{ eat_one, peek_symbol };

#[cfg(test)]
use super::parser_tests::{ yields_expected_results_for_basic_expressions, calculates_right_to_left };

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
    yields_expected_results_for_basic_expressions(expression);
    calculates_right_to_left(expression);
}