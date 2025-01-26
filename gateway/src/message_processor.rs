use crate::error::Error;
use crate::heartbeat::WebsocketUpdate;
use crate::inbound_payloads::{InboundEvent, InboundPayload};
use crate::payloads::DiscordGatewayResponse;
use crate::tldr;
use crate::tldr::create_message;
use crate::TLDR_MESSAGE_LENGTH;
use futures_channel::mpsc::UnboundedSender;
use pog_common::repos::AdminSettings;
use pog_common::{Authorization, CreateMessage, TlDrMessage};
use std::sync::{Arc, Mutex};
use tokio_tungstenite::tungstenite::Message;

pub struct MessageProcessor {
    resume_gateway: String,
    discord_token: String,
    session_id: Option<String>,
    authorization: Authorization,
    gemini_token: String,
    settings: Arc<Mutex<AdminSettings>>,
    sender: UnboundedSender<Message>,
    internal_tx: UnboundedSender<WebsocketUpdate>,
}

impl MessageProcessor {
    pub fn new(
        resume_gateway: String,
        discord_token: String,
        authorization: Authorization,
        gemini_token: String,
        settings: Arc<Mutex<AdminSettings>>,
        sender: UnboundedSender<Message>,
        internal_tx: UnboundedSender<WebsocketUpdate>,
    ) -> Self {
        Self {
            resume_gateway,
            discord_token,
            session_id: None,
            authorization,
            gemini_token,
            settings,
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
                            } else {
                                println!(
                                    "{} - {}",
                                    message_create.content.len(),
                                    message_create.author.username
                                );
                            }
                        }
                        InboundEvent::MemberAdd(member_add) => {
                            let user = member_add
                                .user
                                .expect("member add message did not come with a user");
                            let channel_id = self
                                .settings
                                .lock()
                                .expect("could not unlock admin settings")
                                .welcome_channel
                                .clone();
                            if &channel_id == "" {
                                return;
                            }
                            let name = match user.global_name {
                                None => user.username,
                                Some(name) => name,
                            };
                            let message = format!(
                                "Welcome to the Greenwood Discord {}!\nPlease introduce yourself.",
                                name
                            );
                            println!("channel ({}), send message: {}", channel_id, message);
                            if let Err(err) = create_message(CreateMessage {
                                authorization: self.authorization.clone(),
                                channel_id,
                                message,
                                message_reference: None,
                            })
                            .await
                            {
                                println!("error sending message: {:?}", err);
                            };
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
