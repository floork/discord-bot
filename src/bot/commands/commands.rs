use chrono::{NaiveDate, Utc};
use futures::future;
use futures::stream::{self, Stream, StreamExt};
use openmensa_rust_interface::{get_all_canteens, get_canteen_by_name, get_meals};
use poise::CreateReply;
use serenity::builder::CreateEmbed;
use std::{collections::HashMap, sync::Mutex};

use crate::apis::{meme_api, uselessfact};

/// Struct holding shared data for bot commands.
pub struct Data {
    pub votes: Mutex<HashMap<String, u32>>,
}

/// Alias for the error type used in this module.
pub type Error = Box<dyn std::error::Error + Send + Sync>;

/// Alias for the context type used in this module.
pub type Context<'a> = poise::Context<'a, Data, Error>;

/// Streams names of all canteens matching a partial name.
///
/// # Arguments
///
/// * `_ctx` - The context for executing the command.
/// * `partial` - A partial name used to filter canteens.
///
/// # Returns
///
/// A stream of canteen names matching the partial name.
async fn all_canteens<'a>(_ctx: Context<'_>, partial: &'a str) -> impl Stream<Item = String> + 'a {
    match get_all_canteens().await {
        Ok(canteens) => {
            let filtered_stream = stream::iter(canteens)
                .filter(move |canteen| future::ready(canteen.name.contains(partial)))
                .map(|canteen| canteen.name);
            futures::future::Either::Left(filtered_stream)
        }
        Err(_) => {
            let empty_stream = stream::empty();
            futures::future::Either::Right(empty_stream)
        }
    }
}

/// Checks if a given string is a valid date in the format `%Y-%m-%d`.
///
/// # Arguments
///
/// * `date_str` - The date string to validate.
///
/// # Returns
///
/// `true` if the date string is valid, otherwise `false`.
fn is_valid_date(date_str: &str) -> bool {
    if let Ok(_) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        true
    } else {
        false
    }
}

/// Retrieves the date string for querying meals, defaults to today's date if invalid.
///
/// # Arguments
///
/// * `date` - Optional date string provided by the user.
///
/// # Returns
///
/// The validated date string to use for querying meals.
fn get_date(date: Option<String>) -> String {
    let date_now = Utc::now().date_naive();
    let date_str;

    if let Some(date) = date {
        if is_valid_date(&date) {
            return date;
        }
        eprintln!("Invalid date entered: {}", date);
        return date_now.to_string();
    }

    date_str = date_now.to_string();
    date_str
}

/// Command to fetch and display meals from a specified canteen on a given date.
///
/// # Arguments
///
/// * `ctx` - The context for executing the command.
/// * `canteen` - The name of the canteen from which to fetch meals.
/// * `date` - Optional date string in `%Y-%m-%d` format for fetching meals.
///
/// # Returns
///
/// A result indicating success (`Ok`) or an error (`Err`) if fetching or sending fails.
#[poise::command(slash_command)]
pub async fn meal(
    ctx: Context<'_>,
    #[description = "choose a canteen"]
    #[autocomplete = "all_canteens"]
    canteen: String,
    #[description = "Choose a date"] date: Option<String>,
) -> Result<(), Error> {
    let date_str = get_date(date);

    // Ensure canteen exists
    let meals = match get_canteen_by_name(&canteen).await {
        Ok(Some(can)) => get_meals(&can, &date_str).await?,
        Ok(None) => {
            eprintln!("Canteen not found: {}", canteen);
            ctx.say("Canteen not found.").await?;
            return Ok(());
        }
        Err(err) => {
            eprintln!("Error fetching canteen by name: {:?}", err);
            ctx.say("Failed to fetch canteen.").await?;
            return Ok(());
        }
    };

    if meals.is_empty() {
        ctx.say("No meals found for the selected canteen.").await?;
        return Ok(());
    }

    // Create reply with all embeds
    let mut reply = CreateReply::default();
    for meal in &meals {
        let price_info = format!(
            "Students: {}\nEmployees: {}\nPupils: {}\nOthers: {}",
            meal.prices.students.unwrap_or_default(),
            meal.prices.employees.unwrap_or_default(),
            meal.prices.pupils.unwrap_or_default(),
            meal.prices.others.unwrap_or_default(),
        );

        let notes = meal.notes.join(", ");

        let embed = CreateEmbed::new().title(&meal.name).field(
            &format!("Category: {}", meal.category),
            &format!("Prices:\n{}\nNotes: {}", price_info, notes),
            false,
        );

        reply = reply.embed(embed);
    }

    // Send reply using poise context
    ctx.send(reply).await?;

    Ok(())
}

/// Command to fetch and display a meme.
///
/// # Arguments
///
/// * `ctx` - The context for executing the command.
///
/// # Returns
///
/// A result indicating success (`Ok`) or an error (`Err`) if fetching or sending fails.
#[poise::command(slash_command)]
pub async fn meme(ctx: Context<'_>) -> Result<(), Error> {
    match meme_api::get().await {
        Ok(meme) => {
            ctx.say(meme.url).await?;
            Ok(())
        }
        Err(err) => {
            eprintln!("Error fetching meme: {:?}", err);
            ctx.say("Failed to fetch meme.").await?;
            Err(err.into())
        }
    }
}

/// Command to fetch and display a daily useless fact.
///
/// # Arguments
///
/// * `ctx` - The context for executing the command.
///
/// # Returns
///
/// A result indicating success (`Ok`) or an error (`Err`) if fetching or sending fails.
#[poise::command(slash_command)]
pub async fn daily_fact(ctx: Context<'_>) -> Result<(), Error> {
    match uselessfact::daily(Some(String::from("de"))).await {
        Ok(fact) => {
            ctx.say(fact.text).await?;
            Ok(())
        }
        Err(err) => {
            eprintln!("Error fetching daily fact: {:?}", err);
            ctx.say("Failed to fetch daily fact.").await?;
            Err(err.into())
        }
    }
}

/// Command to fetch and display a random useless fact.
///
/// # Arguments
///
/// * `ctx` - The context for executing the command.
///
/// # Returns
///
/// A result indicating success (`Ok`) or an error (`Err`) if fetching or sending fails.
#[poise::command(slash_command)]
pub async fn random_fact(ctx: Context<'_>) -> Result<(), Error> {
    match uselessfact::random(Some(String::from("de"))).await {
        Ok(fact) => {
            ctx.say(fact.text).await?;
            Ok(())
        }
        Err(err) => {
            eprintln!("Error fetching random fact: {:?}", err);
            ctx.say("Failed to fetch random fact.").await?;
            Err(err.into())
        }
    }
}

/// Command to show the origin of the Bot
///
/// # Arguments
///
/// * `ctx` - The context for executing the command.
///
/// # Returns
///
/// A result indicating success (`Ok`) or an error (`Err`) if fetching or sending fails.
#[poise::command(slash_command)]
pub async fn bot(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("https://github.com/floork/discord-bot.git").await?;
    Ok(())
}
