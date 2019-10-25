#[derive(Debug, PartialEq)]
pub struct Morpheme {
    pub offset: usize,
    pub length: usize,
    pub content: MorphemeContent
}

#[derive(Debug, PartialEq)]
pub enum MorphemeContent {
    Invalid,
    Number {value: f64},
    Symbol {symbol_type: SymbolType},
    Identifier {value: String}
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SymbolType {
    Add, Subtract, Multiply, Divide,
    Point, Semicolon, Pipe, QuestionMark, ExclamationMark, Hash,
    RoundOpeningBrace, RoundClosingBrace,
    Equals, Lesser, LesserOrEqual, Greater, GreaterOrEqual,
    Assignment,
    Begin, Call, Const, End, Do, If, Procedure, Then, Var, While
}
