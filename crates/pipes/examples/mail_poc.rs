//! Sends an email with an attachment over SMTP, then reads it back over IMAP
//! and lists its attachments.
//!
//! Start GreenMail first with `scripts/greenmail.sh up`, then run
//! `cargo run -p pipes --example mail_poc [host]`. The host defaults to
//! `localhost`.

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use futures::TryStreamExt;
use lettre::message::{Attachment, MultiPart, SinglePart, header::ContentType};
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use mail_parser::{MessageParser, MimeHeaders};
use tokio::net::TcpStream;

const SMTP_PORT: u16 = 3025;
const IMAP_PORT: u16 = 3143;
const FROM: &str = "sender@pipes.test";
const TO: &str = "inbox@pipes.test";

type BoxError = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    let host = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "localhost".into());
    let stamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
    let subject = format!("pipes poc {stamp}");

    send(&host, &subject).await?;
    println!("sent '{subject}' to {TO}");

    let raw = receive(&host, &subject).await?;
    let message = MessageParser::default()
        .parse(&raw)
        .ok_or("could not parse the received message")?;
    println!(
        "received '{}' from {}",
        message.subject().unwrap_or_default(),
        message
            .from()
            .and_then(|from| from.first())
            .and_then(|addr| addr.address())
            .unwrap_or_default()
    );
    for attachment in message.attachments() {
        println!(
            "  attachment {} ({} bytes): {}",
            attachment.attachment_name().unwrap_or("<unnamed>"),
            attachment.contents().len(),
            String::from_utf8_lossy(attachment.contents())
        );
    }
    Ok(())
}

async fn send(host: &str, subject: &str) -> Result<(), BoxError> {
    let email = Message::builder()
        .from(FROM.parse()?)
        .to(TO.parse()?)
        .subject(subject)
        .multipart(
            MultiPart::mixed()
                .singlepart(SinglePart::plain(String::from("See attached.")))
                .singlepart(Attachment::new(String::from("invoice.csv")).body(
                    String::from("id,amount\n1,42.00\n"),
                    ContentType::TEXT_PLAIN,
                )),
        )?;
    let mailer = AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(host)
        .port(SMTP_PORT)
        .build();
    mailer.send(email).await?;
    Ok(())
}

/// Polls the inbox until the message with `subject` arrives, and returns its
/// raw RFC 822 bytes. GreenMail has auth disabled, so the address works as
/// both login and password.
async fn receive(host: &str, subject: &str) -> Result<Vec<u8>, BoxError> {
    let stream = TcpStream::connect((host, IMAP_PORT)).await?;
    let mut client = async_imap::Client::new(stream);
    client.read_response().await?.ok_or("no IMAP greeting")?;
    let mut session = client.login(TO, TO).await.map_err(|(e, _)| e)?;
    session.select("INBOX").await?;

    for _ in 0..20 {
        let uids = session.uid_search(format!("SUBJECT \"{subject}\"")).await?;
        if let Some(uid) = uids.into_iter().max() {
            let fetches: Vec<_> = session
                .uid_fetch(uid.to_string(), "RFC822")
                .await?
                .try_collect()
                .await?;
            let raw = fetches
                .first()
                .and_then(|fetch| fetch.body())
                .ok_or("fetch returned no body")?
                .to_vec();
            session.logout().await?;
            return Ok(raw);
        }
        tokio::time::sleep(Duration::from_millis(250)).await;
        session.noop().await?;
    }
    Err(format!("'{subject}' did not arrive within 5 seconds").into())
}
