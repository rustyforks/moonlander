use crate::lines::Line;
use crate::Theme;
use cairo::Context;
use pango::{Alignment, Layout, WrapMode};

pub struct Text {
    line: String,

    x: f64,
    y: f64,

    width: f64,
    height: f64,
}

impl Text {
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

impl Line for Text {
    fn get_pos(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    fn get_size(&self) -> (f64, f64) {
        (self.width, self.height)
    }

    fn draw(&mut self, ctx: &Context, pango: &Layout, theme: &Theme) {
        let (x, y) = ctx.get_current_point();
        self.x = x;
        self.y = y;

        if self.line.is_empty() && self.height != 0.0 {
            return;
        }

        let w = ctx.clip_extents().2.min(theme.max_content_width);
        pango.set_width(pango::units_from_double(w - (theme.margin * 2.0)));

        let mut font_description = pango::FontDescription::from_string(&theme.content.font);
        font_description.set_size(pango::units_from_double(theme.content.size));

        pango.set_spacing(pango::units_from_double(theme.content.line_spacing));
        pango.set_alignment(Alignment::Left);
        pango.set_wrap(WrapMode::Word);
        pango.set_font_description(Some(&font_description));
        pango.set_text(&self.line);

        let (w, h) = pango.get_pixel_size();
        self.width = w as f64;
        self.height = h as f64 + theme.content.line_spacing;

        ctx.set_source_rgb(
            theme.content.color.0 as f64 / 255.0,
            theme.content.color.1 as f64 / 255.0,
            theme.content.color.2 as f64 / 255.0,
        );
        pangocairo::show_layout(ctx, pango);

        ctx.rel_move_to(0.0, theme.content.line_spacing / 2.0);
    }
}
