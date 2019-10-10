use super::*;

pub fn single_numbers() -> Vec<(&'static str, f64)> {
    vec![
        ("0", 0.0), ("1", 1.0), ("123", 123.0), // Integers
        ("0.0", 0.0), ("1.0", 1.0), ("1.25", 1.25), ("0.625", 0.625), // Floats
        (".625", 0.625), (".875", 0.875), // Floats starting with a dot
    ]
}

pub fn single_symbols() -> Vec<(&'static str, SymbolType)> {
    vec![
        ("+", SymbolType::Add), ("-", SymbolType::Subtract),
        ("*", SymbolType::Multiply), ("/", SymbolType::Divide),
        ("(", SymbolType::RoundOpeningBrace), (")", SymbolType::RoundClosingBrace),
    ]
}

pub fn mixed_input() -> Vec<(&'static str, Vec<Morpheme>)> {
    vec![
        ("1+2", vec![
            Morpheme{ offset: 0, length: 1, content: MorphemeContent::Number { value: 1.0 }},
            Morpheme{ offset: 1, length: 1, content: MorphemeContent::Symbol { symbol_type: SymbolType::Add }},
            Morpheme{ offset: 2, length: 1, content: MorphemeContent::Number { value: 2.0 }}
        ]),
        (".5 *1.25", vec![
            Morpheme{ offset: 0, length: 2, content: MorphemeContent::Number { value: 0.5 }},
            Morpheme{ offset: 3, length: 1, content: MorphemeContent::Symbol { symbol_type: SymbolType::Multiply }},
            Morpheme{ offset: 4, length: 4, content: MorphemeContent::Number { value: 1.25 }}
        ]),
        ("  5-0.625\t ", vec![
            Morpheme{ offset: 2, length: 1, content: MorphemeContent::Number { value: 5.0 }},
            Morpheme{ offset: 3, length: 1, content: MorphemeContent::Symbol { symbol_type: SymbolType::Subtract }},
            Morpheme{ offset: 4, length: 5, content: MorphemeContent::Number { value: 0.625 }}
        ]),
        ("2 4 6 8 +-*/() (1 3 7 9)", vec![
            Morpheme{ offset: 0, length: 1, content: MorphemeContent::Number { value: 2.0 }},
            Morpheme{ offset: 2, length: 1, content: MorphemeContent::Number { value: 4.0 }},
            Morpheme{ offset: 4, length: 1, content: MorphemeContent::Number { value: 6.0 }},
            Morpheme{ offset: 6, length: 1, content: MorphemeContent::Number { value: 8.0 }},
            Morpheme{ offset: 8, length: 1, content: MorphemeContent::Symbol { symbol_type: SymbolType::Add }},
            Morpheme{ offset: 9, length: 1, content: MorphemeContent::Symbol { symbol_type: SymbolType::Subtract }},
            Morpheme{ offset: 10, length: 1, content: MorphemeContent::Symbol { symbol_type: SymbolType::Multiply }},
            Morpheme{ offset: 11, length: 1, content: MorphemeContent::Symbol { symbol_type: SymbolType::Divide }},
            Morpheme{ offset: 12, length: 1, content: MorphemeContent::Symbol { symbol_type: SymbolType::RoundOpeningBrace }},
            Morpheme{ offset: 13, length: 1, content: MorphemeContent::Symbol { symbol_type: SymbolType::RoundClosingBrace }},
            Morpheme{ offset: 15, length: 1, content: MorphemeContent::Symbol { symbol_type: SymbolType::RoundOpeningBrace }},
            Morpheme{ offset: 16, length: 1, content: MorphemeContent::Number { value: 1.0 }},
            Morpheme{ offset: 18, length: 1, content: MorphemeContent::Number { value: 3.0 }},
            Morpheme{ offset: 20, length: 1, content: MorphemeContent::Number { value: 7.0 }},
            Morpheme{ offset: 22, length: 1, content: MorphemeContent::Number { value: 9.0 }},
            Morpheme{ offset: 23, length: 1, content: MorphemeContent::Symbol { symbol_type: SymbolType::RoundClosingBrace }},
        ])
    ]
}

pub fn invalid_input() -> Vec<&'static str> {
    vec![".", "ä", "0..", "1.2.", "1.2.3"]
}
