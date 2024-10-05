use crate::cs::utils::get_config_path::get_config_path;
use crate::cs::utils::config::Config;
use std::fs;

pub fn load_config() -> Config {
  let config_path = get_config_path();
  if config_path.exists() {
      let contents = fs::read_to_string(config_path).expect("Failed to read config file");
      serde_json::from_str(&contents).expect("Failed to parse config file")
  } else {
      Config::default()
  }
}
