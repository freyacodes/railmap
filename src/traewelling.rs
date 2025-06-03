use std::env;
use reqwest::{header, Client};
use serde_json::Value;
use crate::traewelling::status::Status;

pub mod http;
mod polyline;
pub mod status;

pub struct Traewelling {
    client: Client,
}

impl Traewelling {
    pub fn new_from_env() -> Traewelling {
        let bearer = env::var("TRAEWELLING_BEARER_TOKEN")
            .expect("Expected `TRAEWELLING_BEARER_TOKEN` env variable");

        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Authorization",
            header::HeaderValue::from_str(&bearer).unwrap(),
        );
        let client = Client::builder().default_headers(headers).build().unwrap();
        
        Traewelling { client }
    }
    
    pub async fn get_statuses(&self) -> Vec<Status> {
        status::get_statuses(&self.client)
            .await
            .into_iter()
            .filter(status::filter_status)
            .collect()
    }
    
    pub async fn get_polylines(&self, statuses: &Vec<Status>) -> Vec<Value> {
        polyline::get_polylines(&self.client, &statuses).await
    }
}

