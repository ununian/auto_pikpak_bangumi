use std::time::Duration;

use anyhow::Result;
use reqwest::Client;
use serde::*;

use crate::bangumi::*;
use crate::directory::*;
use crate::pikpak::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(rename = "version")]
    pub version: i64,

    #[serde(rename = "directory")]
    pub directory: Directory,

    #[serde(rename = "pikpak")]
    pub pikpak: Pikpak,

    #[serde(rename = "bangumi")]
    pub bangumi: Vec<Bangumi>,

    #[serde(rename = "proxy")]
    proxy: Option<String>,
}

impl Config {
    pub fn get_client(&self) -> Result<Client> {
        let mut builder = Client::builder();

        builder = if let Some(proxy) = &self.proxy {
            println!("使用代理: {}", proxy);
            builder
                .proxy(reqwest::Proxy::http(proxy).unwrap())
                .proxy(reqwest::Proxy::https(proxy).unwrap())
        } else {
            builder
        };

        let client = builder.timeout(Duration::from_secs(30)).build()?;

        Ok(client)
    }
}
