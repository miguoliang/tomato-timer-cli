use chrono::Utc;
use reqwest::Client;
use serde::Serialize;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Serialize, Debug)]
pub struct Event {
    event: String,
    properties: HashMap<String, String>,
}

impl Event {
    pub fn new(name: &str) -> Self {
        let mut properties = HashMap::new();
        properties.insert("time".to_string(), Utc::now().to_rfc3339());
        properties.insert("$insert_id".to_string(), Uuid::new_v4().to_string());
        Event {
            event: name.to_string(),
            properties,
        }
    }

    pub fn add_property(&mut self, key: &str, value: &str) {
        self.properties.insert(key.to_string(), value.to_string());
    }
}

pub struct MixpanelClient {
    token: String,
    distinct_id: Option<String>,
}

impl MixpanelClient {
    pub fn new(token: String, distinct_id: Option<String>) -> Self {
        MixpanelClient { token, distinct_id }
    }

    pub async fn send_event(&self, event: &mut Event) {
        let client = Client::new();
        let url = "https://api.mixpanel.com/track";

        event.add_property("token", &self.token);
        if let Some(distinct_id) = &self.distinct_id {
            event.add_property("distinct_id", distinct_id);
        }

        let response = client.post(url).json(&[event]).send().await;

        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    match resp.text().await {
                        Ok(body) => println!("Event sent successfully. Response body: {:?}", body),
                        Err(err) => eprintln!("Failed to read response body: {}", err),
                    }
                } else {
                    eprintln!("Failed to send event: HTTP {}", resp.status());
                }
            }
            Err(err) => eprintln!("Failed to send event: {}", err),
        }
    }
}
