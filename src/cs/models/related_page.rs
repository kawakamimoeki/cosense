use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct RelatedPage {
    id: String,
    title: String,
    #[serde(rename = "titleLc")]
    title_lc: String,
    image: Option<String>,
    descriptions: Option<Vec<String>>,
    #[serde(rename = "linksLc")]
    links_lc: Option<Vec<String>>,
    linked: Option<u32>,
    #[serde(rename = "pageRank")]
    page_rank: f32,
    #[serde(rename = "infoboxDisableLinks")]
    infobox_disable_links: Option<Vec<String>>,
    created: Option<u32>,
    updated: Option<u32>,
    accessed: Option<u32>,
}
