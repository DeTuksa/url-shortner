use clap::{Parser, Subcommand};
use url_shortner::UrlShortner;

#[derive(Parser)]
#[command(name = "URL Shortner")]
#[command(about = "URL shortner CLI tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    Shorten {
        url: String
    },
    Retrieve {
        code: String
    }
}

fn main() {
    let args = Cli::parse();
    let shortner = UrlShortner::new("temp_db");

    match args.command {
        Commands::Shorten { url } => {
            let short_url = shortner.shorten_url(&url);
            println!("Shortened URL: {}", short_url);
        },
        Commands::Retrieve { code } => {
            match shortner.get_url(&code) {
                Some(url) => println!("Original URL: {}", url),
                None => eprintln!("Short url not found")
            }
        }
    }
}