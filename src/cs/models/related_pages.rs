use serde::{Deserialize, Serialize};
use crate::cs::models::related_page::RelatedPage;

#[derive(Deserialize, Serialize, Debug)]
pub struct RelatedPages {
    links1hop: Vec<RelatedPage>,
    links2hop: Vec<RelatedPage>,
    #[serde(rename = "projectLinks1hop")]
    project_links1hop: Vec<RelatedPage>,
    #[serde(rename = "hasBackLinksOrIcons")]
    has_back_links_or_icons: bool,
    search: String,
    #[serde(rename = "searchBackend")]
    search_backend: String,
}
