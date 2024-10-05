use crate::cs::models::page::Page;

pub async fn get_page_json(project: String, page: String, pretty: bool, url: bool, sid: String) -> Result<(), Box<dyn std::error::Error>> {
  let client = reqwest::Client::new();
  let endpoint = format!("https://scrapbox.io/api/pages/{}/{}", project, page);

  if url {
      println!("{}", endpoint);
      return Ok(());
  }

  let response = client.get(endpoint)
      .header("Cookie", format!("connect.sid={}", sid))
      .send().await?;

  if response.status().is_success() {
      let page: Page = response.json().await?;
      if pretty {
          println!("{}", serde_json::to_string_pretty(&page)?);
      } else {
          println!("{}", serde_json::to_string(&page)?);
      }
  } else {
      println!("Error: {}", response.status());
  }

  Ok(())
}
