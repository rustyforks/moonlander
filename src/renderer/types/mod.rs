use super::lines::Line;
use anyhow::Result;

pub mod generic;
pub mod text_gemini;

pub trait Renderer<C> {
    fn parse_line(&mut self, line: &str) -> Result<Box<dyn Line<C>>>;
}
