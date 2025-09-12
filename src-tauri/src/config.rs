use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub path_input: String,
    pub path_watermark: String,
    pub path_output: String,
    pub name_output: String,      // "watermark", "index"
    pub name_output_file: String, // "origin", "index"
    pub format_output: String,    // "png", "jpg", "original"
}

fn default_config() -> Config {
    return Config {
        path_input: "".to_string(),
        path_watermark: "".to_string(),
        path_output: "".to_string(),
        name_output: "watermark".to_string(),
        name_output_file: "origin".to_string(),
        format_output: "original".to_string(),
    };
}

pub fn get_config() -> Config {
    let config_path = "config.json";
    if !fs::exists(config_path).unwrap() {
        let config = default_config();
        let s = serde_json::to_string(&config).unwrap();
        fs::write(config_path, s).unwrap();
        return config;
    } else {
        let res = fs::read_to_string(config_path);
        let s = match res {
            Ok(s) => s,
            Err(_) => panic!("Error reading config file"),
        };
        let config: Config = serde_json::from_str(&s).unwrap();
        return config;
    }
}

pub fn set_config(config: Config) -> Config {
    let config_path = "config.json";
    let s = serde_json::to_string(&config).unwrap();
    fs::write(config_path, s).unwrap();
    return config;
}

pub fn prepare_config(
    path_input: String,
    watermark_path: String,
    output_path: String,
    name_output: String,
    name_output_file: String,
    format_output: String
) -> Config {
    let mut config: Config = get_config();
    config.path_input = path_input;
    config.path_watermark = watermark_path;
    config.path_output = output_path;
    config.name_output = name_output;
    config.name_output_file = name_output_file;
    config.format_output = format_output;
    return set_config(config);
}
