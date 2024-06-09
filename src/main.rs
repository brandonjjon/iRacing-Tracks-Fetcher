mod auth;
mod config;
mod tracks;
mod utils;

use std::env;
use std::io::{self, Write};
use std::fs;
use dotenv::dotenv;
use config::Settings;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env_file_exists = fs::metadata(".env").is_ok();

    if env_file_exists {
        dotenv().ok();
    } else {
        println!("No .env file found. Please provide the necessary information.");
        let email = prompt("Enter your iRacing email: ")?;
        let password = prompt("Enter your iRacing password: ")?;

        env::set_var("IRACING_EMAIL", email);
        env::set_var("IRACING_PASSWORD", password);
    }

    let settings = Settings::new();
    let client = reqwest::Client::new();

    utils::ensure_output_directory(&settings.output_dir)?;

    let cookie = match auth::authenticate(&client, &settings.email, &settings.password, &settings.api_url).await {
        Ok(cookie) => cookie,
        Err(e) => {
            eprintln!("Error during authentication: {}", e);
            return Err(e);
        }
    };

    match tracks::fetch_tracks(&client, &cookie, &settings.api_url).await {
        Ok(tracks) => {
            if let Err(e) = utils::write_tracks_to_csv(tracks, &settings.output_dir) {
                eprintln!("Error writing to CSV: {}", e);
            } else {
                println!("Finished");
            }
        }
        Err(e) => {
            eprintln!("Error fetching tracks: {}", e);
        }
    }

    Ok(())
}

fn prompt(message: &str) -> Result<String, io::Error> {
    print!("{}", message);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}
