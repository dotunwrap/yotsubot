use anyhow::Context as _;
use poise::serenity_prelude::{GuildId, RoleId, UserId};
use shuttle_runtime::{Error, SecretStore};

pub struct Data {
    pub allowed_guild_id: GuildId,
    pub new_member_role_id: RoleId,
    pub verified_role_id: RoleId,
    pub reverify_excluded_role_ids: Vec<RoleId>,
    pub reverify_excluded_user_ids: Vec<UserId>,
}

impl Data {
    pub fn from_secrets(secret_store: SecretStore) -> Result<Self, Error> {
        Ok(Self {
            allowed_guild_id: GuildId::from(
                secret_store
                    .get("ALLOWED_GUILD_ID")
                    .context("`ALLOWED_GUILD_ID` must be set")?
                    .parse::<u64>()
                    .context("`ALLOWED_GUILD_ID` must be a number")?,
            ),
            new_member_role_id: RoleId::from(
                secret_store
                    .get("NEW_MEMBER_ROLE_ID")
                    .context("`NEW_MEMBER_ROLE_ID` must be set")?
                    .parse::<u64>()
                    .context("`NEW_MEMBER_ROLE_ID` must be a number")?,
            ),
            verified_role_id: RoleId::from(
                secret_store
                    .get("VERIFIED_ROLE_ID")
                    .context("`VERIFIED_ROLE_ID` must be set")?
                    .parse::<u64>()
                    .context("`VERIFIED_ROLE_ID` must be a number")?,
            ),
            reverify_excluded_role_ids: secret_store
                .get("REVERIFY_EXCLUDED_ROLE_IDS")
                .context("`REVERIFY_EXCLUDED_ROLE_IDS` must be set")?
                .split(',')
                .filter_map(|id| id.parse::<u64>().ok())
                .map(|id| RoleId::from(id))
                .collect::<Vec<RoleId>>(),
            reverify_excluded_user_ids: secret_store
                .get("REVERIFY_EXCLUDED_USER_IDS")
                .context("`REVERIFY_EXCLUDED_USER_IDS` must be set")?
                .split(',')
                .filter_map(|id| id.parse::<u64>().ok())
                .map(|id| UserId::from(id))
                .collect::<Vec<UserId>>(),
        })
    }
}
