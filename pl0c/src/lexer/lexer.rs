use std::iter::Peekable;
use std::str::Chars;
use super::*;

type LexerState = u8;
type LexerTransition = (LexerOp,Option<LexerState>);

const O_CONS : LexerOp = LexerOp::Consume;
const O_NONE : LexerOp = LexerOp::None;
const O_SKIP : LexerOp = LexerOp::Skip;
const T_DONE : LexerTransition = (O_NONE,None);

// Implemented as a State Machine, as required by the assignment

// State Machine Transition Table indexed by the current state as the row number
// and the next character's class code (refer to the function code of CharClass)
// as the column number.
//
// Each entry is a pair of the operation to be performed and the next state that
// then shall be set (None to terminate).
const LEXER_STATE_MACHINE_TBL : [LexerTransition;35] = [
    // Number         Letter            Symbol[:<>]       Symbol[=]      SingularSymbol Whitespace        OtherChar
    (O_CONS,Some(2)), (O_CONS,Some(1)), (O_CONS,Some(3)), (O_CONS,None), (O_CONS,None), (O_SKIP,Some(0)), (O_CONS,Some(4)), // Whitespace skipping, single symbol detect and dispatch
    (O_CONS,Some(1)), (O_CONS,Some(1)), T_DONE,           T_DONE,        T_DONE,        T_DONE,           T_DONE,           // Identifier detect
    (O_CONS,Some(2)), T_DONE,           T_DONE,           T_DONE,        T_DONE,        T_DONE,           T_DONE,           // Number detect
    T_DONE,           T_DONE,           T_DONE,           (O_CONS,None), T_DONE,        T_DONE,           T_DONE,           // (Colon|Lesser|Greater) [Equal] switch
    T_DONE,           T_DONE,           T_DONE,           T_DONE,        T_DONE,        T_DONE,           (O_CONS,Some(4))  // Invalid morpheme detect
];


#[derive(Clone, Copy)]
enum LexerOp {
    Consume,
    None,
    Skip
}

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    chars_consumed: usize,
    state: LexerState,
    buffer: String,
    start_idx: usize,
    symbol_lookup: SymbolLookup
}

impl<'a> Lexer<'a> {
    pub fn new(chars: Peekable<Chars<'a>>) -> Self {
        Self {
            chars: chars,
            chars_consumed: 0,
            state: 0,
            buffer: String::new(),
            start_idx: 0,
            symbol_lookup: SymbolLookup::new()
        }
    }

    pub fn from_str(input: &'a str) -> Self {
        Self::new(input.chars().peekable())
    }

    // Returns the next morpheme in the input
    // If there are no more morphemes left, this will return None
    pub fn next_morpheme(&mut self) -> Option<Morpheme> {        
        self.reset_state_machine();

        loop {
            match self.lookup_next_transition() {
                Some(transition) => {
                    self.execute_lexer_op(transition.0);

                    match transition.1 {
                        None => break,
                        Some(next_state) => { self.state = next_state; }
                    }
                },
                None => match self.buffer.is_empty() {
                    true => return None,
                    false => break
                }
            }
        }

        return Some(self.build_morpheme())
    }

    fn build_morpheme(&self) -> Morpheme {
        Morpheme {
            content: self.build_morpheme_content(),
            offset: self.start_idx,
            length: self.chars_consumed - self.start_idx
        }
    }

    fn build_morpheme_content(&self) -> MorphemeContent {
        let buffer_uppercase = self.buffer.to_uppercase();
        let lookup_symbol = || self.symbol_lookup.lookup(&buffer_uppercase);

        match self.state {
            0 | 3 => MorphemeContent::Symbol(lookup_symbol().unwrap()),
            1 => match lookup_symbol() {
                Some(symbol_type) => MorphemeContent::Symbol(symbol_type),
                None => MorphemeContent::Identifier(buffer_uppercase)
            },
            2 => MorphemeContent::Number(self.buffer.parse().unwrap()),
            4 => MorphemeContent::Invalid,

            _ => panic!("Unexpected state: {:?}", self.state)
        }
    }

    fn consume_char(&mut self) -> Option<char> {
        match self.chars.next() {
            Some(c) => { self.chars_consumed += 1; Some(c) },
            None => None
        }
    }

    fn execute_lexer_op(&mut self, operation: LexerOp) {
        match operation {
            LexerOp::Consume => {
                let c = self.consume_char().unwrap();
                self.buffer.push(c);
            },
            LexerOp::Skip => {
                self.consume_char().unwrap();
                self.start_idx += 1;
            },
            LexerOp::None => ()
        }
    }

    fn lookup_next_transition(&mut self) -> Option<LexerTransition> {
        let next_char_class = self.peek_char_class()?;
        let transition_idx = self.state*7 + next_char_class.code();
        
        Some(LEXER_STATE_MACHINE_TBL[transition_idx as usize])
    }

    fn peek_char_class(&mut self) -> Option<CharClass> {
        match self.chars.peek() {
            Some(c) => Some(CharClass::of(*c)),
            None => None
        }
    }

    fn reset_state_machine(&mut self) {
        self.state = 0;
        self.buffer.clear();
        self.start_idx = self.chars_consumed;
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::super::test_data::{ eof_input, single_morphemes, mixed_input };

    fn assert_input_yields_morpheme(input: &str, expected: Option<Morpheme>) {
        let morpheme = Lexer::from_str(input).next_morpheme();
        assert_eq!(expected, morpheme);
    }


    #[test]
    fn empty_stream_yields_eof() {
        for input in eof_input() {
            assert_input_yields_morpheme(input, None);
        }
    }

    #[test]
    fn single_morpheme_is_detected() {
        for (input, expected_content) in single_morphemes() {
            assert_input_yields_morpheme(input, Some(
                Morpheme {
                    offset: 0,
                    length: input.chars().count(),
                    content: expected_content
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
}
