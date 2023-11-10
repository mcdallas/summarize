use reqwest::Client;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use crate::util::Summary;

const OPENAI_API_URL: &str = "https://api.openai.com/v1/chat/completions";
const OPENAI_MODEL: &str = "gpt-4-1106-preview";


#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_camel_case_types)]
pub enum Role{
    user,
    system,
    assistant,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message{
    role: Role,
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Prompt {
    model: String,
    messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Choice {
    index: u32,
    message: Message,
}

#[derive(Serialize, Deserialize, Debug)]
struct ApiResponse{
    choices: Vec<Choice>
}

pub struct GPTClient {
    api_key: String,
    url: String,
}

impl GPTClient {
    pub fn new(api_key: String) -> Self {
        GPTClient {
            api_key,
            url: String::from(OPENAI_API_URL),
        }
    }

    pub async fn prompt(&self, prompt: &str, behavior: &str) -> Result<String> {
        let behavior = String::from(behavior);

        let messages = vec![
            Message{
                role: Role::system,
                content: behavior,
            },
            Message{
                role: Role::user,
                content: String::from(prompt),
            }
        ];

        let prompt = Prompt {
            model: String::from(OPENAI_MODEL),
            messages,
        };


        let client = Client::new();
        let res = client.post(&self.url)
            .bearer_auth(&self.api_key)
            .json(&prompt)
            .send()
            .await?
            .json::<ApiResponse>().await?;
        

        let answer = res.choices[0].message.content.clone();

        Ok(answer)
    }

    pub async fn summarize(&self, mut summaries: Vec<Summary>) -> Result<String> {
        summaries.reverse();
        let n_summaries = summaries.len();
        let first = summaries.pop().unwrap();
        let mut current_summary = self.prompt(&first.text, "Summarize the following text:").await?;
        
        for (i, summary) in summaries.iter().enumerate() {
            let behavior = format!("Your task is to summarize in several paragraphs a large text that is presented in chunks. This will be chunk {} of {}. The summary of the previous chunks is the following: ```{}``` You must incorporate the previous summary into the summary of the whole text. This is the current chunk: ", i+1, n_summaries , current_summary);
            current_summary = self.prompt(&summary.text, &behavior).await?;
        }

        Ok(current_summary)
    }
}
