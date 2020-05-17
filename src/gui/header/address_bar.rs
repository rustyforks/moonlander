use gtk::prelude::*;
use relm::Widget;
use relm_derive::{widget, Msg};

#[derive(Msg)]
pub enum Msg {
    Goto(String),
    Redirect(String),
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
            Msg::Goto(_) => { /* listened from parent */ }
            Msg::Redirect(url) => self.model.url = url,
        }
    }

    view! {
        #[name="address_bar"]
        gtk::Entry {
            placeholder_text: Some("Enter gemini:// address..."),
            max_width_chars: 100,
            text: &self.model.url,

            activate(entry) => {
                let text = entry.get_text().expect("cannot get addressbar text").to_string();
                Msg::Goto(text)
            }
        }
    }
}
