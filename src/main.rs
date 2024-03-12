use anyhow::Result;
use std::{
    fs::{read_to_string, File},
    io::Read,
    time::Duration,
};
use tokio::time::sleep;

pub mod bangumi;
pub mod config;
pub mod directory;
pub mod pikpak;

use crate::{config::Config, pikpak::PikpakClient};

#[tokio::main]
async fn main() -> Result<()> {
    let config_yml = read_to_string("./config.local.yml").unwrap();

    let config: Config = serde_yaml::from_str(config_yml.as_str()).unwrap();
    let client = config.get_client().unwrap();
    let item = &config.bangumi[0];

    let rss = item.get_rss_result(&client).await?;
    let rss = item.filter_rss(&rss)?;
    println!("{:#?}", rss);

    let bangumi_name = &item.name;
    let session = match &item.session {
        Some(session) => session.as_str(),
        None => "Session_1",
    };

    let mut pikpak = config.pikpak.get_client().await?;
    pikpak
        .make_sure_bangumi_folder(bangumi_name, session)
        .await?;

    for item in rss.iter().take(1).collect::<Vec<_>>() {
        let task = pikpak
            .add_magnet(&item.magnet, bangumi_name, session)
            .await?;
        sleep(Duration::from_secs(5)).await;

        let client = config.pikpak.get_client().await?;

        println!("{:#?}", task);
        let path = format!(
            "{}/{}",
            client.get_remote_path(bangumi_name, &session),
            task.task.file_name
        );

        let local_path = config.directory.get_download_path(bangumi_name, &session);
        println!("{:#?},, {:#?}", path, local_path);

        PikpakClient::download_file(client.client, path, local_path).await?;
    }

    config.directory.make_sure_bangumi(&bangumi_name, &session);

    let exist_files = config.directory.scan_bangumi(bangumi_name, session).await?;

    println!("{:#?}", exist_files);
    Ok(())
}
