use anyhow::Result;
use std::{fs::create_dir_all, path::Path};
use tokio::fs::read_dir;

use serde::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Directory {
    #[serde(rename = "download")]
    pub download: String,
}

#[derive(Debug)]
pub struct BangumiFile {
    pub name: String,
    pub path: String,
}

impl Directory {
    pub async fn scan_bangumi(
        &self,
        bangumi_name: &str,
        session: &str,
    ) -> Result<Vec<BangumiFile>> {
        let bangumi = format!("{}/{}", &self.download, bangumi_name);
        let session = format!("{}/{}", bangumi, session);

        let bangumi_path = Path::new(&bangumi);
        let session_path = Path::new(&session);

        if !bangumi_path.exists() {
            create_dir_all(bangumi_path).unwrap();
        }

        if !session_path.exists() {
            create_dir_all(session_path).unwrap();
        }

        let mut entries = read_dir(session_path).await?;
        let mut files = vec![];

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.is_file() {
                files.push(BangumiFile {
                    name: path.file_name().unwrap().to_str().unwrap().to_string(),
                    path: path.to_str().unwrap().to_string(),
                });
                println!("文件: {:?}", path);
            }
        }

        Ok(files)
    }

    pub fn make_sure(&self) -> Result<()> {
        Ok(create_dir_all(&self.download)?)
    }

    pub fn make_sure_bangumi(&self, bangumi_name: &str, session: &str) {
        let bangumi = format!("{}/{}", &self.download, bangumi_name);
        let _ = create_dir_all(&bangumi);

        let session = format!("{}/{}", bangumi, session);
        let _ = create_dir_all(session);
    }

    pub fn get_download_path(&self, bangumi_name: &str, session: &str) -> String {
        let bangumi = format!("{}/{}", &self.download, bangumi_name);
        let session = format!("{}/{}", bangumi, session);
        return session;
    }
}
