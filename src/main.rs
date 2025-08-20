pub mod data;
pub mod utils;

mod commands;
mod handlers;

use anyhow::Context as _;
use data::Data;
use handlers::{command_check, event_handler, on_error};
use poise::serenity_prelude as serenity;
use serenity::{ClientBuilder, GatewayIntents};
use shuttle_runtime::SecretStore;
use shuttle_serenity::ShuttleSerenity;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secret_store: SecretStore) -> ShuttleSerenity {
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("`DISCORD_TOKEN` must be set")?;
    let prefix = secret_store.get("PREFIX").context("`PREFIX` must be set")?;
    let data = Data::from_secrets(secret_store)?;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands::all(),
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(prefix),
                edit_tracker: Some(Into::into(poise::EditTracker::for_timespan(
                    std::time::Duration::from_secs(60),
                ))),
                ..Default::default()
            },
            command_check: Some(command_check),
            event_handler,
            on_error,
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(data)
            })
        })
        .build();

    let client = ClientBuilder::new(
        discord_token,
        GatewayIntents::non_privileged()
            | GatewayIntents::MESSAGE_CONTENT
            | GatewayIntents::GUILD_MEMBERS,
    )
    .framework(framework)
    .await
    .map_err(shuttle_runtime::CustomError::new)?;

    Ok(client.into())
}
