use anyhow::{Ok, Result};
use pikpakcli::pikpak::{new::NewMagnetResp, Client, ClientOptions};
use serde::*;

use crate::bangumi;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pikpak {
    #[serde(rename = "username")]
    username: Option<String>,

    #[serde(rename = "password")]
    password: Option<String>,

    #[serde(rename = "path")]
    path: Option<String>,
}

#[derive(Debug)]
pub struct PikpakClient {
    pub config: Pikpak,
    pub client: Client,
}

impl Pikpak {
    pub async fn get_client(&self) -> Result<PikpakClient> {
        if self.username.is_none() || self.password.is_none() {
            panic!("Username and password are required for Pikpak");
        }

        let mut client = Client::new(ClientOptions {
            username: self
                .username
                .clone()
                .expect("Username and password are required for Pikpak"),
            password: self
                .password
                .clone()
                .expect("Username and password are required for Pikpak"),
            retry_times: 3,
            proxy: None,
        })?;

        client.login().await?;

        return Ok(PikpakClient {
            client,
            config: self.clone(),
        });
    }
}

impl PikpakClient {
    pub async fn make_sure_bangumi_folder(
        &mut self,
        bangumi_name: &str,
        session: &str,
    ) -> Result<()> {
        let parent_path = self.config.path.as_ref().map_or("/", |f| f.as_str());
        let bangumi_path = format!("{}/{}", parent_path, bangumi_name);
        let session_path = format!("{}/{}", bangumi_path, session);

        println!("parent_path: {}", parent_path);
        println!("bangumi_path: {}", bangumi_path);

        let bangumi_folder = self.client.get_path_id(&bangumi_path).await;

        if bangumi_folder.is_err() {
            self.client.new_folder(&parent_path, &bangumi_name).await?;
            self.client.new_folder(&bangumi_path, &session).await?;
            return Ok(());
        }
        println!("session_path: {}", session_path);

        let session_folder = self.client.get_path_id(&session_path).await;
        if session_folder.is_err() {
            self.client.new_folder(&bangumi_path, &session).await?;
        }

        Ok(())
    }

    pub async fn add_magnet(
        &mut self,
        magnet: &str,
        bangumi_name: &str,
        session: &str,
    ) -> Result<NewMagnetResp> {
        let parent_path = self.config.path.as_ref().map_or("/", |f| f.as_str());
        let bangumi_path = format!("{}/{}", parent_path, bangumi_name);
        let session_path = format!("{}/{}", bangumi_path, session);

        Ok(self.client.new_magnet(&session_path, magnet).await?)
    }

    pub fn get_remote_path(&self, bangumi_name: &str, session: &str) -> String {
        let parent_path = self.config.path.as_ref().map_or("/", |f| f.as_str());
        let bangumi_path = format!("{}/{}", parent_path, bangumi_name);
        let session_path = format!("{}/{}", bangumi_path, session);

        return session_path;
    }

    pub async fn download_file(client: Client, file: String, output: String) -> Result<()> {
        client.download(vec![file], output, 1).await?;
        Ok(())
    }
}
