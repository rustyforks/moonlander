use super::{
    generic::{Heading, Link, List, Preformat, Text},
    Line, Renderer,
};
use anyhow::{Context as _, Result};

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

impl Renderer for Gemini {
    fn parse_line(&mut self, line: &str) -> Result<Box<dyn Line>> {
        let mut line = line.to_owned();

        if line.starts_with("```") {
            self.is_preformatted = !self.is_preformatted;
            line = String::new();
        }

        if self.is_preformatted {
            return Ok(Box::new(Preformat::new(line)));
        }

        line = line.trim().to_owned();
        if line.starts_with("=>") {
            let data = &mut line[2..].trim().split_whitespace();

            let link = data.next().context("No link?")?;
            let mut caption = data.collect::<Vec<&str>>().join(" ");

            if caption.is_empty() {
                caption = link.to_owned();
            }

            Ok(Box::new(Link::new(caption, link.to_owned())))
        } else if line.starts_with('*') {
            Ok(Box::new(List::new(line[1..].trim().to_owned())))
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
            Ok(Box::new(Heading::new(line, heading)))
        } else {
            Ok(Box::new(Text::new(line)))
        }
    }
}
