use serde::Deserialize;
extern crate chrono;
use chrono::prelude::*;
use std::error::Error;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Canteen {
    id: u32,
    name: String,
    city: String,
    address: String,
    coordinates: Option<[f64; 2]>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Prices {
    students: f64,
    employees: f64,
    pupils: Option<f64>,
    others: f64,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Meal {
    id: u32,
    name: String,
    category: String,
    prices: Prices,
    notes: Vec<String>,
}

async fn fetch<T: for<'de> Deserialize<'de>>(url: &str) -> Result<T, Box<dyn Error>> {
    let response = reqwest::get(url).await?;
    if !response.status().is_success() {
        return Err(format!("Failed to get response: {}", response.status()).into());
    }
    let response_text = response.text().await?;
    let result: T = serde_json::from_str(&response_text)?;
    Ok(result)
}

fn get_dresden_canteens(canteens: Vec<Canteen>) -> Vec<Canteen> {
    let mut dresden = vec![];
    for canteen in canteens {
        if canteen.city == "Dresden" {
            let name_parts: Vec<&str> = canteen.name.split(',').collect();
            let name = name_parts.get(1).unwrap_or(&"").to_owned();
            dresden.push(Canteen {
                id: canteen.id,
                name: name.to_string(),
                city: canteen.city.clone(),
                address: canteen.address.clone(),
                coordinates: canteen.coordinates,
            });
        }
    }
    dresden
}

async fn get_menu(canteen: &Canteen, date: NaiveDate) -> Vec<Meal> {
    let canteen_id = canteen.id;
    let menu_url = format!(
        "https://openmensa.org/api/v2/canteens/{}/days/{}/meals",
        canteen_id, date
    );

    match fetch(&menu_url).await {
        Ok(menus) => menus,
        Err(err) => {
            println!("ERROR: {}", err);
            Vec::new() // Return an empty vector or handle error differently
        }
    }
}

pub async fn list_dresden_canteens() {
    let canteens_url = "https://openmensa.org/api/v2/canteens/";
    let dresden_canteens = match fetch::<Vec<Canteen>>(canteens_url).await {
        Ok(all_canteens) => get_dresden_canteens(all_canteens),
        Err(err) => {
            println!("ERROR: {}", err);
            Vec::new() // Return an empty vector or handle error differently
        }
    };

    for canteen in dresden_canteens {
        let date = Local::now().date_naive();
        for menu in get_menu(&canteen, date).await {
            println!("{:?}", menu);
        }
    }
}

pub async fn main() -> Result<(), Box<dyn Error>> {
    list_dresden_canteens().await;
    Ok(())
}
