use chrono::{NaiveDate, Utc};
use clap::Parser;
extern crate mensa_cli_backend;

mod args;
mod cli;
mod config;
mod models;

use args::Args;
use cli::print_meals;
use config::Configs;

extern crate mensa_cli_backend;
use mensa_cli_backend::{
    get_canteens_by_id, get_canteens_by_ids, get_canteens_by_location, Canteen,
};

fn parse_date(date_str: &str) -> Result<NaiveDate, String> {
    match date_str {
        "today" => Ok(Utc::now().date_naive()),
        _ => NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
            .map_err(|err| format!("Invalid date format: {}", err)),
    }
}

async fn fetch_canteens(args: &Args, configs: &Configs) -> Option<Vec<Canteen>> {
    if let Some(id) = args.id {
        return match get_canteens_by_id(id).await {
            Ok(canteens) => Some(canteens),
            Err(err) => {
                eprintln!("Error fetching canteens by ID: {}", err);
                None
            }
        };
    }

    if let Some(location_str) = args.location.as_deref() {
        return match get_canteens_by_location(location_str).await {
            Ok(canteens) => Some(canteens),
            Err(err) => {
                eprintln!("Error fetching canteens by location: {}", err);
                None
            }
        };
    }

    match get_canteens_by_ids(configs.locations.canteens.to_vec()).await {
        Ok(canteens) => Some(canteens),
        Err(err) => {
            eprintln!("Error fetching canteens by IDs: {}", err);
            None
        }
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let config_path = "~/.config/mensa-cli/config.toml";
    let expanded_path = shellexpand::tilde(config_path).into_owned();
    let configs_file = match fs::read_to_string(Path::new(&expanded_path)) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Error reading config file: {}", err);
            return;
        }
    };

    // Parse the TOML content
    let configs: Configs = match toml::from_str(&configs_file) {
        Ok(parsed) => parsed,
        Err(err) => {
            eprintln!("Failed to parse the TOML: {}", err);
            return;
        }
    };

    if let Err(err) = mensa_cli_backend::main().await {
        eprintln!("Error: {}", err);
    }
}
