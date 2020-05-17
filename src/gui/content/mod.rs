use gtk::prelude::*;
use relm::{Channel, DrawHandler, Relm, Widget};
use relm_derive::{widget, Msg};
use std::sync::mpsc;

use crate::{gemini, renderer::Renderer};

#[derive(Msg)]
pub enum Msg {
    UpdateDrawBuffer,
    ConnectionMessage(gemini::Message),
    Goto(String),
}

pub struct Model {
    relm: Relm<Content>,

    request: mpsc::Sender<String>,
    _channel: Channel<gemini::Message>,

    draw: DrawHandler<gtk::DrawingArea>,
    renderer: Renderer,
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
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::UpdateDrawBuffer => {
                let ctx = self.model.draw.get_context();
                self.model.renderer.render(&ctx);
            }

            Msg::Goto(url) => {
                self.model.renderer.reset();
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
            },
        }
    }
}
