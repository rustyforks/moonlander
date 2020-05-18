use crate::lines::Line;
use crate::Theme;
use cairo::Context;
use pango::{Alignment, Layout, Weight, WrapMode};
use std::ops::Deref;

pub struct Heading {
    line: String,
    size: u8,

    x: f64,
    y: f64,

    width: f64,
    height: f64,
}

impl Heading {
    pub fn new(line: String, size: u8) -> Self {
        Self {
            line,
            size,

            x: 0.0,
            y: 0.0,

            width: 0.0,
            height: 0.0,
        }
    }
}

impl<C: Deref<Target = Context>> Line<C> for Heading {
    fn get_pos(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    fn get_size(&self) -> (f64, f64) {
        (self.width, self.height)
    }

    fn draw(&mut self, ctx: &C, pango: &Layout, theme: &Theme) {
        let (x, y) = ctx.get_current_point();
        self.x = x;
        self.y = y;

        let w = ctx.clip_extents().2;
        pango.set_width(pango::units_from_double(
            w - ((w * theme.margin_percent) * 2.0),
        ));

        let font_description = &mut pango::FontDescription::from_string(&theme.heading.font);
        font_description.set_weight(Weight::Ultraheavy);
        font_description.set_size(pango::units_from_double(if self.size == 1 {
            theme.heading.level1_size
        } else {
            theme.heading.level2_size
        }));

        pango.set_spacing(pango::units_from_double(theme.heading.line_spacing));
        pango.set_font_description(Some(font_description));
        pango.set_alignment(Alignment::Left);
        pango.set_wrap(WrapMode::Word);
        pango.set_text(&self.line);

        let (w, h) = pango.get_pixel_size();
        self.width = w as f64;
        self.height = h as f64 + theme.heading.line_spacing;

        ctx.set_source_rgb(
            theme.heading.color.0 as f64 / 255.0,
            theme.heading.color.1 as f64 / 255.0,
            theme.heading.color.2 as f64 / 255.0,
        );
        pangocairo::show_layout(ctx, pango);

        ctx.rel_move_to(0.0, theme.heading.line_spacing);
    }
}
