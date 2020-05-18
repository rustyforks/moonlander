use anyhow::Context;
use gtk::prelude::*;
use relm::{Channel, DrawHandler, Relm, Widget};
use relm_derive::{widget, Msg};
use std::sync::mpsc;

pub use moonrender;
use moonrender::{Msg as RendererMsg, Renderer};

#[derive(Msg)]
pub enum Msg {
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
    renderer: Renderer<relm::drawing::DrawContext<gtk::DrawingArea>>,
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
                self.model.renderer.reset();

                self.model
                    .renderer
                    .set_url(&url)
                    .context("cannot set renderer url")?;

                self.model
                    .request
                    .send(url)
                    .context("cannot send url to connection thread")?;
            }

            Msg::ConnectionMessage(gemini::Message::Chunk(chunk)) => {
                if let Err(e) = self.model.renderer.new_page_chunk(&chunk) {
                    self.model.relm.stream().emit(Msg::Error(e));
                }
            }

            Msg::ConnectionMessage(gemini::Message::MIME(mime)) => {
                self.model.renderer.set_mime(&mime);
            }

            Msg::ConnectionMessage(gemini::Message::Redirect(url)) => {
                self.model.relm.stream().emit(Msg::Goto(url));
            }

            Msg::ConnectionMessage(gemini::Message::Error(e)) => {
                self.model.relm.stream().emit(Msg::Error(e));
            }

            Msg::ConnectionMessage(gemini::Message::Done) => {
                self.model.relm.stream().emit(Msg::Done);
            }

            Msg::Done => { /* listened by parent */ }
            Msg::Error(_) => { /* listened by parent */ }
        }

        Ok(())
    }
}
