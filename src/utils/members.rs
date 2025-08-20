use crate::{Context, Error};
use poise::serenity_prelude::model::guild::Member;

pub async fn verify_member(ctx: &Context<'_>, member: &Member) -> Result<(), Error> {
    member
        .remove_role(ctx.http(), &ctx.data().new_member_role_id)
        .await?;

    member
        .add_role(ctx.http(), &ctx.data().verified_role_id)
        .await?;

    Ok(())
}

pub async fn unverify_member(ctx: &Context<'_>, member: &Member) -> Result<(), Error> {
    member
        .remove_role(ctx.http(), &ctx.data().verified_role_id)
        .await?;

    member
        .add_role(ctx.http(), &ctx.data().new_member_role_id)
        .await?;

    Ok(())
}
