use crate::discord_id::DiscordId;
use crate::error::Error;
use crate::wager::{Wager, WagerStatus};
use std::sync::{Arc, Mutex};

#[async_trait::async_trait]
pub trait WagerRepository: std::fmt::Debug {
    async fn insert(&self, wager: Wager) -> Result<(), Error>;
    async fn get(&self, wager_id: i32) -> Option<Wager>;
    async fn search_by_user_id(&self, user_id: &DiscordId) -> Result<Vec<Wager>, Error>;
    async fn search_by_user(&self, user: &str) -> Result<Vec<Wager>, Error>;
    async fn update_status(&self, wager_id: i32, wager: &Wager) -> Result<(), Error>;
}

#[derive(Debug, Default, Clone)]
pub struct InMemWagerRepository {
    wagers: Arc<Mutex<Vec<Wager>>>,
}

#[async_trait::async_trait]
impl WagerRepository for InMemWagerRepository {
    async fn insert(&self, wager: Wager) -> Result<(), Error> {
        self.wagers.lock().unwrap().push(wager);
        Ok(())
    }

    async fn get(&self, wager_id: i32) -> Option<Wager> {
        for wager in self.wagers.lock().unwrap().iter() {
            if wager.wager_id == wager_id as u32 {
                return Some(wager.clone());
            }
        }
        None
    }

    async fn search_by_user_id(&self, user_id: &DiscordId) -> Result<Vec<Wager>, Error> {
        let mut result = Vec::new();
        for wager in self.wagers.lock().unwrap().iter() {
            match (
                &wager.resolved_offering_user,
                &wager.resolved_accepting_user,
            ) {
                (Some(user_a), Some(user_b)) => {
                    if user_a == user_id || user_b == user_id && wager.status == WagerStatus::Open {
                        result.push(wager.clone());
                    }
                }
                (Some(user_a), None) => {
                    if user_a == user_id && wager.status == WagerStatus::Open {
                        result.push(wager.clone());
                    }
                }
                (None, Some(user_b)) => {
                    if user_b == user_id && wager.status == WagerStatus::Open {
                        result.push(wager.clone());
                    }
                }
                (_, _) => {}
            };
        }
        Ok(result)
    }

    async fn search_by_user(&self, user: &str) -> Result<Vec<Wager>, Error> {
        let mut result = Vec::new();
        for wager in self.wagers.lock().unwrap().iter() {
            if (wager.offering == user || wager.accepting == user)
                && wager.status == WagerStatus::Open
            {
                result.push(wager.clone());
            }
        }
        Ok(result)
    }

    async fn update_status(&self, wager_id: i32, updated_wager: &Wager) -> Result<(), Error> {
        let status = updated_wager.status;
        for wager in self.wagers.lock().unwrap().iter_mut() {
            if wager.wager_id == wager_id as u32 {
                wager.status = status;
            }
        }
        Ok(())
    }
}

#[tokio::test]
async fn test_in_mem_wager_repo() {
    let repo = InMemWagerRepository::default();
    repo.insert(Wager {
        wager_id: 0,
        time: chrono::Utc::now().to_rfc3339(),
        offering: "Harx".to_string(),
        resolved_offering_user: None,
        accepting: "Woody".to_string(),
        resolved_accepting_user: None,
        wager: "$100".to_string(),
        outcome: "Rangers take the Phillies, should they meet".to_string(),
        status: WagerStatus::Open,
        expected_settle_date: None,
    })
    .await
    .unwrap();
    repo.insert(Wager {
        wager_id: 0,
        time: chrono::Utc::now().to_rfc3339(),
        offering: "Shawn".to_string(),
        resolved_offering_user: None,
        accepting: "Todd".to_string(),
        resolved_accepting_user: None,
        wager: "$40".to_string(),
        outcome: "Jax has a losing season".to_string(),
        status: WagerStatus::Open,
        expected_settle_date: None,
    })
    .await
    .unwrap();
    repo.insert(Wager {
        wager_id: 0,
        time: chrono::Utc::now().to_rfc3339(),
        offering: "Woody".to_string(),
        resolved_offering_user: None,
        accepting: "Todd".to_string(),
        resolved_accepting_user: None,
        wager: "$30".to_string(),
        outcome: "Jets beat the Oilers".to_string(),
        status: WagerStatus::Open,
        expected_settle_date: None,
    })
    .await
    .unwrap();
    repo.insert(Wager {
        wager_id: 0,
        time: chrono::Utc::now().to_rfc3339(),
        offering: "Woody".to_string(),
        resolved_offering_user: None,
        accepting: "Todd".to_string(),
        resolved_accepting_user: None,
        wager: "$30".to_string(),
        outcome: "Something that already happened".to_string(),
        status: WagerStatus::Paid,
        expected_settle_date: None,
    })
    .await
    .unwrap();
    let found = repo.search_by_user("Woody").await.unwrap();
    assert_eq!(2, found.len());
}
