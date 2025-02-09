/// Helper functions for the `flight-tracks` API Endpoint resultant data
use crate::client::Flight;
use chrono::{DateTime, Utc};

pub fn get_timestamps_from_flight(flights: &[Flight]) -> Vec<String> {
    let mut return_val: Vec<String> = Vec::new();

    for flight in flights.iter() {
        for data in flight.tracks.iter() {
            let now_parsed: DateTime<Utc> = data.timestamp.to_string().parse().unwrap();
            return_val.push(now_parsed.to_string());
        }
    }

    return_val
}

pub fn get_gspeed_from_flight(flights: &[Flight]) -> Vec<u32> {
    let mut return_val: Vec<u32> = Vec::new();

    for flight in flights.iter() {
        for data in flight.tracks.iter() {
            return_val.push(data.gspeed);
        }
    }

    return_val
}

pub fn get_lat_from_flight(flights: &[Flight]) -> Vec<f64> {
    let mut return_val: Vec<f64> = Vec::new();

    for flight in flights.iter() {
        for data in flight.tracks.iter() {
            return_val.push(data.lat);
        }
    }

    return_val
}

pub fn get_lon_from_flight(flights: &[Flight]) -> Vec<f64> {
    let mut return_val: Vec<f64> = Vec::new();

    for flight in flights.iter() {
        for data in flight.tracks.iter() {
            return_val.push(data.lon);
        }
    }

    return_val
}

pub fn get_alt_from_flight(flights: &[Flight]) -> Vec<u32> {
    let mut return_val: Vec<u32> = Vec::new();

    for flight in flights.iter() {
        for data in flight.tracks.iter() {
            return_val.push(data.alt);
        }
    }

    return_val
}

pub fn get_vspeed_from_flight(flights: &[Flight]) -> Vec<u32> {
    let mut return_val: Vec<u32> = Vec::new();

    for flight in flights.iter() {
        for data in flight.tracks.iter() {
            return_val.push(data.vspeed);
        }
    }

    return_val
}

pub fn get_track_from_flight(flights: &[Flight]) -> Vec<u32> {
    let mut return_val: Vec<u32> = Vec::new();

    for flight in flights.iter() {
        for data in flight.tracks.iter() {
            return_val.push(data.track);
        }
    }

    return_val
}

pub fn get_squack_from_flight(flights: &[Flight]) -> Vec<String> {
    let mut return_val: Vec<String> = Vec::new();

    for flight in flights.iter() {
        for data in flight.tracks.iter() {
            return_val.push(data.squawk.to_string());
        }
    }

    return_val
}

pub fn get_callsign_from_flight(flights: &[Flight]) -> Vec<String> {
    let mut return_val: Vec<String> = Vec::new();

    for flight in flights.iter() {
        for data in flight.tracks.iter() {
            return_val.push(data.callsign.to_string());
        }
    }

    return_val
}

pub fn get_source_from_flight(flights: &[Flight]) -> Vec<String> {
    let mut return_val: Vec<String> = Vec::new();

    for flight in flights.iter() {
        for data in flight.tracks.iter() {
            return_val.push(data.source.to_string());
        }
    }

    return_val
}
