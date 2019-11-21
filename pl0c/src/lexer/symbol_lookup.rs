use std::collections::HashMap;
use super::SymbolType;

pub struct SymbolLookup {
    symbols_map: HashMap<&'static str, SymbolType>
}


impl SymbolLookup {
    pub fn new() -> Self {
        Self {
            symbols_map: [
                ("+", SymbolType::Add), ("-", SymbolType::Subtract),
                ("*", SymbolType::Multiply), ("/", SymbolType::Divide),
                (".", SymbolType::Point), (";", SymbolType::Semicolon),
                ("|", SymbolType::Pipe), ("?", SymbolType::QuestionMark),
                ("!", SymbolType::ExclamationMark), ("%", SymbolType::Hash),
                ("(", SymbolType::RoundOpeningBrace), (")", SymbolType::RoundClosingBrace),
                ("=", SymbolType::Equals), ("<", SymbolType::Lesser),
                ("<=", SymbolType::LesserOrEqual), (">", SymbolType::Greater),
                (">=", SymbolType::GreaterOrEqual), (":", SymbolType::Colon),
                (":=", SymbolType::Assignment), ("BEGIN", SymbolType::Begin),
                ("CALL", SymbolType::Call), ("CONST", SymbolType::Const),
                ("END", SymbolType::End), ("DO", SymbolType::Do),
                ("IF", SymbolType::If), ("PROCEDURE", SymbolType::Procedure),
                ("THEN", SymbolType::Then), ("VAR", SymbolType::Var),
                ("WHILE", SymbolType::While), (",", SymbolType::Comma),
                ("ODD", SymbolType::Odd)
            ].iter().cloned().collect()
        }
    }

    pub fn lookup(&self, s: &str) -> Option<SymbolType> {
        self.symbols_map.get(s).map(|x| *x)
    }
}