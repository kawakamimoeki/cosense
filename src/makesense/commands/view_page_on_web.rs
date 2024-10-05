pub async fn view_page_on_web(page: String, url: bool, body: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
  let endpoint = format!("https://scrapbox.io/{}?body={}", page, body.unwrap_or_else(|| "".to_string()));
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
