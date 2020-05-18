use crate::lines::Line;
use crate::Theme;
use cairo::Context;
use pango::{Alignment, Layout, WrapMode};
use std::ops::Deref;

pub struct List {
    line: String,

    x: f64,
    y: f64,

    width: f64,
    height: f64,
}

impl List {
    pub fn new(line: String) -> Self {
        Self {
            line,

            x: 0.0,
            y: 0.0,

            width: 0.0,
            height: 0.0,
        }
    }
}

impl<C: Deref<Target = Context>> Line<C> for List {
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

        let mut font_description = pango::FontDescription::from_string(&theme.list.font);
        font_description.set_size(pango::units_from_double(theme.list.size));

        pango.set_spacing(pango::units_from_double(theme.list.line_spacing));
        pango.set_alignment(Alignment::Left);
        pango.set_wrap(WrapMode::Word);
        pango.set_font_description(Some(&font_description));
        pango.set_text(&self.line);

        let (w, h) = pango.get_pixel_size();
        self.width = w as f64;
        self.height = h as f64 + theme.link.line_spacing;

        ctx.set_source_rgb(
            theme.list.color.0 as f64 / 255.0,
            theme.list.color.1 as f64 / 255.0,
            theme.list.color.2 as f64 / 255.0,
        );

        pangocairo::show_layout(ctx, pango);

        // Draw bullet

        let w = ctx.clip_extents().2;
        let w = w as f64 * theme.margin_percent;

        pango.set_width(pango::units_from_double(w));
        ctx.rel_move_to(-(w + theme.list.bullet_padding), 0.0);

        pango.set_alignment(Alignment::Right);
        pango.set_text(&theme.list.bullet);
        pangocairo::show_layout(ctx, pango);

        // reset
        ctx.rel_move_to(w + theme.list.bullet_padding, theme.link.line_spacing / 2.0);
    }
}
