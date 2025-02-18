#[cfg(test)]
mod tests {

    use dotenv::dotenv;
    use flightradar24_api::client::*;
    use flightradar24_api::flight_tracks_helper::*;

    fn setup_client() -> FlightRadarClient {
        dotenv().ok();
        let api_key = std::env::var("API_KEY").expect("API_KEY must be set.");
    
        let mut client = FlightRadarClient::new(api_key);
        client.update_base_url("https://fr24api.flightradar24.com/api/sandbox/".to_string());

        client
    }
    
    #[test]
    fn check_flight_tracks_by_id() {
        let client = setup_client();
        let flight_id = "390163bf"; // Must be hexcode
    
        let flight_list: Vec<flightradar24_api::client::Flight> =
            match client.get_flight_tracks_by_id(flight_id) {
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
        println!("Altitudes: {:?}", get_alt_from_flight(&flight_list));
        println!("Vspeeds: {:?}", get_vspeed_from_flight(&flight_list));
        println!("Tracks: {:?}", get_track_from_flight(&flight_list));
        println!("Squacks: {:?}", get_squack_from_flight(&flight_list));
        println!("Callsigns: {:?}", get_callsign_from_flight(&flight_list));
        println!("Sources: {:?}", get_source_from_flight(&flight_list));

        // Actually test
        assert_eq!(3, get_gspeed_from_flight(&flight_list).len())
    }
    
    #[test]
    fn check_api_usage() {
        dotenv().ok();
        let api_key = std::env::var("API_KEY").expect("API_KEY must be set.");
    
        let mut client = FlightRadarClient::new(api_key);
        client.update_base_url("https://fr24api.flightradar24.com/api/sandbox/".to_string());

        let api_usage: flightradar24_api::client::ApiUsageResponse = match client.get_api_usage("1y") {
            Ok(usage) => usage,
            Err(e) => {
                eprintln!("Error fetching api usage data: {}", e);
                ApiUsageResponse::default()
            }
        };
    
        println!("Usage: {:?}", api_usage);
    }

    #[test]
    fn check_airline_by_icao() {
        dotenv().ok();
        let api_key = std::env::var("API_KEY").expect("API_KEY must be set.");
    
        let mut client = FlightRadarClient::new(api_key);
        client.update_base_url("https://fr24api.flightradar24.com/api/sandbox/".to_string());

        let airline_info = match client.get_airline_by_icao("AAA") {
            Ok(airline) => airline,
            Err(e) => {
                eprintln!("Error fetching flight data: {}", e);
                Airline::default()
            }
        };
    
        println!("Airline Info: {:?}", airline_info);
    }

    #[test]
    fn check_aiport_by_code() {
        dotenv().ok();
        let api_key = std::env::var("API_KEY").expect("API_KEY must be set.");
    
        let mut client = FlightRadarClient::new(api_key);
        client.update_base_url("https://fr24api.flightradar24.com/api/sandbox/".to_string());

        let airport_info = match client.get_airport_by_code("MCO") {
            Ok(airline) => airline,
            Err(e) => {
                eprintln!("Error fetching flight data: {}", e);
                Airport::default()
            }
        };
    
        println!("Airport Info: {:?}", airport_info);
    }

    #[test]
    fn check_airport_light_by_code() {
        dotenv().ok();
        let api_key = std::env::var("API_KEY").expect("API_KEY must be set.");
    
        let mut client = FlightRadarClient::new(api_key);
        client.update_base_url("https://fr24api.flightradar24.com/api/sandbox/".to_string());    
    
        let airport_info = match client.get_airport_lite_by_code("MCO") {
            Ok(airline) => airline,
            Err(e) => {
                eprintln!("Error fetching flight data: {}", e);
                AirportLite::default()
            }
        };
    
        println!("Airport Lite Info: {:?}", airport_info);
    }    

    #[test]
    fn check_get_live_flight() {
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
        };

        let live_flight_full = match client.get_live_flight(Some(&input)) {
            Ok(live_data) => live_data,
            Err(e) => {
                eprintln!("{}", e);
                FullLiveFlightResponse::default()
            }
        };

        println!("Flight Live Info: {:?}", live_flight_full);
    }

    #[test]
    fn check_get_live_flight_light() {
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
    
        let live_flight_light = match client.get_live_flight_light(Some(&input)) {
            Ok(live_data) => live_data,
            Err(e) => {
                eprintln!("{}", e);
                LightLiveFlightResponse::default()
            }
        };
    
        println!("Light Flight Live Info: {:?}", live_flight_light);
    }

    #[test]
    fn check_get_historic_flight() {
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
    
        let historic_flight = match client.get_historic_flight(&1739401921, Some(&input)) {
            Ok(live_data) => live_data,
            Err(e) => {
                eprintln!("{}", e);
                FullLiveFlightResponse::default()
            }
        };
    
        println!("Historic Flight Info: {:?}", historic_flight);
    }

    #[test]
    fn check_get_historic_flight_light() {
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
    
        let historic_flight_light = match client.get_historic_flight_light(&1739401921, Some(&input)) {
            Ok(live_data) => live_data,
            Err(e) => {
                eprintln!("{}", e);
                LightLiveFlightResponse::default()
            }
        };
    
        println!("Light Historic Flight Info: {:?}", historic_flight_light);
    
    }
}