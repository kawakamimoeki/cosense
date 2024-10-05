use crate::makesense::models::pages::Pages;

pub async fn get_pages_json(project: String, pretty: bool, skip: Option<u32>, limit: Option<u32>, url: bool, sid: String) -> Result<(), Box<dyn std::error::Error>> {
  let client = reqwest::Client::new();
  let mut endpoint = format!("https://scrapbox.io/api/pages/{}", project);

  if url {
      println!("{}", endpoint);
      return Ok(());
  }

  let mut query_params = Vec::new();
  if let Some(skip) = skip {
      query_params.push(format!("skip={}", skip));
  }
  if let Some(limit) = limit {
      query_params.push(format!("limit={}", limit));
  }

  endpoint.push_str("?");
  endpoint.push_str(&query_params.join("&"));

  let response = client.get(endpoint)
      .header("Cookie", format!("connect.sid={}", sid))
      .send().await?;

  if response.status().is_success() {
      let pages: Pages = response.json().await?;
      if pretty {
          println!("{}", serde_json::to_string_pretty(&pages)?);
      } else {
          println!("{}", serde_json::to_string(&pages)?);
      }
  } else {
      println!("Error: {}", response.status());
  }

  Ok(())
}
