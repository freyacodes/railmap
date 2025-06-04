use std::env;
use reqwest::{header, Client};
use serde_json::Value;
use crate::config::Config;
use crate::traewelling::status::Status;

pub mod http;
mod polyline;
pub mod status;

pub struct Traewelling {
    config: Config,
    client: Client,
}

impl Traewelling {
    pub fn new_from_env(config: Config) -> Traewelling {
        let bearer = env::var("TRAEWELLING_BEARER_TOKEN")
            .expect("Expected `TRAEWELLING_BEARER_TOKEN` env variable");

        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Authorization",
            header::HeaderValue::from_str(&bearer).unwrap(),
        );
        let client = Client::builder().default_headers(headers).build().unwrap();

        Traewelling { config, client }
    }

    pub async fn get_statuses(&self) -> Vec<Status> {
        status::get_statuses(&self.client)
            .await
            .into_iter()
            .filter(status::filter_status)
            .filter(|status| !self.is_on_ignore_list(&status))
            .collect()
    }

    pub async fn get_polylines(&self, statuses: &Vec<Status>) -> Vec<Value> {
        polyline::get_polylines(&self.client, &statuses).await
    }

    fn is_on_ignore_list(&self, status: &Status) -> bool {
        let result = self.config.ignore.iter().any(|ignore_identifier| {
            if ignore_identifier == &status.id.to_string() { return true }
            if ignore_identifier == &format!("{} <-> {}", status.train.origin.name, status.train.destination.name) { return true }
            if ignore_identifier == &format!("{} <-> {}", status.train.destination.name, status.train.origin.name) { return true }
            return false
        });
        
        if result {
            //println!("Ignoring {}", format!("{} <-> {}", status.train.origin.name, status.train.destination.name));
        } else {
            println!("Accepting {}", format!("{} <-> {}", status.train.origin.name, status.train.destination.name));
        }
        
        result
    }
}
