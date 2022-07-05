mod commands;
mod error;
mod juge;

use error::Error;
use juge::Juge;
use serenity::{prelude::GatewayIntents, Client};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let token = env::var("DISCORD_TOKEN")?;
    let intents = GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents).event_handler(Juge).await?;
    client.start().await?;

    Ok(())
}
