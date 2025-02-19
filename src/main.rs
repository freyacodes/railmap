mod status;
mod polyline;
mod http;

use reqwest::{header, Client};
use std::env;
use std::path::Path;
use tokio::fs;
use status::Status;

const OUT_DIR: &str = "out";

#[tokio::main]
async fn main() {
    if fs::try_exists(OUT_DIR).await.unwrap_or(false) {
        fs::remove_dir_all(OUT_DIR).await.unwrap();
    }

    let bearer = env::var("TRAEWELLING_BEARER_TOKEN")
        .expect("Expected `TRAEWELLING_BEARER_TOKEN` env variable");

    let mut headers = header::HeaderMap::new();
    headers.insert(
        "Authorization",
        header::HeaderValue::from_str(&bearer).unwrap(),
    );
    let client = Client::builder().default_headers(headers).build().unwrap();

    let statuses: Vec<Status> = status::get_statuses(&client)
        .await
        .into_iter()
        .filter(status::filter_status)
        .collect();

    println!("Filtered to {} statuses", statuses.len());
    let data = polyline::get_polylines(&client, &statuses).await;
    let data_str = serde_json::to_string(&data).unwrap();
    fs::write("polylines.json", data_str.clone())
        .await
        .expect("Unable to write file");
    println!("Wrote polylines.json");

    build_html(data_str.as_str()).await;
}

async fn build_html(polylines: &str) {
    let mut html = fs::read_to_string("template.html").await.unwrap();
    html = html.replace("GEOMETRY_PLACEHOLDER", polylines);
    fs::create_dir_all(Path::new(OUT_DIR)).await.unwrap();
    fs::write(Path::new(&format!("{}/index.html", OUT_DIR)), html)
        .await
        .unwrap();
}

