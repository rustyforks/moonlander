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

    Goto(String),
    GotoDone,

    Redirect(String),
    UnsupportedRedirect(String),

    Back,
    Forward,
    Refresh,

    ShowTooltip(String),
    HideTooltip,
}

pub struct Model {
    relm: Relm<Win>,
    header: Component<Header>,

    status_ctx_goto: u32,
    status_ctx_tooltip: u32,

    history: Vec<String>,
    forward_history: Vec<String>,
}

#[widget]
impl Widget for Win {
    fn model(relm: &Relm<Self>, _: ()) -> Model {
        let header = init::<Header>(()).expect("Header cannot be initialized");

        Model {
            header,
            relm: relm.clone(),

            status_ctx_goto: 0,
            status_ctx_tooltip: 0,

            history: vec![],
            forward_history: vec![],
        }
    }

    fn init_view(&mut self) {
        let header = &self.model.header;
        let content = &self.content;

        self.model.status_ctx_goto = self.status.get_context_id("Navigation");
        self.model.status_ctx_tooltip = self.status.get_context_id("Tooltip");

        self.status.hide();

        connect!(header@HeaderMsg::Goto(ref url), self.model.relm, Msg::Goto(url.to_owned()));

        connect!(header@HeaderMsg::Back, self.model.relm, Msg::Back);
        connect!(header@HeaderMsg::Forward, self.model.relm, Msg::Forward);
        connect!(header@HeaderMsg::Refresh, self.model.relm, Msg::Refresh);

        connect!(content@MoonrenderMsg::Back, self.model.relm, Msg::Back);
        connect!(content@MoonrenderMsg::Forward, self.model.relm, Msg::Forward);

        connect!(content@MoonrenderMsg::ShowTooltip(ref tip), self.model.relm, Msg::ShowTooltip(tip.to_owned()));
        connect!(content@MoonrenderMsg::HideTooltip, self.model.relm, Msg::HideTooltip);

        connect!(content@MoonrenderMsg::Done, self.model.relm, Msg::GotoDone);

        connect!(content@MoonrenderMsg::Goto(ref url), self.model.relm, Msg::Redirect(url.to_owned()));
        connect!(content@MoonrenderMsg::UnsupportedRedirect(ref url), self.model.relm, Msg::UnsupportedRedirect(url.clone()));

        let url = crate::CONFIG.homepage.clone();
        self.model.relm.stream().emit(Msg::Goto(url));
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Quit => gtk::main_quit(),

            Msg::Goto(url) => {
                self.content.emit(MoonrenderMsg::Goto(url.clone()));

                self.status.show();

                self.status.remove_all(self.model.status_ctx_goto);
                self.status
                    .push(self.model.status_ctx_goto, &format!("Loading {}...", url));
            }

            Msg::UnsupportedRedirect(url) => {
                let d = gtk::MessageDialog::new(
                    Some(&self.window),
                    gtk::DialogFlags::all(),
                    gtk::MessageType::Question,
                    gtk::ButtonsType::YesNo,
                    &format!("Moonlander does not support the following URL:\n\n{}\n\nDo you want to open it in your default browser?", url),
                );

                d.set_title("Unsupported URL");
                d.connect_response(move |d, resp| {
                    if let gtk::ResponseType::Yes = resp {
                        webbrowser::open(&url).unwrap();
                    }

                    d.destroy();
                });

                self.model.history.pop();
                if self.model.history.len() < 2 {
                    self.model.header.emit(HeaderMsg::EnableBtnBack(false));
                }

                self.model.relm.stream().emit(Msg::GotoDone);

                d.show();
            }

            Msg::Redirect(url) => {
                self.model.header.emit(HeaderMsg::Redirect(url.clone()));

                self.status.show();

                self.model.history.push(url.clone());

                self.model.header.emit(HeaderMsg::EnableBtnRefresh(true));
                log::debug!("r: {:?}", self.model.history);
                if self.model.history.len() >= 2 {
                    self.model.header.emit(HeaderMsg::EnableBtnBack(true));
                }

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

            Msg::ShowTooltip(tip) => {
                self.status.show();
                self.status.push(self.model.status_ctx_tooltip, &tip);
            }

            Msg::HideTooltip => {
                self.status.remove_all(self.model.status_ctx_tooltip);

                // this is even more useless
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

            Msg::Back => {
                if let Some(prev) = self.model.history.pop() {
                    self.model.forward_history.push(prev)
                }

                if let Some(url) = self.model.history.last() {
                    let url = url.to_owned();

                    self.model.history.pop(); // the signals push again
                    self.content.emit(MoonrenderMsg::Goto(url.clone()));
                    self.model.header.emit(HeaderMsg::Redirect(url));
                }

                if self.model.history.len() < 2 {
                    self.model.header.emit(HeaderMsg::EnableBtnBack(false));
                }

                if !self.model.forward_history.is_empty() {
                    self.model.header.emit(HeaderMsg::EnableBtnForward(true));
                }
            }
            Msg::Forward => {
                if let Some(url) = self.model.forward_history.pop() {
                    self.content.emit(MoonrenderMsg::Goto(url.clone()));
                    self.model.header.emit(HeaderMsg::Redirect(url));
                }

                if self.model.forward_history.is_empty() {
                    self.model.header.emit(HeaderMsg::EnableBtnForward(false));
                }
            }
            Msg::Refresh => {
                if let Some(url) = self.model.history.pop() {
                    self.content.emit(MoonrenderMsg::Goto(url.clone()));
                    self.model.header.emit(HeaderMsg::Redirect(url));
                }
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
