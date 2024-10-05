use crate::cs::models::page::Page;

pub async fn get_links(project: String, name: String, url: bool, sid: String) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let endpoint = format!("https://scrapbox.io/api/pages/{}/{}", project, name);
  
    if url {
        println!("{}", endpoint);
        return Ok(());
    }
  
    let response = client.get(endpoint)
        .header("Cookie", format!("connect.sid={}", sid))
        .send().await?;
  
    if response.status().is_success() {
        let page: Page = response.json().await?;
        let links = page.get_links();
        for link in links {
            println!("{}", link);
        }
    }

    Ok(())
}
