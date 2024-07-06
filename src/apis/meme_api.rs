use reqwest::Error;
use serde::Deserialize;

/// Represents a meme fetched from the meme API.
#[derive(Debug, Deserialize)]
pub struct Meme {
    /// The subreddit where the meme was posted.
    pub subreddit: String,
    /// The title of the meme.
    pub title: String,
    /// The URL of the meme image.
    pub url: String,
    /// Indicates if the meme is NSFW (Not Safe For Work).
    pub nsfw: bool,
    /// Indicates if the meme is a spoiler.
    pub spoiler: bool,
    /// The author of the meme.
    pub author: String,
    /// The number of upvotes the meme has received.
    pub ups: i32,
    /// URLs of preview images for the meme.
    pub preview: Vec<String>,
}

/// Fetches a random meme from the meme API.
///
/// # Errors
///
/// Returns a `reqwest::Error` if the HTTP request fails or if deserialization of the response fails.
///
/// # Returns
///
/// Returns a `Result` containing a `Meme` struct if successful, or a `reqwest::Error` if an error occurs.
pub async fn get() -> Result<Meme, Error> {
    let response = reqwest::get("https://meme-api.com/gimme")
        .await?
        .json::<Meme>()
        .await?;

    Ok(response)
}
