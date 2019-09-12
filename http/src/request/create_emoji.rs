use dawn_model::{
    guild::Emoji,
    id::{GuildId, RoleId},
};
use serde::Serialize;
use super::prelude::*;

#[derive(Serialize)]
pub struct CreateEmoji<'a> {
    roles: Option<Vec<RoleId>>,
    #[serde(skip)]
    fut: Option<PendingBody<'a, Emoji>>,
    #[serde(skip)]
    guild_id: GuildId,
    #[serde(skip)]
    http: &'a Client,
    image: String,
    name: String,
}

impl<'a> CreateEmoji<'a> {
    pub(crate) fn new(
        http: &'a Client,
        guild_id: impl Into<GuildId>,
        name: impl Into<String>,
        image: impl Into<String>,
    ) -> Self {
        Self {
            fut: None,
            guild_id: guild_id.into(),
            http,
            image: image.into(),
            name: name.into(),
            roles: None,
        }
    }

    pub fn roles(mut self, roles: Vec<RoleId>) -> Self {
        self.roles.replace(roles);

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(self.http.request(Request {
            body: Some(serde_json::to_vec(self)?),
            route: Route::CreateEmoji {
                guild_id: self.guild_id.0,
            },
            ..Default::default()
        })?);

        Ok(())
    }
}

poll_req!(CreateEmoji<'_>, Emoji);
