use serde::{Deserialize, Serialize};

pub fn load_config() -> Config {
    let string = std::fs::read_to_string("config.yml")
        .expect("Failed to read config.yml");
    
    serde_yml::from_str(&string).expect("Failed to parse config.yml")
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    routes: Vec<Route>,
}

#[derive(Serialize, Deserialize)]
pub struct Route {
    pub identifier: String,
    pub files: Vec<String>,
}