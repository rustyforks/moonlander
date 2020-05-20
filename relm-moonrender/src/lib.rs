use anyhow::Context;
use gtk::prelude::*;
use relm::{Channel, DrawHandler, Relm, Widget};
use relm_derive::{widget, Msg};
use std::sync::mpsc;
use url::Url;

pub use moonrender;
use moonrender::{Msg as RendererMsg, Renderer};

const ERROR_PAGE: &str = include_str!("error.gemini");
const SUPPORTED_PROTOCOLS: &[&str] = &["gemini"];

#[derive(Msg)]
pub enum Msg {
    UnsupportedRedirect(String),
    Goto(String),
    Error(anyhow::Error),
    Done,

    UpdateDrawBuffer,
    Click(gdk::EventButton),
    ConnectionMessage(gemini::Message),
}

pub struct Model {
    relm: Relm<Moonrender>,

    request: mpsc::Sender<String>,
    _channel: Channel<gemini::Message>,

    draw: DrawHandler<gtk::DrawingArea>,
    renderer: Renderer,

    redirect_counter: u8,
}

#[widget]
impl Widget for Moonrender {
    fn model(relm: &Relm<Self>, theme: moonrender::Theme) -> Model {
        let stream = relm.stream().clone();

        let (channel, sender) = Channel::new(move |msg| stream.emit(Msg::ConnectionMessage(msg)));
        let (send, recv) = mpsc::channel::<String>();

        std::thread::spawn(move || {
            let recv = recv;
            let sender = sender;

            while let Ok(data) = recv.recv() {
                if let Err(e) = gemini::get(&data, |msg| {
                    sender.send(msg).expect("Cannot send message to UI thread")
                }) {
                    sender
                        .send(gemini::Message::Error(e))
                        .expect("Cannot send message to UI thread")
                } else {
                    sender
                        .send(gemini::Message::Done)
                        .expect("Cannot send message to UI thread")
                }
            }
        });

        Model {
            relm: relm.clone(),

            request: send,
            _channel: channel,

            draw: DrawHandler::new().expect("Cannot create content draw handler"),
            renderer: Renderer::new(theme),

            redirect_counter: 0,
        }
    }

    fn init_view(&mut self) {
        self.model.draw.init(&self.content);

        self.content.add_events(gdk::EventMask::ALL_EVENTS_MASK); // TODO: maybe make this more granular
    }

    fn update(&mut self, event: Msg) {
        if let Err(e) = self.try_update(event) {
            self.model.relm.stream().emit(Msg::Error(e))
        }
    }

    view! {
        #[name="window"]
        gtk::ScrolledWindow {
            min_content_width: 400,
            min_content_height: 400,

            gtk::Box {
                orientation: gtk::Orientation::Horizontal,

                #[name="content"]
                gtk::DrawingArea {
                    child: {
                        expand: true,
                    },

                    can_focus: true,

                    draw(_, _) => (Msg::UpdateDrawBuffer, Inhibit(false)),
                    button_press_event(_, e) => (Msg::Click(e.clone()), Inhibit(false)),
                },
            },
        }
    }
}

impl Moonrender {
    fn try_update(&mut self, event: Msg) -> anyhow::Result<()> {
        match event {
            Msg::UpdateDrawBuffer => {
                let ctx = self.model.draw.get_context();

                let (size, _) = self.content.get_preferred_size();
                let (y, height) = if let Some(adjustment) = self.window.get_vadjustment() {
                    (adjustment.get_value(), adjustment.get_page_size())
                } else {
                    (0.0, size.height as f64)
                };

                let (_, h) = self.model.renderer.render(y, height as f64, &ctx);
                let w = size.width;

                self.content.set_size_request(w, h);
            }

            Msg::Click(e) => {
                let coords = e.get_coords().context("click coords empty")?;
                let message = self.model.renderer.click(coords);

                if let Some(msg) = message {
                    match msg {
                        RendererMsg::Goto(url) => self.model.relm.stream().emit(Msg::Goto(url)),
                    }
                }
            }

            Msg::Goto(url) => {
                let url = Url::parse(&url).context("Cannot parse URL")?;

                if !SUPPORTED_PROTOCOLS.contains(&url.scheme()) {
                    self.model
                        .relm
                        .stream()
                        .emit(Msg::UnsupportedRedirect(url.to_string()));
                } else {
                    self.model.renderer.reset();

                    self.model
                        .request
                        .send(url.to_string())
                        .context("cannot send url to connection thread")?;

                    self.model
                        .renderer
                        .set_url(url)
                        .context("cannot set renderer url")?;
                }
            }

            Msg::ConnectionMessage(gemini::Message::Chunk(chunk)) => {
                self.model.renderer.new_page_chunk(&chunk)?;
            }

            Msg::ConnectionMessage(gemini::Message::MIME(mime)) => {
                self.model
                    .renderer
                    .set_mime(mime.parse().context("Cannot parse response mimetype")?);
            }

            Msg::ConnectionMessage(gemini::Message::Redirect(url)) => {
                self.model.redirect_counter += 1;
                if self.model.redirect_counter > 5 {
                    return Err(anyhow::anyhow!("Redirect loop detected"));
                } else {
                    self.model.relm.stream().emit(Msg::Goto(url));
                }
            }

            Msg::ConnectionMessage(gemini::Message::Error(e)) => {
                self.model.relm.stream().emit(Msg::Error(e));
            }

            Msg::ConnectionMessage(gemini::Message::Done) => {
                self.model.relm.stream().emit(Msg::Done);
            }

            Msg::ConnectionMessage(gemini::Message::ErrorResponse(code, msg)) => {
                let mut error_page =
                    ERROR_PAGE.replace("{code}", &format!("Error {}", code.to_string()));

                error_page = error_page.replace("{status}", &msg);
                error_page = error_page.replace(
                    "{message}",
                    &if let Some(msg) = gemini::get_message_for(code) {
                        msg
                    } else {
                        "Unknown Status Code".to_owned()
                    },
                );

                self.model.renderer.set_mime("text/gemini".parse().unwrap());
                self.model.renderer.new_page_chunk(&error_page)?;

                self.model.relm.stream().emit(Msg::Done);
            }

            Msg::Error(e) => {
                let mut error_page = ERROR_PAGE.replace("{code}", &e.to_string());
                let mut err_str = String::new();

                for e in e.chain().skip(1) {
                    err_str += &format!("because: {}\n", e.to_string());
                }

                error_page = error_page.replace("{message}", err_str.trim());

                self.model.renderer.set_mime("text/gemini".parse().unwrap());
                self.model.renderer.new_page_chunk(&error_page)?;

                self.model.relm.stream().emit(Msg::Done);
            }

            Msg::Done => { /* listened by parent */ }
            Msg::UnsupportedRedirect(_) => { /* listened by parent */ }
        }

        Ok(())
    }
}
