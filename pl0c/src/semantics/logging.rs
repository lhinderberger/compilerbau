use super::super::lexer::Morpheme;
use super::super::parser::{ GraphLocation, VertexIndex, VertexTookObserver };
use super::Error;

use std::io::Write;

pub struct LoggingSemantics<'a> {
    output: &'a mut dyn Write
}

impl<'a> LoggingSemantics<'a> {
    pub fn with_output(output: &'a mut dyn Write) -> LoggingSemantics<'a> {
        LoggingSemantics { output: output }
    }
}

impl<'a> VertexTookObserver for LoggingSemantics<'a> {
    fn vertex_took(&mut self, at: &GraphLocation, which: VertexIndex, last_morpheme: Option<Morpheme>) -> Result<(), Error> {
        writeln!(self.output, "Took Vertex #{} at {:?}, consumed {:?}", which, at, last_morpheme).unwrap();
        Ok(())
    }
}