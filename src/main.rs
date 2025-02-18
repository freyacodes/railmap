mod data;

use crate::data::{Status, StatusesPage};
use reqwest::{header, Client};
use std::env;
use std::time::Duration;

const STATUSES_URL: &str = "https://traewelling.de/api/v1/user/freya/statuses";

#[tokio::main]
async fn main() {
    let bearer = env::var("TRAEWELLING_BEARER_TOKEN")
        .expect("Expected `TRAEWELLING_BEARER_TOKEN` env variable");

    let mut headers = header::HeaderMap::new();
    headers.insert("Authorization", header::HeaderValue::from_str(&bearer).unwrap());
    let client = Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();

    get_statuses(&client).await;
}

async fn get_statuses(client: &Client) -> Vec<Status> {
    let mut statuses: Vec<Status> = Vec::new();
    let mut next_url: String = STATUSES_URL.to_string();

    loop {
        let response = client.get(next_url.clone())
            .send()
            .await
            .expect("Could not send request");
        
        if response.status() == 429 { 
            match response.headers().get(header::RETRY_AFTER) {
                None => {
                    panic!("Got 429, but no retry-after header");
                }
                Some(value) => {
                    let seconds = value.to_str().unwrap().parse::<u64>()
                        .expect("Failed to parse retry-after header");
                    tokio::time::sleep(Duration::from_secs(seconds)).await;
                    continue
                }
            }
        } else if response.status() != 200 {
            panic!("Unexpected status: {}", response.status());
        }
        
        let body = response.text().await.expect("Failed to decode response body");
        let page: StatusesPage = serde_json::from_str(&body)
            .expect(format!("Failed to decode response body {}", body).as_str());
        page.data.into_iter().for_each(|s| statuses.push(s));

        println!("Got {:03} statuses...", statuses.len());

        match page.links.next {
            None => {
                println!("Got all statuses!");
                return statuses
            },
            Some(next) => next_url = next,
        }
    }
}