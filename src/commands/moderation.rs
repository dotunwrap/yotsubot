use crate::{
    Context, Error,
    utils::{
        guilds::get_top_active_users,
        members::{unverify_member, verify_member},
    },
};
use anyhow::Context as _;
use poise::serenity_prelude as serenity;
use serenity::model::guild::Member;

/// Re-verifies `num_users` active users in the guild. All other users will be unverified.
///
/// NOTE: This command is unoptimized. It will probably take a while to run.
#[poise::command(
    slash_command,
    guild_only,
    category = "Moderation",
    required_permissions = "ADMINISTRATOR"
)]
pub async fn reverify_active_users(ctx: Context<'_>, num_users: u32) -> Result<(), Error> {
    let guild_id = ctx.guild_id().context("Guild ID not found")?;
    let top_active_users = get_top_active_users(&ctx, guild_id, num_users).await?;

    let mut last_user_id = None;

    loop {
        let members = guild_id
            .members(ctx.http(), Some(100), last_user_id)
            .await
            .unwrap_or(Vec::new());

        if members.is_empty() {
            break;
        }

        for member in &members {
            if top_active_users.contains(&member.user.id) {
                continue;
            }

            if member
                .roles
                .iter()
                .any(|r| ctx.data().reverify_excluded_role_ids.contains(&r))
            {
                continue;
            }

            if ctx
                .data()
                .reverify_excluded_user_ids
                .contains(&member.user.id)
            {
                continue;
            }

            unverify_member(&ctx, &member).await?;
        }

        if let Some(user_id) = members.last().map(|m| m.user.id) {
            last_user_id = Some(user_id);
        } else {
            break;
        }
    }

    Ok(())
}

/// Verifies a new user in the guild.
#[poise::command(
    slash_command,
    guild_only,
    category = "Moderation",
    required_permissions = "MANAGE_ROLES"
)]
pub async fn verify(ctx: Context<'_>, member: Member) -> Result<(), Error> {
    verify_member(&ctx, &member).await
}
