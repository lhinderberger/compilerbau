use std::iter::Peekable;

use super::Error;
use super::common_rules::factor;
use super::utils::{ eat_one, peek_symbol };

use super::super::lexer::{ Morphemes, SymbolType };

#[cfg(test)]
use super::tests::{ yields_expected_results_for_basic_expressions, calculates_left_to_right };

pub fn expression(morphemes: &mut Peekable<Morphemes>) -> Result<f64, Error> {
    let right = term(morphemes)?;
    rexpr(morphemes, right)
}

fn rexpr(morphemes: &mut Peekable<Morphemes>, accu: f64) -> Result<f64, Error> {
    let symbol = peek_symbol(morphemes);
    let right = match symbol {
        Some(SymbolType::Add) | Some(SymbolType::Subtract) => {
            eat_one(morphemes);
            term(morphemes)?
        }
        _ => 0.0
    };

    match symbol {
        Some(SymbolType::Add) => rexpr(morphemes, accu + right),
        Some(SymbolType::Subtract) => rexpr(morphemes, accu - right),
        _ => Ok(accu)
    }
}

pub fn term(morphemes: &mut Peekable<Morphemes>) -> Result<f64, Error> {
    let right = factor(morphemes, expression)?;
    rterm(morphemes, right)
}

fn rterm(morphemes: &mut Peekable<Morphemes>, accu: f64) -> Result<f64, Error> {
    let symbol = peek_symbol(morphemes);
    let right = match symbol {
        Some(SymbolType::Multiply) | Some(SymbolType::Divide) => {
            eat_one(morphemes);
            factor(morphemes, expression)?
        }
        _ => 0.0
    };

    match symbol {
        Some(SymbolType::Multiply) => rterm(morphemes, accu * right),
        Some(SymbolType::Divide) => rterm(morphemes, accu / right),
        _ => Ok(accu)
    }
}


#[cfg(test)]
#[test]
fn yields_expected_results() {
    yields_expected_results_for_basic_expressions(expression);
    calculates_left_to_right(expression);
}
