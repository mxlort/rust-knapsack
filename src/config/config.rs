use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Debug)]
pub struct Config {
    pub population_size: usize,
    pub generations_count: u32,
    pub frequency: u32,
    pub path: String,
    pub known_best: u32,
    pub stability_threshold: u32,
    pub mutations_per_1k: u32,
}

impl Config {
    pub fn load_params() -> Result<Config, String> {
        let yaml = fs::read_to_string("./config.yaml").expect("Unable to read the file");

        let deserialized_map = serde_yaml::from_str::<Vec<Config>>(&yaml);

        match deserialized_map {
            Ok(cfg) => Ok(cfg[0].clone()),
            Err(msg) => Err(msg.to_string()),
        }
    }
}
