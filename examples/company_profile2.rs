use finnhubrs::apis::configuration::Configuration;
use finnhubrs::apis::default_api;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("FINNHUB_API_KEY")?;
    let mut config = Configuration::new();
    config.api_key = Some(finnhubrs::apis::configuration::ApiKey {
        key: api_key,
        prefix: None,
    });

    let profile = default_api::company_profile2(&config, Some("AAPL"), None, None).await?;
    println!("{:?}", profile);
    Ok(())
}
