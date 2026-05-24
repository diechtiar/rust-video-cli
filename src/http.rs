// src/http.rs
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::Client;
use scraper::{Html, Selector};
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
            .danger_accept_invalid_hostnames(true)
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

    #[instrument(skip(self, credentials))]
    pub async fn login(
        &self,
        login_url: &str,
        credentials: &[(&str, &str)],
    ) -> Result<String, crate::AppError> {
        tracing::info!("Starting realistic login flow at: {}", login_url);

        // Step 1: GET the login page
        let login_page = self.get(login_url).await?;

        // === TEMPORARY DEBUG LOG ===
        tracing::info!(
            "=== LOGIN PAGE CONTENT (first 2000 chars) ===\n{}",
            &login_page.chars().take(2000).collect::<String>()
        );
        // =============================

        // Step 2: Extract the CSRF token
        let token = extract_authenticity_token(&login_page).ok_or_else(|| {
            crate::AppError::Config(
                "Could not find authenticity_token on the login page".to_string(),
            )
        })?;

        tracing::debug!("Successfully extracted authenticity_token");

        // Step 3: Add the token to the form data
        let mut form_data: Vec<(&str, &str)> = credentials.to_vec();
        form_data.push(("authenticity_token", token.as_str()));

        // Step 4: POST the login form (now includes the token)
        let response = self.post(login_url, &form_data).await?;

        tracing::info!("Login POST request completed");
        Ok(response)
    }
}

pub fn extract_authenticity_token(html: &str) -> Option<String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("input[name=\"authenticity_token\"]").unwrap();

    document
        .select(&selector)
        .next()
        .and_then(|element| element.value().attr("value"))
        .map(|s| s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_authenticity_token() {
        let html = r#"
            <html>
                <input type="hidden" name="authenticity_token" value="abc123xyz">
            </html>
        "#;

        let token = extract_authenticity_token(html);
        assert_eq!(token, Some("abc123xyz".to_string()));
    }
}
