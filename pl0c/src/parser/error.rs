use super::super::semantics;
use super::super::lexer::Morpheme;
use super::GraphLocation;

#[derive(Debug)]
pub enum Error {
    EOF,
    InvalidMorpheme(Morpheme),
    Semantic(semantics::Error),
    Syntax{ location: GraphLocation, next_morpheme: Morpheme }
}

impl From<semantics::Error> for Error {
    fn from(e: semantics::Error) -> Error {
        Error::Semantic(e)
    }
}
