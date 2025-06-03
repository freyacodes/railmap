use serde::{Deserialize, Serialize};

pub fn load_config() -> Config {
    let string = std::fs::read_to_string("config.yml")
        .expect("Failed to read config.yml");
    
    serde_yml::from_str(&string).expect("Failed to parse config.yml")
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub ignore: Vec<String>,
    pub routes: Vec<String>
}
