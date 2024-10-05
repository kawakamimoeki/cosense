use crate::makesense::utils::load_config::load_config;

pub fn get_current_project() -> String {
  let config = load_config();
  config.get_current_project()
}
