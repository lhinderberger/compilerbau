pub mod iterative;
pub mod naive;
pub mod split;

mod common_rules;
mod utils;

#[cfg(test)]
mod tests;


use super::lexer::Morpheme;

#[derive(Debug, PartialEq)]
pub enum Error {
    UnexpectedEOF,
    UnexpectedMorpheme{ morpheme: Morpheme }
}
