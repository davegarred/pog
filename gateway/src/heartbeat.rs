use crate::payloads::DiscordGatewayResponse;
use futures_channel::mpsc::{UnboundedReceiver, UnboundedSender};
use std::time::Duration;
use tokio::time::sleep;
use tokio_tungstenite::tungstenite::Message;

pub enum WebsocketUpdate {
    HeartbeatInterval(u64),
    SequenceNumber(Option<u64>),
}

pub async fn heartbeat(
    mut internal: UnboundedReceiver<WebsocketUpdate>,
    tx: UnboundedSender<Message>,
) {
    let mut heartbeat_interval = 2000;
    let mut sequence_number: Option<u64> = None;
    loop {
        sleep(Duration::from_millis(heartbeat_interval)).await;
        latest_heartbeat(&mut heartbeat_interval, &mut sequence_number, &mut internal);
        let payload = serde_json::to_vec(&DiscordGatewayResponse::heartbeat(sequence_number))
            .expect("serialize a heartbeat");
        tx.unbounded_send(Message::binary(payload))
            .expect("send a heartbeat signal");
    }
}

fn latest_heartbeat(
    heartbeat_interval: &mut u64,
    sequence_number: &mut Option<u64>,
    internal: &mut UnboundedReceiver<WebsocketUpdate>,
) {
    loop {
        match internal.try_next() {
            Ok(Some(msg)) => match msg {
                WebsocketUpdate::HeartbeatInterval(hi) => {
                    *heartbeat_interval = hi;
                }
                WebsocketUpdate::SequenceNumber(s) => {
                    *sequence_number = s;
                }
            },
            _ => return,
        };
    }
}
