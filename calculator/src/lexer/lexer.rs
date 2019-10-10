use std::iter::Peekable;
use std::str::Chars;
use super::*;

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    chars_consumed: usize
}

impl<'a> Lexer<'a> {
    pub fn new(chars: Peekable<Chars<'a>>) -> Self {
        Self {
            chars: chars,
            chars_consumed: 0,
        }
    }

    pub fn from_str(input: &'a str) -> Self {
        Self::new(input.chars().peekable())
    }


    // Returns the next morpheme in the input
    // If there are no more morphemes left, this will return None
    pub fn next_morpheme(&mut self) -> Option<Morpheme> {
        self.skip_whitespace();

        match self.chars.peek() {
            None => None,
            Some(&c) => {
                let offset = self.chars_consumed;
                let content =
                    if is_numeric(c) {
                        self.parse_number()
                    } else {
                        self.parse_symbol()
                    };
                let length = self.chars_consumed - offset;

                Some(Morpheme{
                    offset: offset,
                    length: length,
                    content: content
                })
            }
        }
    }

    // Returns an iterator over the Morphemes of this Lexer
    pub fn morphemes(self) -> Morphemes<'a> where Self: Sized {
        Morphemes::new(self)
    }


    fn next_char(&mut self) -> Option<char> {
        let c = self.chars.next();

        if c != None {
            self.chars_consumed += 1;
        }

        c
    }

    fn parse_number(&mut self) -> MorphemeContent {
        let mut accu = String::with_capacity(32);
        loop {
            let c = self.chars.peek();

            if c != None && is_numeric(*c.unwrap()) {
                accu.push(self.next_char().unwrap());
            }
            else {
                break;
            }
        }

        match accu.parse() {
            Err(_) => MorphemeContent::Invalid,
            Ok(value) => MorphemeContent::Number { value: value }
        }
    }

    fn parse_symbol(&mut self) -> MorphemeContent {
        MorphemeContent::Symbol {
            symbol_type: match self.next_char() {
                Some(c) => match c {
                    '(' => SymbolType::RoundOpeningBrace,
                    ')' => SymbolType::RoundClosingBrace,
                    '+' => SymbolType::Add,
                    '-' => SymbolType::Subtract,
                    '*' => SymbolType::Multiply,
                    '/' => SymbolType::Divide,

                    _ => return MorphemeContent::Invalid
                },
                None => panic!("Called parse_symbol at EOF")
            }
        }
    }

    fn peek_is_whitespace(&mut self) -> bool {
        let c = self.chars.peek();
        c != None && c.unwrap().is_whitespace()
    }

    fn skip_whitespace(&mut self) {
        while self.peek_is_whitespace() {
            self.next_char();
        }
    }
}

fn is_numeric(c: char) -> bool {
    c.is_ascii_digit() || c == '.'
}



#[cfg(test)]
mod tests {
    use super::super::*;
    use super::super::test_data::{ single_numbers, single_symbols, mixed_input, invalid_input };

    fn assert_input_yields_morpheme(input: &str, expected: Option<Morpheme>) {
        let morpheme = Lexer::from_str(input).next_morpheme();
        assert_eq!(expected, morpheme);
    }


    #[test]
    fn empty_stream_yields_eof() {
        assert_input_yields_morpheme("", None)
    }

    #[test]
    fn single_number_is_detected() {
        for (input, expected_number) in single_numbers() {
            assert_input_yields_morpheme(input, Some(
                Morpheme {
                    offset: 0,
                    length: input.chars().count(),
                    content: MorphemeContent::Number { value: expected_number }
                }
            ))
        }
    }
    
    #[test]
    fn single_symbol_is_detected() {
        for (input, expected_symbol_type) in single_symbols() {
            assert_input_yields_morpheme(input, Some(
                Morpheme {
                    offset: 0,
                    length: input.len(),
                    content: MorphemeContent::Symbol { symbol_type: expected_symbol_type }
                }
            ))
        }
    }

    #[test]
    fn mixed_input_is_detected() {
        for (input, expected_morphemes) in mixed_input() {
            let mut lexer = Lexer::from_str(input);

            for expected_morpheme in expected_morphemes {
                assert_eq!(expected_morpheme, lexer.next_morpheme().unwrap());
            }

            for _ in 0..3 { // Checking for fused EOF detection
                assert_eq!(None, lexer.next_morpheme());
            }            
        }
    }

    #[test]
    fn invalid_symbols_will_be_detected() {        
        for input in invalid_input() {
            assert_input_yields_morpheme(input, Some(
                Morpheme{
                    offset: 0,
                    length: input.chars().count(),
                    content: MorphemeContent::Invalid
                }
            ));
        }
    }
}
