use crate::{Context, Error};
use chrono::{Duration, Utc};
use poise::serenity_prelude::{
    builder::GetMessages,
    model::{
        channel::ChannelType,
        id::{GuildId, MessageId, UserId},
    },
    prelude::*,
};
use std::collections::HashMap;

/// Returns a list of the top `num_users` active users in the guild.
///
/// NOTE: This function is unoptimized. It will probably take a while to run.
pub async fn get_top_active_users(
    ctx: &Context<'_>,
    guild_id: GuildId,
    num_users: u32,
) -> Result<Vec<UserId>, Error> {
    let http = ctx.http();
    let cutoff = Utc::now() - Duration::days(7);
    let channels = guild_id.channels(http).await?;
    let mut counts: HashMap<UserId, usize> = HashMap::new();

    for (_child, ch) in channels.iter() {
        if ![
            ChannelType::Text,
            ChannelType::News,
            ChannelType::NewsThread,
            ChannelType::PrivateThread,
            ChannelType::PublicThread,
        ]
        .contains(&ch.kind)
        {
            continue;
        }

        let channel_id = ch.id;
        let mut before: Option<MessageId> = None;

        loop {
            let mut retriever = GetMessages::new().limit(100);
            if let Some(b) = before {
                retriever = retriever.before(b);
            }

            let batch = channel_id
                .messages(http, retriever)
                .await
                .unwrap_or(Vec::new());

            if batch.is_empty() {
                break;
            }

            let mut should_continue = true;

            for msg in &batch {
                let created_utc = msg.timestamp.with_timezone(&Utc);

                if created_utc < cutoff {
                    should_continue = false;
                    break;
                }

                *counts.entry(msg.author.id).or_insert(0) += 1;
            }

            if let Some(oldest) = batch.last() {
                before = Some(oldest.id);
            } else {
                break;
            }

            if !should_continue {
                break;
            }
        }
    }

    let mut ranked: Vec<(UserId, usize)> = counts.into_iter().collect();
    ranked.sort_by(|a, b| b.1.cmp(&a.1));
    ranked.truncate(num_users as usize);
    Ok(ranked.into_iter().map(|(id, _)| id.clone()).collect())
}
