use clap::Parser;
use clap::Subcommand;
mod makesense;
use makesense::commands::create::create;
use makesense::commands::get_code::get_code;
use makesense::commands::get_icon::get_icon;
use makesense::commands::get_pages::get_pages;
use makesense::commands::get_project_json::get_project_json;
use makesense::commands::get_page_json::get_page_json;
use makesense::commands::get_search_json::get_search_json;
use makesense::commands::get_table::get_table;
use makesense::commands::search::search;
use makesense::commands::view_page::view_page;
use makesense::commands::view_page_on_web::view_page_on_web;
use keyring::Entry;

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
    /// Get JSON data of project or page
    Json {
        /// Project name or page name
        resource: String,
        #[clap(short, long)]
        /// Get pretty JSON
        pretty: bool,
        #[clap(short, long)]
        /// Get skip of pages
        skip: Option<u32>,
        #[clap(short, long)]
        /// Get limit of pages
        limit: Option<u32>,
        #[clap(short, long)]
        /// Get URL of API
        url: bool,
        #[clap(short, long)]
        /// Search query
        query: Option<String>,
    },
    /// Get page title list of project
    Pages {
        /// Project name
        project: String,
        #[clap(short, long)]
        /// Get skip of pages
        skip: Option<u32>,
        #[clap(long)]
        /// Get limit of pages
        limit: Option<u32>,
        #[clap(short, long)]
        /// Get URL of API
        url: bool,
        #[clap(long)]
        /// Get link of pages
        link: bool,
    },
    /// Create page with body on Browser
    Create {
        /// Page name
        page: String,
        /// Body of page
        body: String,
        #[clap(short, long)]
        /// Get URL of API
        url: bool,
    },
    /// Open page on Browser
    Page {
        /// Page name
        page: String,
        #[clap(short, long)]
        /// Open page on Browser
        web: bool,
        #[clap(short, long)]
        /// Get URL of API
        url: bool,
    },
    /// Get code of page
    Code {
        /// Page name
        page: String,
        /// Name of code
        name: String,
        #[clap(short, long)]
        /// Get URL of API
        url: bool,
    },
    /// Get table CSV of page
    Table {
        /// Page name
        page: String,
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
    Search {
        /// project name
        project: String,
        /// Search query
        query: String,
        #[clap(short, long)]
        /// Get URL of API
        url: bool,
        #[clap(short, long)]
        // Get Link
        link: bool,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
        SubCommands::Json { resource, pretty, skip, limit, url, query } => {
            if resource.contains('/') {
                get_page_json(resource, pretty, url, sid).await?;
            } else if let Some(query) = query {
                get_search_json(resource, pretty, url, query, sid).await?;
            } else {
                get_project_json(resource, pretty, skip, limit, url, sid).await?;
            }
        }
        SubCommands::Pages { project, skip, limit, url, link } => {
            if !project.contains('/') {
                get_pages(project, skip, limit, url, sid, link).await?;
            }
        }
        SubCommands::Create { page, body, url } => {
            create(page, body, url).await?;
        }
        SubCommands::Page { page, web, url } => {
            if web {
                view_page_on_web(page, url).await?;
            } else {
                view_page(page, url, sid).await?;
            }
        }
        SubCommands::Code { page, name, url } => {
            get_code(page, name, url, sid).await?;
        }
        SubCommands::Table { page, name, url } => {
            get_table(page, name, url, sid).await?;
        }
        SubCommands::Icon { page, url } => {
            get_icon(page, url, sid).await?;
        }
        SubCommands::Search { query, url, project, link } => {
            search(query, url, project, link, sid).await?;
        }
    }

    Ok(())
}
