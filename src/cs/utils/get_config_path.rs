use std::path::PathBuf;

pub fn get_config_path() -> PathBuf {
  let mut path = dirs::home_dir().expect("Failed to get home directory");
  path.push(".cs.json");
  path
}
