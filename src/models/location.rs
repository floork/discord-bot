use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Location {
    pub cities: Vec<String>,
    pub coordinates: Option<Vec<Coordinate>>,
    pub canteens: Vec<u32>,
}

#[derive(Deserialize, Debug)]
pub struct Coordinate {
    pub city: String,
    pub latitude: f64,
    pub longitude: f64,
}
