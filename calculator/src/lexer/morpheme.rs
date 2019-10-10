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
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SymbolType { Add, Subtract, Multiply, Divide, RoundOpeningBrace, RoundClosingBrace }
