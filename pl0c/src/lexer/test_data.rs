use super::*;

pub fn eof_input() -> Vec<&'static str> {
    vec![""," ","\t","\n","\t \n\t \r\n\t  "]
}

pub fn single_morphemes() -> Vec<(&'static str, MorphemeContent)> {
    vec![
        ("+", MorphemeContent::Symbol(SymbolType::Add)),
        (":", MorphemeContent::Symbol(SymbolType::Colon)),
        (":=", MorphemeContent::Symbol(SymbolType::Assignment)),
        ("<", MorphemeContent::Symbol(SymbolType::Lesser)),
        ("<=", MorphemeContent::Symbol(SymbolType::LesserOrEqual)),
        (">", MorphemeContent::Symbol(SymbolType::Greater)),
        (">=", MorphemeContent::Symbol(SymbolType::GreaterOrEqual)),
        ("BEGIN", MorphemeContent::Symbol(SymbolType::Begin)),
        ("Begin", MorphemeContent::Symbol(SymbolType::Begin)),
        ("BEGINNING", MorphemeContent::Identifier("BEGINNING".to_string())),
        ("Beginning", MorphemeContent::Identifier("BEGINNING".to_string())),
        ("Beginning234", MorphemeContent::Identifier("BEGINNING234".to_string())),
        ("234", MorphemeContent::Number(234)),
        ("~", MorphemeContent::Invalid)
    ]
}

pub fn mixed_input() -> Vec<(&'static str, Vec<Morpheme>)> {
    vec![
        ("five := 2+3", vec![
            Morpheme{ offset: 0, length: 4, content: MorphemeContent::Identifier("FIVE".to_string()) },
            Morpheme{ offset: 5, length: 2, content: MorphemeContent::Symbol(SymbolType::Assignment) },
            Morpheme{ offset: 8, length: 1, content: MorphemeContent::Number(2) },
            Morpheme{ offset: 9, length: 1, content: MorphemeContent::Symbol(SymbolType::Add) },
            Morpheme{ offset: 10, length: 1, content: MorphemeContent::Number(3) }
        ]),
        ("  \n\t  var a,b,c\nPROCEDURE foo;\nBEGIN\n\ta := b + c\nend; \t \n ", vec![
            Morpheme{ offset: 6, length: 3, content: MorphemeContent::Symbol(SymbolType::Var) },
            Morpheme{ offset: 10, length: 1, content: MorphemeContent::Identifier("A".to_string()) },
            Morpheme{ offset: 11, length: 1, content: MorphemeContent::Symbol(SymbolType::Comma) },
            Morpheme{ offset: 12, length: 1, content: MorphemeContent::Identifier("B".to_string()) },
            Morpheme{ offset: 13, length: 1, content: MorphemeContent::Symbol(SymbolType::Comma) },
            Morpheme{ offset: 14, length: 1, content: MorphemeContent::Identifier("C".to_string()) },
            Morpheme{ offset: 16, length: 9, content: MorphemeContent::Symbol(SymbolType::Procedure) },
            Morpheme{ offset: 26, length: 3, content: MorphemeContent::Identifier("FOO".to_string()) },
            Morpheme{ offset: 29, length: 1, content: MorphemeContent::Symbol(SymbolType::Semicolon) },
            Morpheme{ offset: 31, length: 5, content: MorphemeContent::Symbol(SymbolType::Begin) },
            Morpheme{ offset: 38, length: 1, content: MorphemeContent::Identifier("A".to_string()) },
            Morpheme{ offset: 40, length: 2, content: MorphemeContent::Symbol(SymbolType::Assignment) },
            Morpheme{ offset: 43, length: 1, content: MorphemeContent::Identifier("B".to_string()) },
            Morpheme{ offset: 45, length: 1, content: MorphemeContent::Symbol(SymbolType::Add) },
            Morpheme{ offset: 47, length: 1, content: MorphemeContent::Identifier("C".to_string()) },
            Morpheme{ offset: 49, length: 3, content: MorphemeContent::Symbol(SymbolType::End) },
            Morpheme{ offset: 52, length: 1, content: MorphemeContent::Symbol(SymbolType::Semicolon) }
        ])
    ]
}