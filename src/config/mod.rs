use dotenv::dotenv;
use color_eyre::Result;
use serde::Deserialize;
use std::env;
use tracing::{info, instrument};
use tracing_subscriber::EnvFilter;

#[derive(Debug, Deserialize)]
pub struct Config {
  pub host: String,
  pub port: i16,
}


impl Config {
  // Create a new config using dotenv environment variables
  #[instrument]
  pub fn new() -> Result<Config> {
    dotenv().ok();

    tracing_subscriber::fmt()
      .with_env_filter(EnvFilter::from_default_env())
      .init();

    info!("Loading configuration");
    Ok(Config {
      host: env::var("HOST")?,
      port: env::var("PORT")?.parse::<i16>()?,
    })
  }
}