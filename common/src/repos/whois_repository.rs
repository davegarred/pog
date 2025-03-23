use crate::error::Error;
use std::future::Future;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, PartialEq)]
pub struct WhoisPerson {
    pub discord_id: u64,
    pub human_name: String,
    pub hash_name: String,
}

pub trait WhoisRepository: Send + Sync {
    fn get_by_discord_id(
        &self,
        discord_id: u64,
    ) -> impl Future<Output = Result<Option<WhoisPerson>, Error>> + Send;
    fn add(
        &self,
        discord_id: u64,
        human_name: &str,
        hash_name: &str,
    ) -> impl Future<Output = Result<(), Error>> + Send;
    fn update(
        &self,
        discord_id: u64,
        human_name: &str,
        hash_name: &str,
    ) -> impl Future<Output = Result<(), Error>> + Send;
    fn set_user(
        &self,
        discord_id: u64,
        human_name: &str,
        hash_name: &str,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}

#[derive(Debug, Clone)]
pub struct InMemWhoisRepository {
    person: Arc<Mutex<Result<Option<WhoisPerson>, Error>>>,
}

impl Default for InMemWhoisRepository {
    fn default() -> Self {
        Self {
            person: Arc::new(Mutex::new(Ok(None))),
        }
    }
}

impl WhoisRepository for InMemWhoisRepository {
    async fn get_by_discord_id(&self, _: u64) -> Result<Option<WhoisPerson>, Error> {
        self.person.lock().unwrap().clone()
    }

    async fn add(&self, discord_id: u64, human_name: &str, hash_name: &str) -> Result<(), Error> {
        self.update(discord_id, human_name, hash_name).await
    }

    async fn update(
        &self,
        discord_id: u64,
        human_name: &str,
        hash_name: &str,
    ) -> Result<(), Error> {
        *self.person.lock().unwrap() = Ok(Some(WhoisPerson {
            discord_id,
            human_name: human_name.to_string(),
            hash_name: hash_name.to_string(),
        }));
        Ok(())
    }

    async fn set_user(
        &self,
        discord_id: u64,
        human_name: &str,
        hash_name: &str,
    ) -> Result<(), Error> {
        self.update(discord_id, human_name, hash_name).await
    }
}
