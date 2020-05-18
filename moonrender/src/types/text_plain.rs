use super::{generic::Preformat, Line, Renderer};
use anyhow::Result;
use cairo::Context;
use std::ops::Deref;

pub struct Plain {}

impl Plain {
    pub fn new() -> Self {
        Self {}
    }
}

impl<C: Deref<Target = Context>> Renderer<C> for Plain {
    fn parse_line(&mut self, line: &str) -> Result<Box<dyn Line<C>>> {
        let line = line.to_owned();

        Ok(Box::new(Preformat::new(line)))
    }
}
