extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate serde_derive;

use serde::{Deserialize, Serialize};
use serde_derive::{Deserialize, Serialize};
use reqwest::blocking::Client;

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestData {
    inputs: serde_json::Value,
    query: String,
    conversation_id: String,
    user: String,
}

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub event: String,
    pub task_id: String,
    pub id: String,
    pub answer: String,
    pub created_at: i64,
    pub conversation_id: String,
}

pub struct ChatService {
    client: Client,
}

impl ChatService {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub fn send_query(&self, query: String) -> Result<ApiResponse, String> {
        let data = RequestData {
            inputs: serde_json::Value::Object(serde_json::Map::new()),
            query,
            conversation_id: "".to_string(),
            user: "abc-123".to_string(),
        };

        let response = self.client.post("https://api.dify.ai/v1/chat-messages")
            .header("Authorization", "Bearer app-ymZW0myntxVFDd5VYIW0FGFd")
            .header("Content-Type", "application/json")
            .json(&data)
            .send();

        match response {
            Ok(res) => {
                if res.status().is_success() {
                    let api_response: ApiResponse = res.json()
                        .expect("Failed to deserialize response");
                    Ok(api_response)
                } else {
                    Err(format!("Error: {}", res.status()))
                }
            }
            Err(e) => Err(format!("Failed to send request: {}", e)),
        }
    }
}
