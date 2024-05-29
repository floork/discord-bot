use clap::Parser;
extern crate mensa_cli_backend;

#[derive(Parser, Debug)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    login: String,

    /// Number of times to greet
    #[clap(short, long, default_value = "1")]
    id: u8,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    println!("{} {}", args.login, args.id);

    if let Err(err) = mensa_cli_backend::main().await {
        eprintln!("Error: {}", err);
    }
}
