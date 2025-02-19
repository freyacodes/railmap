use reqwest::Client;
use serde::Deserialize;
use crate::http::handle_request;

const STATUSES_URL: &str = "https://traewelling.de/api/v1/user/freya/statuses";

#[derive(Deserialize, Debug)]
pub struct StatusesPage {
    pub data: Vec<Status>,
    pub links: Links
}

#[derive(Deserialize, Debug)]
pub struct Status {
    pub id: u32,
    pub visibility: u8,
    pub train: Train
}

/// This is a misnomer, it also covers other modes of transport
#[derive(Deserialize, Debug)]
pub struct Train {
    pub category: String,
}

#[derive(Deserialize, Debug)]
pub struct Links {
    pub next: Option<String>
}

pub async fn get_statuses(client: &Client) -> Vec<Status> {
    let mut statuses: Vec<Status> = Vec::new();
    let mut next_url: String = STATUSES_URL.to_string();

    loop {
        let response = handle_request(client.get(next_url.clone())).await;

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

pub fn filter_status(status: &Status) -> bool {
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