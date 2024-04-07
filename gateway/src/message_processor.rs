use crate::discord_client::DiscordClient;
use futures_channel::mpsc::UnboundedSender;
use tokio_tungstenite::tungstenite::{Error, Message};

use crate::heartbeat::WebsocketUpdate;
use crate::inbound_payloads::{InboundEvent, InboundPayload};
use crate::payloads::DiscordGatewayResponse;
use crate::TLDR_MESSAGE_LENGTH;

pub struct MessageProcessor<T: DiscordClient> {
    resume_gateway: String,
    discord_token: String,
    session_id: Option<String>,
    sender: UnboundedSender<Message>,
    internal_tx: UnboundedSender<WebsocketUpdate>,
    discord_client: T,
}

impl<T: DiscordClient> MessageProcessor<T> {
    pub fn new(
        resume_gateway: String,
        discord_token: String,
        sender: UnboundedSender<Message>,
        internal_tx: UnboundedSender<WebsocketUpdate>,
        discord_client: T,
    ) -> Self {
        Self {
            resume_gateway,
            discord_token,
            session_id: None,
            sender,
            internal_tx,
            discord_client,
        }
    }

    pub async fn process(&mut self, message: Result<Message, Error>) {
        match message {
            Ok(value) => match value {
                Message::Text(text) => {
                    let payload: InboundPayload =
                        serde_json::from_str(text.as_str()).expect("deserialize inbound payload");
                    self.internal_tx
                        .unbounded_send(WebsocketUpdate::SequenceNumber(payload.s))
                        .expect("send sequence number to heartbeat thread");
                    let event = payload.event();
                    match event {
                        InboundEvent::Hello(hello) => {
                            self.internal_tx
                                .unbounded_send(WebsocketUpdate::HeartbeatInterval(
                                    hello.heartbeat_interval,
                                ))
                                .expect("send updated interval to heartbeat thread");
                            let payload = serde_json::to_vec(&DiscordGatewayResponse::identify(
                                self.discord_token.as_str(),
                            ))
                            .expect("serialize identify payload");
                            self.sender
                                .unbounded_send(Message::binary(payload))
                                .expect("send identify payload through websocket");
                        }
                        InboundEvent::Ready(ready) => {
                            self.resume_gateway = ready.resume_gateway_url;
                            self.session_id = Some(ready.session_id);
                        }
                        InboundEvent::Ack => {}
                        InboundEvent::GuildCreate(_) => {}
                        InboundEvent::MessageCreate(message_create) => {
                            if message_create.content.len() > TLDR_MESSAGE_LENGTH {
                                let author = match message_create.author.global_name {
                                    Some(global_name) => global_name,
                                    None => message_create.author.username,
                                };
                                println!(
                                    "tl;dr message: {} chars, {} ({}) : {}",
                                    message_create.content.len(),
                                    author,
                                    message_create.channel_id,
                                    message_create.content
                                );
                                match self
                                    .discord_client
                                    .tldr(
                                        message_create.channel_id.as_str(),
                                        message_create.id.as_str(),
                                        author.as_str(),
                                        message_create.content.as_str(),
                                    )
                                    .await
                                {
                                    Ok(_) => {}
                                    Err(err) => println!("failed to call client tldr: {:?}", err),
                                }
                            } else {
                                println!("{}", message_create.content.len());
                            }
                        }
                        InboundEvent::MessageDelete(_) => {}
                        InboundEvent::MessageReactionAdd(_) => {}
                        InboundEvent::MessageReactionRemove(_) => {}
                        InboundEvent::MessageUpdate(_) => {}
                        InboundEvent::Resumed => {}
                        InboundEvent::TypingStart(_) => {}
                        InboundEvent::Reconnect => println!("TODO: reconnect cleanly"),
                        InboundEvent::Unknown => println!("unknown event: {}", text),
                    }
                }
                Message::Close(close) => {
                    println!("closing cleanly: {:?}", close);
                }
                v => panic!("received an event with a different value: {:?}\n", v),
            },
            Err(err) => match err {
                Error::ConnectionClosed => {
                    println!("connection closed");
                }
                e => panic!("{}", e),
            },
        }
    }
}
