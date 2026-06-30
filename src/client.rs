use anyhow::Result;
use reqwest::{Client, RequestBuilder};
use serde::de::DeserializeOwned;

pub struct RedmineClient {
    base_url: String,
    api_key: String,
    http: Client,
}

impl RedmineClient {
    pub fn new(base_url: impl Into<String>, api_key: impl Into<String>) -> Result<Self> {
        let http = Client::builder().build()?;
        Ok(Self {
            base_url: base_url.into().trim_end_matches('/').to_string(),
            api_key: api_key.into(),
            http,
        })
    }

    fn get(&self, path: &str) -> RequestBuilder {
        self.http
            .get(format!("{}{}", self.base_url, path))
            .header("X-Redmine-API-Key", &self.api_key)
    }

    fn post(&self, path: &str) -> RequestBuilder {
        self.http
            .post(format!("{}{}", self.base_url, path))
            .header("X-Redmine-API-Key", &self.api_key)
    }

    fn put(&self, path: &str) -> RequestBuilder {
        self.http
            .put(format!("{}{}", self.base_url, path))
            .header("X-Redmine-API-Key", &self.api_key)
    }

    pub async fn get_json<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let res = self.get(path).send().await?.error_for_status()?;
        Ok(res.json().await?)
    }

    pub async fn post_json<B: serde::Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let res = self
            .post(path)
            .json(body)
            .send()
            .await?
            .error_for_status()?;
        Ok(res.json().await?)
    }

    pub async fn put_json<B: serde::Serialize>(&self, path: &str, body: &B) -> Result<()> {
        self.put(path).json(body).send().await?.error_for_status()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_client_trims_trailing_slash() {
        let client = RedmineClient::new("https://example.com/", "key").unwrap();
        assert_eq!(client.base_url, "https://example.com");
    }

    #[test]
    fn new_client_trims_multiple_trailing_slashes() {
        let client = RedmineClient::new("https://example.com///", "key").unwrap();
        assert_eq!(client.base_url, "https://example.com");
    }

    #[test]
    fn new_client_keeps_url_without_trailing_slash() {
        let client = RedmineClient::new("https://example.com", "key").unwrap();
        assert_eq!(client.base_url, "https://example.com");
    }

    #[test]
    fn new_client_stores_api_key() {
        let client = RedmineClient::new("https://example.com", "my-secret-key").unwrap();
        assert_eq!(client.api_key, "my-secret-key");
    }
}
