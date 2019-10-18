use std::iter::Peekable;
use super::super::lexer::{ Morphemes, MorphemeContent, SymbolType };

pub fn eat_one(morphemes: &mut Peekable<Morphemes>) {
    let _ = morphemes.next().unwrap();
}

pub fn peek_symbol(morphemes: &mut Peekable<Morphemes>) -> Option<SymbolType> {
    match morphemes.peek() {
        None => None,
        Some(morpheme) => match morpheme.content {
            MorphemeContent::Symbol{ symbol_type } => Some(symbol_type),
            _ => None
        }
    }
}