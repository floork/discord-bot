use crate::models::TabledMeal;
use chrono::NaiveDate;
use openmensa_rust_interface::Canteen;
use tabled::{
    settings::{object::Columns, Modify, Style, Width},
    Table,
};

pub async fn print_meals(canteens: Vec<Canteen>, date: NaiveDate) -> Result<(), String> {
    for canteen in canteens {
        match get_meals_for_canteen(&canteen, &date).await {
            Ok(tabled_meals) => {
                println!("{}", canteen.name);
                print_table(&tabled_meals);
            }
            Err(err) => {
                return Err(format!(
                    "Error fetching meals for {}: {}",
                    canteen.name, err
                ));
            }
        }
    }
    Ok(())
}

async fn get_meals_for_canteen(
    canteen: &Canteen,
    date: &NaiveDate,
) -> Result<Vec<TabledMeal>, String> {
    let meals = openmensa_rust_interface::get_meals(canteen, &date.to_string())
        .await
        .map_err(|e| e.to_string())?;
    let tabled_meals: Vec<TabledMeal> = meals.into_iter().map(TabledMeal::from).collect();
    Ok(tabled_meals)
}

fn print_table(tabled_meals: &[TabledMeal]) {
    let mut table = Table::new(tabled_meals);
    table
        .with(Style::modern())
        .with(Modify::new(Columns::first()).with(Width::wrap(10).keep_words()))
        .with(Modify::new(Columns::last()).with(Width::wrap(10).keep_words()));

    println!("{}", table);
}
