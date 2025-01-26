use crate::error::Error;
use std::sync::{Arc, Mutex};

pub trait DiscordClient: std::fmt::Debug {
    async fn delete_message(&self, message_id: &str, request_token: &str) -> Result<(), Error>;
    async fn set_message(
        &self,
        message_id: &str,
        request_token: &str,
        message: &str,
    ) -> Result<(), Error>;
}

#[derive(Clone, Default, Debug)]
pub struct TestDiscordClient {
    pub message: Arc<Mutex<Option<String>>>,
}

impl DiscordClient for TestDiscordClient {
    async fn delete_message(&self, _message_id: &str, _request_token: &str) -> Result<(), Error> {
        *self.message.lock().unwrap() = None;
        Ok(())
    }

    async fn set_message(
        &self,
        _message_id: &str,
        _request_token: &str,
        message: &str,
    ) -> Result<(), Error> {
        *self.message.lock().unwrap() = Some(message.to_string());
        Ok(())
    }
}
