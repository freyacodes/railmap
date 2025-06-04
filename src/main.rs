mod config;
mod traewelling;
mod gpx_reader;

use crate::config::load_config;
use std::path::Path;
use tokio::fs;
use traewelling::status;
use crate::traewelling::Traewelling;

const OUT_DIR: &str = "out";

#[tokio::main]
async fn main() {
    let config = load_config();

    if fs::try_exists(OUT_DIR).await.unwrap_or(false) {
        fs::remove_dir_all(OUT_DIR).await.unwrap();
    }

    let traewelling = Traewelling::new_from_env(config.clone());
    let statuses = traewelling.get_statuses().await;
    
    let mut polylines = traewelling.get_polylines(&statuses).await;
    polylines.extend(gpx_reader::read_polylines(&config));
    
    let polylines_str = serde_json::to_string(&polylines).unwrap();
    fs::write("polylines.json", polylines_str.clone())
        .await
        .expect("Unable to write file");
    println!("Wrote polylines.json");

    build_html(polylines_str.as_str()).await;
}

async fn build_html(polylines: &str) {
    let mut html = fs::read_to_string("template.html").await.unwrap();
    html = html.replace("GEOMETRY_PLACEHOLDER", polylines);
    fs::create_dir_all(Path::new(OUT_DIR)).await.unwrap();
    fs::write(Path::new(&format!("{}/index.html", OUT_DIR)), html)
        .await
        .unwrap();
}
