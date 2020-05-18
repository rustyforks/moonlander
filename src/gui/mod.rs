mod content;
mod header;

use gtk::prelude::*;
use gtk::Inhibit;
use gtk::WidgetExt;
use relm::{connect, init, Component, Relm, Widget};
use relm_derive::{widget, Msg};

use content::{Content, Msg as ContentMsg};
use header::{Header, Msg as HeaderMsg};

#[derive(Msg)]
pub enum Msg {
    Quit,
    Error(String, String),

    Goto(String),
    Redirect(String),
}

pub struct Model {
    relm: Relm<Win>,
    header: Component<Header>,
}

#[widget]
impl Widget for Win {
    fn model(relm: &Relm<Self>, _: ()) -> Model {
        let header = init::<Header>(()).expect("Header cannot be initialized");

        Model {
            header,
            relm: relm.clone(),
        }
    }

    fn init_view(&mut self) {
        let header = &self.model.header;
        let content = &self.content;

        connect!(header@HeaderMsg::Goto(ref url), self.model.relm, Msg::Goto(url.to_owned()));

        connect!(content@ContentMsg::Goto(ref url), self.model.relm, Msg::Redirect(url.to_owned()));
        connect!(content@ContentMsg::Error(ref e), self.model.relm, Msg::Error(e.to_string(), {
            let mut err_str = String::new();

            for e in e.chain().skip(1) {
                err_str += &format!("because: {}\n", e.to_string());
            }

            err_str.trim().to_owned()
        }));
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Quit => gtk::main_quit(),
            Msg::Error(err, trace) => {
                log::error!("{}", err);
                trace.split('\n').for_each(|l| log::error!("{}", l));

                let d = gtk::MessageDialog::new(
                    Some(&self.window),
                    gtk::DialogFlags::all(),
                    gtk::MessageType::Error,
                    gtk::ButtonsType::Close,
                    &trace,
                );

                d.set_title(&err);
                d.connect_response(|d, _| {
                    d.destroy();
                });

                d.show();
            }

            Msg::Goto(url) => self.content.emit(ContentMsg::Goto(url)),
            Msg::Redirect(url) => self.model.header.emit(HeaderMsg::Redirect(url)),
        }
    }

    view! {
        #[name="window"]
        gtk::ApplicationWindow {
            titlebar: Some(self.model.header.widget()),

            gtk::ScrolledWindow {
                min_content_width: 400,
                min_content_height: 400,

                #[name="content"]
                Content {},
            },

            delete_event(_, _) => (Msg::Quit, Inhibit(false)),
        }
    }
}