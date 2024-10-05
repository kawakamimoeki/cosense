pub async fn get_icon(page: String, url: bool, sid: String) -> Result<(), Box<dyn std::error::Error>> {
  let endpoint = format!("https://scrapbox.io/api/pages/{}/icon", page);

  if url {
      println!("{}", endpoint);
      return Ok(());
  }

  let client = reqwest::Client::new();
  let response = client.get(endpoint)
      .header("Cookie", format!("connect.sid={}", sid))
      .send().await?;
  if response.status().is_success() {
      let icon: String = response.text().await?;
      println!("{}", icon);
  } else {
      println!("Error: {}", response.status());
  }

  Ok(())
}
