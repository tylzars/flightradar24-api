use crate::error::FlightRadarError;
use reqwest::Client;
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
            base_url: "https://fr24api.flightradar24.com/api/sandbox/".to_string(),
            api_key,
        }
    }

    fn build_query_params(
        other_query_in: &FullLiveFlightQuery,
    ) -> Result<String, FlightRadarError> {
        let mut url: String = String::new();

        if !other_query_in.flights.is_empty() {
            url.push_str("&flights=");
            for flight in &other_query_in.flights {
                // The documentation doesn't really give use bounds for what this can be...
                if flight.chars().all(char::is_alphanumeric) && flight.len() > 2 {
                    url.push_str(&flight.to_string());
                    url.push(',');
                } else {
                    return Err(FlightRadarError::Parameter(format!("Flight #: {}", flight)));
                }
            }
            url.pop();
        }
        if !other_query_in.callsigns.is_empty() {
            url.push_str("&callsigns=");
            for callsign in &other_query_in.callsigns {
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
        if !other_query_in.registrations.is_empty() {
            url.push_str("&registrations=");
            for registration in &other_query_in.registrations {
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
        if !other_query_in.painted_as.is_empty() {
            url.push_str("&painted_as=");
            for painted in &other_query_in.painted_as {
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
        if !other_query_in.operating_as.is_empty() {
            url.push_str("&operating_as=");
            for operating in &other_query_in.operating_as {
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
        if !other_query_in.airports.is_empty() {
            url.push_str("&airports=");
            for airport in &other_query_in.airports {
                //TODO: Accurate check ICAO codes
                if airport.chars().all(|c| c.is_alphabetic() || c == ':') {
                    url.push_str(&airport.to_string());
                    url.push(',');
                } else {
                    return Err(FlightRadarError::Parameter(format!("Airport: {}", airport)));
                }
            }
            url.pop();
        }
        if !other_query_in.routes.is_empty() {
            url.push_str("&routes=");
            for route in &other_query_in.routes {
                //TODO: Accurate check ICAO codes
                if route.chars().all(|c| c.is_alphabetic() || c == '-') {
                    url.push_str(&route.to_string());
                    url.push(',');
                } else {
                    return Err(FlightRadarError::Parameter(format!("Route: {}", route)));
                }
            }
            url.pop();
        }
        if !other_query_in.aircraft.is_empty() {
            url.push_str("&aircraft=");
            for aircraft_iter in &other_query_in.aircraft {
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
        if !other_query_in.altitude_ranges.is_empty() {
            url.push_str("&altitude_ranges=");
            for altitude_range in &other_query_in.altitude_ranges {
                url.push_str(&altitude_range.min.to_string());
                url.push('-');
                url.push_str(&altitude_range.max.to_string());
                url.push(',');
            }
            url.pop();
        }
        if !other_query_in.squawks.is_empty() {
            url.push_str("&squawks=");
            for squawk in &other_query_in.squawks {
                if squawk <= &7777 {
                    url.push_str(&squawk.to_string());
                    url.push(',');
                } else {
                    return Err(FlightRadarError::Parameter(format!("Squawk: {}", squawk)));
                }
            }
            url.pop();
        }
        if !other_query_in.categories.is_empty() {
            url.push_str("&categories=");
            for category in &other_query_in.categories {
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
        if !other_query_in.data_sources.is_empty() {
            let valid_data_sources = [
                "ADSB".to_string(),
                "MLAT".to_string(),
                "ESTIMATED".to_string(),
            ];
            url.push_str("&data_sources=");
            for data_source in &other_query_in.data_sources {
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
        if !other_query_in.airspaces.is_empty() {
            url.push_str("&airspaces=");
            for airspace in &other_query_in.airspaces {
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
        if other_query_in.gspeed.max != 0 && !other_query_in.gspeed.min != 0 {
            //TODO: switch this to be an option to take in a single number instead of range (Accomplish with an enum?)
            url.push_str("&gspeed=");
            url.push_str(&other_query_in.gspeed.min.to_string());
            url.push('-');
            url.push_str(&other_query_in.gspeed.max.to_string());
        }
        if other_query_in.limit != 0 && other_query_in.limit <= 30000 {
            url.push_str("&limit=");
            url.push_str(&other_query_in.limit.to_string());
        }

        Ok(url)
    }

    /// Fetches airline information by ICAO.
    /// # Arguments
    ///   * `icao` - The identifier for the airline.
    /// # Returns
    ///   A `Airline` struct on success or a `FlightRadarError` on failure.
    pub async fn get_airline_by_icao(&self, icao: &str) -> Result<Airline, FlightRadarError> {
        // Make URL and GET
        let url = format!("{}static/airlines/{}/light", self.base_url, icao);
        let response = self
            .client
            .get(&url)
            .header("Accept-Version", "v1") // Add "Accept-Version: v1"
            .bearer_auth(&self.api_key) // Add "Authorization: Bearer <API_KEY>"
            .send()
            .await?;

        // Parse
        let text = response.text().await?;
        let airline: Airline =
            serde_json::from_str(&text).map_err(|e| FlightRadarError::Parsing(e.to_string()))?;

        Ok(airline)
    }

    /// Fetches airport information by code.
    /// # Arguments
    ///   * `code` - The identifier for the airport.
    /// # Returns
    ///   A `Airport` struct on success or a `FlightRadarError` on failure.
    pub async fn get_airport_by_code(&self, code: &str) -> Result<Airport, FlightRadarError> {
        // Make URL and GET
        let url = format!("{}static/airports/{}/full", self.base_url, code);
        let response = self
            .client
            .get(&url)
            .header("Accept-Version", "v1") // Add "Accept-Version: v1"
            .bearer_auth(&self.api_key) // Add "Authorization: Bearer <API_KEY>"
            .send()
            .await?;

        // Parse
        let text = response.text().await?;
        let airport: Airport =
            serde_json::from_str(&text).map_err(|e| FlightRadarError::Parsing(e.to_string()))?;

        Ok(airport)
    }

    /// Fetches airport information by code.
    /// # Arguments
    ///   * `code` - The identifier for the airport.
    /// # Returns
    ///   A `Airport` struct on success or a `FlightRadarError` on failure.
    pub async fn get_airport_lite_by_code(
        &self,
        code: &str,
    ) -> Result<AirportLite, FlightRadarError> {
        // Make URL and GET
        let url = format!("{}static/airports/{}/light", self.base_url, code);
        let response = self
            .client
            .get(&url)
            .header("Accept-Version", "v1") // Add "Accept-Version: v1"
            .bearer_auth(&self.api_key) // Add "Authorization: Bearer <API_KEY>"
            .send()
            .await?;

        // Parse
        let text = response.text().await?;
        let airport: AirportLite =
            serde_json::from_str(&text).map_err(|e| FlightRadarError::Parsing(e.to_string()))?;

        Ok(airport)
    }

    /// Fetches live flight information by location (or other parameters).
    /// # Arguments
    ///   * `bounds` - The bounds of an area to get live information
    /// # Returns
    ///   A `FullLiveFlightResponse` struct on success or a `FlightRadarError` on failure.
    pub async fn get_live_flight(
        &self,
        bounds: &Bounds,
        other_queries: Option<&FullLiveFlightQuery>,
    ) -> Result<FullLiveFlightResponse, FlightRadarError> {
        // Make Required URL
        let bounds_str = format!(
            "?bounds={},{},{},{}",
            bounds.north, bounds.south, bounds.west, bounds.east
        );

        // Add optional queries
        let defualt_query_in = &FullLiveFlightQuery::default();
        let other_query_in = match other_queries {
            Some(data) => data,
            _ => &defualt_query_in,
        };

        let params_back = Self::build_query_params(other_query_in)?;
        let endpoint = format!(
            "{}live/flight-positions/full{}{}",
            self.base_url, bounds_str, params_back
        );

        //println!("{}", endpoint);

        // GET
        let response = self
            .client
            .get(&endpoint)
            .header("Accept-Version", "v1") // Add "Accept-Version: v1"
            .bearer_auth(&self.api_key) // Add "Authorization: Bearer <API_KEY>"
            .send()
            .await?;

        // Parse
        let text = response.text().await?;
        //println!("{:?}", text);
        let live_data: FullLiveFlightResponse =
            serde_json::from_str(&text).map_err(|e| FlightRadarError::Parsing(e.to_string()))?;

        Ok(live_data)
    }

    /// Fetches flight information by flight ID.
    /// # Arguments
    ///   * `flight_id` - The identifier for the flight.
    /// # Returns
    ///   A `Flight` struct on success or a `FlightRadarError` on failure.
    pub async fn get_flight_tracks_by_id(
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
        let response = self
            .client
            .get(&url)
            .header("Accept-Version", "v1") // Add "Accept-Version: v1"
            .bearer_auth(&self.api_key) // Add "Authorization: Bearer <API_KEY>"
            .send()
            .await?;

        // Parse
        let text = response.text().await?;
        let flights: Vec<Flight> =
            serde_json::from_str(&text).map_err(|e| FlightRadarError::Parsing(e.to_string()))?;

        Ok(flights)
    }

    /// Fetches API usage details over period
    /// # Arguments
    ///   * `period` - Backwards time to gather usage (Allowed: 24h | 7d | 30d | 1y)
    /// # Returns
    ///   A `ApiUsageResponse` struct on success or a `FlightRadarError` on failure.
    pub async fn get_api_usage(&self, period: &str) -> Result<ApiUsageResponse, FlightRadarError> {
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
        let response = self
            .client
            .get(&url)
            .header("Accept-Version", "v1") // Add "Accept-Version: v1"
            .bearer_auth(&self.api_key) // Add "Authorization: Bearer <API_KEY>"
            .send()
            .await?;

        // Parse
        let text = response.text().await?;
        let usage: ApiUsageResponse =
            serde_json::from_str(&text).map_err(|e| FlightRadarError::Parsing(e.to_string()))?;

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
    pub flights: Vec<String>,
    pub callsigns: Vec<String>,
    pub registrations: Vec<String>,
    pub painted_as: Vec<String>,
    pub operating_as: Vec<String>,
    pub airports: Vec<String>,
    pub routes: Vec<String>,
    pub aircraft: Vec<String>,
    pub altitude_ranges: Vec<ApiRange>,
    pub squawks: Vec<u16>,
    pub categories: Vec<char>,
    pub data_sources: Vec<String>,
    pub airspaces: Vec<String>,
    pub gspeed: ApiRange,
    pub limit: u32,
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
    pub vspeed: u32,
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
