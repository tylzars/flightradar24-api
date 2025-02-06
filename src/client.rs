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
        // TODO: Make this not work this way and instead work the right way!!!!
        if let Ok(uval) = u64::from_str_radix(flight_id, 16) {
        } else {
            (Err(FlightRadarError::General(
                "period: Invalid Arguement".to_string(),
            )))?
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

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct Flight {
    #[serde(rename = "fr24_id")]
    pub id: String,
    pub tracks: Vec<Track>,
}

#[derive(Debug, Deserialize)]
pub struct ApiUsageResponse {
    pub data: Vec<ApiEndpointUsage>,
}

#[derive(Debug, Deserialize)]
pub struct ApiEndpointUsage {
    pub endpoint: String,
    pub metadata: String,
    pub request_count: u32,
    pub results: u32,
    pub credits: u32,
}
