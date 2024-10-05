use crate::makesense::models::search_result::SearchResult;

pub async fn get_search_json(project: String, pretty: bool, url: bool, query: String, sid: String) -> Result<(), Box<dyn std::error::Error>> {
  let client = reqwest::Client::new();
  let endpoint = format!("https://scrapbox.io/api/pages/{}/search/query?q={}", project, query);

  if url {
      println!("{}", endpoint);
      return Ok(());
  }

  let response = client.get(endpoint)
      .header("Cookie", format!("connect.sid={}", sid))
      .send().await?;
  if response.status().is_success() {
      let result: SearchResult = response.json().await?;
      if pretty {
          println!("{}", serde_json::to_string_pretty(&result)?);
      } else {
          println!("{}", serde_json::to_string(&result)?);
      }
  } else {
      println!("Error: {}", response.status());
  }

  Ok(())
}
