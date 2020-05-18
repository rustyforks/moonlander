mod verifier;

use anyhow::{Context, Result};
use std::{
    io::{Read, Write},
    net::TcpStream,
    sync::Arc,
};

lazy_static::lazy_static! {
    static ref TLS: Arc<rustls::ClientConfig> = Arc::new({
        let mut cfg = rustls::ClientConfig::new();
        cfg.dangerous().set_certificate_verifier(Arc::new(verifier::GeminiVerifier::new()));
        cfg
    });
}

pub enum Message {
    Chunk(String),
    MIME(String),
    Redirect(String),
}

pub fn get(url: &str, chunk_callback: impl Fn(Message) -> ()) -> Result<()> {
    let url = url::Url::parse(&url).context("Cannot parse URL")?;

    let host = url.host_str().context("Url doesn't have host")?;
    let port = url.port().unwrap_or(1965);

    let dns =
        webpki::DNSNameRef::try_from_ascii_str(host).context("Cannot get DNSNameRef from host")?;

    let mut raw = TcpStream::connect((host, port))?;
    let mut tls = rustls::ClientSession::new(&TLS, dns);

    let mut stream = rustls::Stream::new(&mut tls, &mut raw);

    let mut buf: [u8; 1024] = [0; 1024];
    let mut is_content = false;

    log::info!("Requesting {}", url);
    write!(stream, "{}\r\n", url).expect("Cannot write gemini header");

    let mut break_response = Ok(());

    loop {
        log::debug!("reading");
        let len = {
            match stream.read(&mut buf) {
                Ok(len) => len,
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::ConnectionAborted {
                        log::debug!("connection aborted, assume server intended to do that");
                        chunk_callback(Message::Chunk("\n".to_owned()));
                    } else {
                        break_response = Err(e).context("Cannot read");
                    }

                    break;
                }
            }
        };

        if len == 0 {
            break;
        }

        let data = String::from_utf8_lossy(&buf[0..len]);
        let content = if is_content {
            log::debug!("content, pass through");
            data.as_ref()
        } else {
            log::debug!("parse headers: {}", data);

            let mut split_data = data.splitn(1, "\r\n");

            let header = split_data.next().context("No header received")?;
            let mut header = header.split_whitespace();

            let status = header
                .next()
                .context("No status given?")?
                .parse::<u8>()
                .context("Status isn't number 0-255")?;

            let meta = header.next().context("No metadata given?")?;

            log::info!("{}: {} {}", url, status, meta);
            if status >= 20 && status < 30 {
                /* success */
            } else if status >= 30 && status < 40 {
                chunk_callback(Message::Redirect(meta.to_owned()));
                break;
            } else {
                todo!("non success responses");
            }

            is_content = true;

            chunk_callback(Message::MIME(meta.to_owned()));
            split_data.next().unwrap_or("")
        }
        .to_owned();

        chunk_callback(Message::Chunk(content));
    }

    log::debug!("gemini machine broke");
    break_response
}
