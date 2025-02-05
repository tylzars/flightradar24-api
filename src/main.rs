use dotenv::dotenv;
use flightradar24_api::client::FlightRadarClient;
use tokio;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_key = std::env::var("API_KEY").expect("API_KEY must be set.");

    let client = FlightRadarClient::new(api_key);

    let flight_id = "34242a02"; // Must be hexcode

    let flight_list: Vec<flightradar24_api::client::Flight>;

    match client.get_flight_by_id(flight_id).await {
        Ok(flight) => flight_list = flight,
        Err(e) => {
            eprintln!("Error fetching flight data: {}", e);
            flight_list = Vec::with_capacity(1);
        }
    }

    for i in 0..flight_list.len() {
        match flight_list.get(i) {
            Some(first_flight) => {
                for j in 0..first_flight.tracks.len() {
                    match first_flight.tracks.get(j) {
                        Some(first_flight_data) => {
                            println!(
                                "Timestamp is: {} with ground speed of {}",
                                first_flight_data.timestamp, first_flight_data.gspeed
                            )
                        }
                        None => eprintln!("Error getting data from item"),
                    }
                }
            }
            None => eprintln!("Error getting item"),
        }
    }
}
