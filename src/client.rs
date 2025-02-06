use crate::error::FlightRadarError;
use reqwest::Client;
use serde::Deserialize;

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
            base_url: " https://fr24api.flightradar24.com/api/sandbox/".to_string(),
            api_key,
        }
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
            return Err(FlightRadarError::General(
                "period: Invalid Argument".to_string(),
            ));
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
            _ => Err(FlightRadarError::General(
                "period: Invalid Arguement".to_string(),
            )),
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

#[derive(Debug, Deserialize, Default)]
pub struct Flight {
    #[serde(rename = "fr24_id")]
    pub id: String,
    pub tracks: Vec<Track>,
}

#[derive(Debug, Deserialize, Default)]
pub struct ApiUsageResponse {
    pub data: Vec<ApiEndpointUsage>,
}

#[derive(Debug, Deserialize, Default)]
pub struct ApiEndpointUsage {
    pub endpoint: String,
    pub metadata: String,
    pub request_count: u32,
    pub results: u32,
    pub credits: u32,
}

#[derive(Debug, Deserialize, Default)]
pub struct Airline {
    pub name: String,
    pub iata: Option<String>,
    pub icao: String,
}

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

#[derive(Debug, Deserialize, Default)]
pub struct Country {
    pub code: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct Timezone {
    pub name: String,
    pub offset: i32,
}

#[derive(Debug, Deserialize, Default)]
pub struct AirportLite {
    pub name: String,
    pub iata: String,
    pub icao: String,
}
