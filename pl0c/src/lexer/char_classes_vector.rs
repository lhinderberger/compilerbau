use super::*;

const C1:  CharClass = CharClass::Letter;
const C2:  CharClass = CharClass::Number;
const C3:  CharClass = CharClass::Whitespace;
const C41: CharClass = CharClass::Symbol(SymbolCharType::Colon);
const C42: CharClass = CharClass::Symbol(SymbolCharType::Equals);
const C43: CharClass = CharClass::Symbol(SymbolCharType::Lesser);
const C44: CharClass = CharClass::Symbol(SymbolCharType::Greater);
const C49: CharClass = CharClass::Symbol(SymbolCharType::Other);
const C9:  CharClass = CharClass::Other;


pub const ASCII_CHAR_CLASSES: [CharClass;128] = [
    C9,  C9,  C9,  C9,  C9,  C9,  C9,  C9,  C9,  C3,  C3,  C9,  C9,  C3,  C9,  C9,
    C9,  C9,  C9,  C9,  C9,  C9,  C9,  C9,  C9,  C9,  C9,  C9,  C9,  C9,  C9,  C9,
    C3, C49,  C9, C49,  C9,  C9,  C9,  C9, C49, C49, C49, C49, C49, C49, C49, C49,
    C2,  C2,  C2,  C2,  C2,  C2,  C2,  C2,  C2,  C2, C41, C49, C43, C42, C44, C49,
    C9,  C1,  C1,  C1,  C1,  C1,  C1,  C1,  C1,  C1,  C1,  C1,  C1,  C1,  C1,  C1,
    C1,  C1,  C1,  C1,  C1,  C1,  C1,  C1,  C1,  C1,  C1,  C9,  C9,  C9,  C9,  C9,
    C9,  C1,  C1,  C1,  C1,  C1,  C1,  C1,  C1,  C1,  C1,  C1,  C1,  C1,  C1,  C1,
    C1,  C1,  C1,  C1,  C1,  C1,  C1,  C1,  C1,  C1,  C1,  C9, C49,  C9,  C9,  C9
];