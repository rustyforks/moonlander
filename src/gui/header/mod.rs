mod address_bar;

use gtk::prelude::*;
use relm::{connect, init, Component, Relm, Widget};
use relm_derive::{widget, Msg};

use address_bar::{AddressBar, Msg as ABMsg};

#[derive(Msg)]
pub enum Msg {
    Goto(String),
    Redirect(String),

    Back,
    Forward,
    Refresh,

    EnableBtnBack(bool),
    EnableBtnForward(bool),
    EnableBtnRefresh(bool),
}

pub struct Model {
    relm: Relm<Header>,
    address_bar: Component<AddressBar>,

    has_history_back: bool,
    has_history_forwards: bool,
    has_refresh: bool,
}

#[widget]
impl Widget for Header {
    fn model(relm: &Relm<Self>, _: ()) -> Model {
        let address_bar = init::<AddressBar>(()).expect("Header cannot be initialized");
        Model {
            address_bar,
            relm: relm.clone(),

            has_history_back: false,
            has_history_forwards: false,
            has_refresh: false,
        }
    }

    fn init_view(&mut self) {
        let addr = &self.model.address_bar;
        connect!(addr@ABMsg::Goto(ref url), self.model.relm, Msg::Goto(url.to_owned()));
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Goto(_) => { /* listened from parent */ }
            Msg::Redirect(url) => self.model.address_bar.emit(ABMsg::Redirect(url)),

            Msg::Back => { /* listened from parent */ }
            Msg::Forward => { /* listened from parent */ }
            Msg::Refresh => { /* listened from parent */ }

            Msg::EnableBtnBack(b) => self.model.has_history_back = b,
            Msg::EnableBtnForward(b) => self.model.has_history_forwards = b,
            Msg::EnableBtnRefresh(b) => self.model.has_refresh = b,
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
                sensitive: self.model.has_history_back,

                clicked => Msg::Back,
            },

            #[name="btn_forward"]
            gtk::Button {
                image: Some(&gtk::Image::new_from_icon_name(Some("gtk-go-forward"), gtk::IconSize::SmallToolbar)),
                sensitive: self.model.has_history_forwards,

                clicked => Msg::Forward,
            },

            #[name="btn_refresh"]
            gtk::Button {
                image: Some(&gtk::Image::new_from_icon_name(Some("gtk-refresh"), gtk::IconSize::SmallToolbar)),
                sensitive: self.model.has_refresh,

                clicked => Msg::Refresh,
            },
        },
    }
}
