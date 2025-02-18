mod data;

use crate::data::{Status, StatusesPage};
use reqwest::{header, Body, Client, Response};
use serde_json::Value;
use std::env;
use std::time::Duration;
use tokio::fs;

const STATUSES_URL: &str = "https://traewelling.de/api/v1/user/freya/statuses";
/// The path takes a comma-separated list of status IDs
const POLYLINE_URL: &str = "https://traewelling.de/api/v1/polyline/";

#[tokio::main]
async fn main() {
    let bearer = env::var("TRAEWELLING_BEARER_TOKEN")
        .expect("Expected `TRAEWELLING_BEARER_TOKEN` env variable");

    let mut headers = header::HeaderMap::new();
    headers.insert(
        "Authorization",
        header::HeaderValue::from_str(&bearer).unwrap(),
    );
    let client = Client::builder().default_headers(headers).build().unwrap();

    let statuses: Vec<Status> = get_statuses(&client)
        .await
        .into_iter()
        .filter(filter_status)
        .collect();

    println!("Filtered to {} statuses", statuses.len());
    let data = get_polylines(&client, &statuses).await;

    fs::write("data.json", serde_json::to_string(&data).unwrap())
        .await
        .expect("Unable to write file");
    println!("Wrote data.json");
}

async fn get_statuses(client: &Client) -> Vec<Status> {
    let mut statuses: Vec<Status> = Vec::new();
    let mut next_url: String = STATUSES_URL.to_string();

    loop {
        let response = client
            .get(next_url.clone())
            .send()
            .await
            .expect("Could not send request");

        if (!handle_status(&response).await) {
            continue;
        }

        let body = response
            .text()
            .await
            .expect("Failed to decode response body");
        let page: StatusesPage = serde_json::from_str(&body)
            .expect(format!("Failed to decode response body {}", body).as_str());
        page.data.into_iter().for_each(|s| statuses.push(s));

        println!("Got {:04} statuses...", statuses.len());

        match page.links.next {
            None => {
                println!("Got all statuses!");
                return statuses;
            }
            Some(next) => next_url = next,
        }
    }
}

fn filter_status(status: &Status) -> bool {
    match status.visibility {
        0 | 1 | 2 | 4 => (),
        3 => return false,
        _ => {
            println!("Got unexpected visibility in status {:?}", status);
            return false;
        }
    };

    // See https://github.com/Traewelling/traewelling/blob/6193e1cec5347a16e90a56338abac4a9f977c28c/app/Enum/HafasTravelType.php#L18-L28
    match status.train.category.as_str() {
        "nationalExpress" | "national" | "regionalExp" | "regional" | "suburban" | "subway"
        | "tram" => true,
        "bus" | "ferry" | "taxi" | "plane" => false,
        _ => {
            println!("Got unexpected category in status {:?}", status);
            true
        }
    }
}

async fn get_polylines(client: &Client, statuses: &Vec<Status>) -> Value {
    let id_strings: Vec<String> = statuses.iter().map(|s| s.id.to_string()).collect();
    let url = format!("{}{}", POLYLINE_URL, id_strings.join(","));

    async fn try_get_lines(client: &Client, url: String) -> String {
        loop {
            let response = client
                .get(url.clone())
                .send()
                .await
                .expect("Could not send request");

            if (!handle_status(&response).await) {
                continue;
            }

            return response
                .text()
                .await
                .expect("Could not decode response body");
        }
    }

    serde_json::from_str(&try_get_lines(client, url).await).expect("Failed to parse polyline json")
}

async fn handle_status(response: &Response) -> bool {
    if response.status() == 429 {
        match response.headers().get(header::RETRY_AFTER) {
            None => {
                panic!("Got 429, but no retry-after header");
            }
            Some(value) => {
                let seconds = value
                    .to_str()
                    .unwrap()
                    .parse::<u64>()
                    .expect("Failed to parse retry-after header");
                tokio::time::sleep(Duration::from_secs(seconds)).await;
            }
        }
        false
    } else if response.status() != 200 {
        panic!("Unexpected status: {}", response.status());
    } else {
        true
    }
}
