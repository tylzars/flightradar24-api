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
        // Construct the URL (adjust the endpoint as per the actual API documentation)
        let url = format!("{}flight-tracks?flight_id={}", self.base_url, flight_id);

        // Send the GET request with Bearer authentication.
        let response = self
            .client
            .get(&url)
            .header("Accept-Version", "v1") // Add "Accept-Version: v1"
            .bearer_auth(&self.api_key) // Add "Authorization: Bearer <API_KEY>"
            .send()
            .await?;

        let text = response.text().await?;

        //println!("{:?}", text);

        // Parse the JSON response as an array of flights.
        let flights: Vec<Flight> =
            serde_json::from_str(&text).map_err(|e| FlightRadarError::Parsing(e.to_string()))?;

        Ok(flights)
    }

    pub async fn get_api_usage(&self) -> Result<ApiUsageResponse, FlightRadarError> {
        // Construct the URL (adjust the endpoint as per the actual API documentation)
        let url = format!("{}usage", self.base_url);

        // Send the GET request with Bearer authentication.
        let response = self
            .client
            .get(&url)
            .header("Accept-Version", "v1") // Add "Accept-Version: v1"
            .bearer_auth(&self.api_key) // Add "Authorization: Bearer <API_KEY>"
            .send()
            .await?;

        let text = response.text().await?;

        //println!("{:?}", text);

        // Parse the JSON response as an array of flights.
        let usage: ApiUsageResponse =
            serde_json::from_str(&text).map_err(|e| FlightRadarError::Parsing(e.to_string()))?;

        Ok(usage)
    }
}

#[derive(Debug, Deserialize)]
pub struct Track {
    pub timestamp: String, // Possibly use Chrono here for time??
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
