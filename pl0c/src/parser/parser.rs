use super::super::lexer::*;
use super::error::*;
use super::syntax_graph::*;
use super::pl0_syntax::*;

use std::cell::RefCell;
use std::collections::HashMap;
use std::iter::Peekable;


pub struct Parser<'a, T: VertexTookObserver> {
    graphs: HashMap<GraphID, Graph>,
    morphemes: RefCell<Peekable<Morphemes<'a>>>,
    observer: RefCell<&'a mut T>
}


pub trait VertexTookObserver{
    fn vertex_took(&mut self, at: &GraphLocation, which: VertexIndex, last_morpheme: Option<Morpheme>) -> Result<(), SemanticError>;
}


impl<'a, T: VertexTookObserver> Parser<'a, T> {
    pub fn new(morphemes: Peekable<Morphemes<'a>>, observer: &'a mut T) -> Self {
        Self {
            graphs: newPL0SyntaxMap(),
            morphemes: RefCell::from(morphemes),
            observer: RefCell::from(observer)
        }
    }

    
    pub fn parse(&mut self) -> Result<(), Error> {
        self.parse_inner(self.graphs.get("PROGRAM").unwrap())
    }

    fn parse_inner(&self, graph: &Graph) -> Result<(), Error> {
        let mut location = GraphLocation{graph: graph.id.clone(), node: 0};

        loop {
            let node = graph.nodes.get(location.node).unwrap();

            let peek = self.morphemes.borrow_mut().peek().cloned();
            let eof = peek.is_none();

            let taken = node.vertices.iter().enumerate().find(|(_,v)| v.condition.met_for_morpheme_or_eof(&peek));

            if let Some((vertex_idx, vertex)) = taken {
                // Consume morpheme (unless Nil / Subgraph condition)
                let morpheme = match vertex.condition {
                    VertexCondition::Nil => None,
                    VertexCondition::Subgraph(_) => None,
                    _ => self.morphemes.borrow_mut().next()
                };

                // Call Observer
                self.observer.borrow_mut().vertex_took(&location, vertex_idx, morpheme)?;

                // Recurse to subgraph (if neccessary)
                if let VertexCondition::Subgraph(subgraph_id) = &vertex.condition {
                    self.parse_inner(self.graphs.get(subgraph_id).unwrap())?;
                }

                // Advance to next node
                match vertex.target {
                    VertexTarget::Node(next_node_idx) => location.node = next_node_idx,
                    VertexTarget::EndOfGraph => return Ok(())
                }
            }
            else if eof {
                return Err(Error::EOF)
            }
            else {
                let bad_morpheme = peek.unwrap();
                
                return Err(match bad_morpheme.content {
                    MorphemeContent::Invalid => Error::InvalidMorpheme(bad_morpheme),
                    _ => Error::Syntax{ location: location, next_morpheme: bad_morpheme }
                });
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::super::super::*;

    #[test]
    fn valid_syntax_is_accepted() {
        let test_data = vec![
            ".",
            "!1+1.",
            "CALL HelloWorld.",
            "CONST A = 1, B = 2;\nVAR C;\nPROCEDURE ADD;C:=A+B;."
        ]; //TODO: Add more test data

        for t in test_data {
            let lexer = lexer::Lexer::from_str(t);

            println!("\n\nSource Code:\n{}\n\n", t);
            let mut semantics = parser::LoggingSemantics{};
            let mut parser = parser::Parser::new(lexer.morphemes().peekable(), &mut semantics);

            parser.parse().unwrap();
        }
    }

    #[test]
    fn invalid_syntax_is_rejected() {
        let test_data = vec![
            "!1++1.",
            "CONST A = 1, B = 2;\nVAR C;\nPROCEDURE ADD;C:=A+B."
        ]; //TODO: Add more test data

        for t in test_data {
            let lexer = lexer::Lexer::from_str(t);

            let mut semantics = parser::NopSemantics{};
            let mut parser = parser::Parser::new(lexer.morphemes().peekable(), &mut semantics);

            match parser.parse() {
                Err(parser::Error::Syntax{..}) => (),
                _ => panic!("Parser did not detect syntax error!")
            }
        }
    }
}
