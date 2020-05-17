mod address_bar;

use gtk::prelude::*;
use relm::{connect, init, Component, Relm, Widget};
use relm_derive::{widget, Msg};

use address_bar::{AddressBar, Msg as ABMsg};

#[derive(Msg)]
pub enum Msg {
    Back,
    Forward,
    Refresh,
    Goto(String),
    Redirect(String),
}

pub struct Model {
    relm: Relm<Header>,
    address_bar: Component<AddressBar>,
}

#[widget]
impl Widget for Header {
    fn model(relm: &Relm<Self>, _: ()) -> Model {
        let address_bar = init::<AddressBar>(()).expect("Header cannot be initialized");
        Model {
            address_bar,
            relm: relm.clone(),
        }
    }

    fn init_view(&mut self) {
        let addr = &self.model.address_bar;
        connect!(addr@ABMsg::Goto(ref url), self.model.relm, Msg::Goto(url.to_owned()));
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Back => {}
            Msg::Forward => {}
            Msg::Refresh => {}
            Msg::Goto(_) => { /* listened from parent */ }
            Msg::Redirect(url) => self.model.address_bar.emit(ABMsg::Redirect(url)),
        }
    }

    view! {
        #[name="header"]
        gtk::HeaderBar {
            show_close_button: true,
            custom_title: Some(self.model.address_bar.widget()),

            #[name="btn_back"]
            gtk::Button {
                image: Some(&gtk::Image::new_from_icon_name(Some("gtk-go-back"), gtk::IconSize::SmallToolbar)),

                clicked => Msg::Back,
            },

            #[name="btn_forward"]
            gtk::Button {
                image: Some(&gtk::Image::new_from_icon_name(Some("gtk-go-forward"), gtk::IconSize::SmallToolbar)),

                clicked => Msg::Forward,
            },

            #[name="btn_refresh"]
            gtk::Button {
                image: Some(&gtk::Image::new_from_icon_name(Some("gtk-refresh"), gtk::IconSize::SmallToolbar)),

                clicked => Msg::Refresh,
            },
        },
    }
}
