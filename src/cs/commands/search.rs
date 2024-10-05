use crate::cs::models::search_result::SearchResult;

pub async fn search(project: String, query: String, url: bool, link: bool, sid: String) -> Result<(), Box<dyn std::error::Error>> {
  let endpoint = format!("https://scrapbox.io/api/pages/{}/search/query?q={}", project, query);

  if url {
      println!("{}", endpoint);
      return Ok(());
  }

  let client = reqwest::Client::new();
  let response = client.get(endpoint)
      .header("Cookie", format!("connect.sid={}", sid))
      .send().await?;
  if response.status().is_success() {
      let result: SearchResult = response.json().await?;
      for title in result.get_page_titles() {
        if link {
            println!("https://scrapbox.io/{}/{}", project, title ) 
        } else {
            println!("{}", title);
        }
      }
  }

  Ok(())
}
