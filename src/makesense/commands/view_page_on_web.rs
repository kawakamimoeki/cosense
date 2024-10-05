pub async fn view_page_on_web(page: String, url: bool) -> Result<(), Box<dyn std::error::Error>> {
  let endpoint = format!("https://scrapbox.io/{}", page);
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
