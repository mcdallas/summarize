use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use anyhow::Result;

const OPENAI_API_URL: &str = "https://api.openai.com/v1/chat/completions";
const OPENAI_MODEL: &str = "gpt-3.5-turbo";


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

    pub fn prompt(&self, prompt: String) -> Result<String> {
        let behavior = String::from("Summarize the following text:");

        let messages = vec![
            Message{
                role: Role::system,
                content: behavior,
            },
            Message{
                role: Role::user,
                content: prompt
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
            .send()?
            .json::<ApiResponse>()?;

        let answer = res.choices[0].message.content.clone();

        Ok(answer)
    }
}
