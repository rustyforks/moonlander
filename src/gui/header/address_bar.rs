use anyhow::Context;
use gtk::prelude::*;
use relm::Widget;
use relm_derive::{widget, Msg};

#[derive(Msg)]
pub enum Msg {
    Goto(String),
    Redirect(String),

    Error(anyhow::Error),
}

pub struct Model {
    url: String,
}

#[widget]
impl Widget for AddressBar {
    fn model() -> Model {
        Model { url: String::new() }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Redirect(url) => self.model.url = url,

            Msg::Goto(_) => { /* listened from parent */ }
            Msg::Error(_) => { /* listened from parent */ }
        }
    }

    view! {
        #[name="address_bar"]
        gtk::Entry {
            placeholder_text: Some("Enter gemini:// address..."),
            max_width_chars: 100,
            text: &self.model.url,

            activate(entry) => {
                match entry.get_text().context("cannot get addressbar text") {
                    Ok(text) => Msg::Goto(text.to_string()),
                    Err(e) => Msg::Error(e)
                }
            }
        }
    }
}
