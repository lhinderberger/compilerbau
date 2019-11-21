use super::super::lexer::Morpheme;
use super::super::parser::{ GraphLocation, VertexIndex, VertexTookObserver };
use super::Error;


pub struct LoggingSemantics {}

impl VertexTookObserver for LoggingSemantics {
    fn vertex_took(&mut self, at: &GraphLocation, which: VertexIndex, last_morpheme: Option<Morpheme>) -> Result<(), Error> {
        println!("Took Vertex #{} at {:?}, consumed {:?}", which, at, last_morpheme);
        Ok(())
    }
}