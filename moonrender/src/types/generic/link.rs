use crate::lines::Line;
use crate::{Data, Msg as RendererMsg, MARGIN};
use cairo::Context;
use pango::{Alignment, Layout, WrapMode};
use std::ops::Deref;

pub struct Link {
    url: String,
    line: String,

    width: f64,
    height: f64,
}

impl Link {
    pub fn new(line: String, url: String) -> Self {
        Self {
            url,
            line,

            width: 0.0,
            height: 0.0,
        }
    }
}

impl<C: Deref<Target = Context>> Line<C> for Link {
    fn get_size(&self) -> (f64, f64) {
        (self.width, self.height)
    }

    fn draw(&mut self, ctx: &C, pango: &Layout) {
        let w = ctx.clip_extents().2;
        pango.set_width(pango::units_from_double(w - ((w * MARGIN) * 2.0)));

        pango.set_alignment(Alignment::Left);
        pango.set_wrap(WrapMode::Word);
        pango.set_font_description(Some(&pango::FontDescription::from_string("sans-serif")));
        pango.set_text(&self.line);

        let (w, h) = pango.get_pixel_size();
        self.width = w as f64;
        self.height = h as f64;

        ctx.set_source_rgb(0.0, 0.0, 1.0);
        pangocairo::show_layout(ctx, pango);
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