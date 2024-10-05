use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    current_project: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            current_project: String::from(""),
        }
    }
}

impl Config {
    pub fn get_current_project(&self) -> String {
        self.current_project.clone()
    }

    pub fn set_current_project(&mut self, project: String) {
        self.current_project = project;
    }
}
