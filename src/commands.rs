use crate::{Context, Error};
use poise::serenity_prelude as serenity;

/// Sends an embed with instructions on how to install Schooltape.
#[poise::command(slash_command)]
pub async fn install(
    ctx: Context<'_>,
    #[description = "Browser you want instructions for"] browser: Option<std::string::String>,
) -> Result<(),Error> {
    let author: serenity::CreateEmbedAuthor = serenity::CreateEmbedAuthor::new("Installation Guide")
            .icon_url("https://github.com/schooltape/schooltape/raw/main/assets/schooltape-logo.png")
            .url("https://github.com/schooltape/schooltape/wiki/Getting-Started#installation");
        
    let chromium: serenity::CreateEmbed = serenity::CreateEmbed::default()
        .title("<:chromium:1246037483192582175> Chromium")
        .description(r#"
1. Download the `Source code (zip)` from the [releases page](https://github.com/schooltape/schooltape/releases/latest).
2. Unzip the file.
3. Navigate to `chrome://extensions`
4. Turn on __Developer Mode__ at the top right.
5. Click the __Load Unpacked__ button near the top left, select the folder you downloaded and navigate to `/schooltape/src` and click ok.
6. Done! Options can be found in the right click context menu, and use left click to toggle.
        "#)
        .author(author.clone());

    let firefox: serenity::CreateEmbed = serenity::CreateEmbed::default()
        .title("<:firefox:1246037481975971870> Firefox")
        .description(r#"
1. Download the `schooltape-X.X.X.xpi` from the [releases page](https://github.com/schooltape/schooltape/releases/latest).
2. Click install
3. Done! Options can be found in the right click context menu, and use left click to toggle.
        "#)
        .author(author.clone());

    let edge: serenity::CreateEmbed = serenity::CreateEmbed::default()
        .title("<:edge:1246037472761217065> Edge")
        .description(r#"
1. Download the `Source code (zip)` from the [releases page](https://github.com/schooltape/schooltape/releases/latest).
2. Unzip the file.
3. Navigate to `edge://extensions`
4. Turn on __Developer Mode__ near the bottom left.
5. Click the __Load Unpacked__ button near the top right, select the folder you downloaded and navigate to `/schooltape/src` and click ok.
6. Scroll down until you find a heading that says __From other sources__.
7. Done! Options can be found in the right click context menu, and use left click to toggle.
        "#)
        .author(author.clone());

    let safari: serenity::CreateEmbed = serenity::CreateEmbed::default()
        .title("<:safari:1246037470840094720> Safari")
        .description(r#"
Sorry, Safari is not supported at this time.
        "#)
        .author(author.clone());
    
    let reply = {
        let components: Vec<serenity::CreateActionRow> = vec![
            serenity::CreateActionRow::SelectMenu(
                serenity::CreateSelectMenu::new(
                    "browsers",
                    serenity::CreateSelectMenuKind::String {
                    options: (
                        vec![
                            serenity::CreateSelectMenuOption::new(
                                "Chromium",
                                "chromium",
                            ),
                            serenity::CreateSelectMenuOption::new(
                                "Firefox",
                                "firefox",
                            ),
                            serenity::CreateSelectMenuOption::new(
                                "Microsoft Edge",
                                "edge",
                            ),
                            serenity::CreateSelectMenuOption::new(
                                "Safari",
                                "safari",
                            ),
                        ]
                    )
                }
                )
                .placeholder("Select a browser")
            )
        ];

        let browser = browser.unwrap_or_else(|| String::from("chromium"));

        let embed = match browser.as_str() {
            "chromium" => chromium.clone(),
            "firefox" => firefox.clone(),
            "edge" => edge.clone(),
            "safari" => safari.clone(),
            _ => return Err(Error::from("Invalid browser name")),
        };

        poise::CreateReply::default()
            .embed(embed)
            .components(components)
    };

    ctx.send(reply).await?;

    while let Some(mci) = serenity::ComponentInteractionCollector::new(ctx)
        .author_id(ctx.author().id)
        .channel_id(ctx.channel_id())
        .timeout(std::time::Duration::from_secs(120))
        .filter(move |mci| mci.data.custom_id == "browsers")
        .await
    {
        let mut msg = mci.message.clone();
        println!("{:?}", mci.data.kind);
        match &mci.data.kind {
            serenity::ComponentInteractionDataKind::StringSelect { values, .. } => {
                if let Some(first_value) = values.get(0) {
                    match first_value.as_str() {
                        "chromium" => msg.edit(ctx, serenity::EditMessage::new().embed(chromium.clone())).await?,
                        "firefox" => msg.edit(ctx, serenity::EditMessage::new().embed(firefox.clone())).await?,
                        "edge" => msg.edit(ctx, serenity::EditMessage::new().embed(edge.clone())).await?,
                        "safari" => msg.edit(ctx, serenity::EditMessage::new().embed(safari.clone())).await?,
                        _ => (),
                    }
                }
            }
            _ => (),
        }

        mci.create_response(ctx, serenity::CreateInteractionResponse::Acknowledge)
            .await?;
    }

    Ok(())
}

// /// Purge a specified number of messages from the current channel.
// #[poise::command(
//     slash_command,
//     owners_only,
// )]
// pub async fn purge(ctx: Context<'_>, #[description = "Number of messages to purge"] count: u64) -> Result<(), Error> {
//     let messages = ctx
//         .channel_id
//         .messages(&ctx.discord.http, |retriever| retriever.limit(count))
//         .await?;

//     let message_ids = messages.iter().map(|m| m.id).collect::<Vec<_>>();
//     ctx.channel_id.delete_messages(&ctx.discord.http, message_ids).await?;

//     Ok(())
// }

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

/// Provides a menu to register application commands.
#[poise::command(
    prefix_command,
    owners_only,
)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}
