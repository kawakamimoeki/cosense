use crate::cs::models::page::Page;

pub async fn view_page(project: String, page: String, url: bool, sid: String) -> Result<(), Box<dyn std::error::Error>> {
  let endpoint = format!("https://scrapbox.io/api/pages/{}/{}", project, page);

  if url {
      println!("{}", endpoint);
      return Ok(());
  }

  let client = reqwest::Client::new();
  let response = client.get(endpoint)
      .header("Cookie", format!("connect.sid={}", sid))
      .send().await?;
  if response.status().is_success() {
      let page: Page = response.json().await?;
      for text in page.get_line_text() {
          println!("{}", text);
      }
  }
  Ok(())
}
