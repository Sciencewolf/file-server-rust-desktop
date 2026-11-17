use serde::Deserialize;
use super::api_url;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct FilesResponse {
    files: Vec<FileEntry>,
}

#[derive(Debug, Deserialize)]
pub struct FileEntry {
    pub name: String,
    pub path: String,
    pub size: u64,
    #[serde(rename = "type")]
    pub kind: String,
}

pub async fn get_all_files() -> Result<Vec<FileEntry>, Box<dyn Error>> {
    let url = api_url() + "/files";
    let body = reqwest::Client::new().get(&url).send().await?
        .error_for_status()?
        .json::<FilesResponse>()
        .await?;
    Ok(body.files)
}