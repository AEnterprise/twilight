use dawn_model::{
    guild::Emoji,
    id::{EmojiId, GuildId, RoleId},
};
use super::prelude::*;

#[derive(Serialize)]
pub struct UpdateEmoji<'a> {
    name: Option<String>,
    roles: Option<Vec<RoleId>>,
    #[serde(skip)]
    emoji_id: EmojiId,
    #[serde(skip)]
    fut: Option<PendingBody<'a, Emoji>>,
    #[serde(skip)]
    guild_id: GuildId,
    #[serde(skip)]
    http: &'a Client,
}

impl<'a> UpdateEmoji<'a> {
    pub(crate) fn new(
        http: &'a Client,
        guild_id: impl Into<GuildId>,
        emoji_id: impl Into<EmojiId>,
    ) -> Self {
        Self {
            name: None,
            roles: None,
            emoji_id: emoji_id.into(),
            fut: None,
            guild_id: guild_id.into(),
            http,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name.replace(name.into());

        self
    }

    pub fn roles(mut self, roles: Vec<RoleId>) -> Self {
        self.roles.replace(roles);

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(self.http.request(Request {
            body: Some(serde_json::to_vec(self)?),
            route: Route::UpdateEmoji {
                emoji_id: self.emoji_id.0,
                guild_id: self.guild_id.0,
            },
            ..Default::default()
        })?);

        Ok(())
    }
}

poll_req!(UpdateEmoji<'_>, Emoji);
