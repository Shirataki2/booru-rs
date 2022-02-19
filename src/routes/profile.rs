pub use crate::prelude::*;

type DebugBody = serde_json::Value;

#[async_trait]
pub trait ProfileRequest {
    async fn profile(&self) -> Result<DebugBody, Error>;
}

#[async_trait]
impl ProfileRequest for crate::client::BooruClient {
    async fn profile(&self) -> Result<DebugBody, Error> {
        let url = "profile.json";
        let res = self.get(url).await?;
        Ok(res)
    }   
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;
    
    #[test(tokio::test)]
    async fn test_profile() {
        let config = crate::test_config();
        let client = crate::client::BooruClient::danbooru(true, Some(config));
        let res = client.profile().await.unwrap();
        println!("{}", res);
    }
}
