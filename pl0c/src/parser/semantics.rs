use super::super::lexer::Morpheme;
use super::*;
use super::syntax_graph::*;

pub struct NopSemantics {}

impl VertexTookObserver for NopSemantics {
    fn vertex_took(&mut self, at: &GraphLocation, which: VertexIndex, last_morpheme: Option<Morpheme>) -> Result<(),SemanticError> {
        Ok(())
    }
}


pub struct LoggingSemantics {}

impl VertexTookObserver for LoggingSemantics {
    fn vertex_took(&mut self, at: &GraphLocation, which: VertexIndex, last_morpheme: Option<Morpheme>) -> Result<(),SemanticError> {
        println!("Took Vertex #{} at location {:?} for morpheme {:?}", which, at, last_morpheme);
        Ok(())
    }
}