use async_trait::async_trait;
use futures::StreamExt;
use std::{env, error::Error, sync::Arc};
use twilight_gateway::{queue::Queue, Shard, ShardConfig};

#[derive(Debug)]
struct BadQueue;

#[async_trait]
impl Queue for BadQueue {
    // DISCLAIMER: THIS IS A VERY BAD QUEUE!
    async fn request(&self, _shard_id: [u64; 2]) {}
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    pretty_env_logger::init_timed();

    let token = env::var("DISCORD_TOKEN")?;
    let config = ShardConfig::builder(&token).queue(Arc::new(Box::new(BadQueue)));

    let shard = Shard::new(config).await?;
    println!("Created shard");

    let mut events = shard.events().await;

    while let Some(event) = events.next().await {
        println!("Event: {:?}", event.event_type());
    }

    Ok(())
}
