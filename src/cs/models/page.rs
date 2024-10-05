use serde::{Deserialize, Serialize};
use crate::cs::models::user::User;
use crate::cs::models::line::Line;
use crate::cs::models::related_pages::RelatedPages;

#[derive(Deserialize, Serialize, Debug)]
pub struct Page {
    id: String,
    title: String,
    image: Option<String>,
    descriptions: Vec<String>,
    user: Option<User>,
    #[serde(rename = "lastUpdateUser")]
    last_update_user: Option<User>,
    pin: u64,
    views: Option<u32>,
    linked: Option<u32>,
    #[serde(rename = "commitId")]
    commit_id: Option<String>,
    created: Option<u32>,
    updated: Option<u32>,
    accessed: Option<u32>,
    #[serde(rename = "snapshotCreated")]
    snapshot_created: Option<u32>,
    #[serde(rename = "snapshotCount")]
    snapshot_count: Option<u32>,
    #[serde(rename = "pageRank")]
    page_rank: f32,
    #[serde(rename = "lastAccessed")]
    last_accessed: Option<u32>,
    #[serde(rename = "linesCount")]
    lines_count: Option<u32>,
    #[serde(rename = "charsCount")]
    chars_count: Option<u32>,
    persistent: bool,
    lines: Vec<Line>,
    links: Vec<String>,
    #[serde(rename = "projectLinks")]
    project_links: Vec<String>,
    icons: Vec<String>,
    files: Vec<String>,
    #[serde(rename = "infoboxDefinition")]
    infobox_definition: Option<Vec<String>>,
    #[serde(rename = "infoboxResult")]
    infobox_result: Option<Vec<String>>,
    #[serde(rename = "infoboxDisableLinks")]
    infobox_disable_links: Option<Vec<String>>,
    #[serde(rename = "relatedPages")]
    related_pages: Option<RelatedPages>,
    #[serde(rename = "collaborators")]
    collaborators: Vec<User>,
}

impl Page {
    pub fn get_line_text(&self) -> Vec<String> {
        self.lines.iter().map(|line| line.get_text()).flatten().collect()
    }

    pub fn get_links(&self) -> Vec<String> {
        self.links.clone()
    }
}
