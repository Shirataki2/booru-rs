#[macro_use] extern crate serde;
#[macro_use] extern crate async_trait;

pub mod config;
pub mod client;
pub mod error;
pub mod prelude;
pub mod routes;

#[allow(dead_code)]
pub(crate) fn test_config() -> crate::config::Config {
    crate::config::Config {
        account: crate::config::AccountConfig {
            username: include_str!("../account/test_username").to_string(),
            api_key: include_str!("../account/test_apikey").to_string(),
        },
    }
}
