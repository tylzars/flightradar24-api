use std::vec;

use dotenv::dotenv;
use flightradar24_api::client::*;

fn main() {
    dotenv().ok();
    let api_key = std::env::var("API_KEY").expect("API_KEY must be set.");

    let mut client = FlightRadarClient::new(api_key);
    client.update_base_url("https://fr24api.flightradar24.com/api/sandbox/".to_string());

    let bounds_in = Bounds {
        north: 42.473,
        south: 37.331,
        west: -10.014,
        east: -4.115,
    };

    let input = FullLiveFlightQuery {
        bounds: Some(bounds_in),
        squawks: Some(vec![2222, 3333, 5555, 7777]),
        flights: Some(vec!["DL4204".to_string(), "aa1".to_string()]),
        callsigns: Some(vec!["WJA329".to_string(), "w1ssstt".to_string()]),
        registrations: Some(vec!["D-AFAM".to_string(), "E1".to_string()]),
        painted_as: Some(vec![
            "SAS".to_string(),
            "ART".to_string(),
            "aaa".to_string(),
        ]),
        operating_as: Some(vec![
            "SAS".to_string(),
            "ART".to_string(),
            "aaa".to_string(),
        ]),
        airports: Some(vec!["LHR".to_string(), "both:ESSA".to_string()]),
        routes: Some(vec!["SE-US".to_string(), "ESSA-JFK".to_string()]),
        aircraft: Some(vec![
            "B38M".to_string(),
            "A32*".to_string(),
            "*33".to_string(),
        ]),
        altitude_ranges: Some(vec![ApiRange {
            max: 3333,
            min: 2222,
        }]),
        categories: Some(vec!['B', 'P']),
        data_sources: Some(vec!["MLAT".to_string()]),
        airspaces: Some(vec!["ESAA".to_string(), "DFZZ".to_string()]),
        gspeed: Some(ApiRangeEnum::ApiRange(ApiRange {
            max: 5000,
            min: 2222,
        })),
        limit: Some(4444),
        //..FullLiveFlightQuery::default()
    };

    let live_flight_full = match client.get_live_flight(Some(&input)) {
        Ok(live_data) => live_data,
        Err(e) => {
            eprintln!("{}", e);
            FullLiveFlightResponse::default()
        }
    };

    println!("Flight Live Info: {:?}", live_flight_full);

    let input_default = FullLiveFlightQuery {
        ..FullLiveFlightQuery::default()
    };

    match client.get_live_flight(Some(&input_default)) {
        Ok(live_data) => println!("Default Query: {:?}", live_data),
        Err(e) => eprintln!("{}", e),
    };

    match client.get_live_flight_light(Some(&input)) {
        Ok(live_data) => println!("Light Flight Live Info: {:?}", live_data),
        Err(e) => eprintln!("{}", e),
    };

    match client.get_historic_flight(&1739401921, Some(&input)) {
        Ok(live_data) => println!("Historic Flight Info: {:?}", live_data),
        Err(e) => eprintln!("{}", e),
    };

    match client.get_historic_flight_light(&1739401921, Some(&input)) {
        Ok(live_data) => println!("Light Historic Flight Info: {:?}", live_data),
        Err(e) => eprintln!("{}", e),
    };

    match client.get_airline_by_icao("AAA") {
        Ok(airline) => println!("Airline Info: {:?}", airline),
        Err(e) => eprintln!("{}", e),
    };
}
