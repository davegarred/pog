use std::sync::{Arc, Mutex};
use async_trait::async_trait;
use crate::error::Error;

#[async_trait]
pub trait DiscordClient {
    async fn set_message(&self, message_id: &str, request_token: &str, message: &str) -> Result<(),Error>;
}

#[derive(Clone,Default,Debug)]
pub struct TestDiscordClient {
    pub message: Arc<Mutex<Option<String>>>,
}

#[async_trait]
impl DiscordClient for TestDiscordClient {
    async fn set_message(&self, _message_id: &str, _request_token: &str, message: &str) -> Result<(), Error> {
        *self.message.lock().unwrap() = Some(message.to_string());
        Ok(())
    }
}
