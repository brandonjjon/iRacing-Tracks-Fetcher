use reqwest::Client;
use serde::Deserialize;
use std::error::Error;

const API_URL: &str = "https://members-ng.iracing.com";

#[derive(Deserialize, Debug)]
pub struct Track {
    pub track_id: i32,
    pub track_name: String,
    pub config_name: Option<String>,
    pub location: Option<String>,
    pub track_config_length: Option<f64>,
    pub category: Option<String>,
}

#[derive(Deserialize, Debug)]
struct InitialResponse {
    link: String,
}

pub async fn fetch_tracks(client: &Client, cookie: &str) -> Result<Vec<Track>, Box<dyn Error>> {
    // First request to get the link
    let response = client
        .get(format!("{}/data/track/get", API_URL))
        .header(reqwest::header::COOKIE, cookie)
        .send()
        .await?;

    if response.status().is_success() {
        let initial_response: InitialResponse = response.json().await?;

        // Second request to fetch the actual track data from the provided link
        let track_response = client.get(&initial_response.link).send().await?;

        if track_response.status().is_success() {
            let tracks: Vec<Track> = track_response.json().await?;
            Ok(tracks)
        } else {
            let error_text = track_response.text().await?;
            Err(format!("Failed to fetch track data: {}", error_text).into())
        }
    } else {
        let error_text = response.text().await?;
        Err(format!("Failed to fetch data: {}", error_text).into())
    }
}
