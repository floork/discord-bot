use reqwest::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UselessFact {
    pub id: String,
    pub text: String,
    pub source: String,
    pub source_url: String,
    pub language: String,
    pub permalink: String,
}

pub async fn daily(language: Option<String>) -> Result<UselessFact, Error> {
    let lang = if let Some(language) = language {
        language
    } else {
        String::from("en")
    };

    let response = reqwest::get(format!(
        "https://uselessfacts.jsph.pl/api/v2/facts/today?language={}",
        lang
    ))
    .await?
    .json::<UselessFact>()
    .await?;

    Ok(response)
}

pub async fn random(language: Option<String>) -> Result<UselessFact, Error> {
    let lang = if let Some(language) = language {
        language
    } else {
        String::from("en")
    };

    let response = reqwest::get(format!(
        "https://uselessfacts.jsph.pl/api/v2/facts/random?language={}",
        lang
    ))
    .await?
    .json::<UselessFact>()
    .await?;

    Ok(response)
}
