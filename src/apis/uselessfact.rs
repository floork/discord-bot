use reqwest::Error;
use serde::Deserialize;

/// Represents a useless fact fetched from the API.
#[derive(Debug, Deserialize)]
pub struct UselessFact {
    /// The unique identifier of the fact.
    pub id: String,
    /// The text content of the useless fact.
    pub text: String,
    /// The source of the fact.
    pub source: String,
    /// The URL of the source.
    pub source_url: String,
    /// The language in which the fact is presented.
    pub language: String,
    /// The permalink to the fact.
    pub permalink: String,
}

/// Fetches a daily useless fact from the API.
///
/// # Arguments
///
/// * `language` - Optional language code to specify the language of the fact (default is "en" for English).
///
/// # Errors
///
/// Returns a `reqwest::Error` if the HTTP request fails or if deserialization of the response fails.
///
/// # Returns
///
/// Returns a `Result` containing a `UselessFact` struct if successful, or a `reqwest::Error` if an error occurs.
pub async fn daily(language: Option<String>) -> Result<UselessFact, Error> {
    let lang = language.unwrap_or_else(|| String::from("en"));

    let response = reqwest::get(format!(
        "https://uselessfacts.jsph.pl/api/v2/facts/today?language={}",
        lang
    ))
    .await?
    .json::<UselessFact>()
    .await?;

    Ok(response)
}

/// Fetches a random useless fact from the API.
///
/// # Arguments
///
/// * `language` - Optional language code to specify the language of the fact (default is "en" for English).
///
/// # Errors
///
/// Returns a `reqwest::Error` if the HTTP request fails or if deserialization of the response fails.
///
/// # Returns
///
/// Returns a `Result` containing a `UselessFact` struct if successful, or a `reqwest::Error` if an error occurs.
pub async fn random(language: Option<String>) -> Result<UselessFact, Error> {
    let lang = language.unwrap_or_else(|| String::from("en"));

    let response = reqwest::get(format!(
        "https://uselessfacts.jsph.pl/api/v2/facts/random?language={}",
        lang
    ))
    .await?
    .json::<UselessFact>()
    .await?;

    Ok(response)
}
