use reqwest::Client;
use sha2::{Digest, Sha256};
use std::error::Error;

pub async fn authenticate(client: &Client, email: &str, password: &str, api_url: &str) -> Result<String, Box<dyn Error>> {
    let normalized_email = email.to_lowercase();
    let mut hasher = Sha256::new();
    hasher.update(format!("{}{}", password, normalized_email));
    let hashed_password = base64::encode(hasher.finalize());

    let auth_body = serde_json::json!({
        "email": email,
        "password": hashed_password,
    });

    let res = client
        .post(format!("{}/auth", api_url))
        .json(&auth_body)
        .send()
        .await?;

    if res.status().is_success() {
        let cookies = res.headers().get_all(reqwest::header::SET_COOKIE);
        let cookie_value = cookies
            .iter()
            .filter_map(|header| header.to_str().ok())
            .collect::<Vec<&str>>()
            .join("; ");
        Ok(cookie_value)
    } else {
        let error_text = res.text().await?;
        Err(format!("Authentication failed: {}", error_text).into())
    }
}
