use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn hello(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("world!").await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Pong!").await?;
    Ok(())
}

#[poise::command(prefix_command)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    if ctx.author().id == 1182451860666318928 {
        poise::builtins::register_application_commands_buttons(ctx).await?;
    } else {
        ctx.say("You are not allowed to use this command!").await?;
    }
    Ok(())
}
