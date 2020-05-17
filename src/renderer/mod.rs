use cairo::Context;
use std::ops::Deref;

#[derive(Default)]
pub struct Renderer {}

impl Renderer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn new_page_chunk(&mut self, contents: &str) {
        log::debug!("renderer document append: {:?}", contents);
    }

    pub fn set_mime(&mut self, mime: &str) {
        log::debug!("renderer mime: {:?}", mime);
    }

    pub fn reset(&mut self) {}

    pub fn render(&self, ctx: &impl Deref<Target = Context>) {
        ctx.set_source_rgb(1.0, 1.0, 1.0);
        ctx.paint();
    }
}
