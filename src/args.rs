use clap::Parser;

/// a discord bot with an according cli
#[derive(Parser, Debug)]
pub struct Args {
    /// Manually select a city for location.
    #[clap(short = 'L', long)]
    pub location: Option<String>,

    /// Manually select a canteen based on its ID.
    #[clap(short = 'I', long)]
    pub id: Option<u32>,

    /// Date of the meal (defaults to "today").
    #[clap(short = 'D', long, default_value = "today")]
    pub date: String,

    /// Start a Discord bot.
    #[clap(short = 'B', long)]
    pub discord_bot: bool,

    /// Set the token for the Discord bot.
    #[clap(short = 'T', long)]
    pub token: Option<String>,

    /// Set the path for the .env file containing the token
    #[clap(short = 'E', long)]
    pub env_file: Option<String>,

    /// Display a random meme.
    #[clap(short = 'M', long)]
    pub meme: bool,

    /// Display a daily fact.
    #[clap(short = 'F', long)]
    pub daily_fact: bool,

    /// Display a random fact.
    #[clap(short = 'R', long)]
    pub random_fact: bool,
}
