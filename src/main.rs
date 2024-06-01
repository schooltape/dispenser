// see https://github.com/serenity-rs/poise/blob/current/examples/basic_structure/main.rs

mod commands;

use anyhow::Context as _;
use poise::serenity_prelude::{ClientBuilder, GatewayIntents, GuildId};
use shuttle_runtime::SecretStore;
use shuttle_serenity::ShuttleSerenity;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

// Schooltape Guild ID
const GUILD_ID: GuildId = GuildId::new(1109737131225649283);

pub struct Data {}

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx, .. } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secret_store: SecretStore) -> ShuttleSerenity {
    let options = poise::FrameworkOptions {
        commands: vec![
            commands::hello(),
            commands::register(),
            commands::ping(),
        ],
        // The global error handler for all error cases that may occur
        on_error: |error| Box::pin(on_error(error)),
        // This code is run before every command
        pre_command: |ctx| {
            Box::pin(async move {
                println!("Executing command {}...", ctx.command().qualified_name);
            })
        },
        // This code is run after a command if it was successful (returned Ok)
        post_command: |ctx| {
            Box::pin(async move {
                println!("Executed command {}!", ctx.command().qualified_name);
            })
        },
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
    };

    let framework = poise::Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", _ready.user.name);
                // poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                poise::builtins::register_in_guild(ctx, &framework.options().commands, GUILD_ID).await?;
                Ok(Data {})
            })
        })
        .options(options)
        .build();

    // Get the discord token set in `Secrets.toml`
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    let client = ClientBuilder::new(discord_token, GatewayIntents::non_privileged())
        .framework(framework)
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    Ok(client.into())
}
