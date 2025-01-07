use discord_api::interaction_request::{GuildMember, User};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetGateway {
    pub url: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InboundPayload {
    pub t: Option<String>,
    pub s: Option<u64>,
    pub op: u8,
    pub d: serde_json::Value,
}

impl InboundPayload {
    pub fn event(self) -> InboundEvent {
        match self.op {
            10 => {
                let payload = serde_json::from_value(self.d).expect("deserialize a hello event");
                InboundEvent::Hello(payload)
            }
            11 => InboundEvent::Ack,
            7 => InboundEvent::Reconnect,
            0 => match self.t {
                Some(code) => match code.as_str() {
                    "READY" => {
                        let payload =
                            serde_json::from_value(self.d).expect("deserialize a ready event");
                        InboundEvent::Ready(payload)
                    }
                    "RESUMED" => InboundEvent::Resumed,
                    "TYPING_START" => {
                        let payload = serde_json::from_value(self.d)
                            .expect("deserialize a typing start event");
                        InboundEvent::TypingStart(payload)
                    }
                    "GUILD_CREATE" => {
                        let payload = serde_json::from_value(self.d)
                            .expect("deserialize a guild create event");
                        InboundEvent::GuildCreate(payload)
                    }
                    "MESSAGE_CREATE" => {
                        let payload = serde_json::from_value(self.d)
                            .expect("deserialize a message create event");
                        InboundEvent::MessageCreate(payload)
                    }
                    "MESSAGE_DELETE" => {
                        let payload = serde_json::from_value(self.d)
                            .expect("deserialize a message delete event");
                        InboundEvent::MessageDelete(payload)
                    }
                    "MESSAGE_REACTION_ADD" => {
                        let payload = serde_json::from_value(self.d)
                            .expect("deserialize a message reaction add event");
                        InboundEvent::MessageReactionAdd(payload)
                    }
                    "MESSAGE_REACTION_REMOVE" => {
                        let payload = serde_json::from_value(self.d)
                            .expect("deserialize a message reaction remove event");
                        InboundEvent::MessageReactionRemove(payload)
                    }
                    "MESSAGE_UPDATE" => {
                        let payload = serde_json::from_value(self.d)
                            .expect("deserialize a message update event");
                        InboundEvent::MessageUpdate(payload)
                    }
                    "GUILD_MEMBER_ADD" => {
                        let payload = serde_json::from_value(self.d)
                            .expect("deserialize a message update event");
                        InboundEvent::MemberAdd(payload)
                    }
                    name => {
                        println!("unknown event name {}", name);
                        InboundEvent::Unknown
                    }
                },
                _ => InboundEvent::Unknown,
            },
            9 => panic!("invalid session, op: 9"),
            op => panic!("Unknown inbound message op: {}", op),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum InboundEvent {
    Hello(HelloEvent),
    Ack,
    Ready(ReadyEvent),
    Resumed,
    Reconnect,
    GuildCreate(GuildCreateEvent),
    MessageCreate(MessageCreateEvent),
    MessageDelete(MessageDeleteEvent),
    MessageReactionAdd(MessageReactionAddEvent),
    MessageReactionRemove(MessageReactionRemoveEvent),
    MessageUpdate(MessageUpdateEvent),
    MemberAdd(MemberAddEvent),
    TypingStart(TypingStartEvent),
    Unknown,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HelloEvent {
    pub heartbeat_interval: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReadyEvent {
    // TODO: missing most of these fields
    pub v: u8,
    pub session_id: String,
    pub resume_gateway_url: String,
}

// https://discord.com/developers/docs/topics/gateway-events#guild-create
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GuildCreateEvent {
    // TODO: missing most of these fields
    pub channels: Vec<Channel>,
}

// https://discord.com/developers/docs/resources/channel#channel-object
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Channel {
    // TODO: missing most of these fields
    pub id: String,
    // https://discord.com/developers/docs/resources/channel#channel-object-channel-types
    #[serde(rename = "type")]
    pub channel_type: u8,
    pub name: String,
    pub last_message_id: Option<String>,
}

// https://discord.com/developers/docs/topics/gateway-events#message-create
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageCreateEvent {
    pub id: String,
    pub channel_id: String,
    pub content: String,
    pub author: User,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageDeleteEvent {
    pub id: String,
    pub channel_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageUpdateEvent {
    pub content: Option<String>,
    pub author: Option<User>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MemberAddEvent {
    pub user: Option<User>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChannelMessage {
    pub id: String,
    pub content: String,
    pub author: User,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TypingStartEvent {
    pub channel_id: String,
    pub user_id: String,
    pub timestamp: u64,
    pub member: Option<GuildMember>,
}

// https://discord.com/developers/docs/topics/gateway-events#message-reaction-add
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageReactionAddEvent {
    pub user_id: String,
    pub channel_id: String,
    pub message_id: String,
    pub member: Option<GuildMember>,
    pub emoji: Emoji,
}

// https://discord.com/developers/docs/topics/gateway-events#message-reaction-remove
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageReactionRemoveEvent {
    pub user_id: String,
    pub channel_id: String,
    pub message_id: String,
    pub emoji: Emoji,
}

// https://discord.com/developers/docs/resources/emoji#emoji-object
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Emoji {
    pub name: Option<String>,
    pub id: Option<String>,
}

#[cfg(test)]
mod test {
    use crate::inbound_payloads::{InboundEvent, InboundPayload};
    use std::fs;

    #[test]
    fn guild_create() {
        let contents = fs::read_to_string("dto_payloads/guild_create.json").unwrap();
        let payload: InboundPayload = serde_json::from_str(&contents).unwrap();
        match payload.event() {
            InboundEvent::GuildCreate(guild_create) => {
                assert_eq!(5, guild_create.channels.len())
            }
            _ => panic!("fail"),
        }
    }

    #[test]
    fn hello() {
        let contents = fs::read_to_string("dto_payloads/hello_event.json").unwrap();
        let payload: InboundPayload = serde_json::from_str(&contents).unwrap();
        match payload.event() {
            InboundEvent::Hello(hello) => {
                assert_eq!(hello.heartbeat_interval, 41250);
            }
            _ => panic!("fail"),
        }
    }

    #[test]
    fn message_create() {
        let contents = fs::read_to_string("dto_payloads/message_create.json").unwrap();
        let payload: InboundPayload = serde_json::from_str(&contents).unwrap();
        match payload.event() {
            InboundEvent::MessageCreate(message_create) => {
                assert_eq!(
                    "Yup, be there by tip off for the JMU/Duke game",
                    message_create.content
                )
            }
            _ => panic!("fail"),
        }
    }

    #[test]
    fn message_delete() {
        let contents = fs::read_to_string("dto_payloads/message_delete.json").unwrap();
        let payload: InboundPayload = serde_json::from_str(&contents).unwrap();
        match payload.event() {
            InboundEvent::MessageDelete(message_delete) => {
                assert_eq!("1221636339213139968", message_delete.id);
            }
            _ => panic!("fail"),
        }
    }

    #[test]
    fn message_reaction_add() {
        let contents = fs::read_to_string("dto_payloads/message_reaction_add.json").unwrap();
        let payload: InboundPayload = serde_json::from_str(&contents).unwrap();
        match payload.event() {
            InboundEvent::MessageReactionAdd(message_reaction_add) => {
                assert_eq!("348912925609820162", message_reaction_add.user_id);
                assert_eq!(
                    "downwitda",
                    message_reaction_add.member.unwrap().user.unwrap().username
                )
            }
            _ => panic!("fail"),
        }
    }

    #[test]
    fn message_update() {
        let contents = fs::read_to_string("dto_payloads/message_update.json").unwrap();
        let payload: InboundPayload = serde_json::from_str(&contents).unwrap();
        match payload.event() {
            InboundEvent::MessageUpdate(message_update) => {
                assert_eq!("ingae8641", message_update.author.unwrap().username);
                assert_eq!("Moving the previous post to the right channel. I’ve got some stuff to give away!", message_update.content.unwrap());
            }
            _ => panic!("fail"),
        }
    }

    #[test]
    fn guild_member_add() {
        let contents = fs::read_to_string("dto_payloads/guild_member_add.json").unwrap();
        let payload: InboundPayload = serde_json::from_str(&contents).unwrap();
        match payload.event() {
            InboundEvent::MemberAdd(member_add) => {
                let user = member_add.user.unwrap();
                assert_eq!("testuser2_35118", &user.username);
                assert_eq!("test-user-2", &user.global_name.unwrap());
            }
            _ => panic!("fail"),
        }
    }

    #[test]
    fn ready() {
        let contents = fs::read_to_string("dto_payloads/ready_event.json").unwrap();
        let payload: InboundPayload = serde_json::from_str(&contents).unwrap();
        match payload.event() {
            InboundEvent::Ready(ready) => {
                assert_eq!(
                    ready.resume_gateway_url.as_str(),
                    "wss://gateway-us-east1-c.discord.gg"
                );
                assert_eq!(
                    ready.session_id.as_str(),
                    "2e949f84eb383e88ce5fcf8d2b21bebe"
                );
            }
            _ => panic!("fail"),
        }
    }

    #[test]
    fn typing_start() {
        let contents = fs::read_to_string("dto_payloads/typing_start.json").unwrap();
        let payload: InboundPayload = serde_json::from_str(&contents).unwrap();
        match payload.event() {
            InboundEvent::TypingStart(typing_start) => {
                assert_eq!("695398918694895710", typing_start.user_id);
                assert_eq!(1711304246, typing_start.timestamp);
            }
            _ => panic!("fail"),
        }
    }
}
