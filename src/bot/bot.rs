use super::commands::{bot, daily_fact, meal, meme, random_fact, Data, Error};
use poise::serenity_prelude as serenity;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};

/// Handles errors encountered by the poise framework.
async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx, .. } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}

/// Sets up framework options for the bot.
fn setup_framework_options() -> poise::FrameworkOptions<Data, Error> {
    poise::FrameworkOptions {
        commands: vec![meal(), meme(), daily_fact(), random_fact(), bot()],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("~".into()),
            edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(
                Duration::from_secs(3600),
            ))),
            additional_prefixes: vec![
                poise::Prefix::Literal("hey bot"),
                poise::Prefix::Literal("hey bot,"),
            ],
            ..Default::default()
        },
        on_error: |error| Box::pin(on_error(error)),
        pre_command: |ctx| {
            Box::pin(async move {
                println!("Executing command {}...", ctx.command().qualified_name);
            })
        },
        post_command: |ctx| {
            Box::pin(async move {
                println!("Executed command {}!", ctx.command().qualified_name);
            })
        },
        command_check: Some(|ctx| {
            Box::pin(async move {
                if ctx.author().id == 123456789 {
                    return Ok(false);
                }
                Ok(true)
            })
        }),
        skip_checks_for_owners: false,
        event_handler: |_ctx, event, _framework, _data| {
            Box::pin(async move {
                println!(
                    "Got an event in event handler: {:?}",
                    event.snake_case_name()
                );
                Ok(())
            })
        },
        ..Default::default()
    }
}

/// Sets up the framework with the provided options.
fn setup_framework(options: poise::FrameworkOptions<Data, Error>) -> poise::Framework<Data, Error> {
    poise::Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", _ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    votes: Mutex::new(HashMap::new()),
                })
            })
        })
        .options(options)
        .build()
}

/// Initializes the Discord bot client.
async fn initialize_client(
    token: &str,
    framework: poise::Framework<Data, Error>,
) -> Result<serenity::Client, serenity::Error> {
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    // Check if the token is empty
    if token.is_empty() {
        panic!("Invalid token: Please provide a valid bot token. Make sure it starts with 'Bot '.");
    }

    serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await
}

/// Starts the Discord bot with the specified token.
pub async fn start_bot(token: &str) {
    let options = setup_framework_options();
    let framework = setup_framework(options);

    match initialize_client(token, framework).await {
        Ok(mut client) => {
            if let Err(e) = client.start().await {
                panic!("Failed to start the client: {:?}", e);
            }
        }
        Err(e) => {
            panic!("Failed to create the client: {:?}", e);
        }
    }
}
