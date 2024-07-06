use serde::Deserialize;

/// Represents a location with cities, coordinates, and associated canteens.
#[derive(Deserialize, Debug)]
pub struct Location {
    /// List of cities associated with this location.
    pub cities: Vec<String>,
    /// Optional coordinates (latitude and longitude) associated with this location.
    pub coordinates: Option<Vec<Coordinate>>,
    /// List of canteen IDs associated with this location.
    pub canteens: Vec<u32>,
}

/// Represents geographical coordinates (latitude and longitude) of a city.
#[derive(Deserialize, Debug)]
pub struct Coordinate {
    /// The name of the city.
    pub city: String,
    /// The latitude coordinate of the city.
    pub latitude: f64,
    /// The longitude coordinate of the city.
    pub longitude: f64,
}
