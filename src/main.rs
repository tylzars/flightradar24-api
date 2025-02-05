use dotenv::dotenv;
use flightradar24_api::client::FlightRadarClient;
use flightradar24_api::flight_tracks_helper::{get_gspeed_from_flight, get_timestamps_from_flight};
use tokio;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_key = std::env::var("API_KEY").expect("API_KEY must be set.");

    let client = FlightRadarClient::new(api_key);

    let flight_id = "390163bf"; // Must be hexcode

    let flight_list: Vec<flightradar24_api::client::Flight>;

    match client.get_flight_by_id(flight_id).await {
        Ok(flight) => flight_list = flight,
        Err(e) => {
            eprintln!("Error fetching flight data: {}", e);
            flight_list = Vec::with_capacity(1);
        }
    }

    match get_timestamps_from_flight(&flight_list) {
        Ok(timestamps) => {
            for i in 0..timestamps.len() {
                println!(
                    "Timestamp {} is {}",
                    i,
                    timestamps.get(i).unwrap_or(&String::from("N/A")) // Yea this isn't the right way to do this...
                );
            }
        }
        Err(e) => {
            eprintln!("Error parsing flight data: {}", e);
        }
    }

    match get_gspeed_from_flight(&flight_list) {
        Ok(timestamps) => {
            for i in 0..timestamps.len() {
                println!(
                    "Timestamp {} is {}",
                    i,
                    timestamps.get(i).unwrap_or(&0) // Yea this isn't the right way to do this...
                );
            }
        }
        Err(e) => {
            eprintln!("Error parsing flight data: {}", e);
        }
    }
}
