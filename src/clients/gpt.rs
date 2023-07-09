use serde_json::Value as JsonValue;

use crate::{
    env::ENV,
    error::MyResult,
};

const URL: &str = "https://api.openai.com/v1/chat/completions";


pub struct GptClient {
    client: reqwest::Client,
}


impl GptClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    /// takes in a json request of the gpt api form and passes it
    /// along to open ai while attaching secret key header
    pub async fn chat(&self, request: JsonValue) -> MyResult<JsonValue> {

        if log::log_enabled!(log::Level::Trace) {
            log::trace!("Sending GPT Request: {}", serde_json::to_string_pretty(&request)?);
        }

        let response = self.client.post(URL)
            .bearer_auth(&ENV.gpt_api_key)
            .json(&request)
            .send()
            .await?;

        log::trace!("Received GPT Response: {response:#?}");

        Ok(serde_json::from_slice(&response.bytes().await?)?)

    }
}