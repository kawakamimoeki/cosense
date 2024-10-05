pub async fn search_on_web(project: String, query: String, url: bool) -> Result<(), Box<dyn std::error::Error>> {
  let endpoint = format!("https://scrapbox.io/{}/search/page?q={}", project, query);
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
