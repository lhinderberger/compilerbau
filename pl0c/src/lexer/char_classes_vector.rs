use super::*;

const C1:  CharClass = CharClass::Letter;
const C2:  CharClass = CharClass::Number;
const C3:  CharClass = CharClass::Whitespace;
const C41: CharClass = CharClass::Symbol(SymbolCharType::CompositePrefix);
const C42: CharClass = CharClass::Symbol(SymbolCharType::CompositeSuffix);
const C43: CharClass = CharClass::Symbol(SymbolCharType::Singular);
const C9:  CharClass = CharClass::Other;


pub const ASCII_CHAR_CLASSES: [CharClass;128] = [
    C9,  C9,  C9,  C9,  C9,  C9,  C9,  C9,  C9,  C3,  C3,  C9,  C9,  C3,  C9,  C9,
    C9,  C9,  C9,  C9,  C9,  C9,  C9,  C9,  C9,  C9,  C9,  C9,  C9,  C9,  C9,  C9,
    C3, C43,  C9, C43,  C9,  C9,  C9,  C9, C43, C43, C43, C43, C43, C43, C43, C43,
    C2,  C2,  C2,  C2,  C2,  C2,  C2,  C2,  C2,  C2, C41, C43, C41, C42, C41, C43,
    C9,  C1,  C1,  C1,  C1,  C1,  C1,  C1,  C1,  C1,  C1,  C1,  C1,  C1,  C1,  C1,
    C1,  C1,  C1,  C1,  C1,  C1,  C1,  C1,  C1,  C1,  C1,  C9,  C9,  C9,  C9,  C9,
    C9,  C1,  C1,  C1,  C1,  C1,  C1,  C1,  C1,  C1,  C1,  C1,  C1,  C1,  C1,  C1,
    C1,  C1,  C1,  C1,  C1,  C1,  C1,  C1,  C1,  C1,  C1,  C9, C43,  C9,  C9,  C9
];