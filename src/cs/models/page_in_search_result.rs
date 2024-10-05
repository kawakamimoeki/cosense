use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct PageInSearchResult {
    id: String,
    title: String,
    image: Option<String>,
    words: Vec<String>,
    lines: Vec<String>,
}

impl PageInSearchResult {
    pub fn get_title(&self) -> String {
        self.title.clone()
    }
}
