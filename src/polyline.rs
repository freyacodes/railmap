use reqwest::Client;
use serde_json::Value;
use crate::status::Status;

/// The path takes a comma-separated list of status IDs
const POLYLINE_URL: &str = "https://traewelling.de/api/v1/polyline/";

pub async fn get_polylines(client: &Client, statuses: &Vec<Status>) -> Vec<Value> {
    let mut lines: Vec<Value> = Vec::new();
    for (i, status) in statuses.iter().enumerate() {
        let data = get_polyline(client, &status).await["data"].take();
        lines.push(data);
        println!("Fetched polyline {:04} out of {}", i + 1, statuses.len());
    }
    lines
}

async fn get_polyline(client: &Client, status: &Status) -> Value {
    loop {
        let response = crate::http::handle_request(client.get(format!("{}{}", POLYLINE_URL, status.id))).await;

        let json = response
            .text()
            .await
            .expect("Could not decode response body");

        return serde_json::from_str(&json).expect("Failed to parse polyline json");
    }
}