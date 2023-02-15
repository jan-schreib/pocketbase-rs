use crate::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Health {
    pub code: i64,
    pub message: String,
}

#[derive(Debug, Error)]
pub enum HealthError {
    #[error("Reqwest error")]
    HttpError(#[from] reqwest::Error),
}

impl Client {
    pub async fn health(&self) -> Result<Health, HealthError> {
        Ok(self
            .client
            .get(self.base_url().to_string() + "/api/health")
            .send()
            .await?
            .json::<Health>()
            .await?)
    }
}
