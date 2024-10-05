use serde::{Deserialize, Serialize};
use crate::makesense::models::page_in_project::PageInProject;

#[derive(Deserialize, Serialize, Debug)]
pub struct Pages {
    #[serde(rename = "projectName")]
    project_name: String,
    skip: Option<u32>,
    limit: Option<u32>,
    count: Option<u32>,
    pages: Vec<PageInProject>,
}

impl Pages {
    pub fn get_titles(&self) -> Vec<String> {
        self.pages.iter().map(|page| page.get_title()).collect()
    }
}
