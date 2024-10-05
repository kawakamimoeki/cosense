use crate::cs::utils::load_config::load_config;
use crate::cs::utils::save_config::save_config;

pub fn set_current_project(project: &str) {
  let mut config = load_config();
  config.set_current_project(project.to_string());
  save_config(&config);
}
