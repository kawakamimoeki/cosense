pub async fn get_code(page: String, name: String, url: bool, sid: String) -> Result<(), Box<dyn std::error::Error>> {
  let endpoint = format!("https://scrapbox.io/api/code/{}/{}", page, name);


  if url {
      println!("{}", endpoint);
      return Ok(());
  }

  let client = reqwest::Client::new();
  let response = client.get(endpoint)
      .header("Cookie", format!("connect.sid={}", sid))
      .send().await?;
  if response.status().is_success() {
      let code: String = response.text().await?;
      println!("{}", code);
  } else {
      println!("Error: {}", response.status());
  }

  Ok(())
}
