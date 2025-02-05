use dotenv::dotenv;
use flightradar24_api::client::FlightRadarClient;
use flightradar24_api::flight_tracks_helper::*;
use tokio;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_key = std::env::var("API_KEY").expect("API_KEY must be set.");

    let client = FlightRadarClient::new(api_key);

    let flight_id = "390163bf"; // Must be hexcode

    let flight_list: Vec<flightradar24_api::client::Flight> =
        match client.get_flight_by_id(flight_id).await {
            Ok(flight) => flight,
            Err(e) => {
                eprintln!("Error fetching flight data: {}", e);
                Vec::with_capacity(1)
            }
        };

    println!("Groundspeeds: {:?}", get_gspeed_from_flight(&flight_list));
    println!("Timestamps: {:?}", get_timestamps_from_flight(&flight_list));
    println!("Latitudes: {:?}", get_lat_from_flight(&flight_list));
    println!("Longitudes: {:?}", get_lon_from_flight(&flight_list));
    println!("Altitudes: {:?}", get_alt_from_flight(&flight_list))
}
