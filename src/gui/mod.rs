mod header;

use gtk::prelude::*;
use gtk::Inhibit;
use gtk::WidgetExt;
use relm::{connect, init, Component, Relm, Widget};
use relm_derive::{widget, Msg};

use header::{Header, Msg as HeaderMsg};
use relm_moonrender::{Moonrender, Msg as MoonrenderMsg};

#[derive(Msg)]
pub enum Msg {
    Quit,
    Error(String, String),

    Goto(String),
    GotoDone,

    Redirect(String),
}

pub struct Model {
    relm: Relm<Win>,
    header: Component<Header>,

    status_ctx_goto: u32,
}

#[widget]
impl Widget for Win {
    fn model(relm: &Relm<Self>, _: ()) -> Model {
        let header = init::<Header>(()).expect("Header cannot be initialized");

        Model {
            header,
            relm: relm.clone(),

            status_ctx_goto: 0,
        }
    }

    fn init_view(&mut self) {
        let header = &self.model.header;
        let content = &self.content;

        self.model.status_ctx_goto = self.status.get_context_id("Navigation");
        self.status.hide();

        connect!(header@HeaderMsg::Goto(ref url), self.model.relm, Msg::Goto(url.to_owned()));

        connect!(content@MoonrenderMsg::Done, self.model.relm, Msg::GotoDone);
        connect!(content@MoonrenderMsg::Goto(ref url), self.model.relm, Msg::Redirect(url.to_owned()));
        connect!(content@MoonrenderMsg::Error(ref e), self.model.relm, Msg::Error(e.to_string(), {
            let mut err_str = String::new();

            for e in e.chain().skip(1) {
                err_str += &format!("because: {}\n", e.to_string());
            }

            err_str.trim().to_owned()
        }));

        let url = crate::CONFIG.homepage.clone();
        self.model.relm.stream().emit(Msg::Redirect(url.clone()));
        self.model.relm.stream().emit(Msg::Goto(url));
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

            Msg::Goto(url) => {
                self.content.emit(MoonrenderMsg::Goto(url.clone()));

                self.status.show();

                self.status.remove_all(self.model.status_ctx_goto);
                self.status
                    .push(self.model.status_ctx_goto, &format!("Loading {}...", url));
            }

            Msg::Redirect(url) => {
                self.model.header.emit(HeaderMsg::Redirect(url.clone()));

                self.status.show();

                self.status.remove_all(self.model.status_ctx_goto);
                self.status
                    .push(self.model.status_ctx_goto, &format!("Loading {}...", url));
            }

            Msg::GotoDone => {
                self.status.remove_all(self.model.status_ctx_goto);

                // this is useless
                if let Some(area) = self.status.get_message_area() {
                    if let Some(widget) = area.get_children().iter().cloned().next() {
                        if let Ok(label) = widget.downcast::<gtk::Label>() {
                            if let Some(text) = label.get_text() {
                                if !text.is_empty() {
                                    return;
                                }
                            }
                        };
                    }
                }

                self.status.hide();
            }
        }
    }

    view! {
        #[name="window"]
        gtk::ApplicationWindow {
            titlebar: Some(self.model.header.widget()),

            gtk::Box {
                orientation: gtk::Orientation::Vertical,

                #[name="content"]
                Moonrender(crate::CONFIG.theme.clone()) {
                    child: {
                        expand: true
                    },
                },

                #[name="status"]
                gtk::Statusbar {},
            },

            delete_event(_, _) => (Msg::Quit, Inhibit(false)),
        }
    }
}
