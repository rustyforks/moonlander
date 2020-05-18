mod config;
mod lines;
mod types;

use anyhow::{Context as _, Result};
use cairo::Context;
use lines::Line;
use std::{collections::HashMap, ops::Deref};
use types::{text_gemini::Gemini, text_plain::Plain};
use url::Url;

pub use config::Theme;

pub enum Msg {
    Goto(String),
}

#[derive(Default)]
pub struct Data {
    pub mime: String,
    pub url: Option<Url>,

    pub source: String,

    pub theme: Theme,
}

#[derive(Default)]
pub struct Renderer<C: Deref<Target = Context>> {
    pub data: Data,

    lines: Vec<Box<dyn Line<C>>>,

    chunk_incomplete: String,

    renderers: HashMap<String, Box<dyn types::Renderer<C>>>,
    margin: f64,
}

impl<C: Deref<Target = Context>> Renderer<C> {
    pub fn new(theme: Theme) -> Self {
        let mut renderers: HashMap<String, Box<dyn types::Renderer<C>>> = HashMap::new();

        renderers.insert("text/gemini".to_owned(), Box::new(Gemini::new()));
        renderers.insert("text/plain".to_owned(), Box::new(Plain::new()));

        Self {
            data: Data {
                mime: "text/plain".to_owned(),
                url: None,

                source: String::new(),

                theme,
            },

            lines: vec![],

            chunk_incomplete: String::new(),

            renderers,
            margin: 0.0,
        }
    }

    pub fn new_page_chunk(&mut self, contents: &str) -> Result<()> {
        if self.data.source.is_empty() {
            self.lines.clear();
        }

        for chr in contents.chars() {
            if chr == '\n' {
                let line = self.chunk_incomplete.clone();

                self.lines.push(
                    self.renderers
                        .get_mut(&self.data.mime)
                        .context("no renderer for mime")?
                        .parse_line(&line)
                        .context("Cannot render line")?,
                );

                log::debug!("({}) Render line: {}", self.data.mime, line);
                self.chunk_incomplete.clear();
            } else {
                self.chunk_incomplete.push(chr);
            }
        }

        self.data.source += contents;
        Ok(())
    }

    pub fn set_mime(&mut self, mime: &str) {
        // we might want to assume this runs before any chunks are sent.
        log::debug!("renderer mime: {:?}", mime);
        self.data.mime = mime.split(';').next().expect("No mime sent?").to_owned();
    }

    pub fn set_url(&mut self, url: &str) -> Result<()> {
        log::debug!("renderer url: {:?}", url);
        self.data.url = Some(Url::parse(url).context("Cannot parse URL")?);

        Ok(())
    }

    pub fn reset(&mut self) {
        self.data.mime = "text/plain".to_owned();
        self.data.source = String::new();
    }

    pub fn render(&mut self, y_offset: f64, height: f64, ctx: &C) -> (i32, i32) {
        ctx.set_source_rgb(
            self.data.theme.background_color.0 as f64 / 255.0,
            self.data.theme.background_color.1 as f64 / 255.0,
            self.data.theme.background_color.2 as f64 / 255.0,
        );

        ctx.paint();

        let pango = pangocairo::create_layout(ctx).expect("cannot create pango layout");

        let w = ctx.clip_extents().2;
        self.margin = w * self.data.theme.margin_percent;

        ctx.move_to(self.margin, self.data.theme.paragraph_spacing * 2.0);

        for line in &mut self.lines {
            let pos = line.get_pos();
            let size = line.get_size();

            // clip lines for performance, but include extra 2 lines as to make
            // sure other lines load properly. could be smaller I assume, but
            // let's play it safe
            if pos.1 - (pos.1 * 2.0) <= y_offset + height && (pos.1 + size.1) * 2.0 >= y_offset {
                line.draw(ctx, &pango, &self.data.theme);
            }

            // this is required to be outside of the if to be able to figure out
            // the rest of the page's size and send to the parent
            ctx.rel_move_to(0.0, line.get_size().1 + self.data.theme.paragraph_spacing);
        }

        (
            w as i32,
            ctx.get_current_point().1 as i32 + self.data.theme.paragraph_spacing as i32 * 2,
        )
    }

    pub fn click(&mut self, pos: (f64, f64)) -> Option<Msg> {
        log::debug!("click {:?}", pos);

        for line in self.lines.iter_mut() {
            let coords = line.get_pos();
            let size = line.get_size();

            if pos.0 >= coords.0
                && pos.0 <= coords.0 + size.0
                && pos.1 >= coords.1
                && pos.1 <= coords.1 + size.1
            {
                return line.click(&self.data);
            }
        }

        None
    }
}
