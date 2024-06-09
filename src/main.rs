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

    // Ensure the output directory exists
    utils::ensure_output_directory(&settings.output_dir)?;

    // Authenticate and get the cookie
    let cookie = match auth::authenticate(&client, &settings.email, &settings.password).await {
        Ok(cookie) => cookie,
        Err(e) => {
            eprintln!("Error during authentication: {}", e);
            return Err(e);
        }
    };

    // Fetch tracks using the authenticated session
    match tracks::fetch_tracks(&client, &cookie).await {
        Ok(tracks) => {
            // Write the tracks to a CSV file
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
