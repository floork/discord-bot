use crate::models::Location;
use serde::Deserialize;

/// Struct representing configurations loaded from a file.
#[derive(Deserialize, Debug)]
pub struct Configs {
    /// Configuration for locations.
    pub locations: Location,
}
