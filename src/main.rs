use std::vec;

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
        flights: vec!["DL4204".to_string(), "aa1".to_string()],
        callsigns: vec!["WJA329".to_string(), "w1ssstt".to_string()],
        registrations: vec!["D-AFAM".to_string(), "E1".to_string()],
        painted_as: vec!["SAS".to_string(), "ART".to_string(), "aaa".to_string()],
        operating_as: vec!["SAS".to_string(), "ART".to_string(), "aaa".to_string()],
        airports: vec!["LHR".to_string(), "both:ESSA".to_string()],
        routes: vec!["SE-US".to_string(), "ESSA-JFK".to_string()],
        aircraft: vec!["B38M".to_string(), "A32*".to_string(), "*33".to_string()],
        altitude_ranges: vec![ApiRange {
            max: 3333,
            min: 2222,
        }],
        categories: vec!['B', 'P'],
        data_sources: vec!["MLAT".to_string()],
        airspaces: vec!["ESAA".to_string(), "DFZZ".to_string()],
        gspeed: ApiRangeEnum::ApiRange(ApiRange {
            max: 5000,
            min: 2222,
        }),
        //gspeed: ApiRangeEnum::U32(2332),
        limit: 4444,
        //..FullLiveFlightQuery::default()
    };

    let live_flight_full = match client.get_live_flight(&bounds, Some(&input)).await {
        Ok(live_data) => live_data,
        Err(e) => {
            eprintln!("{}", e);
            FullLiveFlightResponse::default()
        }
    };

    println!("Flight Live Info: {:?}", live_flight_full);

    let live_flight_light = match client.get_live_flight_light(&bounds, Some(&input)).await {
        Ok(live_data) => live_data,
        Err(e) => {
            eprintln!("{}", e);
            LightLiveFlightResponse::default()
        }
    };

    println!("Flight Live Info: {:?}", live_flight_light);
}
