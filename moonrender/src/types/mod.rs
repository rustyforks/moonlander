use super::lines::Line;
use anyhow::Result;

pub mod generic;
pub mod text_gemini;
pub mod text_plain;

pub trait Renderer {
    fn parse_line(&mut self, line: &str) -> Result<Box<dyn Line>>;
}
