// see https://github.com/serenity-rs/poise/blob/current/examples/basic_structure/main.rs

mod commands;

use poise::serenity_prelude as serenity;
use anyhow::Context as _;
use shuttle_runtime::SecretStore;
use shuttle_serenity::ShuttleSerenity;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
use tracing::info;

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
            commands::install(),
        ],
        initialize_owners: true,
        // The global error handler for all error cases that may occur
        on_error: |error| Box::pin(on_error(error)),
        // This code is run before every command
        pre_command: |ctx| {
            Box::pin(async move {
                info!("Executing command '{}'...", ctx.command().qualified_name);
            })
        },
        // This code is run after a command if it was successful (returned Ok)
        post_command: |ctx| {
            Box::pin(async move {
                info!("Executed command '{}'!", ctx.command().qualified_name);
            })
        },
        event_handler: |ctx, event, framework, data| {
            Box::pin(event_handler(ctx, event, framework, data))
        },
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .setup(move |ctx, ready, framework| {
            Box::pin(async move {
                info!("{} is connected!", ready.user.name);
                // poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                for guild in &ready.guilds {
                    poise::builtins::register_in_guild(ctx, &framework.options().commands, guild.id).await?;
                    info!("Loaded modules for guild {}", guild.id);
                }
                Ok(Data {})
            })
        })
        .options(options)
        .build();

    // Get the discord token set in `Secrets.toml`
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    let client = serenity::ClientBuilder::new(discord_token, serenity::GatewayIntents::non_privileged())
        .framework(framework)
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    Ok(client.into())
}


async fn event_handler(
    _ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    info!(
        "Got an event in event handler: {:?}",
        event.snake_case_name()
    );
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            info!("Logged in as {}", data_about_bot.user.name);
        }
        _ => {}
    }
    Ok(())
}
