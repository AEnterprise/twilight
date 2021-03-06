use crate::{
    gateway::presence::Presence,
    guild::Member,
    id::{GuildId, UserId},
};
use std::collections::HashMap;

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MemberChunk {
    pub guild_id: GuildId,
    #[cfg_attr(feature = "serde-support", serde(with = "serde_mappable_seq"))]
    pub members: HashMap<UserId, Member>,
    #[cfg_attr(feature = "serde-support", serde(with = "serde_mappable_seq", default))]
    pub presences: HashMap<UserId, Presence>,
    pub chunk_index: u32,
    pub chunk_count: u32,
    #[cfg_attr(feature = "serde-support", serde(default))]
    pub not_found: Vec<UserId>,
    pub nonce: Option<String>,
}
