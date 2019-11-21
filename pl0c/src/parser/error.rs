use super::super::lexer::Morpheme;
use super::syntax_graph::GraphLocation;

#[derive(Debug)]
pub enum Error {
    EOF,
    InvalidMorpheme(Morpheme),
    Semantic(SemanticError),
    Syntax{ location: GraphLocation, next_morpheme: Morpheme }
}

#[derive(Debug)]
pub enum SemanticError {

}

impl From<SemanticError> for Error {
    fn from(e: SemanticError) -> Error {
        Error::Semantic(e)
    }
}
