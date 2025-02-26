use crate::error::FlightRadarError;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

/// Main structure for storing API internal data
pub struct FlightRadarClient {
    client: Client,
    base_url: String,
    api_key: String,
}

impl FlightRadarClient {
    /// Creates a new instance of the client.
    /// # Arguments
    ///   * `api_key` - Your Flightradar24 API key.
    pub fn new(api_key: String) -> Self {
        FlightRadarClient {
            client: Client::new(),
            // This is an example base URL. Adjust as needed.
            base_url: "https://fr24api.flightradar24.com/api/".to_string(),
            api_key,
        }
    }

    pub fn update_base_url(&mut self, base_url: String) {
        self.base_url = base_url;
    }

    /// Build parameters string for query URL
    /// # Arguments
    ///   * `params` - Structure of all possible parameters
    /// # Returns
    ///   `String` of params on success or a `FlightRadarError` on failure.
    fn build_query_params(params: &FullLiveFlightQuery) -> Result<String, FlightRadarError> {
        let mut url: String = String::new();

        if let Some(bounds) = &params.bounds {
            let bounds_str = format!(
                "?bounds={},{},{},{}",
                bounds.north, bounds.south, bounds.west, bounds.east
            );
            url.push_str(&bounds_str);
        }
        if let Some(flights) = &params.flights {
            url.push_str("&flights=");
            for flight in flights {
                if flight.chars().all(char::is_alphanumeric) && flight.len() > 2 {
                    url.push_str(&flight.to_string());
                    url.push(',');
                } else {
                    return Err(FlightRadarError::Parameter(format!("Flight #: {}", flight)));
                }
            }
            url.pop();
        }
        if let Some(callsigns) = &params.callsigns {
            url.push_str("&callsigns=");
            for callsign in callsigns {
                if callsign.chars().all(char::is_alphanumeric)
                    && callsign.len() > 2
                    && callsign.len() <= 8
                {
                    url.push_str(&callsign.to_string());
                    url.push(',');
                } else {
                    return Err(FlightRadarError::Parameter(format!(
                        "Callsign: {}",
                        callsign
                    )));
                }
            }
            url.pop();
        }
        if let Some(registrations) = &params.registrations {
            url.push_str("&registrations=");
            for registration in registrations {
                if registration
                    .chars()
                    .all(|c| c.is_alphanumeric() || c == '-')
                    && registration.len() > 1
                    && registration.len() <= 12
                {
                    url.push_str(&registration.to_string());
                    url.push(',');
                } else {
                    return Err(FlightRadarError::Parameter(format!(
                        "Registration #: {}",
                        registration
                    )));
                }
            }
            url.pop();
        }
        if let Some(painted_as) = &params.painted_as {
            url.push_str("&painted_as=");
            for painted in painted_as {
                if painted.chars().all(char::is_alphabetic) && painted.len() == 3 {
                    url.push_str(&painted.to_string());
                    url.push(',');
                } else {
                    return Err(FlightRadarError::Parameter(format!(
                        "Painted As: {}",
                        painted
                    )));
                }
            }
            url.pop();
        }
        if let Some(operating_as) = &params.operating_as {
            url.push_str("&operating_as=");
            for operating in operating_as {
                if operating.chars().all(char::is_alphabetic) && operating.len() == 3 {
                    url.push_str(&operating.to_string());
                    url.push(',');
                } else {
                    return Err(FlightRadarError::Parameter(format!(
                        "Operating As: {}",
                        operating
                    )));
                }
            }
            url.pop();
        }
        if let Some(airports) = &params.airports {
            url.push_str("&airports=");
            for airport in airports {
                if airport
                    .chars()
                    .all(|c| (c.is_alphabetic() && (c != 'i' || c != 'q' || c != 'x')) || c == ':')
                {
                    url.push_str(&airport.to_string());
                    url.push(',');
                } else {
                    return Err(FlightRadarError::Parameter(format!("Airport: {}", airport)));
                }
            }
            url.pop();
        }
        if let Some(routes) = &params.routes {
            url.push_str("&routes=");
            for route in routes {
                if route
                    .chars()
                    .all(|c| (c.is_alphabetic() && (c != 'i' || c != 'q' || c != 'x')) || c == '-')
                {
                    url.push_str(&route.to_string());
                    url.push(',');
                } else {
                    return Err(FlightRadarError::Parameter(format!("Route: {}", route)));
                }
            }
            url.pop();
        }
        if let Some(aircraft) = &params.aircraft {
            url.push_str("&aircraft=");
            for aircraft_iter in aircraft {
                if aircraft_iter
                    .chars()
                    .all(|c| c.is_alphanumeric() || c == '*')
                {
                    if aircraft_iter.chars().filter(|c| c == &'*').count() == 1 {
                        url.push_str(&aircraft_iter.to_string());
                        url.push(',');
                    }
                } else {
                    return Err(FlightRadarError::Parameter(format!(
                        "Aircraft: {}",
                        aircraft_iter
                    )));
                }
            }
            url.pop();
        }
        if let Some(altitude_ranges) = &params.altitude_ranges {
            url.push_str("&altitude_ranges=");
            for altitude_range in altitude_ranges {
                url.push_str(&altitude_range.min.to_string());
                url.push('-');
                url.push_str(&altitude_range.max.to_string());
                url.push(',');
            }
            url.pop();
        }
        if let Some(squawks) = &params.squawks {
            url.push_str("&squawks=");
            for squawk in squawks {
                if squawk <= &7777 {
                    url.push_str(&squawk.to_string());
                    url.push(',');
                } else {
                    return Err(FlightRadarError::Parameter(format!("Squawk: {}", squawk)));
                }
            }
            url.pop();
        }
        if let Some(categories) = &params.categories {
            url.push_str("&categories=");
            for category in categories {
                if "PCMJTHBGDVON".contains(*category) {
                    url.push(*category);
                    url.push(',');
                } else {
                    return Err(FlightRadarError::Parameter(format!(
                        "Category: {}",
                        category
                    )));
                }
            }
            url.pop();
        }
        if let Some(data_sources) = &params.data_sources {
            let valid_data_sources = [
                "ADSB".to_string(),
                "MLAT".to_string(),
                "ESTIMATED".to_string(),
            ];
            url.push_str("&data_sources=");
            for data_source in data_sources {
                if valid_data_sources.contains(data_source) {
                    url.push_str(data_source);
                    url.push(',');
                } else {
                    return Err(FlightRadarError::Parameter(format!(
                        "Data Source: {}",
                        data_source
                    )));
                }
            }
            url.pop();
        }
        if let Some(airspaces) = &params.airspaces {
            //TODO: This doesn't matter for Historic endpoints
            url.push_str("&airspaces=");
            for airspace in airspaces {
                if airspace.chars().all(char::is_alphabetic) {
                    url.push_str(&airspace.to_string());
                    url.push(',');
                } else {
                    return Err(FlightRadarError::Parameter(format!(
                        "Airspace: {}",
                        airspace
                    )));
                }
            }
            url.pop();
        }
        if let Some(gspeed) = &params.gspeed {
            match gspeed {
                ApiRangeEnum::U32(val) => {
                    // Can't be greater than 5000
                    if val > &5000 {
                        return Err(FlightRadarError::Parameter(format!("GSpeed: {}", val)));
                    }
                    url.push_str("&gspeed=");
                    url.push_str(&val.to_string());
                }
                ApiRangeEnum::ApiRange(gspeed) => {
                    // Can't be greater than 5000
                    if gspeed.max > 5000 {
                        return Err(FlightRadarError::Parameter(format!(
                            "GSpeed: {}",
                            gspeed.max
                        )));
                    }
                    url.push_str("&gspeed=");
                    url.push_str(&gspeed.min.to_string());
                    url.push('-');
                    url.push_str(&gspeed.max.to_string());
                }
            }
        };
        if let Some(limit) = &params.limit {
            url.push_str("&limit=");
            url.push_str(&limit.to_string());
        }

        Ok(url)
    }

    /// Check to ensure one API query is provided for endpoint
    /// # Arguments
    ///   * `query_in` - FullLiveFlightQuery to check
    /// # Returns
    ///   A `bool` based on check
    pub fn check_live_parameters(query_in: &FullLiveFlightQuery) -> bool {
        if query_in.aircraft.is_some()
            || query_in.airports.is_some()
            || query_in.airspaces.is_some()
            || query_in.altitude_ranges.is_some()
            || query_in.bounds.is_some()
            || query_in.callsigns.is_some()
            || query_in.categories.is_some()
            || query_in.data_sources.is_some()
            || query_in.flights.is_some()
            || query_in.gspeed.is_some()
            || query_in.operating_as.is_some()
            || query_in.painted_as.is_some()
            || query_in.registrations.is_some()
            || query_in.routes.is_some()
            || query_in.squawks.is_some()
        {
            return true;
        };

        // Needed parameter not found
        false
    }

    pub fn check_historic_parameters(query_in: &FullLiveFlightQuery) -> bool {
        if query_in.aircraft.is_some()
            || query_in.airports.is_some()
            || query_in.altitude_ranges.is_some()
            || query_in.bounds.is_some()
            || query_in.callsigns.is_some()
            || query_in.categories.is_some()
            || query_in.data_sources.is_some()
            || query_in.flights.is_some()
            || query_in.gspeed.is_some()
            || query_in.operating_as.is_some()
            || query_in.painted_as.is_some()
            || query_in.registrations.is_some()
            || query_in.routes.is_some()
            || query_in.squawks.is_some()
        {
            return true;
        };

        // Needed parameter not found
        false
    }

    /// Issue the GET command to API Endpoint
    /// # Arguments
    ///   * `url` - API URL to send GET request to
    /// # Returns
    ///   A `String` on success and `FlightRadarError` on failure.
    pub fn query_endpoint(&self, url: String) -> Result<String, FlightRadarError> {
        let response = self
            .client
            .get(&url)
            .header("Accept-Version", "v1")
            .bearer_auth(&self.api_key)
            .send()?; // synchronous send
        let response_text = response.text()?; // synchronous text retrieval
        Ok(response_text)
    }

    /// Fetches airline information by ICAO.
    /// # Arguments
    ///   * `icao` - The identifier for the airline.
    /// # Returns
    ///   A `Airline` struct on success or a `FlightRadarError` on failure.
    pub fn get_airline_by_icao(&self, icao: &str) -> Result<Airline, FlightRadarError> {
        // Make URL and GET
        let url = format!("{}static/airlines/{}/light", self.base_url, icao);
        let text = match self.query_endpoint(url) {
            Ok(data) => data,
            Err(_) => return Err(FlightRadarError::General("GET Request Failed".to_string())),
        };

        // Parse
        let airline: Airline = serde_json::from_str(&text)
            .map_err(|e| FlightRadarError::Parsing(format!("{}\nResponse: {}", e, text)))?;

        Ok(airline)
    }

    /// Fetches airport information by code.
    /// # Arguments
    ///   * `code` - The identifier for the airport.
    /// # Returns
    ///   A `Airport` struct on success or a `FlightRadarError` on failure.
    pub fn get_airport_by_code(&self, code: &str) -> Result<Airport, FlightRadarError> {
        // Make URL and GET
        let url = format!("{}static/airports/{}/full", self.base_url, code);
        let text = match self.query_endpoint(url) {
            Ok(data) => data,
            Err(_) => return Err(FlightRadarError::General("GET Request Failed".to_string())),
        };

        // Parse
        let airport: Airport = serde_json::from_str(&text)
            .map_err(|e| FlightRadarError::Parsing(format!("{}\nResponse: {}", e, text)))?;

        Ok(airport)
    }

    /// Fetches airport information by code.
    /// # Arguments
    ///   * `code` - The identifier for the airport.
    /// # Returns
    ///   A `Airport` struct on success or a `FlightRadarError` on failure.
    pub fn get_airport_lite_by_code(&self, code: &str) -> Result<AirportLite, FlightRadarError> {
        // Make URL and GET
        let url = format!("{}static/airports/{}/light", self.base_url, code);
        let text = match self.query_endpoint(url) {
            Ok(data) => data,
            Err(_) => return Err(FlightRadarError::General("GET Request Failed".to_string())),
        };

        // Parse
        let airport: AirportLite = serde_json::from_str(&text)
            .map_err(|e| FlightRadarError::Parsing(format!("{}\nResponse: {}", e, text)))?;

        Ok(airport)
    }

    /// Fetches live flight information by location (or other parameters).
    /// # Arguments
    ///   * `other_queries` - Optional parameters to narrow down data
    /// # Returns
    ///   A `FullLiveFlightResponse` struct on success or a `FlightRadarError` on failure.
    pub fn get_live_flight(
        &self,
        other_queries: Option<&FullLiveFlightQuery>,
    ) -> Result<FullLiveFlightResponse, FlightRadarError> {
        // Make Required URL
        let defualt_query_in = &FullLiveFlightQuery::default();
        let other_query_in = match other_queries {
            Some(data) => data,
            _ => defualt_query_in,
        };

        // If parameters not included, bailout
        if !Self::check_live_parameters(other_query_in) {
            return Err(FlightRadarError::Parameter(
                "Missing One Of Required Parameters".to_string(),
            ));
        }

        let params_back = Self::build_query_params(other_query_in)?;
        let endpoint = format!("{}live/flight-positions/full{}", self.base_url, params_back);

        //println!("{}", endpoint);

        // GET
        let text = match self.query_endpoint(endpoint) {
            Ok(data) => data,
            Err(_) => return Err(FlightRadarError::General("GET Request Failed".to_string())),
        };
        //println!("{:?}", text);

        // Parse
        let live_data: FullLiveFlightResponse = serde_json::from_str(&text)
            .map_err(|e| FlightRadarError::Parsing(format!("{}\nResponse: {}", e, text)))?;

        Ok(live_data)
    }

    /// Fetches light live flight information by location (or other parameters).
    /// # Arguments
    ///   * `other_queries` - Optional parameters to narrow down data
    /// # Returns
    ///   A `LightLiveFlightResponse` struct on success or a `FlightRadarError` on failure.
    pub fn get_live_flight_light(
        &self,
        other_queries: Option<&FullLiveFlightQuery>,
    ) -> Result<LightLiveFlightResponse, FlightRadarError> {
        // Make Required URL
        let defualt_query_in = &FullLiveFlightQuery::default();
        let other_query_in = match other_queries {
            Some(data) => data,
            _ => defualt_query_in,
        };

        let params_back = Self::build_query_params(other_query_in)?;
        let endpoint = format!("{}live/flight-positions/full{}", self.base_url, params_back);

        //println!("{}", endpoint);

        // GET
        let text = match self.query_endpoint(endpoint) {
            Ok(data) => data,
            Err(_) => return Err(FlightRadarError::General("GET Request Failed".to_string())),
        };
        //println!("{:?}", text);

        // Parse
        let live_data: LightLiveFlightResponse = serde_json::from_str(&text)
            .map_err(|e| FlightRadarError::Parsing(format!("{}\nResponse: {}", e, text)))?;

        Ok(live_data)
    }

    /// Fetches historic flight information by timestamp (or other parameters).
    /// # Arguments
    ///   * `timestamp` - Timestamp to gather information from
    ///   * `other_queries` - Optional parameters to narrow down data
    /// # Returns
    ///   A `FullLiveFlightResponse` struct on success or a `FlightRadarError` on failure.
    pub fn get_historic_flight(
        &self,
        timestamp: &u64,
        other_queries: Option<&FullLiveFlightQuery>,
    ) -> Result<FullLiveFlightResponse, FlightRadarError> {
        // Check Timestamp
        const MIN_TIMESTAMP: u64 = 1462924800;
        if timestamp < &MIN_TIMESTAMP {
            return Err(FlightRadarError::Parameter(format!(
                "Invalid Timestamp: {}",
                timestamp
            )));
        };

        // If parameters not included, bailout
        let stuff = match other_queries {
            Some(stuff) => stuff,
            None => &FullLiveFlightQuery::default(),
        };

        if !Self::check_historic_parameters(stuff) {
            return Err(FlightRadarError::Parameter(
                "Missing One Of Required Parameters".to_string(),
            ));
        }

        // Build optional params
        let params_back = if let Some(other_queries) = other_queries {
            match Self::build_query_params(other_queries) {
                Ok(param_str) => param_str[1..param_str.len()].to_string(),
                Err(_) => String::from(""),
            }
        } else {
            String::from("")
        };

        let endpoint = format!(
            "{}historic/flight-positions/full?timestamp={}&{}",
            self.base_url, timestamp, params_back
        );

        // GET
        let text = match self.query_endpoint(endpoint) {
            Ok(data) => data,
            Err(_) => return Err(FlightRadarError::General("GET Request Failed".to_string())),
        };
        //println!("{:?}", text);

        // Parse
        let historic_data: FullLiveFlightResponse = serde_json::from_str(&text)
            .map_err(|e| FlightRadarError::Parsing(format!("{}\nResponse: {}", e, text)))?;

        Ok(historic_data)
    }

    /// Fetches light historic flight information by timestamp (or other parameters).
    /// # Arguments
    ///   * `timestamp` - Timestamp to gather information from
    ///   * `other_queries` - Optional parameters to narrow down data
    /// # Returns
    ///   A `LightLiveFlightResponse` struct on success or a `FlightRadarError` on failure.
    pub fn get_historic_flight_light(
        &self,
        timestamp: &u64,
        other_queries: Option<&FullLiveFlightQuery>,
    ) -> Result<LightLiveFlightResponse, FlightRadarError> {
        // Check Timestamp
        const MIN_TIMESTAMP: u64 = 1462924800;
        if timestamp < &MIN_TIMESTAMP {
            return Err(FlightRadarError::Parameter(format!(
                "Invalid Timestamp: {}",
                timestamp
            )));
        };

        // Build optional params
        let params_back = if let Some(other_queries) = other_queries {
            match Self::build_query_params(other_queries) {
                Ok(param_str) => param_str[1..param_str.len()].to_string(),
                Err(_) => String::from(""),
            }
        } else {
            String::from("")
        };

        let endpoint = format!(
            "{}historic/flight-positions/light?timestamp={}&{}",
            self.base_url, timestamp, params_back
        );

        // GET
        let text = match self.query_endpoint(endpoint) {
            Ok(data) => data,
            Err(_) => return Err(FlightRadarError::General("GET Request Failed".to_string())),
        };
        //println!("{:?}", text);

        // Parse
        let historic_data: LightLiveFlightResponse = serde_json::from_str(&text)
            .map_err(|e| FlightRadarError::Parsing(format!("{}\nResponse: {}", e, text)))?;

        Ok(historic_data)
    }

    /// Fetches flight information by flight ID.
    /// # Arguments
    ///   * `flight_id` - The identifier for the flight.
    /// # Returns
    ///   A `Flight` struct on success or a `FlightRadarError` on failure.
    pub fn get_flight_tracks_by_id(
        &self,
        flight_id: &str,
    ) -> Result<Vec<Flight>, FlightRadarError> {
        // If value isn't valid hexadecimal, exit function and raise error
        if u64::from_str_radix(flight_id, 16).is_err() {
            return Err(FlightRadarError::Parameter(format!(
                "Flight ID Not Hexadecimal: {}",
                flight_id
            )));
        }

        // Make URL and GET
        let url = format!("{}flight-tracks?flight_id={}", self.base_url, flight_id);
        let text = match self.query_endpoint(url) {
            Ok(data) => data,
            Err(_) => return Err(FlightRadarError::General("GET Request Failed".to_string())),
        };
        //println!("{:?}", text);

        // Parse
        let flights: Vec<Flight> = serde_json::from_str(&text)
            .map_err(|e| FlightRadarError::Parsing(format!("{}\nResponse: {}", e, text)))?;

        Ok(flights)
    }

    /// Fetches API usage details over period
    /// # Arguments
    ///   * `period` - Backwards time to gather usage (Allowed: 24h | 7d | 30d | 1y)
    /// # Returns
    ///   A `ApiUsageResponse` struct on success or a `FlightRadarError` on failure.
    pub fn get_api_usage(&self, period: &str) -> Result<ApiUsageResponse, FlightRadarError> {
        // If value isn't valid, exit function and raise error
        (match period {
            "24h" | "7d" | "30d" | "1y" => Ok(()),
            _ => Err(FlightRadarError::Parameter(format!(
                "Period: {}. Should be: 24h|7d|30d|1y",
                period
            ))),
        })?;

        // Make URL and GET
        let url = format!("{}usage?period={}", self.base_url, period);
        let text = match self.query_endpoint(url) {
            Ok(data) => data,
            Err(_) => return Err(FlightRadarError::General("GET Request Failed".to_string())),
        };
        //println!("{:?}", text);

        // Parse
        let usage: ApiUsageResponse = serde_json::from_str(&text)
            .map_err(|e| FlightRadarError::Parsing(format!("{}\nResponse: {}", e, text)))?;

        Ok(usage)
    }
}

/// Individual Tracks for flight-tracks endpoint
#[derive(Debug, Deserialize, Default)]
pub struct Track {
    pub timestamp: String,
    pub lat: f64,
    pub lon: f64,
    pub alt: u32,
    pub gspeed: u32,
    pub vspeed: u32,
    pub track: u32,
    pub squawk: String,
    pub callsign: String,
    pub source: String,
}

/// Wrapper struct for flight-tracks endpoint
#[derive(Debug, Deserialize, Default)]
pub struct Flight {
    #[serde(rename = "fr24_id")]
    pub id: String,
    pub tracks: Vec<Track>,
}

/// Wrapper struct of usage endpoint
#[derive(Debug, Deserialize, Default)]
pub struct ApiUsageResponse {
    pub data: Vec<ApiEndpointUsage>,
}

/// Individual endpoint usage data
#[derive(Debug, Deserialize, Default)]
pub struct ApiEndpointUsage {
    pub endpoint: String,
    pub metadata: String,
    pub request_count: u32,
    pub results: u32,
    pub credits: u32,
}

/// Basic Airline stucture
#[derive(Debug, Deserialize, Default)]
pub struct Airline {
    pub name: String,
    pub iata: Option<String>,
    pub icao: String,
}

/// Result airport data from airport/full endpoint
#[derive(Debug, Deserialize, Default)]
pub struct Airport {
    pub name: String,
    pub iata: String,
    pub icao: String,
    pub lon: f64,
    pub lat: f64,
    pub elevation: i32,
    pub country: Country,
    pub city: String,
    pub state: Option<String>,
    pub timezone: Timezone,
}

/// Nested Struct for Country Data
#[derive(Debug, Deserialize, Default)]
pub struct Country {
    pub code: String,
    pub name: String,
}

/// Nested Struct for Timezone Data
#[derive(Debug, Deserialize, Default)]
pub struct Timezone {
    pub name: String,
    pub offset: i32,
}

/// Result airport data from airport/light endpoint
#[derive(Debug, Deserialize, Default)]
pub struct AirportLite {
    pub name: String,
    pub iata: String,
    pub icao: String,
}

/// Represents a query for flight positions.
#[derive(Debug, Deserialize, Default)]
pub struct FullLiveFlightQuery {
    pub bounds: Option<Bounds>,
    pub flights: Option<Vec<String>>,
    pub callsigns: Option<Vec<String>>,
    pub registrations: Option<Vec<String>>,
    pub painted_as: Option<Vec<String>>,
    pub operating_as: Option<Vec<String>>,
    pub airports: Option<Vec<String>>,
    pub routes: Option<Vec<String>>,
    pub aircraft: Option<Vec<String>>,
    pub altitude_ranges: Option<Vec<ApiRange>>,
    pub squawks: Option<Vec<u16>>,
    pub categories: Option<Vec<char>>,
    pub data_sources: Option<Vec<String>>,
    pub airspaces: Option<Vec<String>>,
    pub gspeed: Option<ApiRangeEnum>,
    pub limit: Option<u32>,
}

/// Represents a geographic bounding box.
#[derive(Debug, Deserialize, Default, Serialize)]
pub struct Bounds {
    pub north: f64,
    pub south: f64,
    pub west: f64,
    pub east: f64,
}

/// Represents a numeric range with a minimum and maximum.
#[derive(Debug, Deserialize, Default)]
pub struct ApiRange {
    pub min: u32,
    pub max: u32,
}

/// Allow range to be specific number or range
#[derive(Debug, Deserialize)]
pub enum ApiRangeEnum {
    U32(u32),
    ApiRange(ApiRange),
}

/// Wrapper struct for flight-positions endpoint
#[derive(Deserialize, Debug, Default)]
pub struct FullLiveFlightResponse {
    pub data: Vec<FullLiveFlightData>,
}

/// Data for each flight returned from flight-positions endpoint
#[derive(Deserialize, Debug, Default)]
pub struct FullLiveFlightData {
    pub fr24_id: String,
    pub flight: String,
    pub callsign: String,
    pub lat: f64,
    pub lon: f64,
    pub track: u32,
    pub alt: u32,
    pub gspeed: u32,
    pub vspeed: i32, // Plane can be descending
    pub squawk: String,
    pub timestamp: String,
    pub source: String,
    pub hex: String,
    // `type` is a reserved keyword in Rust so we rename it to `type_field`
    #[serde(rename = "type")]
    pub type_field: String,
    pub reg: String,
    pub painted_as: String,
    pub operating_as: String,
    pub orig_iata: String,
    pub orig_icao: String,
    pub dest_iata: String,
    pub dest_icao: String,
    pub eta: String,
}

/// Data for light flight responses
#[derive(Deserialize, Debug, Default)]
pub struct LightLiveFlightResponse {
    pub data: Vec<LightLiveFlightData>,
}

/// Data for each flight returned from flight-positions endpoint
#[derive(Deserialize, Debug, Default)]
pub struct LightLiveFlightData {
    pub fr24_id: String,
    pub hex: String,
    pub callsign: String,
    pub lat: f64,
    pub lon: f64,
    pub track: u32,
    pub alt: u32,
    pub gspeed: u32,
    pub vspeed: i32, // Plane can be descending
    pub squawk: String,
    pub timestamp: String,
    pub source: String,
}
