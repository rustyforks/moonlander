use crate::lines::Line;
use crate::{Data, Msg as RendererMsg, Theme};
use cairo::Context;
use pango::{Alignment, Layout, WrapMode};

pub struct Link {
    url: String,
    line: String,

    x: f64,
    y: f64,

    width: f64,
    height: f64,
}

impl Link {
    pub fn new(line: String, url: String) -> Self {
        Self {
            url,
            line,

            x: 0.0,
            y: 0.0,

            width: 0.0,
            height: 0.0,
        }
    }
}

impl Line for Link {
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

        let w = ctx.clip_extents().2.min(theme.max_content_width);
        pango.set_width(pango::units_from_double(w - (theme.margin * 2.0)));

        let mut font_description = pango::FontDescription::from_string(&theme.link.font);
        font_description.set_size(pango::units_from_double(theme.link.size));

        pango.set_spacing(pango::units_from_double(theme.link.line_spacing));
        pango.set_alignment(Alignment::Left);
        pango.set_wrap(WrapMode::Word);
        pango.set_font_description(Some(&font_description));
        pango.set_text(&self.line);

        let (w, h) = pango.get_pixel_size();
        self.width = w as f64;
        self.height = h as f64 + theme.link.line_spacing;

        ctx.set_source_rgb(
            theme.link.color.0 as f64 / 255.0,
            theme.link.color.1 as f64 / 255.0,
            theme.link.color.2 as f64 / 255.0,
        );
        pangocairo::show_layout(ctx, pango);

        ctx.rel_move_to(0.0, theme.link.line_spacing / 2.0);
    }

    fn click(&mut self, data: &Data) -> Option<RendererMsg> {
        if let Some(url) = &data.url {
            match url.join(&self.url) {
                Ok(new) => Some(RendererMsg::Goto(new.to_string())),
                Err(e) => {
                    log::error!("Not following link since: {}", e);
                    None
                }
            }
        } else {
            log::warn!("Not following link since renderer doesn't have a URL");
            None
        }
    }
}
