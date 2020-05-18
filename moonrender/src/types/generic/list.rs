use crate::lines::Line;
use crate::MARGIN;
use cairo::Context;
use pango::{Alignment, Layout, WrapMode};
use std::ops::Deref;

const BULLET: char = '\u{2022}';
const BULLET_SIZE: f64 = 13.0;

pub struct List {
    line: String,

    width: f64,
    height: f64,
}

impl List {
    pub fn new(line: String) -> Self {
        Self {
            line,

            width: 0.0,
            height: 0.0,
        }
    }
}

impl<C: Deref<Target = Context>> Line<C> for List {
    fn get_size(&self) -> (f64, f64) {
        (self.width, self.height)
    }

    fn draw(&mut self, ctx: &C, pango: &Layout) {
        let w = ctx.clip_extents().2;
        pango.set_width(pango::units_from_double(
            w - ((w * MARGIN) * 2.0) - BULLET_SIZE,
        ));

        pango.set_alignment(Alignment::Left);
        pango.set_wrap(WrapMode::Word);
        pango.set_font_description(Some(&pango::FontDescription::from_string("sans-serif")));
        pango.set_text(&(BULLET.to_string() + " " + &self.line));

        let (w, h) = pango.get_pixel_size();
        self.width = w as f64;
        self.height = h as f64;

        ctx.set_source_rgb(0.0, 0.0, 0.0);

        ctx.rel_move_to(-BULLET_SIZE, 0.0);
        pangocairo::show_layout(ctx, pango);
        ctx.rel_move_to(BULLET_SIZE, 0.0); // reset
    }
}
