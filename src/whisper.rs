use std::time::Duration;
use reqwest::{multipart, Body, Client};
use serde::Deserialize;
use anyhow::Result;
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};
use crate::util::Segment;

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

    pub async fn transcribe(&self, mut segment: Segment) -> Result<Segment> {
        let filename = segment.path.file_name().unwrap().to_str().unwrap().to_string();
        let file = File::open(&segment.path).await?;
        
        let stream = FramedRead::new(file, BytesCodec::new());
        let file_body = Body::wrap_stream(stream);
        let some_file = multipart::Part::stream(file_body).file_name(filename);

        let form = multipart::Form::new()
        .text("model", OPENAI_MODEL)
        .part("file", some_file);

        let client = Client::new();
        let r = client.post(&self.url)
        .bearer_auth(&self.api_key)
        .timeout(Duration::from_secs(600))
        .multipart(form)
        .send()
        .await?;
        
        let text = r.text().await?;

        let res: ApiResponse = match serde_json::from_str(&text) {
            Ok(json) => json,
            Err(e) => {
                println!("Error: {:?}", text);
                return Err(e.into());
            }
        };


        segment.transcript = res.text;
        Ok(segment)
    }
}