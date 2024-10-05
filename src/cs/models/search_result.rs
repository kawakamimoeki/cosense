use serde::{Deserialize, Serialize};
use crate::cs::models::page_in_search_result::PageInSearchResult;
use crate::cs::models::query::Query;

#[derive(Deserialize, Serialize, Debug)]
pub struct SearchResult {
    #[serde(rename = "searchQuery")]
    search_query: String,
    pages: Vec<PageInSearchResult>,
    exists_exact_title_match: Option<bool>,
    field: Option<String>,
    query: Query,
    backend: Option<String>,
}

impl SearchResult {
    pub fn get_page_titles(&self) -> Vec<String> {
        self.pages.iter().map(|page| page.get_title()).collect()
    }
}
