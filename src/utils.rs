use crate::tracks::Track;
use std::error::Error;
use std::fs;
use std::path::Path;

pub fn ensure_output_directory(output_dir: &str) -> Result<(), Box<dyn Error>> {
    let path = Path::new(output_dir);
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

pub fn write_tracks_to_csv(tracks: Vec<Track>, output_dir: &str) -> Result<(), Box<dyn Error>> {
    // Ensure the output directory exists
    ensure_output_directory(output_dir)?;

    // Generate the file path
    let file_path = format!("{}/iracing_tracks_{}.csv", output_dir, chrono::Utc::now().format("%Y-%m-%d_%H-%M-%S"));
    let mut wtr = csv::Writer::from_path(&file_path)?;

    // Write the headers
    wtr.write_record(&["Track ID", "Track Name", "Config Name", "Location", "Track Length (miles)", "Category"])?;

    // Write the track data
    for track in tracks {
        wtr.write_record(&[
            track.track_id.to_string(),
            track.track_name.clone(),
            track.config_name.clone().unwrap_or("N/A".to_string()),
            track.location.clone().unwrap_or("N/A".to_string()),
            track.track_config_length.unwrap_or(0.0).to_string(),
            track.category.clone().unwrap_or("N/A".to_string()),
        ])?;
    }

    wtr.flush()?;
    println!("Tracks written to {}", file_path);
    Ok(())
}
