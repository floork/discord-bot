use crate::models::Location;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Configs {
    pub locations: Location,
}
