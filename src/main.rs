mod commands;
mod db;
mod error;
mod juge;
mod setting;

use db::DataBase;
use error::Error;
use juge::Juge;
use serenity::{prelude::GatewayIntents, Client};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let token = env::var("DISCORD_TOKEN").expect("No found token");
    let intents = GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::MESSAGE_CONTENT;

    DataBase::init();
    let mut client = Client::builder(&token, intents)
        .event_handler(Juge::new())
        .await?;
    client.start().await?;

    Ok(())
}
