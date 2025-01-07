use crate::error::Error;
use crate::heartbeat::WebsocketUpdate;
use crate::inbound_payloads::{InboundEvent, InboundPayload};
use crate::payloads::DiscordGatewayResponse;
use crate::tldr;
use crate::TLDR_MESSAGE_LENGTH;
use futures_channel::mpsc::UnboundedSender;
use pog_common::{Authorization, TlDrMessage};
use tokio_tungstenite::tungstenite::Message;

pub struct MessageProcessor {
    resume_gateway: String,
    discord_token: String,
    session_id: Option<String>,
    authorization: Authorization,
    gemini_token: String,
    sender: UnboundedSender<Message>,
    internal_tx: UnboundedSender<WebsocketUpdate>,
}

impl MessageProcessor {
    pub fn new(
        resume_gateway: String,
        discord_token: String,
        authorization: Authorization,
        gemini_token: String,
        sender: UnboundedSender<Message>,
        internal_tx: UnboundedSender<WebsocketUpdate>,
    ) -> Self {
        Self {
            resume_gateway,
            discord_token,
            session_id: None,
            authorization,
            gemini_token,
            sender,
            internal_tx,
        }
    }

    pub async fn process(
        &mut self,
        message: Result<Message, tokio_tungstenite::tungstenite::Error>,
    ) {
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
                            if message_create.content.len() > TLDR_MESSAGE_LENGTH
                                && message_create.author.bot != Some(true)
                            {
                                let author = match message_create.author.global_name {
                                    Some(global_name) => global_name,
                                    None => message_create.author.username.clone(),
                                };
                                let tldr_message = TlDrMessage {
                                    authorization: self.authorization.clone(),
                                    original_message_id: message_create.id,
                                    channel_id: message_create.channel_id,
                                    gemini_key: self.gemini_token.clone(),
                                    author,
                                    message: message_create.content,
                                };
                                match tldr::tldr(tldr_message).await {
                                    Ok(()) => {}
                                    Err(err) => match err {
                                        Error::ClientFailure(msg) => {
                                            println!("failed to call client tldr: {:?}", msg)
                                        }
                                        Error::Gemini(msg) => println!("gemini error: {:?}", msg),
                                        Error::NoGeminiCandidatesReceived => {
                                            println!("no gemini candidates found")
                                        }
                                    },
                                }

                                // match self
                                //     .discord_client
                                //     .tldr(
                                //         message_create.channel_id.as_str(),
                                //         message_create.id.as_str(),
                                //         author.as_str(),
                                //         message_create.content.as_str(),
                                //     )
                                //     .await
                                // {
                                //     Ok(_) => {}
                                //     Err(ClientFailure(message)) => {
                                //         println!("failed to call client tldr: {:?}", message)
                                //     }
                                // }
                            } else {
                                println!(
                                    "{} - {}",
                                    message_create.content.len(),
                                    message_create.author.username
                                );
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
                Message::Close(_close) => {}
                v => panic!("received an event with an unexpected message: {:?}\n", v),
            },
            Err(err) => match err {
                tokio_tungstenite::tungstenite::Error::ConnectionClosed => {
                    println!("connection closed");
                }
                e => panic!("{}", e),
            },
        }
    }
}
