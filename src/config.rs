use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub api_url: String,
    pub email: String,
    pub password: String,
    pub output_dir: String,
}

impl Settings {
    pub fn new() -> Self {
        let builder = config::Config::builder()
            .add_source(config::Environment::with_prefix("APP").separator("__"));

        let config = builder.build().unwrap();

        Settings {
            api_url: config.get::<String>("API_URL").unwrap_or_else(|_| "https://members-ng.iracing.com".to_string()),
            email: env::var("IRACING_EMAIL").expect("IRACING_EMAIL must be set"),
            password: env::var("IRACING_PASSWORD").expect("IRACING_PASSWORD must be set"),
            output_dir: config.get::<String>("OUTPUT_DIR").unwrap_or_else(|_| "output".to_string()),
        }
    }
}
