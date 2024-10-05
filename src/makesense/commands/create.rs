pub async fn create(page: String, body: String, url: bool) -> Result<(), Box<dyn std::error::Error>> {
  let endpoint = format!("https://scrapbox.io/{}?body={}", page, body);
  if url {
      println!("{}", endpoint);
      return Ok(());
  }

  if webbrowser::open(&endpoint).is_ok() {
      println!("{}", endpoint);
  } else {
      println!("{}", endpoint);
  }
  Ok(())
}
