use crate::renderer::{lines::Line, MARGIN};
use cairo::Context;
use pango::{Alignment, Layout, Weight, WrapMode};
use std::ops::Deref;

pub struct Heading {
    line: String,
    size: u8,

    width: f64,
    height: f64,
}

impl Heading {
    pub fn new(line: String, size: u8) -> Self {
        Self {
            line,
            size,

            width: 0.0,
            height: 0.0,
        }
    }
}

impl<C: Deref<Target = Context>> Line<C> for Heading {
    fn get_size(&self) -> (f64, f64) {
        (self.width, self.height)
    }

    fn draw(&mut self, ctx: &C, pango: &Layout) {
        let w = ctx.clip_extents().2;
        pango.set_width(pango::units_from_double(w - ((w * MARGIN) * 2.0)));

        let font_description = &mut pango::FontDescription::from_string("sans-serif");
        font_description.set_weight(Weight::Heavy);
        font_description.set_size(pango::units_from_double((26 / self.size).into()));

        pango.set_alignment(Alignment::Left);
        pango.set_wrap(WrapMode::Word);
        pango.set_font_description(Some(font_description));
        pango.set_text(&self.line);

        let (w, h) = pango.get_pixel_size();
        self.width = w as f64;
        self.height = h as f64;

        ctx.set_source_rgb(0.0, 0.0, 0.0);
        pangocairo::show_layout(ctx, pango);
    }
}
