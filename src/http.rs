// src/http.rs
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::Client;
use std::time::Duration;
use tracing::instrument;

pub struct HttpClient {
    client: Client,
    default_headers: HeaderMap,
}

impl HttpClient {
    pub fn new() -> Result<Self, crate::AppError> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .cookie_store(true)
            .proxy(reqwest::Proxy::all("http://127.0.0.1:8080")?)
            .danger_accept_invalid_certs(true)
            .build()?;

        Ok(Self {
            client,
            default_headers: HeaderMap::new(),
        })
    }

    /// Add a default header that will be sent with every request
    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        let name = HeaderName::from_bytes(key.as_bytes()).expect("Invalid header name");
        let value = HeaderValue::from_str(value).expect("Invalid header value");
        self.default_headers.insert(name, value);
        self
    }

    #[instrument(skip(self))]
    pub async fn get(&self, url: &str) -> Result<String, crate::AppError> {
        tracing::debug!("GET {}", url);

        let response = self
            .client
            .get(url)
            .headers(self.default_headers.clone())
            .send()
            .await?;

        let body = response.text().await?;
        Ok(body)
    }

    #[instrument(skip(self, form_data))]
    pub async fn post(
        &self,
        url: &str,
        form_data: &[(&str, &str)],
    ) -> Result<String, crate::AppError> {
        tracing::debug!("POST {}", url);

        let response = self
            .client
            .post(url)
            .headers(self.default_headers.clone())
            .form(form_data)
            .send()
            .await?;

        let body = response.text().await?;
        Ok(body)
    }
}
