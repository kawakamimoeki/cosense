pub async fn get_table(project: String, name: String, url: bool, sid: String) -> Result<(), Box<dyn std::error::Error>> {
  let endpoint = format!("https://scrapbox.io/api/table/{}/{}.csv", project, name);

  if url {
      println!("{}", endpoint);
      return Ok(());
  }

  let client = reqwest::Client::new();
  let response = client.get(endpoint)
      .header("Cookie", format!("connect.sid={}", sid))
      .send().await?;
  if response.status().is_success() {
      let table: String = response.text().await?;
      println!("{}", table);
  } else {
      println!("Error: {}", response.status());
  }

  Ok(())
}
