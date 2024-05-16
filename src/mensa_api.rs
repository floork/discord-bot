use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Canteen {
    id: u32,
    name: String,
    city: String,
    address: String,
    coordinates: Option<[f64; 2]>,
}

async fn fetch_canteens() -> Result<Vec<Canteen>, Box<dyn std::error::Error>> {
    let request_url = "https://openmensa.org/api/v2/canteens/";
    let response = reqwest::get(request_url).await?;

    if !response.status().is_success() {
        return Err(format!("Failed to get response: {}", response.status()).into());
    }

    // Buffer the response into a string
    let response_text = response.text().await?;

    // Deserialize the response from the buffered string
    let canteens: Vec<Canteen> = serde_json::from_str(&response_text)?;

    Ok(canteens)
}

fn get_dresden_canteens(canteens: Vec<Canteen>) -> Vec<Canteen> {
    let mut dresden = vec![];
    for canteen in canteens {
        if canteen.city == "Dresden" {
            let name_parts: Vec<&str> = canteen.name.split(",").collect();
            let name = name_parts.get(1).unwrap_or(&"").to_owned();

            dresden.push(Canteen {
                id: canteen.id,
                name: name.to_string(),
                city: canteen.city,
                address: canteen.address,
                coordinates: canteen.coordinates,
            });
        }
    }

    dresden
}

pub async fn list_dresden_canteens() {
    let dresden_canteens = match fetch_canteens().await {
        Ok(all_canteens) => get_dresden_canteens(all_canteens),
        Err(err) => {
            println!("ERROR: {}", err);
            Vec::new() // Return an empty vector or handle error differently
        }
    };

    println!("Canteens in Dresden:");
    for canteen in dresden_canteens {
        println!("{}", canteen.name);
    }
}

pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    list_dresden_canteens().await;
    Ok(())
}
