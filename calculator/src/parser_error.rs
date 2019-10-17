use super::lexer::Morpheme;

#[derive(Debug, PartialEq)]
pub enum Error {
    UnexpectedEOF,
    UnexpectedMorpheme{ morpheme: Morpheme }
}
