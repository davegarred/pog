use crate::error::Error;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct AdminSettings {
    pub welcome_channel: String,
    pub ff_year: u16,
    pub ff_week: u8,
}

impl AdminSettings {
    pub fn new(welcome_channel: String, ff_year: u16, ff_week: u8) -> Self {
        Self {
            welcome_channel,
            ff_year,
            ff_week,
        }
    }
}

pub trait AdminRepository: Send + Sync {
    fn get(&self) -> impl std::future::Future<Output = Result<AdminSettings, Error>> + Send;
    fn update(
        &self,
        settings: AdminSettings,
    ) -> impl std::future::Future<Output = Result<(), Error>> + Send;
}

#[derive(Debug, Clone)]
pub struct InMemAdminRepository {
    settings: Arc<Mutex<Result<AdminSettings, Error>>>,
}

impl Default for InMemAdminRepository {
    fn default() -> Self {
        Self {
            settings: Arc::new(Mutex::new(Ok(AdminSettings::default()))),
        }
    }
}

impl AdminRepository for InMemAdminRepository {
    async fn get(&self) -> Result<AdminSettings, Error> {
        self.settings.lock().unwrap().clone()
    }

    async fn update(&self, settings: AdminSettings) -> Result<(), Error> {
        *self.settings.lock().unwrap() = Ok(settings);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization() {
        let mut settings = AdminSettings::default();
        settings.ff_year = 2024;
        settings.ff_week = 18;
        settings.welcome_channel = "1234567890".to_string();
        let ser = serde_json::to_string(&settings).unwrap();
        assert_eq!(
            ser,
            r##"{"welcome_channel":"1234567890","ff_year":2024,"ff_week":18}"##
        );
        let des: AdminSettings = serde_json::from_str(&ser).unwrap();
        assert_eq!(
            des,
            AdminSettings {
                ff_week: 18,
                ff_year: 2024,
                welcome_channel: "1234567890".to_string()
            }
        )
    }
}
