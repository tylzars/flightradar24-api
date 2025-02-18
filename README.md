# flightradar24-api

Rust crate for utilizing the FlightRadar24 API. All provided functionality from the API is implemented but some helper functionality is missing.

API information can be found [here](https://fr24api.flightradar24.com/).

Structure definitions are built from schemas provided [here](https://fr24api.flightradar24.com/docs/endpoints/overview).

This crate has been switched off of async, scrolling far enough back in the commits will allow for you to find an async implementation.

## Usage

Add dependency to `Cargo.toml`:

```toml
[dependencies]
change-detection = "1.2"
```

To start using this crate in your code:

```rust
use flightradar24_api::client::*;
let api_key = STRING_API_KEY;
let client = FlightRadarClient::new(api_key);
```

After this, functionality can be called however you'd like.

## Testing

The provided cargo tests are mainly for ensuring the parsers are able to get accurate data back in the structures. Please note that these will error out if run in quick succession as you are exceeding the rate limit.

Tests can be run with `cargo test`.
