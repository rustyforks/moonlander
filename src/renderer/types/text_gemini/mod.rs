use super::{
    generic::{Heading, Link, List, Preformat, Text},
    Renderer,
};
use crate::renderer::lines::Line;
use cairo::Context;
use std::ops::Deref;

pub struct Gemini {
    is_preformatted: bool,
}

impl Gemini {
    pub fn new() -> Self {
        Self {
            is_preformatted: false,
        }
    }
}

impl<C: Deref<Target = Context>> Renderer<C> for Gemini {
    fn parse_line(&mut self, line: &str) -> Box<dyn Line<C>> {
        let mut line = line.to_owned();

        if line.starts_with("```") {
            self.is_preformatted = !self.is_preformatted;
            line = String::new();
        }

        if self.is_preformatted {
            return Box::new(Preformat::new(line));
        }

        line = line.trim().to_owned();
        if line.starts_with("=>") {
            let data = &mut line[2..].trim().split_whitespace();

            let link = data.next().expect("no link?");
            let mut caption = data.collect::<Vec<&str>>().join(" ");

            if caption.is_empty() {
                caption = link.to_owned();
            }

            Box::new(Link::new(caption, link.to_owned()))
        } else if line.starts_with('*') {
            Box::new(List::new(line[1..].trim().to_owned()))
        } else if line.starts_with('#') {
            let mut heading = 0;
            let mut iter = line.chars();

            while let Some(chr) = iter.next() {
                if chr == '#' {
                    heading += 1;
                } else {
                    break;
                }
            }

            let line = iter.collect::<String>();

            log::debug!("{} heading {}", heading, line);
            Box::new(Heading::new(line, heading))
        } else {
            Box::new(Text::new(line))
        }
    }
}
