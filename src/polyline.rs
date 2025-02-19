use reqwest::Client;
use serde_json::Value;
use crate::status::Status;
use itertools::Itertools;

/// The path takes a comma-separated list of status IDs
const POLYLINE_URL: &str = "https://traewelling.de/api/v1/polyline/";
/// https://github.com/Traewelling/traewelling/blob/develop/app/Http/Controllers/API/v1/StatusController.php#L500
const POLYLINE_PAGE_COUNT: usize = 50;

pub async fn get_polylines(client: &Client, statuses: &Vec<Status>) -> Vec<Value> {
    let mut lines: Vec<Value> = Vec::new();
    let pages = (statuses.len() as f32 / POLYLINE_PAGE_COUNT as f32).ceil() as usize;

    for (page, chunk) in statuses.chunks(POLYLINE_PAGE_COUNT).enumerate() {
        let ids = chunk.iter().map(|s| s.id).join(",");
        let data = get_polyline_page(client, format!("{}{}", POLYLINE_URL, ids)).await["data"].take();
        lines.push(data);
        println!("Fetched polyline page {} out of {}", page + 1, pages);
    }
    lines
}

async fn get_polyline_page(client: &Client, url: String) -> Value {
    loop {
        let response = crate::http::handle_request(client.get(url)).await;

        let json = response
            .text()
            .await
            .expect("Could not decode response body");

        return serde_json::from_str(&json).expect(format!("Failed to parse polyline json:\n{}", json).as_str());
    }
}