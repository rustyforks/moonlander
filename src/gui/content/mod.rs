use gtk::prelude::*;
use relm::{Channel, DrawHandler, Relm, Widget};
use relm_derive::{widget, Msg};
use std::sync::mpsc;

use crate::{
    gemini,
    renderer::{Msg as RendererMsg, Renderer},
};

#[derive(Msg)]
pub enum Msg {
    ConnectionMessage(gemini::Message),
    Goto(String),

    UpdateDrawBuffer,
    Click(gdk::EventButton),
}

pub struct Model {
    relm: Relm<Content>,

    request: mpsc::Sender<String>,
    _channel: Channel<gemini::Message>,

    draw: DrawHandler<gtk::DrawingArea>,
    renderer: Renderer<relm::drawing::DrawContext<gtk::DrawingArea>>,
}

#[widget]
impl Widget for Content {
    fn model(relm: &Relm<Self>, _: ()) -> Model {
        let stream = relm.stream().clone();

        let (channel, sender) = Channel::new(move |msg| stream.emit(Msg::ConnectionMessage(msg)));
        let (send, recv) = mpsc::channel::<String>();

        std::thread::spawn(move || {
            let recv = recv;
            let sender = sender;

            while let Ok(data) = recv.recv() {
                crate::gemini::get(&data, |msg| {
                    sender.send(msg).expect("Cannot send message to UI thread")
                })
                .expect("Cannot get url");
            }
        });

        Model {
            relm: relm.clone(),

            request: send,
            _channel: channel,

            draw: DrawHandler::new().expect("Cannot create content draw handler"),
            renderer: Renderer::new(),
        }
    }

    fn init_view(&mut self) {
        self.model.draw.init(&self.content);

        self.content.add_events(gdk::EventMask::ALL_EVENTS_MASK); // TODO: maybe make this more granular
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::UpdateDrawBuffer => {
                let ctx = self.model.draw.get_context();

                let (_, h) = self.model.renderer.render(&ctx);
                let w = self.content.get_preferred_size().1.width;

                self.content.set_size_request(w, h);
            }

            Msg::Click(e) => {
                let message = self
                    .model
                    .renderer
                    .click(e.get_coords().expect("click coords empty"));

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
                    .expect("cannot set renderer url");

                self.model
                    .request
                    .send(url)
                    .expect("cannot send url to connection thread");
            }

            Msg::ConnectionMessage(gemini::Message::Chunk(chunk)) => {
                self.model.renderer.new_page_chunk(&chunk)
            }

            Msg::ConnectionMessage(gemini::Message::MIME(mime)) => {
                self.model.renderer.set_mime(&mime)
            }

            Msg::ConnectionMessage(gemini::Message::Redirect(url)) => {
                self.model.relm.stream().emit(Msg::Goto(url))
            }
        }
    }

    view! {
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
        }
    }
}
