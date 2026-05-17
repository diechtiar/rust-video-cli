use reqwest::Client;
use std::time::Duration;
use tracing::instrument;

pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .cookie_store(true)
            .build()
            .expect("Failed to create HTTP client");

        Self { client }
    }

    #[instrument(skip(self))]
    pub async fn get(&self, url: &str) -> Result<String, crate::AppError> {
        tracing::debug!("GET request to: {}", url);

        let response = self.client.get(url).send().await?;
        let body = response.text().await?;

        Ok(body)
    }
}
