use dotenv::dotenv;
use flightradar24_api::client::*;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_key = std::env::var("API_KEY").expect("API_KEY must be set.");

    let client = FlightRadarClient::new(api_key);

    let bounds = Bounds {
        north: 42.473,
        south: 37.331,
        west: -10.014,
        east: -4.115,
    };

    let live_flight_full = match client.get_live_flight(&bounds).await {
        Ok(live_data) => live_data,
        Err(e) => {
            eprintln!("Error fetching flight data: {}", e);
            FullLiveFlightResponse::default()
        }
    };

    println!("Flight Live Info: {:?}", live_flight_full);
}
