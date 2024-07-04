use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(short = 'L', long, default_value = None)]
    pub location: Option<String>,

    #[clap(short = 'I', long, default_value = None)]
    pub id: Option<u32>,

    #[clap(short = 'D', long, default_value = "today")]
    pub date: String,

    #[clap(short = 'B', long, default_value = "false")]
    pub discord_bot: bool,

    #[clap(short = 'T', long, default_value = None)]
    pub token: Option<String>,
}
