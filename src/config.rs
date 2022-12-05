use figment::{
    providers::{Env, Format, Json},
    Figment,
};
use serde::Deserialize;
use serde_json;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub dev_mode: bool,
    pub host_core_storage: String,
    pub enable_logging: bool,
}

// don't understand how to properly set default values, lib looks a bit over engineered
// at this point not worth spending effort on it, this solution is good enough
const DEFAULT_CONFIG: &str = r#"{
    "dev_mode": false,
    "host_core_storage": "http://127.0.0.1:8788",
    "enable_logging": true
}"#;

impl Default for Config {
    fn default() -> Self {
        serde_json::from_str(DEFAULT_CONFIG).unwrap()
    }
}

pub fn load_config() -> Config {
    let cfg = Figment::new()
        .merge(Json::string(DEFAULT_CONFIG))
        .merge(Env::raw())
        .extract::<Config>();
    if cfg.is_err() {
        println!(" ---========!!!! Invalid configuration !!!!========---");
        println!("    {}", cfg.err().unwrap());
        println!(" ---===============================================---");
        panic!("Invalid configuration")
    }
    cfg.unwrap()
}
