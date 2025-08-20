use crate::{Context, Data, Error};
use poise::serenity_prelude as serenity;
use std::{future::Future, pin::Pin};

pub fn command_check<'a>(
    ctx: Context<'a>,
) -> Pin<Box<dyn Future<Output = Result<bool, Error>> + Send + 'a>> {
    Box::pin(perform_command_check(ctx))
}

pub fn event_handler<'a>(
    ctx: &'a serenity::Context,
    event: &'a serenity::FullEvent,
    framework: poise::FrameworkContext<'a, Data, Error>,
    data: &'a Data,
) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>> {
    Box::pin(perform_event_handler(ctx, event, framework, data))
}

pub fn on_error<'a>(
    error: poise::FrameworkError<'a, Data, Error>,
) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
    Box::pin(perform_on_error(error))
}

async fn perform_command_check(ctx: Context<'_>) -> Result<bool, Error> {
    if ctx.guild_id() != Some(ctx.data().allowed_guild_id) {
        Ok(false)
    } else {
        Ok(true)
    }
}

async fn perform_event_handler(
    _ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    if let serenity::FullEvent::Ready { data_about_bot } = event {
        println!("Connected as {}", data_about_bot.user.name);
    }

    Ok(())
}

async fn perform_on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => {
            panic!("Failed to setup framework: {error:?}");
        }
        poise::FrameworkError::Command { error, ctx, .. } => {
            println!(
                "Error in command `{}`: {:?}",
                ctx.command().qualified_name,
                error
            );
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Failed to call error handler: {e:?}");
            }
        }
    }
}
