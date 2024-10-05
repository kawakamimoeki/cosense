use clap::Parser;
use clap::Subcommand;
mod makesense;
use makesense::commands::get_code::get_code;
use makesense::commands::get_icon::get_icon;
use makesense::commands::get_page_json::get_page_json;
use makesense::commands::get_pages::get_pages;
use makesense::commands::get_pages_json::get_pages_json;
use makesense::commands::get_search_json::get_search_json;
use makesense::commands::get_table::get_table;
use makesense::commands::search::search;
use makesense::commands::search_on_web::search_on_web;
use makesense::commands::view_page::view_page;
use makesense::commands::view_page_on_web::view_page_on_web;
use keyring::Entry;
use makesense::commands::view_project_on_web::view_project_on_web;
use makesense::utils::get_current_project::get_current_project;
use makesense::utils::set_current_project::set_current_project;

#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    subcommand: SubCommands,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    #[clap(arg_required_else_help = true)]
    /// Login to Cosense
    Login {
        /// Cosense `connect.sid` cookie
        sid: String,
    },
    /// Get current project
    Project {
        /// Set current project
        name: Option<String>,
        #[clap(short, long)]
        /// Get URL of API
        url: bool,
        #[clap(short, long)]
        /// Open page on Browser
        web: bool,
    },
    /// List pages of project
    List {
        #[clap(short, long)]
        /// Get JSON
        json: bool,
        #[clap(short, long)]
        /// Pretty JSON
        pretty: bool,
        #[clap(short, long)]
        /// Get skip of pages
        skip: Option<u32>,
        #[clap(short, long)]
        /// Get limit of pages
        limit: Option<u32>,
        #[clap(long)]
        /// Get URL of API
        url: bool,
        #[clap(long)]
        /// Get link of pages
        link: bool,
        #[clap(short, long)]
        /// Open page on Browser
        web: bool,
    },
    /// Open page on Browser
    Page {
        #[clap(short, long)]
        /// Get JSON
        json: bool,
        #[clap(short, long)]
        /// Pretty JSON
        pretty: bool,
        /// Page name
        page: String,
        #[clap(short, long)]
        /// Open page on Browser
        web: bool,
        #[clap(short, long)]
        /// Get URL of API
        body: Option<String>,
        #[clap(short, long)]
        /// Get URL of API
        url: bool,
    },
    /// Get code of page
    Code {
        /// Name of code
        name: String,
        #[clap(short, long)]
        /// Get URL of API
        url: bool,
    },
    /// Get table CSV of page
    Table {
        /// Name of table
        name: String,
        #[clap(short, long)]
        /// Get URL of API
        url: bool,
    },
    /// Get icon of page
    Icon {
        /// Page name
        page: String,
        #[clap(short, long)]
        /// Get URL of API
        url: bool,
    },
    /// Search query
    Search {
        /// Search query
        query: String,
        #[clap(short, long)]
        /// Get URL of API
        url: bool,
        #[clap(short, long)]
        /// Get JSON
        json: bool,
        #[clap(short, long)]
        /// Pretty JSON
        pretty: bool,
        #[clap(short, long)]
        /// Get link of pages
        link: bool,
        #[clap(short, long)]
        /// Open page on Browser
        web: bool,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut project = get_current_project();
    let args = Args::parse();

    let entry = Entry::new("cosense", "default");
    let sid = match entry.expect("Failed to create keyring entry").get_password() {
        Ok(password) => password,
        Err(_) => String::new(),
    };

    match args.subcommand {
        SubCommands::Login { sid } => {
            let entry = Entry::new("cosense", "default");
            let _ = entry.expect("Failed to create keyring entry").set_password(&sid);
        }
        SubCommands::Project { name, url, web } => {
            if let Some(name) = name {
                set_current_project(&name);
                project = name;
            }
            if url {
                println!("https://scrapbox.io/{}", project);
            } else if web {
                view_project_on_web(project, url).await?;
            } else {
                println!("{}", project);
            }
        }
        SubCommands::List { json, pretty, skip, limit, url, link, web } => {
            if json {
                get_pages_json(project, pretty, skip, limit, url, sid).await?;
            } else if web {
                view_project_on_web(project, url).await?;
            } else {
                get_pages(project, skip, limit, url, sid, link).await?;
            }
        }
        SubCommands::Page { json, pretty, page, web, url, body } => {
            if json {
                get_page_json(project, page, pretty, url, sid).await?;
            } else if web {
                view_page_on_web(project, page, url, body).await?;
            } else {
                view_page(project, page, url, sid).await?;
            }
        }
        SubCommands::Code { name, url } => {
            get_code(project, name, url, sid).await?;
        }
        SubCommands::Table { name, url } => {
            get_table(project, name, url, sid).await?;
        }
        SubCommands::Icon { page, url } => {
            get_icon(project, page, url, sid).await?;
        }
        SubCommands::Search { url, query, json, pretty, web, link } => {
            if json {
                get_search_json(project, pretty, url, query, sid).await?;
            } else if web {
                search_on_web(project, query, url).await?;
            } else {
                search(project, query, url, link, sid).await?;
            }
        }
    }

    Ok(())
}
