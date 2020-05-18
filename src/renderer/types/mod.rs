use super::lines::Line;

pub mod generic;
pub mod text_gemini;

pub trait Renderer<C> {
    fn parse_line(&mut self, line: &str) -> Box<dyn Line<C>>;
}
