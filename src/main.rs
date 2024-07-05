use chrono::{NaiveDate, Utc};
use clap::Parser;
use dotenv::dotenv;
use std::fs;
use std::path::Path;

mod args;
mod bot;
mod cli;
mod config;
mod models;

use args::Args;
use cli::print_meals;
use config::Configs;

extern crate openmensa_rust_interface;
use openmensa_rust_interface::{
    get_canteen_by_id, get_canteens_by_ids, get_canteens_by_location, Canteen,
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
        return match get_canteen_by_id(id).await {
            Ok(Some(canteen)) => Some(vec![canteen]), // Wrap the Canteen in a Vec
            Ok(None) => {
                eprintln!("Canteen not found by ID");
                None
            }
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
    let config_path = "~/.config/discord-bot/config.toml";
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

    if args.discord_bot {
        if let Some(token) = args.token {
            bot::start_bot(&token).await;
            return;
        }

        let path = Path::new(".env");
        if path.exists() {
            dotenv().ok();
            let token = std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN must be set.");
            bot::start_bot(&token).await;
            return;
        }

        eprintln!("Please provide a Discord Token either as a parameter or in a .env file");
        return;
    }

    if args.id.is_some() && args.location.is_some() {
        eprintln!("Use either location or id");
        return;
    }

    let canteens = match fetch_canteens(&args, &configs).await {
        Some(canteens) => canteens,
        None => return,
    };

    let date = match parse_date(&args.date) {
        Ok(date) => date,
        Err(err) => {
            eprintln!("Error parsing date: {}", err);
            return;
        }
    };

    if let Err(err) = print_meals(canteens, date).await {
        eprintln!("Error printing meals: {}", err);
    }
}
