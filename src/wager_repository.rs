use crate::error::Error;
use crate::wager::Wager;
use std::sync::{Arc, Mutex};

#[async_trait::async_trait]
pub trait WagerRepository {
    async fn insert(&self, wager: Wager) -> Result<(), Error>;
    async fn search_by_user(&self, user: &str) -> Result<Vec<Wager>, Error>;
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

    async fn search_by_user(&self, user: &str) -> Result<Vec<Wager>, Error> {
        let mut result = Vec::new();
        for wager in self.wagers.lock().unwrap().iter() {
            if wager.offering == user || wager.accepting == user {
                result.push(wager.clone());
            }
        }
        Ok(result)
    }
}

#[tokio::test]
async fn test_in_mem_wager_repo() {
    let repo = InMemWagerRepository::default();
    repo.insert(Wager {
        offering: "Harx".to_string(),
        accepting: "Woody".to_string(),
        wager: "$100".to_string(),
        outcome: "Rangers take the Phillies, should they meet".to_string(),
    })
    .await
    .unwrap();
    repo.insert(Wager {
        offering: "Shawn".to_string(),
        accepting: "Todd".to_string(),
        wager: "$40".to_string(),
        outcome: "Jax has a losing season".to_string(),
    })
    .await
    .unwrap();
    repo.insert(Wager {
        offering: "Woody".to_string(),
        accepting: "Todd".to_string(),
        wager: "$30".to_string(),
        outcome: "Jets beat the Oilers".to_string(),
    })
    .await
    .unwrap();
    let found = repo.search_by_user("Woody").await.unwrap();
    assert_eq!(2, found.len());
}
