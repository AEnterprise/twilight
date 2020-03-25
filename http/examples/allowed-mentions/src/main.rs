use std::env;
use twilight_http::prelude::*;
use twilight_model::id::{ChannelId, UserId};

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init_timed();

    let client = Client::new(env::var("DISCORD_TOKEN").expect("Could not create new Client with provided token"));
    let channel_id = ChannelId(381_926_291_785_383_946);
    let user_id = UserId(77_469_400_222_932_992);

    client
        .create_message(channel_id)
        .content(format!("Hi <@{}>", user_id.0))
        .allowed_mentions()
        .parse_specific_users(vec![user_id])
        .build()
        .await?;

    Ok(())
}
