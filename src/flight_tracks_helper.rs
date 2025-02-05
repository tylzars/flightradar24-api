use crate::client::Flight;
use crate::error::FlightRadarError;

pub fn get_timestamps_from_flight(flights: &[Flight]) -> Result<Vec<String>, FlightRadarError> {
    let mut return_val: Vec<String> = Vec::new();

    for flight in flights.iter() {
        for data in flight.tracks.iter() {
            return_val.push(data.timestamp.to_string());
        }
    }

    if return_val.is_empty() {
        return Err(FlightRadarError::General(
            "Couldn't parse out any timestamps".to_string(),
        ));
    }

    return Ok(return_val);
}

pub fn get_gspeed_from_flight(flights: &[Flight]) -> Result<Vec<u32>, FlightRadarError> {
    let mut return_val: Vec<u32> = Vec::new();

    for flight in flights.iter() {
        for data in flight.tracks.iter() {
            return_val.push(data.gspeed);
        }
    }

    if return_val.is_empty() {
        return Err(FlightRadarError::General(
            "Couldn't parse out any groundspeed".to_string(),
        ));
    }

    return Ok(return_val);
}

pub fn get_lat_from_flight(flights: &[Flight]) -> Result<Vec<f64>, FlightRadarError> {
    let mut return_val: Vec<f64> = Vec::new();

    for flight in flights.iter() {
        for data in flight.tracks.iter() {
            return_val.push(data.lat);
        }
    }

    if return_val.is_empty() {
        return Err(FlightRadarError::General(
            "Couldn't parse out any latitude".to_string(),
        ));
    }

    return Ok(return_val);
}

pub fn get_lon_from_flight(flights: &[Flight]) -> Result<Vec<f64>, FlightRadarError> {
    let mut return_val: Vec<f64> = Vec::new();

    for flight in flights.iter() {
        for data in flight.tracks.iter() {
            return_val.push(data.lon);
        }
    }

    if return_val.is_empty() {
        return Err(FlightRadarError::General(
            "Couldn't parse out any longitude".to_string(),
        ));
    }

    return Ok(return_val);
}

pub fn get_alt_from_flight(flights: &[Flight]) -> Result<Vec<u32>, FlightRadarError> {
    let mut return_val: Vec<u32> = Vec::new();

    for flight in flights.iter() {
        for data in flight.tracks.iter() {
            return_val.push(data.alt);
        }
    }

    if return_val.is_empty() {
        return Err(FlightRadarError::General(
            "Couldn't parse out any altitude".to_string(),
        ));
    }

    return Ok(return_val);
}

pub fn get_vspeed_from_flight(flights: &[Flight]) -> Result<Vec<u32>, FlightRadarError> {
    let mut return_val: Vec<u32> = Vec::new();

    for flight in flights.iter() {
        for data in flight.tracks.iter() {
            return_val.push(data.vspeed);
        }
    }

    if return_val.is_empty() {
        return Err(FlightRadarError::General(
            "Couldn't parse out any vspeed".to_string(),
        ));
    }

    return Ok(return_val);
}

pub fn get_track_from_flight(flights: &[Flight]) -> Result<Vec<u32>, FlightRadarError> {
    let mut return_val: Vec<u32> = Vec::new();

    for flight in flights.iter() {
        for data in flight.tracks.iter() {
            return_val.push(data.track);
        }
    }

    if return_val.is_empty() {
        return Err(FlightRadarError::General(
            "Couldn't parse out any track".to_string(),
        ));
    }

    return Ok(return_val);
}

pub fn get_squack_from_flight(flights: &[Flight]) -> Result<Vec<String>, FlightRadarError> {
    let mut return_val: Vec<String> = Vec::new();

    for flight in flights.iter() {
        for data in flight.tracks.iter() {
            return_val.push(data.squawk.to_string());
        }
    }

    if return_val.is_empty() {
        return Err(FlightRadarError::General(
            "Couldn't parse out any squacks".to_string(),
        ));
    }

    return Ok(return_val);
}

pub fn get_callsign_from_flight(flights: &[Flight]) -> Result<Vec<String>, FlightRadarError> {
    let mut return_val: Vec<String> = Vec::new();

    for flight in flights.iter() {
        for data in flight.tracks.iter() {
            return_val.push(data.callsign.to_string());
        }
    }

    if return_val.is_empty() {
        return Err(FlightRadarError::General(
            "Couldn't parse out any callsigns".to_string(),
        ));
    }

    return Ok(return_val);
}

pub fn get_source_from_flight(flights: &[Flight]) -> Result<Vec<String>, FlightRadarError> {
    let mut return_val: Vec<String> = Vec::new();

    for flight in flights.iter() {
        for data in flight.tracks.iter() {
            return_val.push(data.source.to_string());
        }
    }

    if return_val.is_empty() {
        return Err(FlightRadarError::General(
            "Couldn't parse out any source".to_string(),
        ));
    }

    return Ok(return_val);
}
