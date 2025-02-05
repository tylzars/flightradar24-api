use crate::client::Flight;
use crate::error::FlightRadarError;

// Take in borrowed param to prevent move error
// https://stackoverflow.com/questions/28800121/what-do-i-have-to-do-to-solve-a-use-of-moved-value-error
pub fn get_timestamps_from_flight(flight: &[Flight]) -> Result<Vec<String>, FlightRadarError> {
    let mut return_val: Vec<String> = vec![];

    for i in 0..flight.len() {
        match flight.get(i) {
            Some(first_flight) => {
                for j in 0..first_flight.tracks.len() {
                    match first_flight.tracks.get(j) {
                        Some(first_flight_data) => {
                            //println!("{}", first_flight_data.timestamp);
                            return_val.push(first_flight_data.timestamp.to_string());
                        }
                        None => {
                            return Err(FlightRadarError::General(
                                "Couldn't get timestamp".to_string(),
                            ))
                        }
                    }
                }
            }
            None => {
                return Err(FlightRadarError::General(
                    "Couldn't get timestamp".to_string(),
                ))
            }
        }
    }

    return Ok(return_val);
}

pub fn get_gspeed_from_flight(flight: &[Flight]) -> Result<Vec<u32>, FlightRadarError> {
    let mut return_val: Vec<u32> = vec![];

    for i in 0..flight.len() {
        match flight.get(i) {
            Some(first_flight) => {
                for j in 0..first_flight.tracks.len() {
                    match first_flight.tracks.get(j) {
                        Some(first_flight_data) => {
                            return_val.push(first_flight_data.gspeed);
                        }
                        None => {
                            return Err(FlightRadarError::General(
                                "Couldn't get timestamp".to_string(),
                            ))
                        }
                    }
                }
            }
            None => {
                return Err(FlightRadarError::General(
                    "Couldn't get timestamp".to_string(),
                ))
            }
        }
    }

    return Ok(return_val);
}
