pub mod delete;
pub mod download;
pub mod files;
pub mod rename;
pub mod sys;
pub mod upload;

const DEFAULT_API_URL: &str = "https://files.martonaron.dev/api";

pub fn api_url() -> String {
    std::env::var("API_URL").unwrap_or_else(|_| DEFAULT_API_URL.to_string())
}
