mod moderation;

pub fn all() -> Vec<poise::Command<crate::Data, crate::Error>> {
    vec![moderation::reverify_active_users(), moderation::verify()]
}
