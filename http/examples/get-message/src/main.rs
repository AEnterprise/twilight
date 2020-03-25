use futures::future;
use std::env;
use twilight_http::prelude::*;
use twilight_model::id::ChannelId;

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init_timed();

    let client = Client::new(env::var("DISCORD_TOKEN").expect("Could not create new Client with provided token"));
    let channel_id = ChannelId(381_926_291_785_383_946);

    future::join_all((1u8..=10).map(|x| {
        client
            .create_message(channel_id)
            .content(format!("Ping #{}", x))
    }))
    .await;

    let me = client.current_user().await?;
    println!("Current user: {}#{}", me.name, me.discriminator);

    Ok(())
}
