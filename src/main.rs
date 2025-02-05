use flightradar24_api::client::FlightRadarClient;
use tokio;
use dotenv::dotenv;

#[tokio::main]
async fn main() {

    dotenv().ok();
    let api_key = std::env::var("API_KEY").expect("API_KEY must be set.");

    let client = FlightRadarClient::new(api_key);

    let flight_id = "34242a02"; // Must be hexcode

    match client.get_flight_by_id(flight_id).await {
        Ok(flight) => println!("Flight data: {:?}", flight),
        Err(e) => eprintln!("Error fetching flight data: {}", e),
    }
}
