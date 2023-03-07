use std::path::PathBuf;
use std::time::Duration;
use reqwest::blocking::Client;
use serde::Deserialize;
use anyhow::Result;

const OPENAI_API_URL: &str = "https://api.openai.com/v1/audio/transcriptions";
const OPENAI_MODEL: &str = "whisper-1";

#[derive(Deserialize, Debug)]
struct ApiResponse {
    text: String,
}

pub struct WhisperClient {
    api_key: String,
    url: String,
}

impl WhisperClient {
    pub fn new(api_key: String) -> Self {
        WhisperClient {
            api_key,
            url: String::from(OPENAI_API_URL),
        }
    }

    pub fn transcribe(&self, path: PathBuf) -> Result<String> {

        let form = reqwest::blocking::multipart::Form::new()
            .text("model", OPENAI_MODEL)
            .file("file", path)?;

        let client = Client::new();
        let res = client.post(&self.url)
        .bearer_auth(&self.api_key)
        .timeout(Duration::from_secs(600))
        .multipart(form)
        .send()?
        .json::<ApiResponse>()?;

        Ok(res.text)
    }
}