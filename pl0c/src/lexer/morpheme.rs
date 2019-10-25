#[derive(Debug, PartialEq)]
pub struct Morpheme {
    pub offset: usize,
    pub length: usize,
    pub content: MorphemeContent
}

#[derive(Debug, PartialEq)]
pub enum MorphemeContent {
    Invalid,
    Number(u64), //TODO: What about other numeric data types? Smaller integers?
    Symbol(SymbolType),
    Identifier(String)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SymbolType {
    Add, Subtract, Multiply, Divide,
    Point, Comma, Semicolon, Pipe, QuestionMark, ExclamationMark, Hash,
    RoundOpeningBrace, RoundClosingBrace,
    Equals, Lesser, LesserOrEqual, Greater, GreaterOrEqual,
    Colon, Assignment,
    Begin, Call, Const, End, Do, If, Procedure, Then, Var, While
}
