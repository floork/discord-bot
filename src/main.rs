use chrono::prelude::*;
use clap::Parser;
use serde::Deserialize;
use std::fs;
use std::path::Path;

extern crate mensa_cli_backend;

#[allow(dead_code)]
#[derive(Parser, Debug)]
struct Args {
    #[clap(short = 'L', long, default_value = "")]
    location: String,

    #[clap(short, long, default_value = "1")]
    id: u8,

    #[clap(short = 'D', long, default_value = "today")]
    date: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Location {
    cities: Vec<String>,
    coordinates: Option<Vec<Coordinate>>,
    canteens: Vec<u16>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Coordinate {
    city: String,
    latitude: f64,
    longitude: f64,
}

#[derive(Deserialize, Debug)]
struct Configs {
    locations: Location,
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

    let args_date = args.date;

    let date: DateTime<Utc> = match args_date.as_str() {
        "today" => Utc::now(),
        _ => match NaiveDate::parse_from_str(&args_date, "%Y-%m-%d") {
            Ok(naive_date) => {
                if let Some(naive_datetime) = naive_date.and_hms_opt(0, 0, 0) {
                    Utc.from_utc_datetime(&naive_datetime)
                } else {
                    panic!("Invalid date format: {}", args_date);
                }
            }
            Err(_) => {
                panic!("Invalid date format: {}", args_date);
            }
        },
    };

    let date = date.date_naive();

    let ids = &configs.locations.canteens;
    let canteens = match mensa_cli_backend::get_canteens_by_ids(ids.to_vec()).await {
        Ok(canteens) => canteens,
        Err(err) => {
            eprintln!("Error: {}", err);
            return; // Exit the function if there's an error
        }
    };

    let mut all_meals: Vec<mensa_cli_backend::Meal> = Vec::new(); // Vector to store all meals

    for canteen in canteens {
        match mensa_cli_backend::get_meals(&canteen, date).await {
            Ok(meals) => {
                all_meals.extend(meals); // Add meals for this canteen to the vector
            }
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        }
    }

    println!("{:#?}", all_meals);
}
