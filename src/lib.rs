pub mod error;
pub mod health;

use std::time::Duration;

use error::Error;

pub struct Client {
    base_url: String,
    client: reqwest::Client,
}

impl Client {
    pub fn new(base_url: &str, timeout: Option<Duration>) -> Result<Self, Error> {
        let mut client = reqwest::ClientBuilder::new().user_agent("pocketbase-rs");

        client = if let Some(timeout) = timeout {
            client.timeout(timeout)
        } else {
            client.timeout(Duration::from_millis(5000))
        };

        let client = client.build()?;

        Ok(Client {
            base_url: base_url.to_string(),
            client,
        })
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn health_test() {
        let client = Client::new("http://127.0.0.1:8090", None).unwrap();

        let health = client.health().await.unwrap();

        assert_eq!(health.code, 200);
    }
}
