use super::super::lexer::Morpheme;
use super::super::parser::VertexTookObserver;
use super::super::parser::syntax_graph::*;
use super::Error;

pub struct NopSemantics {}

impl VertexTookObserver for NopSemantics {
    fn vertex_took(&mut self, _at: &GraphLocation, _which: VertexIndex, _last_morpheme: Option<Morpheme>) -> Result<(), Error> {
        Ok(())
    }
}
