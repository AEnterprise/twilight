use bitflags::bitflags;

bitflags! {
    pub struct UserFlags: u64 {
        const DISCORD_EMPLOYEE = 1;
        const DISCORD_PARTNER = 1 << 1;
        const HYPESQUAD_EVENTS = 1 << 2;
        const BUG_HUNTER = 1 << 3;
        const HOUSE_BRAVERY = 1 << 6;
        const HOUSE_BRILLIANCE = 1 << 7;
        const HOUSE_BALANCE = 1 << 8;
        const EARLY_SUPPORTER = 1 << 9;
        const TEAM_USER = 1 << 10;
        const SYSTEM = 1 << 12;
    }
}

#[cfg(feature = "serde-support")]
mod serde_support {
    use super::UserFlags;
    use serde::{
        de::{Deserialize, Deserializer},
        ser::{Serialize, Serializer},
    };

    impl<'de> Deserialize<'de> for UserFlags {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            Ok(Self::from_bits_truncate(u64::deserialize(deserializer)?))
        }
    }

    impl Serialize for UserFlags {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_u64(self.bits())
        }
    }
}
