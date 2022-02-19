use serde::de::DeserializeOwned;

use crate::{config::Config, prelude::*};

#[derive(Debug)]
pub struct BooruClient {
    ssl: bool,
    host: String,
    config: Option<Config>,
    client: reqwest::Client,
}

impl BooruClient {
    pub fn endpoint(&self) -> String {
        if self.ssl {
            format!("https://{}", self.host)
        } else {
            format!("http://{}", self.host)
        }
    }

    pub fn danbooru(ssl: bool, config: Option<Config>) -> Self {
        BooruClient {
            ssl,
            host: "danbooru.donmai.us".to_string(),
            config,
            client: reqwest::Client::new(),
        }
    }

    pub fn create_without_auth(host: &str, ssl: bool) -> Self {
        BooruClient {
            ssl,
            host: host.to_string(),
            config: None,
            client: reqwest::Client::new(),
        }
    }

    pub fn create_with_auth(host: &str, ssl: bool, config: Config) -> Self {
        BooruClient {
            ssl,
            host: host.to_string(),
            config: Some(config),
            client: reqwest::Client::new(),
        }
    }

    pub async fn get<Res>(&self, url: &str) -> Result<Res, Error>
    where
        Res: DeserializeOwned,
    {
        let url = format!("{}/{}", self.endpoint(), url);
        let req = if let Some(config) = &self.config {
            self.client.get(url.as_str())
                .query(&[
                    ("login", config.account.username.as_str()),
                    ("api_key", config.account.api_key.as_str()),
                ])
        } else {
            self.client.get(url.as_str())
        };
        let res = req.send().await.with_context(|| "Failed to send request")?;
        let res = res.json::<Res>().await.with_context(|| "Failed to parse response")?;
        Ok(res)
    }

    pub async fn post<B, Res>(&self, url: &str, body: Option<B>) -> Result<Res, Error>
    where
        B: serde::Serialize,
        Res: DeserializeOwned,
    {
        let url = format!("{}/{}", self.endpoint(), url);
        let req = if let Some(config) = &self.config {
            self.client.post(url.as_str())
                .query(&[
                    ("login", config.account.username.as_str()),
                    ("api_key", config.account.api_key.as_str()),
                ])
        } else {
            self.client.post(url.as_str())
        };
        let req = if let Some(body) = body {
            req.json(&body)
        } else {
            req
        };
        let res = req.send().await.with_context(|| "Failed to send request")?;
        let res = res.json::<Res>().await.with_context(|| "Failed to parse response")?;
        Ok(res)
    }
}
