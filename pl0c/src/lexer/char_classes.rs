use super::char_classes_vector::ASCII_CHAR_CLASSES;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum CharClass {
    Number,
    Letter,
    Symbol(SymbolCharType),
    Whitespace,
    Other
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum SymbolCharType {
    Singular,
    CompositePrefix,
    CompositeSuffix
}

impl CharClass {
    pub fn from(c: char) -> Self {
        if c.is_ascii() {
            ASCII_CHAR_CLASSES[(c as u8) as usize]
        }
        else if c.is_alphabetic() {
            Self::Letter
        }
        else {
            Self::Other
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn correct_class_is_detected() {
        let mut test_data = HashMap::new();
        test_data.insert(CharClass::Letter, vec![
            'a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z',
            'A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z',
            'ä','ö','ü','Ä','Ö','Ü','á','é','à','è','â','ê','Á','É','À','È','Â','Ê','ß'
        ]);
        test_data.insert(CharClass::Number, vec!['0','1','2','3','4','5','6','7','8','9']);
        test_data.insert(CharClass::Symbol(SymbolCharType::CompositePrefix), vec![':', '<', '>']);
        test_data.insert(CharClass::Symbol(SymbolCharType::CompositeSuffix), vec!['=']);
        test_data.insert(CharClass::Symbol(SymbolCharType::Singular), vec!['+','-','*','/',',','.',';','(',')','?','!','#','|']);
        test_data.insert(CharClass::Whitespace, vec![' ', '\t', '\n', '\r']);
        test_data.insert(CharClass::Other, vec!['%','§','$','~','½','²','³']);

        for (char_class,chars) in test_data {
            for c in chars {
                assert_eq!(char_class, CharClass::from(c));
            }
        }
    }
}