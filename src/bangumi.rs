use anyhow::Result;
use regex::Regex;
use reqwest::Client;
use rss::Channel;
use serde::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Bangumi {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "rss")]
    rss: String,

    #[serde(rename = "session")]
    pub session: Option<String>,

    #[serde(rename = "filter")]
    filter: Filter,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Filter {
    #[serde(rename = "expect")]
    expect: Vec<String>,

    #[serde(rename = "exclude")]
    exclude: Vec<String>,
}

#[derive(Debug)]
pub struct BangumiRSSItem {
    pub title: String,
    pub magnet: String,
    pub episode: Option<String>,
}

impl Bangumi {
    pub async fn get_rss_result(&self, client: &Client) -> Result<Channel> {
        let content = client.get(&self.rss).send().await?.bytes().await?;
        let channel = Channel::read_from(&content[..])?;
        Ok(channel)
    }

    pub fn filter_rss(&self, channel: &Channel) -> Result<Vec<BangumiRSSItem>> {
        let mut items = vec![];

        let re_cleanup = Regex::new(r"\d{3,4}p|\d{2}bit").unwrap();
        let re_extract = Regex::new(r"\b(\d{1,2})\b").unwrap();

        for item in channel.items() {
            let title = item.title().unwrap().to_string();
            let link = item.link().unwrap().to_string();
            let guid = link.split("/").last().unwrap().to_string();
            let magnet = format!("magnet:?xt=urn:btih:{}", guid);

            if self
                .filter
                .exclude
                .iter()
                .any(|exclude| title.contains(exclude))
            {
                continue;
            }

            if self
                .filter
                .expect
                .iter()
                .all(|exclude| title.contains(exclude))
            {
                let clean_title = re_cleanup.replace_all(&title, "");

                let episode = re_extract
                    .captures(&clean_title)
                    .map(|c| c.get(0).unwrap().as_str().to_string());

                items.push(BangumiRSSItem {
                    title,
                    magnet,
                    episode,
                });
            }
        }

        Ok(items)
    }
}
