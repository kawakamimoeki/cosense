use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Line {
    id: String,
    text: String,
    #[serde(rename = "userId")]
    user_id: String,
    created: Option<u32>,
    updated: Option<u32>,
}

impl Line {
    pub fn get_text(&self) -> Vec<String> {
        self.text.split("\n").map(|s| s.to_string()).collect()
    }
}
