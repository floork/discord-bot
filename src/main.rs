use chrono::prelude::*;
use clap::Parser;
use serde::Deserialize;
use std::fs;
use std::path::Path;
use tabled::{
    settings::{object::Columns, Modify, Style, Width},
    Table, Tabled,
};

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
    canteens: Vec<u32>,
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

// Wrapper struct for Meal to derive Tabled
#[derive(Tabled)]
struct TabledMeal {
    id: u64,
    name: String,
    // category: String,
    student_price: f64,
    employee_price: f64,
    guest_price: f64,
    notes: String,
}

// Function to convert Meal to TabledMeal
impl From<mensa_cli_backend::Meal> for TabledMeal {
    fn from(meal: mensa_cli_backend::Meal) -> Self {
        TabledMeal {
            id: meal.id,
            name: meal.name,
            // category: meal.category,
            student_price: meal.prices.students.unwrap_or(0.0),
            employee_price: meal.prices.employees.unwrap_or(0.0),
            guest_price: meal.prices.pupils.unwrap_or(0.0),
            notes: meal.notes.join(", "),
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

    for canteen in canteens {
        match mensa_cli_backend::get_meals(&canteen, &date.to_string()).await {
            Ok(meals) => {
                // Convert all_meals to TabledMeal
                let tabled_meals: Vec<TabledMeal> =
                    meals.into_iter().map(TabledMeal::from).collect();

                // Print meals as a table
                let mut table = Table::new(&tabled_meals);
                table
                    .with(Style::modern())
                    .with(Modify::new(Columns::single(1)).with(Width::wrap(10).keep_words()))
                    .with(Modify::new(Columns::last()).with(Width::wrap(10).keep_words()));

                println!("{}", canteen.name);
                println!("{}", table);
            }
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        }
    }
}
