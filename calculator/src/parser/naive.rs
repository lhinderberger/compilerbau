use std::iter::Peekable;

use super::Error;
use super::common_rules::factor;
use super::utils::{ eat_one, peek_symbol };
use super::super::lexer::{ Morphemes, SymbolType };

#[cfg(test)]
use super::tests::{ yields_expected_results_for_basic_expressions, calculates_right_to_left };

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