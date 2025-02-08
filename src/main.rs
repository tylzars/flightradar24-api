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

    let input = FullLiveFlightQuery {
        squawks: vec![2222, 3333, 5555, 7777],
        ..FullLiveFlightQuery::default()
    };

    let live_flight_full = match client.get_live_flight(&bounds, Some(&input)).await {
        Ok(live_data) => live_data,
        Err(e) => {
            eprintln!("{}", e);
            FullLiveFlightResponse::default()
        }
    };

    println!("Flight Live Info: {:?}", live_flight_full);
}
