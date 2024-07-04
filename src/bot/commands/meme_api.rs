use reqwest::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Meme {
    pub subreddit: String,
    pub title: String,
    pub url: String,
    pub nsfw: bool,
    pub spoiler: bool,
    pub author: String,
    pub ups: i32,
    pub preview: Vec<String>,
}

pub async fn get() -> Result<Meme, Error> {
    let response = reqwest::get("https://meme-api.com/gimme")
        .await?
        .json::<Meme>()
        .await?;

    Ok(response)
}
