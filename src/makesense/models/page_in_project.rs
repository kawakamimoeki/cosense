use serde::{Deserialize, Serialize};
use makesense::models::user::User;

use crate::makesense;

#[derive(Deserialize, Serialize, Debug)]
pub struct PageInProject {
    id: String,
    title: String,
    image: Option<String>,
    descriptions: Vec<String>,
    user: User,
    #[serde(rename = "lastUpdateUser")]
    last_update_user: User,
    pin: u64,
    views: Option<u32>,
    linked: Option<u32>,
    #[serde(rename = "commitId")]
    commit_id: String,
    created: Option<u32>,
    updated: Option<u32>,
    accessed: Option<u32>,
    #[serde(rename = "snapshotCreated")]
    snapshot_created: Option<u32>,
    #[serde(rename = "pageRank")]
    page_rank: f32,
    #[serde(rename = "linesCount")]
    lines_count: Option<u32>,
    #[serde(rename = "charsCount")]
    chars_count: Option<u32>,
}

impl PageInProject {
    pub fn get_title(&self) -> String {
        self.title.clone()
    }
}
