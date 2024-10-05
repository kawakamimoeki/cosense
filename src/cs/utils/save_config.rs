use std::fs;
use crate::cs::utils::get_config_path::get_config_path;
use crate::cs::utils::config::Config;

pub fn save_config(config: &Config) {
  let config_path = get_config_path();
  let contents = serde_json::to_string_pretty(config).expect("Failed to serialize config");
  fs::write(config_path, contents).expect("Failed to write config file");
}
