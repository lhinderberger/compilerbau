use super::{ Lexer, Morpheme };

// An iterator over the Morphemes of a Lexer
pub struct Morphemes<'a> {
    lexer: Lexer<'a>
}

impl<'a> Morphemes<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self{ lexer: lexer }
    }
}

impl<'a> Iterator for Morphemes<'a> {
    type Item = Morpheme;

    fn next(&mut self) -> Option<Self::Item> {
        self.lexer.next_morpheme()
    }
}


#[cfg(test)]
mod tests {
    use super::super::*;
    use super::super::test_data::mixed_input;

    #[test]
    fn iterator_returns_correct_morphemes() {
        for (input, expected_morphemes) in mixed_input() {
            let actual_morphemes: Vec<Morpheme> = Lexer::from_str(input).morphemes().collect();
            assert_eq!(expected_morphemes, actual_morphemes);
        }
    }

}
