use crate::{Context, Error};

/// Hello world!
#[poise::command(slash_command)]
pub async fn hello(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("world!").await?;
    Ok(())
}

/// Returns the current gateway heartbeat latency.
/// If the shard has just connected, this will return 0.
#[poise::command(
    slash_command,
    ephemeral,
)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let latency = ctx.ping().await;
    ctx.say(format!("Pong! Latency: {} ms", latency.as_millis())).await?;
    Ok(())
}

#[poise::command(
    prefix_command,
    owners_only,
)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}
