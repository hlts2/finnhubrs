# finnhubrs

Rust client library for [Finnhub API](https://finnhub.io/docs/api).

Finnhub provides institutional-grade financial data including real-time stock prices, global fundamentals, and alternative data.

OpenAPI spec sourced from [finnhub-go](https://github.com/Finnhub-Stock-API/finnhub-go/blob/master/api/openapi.yaml).

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
finnhubrs = { git = "https://github.com/hlts2/finnhubrs" }
```

## Usage

```rust
use finnhubrs::apis::configuration::Configuration;
use finnhubrs::apis::default_api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure API key
    let mut config = Configuration::new();
    config.api_key = Some(finnhubrs::apis::configuration::ApiKey {
        key: "YOUR_API_KEY".to_string(),
        prefix: None,
    });

    // Get company profile
    let profile = default_api::company_profile2(
        &config,
        Some("AAPL"),
        None,
        None,
    ).await?;

    println!("{:?}", profile);
    Ok(())
}
```

## Features

- `native-tls` (default) - Use system native TLS
- `rustls-tls` - Use rustls for TLS

```toml
[dependencies]
finnhubrs = { git = "https://github.com/hlts2/finnhubrs", default-features = false, features = ["rustls-tls"] }
```

## API Documentation

See [docs/](docs/) directory for detailed API documentation.

Full API reference: https://finnhub.io/docs/api

## Supported Endpoints

- Stock fundamentals (profile, financials, earnings, etc.)
- Stock price (quote, candles, tick data)
- ETFs & Mutual Funds
- Forex & Crypto
- Alternative data (insider transactions, congressional trading, etc.)
- Economic data
- And more...

## License

Apache-2.0
