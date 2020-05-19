use super::{generic::Preformat, Line, Renderer};
use anyhow::Result;

pub struct Plain {}

impl Plain {
    pub fn new() -> Self {
        Self {}
    }
}

impl Renderer for Plain {
    fn parse_line(&mut self, line: &str) -> Result<Box<dyn Line>> {
        let line = line.to_owned();

        Ok(Box::new(Preformat::new(line)))
    }
}
